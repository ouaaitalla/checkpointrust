#[derive(PartialEq)]
pub enum AccessLevel {
    Admin,
    Normal,
    Guest,
}
#[derive(PartialEq)]
pub struct User {
    name: String,
    level: AccessLevel,
}

impl User {
    pub fn new(name: String, level: AccessLevel) -> Self {
        User { name, level }
    }

    pub fn send_name(&self) -> Option<&str> {
        if self.level == AccessLevel::Admin || self.level == AccessLevel::Normal {
            Some(&self.name)
        } else {
            None
        }
    }
}

pub fn check_user_name(user: &User) -> (bool, &str) {
    let name = user.send_name().unwrap_or("ERROR: User is guest");
    if name == "ERROR: User is guest" {
        (false, name)
    } else {
        (true, name)
    }
}   


fn main() {
    let user0 = User::new("Didier".to_owned(), AccessLevel::Guest);
    println!("{:?}", check_user_name(&user0));

    let user1 = User::new("Mary".to_owned(), AccessLevel::Normal);
    println!("{:?}", check_user_name(&user1));

    let user2 = User::new("John".to_owned(), AccessLevel::Admin);
    println!("{:?}", check_user_name(&user2));
}
