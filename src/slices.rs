fn first_word(s: &str) -> &str {
    let chars = s.chars();
    for (i, c) in chars.enumerate() {
        if c == ' ' {
            return &s[0..i];
        }
    }

    return &s[..];
}

fn main() {
    // note that we can send String to a function expecting &str due to deref coercions
    let my_string = String::from("hello world!");
    let result = first_word(&my_string);
    println!("{:?}", result);


    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}
