use std::io;

fn main() {

    // compound();
    // numbers();
    // another_function(5, 8);
    expressions();
}


fn expressions(){
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}


fn another_function(x: i32, y:u32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn compound() {
    //The Tuple Type
    println!("The Tuple Type");
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    println!("The values of is:\nfive_hundred: {}\nsix_point_four: {}\none: {}",
             five_hundred, six_point_four, one);


    //The Array Type
    println!("The Array Type");
    /*
     let a = [1, 2, 3, 4, 5];
     let months = ["January", "February", "March", "April", "May", "June", "July",
         "August", "September", "October", "November", "December"];
     let b: [i32; 5] = [1, 2, 3, 4, 5];

     let same_values_for_all_elements = [3; 5];

     let ar = [1, 2, 3, 4, 5];
     let first = ar[0];
     let second = ar[1];
     */

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}


fn numbers() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
// ----------

    let y = 7;
    let y = y * 1;
    let y = y * 2;

    println!("The value of y is: {}", y);
    //---- shadow

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of space is: {}", spaces);

    // FAIL
    // let mut spaces = "   ";
    // spaces = spaces.len();
}


