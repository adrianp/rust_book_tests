#[derive(Debug)]
struct User {
    active: bool,
    // https://doc.rust-lang.org/book/ch05-01-defining-structs.html#ownership-of-struct-data
    username: String,
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
