fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();

    println!("{}",spaces);

    println!("Variaveis");

    let num: i32 = 2_000_000_999;

    println!("num = {}", num);

    println!("let tup = (500, 6.4, 1) =================");
    let tup = (500, 6.4, 1);

    println!("The value of tup is: {:?}", tup);

    println!("Position 0: [{}]", tup.0);
    println!("Position 1: [{}]", tup.1);
    println!("Position 2: [{}]", tup.2);

    println!("let (x, y, z) = tup ======================");
    let (x, y, z) = tup;

    println!("The value of x is: [{}]", x);
    println!("The value of y is: [{}]", y);
    println!("The value of z is: [{}]", z);

    // let a: [i32; 5];

    println!("let a = [1,2,3,4,5] =====================");
    let a = [1,2,3,4,5];

    println!("The value of a is: {:?}", a);
    // println!("The value of a is: {:#?}", a);
    println!("Value of a[0] = {}", a[0]);

}
