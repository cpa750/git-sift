use crossterm::event::KeyEvent;

pub struct Keybinds {
    pub next: KeyEvent,
    pub next_alternate: KeyEvent,
    pub prev: KeyEvent,
    pub prev_alternate: KeyEvent,
    pub submit: KeyEvent,
    pub quit: KeyEvent,
    pub quit_alternate: KeyEvent,
}
