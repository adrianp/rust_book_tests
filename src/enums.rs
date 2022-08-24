enum IpAddr {
    V4(String),
    V6(String),
}

impl IpAddr {
    fn ping(&self) {}
}

fn main() {
    let four = IpAddr::V4(String::from("127.0.0.1"));
    let six = IpAddr::V6(String::from("::1"));
    route(four);
    route(six);

    // Option<T> is included in the prelude, no need to bring it into scope
    let something = Some(5);
    let nothing: Option<i32> = None;

    let x = 5;
    let y: Option<i8> = Some(5);
    let result = match y {
        None => x,
        Some(i) => x + i,
        _ => x, // assuming this was reachable, this would be the fallback for the match
    };
    println!("{}", result);

    if let Some(i) = y {
        println!("{}", x + i);
    } else {
        println!("{}", x);
    }
}

fn route(ip: IpAddr) {}
