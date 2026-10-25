use crate::{game::Round, modes::Mode, player::Player};
use ratatui::{prelude::*, widgets::*};

fn card_line(cards: impl Iterator<Item = String>) -> Line<'static> {
    let cards: Vec<String> = cards.collect();
    Line::from(cards.join("   "))
}

fn card_block(cards: impl Iterator<Item = String>, hidden: bool) -> Vec<Line<'static>> {
    let cards: Vec<String> = cards.collect();
    let mut out = vec![Line::from(" ")];
    out.push(card_line(cards.iter().enumerate().map(|(i, _card)| {
        if hidden && i == 1 { "┌─────┐".to_string() } else { "┌─────┐".to_string() }
    })));
    out.push(card_line(cards.iter().enumerate().map(|(i, card)| {
        if hidden && i == 1 { "│░░░░░│".to_string() } else { format!("│ {:<3} │", card) }
    })));
    out.push(card_line(cards.iter().map(|_| "│     │".to_string())));
    out.push(card_line(cards.iter().map(|_| "└─────┘".to_string())));
    out
}

pub fn draw(f: &mut Frame, player: &Player, mode: Mode, round: Option<&Round>, screen: &str, bet: u64, selected: usize) {
    let outer = Layout::vertical([Constraint::Length(4), Constraint::Min(8), Constraint::Length(3)]).split(f.area());
    let header = Paragraph::new(Line::from(vec![
        Span::styled("  ♠  BLACKJACK  ♣  ", Style::default().fg(Color::Black).bg(Color::Rgb(245, 190, 45)).bold()),
        Span::styled(format!("  {}  ", screen), Style::default().fg(Color::LightGreen).bold()),
        Span::styled(format!("  ◈ {} chips   LVL {}   {}", player.chips, player.level(), mode.config().name), Style::default().fg(Color::Yellow)),
    ])).alignment(Alignment::Center).block(Block::default().borders(Borders::BOTTOM).border_style(Style::default().fg(Color::Yellow)));
    f.render_widget(header, outer[0]);

    let content = match screen {
        "BANK" => bank_view(player, bet),
        "STATS" => stats_view(player),
        "HELP" => help_view(),
        "ACHIEVEMENTS" => achievements_view(player),
        "TABLE" => table_view(player, mode, round.expect("table has a round")), "RESULT" => result_view(round.expect("result has a round")), "DEV" => dev_view(player, selected),
        _ => menu_view(player, mode, bet),
    };
    f.render_widget(content, outer[1]);

    let footer = if screen == "TABLE" { " [Space/H] Hit   [S/A] Stand   [D] Double   [P] Split   [R] Surrender   [I] Insurance   [Enter/Q] Menu after result " } else { " [Enter] Play   [V] Dev mode   [M] Next mode   [N] Previous mode   [B] Banker   [T] Stats   [A] Achievements   [?] Help   [Q] Quit " };
    f.render_widget(Paragraph::new(footer).alignment(Alignment::Center).style(Style::default().fg(Color::DarkGray)), outer[2]);
}

fn menu_view(_player: &Player, mode: Mode, bet: u64) -> Paragraph<'static> {
    let text = vec![
        Line::from(Span::styled("WELCOME TO THE FELT", Style::default().fg(Color::LightGreen).bold())),
        Line::from(""),
        Line::from(format!("Selected table: {}   Minimum: ◈ {}   Current bet: ◈ {}", mode.config().name, mode.config().min_bet, bet)),
        Line::from(""),
        Line::from("  Enter   Deal a hand"),
        Line::from("  M       Choose table mode"),
        Line::from("  B       Visit the Banker"),
        Line::from("  T       View statistics"),
        Line::from("  A       Achievements"),
        Line::from("  ?       How to play"),
        Line::from("  Q       Quit"),
        Line::from(""),
        Line::from(Span::styled("Tip: Space is the fastest way to hit.", Style::default().fg(Color::Yellow))),
    ];
    Paragraph::new(text).block(panel(" MAIN MENU ")).alignment(Alignment::Center)
}

fn dev_view(player: &Player, selected: usize) -> Paragraph<'static> { let options = ["Add 1000 chips", "Remove 1000 chips", "Add XP", "Remove XP", "Lock dev mode"]; let prompt = if player.dev_mode { "↑/↓ select   Enter activate   Q back" } else { "Enter passcode   Backspace delete   Q back" }; Paragraph::new(std::iter::once(Line::from(Span::styled("DEVELOPER SANDBOX", Style::default().fg(Color::Red).bold()))).chain(std::iter::once(Line::from(""))).chain(std::iter::once(Line::from(format!("Chips: {}    XP: {}    Level: {}", player.chips, player.xp, player.level())))).chain(options.iter().enumerate().map(|(i, option)| Line::from(format!("{} {}", if i == selected { "▶" } else { " " }, option)))).chain(std::iter::once(Line::from(""))).chain(std::iter::once(Line::from(prompt))).collect::<Vec<_>>()).block(panel(" DEV MODE ")) }
fn result_view(round: &Round) -> Paragraph<'static> { Paragraph::new(vec![Line::from(Span::styled(if round.result > 0 { "YOU WIN!" } else if round.result < 0 { "HOUSE WINS" } else { "PUSH" }, Style::default().fg(Color::Yellow).bold())), Line::from(""), Line::from(format!("Net result: {:+} chips", round.result)), Line::from(""), Line::from("Press SPACE to continue")]).block(panel(" ROUND RESULT ")).alignment(Alignment::Center) }
fn table_view(_player: &Player, mode: Mode, round: &Round) -> Paragraph<'static> {
    let mut lines = vec![Line::from(Span::styled("DEALER", Style::default().fg(Color::LightCyan).bold()))];
    lines.extend(card_block(round.dealer.cards.iter().map(|c| c.text()), !round.over));
    let (dealer_value, dealer_soft) = round.dealer.value();
    lines.push(Line::from(format!("  Total: {}{}", if !round.over { "?".to_string() } else if dealer_soft { "Soft ".to_string() } else { String::new() }, if round.over { dealer_value.to_string() } else { String::new() })));
    lines.push(Line::from(""));
    for (index, hand) in round.hands.iter().enumerate() {
        let (value, soft) = hand.value();
        lines.push(Line::from(Span::styled(format!("PLAYER HAND {}{}", index + 1, if index == round.active { "  ◀ ACTIVE" } else { "" }), Style::default().fg(Color::LightGreen).bold())));
        lines.extend(card_block(hand.cards.iter().map(|c| c.text()), false));
        lines.push(Line::from(format!("  {}{}   ◈ Bet: {}", if soft { "Soft " } else { "" }, value, hand.bet)));
    }
    lines.push(Line::from(format!("  Mode: {}   Insurance: {}", mode.config().name, if round.insurance > 0 { format!("◈ {} (ACTIVE)", round.insurance) } else { "None".to_string() }))); if round.settled { lines.push(Line::from(Span::styled(format!("RESULT: {}{}  (Enter/Q: menu)", if round.result >= 0 { "+" } else { "" }, round.result), Style::default().fg(if round.result >= 0 { Color::LightGreen } else { Color::LightRed }).bold()))); }
    Paragraph::new(lines).block(panel(" LIVE TABLE ")).wrap(Wrap { trim: true })
}

fn bank_view(player: &Player, bet: u64) -> Paragraph<'static> { Paragraph::new(vec![Line::from(Span::styled("THE BANKER", Style::default().fg(Color::Yellow).bold())), Line::from(""), Line::from(format!("Savings                 ◈ {}", player.bank.savings)), Line::from("Interest                1% per real day"), Line::from(format!("Open loan               ◈ {}", player.bank.loan)), Line::from("Loan interest           10% per real day"), Line::from(format!("Current operation amount    ◈ {}", bet)), Line::from(""), Line::from("D deposit   W withdraw   L loan"), Line::from("Up/Down amount   1=10   2=savings   3=chips   Q menu")]).block(panel(" BANK ")) }
fn stats_view(player: &Player) -> Paragraph<'static> { let rate = if player.stats.hands == 0 { 0.0 } else { player.stats.wins as f64 * 100.0 / player.stats.hands as f64 }; Paragraph::new(vec![Line::from(Span::styled("PLAYER LEDGER", Style::default().fg(Color::Yellow).bold())), Line::from(""), Line::from(format!("Hands played       {:>8}", player.stats.hands)), Line::from(format!("Win rate           {:>7.1}%", rate)), Line::from(format!("Total profit       {:>8}", player.stats.total_profit)), Line::from(format!("Biggest win        {:>8}", player.stats.biggest_win)), Line::from(format!("Biggest loss       {:>8}", player.stats.biggest_loss)), Line::from(format!("Current streak     {:>8}", player.stats.streak)), Line::from(format!("Best streak        {:>8}", player.stats.best_streak)), Line::from(format!("Experience         {:>8} / level {}", player.xp, player.level()))]).block(panel(" STATS ")) }
fn help_view() -> Paragraph<'static> { Paragraph::new(vec![Line::from(Span::styled("HOW TO PLAY", Style::default().fg(Color::Yellow).bold())), Line::from(""), Line::from("Beat the dealer without going over 21."), Line::from("The dealer stands on soft 17. Blackjack pays 3:2."), Line::from(""), Line::from("SPACE / H   Hit                    S   Stand"), Line::from("D           Double down            P   Split\nA           Alternate stand"), Line::from("R           Surrender               I   Insurance"), Line::from("Q           Return after the result is shown"), Line::from(""), Line::from("Arrow keys adjust the bet. 1=min, 2=half, 3=all-in.")]).block(panel(" HELP ")) }
fn achievements_view(player: &Player) -> Paragraph<'static> { Paragraph::new(vec![Line::from(Span::styled("ACHIEVEMENTS", Style::default().fg(Color::Yellow).bold())), Line::from(""), Line::from(if player.broken_bank { "★ BREAK THE BANK   Completed" } else { "☆ BREAK THE BANK   Win enough to empty the house bankroll" }), Line::from(format!("House bankroll remaining: ◈ {}\nAchievements unlocked: {}", player.house_bankroll, player.achievements.len())), Line::from(format!("{} HIGH ROLLER       Reach level 5", if player.level() >= 5 { "★" } else { "☆" })), Line::from(format!("{} WHALE             Reach level 10", if player.level() >= 10 { "★" } else { "☆" }))]).block(panel(" ACHIEVEMENTS ")) }
fn panel(title: &'static str) -> Block<'static> { Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Green)).title(Span::styled(title, Style::default().fg(Color::Yellow).bold())) }
