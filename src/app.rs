use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::ui::Ui;

use std::{error, io};

#[derive(Debug)]
pub struct App {
    ui: Ui,
}

impl App {
    pub fn new() -> Result<Self, Box<dyn error::Error>> {
        Ok(Self { ui: Ui::new()? })
    }

    pub fn run(&mut self) -> Result<(), Box<dyn error::Error>> {
        self.ui.clear();
        self.ui.display(); // Initial display

        loop {
            if poll(std::time::Duration::from_millis(16))? {
                if let Event::Key(key) = read()? {
                    if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                        break;
                    }
                }
            }
        }

        self.ui.finish().unwrap();

        Ok(())
    }
}
