use crate::cmd;

pub fn create(resource: &cmd::Resource) {
    match resource.resource.as_str() {
        "group" => println!("Group called"),
        "user" => println!("User is called"),
        _ => {
            println!("error: input wasn't recognized\n[possible values: group, user]")
        }
    }
}
