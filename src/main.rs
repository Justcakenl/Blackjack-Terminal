mod bank;
mod cards;
mod game;
mod modes;
mod player;
mod ui;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use game::Round;
use modes::Mode;
use player::{load, save, Player};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

fn main() -> io::Result<()> {
    let mut player = load();
    player.daily();
    let result = run(&mut player);
    let _ = save(&player);
    result
}

fn run(player: &mut Player) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;
    let result = app(&mut terminal, player);
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    result
}

fn app(t: &mut Terminal<CrosstermBackend<io::Stdout>>, player: &mut Player) -> io::Result<()> {
    let mut mode = Mode::Classic;
    let mut round: Option<Round> = None;
    let mut screen = "MENU";
    let mut bet = mode.config().min_bet;
    let mut bank_amount = mode.config().min_bet;
    let mut selected = 0usize;
    let mut popup = false;
    let mut dev_input = String::new();

    loop {
        t.draw(|f| {
            ui::draw(
                f,
                player,
                mode,
                round.as_ref(),
                if popup { "RESULT" } else if round.is_some() { "TABLE" } else { screen },
                bet,
                selected,
            )
        })?;

        if !event::poll(std::time::Duration::from_millis(250))? {
            continue;
        }

        let Event::Key(key) = event::read()? else {
            continue;
        };
        let code = key.code;

        // 1. Popup / Result Handling
        if popup {
            if code == KeyCode::Char(' ') {
                popup = false;
                if round.as_ref().is_some_and(|r| r.settled) {
                    round = None;
                    screen = "MENU";
                }
            }
            continue;
        }

        // 2. Dev Mode handling
        if screen == "DEV" {
            if code == KeyCode::Char('q') {
                screen = "MENU";
                dev_input.clear();
            } else if !player.dev_mode {
                match code {
                    KeyCode::Char(c) if c.is_ascii_alphanumeric() => dev_input.push(c),
                    KeyCode::Backspace => {
                        dev_input.pop();
                    }
                    KeyCode::Enter => {
                        if dev_input == "Admin1234" {
                            player.dev_mode = true;
                             let _ = save(&player);
                        }
                        dev_input.clear();
                    }
                    _ => {}
                }
            } else {
                match code {
                    KeyCode::Up => selected = selected.saturating_sub(1),
                    KeyCode::Down => selected = (selected + 1) % 4,
                    KeyCode::Enter => match selected {
                        0 => player.chips = player.chips.saturating_add(1000),
                        1 => player.chips = player.chips.saturating_sub(1000),
                        2 => player.xp = player.xp.saturating_add(100),
                        3 => {
                            player.xp = player.xp.saturating_sub(100);
                        }
                        _ => {
                            player.dev_mode = false;
                            let _ = save(player);
                        }
                    },
                    _ => {}
                }
            }
            continue;
        }

        // 3. Table / Round handling
        if let Some(r) = round.as_mut() {
            if r.settled {
                if matches!(code, KeyCode::Char('q') | KeyCode::Esc | KeyCode::Enter) {
                    round = None;
                    screen = "MENU";
                }
                continue;
            }

            match code {
                KeyCode::Char('h') | KeyCode::Char(' ') => r.hit(),
                KeyCode::Char('s') | KeyCode::Char('a') => r.stand(),
                KeyCode::Char('p') => {
                    let b = r.hands[r.active].bet;
                    if player.chips >= b && r.split() {
                        player.chips -= b;
                    }
                }
                KeyCode::Char('d') => {
                    let b = r.hands[r.active].bet;
                    if player.chips >= b {
                        player.chips -= b;
                        r.hands[r.active].bet *= 2;
                        r.hands[r.active].doubled = true;
                        r.hit();
                        r.stand();
                    }
                }
                KeyCode::Char('r') => {
                    if mode.config().surrender {
                        r.hands[r.active].surrendered = true;
                        r.stand();
                    }
                }
                KeyCode::Char('i') => {
                    if mode.config().insurance
                        && r.insurance == 0
                        && r.hands.len() == 1
                        && r.hands[0].cards.len() == 2
                        && r.dealer.cards[0].rank == cards::Rank::Ace
                    {
                        let i = r.hands[0].bet / 2;
                        if player.chips >= i {
                            r.insurance = i;
                            player.chips -= i;
                        }
                    }
                }
                _ => {}
            }

            if r.over && !r.settled {
                r.dealer_play(mode.config().dealer_hits_soft_17);
                let mut net = 0;
                for h in &r.hands {
                    if !h.surrendered {
                        player.chips += h.bet;
                    }
                    net += game::payout(h, &r.dealer, mode);
                }
                if r.insurance > 0 && r.dealer.blackjack() {
                    net += (r.insurance * 2) as i64;
                }
                if net >= 0 {
                    player.chips += net as u64;
                } else {
                    player.chips = player.chips.saturating_sub((-net) as u64);
                }
                r.result = net;
                r.settled = true;
                if net > 0 {
                    player.unlock("First Win");
                } else if net < 0 {
                    player.unlock("Survived a Loss");
                }
                if player.settle_house(net) {
                    player.unlock("Break the Bank");
                }
                player.record(net);
                popup = true;
            }
            continue;
        }

        // 4. Menu handling
        if matches!(code, KeyCode::Char('q') | KeyCode::Esc) && screen == "MENU" {
            break;
        }

        match code {
            KeyCode::Char('m') => {
                let n = mode.next();
                if player.mode_unlocked(n) {
                    mode = n;
                    bet = mode.config().min_bet;
                }
            }
            KeyCode::Char('n') => {
                let prev = mode.previous();
                if player.mode_unlocked(prev) {
                    mode = prev;
                    bet = mode.config().min_bet;
                }
            }
            KeyCode::Char('b') => screen = "BANK",
            KeyCode::Char('t') => screen = "STATS",
            KeyCode::Char('a') => screen = "ACHIEVEMENTS",
            KeyCode::Char('v') => screen = "DEV",
            KeyCode::Char('?') => screen = "HELP",
            KeyCode::Char('q') | KeyCode::Esc => screen = "MENU",
            KeyCode::Left => bet = bet.saturating_sub(10).max(mode.config().min_bet),
            KeyCode::Right => bet = (bet + 10).min(player.chips),
            KeyCode::Char('1') if screen != "BANK" => bet = mode.config().min_bet,
            KeyCode::Char('2') if screen != "BANK" => bet = player.chips / 2,
            KeyCode::Char('3') if screen != "BANK" => bet = player.chips,
            KeyCode::Enter | KeyCode::Char(' ') if screen == "MENU" && player.chips >= bet => {
                player.chips -= bet;
                let mut r = Round::new(cards::Shoe::new(), bet);
                if r.dealer.blackjack() && !r.hands[0].blackjack() {
                    r.over = true;
                }
                round = Some(r);
            }
            KeyCode::Char('d') if screen == "BANK" => {
                let a = bank_amount.min(player.chips);
                let _ = player.bank.deposit(&mut player.chips, a);
            }
            KeyCode::Char('w') if screen == "BANK" => {
                let _ = player.bank.withdraw(&mut player.chips, bank_amount);
            }
            KeyCode::Char('l') if screen == "BANK" => {
                let l = player.level();
                let _ = player.bank.borrow(&mut player.chips, bank_amount, l);
            }
            KeyCode::Up if screen == "BANK" => bank_amount = bank_amount.saturating_add(10),
            KeyCode::Down if screen == "BANK" => bank_amount = bank_amount.saturating_sub(10),
            KeyCode::Char('1') if screen == "BANK" => bank_amount = 10,
            KeyCode::Char('2') if screen == "BANK" => bank_amount = player.bank.savings,
            KeyCode::Char('3') if screen == "BANK" => bank_amount = player.chips,
            _ => {}
        }
    }
    Ok(())
}
