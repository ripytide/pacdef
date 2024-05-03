use anyhow::Result;
use std::io::stdout;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    backend::CrosstermBackend,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, List, Paragraph, Widget},
    Terminal,
};

use crate::prelude::*;

pub fn review(groups: &Groups, config: &Config) -> Result<()> {
    Review::new(groups, config)?.run()
}

#[derive(Debug, Clone)]
struct Review {
    unmanaged: PackagesIds,
    assigned: Groups,
}

#[derive(Debug, Copy, Clone)]
enum ReviewAction {
    Exit,
    Down,
}

impl Review {
    fn new(groups: &Groups, config: &Config) -> Result<Self> {
        let unmanaged = PackagesIds::unmanaged(groups, config)?;

        Ok(Self {
            unmanaged,
            assigned: groups.clone(),
        })
    }

    fn run(mut self) -> Result<()> {
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

        loop {
            terminal.draw(|frame| frame.render_widget(&self, frame.size()))?;

            let review_action = Self::next_review_action()?;

            self.update(review_action);

            if let ReviewAction::Exit = review_action {
                break;
            }
        }

        terminal.clear()?;

        Ok(())
    }

    fn update(&mut self, review_action: ReviewAction) {
        match review_action {
            ReviewAction::Exit => {}
            ReviewAction::Down => todo!(),
        }
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

        List::new(self.unmanaged.iter_strings())
            .block(block.clone())
            .render(layout[0], buf);

        Paragraph::new("Hello World!")
            .centered()
            .block(block.clone())
            .render(layout[1], buf);
    }
}
