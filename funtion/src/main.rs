fn main() {
    print_values(10, 13); // Statement
    let sum_val: i32 = add_values(10, 13); //Expression
                                           //In you can think about two things Statement and Expression;
                                           //Statement performs actions but do not return value
                                           //Expression does return value
    println!("Sum of two numbers {}", sum_val);

    //Learn Control Flow
    // IN Rust if else condition must be boolean
    let number: i32 = 10;
    if number > 0 {
        // This would give error if (Statement) is not boolean try removing > 0
        println!("Number is positive")
    } else if number < 0 {
        println!("Number is negative")
    } else {
        println!("Number is 0")
    }
    let condition: bool = true;
    let number: i32 = if condition { 5 } else { 10 };

    //LOOP
    // Basic loop would not stop until we call break in {};
    loop {
        println!("Dont uncomment the break statement below");
        break;
    }

    //loop with return value
    let mut counter: i32 = 0;
    let result: i32 = loop {
        counter += 1;
        if counter == 10 {
            break counter; // this will stop and return counter
        }
    };

    //while loop
    while counter > 100 {
        counter -= 1;
        println!("While we wait");
    }

    //For Loop
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    for element in arr.iter() {
        println!("Arr element value {}", element);
    }

    //We can use (...) iterator also
    for element in 1..5 {
        println!("Iterated value {}", element);
    }
}

//Rust suggest function name should be lower case with (_)
//EX - fn RustIsAmazing not valid, rust_is_amazing is valid

fn print_values(x: i32, y: i32) {
    println!("Value of x is {}", x);
    println!("Value of y is {}", y);
}

fn add_values(x: i32, y: i32) -> i32 {
    let sum: i32 = x + y;
    // you can return even without return keyword, just add it at last
    // Ex - replace return sum with just sum without ; at end
    // return sum;
    sum
}
