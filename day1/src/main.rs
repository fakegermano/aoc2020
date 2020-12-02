use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let mut input = Vec::new();
    for line in stdin.lock().lines() {
        input.push(line.unwrap_or_default().parse::<i32>().unwrap_or_default());
    }
    input.sort_unstable();
    println!("{:?}", input);

    let (a, b) = check_sum_2(&input, 2020);
    println!("numbers that sum 2020: {} {}", a,  b);
    println!("multiplied are: {}", a*b);
    let (a, b, c) = check_sum_3(&input, 2020);
    println!("numbers that sum 2020: {} {} {}", a,  b, c);
    println!("multiplied are: {}", a*b*c);
}

fn check_sum_3(slice: &[i32], target: i32) -> (i32, i32, i32) {
    let mut k = 0;
    while k < slice.len() {
        let aux = target - slice[k];
        let mut j = slice.len() - 1;
        let mut i = 0;
        loop {
            if i >= slice.len() || j <= 0 {
                break
            }
            if i == k {
                i += 1;
                continue;
            }
            if j == k {
                j -= 1;
                continue;
            }
            let sum = slice[i] + slice[j];
            println!("{} - {} = {} | {} = {}[{}] + {}[{}]", target, slice[k], aux, sum, slice[i], i, slice[j], j);
            if sum == aux {
                return (slice[k], slice[i], slice[j]);
            } else if sum > aux {
                j -= 1;
            } else {
                i += 1;
            }
        }
        k += 1;
    }
    return (-1, -1, -1)
}

fn check_sum_2(slice: &[i32], target: i32) -> (i32, i32) {
    let mut j = slice.len() - 1;
    let mut i = 0;
    loop {
        if i >= slice.len() || j <= 0 {
            return (-1, -1)
        }
        let sum = slice[i] + slice[j];
        println!("{} = {}[{}] + {}[{}]", sum, slice[i], i, slice[j], j);
        if sum == target {
            return (slice[i], slice[j]);
        } else if sum > target {
            j -= 1;
        } else {
            i += 1;
        }
    }
}