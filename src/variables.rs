fn main() {

    // shadowing
 
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");


    let y:i8 = 127;
    let result = overflows(y);
    println!("returned {result}");

    let z = {
        let x = 3;
        x + 1
    };
    println!("{z}");

    let number = if true {3} else {4};
    println!("{number}");

}

fn overflows(y: i8) -> i8 {
    let result = y.wrapping_add(1);
    println!("{result}");

    let result = y.checked_add(1);
    println!("{result:?}");

    let tup: (i8, bool) = y.overflowing_add(1);
    println!("{}, {}", tup.0, tup.1);

    let result = y.saturating_add(1);
    println!("{result}");

    y
}
