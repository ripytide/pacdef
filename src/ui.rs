use std::io::{BufRead, Write};

pub(crate) fn get_user_confirmation() -> bool {
    print!("Continue? [Y/n] ");
    std::io::stdout().flush().unwrap();
    let reply = std::io::stdin().lock().lines().next().unwrap().unwrap();
    !(reply.is_empty() || reply.to_lowercase().contains('y'))
}
