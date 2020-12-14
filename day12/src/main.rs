/*
--- Day 12: Rain Risk ---

Your ferry made decent progress toward the island, but the storm came in faster than anyone expected. The ferry needs to take evasive actions!

Unfortunately, the ship's navigation computer seems to be malfunctioning; rather than giving a route directly to safety, it produced extremely circuitous instructions. When the captain uses the PA system to ask if anyone can help, you quickly volunteer.

The navigation instructions (your puzzle input) consists of a sequence of single-character actions paired with integer input values. After staring at them for a few minutes, you work out what they probably mean:

    Action N means to move north by the given value.
    Action S means to move south by the given value.
    Action E means to move east by the given value.
    Action W means to move west by the given value.
    Action L means to turn left the given number of degrees.
    Action R means to turn right the given number of degrees.
    Action F means to move forward by the given value in the direction the ship is currently facing.

The ship starts by facing east. Only the L and R actions change the direction the ship is facing. (That is, if the ship is facing east and the next instruction is N10, the ship would move north 10 units, but would still move east if the following action were F.)

For example:

F10
N3
F7
R90
F11

These instructions would be handled as follows:

    F10 would move the ship 10 units east (because the ship starts by facing east) to east 10, north 0.
    N3 would move the ship 3 units north to east 10, north 3.
    F7 would move the ship another 7 units east (because the ship is still facing east) to east 17, north 3.
    R90 would cause the ship to turn right by 90 degrees and face south; it remains at east 17, north 3.
    F11 would move the ship 11 units south to east 17, south 8.

At the end of these instructions, the ship's Manhattan distance (sum of the absolute values of its east/west position and its north/south position) from its starting position is 17 + 8 = 25.

Figure out where the navigation instructions lead. What is the Manhattan distance between that location and the ship's starting position?
*/
use std::io;
use std::io::prelude::*;

#[derive(Debug)]
struct Ship {
    x: i32,
    y: i32,
    dir: i32
}

impl Ship {
    fn distance(self) -> i32 {
        return self.x.abs() + self.y.abs();
    }
}
fn decode(entry: &str, ship: &mut Ship) {
    //print!("{} {:?}", entry, ship);
    let amount = entry[1..].parse::<i32>().unwrap();
    match entry.chars().nth(0).unwrap() {
        'F' => {
            let face = ship.dir % 360;
            match face {
                0 | -360 => { // east
                    ship.x += amount;
                },
                90 | -270 => { // north
                    ship.y += amount;
                },
                180 | -180 => { // west
                    ship.x -= amount;
                },
                270 | -90 => { // south
                    ship.y -= amount;
                },
                _ => {
                    panic!("WHAT? {}", face);
                }
            }
        },
        'N' => {
            ship.y += amount;
        },
        'S' => {
            ship.y -= amount;
        },
        'E' => {
            ship.x += amount;
        },
        'W' => {
            ship.x -= amount;
        },
        'L' => {
            ship.dir += amount;
        },
        'R' => {
            ship.dir -= amount;
        },
        _ => panic!("shouldn't happen")
    }
    //println!(" -> {:?}", ship);
}
fn main() {
    let stdin = io::stdin();
    let mut s = Ship {
        x: 0, y: 0, 
        dir: 0
    };
    //println!("{:?}", s);
    for line in stdin.lock().lines() {
        decode(&line.unwrap(), &mut s);
    }
    println!("{}", s.distance());
}
