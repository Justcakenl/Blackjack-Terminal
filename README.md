# 🃏 Blackjack TUI

A fast, colorful Blackjack game for your terminal, written in Rust.

Blackjack TUI combines classic casino rules with persistent progression: earn XP, unlock increasingly dangerous betting modes, manage a bank, collect achievements, and try to break the house.

> **Project note**
>
> This is a test and showcase project for **cake-cli**. It is used to exercise cake-cli's ability to work on a real Rust application: terminal UI, game logic, persistence, testing, documentation, and iterative feature development. It is playable software, but it is also intentionally a hands-on development project.

---

## What is in the game?

- **Terminal-first UI** powered by `ratatui` and `crossterm`
- **Persistent save data** for chips, XP, achievements, statistics, and banking
- **Classic Blackjack actions**: hit, stand, double down, split, surrender, and insurance
- **A six-deck shoe** with shuffled cards
- **Progression** from Classic to extreme `x10000` stakes
- **Banking** with savings, daily interest, loans, bonuses, and bailouts
- **Achievements** and a house bankroll you can try to bankrupt
- **Developer mode** for testing game progression
- **Virtual chips only** — no real-money gambling, payments, or online accounts

## Preview

Text recreations of the game's actual `ratatui` layouts in `src/ui.rs`. GitHub renders
fenced blocks in monochrome, so the gold header, green felt panels and cyan dealer
labels are omitted here.

### Main menu

```text
┌──────────────────────────── MAIN MENU ────────────────────────────┐
│                                                                   │
│                      WELCOME TO THE FELT                          │
│                                                                   │
│   Selected table: x1 Classic   Minimum: ◈ 10   Current bet: ◈ 10  │
│                                                                   │
│     Enter   Deal a hand                                           │
│     M       Choose table mode                                     │
│     B       Visit the Banker                                      │
│     T       View statistics                                       │
│     A       Achievements                                          │
│     ?       How to play                                           │
│     Q       Quit                                                  │
│                                                                   │
│            Tip: Space is the fastest way to hit.                  │
│                                                                   │
└───────────────────────────────────────────────────────────────────┘
```

### Live table

```text
┌─────────────────────────── LIVE TABLE ────────────────────────────┐
│ DEALER                                                            │
│  ┌─────┐   ┌─────┐                                                │
│  │ K♠  │   │░░░░░│                                                │
│  │     │   │░░░░░│                                                │
│  └─────┘   └─────┘                                                │
│   Total: ?                                                        │
│                                                                   │
│ PLAYER HAND 1  ◀ ACTIVE                                           │
│  ┌─────┐   ┌─────┐                                                │
│  │ A♥  │   │ 9♦  │                                                │
│  │     │   │     │                                                │
│  └─────┘   └─────┘                                                │
│   Soft 20   ◈ Bet: 10                                             │
│                                                                   │
│   Mode: x1 Classic   Insurance: None                              │
└───────────────────────────────────────────────────────────────────┘
```

### Round result

```text
┌────────────────────────── ROUND RESULT ───────────────────────────┐
│                                                                   │
│                            YOU WIN!                               │
│                                                                   │
│                      Net result: +15 chips                        │
│                                                                   │
│                    Press SPACE to continue                        │
│                                                                   │
└───────────────────────────────────────────────────────────────────┘
```

### The Banker

```text
┌────────────────────────────── BANK ───────────────────────────────┐
│ THE BANKER                                                        │
│                                                                   │
│ Savings                 ◈ 2500                                    │
│ Interest                1% per real day                           │
│ Open loan               ◈ 0                                       │
│ Loan interest           10% per real day                          │
│ Current operation amount    ◈ 1000                                │
│                                                                   │
│ D deposit   W withdraw   L loan                                   │
│ Up/Down amount   1=10   2=savings   3=chips   Q menu              │
└───────────────────────────────────────────────────────────────────┘
```

---

## Install and run

There are two ways to get the game:

1. **Build the latest source from this repository** with Cargo — recommended while the project is in development.
2. **Download a release binary** from the GitHub Releases page when a release is available.

### Option A: Clone and install from source

These commands work on Debian, Ubuntu, Arch Linux, Windows, and other systems supported by Rust.

```bash
git clone https://github.com/Justcakenl/blackjack-tui.git
cd blackjack-tui
cargo install --path .
bljack
```

`cargo install` installs the optimized executable as `bljack`, normally in Cargo's binary directory (`~/.cargo/bin` on Linux/macOS or `%USERPROFILE%\.cargo\bin` on Windows).

### Debian and Ubuntu

Install Git, a compiler toolchain, and Rust:

```bash
sudo apt update
sudo apt install -y git build-essential curl pkg-config
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

Download and install the game:

```bash
git clone https://github.com/Justcakenl/blackjack-tui.git
cd blackjack-tui
cargo install --path .
bljack
```

If `bljack: command not found` appears, open a new terminal or run:

```bash
source "$HOME/.cargo/env"
```

### Arch Linux

Install the required packages:

```bash
sudo pacman -S --needed git base-devel rust
```

Then install from source:

```bash
git clone https://github.com/Justcakenl/blackjack-tui.git
cd blackjack-tui
cargo install --path .
bljack
```

The repository also includes a `PKGBUILD` for packagers. From a cloned checkout, build and install it with:

```bash
makepkg -si
```

### Windows 10 / 11

1. Install [Rustup for Windows](https://www.rust-lang.org/tools/install).
2. During installation, allow Rustup to install the recommended MSVC build tools if prompted. If compilation later asks for a linker, install **Visual Studio Build Tools** with the **Desktop development with C++** workload.
3. Open PowerShell or Command Prompt and run:

```powershell
git clone https://github.com/Justcakenl/blackjack-tui.git
cd blackjack-tui
cargo install --path .
bljack
```

If Windows cannot find `bljack`, restart the terminal so the Cargo bin directory is added to `PATH`. You can also run it directly:

```powershell
& "$env:USERPROFILE\.cargo\bin\bljack.exe"
```

### Build without installing

To build a local release executable without copying it to your PATH:

```bash
cargo build --release
```

The executable is created at:

- Linux/macOS: `target/release/bljack`
- Windows: `target\release\bljack.exe`

Run it directly with `./target/release/bljack` on Linux/macOS or `target\release\bljack.exe` on Windows.

### Downloading a release

When releases are published, open the repository's **Releases** page, download the archive matching your operating system and CPU architecture, extract it, and run `bljack` (or `bljack.exe` on Windows).

Only download binaries from the project's official GitHub repository. If no release is listed yet, use the source installation instructions above.

---

## How to play

From the main menu, choose a bet with the left/right arrows or the number shortcuts, then press **Enter** or **Space** to deal.

### Table controls

| Key | Action | Details |
|---|---|---|
| `H` / `Space` | Hit | Take another card. |
| `S` / `A` | Stand | Keep the current hand and move on. |
| `D` | Double down | Pay an additional bet, then receive one final card. |
| `P` | Split | Available only when the active hand contains a pair and fewer than four hands exist. Splitting requires another bet of the same size. |
| `I` | Insurance | Available when the dealer shows an Ace. It costs up to half the original bet and pays 2:1 if the dealer has Blackjack. |
| `R` | Surrender | Available in modes that support it; ends the hand for half the bet. |
| `Q` / `Esc` | Leave | Return to the menu or quit from the main menu. |

**Why does Split sometimes do nothing?** The active hand must have exactly two cards of the same rank, such as two 8s. You must also have enough chips for the second bet, and the game allows at most four hands in one round. The action is only available while playing a hand.

### Menu controls

| Key | Action |
|---|---|
| `M` | Next unlocked mode |
| `N` | Previous unlocked mode |
| `B` | Banker |
| `T` | Statistics |
| `A` | Achievements |
| `V` | Developer mode |
| `?` | Help |
| `Q` / `Esc` | Quit |

### Progression and modes

Each completed hand grants XP. Higher levels unlock modes with larger minimum bets and special multipliers:

| Mode | Level | Minimum bet | Multiplier |
|---|---:|---:|---:|
| Classic | 1 | 10 | x1 |
| Double | 2 | 50 | x2 |
| X20 | 3 | 200 | x20 |
| X40 | 4 | 400 | x40 |
| High Roller | 5 | 500 | x10 |
| X80 | 6 | 1,000 | x80 |
| Whale | 10 | 5,000 | x100 |
| X200 | 12 | 10,000 | x200 |
| X400 | 15 | 20,000 | x400 |
| X800 | 20 | 40,000 | x800 |
| X1000 | 25 | 100,000 | x1000 |
| X10000 | 50 | 1,000,000 | x10000 |
| Degenerate | Break the bank | 50,000 | x1000 |

The `M` and `N` keys cycle through the complete mode order. Locked modes are skipped until their requirements are met.

### Banker

Press `B` in the menu to access the bank:

- Deposit and withdraw chips
- Earn interest on savings
- Take loans and manage repayments
- Receive a daily bonus
- Receive a bailout when your playing chips reach zero

### Developer mode

Press `V` and enter `Admin1234` to enable the developer tools. Developer mode is stored in the normal save file, so it remains enabled after restarting the game. It exists for testing and development, not for competitive play.

---

## Save data

The game saves automatically when it exits. On Linux, the save is normally stored at:

```text
~/.local/share/blackjack-tui/save.json
```

The exact location is determined by the platform's standard application-data directory:

- **Linux:** usually `~/.local/share/blackjack-tui/save.json`
- **Windows:** usually `%APPDATA%\blackjack-tui\data\save.json`
- **macOS:** usually `~/Library/Application Support/blackjack-tui/save.json`

Back up this file before manually editing or removing it. Deleting it resets your local progress.

---

## Development

Requirements: Rust stable and Cargo.

```bash
cargo test
cargo build
cargo run
cargo fmt --check
```

To install the current checkout as the global command:

```bash
cargo install --path . --force
```

This project is intentionally used as a practical test project for cake-cli, so APIs, rules, visuals, and documentation may evolve.

## Contributing

Issues, bug reports, documentation improvements, and pull requests are welcome. For a useful bug report, include:

- Operating system and terminal
- Rust version (`rustc --version`)
- Steps to reproduce
- Expected and actual behavior
- A screenshot or terminal log where helpful

Please do not include your personal save file in an issue; it may contain your progression data.

## License

Blackjack TUI is open-source software released under the **MIT License**. See [LICENSE](LICENSE) for the complete license text.

Copyright (c) 2026 Justcakenl
