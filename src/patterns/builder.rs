pub fn builder_pattern() {
    let joey = UserBuilder::new()
        .set_username("joey".to_string())
        .expect("name")
        .set_birthday(27)
        .build()
        .unwrap();

    println!("{} {}", joey.username, joey.birthday);
}

// this is what our builder builds
#[derive(Debug)]
pub struct User {
    username: String,
    birthday: u8,
}

// this is our builder struct
pub struct UserBuilder {
    username: Option<String>,
    birthday: Option<u8>,
}

// error
#[derive(Debug)]
pub struct InvalidUsername;

// error
#[derive(Debug)]
pub enum IncompleteUserBuild {
    NoUsername,
    NoCreatedOn,
}

// methods
impl UserBuilder {
    pub fn new() -> Self {
        Self {
            username: None,
            birthday: None,
        }
    }

    // our methods must include a &mut self arg, it must also return the type &mut Self
    pub fn set_username(&mut self, username: String) -> Result<&mut Self, InvalidUsername> {
        // true if every character is number of lowercase letter in English alphabet
        let valid = username
            .chars()
            .all(|chr| matches!(chr, 'a'..='z' | '0'..='9'));

        if valid {
            self.username = Some(username);
            Ok(self)
        } else {
            Err(InvalidUsername)
        }
    }

    // our methods must include a &mut self arg, it must also return the type &mut Self
    pub fn set_birthday(&mut self, date: u8) -> &mut Self {
        self.birthday = Some(date);
        self
    }

    // the build method must always return our target struct, in this case `User`
    pub fn build(&self) -> Result<User, IncompleteUserBuild> {
        if let Some(username) = self.username.clone() {
            if let Some(birthday) = self.birthday.clone() {
                Ok(User { username, birthday })
            } else {
                Err(IncompleteUserBuild::NoCreatedOn)
            }
        } else {
            Err(IncompleteUserBuild::NoUsername)
        }
    }
}
