// largest is restricted to work on types that implement the PartialOrd trait
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U
}

impl<T: std::fmt::Display, U: std::fmt::Display> Point<T, U> {
    fn print(&self) {
        println!("({},{})", &self.x, &self.y );
    }
}

// this method will only work if both x and y are floats
impl Point<f32, f32> {
    fn dist(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let numbers = [1, 43, 2, 100, 57];
    let largest_number = largest(&numbers);
    println!("{}", largest_number);

    let chars = ['x', 'z', 'a', 'd', 'y'];
    let largest_char = largest(&chars);
    println!("{}", largest_char);

    let integer = Point {x: 1, y:2};
    let float = Point{x: 2.0, y:3.0};
    println!("{:?} {:?}", integer.x, float.y);
    println!("{}", float.dist());

    let floaty = integer.mixup(float);
    println!("{:?}", floaty);
    floaty.print();

}
