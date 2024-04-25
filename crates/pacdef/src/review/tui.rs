use anyhow::Result;
use std::{collections::BTreeMap, io::stdout};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    backend::CrosstermBackend,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, List, Paragraph, Widget},
    Frame, Terminal,
};

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Review {
    unmanaged: ToDoPerBackend,
    assigned: BTreeMap<String, Packages>,
}

#[derive(Debug, Clone)]
pub enum ReviewAction {
    Exit,
}

impl Review {
    pub fn new(unmanaged: ToDoPerBackend, groups: &Groups) -> Self {
        Self {
            unmanaged,
            assigned: groups
                .iter()
                .map(|x| (x.name.clone(), Packages::new()))
                .collect(),
        }
    }

    pub fn run(mut self) -> Result<()> {
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

        loop {
            terminal.draw(|frame| self.render_frame(frame))?;

            let review_action = Self::next_review_action()?;

            let should_quit = self.update(review_action);

            if should_quit {
                break;
            }
        }

        Ok(())
    }

    fn update(&mut self, review_action: ReviewAction) -> bool {
        match review_action {
            ReviewAction::Exit => return true,
        };

        false
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn next_review_action() -> Result<ReviewAction> {
        loop {
            let event = event::read()?;

            if let Some(review_action) = Self::try_parse_event(event) {
                return Ok(review_action);
            }
        }
    }
    fn try_parse_event(event: Event) -> Option<ReviewAction> {
        match event {
            Event::Key(key_event)
                if key_event.kind == KeyEventKind::Press
                    && matches!(key_event.code, KeyCode::Esc | KeyCode::Char('q')) =>
            {
                Some(ReviewAction::Exit)
            }
            _ => None,
        }
    }
}

impl Widget for &Review {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::new(
            Direction::Horizontal,
            [Constraint::Fill(1), Constraint::Fill(1)],
        )
        .split(area);

        let block = Block::default().borders(Borders::all());

        List::new(self.unmanaged.combined_iter())
            .block(block.clone())
            .render(layout[0], buf);

        Paragraph::new("Hello World!")
            .centered()
            .block(block.clone())
            .render(layout[1], buf);
    }
}
