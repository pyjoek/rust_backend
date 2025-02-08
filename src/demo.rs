/*
fn main() {
    println!("Hello i'm joel");
}
*/

/*
fn main() {
    let x: i32 = 5;
    let y: i32 = 3;

    let x = x +  9;
    print!("{}",x);
}
*/

// fn main() {
//     let x: u32 = "19".parse().expect("not a number");
//     let i = 10;
//     let t = x + i;
//     print!("{}", t);
// }

// fn main() {
//     let names : (i32, i32, i32) = (3200, 5400, 67);
//     // println!("Tuple: {:?}", names);
//     println!("FIrst value in tuple is: {}", names.0);
// }

// fn main() {
//     let even: [i32; 5] = [1,2,3,4,5];
//     println!("{}", even[3]);
// }

use std::io;
fn main() {
    let numbers = [1,2,3,4,5];
    println!("Enter index number:");

    let mut guest = String::new();
    io::stdin().read_line(&mut guest).expect("failed to read line");
    let index: usize = guest.trim().parse().expect("invalid number");
    println!("The index of {} in the list is {}", guest, numbers[index]);
}

