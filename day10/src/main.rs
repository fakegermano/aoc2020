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
    //println!("{:?}", adapters);

    for i in 1..adapters.len() {
        let dif = adapters[i] - adapters[i-1];
        match dif {
            1 => one += 1,
            2 => {},
            3 => three += 1,
            _ => panic!("shouldn't happen")
        }
    }

    println!("{} {} {}", one, three, one*three);

    let mut dp = vec![0u128; adapters.len()];
    dp[adapters.len()-1] = 1;

    for i in (0..adapters.len()).rev() {
        let i1: i32 = i as i32 - 1;
        let i2: i32 = i as i32 - 2;
        let i3: i32 = i as i32 - 3;
        if i3 >= 0 {
            let dif = adapters[i] - adapters[i3 as usize];
            if dif <= 3 {
                dp[i3 as usize] += dp[i];
            }
        }
        if i2 >= 0 {
            let dif = adapters[i] - adapters[i2 as usize];
            if dif <= 3 {
                dp[i2 as usize] += dp[i];
            }
        }
        if i1 >= 0 {
            let dif = adapters[i] - adapters[i1 as usize];
            if dif <= 3 {
                dp[i1 as usize] += dp[i];
            }
        }
        //println!("{:?}", dp);
    }
    println!("{}", dp[0]);

}
