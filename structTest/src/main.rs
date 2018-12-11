
fn main() {
    let user1 = User {
        username: String::from("myUsername"),
        signin_count: 0,
        active: true,
        email: String::from("drew@email.com"),
        int1: 4,
        int2: 7
    };
    //let s1 = user.username;
    println!("Username: {}", user1.username);
    println!("Debug test: {:#?}", user1);
    println!("Testing method: {}", user1.multiply());
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    signin_count: i32,
    active: bool,
    int1: i32,
    int2: i32
}

impl User {
    fn multiply(&self) -> i32 {
        self.int1 * self.int2
    }
}





//testing mutability of String

/*
fn main() {
    let mut string = String::from("test");
    println!("My string: {}", string);
    modify_str(&mut string);
    println!("Last string check: {}", string);
    println!("Length: {}", get_len(&string));
    println!("One last string check: {}", string);
}

fn modify_str(s: &mut String) {
    println!("My string inside the function: {}", s);
    s.push_str(" more input");
    println!("Another check: {}", s);
}

fn get_len(s: &String) -> usize {
    s.len()
}
*/

