#![feature(inclusive_range_syntax)]

fn main() {
    for i in 1...10 { 
        print!("{} ", i);
    }
    println!("");
    // output: 1 2 3 4 5 6 7 8 9 10
}