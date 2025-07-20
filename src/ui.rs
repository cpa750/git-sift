use crossterm::event::{self, Event, KeyCode};
use git2::Error;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Position},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use std::{io::Stdout, result::Result, time::Duration};

use crate::{config::Keybinds, finder::filter, Checkout, CheckoutType};

pub struct UI<'a, G>
where
    G: Fn(&String) -> Result<Checkout, Error>,
{
    needle: String,
    haystack: Vec<String>,
    results: Vec<String>,
    selected: usize,
    keybinds: Keybinds,
    on_submit: G,
    terminal: Terminal<CrosstermBackend<&'a mut Stdout>>,
}

impl<'a, G> UI<'a, G>
where
    G: Fn(&String) -> Result<Checkout, Error>,
{
    pub fn new(
        on_submit: G,
        keybinds: Keybinds,
        haystack: Vec<String>,
        terminal: Terminal<CrosstermBackend<&'a mut Stdout>>,
        needle: String,
    ) -> Self {
        let results = filter(&needle, &haystack).unwrap_or_default();

        Self {
            needle,
            haystack,
            results,
            selected: 0,
            keybinds,
            on_submit,
            terminal,
        }
    }

    pub fn run(&mut self) -> Result<(), Error> {
        let mut ui_result: Option<Result<Checkout, Error>> = None;
        self.terminal.clear().unwrap();
        loop {
            if event::poll(Duration::from_millis(16)).unwrap() {
                if let Event::Key(key) = event::read().unwrap() {
                    if key == self.keybinds.quit || key == self.keybinds.quit_alternate {
                        break;
                    }
                    match self.handle_input(key) {
                        Some(v) => {
                            ui_result = Some(v);
                            break;
                        }
                        None => continue,
                    }
                }
            }
            self.terminal.autoresize().unwrap();
            self.terminal
                .draw(|f| draw_ui(f, &self.needle, &self.results, self.selected))
                .unwrap();
        }
        self.terminal.clear().unwrap();
        self.terminal
            .set_cursor_position(Position::new(0, 0))
            .unwrap();
        crossterm::terminal::disable_raw_mode().unwrap();

        if let Some(checkout_result) = ui_result {
            self.print_checkout_result(checkout_result);
        }
        Ok(())
    }

    fn handle_input(&mut self, key: crossterm::event::KeyEvent) -> Option<Result<Checkout, Error>> {
        if key == self.keybinds.next || key == self.keybinds.next_alternate {
            self.selected = (self.selected + 1).min(self.results.len().saturating_sub(1));
        } else if key == self.keybinds.prev || key == self.keybinds.prev_alternate {
            if self.selected > 0 {
                self.selected -= 1;
            }
        } else if key == self.keybinds.submit {
            if let Some(item) = self.results.get(self.selected) {
                return Some((self.on_submit)(item));
            }
        } else if let KeyCode::Char(c) = key.code {
            self.needle.push(c);
            self.results = filter(&self.needle, &self.haystack)?;
        } else if key.code == KeyCode::Backspace {
            self.needle.pop();
            self.results = filter(&self.needle, &self.haystack)?;
        }
        None
    }

    fn print_checkout_result(&self, result: Result<Checkout, Error>) {
        match result {
            Ok(v) => {
                match v.checkout_type {
                    CheckoutType::LOCAL => println!("Successfully checked out {}.", v.branch_name),
                    CheckoutType::REMOTE => {
                        println!("Caution: Checking out a remote ref puts you in a detached HEAD state.\n\
                                  Successfully checked out remote ref {}.", v.branch_name)
                    }
                }
            }
            Err(e) => println!("Error checking out branch: {}", e.message()),
        }
    }
}

fn draw_ui(f: &mut Frame, needle: &str, results: &[String], selected: usize) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Min(1),
        ])
        .split(f.area());

    let search =
        Paragraph::new(needle).block(Block::default().title("Search").borders(Borders::ALL));
    f.render_widget(search, chunks[0]);

    let hint_text =
        "[Tab/Down: Next] [Shift+Tab/Up: Prev] [Enter: Checkout] [Esc/Ctrl+C: Exit]".to_string();
    let hints = Paragraph::new(hint_text);
    f.render_widget(hints, chunks[1]);

    let items: Vec<_> = results
        .iter()
        .enumerate()
        .map(|(i, item)| {
            if i == selected {
                ListItem::new(item.clone()).style(Style::default().add_modifier(Modifier::REVERSED))
            } else {
                ListItem::new(item.clone())
            }
        })
        .collect();

    let list = List::new(items).block(Block::default().title("Results").borders(Borders::ALL));
    f.render_widget(list, chunks[2]);
}
