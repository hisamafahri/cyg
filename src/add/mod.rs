use crate::cmd;

mod user;
mod file;

pub fn add(resource: &cmd::AddResource) {
    match resource.resource.as_str() {
        "file" => file::file(),
        "user" => user::user(),
        _ => {
            println!("error: input wasn't recognized\n[possible values: file, user]")
        }
    }
}
