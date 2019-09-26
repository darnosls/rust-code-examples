fn main() {
    let mut counter = 0;

    // loop
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    //while
    let mut number_a = 3;

    while number_a != 0 {
        println!("{}!", number_a);

        number_a -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        print!(" {} ", a[index]);

        index += 1;
    }

    println!("");
    println!("");

    // for
    for element in a.iter() {
        print!(" {} ", element);
    }

    println!("");

}
