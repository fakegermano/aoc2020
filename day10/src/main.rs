use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let mut adapters = Vec::new();
    adapters.push(0);
    for line in stdin.lock().lines() {
        let adapter = line.unwrap().parse::<i32>().unwrap();
        adapters.push(adapter);
    }
    adapters.sort();
    adapters.push(adapters[adapters.len()-1] + 3);
    let mut one = 0;
    let mut three = 0;
    println!("{:?}", adapters);

    for i in 1..adapters.len() {
        let dif = adapters[i] - adapters[i-1];
        if dif == 1 {
            one += 1;
        } else if dif == 3 {
            three += 1;
        }
    }

    println!("{} {} {}", one, three, one*three);
}
