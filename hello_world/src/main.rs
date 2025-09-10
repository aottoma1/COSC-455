fn main() {
    let mut a = 5;
    println!("a is {}", a);
    a = 10;
    let b = 5;
    println!("a is {}", a);
    let x: f32 = 10.389431312;
    println! ("x is {}", x);

    let c = a +b;
    println!("c is {}", c);


    let value = 0b1111_0000;

    println!("the value is {}", value);
    println!("the value in binary is {:08b}", value);

    let t = true;
    let f = false;

    println!("t is {} and f is {}", t, f);
    println!("NOT t is {}", !t);
    println!("t AND f is {}", t & f);
    println!("t OR f is {}", t | f);

    let m = (t & f) ^ (t | f);
    println!("the value of the xor statement is {}", m);

    let letter = 'a';
    let number = '1';
    let finger = '\u{261D}';
    println!("{}\n{}\n{}", letter, number, finger);

    // find average of 3 numbs with diff data types

    let m = 10;
    let n = 35.5;
    let o: f32 = 120.0;

    let average = (m as f32 + n as f32 + o)/3.0;
    println!("the average of the 3 numbers is {}", average);
}
