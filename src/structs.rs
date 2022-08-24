#[derive(Debug)]
struct User {
    active: bool,
    // https://doc.rust-lang.org/book/ch05-01-defining-structs.html#ownership-of-struct-data
    username: String,
}

impl User {
    // &self is shorthand for self: &Self
    fn username_length(&self) -> usize {
        self.username.len()
    }

    fn get_some_user(active: bool) -> Self {
        User {
            username: String::from("janedoe"),
            active,
        }
    }
}

#[derive(Debug)]
struct RGB(i32, i32, i32);

struct Something;

fn main() {
    let username = String::from("adrianp");

    let mut user = User {
        active: true,
        username,
    };

    println!("username length is: {}", user.username_length());
    println!("username length is: {}", (&user).username_length());

    let jane_doe = User::get_some_user(false);
    println!("{:#?}", jane_doe);

    user.username = String::from("adriantudorp");
    print_user(&user);

    let user2 = User {
        active: false,
        ..user
    };

    // user.username was moved so next line no longer works
    //println!("{}", user.username);

    println!("{}, {}", user2.active, user2.username);

    let blue = RGB(0, 0, 10);
    println!("{}", blue.2);
    dbg!(&blue);
}

fn print_user(user: &User) {
    println!("{:#?}", user);
}
