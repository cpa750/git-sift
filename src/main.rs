mod config;
mod finder;
mod git;
mod ui;

use crate::config::Keybinds;
use crate::git::*;
use crate::ui::UI;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use git2::Error;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::stdout;
use std::result::Result;

fn main() -> Result<(), Error> {
    let git = GitManager::new();
    let all_branches: &Vec<String> = &git
        .local_branches
        .iter()
        .cloned()
        .chain(git.remote_branches.iter().cloned())
        .collect();

    let keybinds = Keybinds {
        next: KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE),
        next_alternate: KeyEvent::new(KeyCode::Down, KeyModifiers::NONE),
        prev: KeyEvent::new(KeyCode::BackTab, KeyModifiers::SHIFT),
        prev_alternate: KeyEvent::new(KeyCode::Up, KeyModifiers::NONE),
        submit: KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE),
        quit: KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE),
        quit_alternate: KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL),
    };
    let mut stdout = stdout();
    crossterm::terminal::enable_raw_mode().unwrap();
    let backend = CrosstermBackend::new(&mut stdout);
    let terminal = Terminal::new(backend).unwrap();
    let mut ui = UI::new(
        |branch: &String| git.checkout(branch),
        keybinds,
        all_branches.to_vec(),
        terminal,
    );
    ui.run()
}
