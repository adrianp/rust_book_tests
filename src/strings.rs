fn main() {
    // String is a wrapper over Vec<u8> and is mutable
    let s1 = String::from("Hello ");
    // a str is immutable
    let s2 = "world!".to_string();

    // note that the + operator has the following signature: fn add(self, s: &str) -> String {
    // String can be coerced to str
    // s1 has been moved and can no longer be used
    let s3 = s1 + &s2;

    // format does not take any ownership
    let _ = format!("{} {}", s2, s3);

    let uni = "ăă".to_string();
    let len = uni.len();
    let first = &uni[0..2];
    println!("{}, {}", len, first);

    for c in uni.chars() {
        println!("{}", c)
    }

    for b in uni.bytes() {
        println!("{}", b)
    }
}
