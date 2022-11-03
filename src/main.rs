mod user;
use dotenv::dotenv;
use std::io;
use user::User;

fn main() {
    dotenv().ok();

    let mut users: Vec<User> = Vec::new();

    loop {
        let mut input = String::new();

        println!(
            "\nSelect an option: \n1 - Create new user\n2 - List all users\n3 - Login\n4 - Exit"
        );

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim() {
            "1" => {
                users.push(create_user());
                println!("User created!")
            }
            "2" => list_users(&users),
            "3" => login(&users),
            "4" => break,
            _ => {
                println!("Invalid selection. Try again.");
                continue;
            }
        }
    }
}

fn create_user() -> User {
    println!("\n========User creation========");

    let mut username = String::new();
    let mut email = String::new();
    let mut password = String::new();

    println!("Enter username:");
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read line");

    println!("Enter email:");
    io::stdin()
        .read_line(&mut email)
        .expect("Failed to read line");

    println!("Enter password:");
    io::stdin()
        .read_line(&mut password)
        .expect("Failed to read line");

    User::new(
        username.trim().to_string(),
        email.trim().to_string(),
        password.trim().to_string(),
    )
}

fn list_users(users: &Vec<User>) {
    if users.len() == 0 {
        println!("There are any registered users!");
        return;
    }

    println!("\nThere are {} registered users:", users.len());

    for user in users {
        println!(
            "
        =======================
        Username: {}
        Email: {}
        Active: {}
        Created At: {}
        =======================
        ",
            user.username, user.email, user.active, user.created_at
        );
    }
}

fn login(users: &Vec<User>) {
    let mut username = String::new();
    let mut password = String::new();

    println!("\nEnter your username:");
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read line");

    println!("Enter your password:");
    io::stdin()
        .read_line(&mut password)
        .expect("Failed to read line");

    let mut found_user: Option<&User> = None;

    for user in users.iter() {
        if user.auth(&username, &password) {
            println!("Successful login. Welcome {}!", user.username);
            found_user = Some(user);
        }
    }

    if let None = found_user {
        println!("Incorrect credentials.");
    }
}
