use std::string::String; // Add missing import statement

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    uri: String,
    active: bool,
}

impl User {
    fn new(username: String, email: String, uri: String) -> Self {
        Self {
            username,
            email,
            uri,
            active: true,
        }
    }
    fn from_email(email: String) -> User {
        let email_parts: Vec<&str> = email.split("@").collect();
        User::new(email_parts[0].to_string(), email.to_string(), String::from(""))
    }

    fn update_uri(&mut self, uri: String) {
        self.uri = uri;
    }
    fn deactivate(&mut self) {
        self.active = false;
    }
}

fn main() {
    let mut new_user = User::new(
        String::from("alfredodeza"),
        String::from("alfreodezaçexample.com"),
        String::from("https://alfredodeza.com"),
    );

    let mut tmp_user = User::from_email(
        String::from("tmp_user@gmail.com"),
    );
    println!("Hello, {:?}!", tmp_user);
    tmp_user.update_uri(String::from("https://tmpuser.com"));
    println!("Hello, {:?}!", tmp_user);
   

}
