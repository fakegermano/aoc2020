/*
--- Day 1: Report Repair ---

After saving Christmas five years in a row, you've decided to take a vacation at a nice resort on a tropical island. Surely, Christmas will go on without you.

The tropical island has its own currency and is entirely cash-only. The gold coins used there have a little picture of a starfish; the locals just call them stars. None of the currency exchanges seem to have heard of them, but somehow, you'll need to find fifty of these coins by the time you arrive so you can pay the deposit on your room.

To save your vacation, you need to get all fifty stars by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

Before you leave, the Elves in accounting just need you to fix your expense report (your puzzle input); apparently, something isn't quite adding up.

Specifically, they need you to find the two entries that sum to 2020 and then multiply those two numbers together.

For example, suppose your expense report contained the following:

1721
979
366
299
675
1456

In this list, the two entries that sum to 2020 are 1721 and 299. Multiplying them together produces 1721 * 299 = 514579, so the correct answer is 514579.

Of course, your expense report is much larger. Find the two entries that sum to 2020; what do you get if you multiply them together?
*/

/*
--- Part Two ---
The Elves in accounting are thankful for your help; one of them even offers you a starfish coin they had left over from a past vacation. They offer you a second one if you can find three numbers in your expense report that meet the same criteria.

Using the above example again, the three entries that sum to 2020 are 979, 366, and 675. Multiplying them together produces the answer, 241861950.

In your expense report, what is the product of the three entries that sum to 2020?
*/
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let mut input = Vec::new();
    // read input from stdio, line by line, parsing to a int32 number
    for line in stdin.lock().lines() {
        input.push(line.unwrap_or_default().parse::<i32>().unwrap_or_default());
    }
    // a sorted input decreases the complexity of our algorithm
    input.sort_unstable();
    println!("{:?}", input);

    let (a, b) = check_sum_2(&input, 2020);
    println!("2 numbers that sum 2020: {} {}", a,  b);
    println!("multiplied are: {}", a*b);
    let (a, b, c) = check_sum_3(&input, 2020);
    println!("3 numbers that sum 2020: {} {} {}", a,  b, c);
    println!("multiplied are: {}", a*b*c);
}

fn check_sum_3(slice: &[i32], target: i32) -> (i32, i32, i32) {
    // same logic as check_sum_2
    // but we go through every element of the input first, remove its value from the target
    // sum and then try to find 2 elements that sum to this new target. if we can't find any
    // we go to the next entry on the input.
    // NOTE: need to avoid repeating pointers (we skip k in the i and j stepping)
    // NOTE: prob can use pointers here to but I got lazy
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
            //println!("{} - {} = {} | {} = {}[{}] + {}[{}]", target, slice[k], aux, sum, slice[i], i, slice[j], j);
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
    // goes from head to the tail with pointers of the input trying to sum the parts
    // if sum is larger than target, we need to get lower sums, so step the tail back
    // if sum is smaller than target, we need to get larger sums, so step the head forwards
    // NOTE: we can prob use pointers directly but I got lazy
    loop {
        if i >= slice.len() || j <= 0 {
            return (-1, -1)
        }
        let sum = slice[i] + slice[j];
        //println!("{} = {}[{}] + {}[{}]", sum, slice[i], i, slice[j], j);
        if sum == target {
            return (slice[i], slice[j]);
        } else if sum > target {
            j -= 1;
        } else {
            i += 1;
        }
    }
}