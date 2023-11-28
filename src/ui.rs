use crate::fs::Dir;
use crossterm::ExecutableCommand;
use ratatui::{
    prelude::*,
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState},
};
use std::{error, io, path::Path};

#[derive(Debug)]
pub struct Ui {
    pub dir: Dir,
    pub length: u32,
    pub idx: i32,
    pub horizontal: i32,
    pub size: Rect,
    pub terminal: Terminal<CrosstermBackend<io::Stderr>>,
}

impl Ui {
    pub fn new() -> Result<Self, Box<dyn error::Error>> {
        io::stdout().execute(crossterm::terminal::EnterAlternateScreen)?;
        crossterm::terminal::enable_raw_mode()?;
        let terminal = Terminal::new(CrosstermBackend::new(io::stderr()))?;
        let binding = std::env::current_dir()?;
        let current = binding.as_path();
        let dir = Dir::from(current)?;
        Ok(Self {
            dir,
            length: 0,
            idx: 0,
            horizontal: 0,
            size: terminal.backend().size()?,
            terminal,
        })
    }

    pub fn from(path: &str) -> Result<Self, Box<dyn error::Error>> {
        io::stdout().execute(crossterm::terminal::EnterAlternateScreen)?;
        crossterm::terminal::enable_raw_mode()?;
        let terminal = Terminal::new(CrosstermBackend::new(io::stderr()))?;
        Ok(Self {
            dir: Dir::from(Path::new(path))?,
            length: 0,
            idx: 0,
            horizontal: 0,
            size: terminal.backend().size()?,
            terminal,
        })
    }

    pub fn clear(&mut self) {
        self.terminal.clear().unwrap();
    }

    pub fn display(&mut self, path: &str) {
        self.dir.order_alphabetically();
        let mut list_items = Vec::<ListItem>::new();

        self.dir
            .files
            .iter()
            .enumerate()
            .filter(|(i, _f)| i32::try_from(*i).unwrap() >= self.idx)
            .for_each(|(i, f)| {
                let (item_text, text_color, background_color) = if f.is_dir {
                    (
                        format!("{} {} {}", f.permissions, f.name, f.last_modified),
                        Color::Blue,
                        if self.idx == i.try_into().unwrap() {
                            Color::White
                        } else {
                            Color::Reset
                        },
                    )
                } else {
                    (
                        format!(
                            "{} {} {}bytes {}",
                            f.permissions, f.name, f.size, f.last_modified
                        ),
                        Color::Green,
                        if self.idx == i.try_into().unwrap() {
                            Color::White
                        } else {
                            Color::Reset
                        },
                    )
                };

                list_items.push(ListItem::new(Line::from(Span::styled(
                    item_text,
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .fg(text_color)
                        .bg(background_color), // Set background color here
                ))));
            });

        let list = List::new(list_items);

        self.terminal
            .draw(|f| {
                f.render_widget(
                    list.block(
                        Block::default()
                            .add_modifier(Modifier::BOLD)
                            .fg(Color::Cyan)
                            .title(path)
                            .borders(Borders::ALL),
                    ),
                    f.size(),
                )
            })
            .unwrap();
    }

    /*pub fn display(&mut self) -> Result<ratatui::prelude::CompletedFrame<'_>, io::Error> {
        let files = self.dir.files.clone();
        self.terminal.draw(|f| {
            files.into_iter().for_each(|w| {
                f.render_widget(Paragraph::new(w.name), f.size());
            });
        })
    } */

    pub fn display_title(&self) {}

    pub fn finish(&self) -> Result<(), Box<dyn error::Error>> {
        crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
        crossterm::terminal::disable_raw_mode()?;
        Ok(())
    }
}
