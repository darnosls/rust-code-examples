fn main() {
    println!("Hello, world!");
    another_function(5);
    function_with_multiple_params(4,3);

    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    // function with return value

    let five = five();
    println!("The value of five funtion is: {}", five);

    let plus_one = plus_one(2);
    println!("The value of plus_one funtion is: {}", plus_one);

}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn function_with_multiple_params(x: i32, y: i32) {
    println!("The value of x in function_with_multiple_params is: {}", x);
    println!("The value of y in function_with_multiple_params is: {}", y);
}


// function with return value

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}