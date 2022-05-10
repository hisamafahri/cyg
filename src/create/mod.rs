use crate::cmd;
mod group;
mod user;

pub fn create(resource: &cmd::Resource) {
    match resource.resource.as_str() {
        "group" => group::group(),
        "user" => user::user(),
        _ => {
            println!("error: input wasn't recognized\n[possible values: group, user]")
        }
    }
}
