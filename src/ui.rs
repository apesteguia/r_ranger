use crate::fs::Dir;
use crossterm::ExecutableCommand;
use ratatui::{
    prelude::{CrosstermBackend, Terminal},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, List, ListItem, Paragraph},
};
use std::{error, io, path::Path};

#[derive(Debug)]
pub struct Ui {
    pub dir: Dir,
    pub length: u32,
    pub idx: u32,
    pub horizontal: i32,
    pub terminal: Terminal<CrosstermBackend<io::Stderr>>,
}

impl Ui {
    pub fn new() -> Result<Self, Box<dyn error::Error>> {
        io::stdout().execute(crossterm::terminal::EnterAlternateScreen)?;
        crossterm::terminal::enable_raw_mode()?;
        Ok(Self {
            dir: Dir::from(Path::new("/home/mikel/Escritorio"))?,
            length: 0,
            idx: 0,
            horizontal: 0,
            terminal: Terminal::new(CrosstermBackend::new(io::stderr()))?,
        })
    }

    pub fn clear(&mut self) {
        self.terminal.clear().unwrap();
    }

    pub fn display(&mut self) {
        let mut list_items = Vec::<ListItem>::new();

        self.dir.files.iter().for_each(|f| {
            list_items.push(ListItem::new(Line::from(Span::styled(
                format!("{}", f.name),
                Style::default().fg(Color::Blue),
            ))))
        });

        let list = List::new(list_items);

        self.terminal
            .draw(|f| f.render_widget(list, f.size()))
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

    pub fn finish(&self) -> Result<(), Box<dyn error::Error>> {
        crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
        crossterm::terminal::disable_raw_mode()?;
        Ok(())
    }
}