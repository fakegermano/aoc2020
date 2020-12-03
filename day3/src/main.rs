/*
--- Day 3: Toboggan Trajectory ---

With the toboggan login problems resolved, you set off toward the airport. While travel by toboggan might be easy, it's certainly not safe: there's very minimal steering and the area is covered in trees. You'll need to see which angles will take you near the fewest trees.

Due to the local geology, trees in this area only grow on exact integer coordinates in a grid. You make a map (your puzzle input) of the open squares (.) and trees (#) you can see. For example:

..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#

These aren't the only trees, though; due to something you read about once involving arboreal genetics and biome stability, the same pattern repeats to the right many times:

..##.........##.........##.........##.........##.........##.......  --->
#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
.#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
.#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....  --->
.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
.#........#.#........#.#........#.#........#.#........#.#........#
#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
#...##....##...##....##...##....##...##....##...##....##...##....#
.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#  --->

You start on the open square (.) in the top-left corner and need to reach the bottom (below the bottom-most row on your map).

The toboggan can only follow a few specific slopes (you opted for a cheaper model that prefers rational numbers); start by counting all the trees you would encounter for the slope right 3, down 1:

From your starting position at the top-left, check the position that is right 3 and down 1. Then, check the position that is right 3 and down 1 from there, and so on until you go past the bottom of the map.

The locations you'd check in the above example are marked here with O where there was an open square and X where there was a tree:

..##.........##.........##.........##.........##.........##.......  --->
#..O#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
.#....X..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
..#.#...#O#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
.#...##..#..X...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
..#.##.......#.X#.......#.##.......#.##.......#.##.......#.##.....  --->
.#.#.#....#.#.#.#.O..#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
.#........#.#........X.#........#.#........#.#........#.#........#
#.##...#...#.##...#...#.X#...#...#.##...#...#.##...#...#.##...#...
#...##....##...##....##...#X....##...##....##...##....##...##....#
.#..#...#.#.#..#...#.#.#..#...X.#.#..#...#.#.#..#...#.#.#..#...#.#  --->

In this example, traversing the map using this slope would cause you to encounter 7 trees.

Starting at the top-left corner of your map and following a slope of right 3 and down 1, how many trees would you encounter?

--- Part Two ---

Time to check the rest of the slopes - you need to minimize the probability of a sudden arboreal stop, after all.

Determine the number of trees you would encounter if, for each of the following slopes, you start at the top-left corner and traverse the map all the way to the bottom:

    Right 1, down 1.
    Right 3, down 1. (This is the slope you already checked.)
    Right 5, down 1.
    Right 7, down 1.
    Right 1, down 2.

In the above example, these slopes would find 2, 7, 3, 4, and 2 tree(s) respectively; multiplied together, these produce the answer 336.

What do you get if you multiply together the number of trees encountered on each of the listed slopes?

*/
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let mut count = 0;
    let mut lineno = 0;
    let mut input = Vec::new();
    let mut total: i128 = 1;
    for line in stdin.lock().lines() {
        let place = lineno * 3;
        let sline = line.unwrap();
        input.push(sline.clone());
        let is_tree = sline.chars().nth(place % sline.len()).unwrap();
        //print!("{} {} {}", sline, is_tree, place % sline.len());
        if is_tree == '#' {
            //print!(" t");
            count += 1;
        }
        //println!("");
        lineno += 1;
    }
    println!("3 - 1 {}", count);
    total *= count;
    println!("{}", total);
    count = 0;
    lineno = 0;

    for line in &input {
        let place = lineno * 1;
        let is_tree = line.chars().nth(place % line.len()).unwrap();
        //print!("{} {} {}", line, is_tree, place % line.len());
        if is_tree == '#' {
            //print!(" t");
            count += 1;
        }
        lineno += 1;
        //println!("");
    }
    println!("1 - 1 {}", count);
    total *= count;
    println!("{}", total);
    count = 0;
    lineno = 0;

    for line in &input {
        let place = lineno * 5;
        let is_tree = line.chars().nth(place % line.len()).unwrap();
        //print!("{} {} {}", line, is_tree, place % line.len());
        if is_tree == '#' {
            //print!(" t");
            count += 1;
        }
        lineno += 1;
        //println!("");
    }
    println!("5 - 1 {}", count);
    total *= count;
    println!("{}", total);

    count = 0;
    lineno = 0;

    for line in &input {
        let place = lineno * 7;
        let is_tree = line.chars().nth(place % line.len()).unwrap();
        //print!("{} {} {}", line, is_tree, place % line.len());
        if is_tree == '#' {
            //print!(" t");
            count += 1;
        }
        lineno += 1;
        //println!("");
    }
    println!("7 - 1 {}", count);
    total *= count;
    println!("{}", total);

    count = 0;
    lineno = 0;

    for line in &input {
        //print!("{}", line);
        if lineno % 2 == 0 {
            let place = lineno / 2;
            let is_tree = line.chars().nth(place % line.len()).unwrap();
            //print!(" {} {}", is_tree, place % line.len());
            if is_tree == '#' {
                //print!(" t");
                count += 1;
            }
        }
        //println!("");
        lineno += 1;
    }
    println!("1 - 2 {}", count);
    total *= count;
    println!("{}", total);

}
