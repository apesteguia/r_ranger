use crossterm::event::{poll, read, Event, KeyCode, KeyEventKind};

use crate::ui::Ui;

use std::error;

#[derive(Debug)]
pub struct App {
    ui: Ui,
}

impl App {
    pub fn new() -> Result<Self, Box<dyn error::Error>> {
        Ok(Self { ui: Ui::new()? })
    }

    pub fn from(path: &str) -> Result<Self, Box<dyn error::Error>> {
        Ok(Self {
            ui: Ui::from(path)?,
        })
    }

    pub fn run(&mut self) -> Result<(), Box<dyn error::Error>> {
        self.ui.clear();
        self.ui.display(
            self.ui
                .dir
                .current_path
                .clone()
                .to_str()
                .unwrap_or_default(),
        );

        loop {
            if poll(std::time::Duration::from_millis(16))? {
                if let Event::Key(key) = read()? {
                    if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                        break;
                    } else if key.kind == KeyEventKind::Press
                        && key.code == KeyCode::Char('j')
                        && self.ui.idx < self.ui.dir.files.len().try_into().unwrap()
                    {
                        self.ui.idx += 1;
                        self.ui.display(
                            self.ui
                                .dir
                                .current_path
                                .clone()
                                .to_str()
                                .unwrap_or_default(),
                        );
                    } else if key.kind == KeyEventKind::Press
                        && key.code == KeyCode::Char('k')
                        && self.ui.idx > 0
                    {
                        self.ui.idx -= 1;
                        self.ui.display(
                            self.ui
                                .dir
                                .current_path
                                .clone()
                                .to_str()
                                .unwrap_or_default(),
                        );
                    } else if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('l') {
                        if let Some((_index, selected_dir)) = self
                            .ui
                            .dir
                            .files
                            .iter()
                            .enumerate()
                            .find(|(i, f)| f.is_dir && *i == self.ui.idx.try_into().unwrap())
                        {
                            self.ui.idx = 0;
                            let selected_dir_name = &selected_dir.name;
                            let path = self.ui.dir.current_path.join(selected_dir_name);
                            self.ui.dir.get_dir(&path);
                            self.ui.dir.current_path = path;
                        }
                        self.ui.display(
                            self.ui
                                .dir
                                .current_path
                                .clone()
                                .to_str()
                                .unwrap_or_default(),
                        );
                    } else if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('h') {
                        if self.ui.dir.last_dir_path.is_dir() {
                            self.ui.idx = 0;
                            self.ui.dir.father_path();
                            self.ui.dir.get_dir(&self.ui.dir.last_dir_path.clone());
                            self.ui.dir.current_path = self.ui.dir.last_dir_path.clone();
                        }
                        self.ui.display(
                            self.ui
                                .dir
                                .current_path
                                .clone()
                                .to_str()
                                .unwrap_or_default(),
                        );
                    }
                }
            }
        }

        self.ui.finish().unwrap();

        Ok(())
    }
}
