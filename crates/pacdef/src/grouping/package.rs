use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fmt::{Display, Write};

pub type Packages = BTreeSet<Package>;

/// A struct to represent a single package, consisting of a `name`, and
/// optionally a `repo`.
#[derive(Debug, Clone)]
pub struct Package {
    /// The name of the package
    pub name: String,
    /// Optionally, which repository the package belongs to
    pub repo: Option<String>,
}

fn remove_comment_and_trim_whitespace(s: &str) -> &str {
    s.split('#') // remove comment
        .next()
        .expect("line contains something")
        .trim() // remove whitespace
}

impl From<String> for Package {
    fn from(value: String) -> Self {
        let trimmed = remove_comment_and_trim_whitespace(&value);
        debug_assert!(!trimmed.is_empty(), "empty package names are not allowed");

        let (name, repo) = Self::split_into_name_and_repo(trimmed);
        Self { name, repo }
    }
}

impl From<&str> for Package {
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}

impl Package {
    /// From a string that contains a package name, optionally prefixed by a
    /// repository, return the package name as well as the repository if it
    /// exists.
    ///
    /// # Panics
    ///
    /// Panics if `string` is empty.
    fn split_into_name_and_repo(string: &str) -> (String, Option<String>) {
        if let Some((before, after)) = string.split_once('/') {
            (after.to_string(), Some(before.to_string()))
        } else {
            (string.to_string(), None)
        }
    }

    /// Try to parse a string (from a line in a group file) and return a package.
    /// From the string, any possible comment is removed and whitespace is trimmed.package
    /// Returns `None` if there is nothing left after trimming.
    pub fn try_from<S>(s: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        let trimmed = remove_comment_and_trim_whitespace(s.as_ref());
        if trimmed.is_empty() {
            return None;
        }

        let (name, repo) = Self::split_into_name_and_repo(trimmed);
        Some(Self { name, repo })
    }
}

impl PartialEq for Package {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}
impl Eq for Package {}
impl PartialOrd for Package {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Package {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name
            .cmp(&other.name)
            .then(self.repo.as_ref().map_or(Ordering::Equal, |self_repo| {
                other
                    .repo
                    .as_ref()
                    .map_or(Ordering::Equal, |other_repo| self_repo.cmp(other_repo))
            }))
    }
}

impl Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.repo {
            None => (),
            Some(repo) => {
                f.write_str(repo)?;
                f.write_char('/')?;
            }
        }
        f.write_str(&self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::Package;

    #[test]
    fn split_into_name_and_repo() {
        let x = "repo/name".to_string();
        let (name, repo) = Package::split_into_name_and_repo(&x);
        assert_eq!(name, "name");
        assert_eq!(repo, Some("repo".to_string()));

        let x = "something".to_string();
        let (name, repo) = super::Package::split_into_name_and_repo(&x);
        assert_eq!(name, "something");
        assert_eq!(repo, None);
    }

    #[test]
    fn from() {
        let x = "myrepo/somepackage  #  ".to_string();
        let p = Package::try_from(x).expect("this should be a valid package line");
        assert_eq!(p.name, "somepackage");
        assert_eq!(p.repo, Some("myrepo".to_string()));
    }
}
