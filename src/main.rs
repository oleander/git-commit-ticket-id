use git2::Repository;
use std::env;
mod lib;

fn main() {
    let current_path = env::current_dir().unwrap();
    let mut args: Vec<String> = env::args().collect();
    let bin = args.remove(0);

    let bin_name = bin.split("/").last().unwrap();

    if args.is_empty() {
        return lib::warn(&format!("Usage: {} [TICKET-ID] [MESSAGE]", bin_name));
    }

    let repo = match Repository::discover(current_path) {
        Err(e) => return lib::err(&format!("{}", e)),
        Ok(repo) => repo,
    };

    let head = repo.head().unwrap();

    // Get current branch
    let branch = match head.shorthand() {
        None => return lib::err("No branch found"),
        Some(branch) => branch,
    };

    let ticket_id = &args[0];
    let mut message = args.join(" ");

    if branch.starts_with("KDB-") && !ticket_id.starts_with("KDB-") {
        if let Some(ticket_id) = lib::extract_ticket_number(&branch) {
            lib::info(&format!("Ticket ID {} will be used", ticket_id));
            message = format!("{} {}", ticket_id, message);
        }
    }

    let oid = head.target().unwrap();
    let parent = repo.find_commit(oid).unwrap();
    let parents = &[&parent];
    let mut index = repo.index().unwrap();
    let tree_id = index.write_tree().unwrap();
    let tree = repo.find_tree(tree_id).unwrap();
    let success = repo
        .statuses(None)
        .unwrap()
        .iter()
        .any(|x| x.status().contains(git2::Status::INDEX_MODIFIED));
    if !success {
        return lib::warn("No changes to commit");
    }

    let user = &repo.signature().unwrap();

    if let Err(msg) = repo.commit(Some("HEAD"), &user, &user, &message, &tree, parents) {
        lib::warn(&msg.to_string())
    }
}
