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

    let r1 = &mut s2;
    let size = string_length(r1);
    println!("{}, {}", s2, size);

}

fn takes_ownership(some_string: String) {
    println!("taking: {}", some_string);
}

fn takes_and_gives_back(some_string: String) -> String {
    some_string
}

fn string_length(some_string: &mut String) -> usize {
    // we need to clone as appending might cause the string to be relocated in the memory and we would lose the reference to it
    // https://stackoverflow.com/questions/47618823/cannot-borrow-as-mutable-because-it-is-also-borrowed-as-immutable
    some_string.push_str(some_string.clone().as_str());
    some_string.len()
}

fn _nll() {
    let mut x = 5;
    let _y = &x;
    let _z = &mut x;
    //println!("_y: {}", _y);
}
