use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("test", 1);
    scores.entry("test").or_insert(50);
    let score = scores.get("test");

    if let Some(i) = score {
        println!("{}", i)
    }

    let test = scores.get("something");
    println!("{}", test.unwrap_or_else(|| &100));

    for (k, v) in &scores {
        println!("{}: {}", k, v);
    }

    let k = String::from("foo");
    let v = String::from("bar");
    let mut mappy = HashMap::new();
    mappy.insert(k, v);
    // k and v are moved now
    //println!("{}, {}", k, v);

    let text = "The quick Brown fox jumps over the lazy brown dog";
    let mut words = HashMap::new();
    for word in text.split_whitespace() {
        let count = words.entry(word.to_lowercase()).or_insert(0);
        *count += 1;
    }
    println!("{:?}", words);

    let unique = HashSet::from([1, 1, 2, 3, 4, 4]);
    println!("{:?}", unique);

}
