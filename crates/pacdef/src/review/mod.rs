mod datastructures;
mod strategy;
#[allow(dead_code)]
mod tui;

use std::io::{stdin, stdout, Write};

use anyhow::Result;

use crate::prelude::*;
use crate::ui::read_single_char_from_terminal;

use self::{
    datastructures::{
        old_review, ContinueWithReview, ReviewAction, ReviewIntention, ReviewsPerBackend,
    },
    tui::Review,
};

pub fn review(unmanaged_per_backend: ToDoPerBackend, groups: &Groups) -> Result<()> {
    Review::new(unmanaged_per_backend, groups).run()
}

fn get_action_for_package(
    package: Package,
    groups: &Groups,
    reviews: &mut Vec<ReviewAction>,
    backend: &AnyBackend,
) -> Result<ContinueWithReview> {
    loop {
        match ask_user_action_for_package(backend.supports_as_dependency())? {
            ReviewIntention::AsDependency => {
                assert!(
                    backend.supports_as_dependency(),
                    "backend does not support dependencies"
                );
                reviews.push(ReviewAction::AsDependency(package));
                break;
            }
            ReviewIntention::AssignGroup => {
                if let Ok(Some(group)) = ask_group(groups) {
                    reviews.push(ReviewAction::AssignGroup(package, group.clone()));
                    break;
                };
            }
            ReviewIntention::Delete => {
                reviews.push(ReviewAction::Delete(package));
                break;
            }
            ReviewIntention::Info => {
                backend.show_package_info(&package)?;
            }
            ReviewIntention::Invalid => (),
            ReviewIntention::Skip => break,
            ReviewIntention::Quit => return Ok(ContinueWithReview::No),
            ReviewIntention::Apply => return Ok(ContinueWithReview::NoAndApply),
        }
    }
    Ok(ContinueWithReview::Yes)
}

/// Ask the user for the desired action, and return the associated
/// [`ReviewIntention`]. The query depends on the capabilities of the backend.
///
/// # Errors
///
/// This function will return an error if stdin or stdout cannot be accessed.
fn ask_user_action_for_package(supports_as_dependency: bool) -> Result<ReviewIntention> {
    print_query(supports_as_dependency)?;

    match read_single_char_from_terminal()?.to_ascii_lowercase() {
        'a' if supports_as_dependency => Ok(ReviewIntention::AsDependency),
        'd' => Ok(ReviewIntention::Delete),
        'g' => Ok(ReviewIntention::AssignGroup),
        'i' => Ok(ReviewIntention::Info),
        'q' => Ok(ReviewIntention::Quit),
        's' => Ok(ReviewIntention::Skip),
        'p' => Ok(ReviewIntention::Apply),
        _ => Ok(ReviewIntention::Invalid),
    }
}

/// Print a space-terminated string that asks the user for the desired action.
/// The items of the string depend on whether the backend supports dependent
/// packages.
///
/// # Errors
///
/// This function will return an error if stdout cannot be flushed.
fn print_query(supports_as_dependency: bool) -> Result<()> {
    let mut query = String::from("assign to (g)roup, (d)elete, (s)kip, (i)nfo, ");

    if supports_as_dependency {
        query.push_str("(a)s dependency, ");
    }

    query.push_str("a(p)ply, (q)uit? ");

    print!("{query}");
    stdout().lock().flush()?;
    Ok(())
}

fn print_enumerated_groups(groups: &Groups) {
    let number_digits = get_amount_of_digits_for_number(groups.len());

    for (i, group) in groups.iter().enumerate() {
        println!("{i:>number_digits$}: {}", group.name);
    }
}

fn get_amount_of_digits_for_number(number: usize) -> usize {
    number.to_string().len()
}

fn ask_group(groups: &Groups) -> Result<Option<&Group>> {
    print_enumerated_groups(groups);
    let mut buf = String::new();
    stdin().read_line(&mut buf)?;
    let reply = buf.trim();

    let idx: usize = if let Ok(idx) = reply.parse() {
        idx
    } else {
        return Ok(None);
    };

    if idx < groups.len() {
        Ok(groups.iter().nth(idx))
    } else {
        Ok(None)
    }
}
