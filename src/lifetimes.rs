/*
fn borrow() {
    // r is not null, if not initialized a compile-time error will be thrown
    let r;
    {
        let x = 5;
        // borrowed value does not live long enough
        r = &x;
    }
    println!("{}", r);
}
*/

#[derive(Debug)]
// lifetimes are a type of generic
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // first the compiler will assign each parameter its own lifetime; then it
    // will assign the output parameter the lifetime of &self
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// the reference returned by this function will be valid for as long as both
// parameters are valid ('a is the lifetime)
// it's also worth noting that the compiler cannot determine lifetimes
// automatically because we have two input lifetimes
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);

    let i;

    let novel = String::from("This. is. a. test.");
    i = ImportantExcerpt {
        part: novel.split('.').next().expect("No fullstop found")
    };

    println!("{:?}", i)
}

#[test]
fn test_longest() {
    let mut result = longest("abc", "ab");
    assert_eq!(result, "abc");

    result = longest("ab", "abc");
    assert_ne!(result, "ab");

    result = longest("abc", "abc");
    assert_eq!(result, "abc");

    result = longest("", "");
    assert_eq!(result, "");
}

#[test]
fn test_longest_fail() {
    let result = longest("abc", "ab");
    assert_eq!(result, "ab", "This is a custom failure message");
}

