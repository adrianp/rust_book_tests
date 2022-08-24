fn main() {
    let l = [1, 2, 3];
    let mut v : Vec<i32> = Vec::from(l);
    v.push(4);

    let mut first = v[0];
    first += 1;
    println!("{}", first);
    println!("{}", v[0]);

    v.push(5);

    // this works because first is not a reference; remember that pushing into the vector might
    // move it to a different memory address
    println!("{}", first);

    let something: Option<&i32> = v.get(100);
    match something {
        Some(something) => println!("The element is {}", something),
        None => println!("No element"),
    }

    for i in &mut v {
        *i += 10;
    }

    println!("{:?}", v);

    #[derive(Debug)]
    enum Animal {
        Cat(i32, String),
        Dog(i32, String),
    }
    let zoo = vec![Animal::Cat(1, String::from("Tom")), Animal::Cat(2, String::from("Tommy")), Animal::Dog(3, String::from("Spike"))];
    println!("{:?}", zoo);
}
