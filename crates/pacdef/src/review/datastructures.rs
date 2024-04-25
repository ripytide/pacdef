use anyhow::Result;
use std::io::stdout;

use crate::prelude::*;
use crate::review::get_action_for_package;
use crate::ui::get_user_confirmation;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use super::strategy::Strategy;

#[derive(Debug, PartialEq)]
pub enum ReviewAction {
    AsDependency(Package),
    Delete(Package),
    AssignGroup(Package, Group),
}

#[derive(Debug)]
pub enum ReviewIntention {
    AsDependency,
    AssignGroup,
    Delete,
    Info,
    Invalid,
    Skip,
    Quit,
    Apply,
}

#[derive(Debug)]
pub struct ReviewsPerBackend {
    items: Vec<(AnyBackend, Vec<ReviewAction>)>,
}

pub fn old_review(unmanaged_per_backend: ToDoPerBackend, groups: &Groups) -> Result<()> {
    let mut reviews = ReviewsPerBackend::new();

    if unmanaged_per_backend.nothing_to_do_for_all_backends() {
        println!("nothing to do, all installed packages are associated with groups");

        return Ok(());
    }

    'outer: for (backend, packages) in unmanaged_per_backend {
        let mut actions = vec![];
        for package in packages {
            println!("{}: {package}", backend.backend_info().section);
            match get_action_for_package(package, groups, &mut actions, &backend)? {
                ContinueWithReview::Yes => continue,
                ContinueWithReview::No => return Ok(()),
                ContinueWithReview::NoAndApply => {
                    reviews.push((backend, actions));
                    break 'outer;
                }
            }
        }
        reviews.push((backend, actions));
    }

    reviews.run()
}

impl ReviewsPerBackend {
    pub fn new() -> Self {
        Self { items: vec![] }
    }

    pub fn nothing_to_do(&self) -> bool {
        self.items.iter().all(|(_, vec)| vec.is_empty())
    }

    pub fn push(&mut self, value: (AnyBackend, Vec<ReviewAction>)) {
        self.items.push(value);
    }

    pub fn run(self) -> Result<()> {
        if self.nothing_to_do() {
            println!("nothing to do");
            return Ok(());
        }

        let strategies: Vec<Strategy> = self.into_strategies();

        println!();
        let mut iter = strategies.iter().peekable();

        while let Some(strategy) = iter.next() {
            strategy.show();

            if iter.peek().is_some() {
                println!();
            }
        }

        println!();
        if !get_user_confirmation()? {
            return Ok(());
        }

        for strategy in strategies {
            strategy.execute()?;
        }

        Ok(())
    }

    /// Convert the reviews per backend to a vector of [`Strategy`], where one `Strategy` contains
    /// all actions that must be executed for a [`Backend`].
    ///
    /// If there are no actions for a `Backend`, then that `Backend` is removed from the return
    /// value.
    pub fn into_strategies(self) -> Vec<Strategy> {
        let mut result = vec![];

        for (backend, actions) in self {
            let mut to_delete = Packages::new();
            let mut assign_group = vec![];
            let mut as_dependency = Packages::new();

            extract_actions(
                actions,
                &mut to_delete,
                &mut assign_group,
                &mut as_dependency,
            );

            result.push(Strategy::new(
                backend,
                to_delete,
                as_dependency,
                assign_group,
            ));
        }

        result.retain(|s| !s.nothing_to_do());

        result
    }
}

impl IntoIterator for ReviewsPerBackend {
    type Item = (AnyBackend, Vec<ReviewAction>);

    type IntoIter = std::vec::IntoIter<(AnyBackend, Vec<ReviewAction>)>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

pub enum ContinueWithReview {
    Yes,
    No,
    NoAndApply,
}

fn extract_actions(
    actions: Vec<ReviewAction>,
    to_delete: &mut Packages,
    assign_group: &mut Vec<(Package, Group)>,
    as_dependency: &mut Packages,
) {
    for action in actions {
        match action {
            ReviewAction::Delete(package) => {
                to_delete.insert(package);
            }
            ReviewAction::AssignGroup(package, group) => assign_group.push((package, group)),
            ReviewAction::AsDependency(package) => {
                as_dependency.insert(package);
            }
        }
    }
}
