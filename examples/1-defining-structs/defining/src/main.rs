
#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    email: String,
    phone_number: String,
    age: u8,
}

impl Person {
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

fn main() {
    println!("{:?}", Person {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            age: 25,
            email: "john-doe@foo.com".to_string(),
            phone_number: "123-456-7890".to_string(),
        }.full_name()
    );
}