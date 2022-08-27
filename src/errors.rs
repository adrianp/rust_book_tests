use std::fs::File;
use std::io::{BufReader, ErrorKind, Error};
use std::io::prelude::*;
use std::error;


fn main() -> Result<(), Box<dyn error::Error>> {
    //panic!("bye");

    let _v = vec![1, 2, 3];
    // Rust does not allow buffer overreads
    //_v[90];

    let path = "README.md";
    let f = File::open(path);
    match f {
        Ok(file) => {
            let mut buffer = BufReader::new(file);
            let mut contents = String::new();
            match buffer.read_to_string(&mut contents) {
                Ok(_) => println!("{:?}", last_char_of_first_line(contents)),
                Err(error) => panic!("{:?}", error),
            }
        },
        Err(error) => {
            if error.kind() == ErrorKind::NotFound {
                File::create(path).unwrap_or_else(|error| {
                    panic!("Could not create file: {:?}", error);
                });
            } else {
                panic!("{:?}", error);
            }
        }
    };

    File::open("hello.txt").expect_err("Could not open file!");

    match read_from_file() {
        Ok(username) => println!("{}", username),
        Err(error) => println!("{}", error),
    }

    File::open("hello.txt")?;
    return Ok(());
}

fn read_from_file() -> Result<String, Error> {
    // this functions could be rewritten as fs::read_to_string("hello.txt");

    let mut username = String::new();

    // ? performs early returns
    File::open("hello.txt")?.read_to_string(&mut username)?;

    return Ok(username);
}

fn last_char_of_first_line(text: String) -> Option<char> {
    // ? can be used with functions returning both Result or Option, but it won't convert between
    // the two
    text.lines().next()?.chars().last()
}
