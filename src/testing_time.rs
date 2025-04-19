use std::time::Instant;

fn main() {
    println!("THe current time is{:?}", Instant::now());
    let mut total = 0;
    for i in 1..11 {
        total += i;
    }
    
    println!("Total of 100 number s is {}", total);
    println!("THe current time is{:?}", Instant::now())

}
