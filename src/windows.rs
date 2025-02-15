mod linux;
use linux::unix::hello;

fn main() {
    let hi = hello();
    println!("{:?}", hi);
}