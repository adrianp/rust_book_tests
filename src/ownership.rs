fn main() {
    let mut s1 = String::from("helllo");
    let mut s2 = s1.clone();  // if we didn't clone, s1 would be moved and s2 would be invalid
    s1.push_str(" world");
    println!("{}, {}", s1, s2);

    takes_ownership(s1);
    // s1 is not longer valid here
    // println!("{}", s1);
    
    s2 = takes_and_gives_back(s2);
    println!("{}", s2);
}

fn takes_ownership(some_string: String) {
    println!("taking: {}", some_string);
}

fn takes_and_gives_back(some_string: String) -> String {
    some_string
}
