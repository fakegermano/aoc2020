/*
--- Day 11: Seating System ---

Your plane lands with plenty of time to spare. The final leg of your journey is a ferry that goes directly to the tropical island where you can finally start your vacation. As you reach the waiting area to board the ferry, you realize you're so early, nobody else has even arrived yet!

By modeling the process people use to choose (or abandon) their seat in the waiting area, you're pretty sure you can predict the best place to sit. You make a quick map of the seat layout (your puzzle input).

The seat layout fits neatly on a grid. Each position is either floor (.), an empty seat (L), or an occupied seat (#). For example, the initial seat layout might look like this:

L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL

Now, you just need to model the people who will be arriving shortly. Fortunately, people are entirely predictable and always follow a simple set of rules. All decisions are based on the number of occupied seats adjacent to a given seat (one of the eight positions immediately up, down, left, right, or diagonal from the seat). The following rules are applied to every seat simultaneously:

    If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
    If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
    Otherwise, the seat's state does not change.

Floor (.) never changes; seats don't move, and nobody sits on the floor.

After one round of these rules, every seat in the example layout becomes occupied:

#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##

After a second round, the seats with four or more occupied adjacent seats become empty again:

#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##

This process continues for three more rounds:

#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##

#.#L.L#.##
#LLL#LL.L#
L.L.L..#..
#LLL.##.L#
#.LL.LL.LL
#.LL#L#.##
..L.L.....
#L#LLLL#L#
#.LLLLLL.L
#.#L#L#.##

#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##

At this point, something interesting happens: the chaos stabilizes and further applications of these rules cause no seats to change state! Once people stop moving around, you count 37 occupied seats.

Simulate your seating area by applying the seating rules repeatedly until no seats change state. How many seats end up occupied?

*/

use std::io;
use std::io::prelude::*;

fn neighbours(i: i32, j: i32, n: usize, m: usize, input: &Vec<Vec<u8>>) -> usize {
    let mut count = 0;
    for k in -1..2 {
        for l in -1..2 {
            if (k, l) != (0, 0) {
                if i + k >= 0 && i + k < n as i32 {
                    if j + l >= 0 && j + l < m as i32 {
                        if input[(i+k) as usize][(j+l) as usize] == 2 {
                            count += 1;
                        }
                    }
                }
            }
        }
    }
    return count;
}
fn main() {
    let stdin = io::stdin();

    let mut input = Vec::new();
    for line in stdin.lock().lines() {
        let row: Vec<u8> = line.unwrap().chars().map(|c| match c {
            '.' => 0,
            'L' => 1,
            _ => 2
        }).collect();
        input.push(row.to_owned());
    }
    //println!("{:?}", input);

    loop {
        let mut changes = 0;
        let mut occupied = 0;
        let old_input = input.clone();
        for i in 0..input.len() {
            for j in 0..old_input[i].len() {
                match old_input[i][j] {
                    1 => if neighbours(i as i32, j as i32, old_input.len(), old_input[i].len(), &old_input) == 0 {
                        input[i][j] = 2;
                        occupied += 1;
                        changes += 1;
                    },
                    2 => if neighbours(i as i32, j as i32, old_input.len(), old_input[i].len(), &old_input) >= 4 {
                        input[i][j] = 1;
                        changes += 1;
                    } else {
                        occupied += 1;
                    }
                    _ => {}
                }
                //print!("{}", old_input[i][j]);
            }
            //println!();
        }
        //println!("{}", changes);
        if changes == 0 {
            println!("{}", occupied);
            break;
        }
    }
}
