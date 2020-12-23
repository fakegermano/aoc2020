/*
--- Day 17: Conway Cubes ---

As your flight slowly drifts through the sky, the Elves at the Mythical Information Bureau at the North Pole contact you. They'd like some help debugging a malfunctioning experimental energy source aboard one of their super-secret imaging satellites.

The experimental energy source is based on cutting-edge technology: a set of Conway Cubes contained in a pocket dimension! When you hear it's having problems, you can't help but agree to take a look.

The pocket dimension contains an infinite 3-dimensional grid. At every integer 3-dimensional coordinate (x,y,z), there exists a single cube which is either active or inactive.

In the initial state of the pocket dimension, almost all cubes start inactive. The only exception to this is a small flat region of cubes (your puzzle input); the cubes in this region start in the specified active (#) or inactive (.) state.

The energy source then proceeds to boot up by executing six cycles.

Each cube only ever considers its neighbors: any of the 26 other cubes where any of their coordinates differ by at most 1. For example, given the cube at x=1,y=2,z=3, its neighbors include the cube at x=2,y=2,z=2, the cube at x=0,y=2,z=3, and so on.

During a cycle, all cubes simultaneously change their state according to the following rules:

    If a cube is active and exactly 2 or 3 of its neighbors are also active, the cube remains active. Otherwise, the cube becomes inactive.
    If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active. Otherwise, the cube remains inactive.

The engineers responsible for this experimental energy source would like you to simulate the pocket dimension and determine what the configuration of cubes should be at the end of the six-cycle boot process.

For example, consider the following initial state:

.#.
..#
###

Even though the pocket dimension is 3-dimensional, this initial state represents a small 2-dimensional slice of it. (In particular, this initial state defines a 3x3x1 region of the 3-dimensional space.)

Simulating a few cycles from this initial state produces the following configurations, where the result of each cycle is shown layer-by-layer at each given z coordinate (and the frame of view follows the active cells in each cycle):

Before any cycles:
z=0
..#..
...#.
.###.
.....
.....

Round 1
z=-1
.....
.#...
...#.
..#..
.....

z=0
.....
.#.#.
..##.
..#..
.....

z=1
.....
.#...
...#.
..#..
.....

Round 2
z=-2
.....
.....
..#..
.....
.....

z=-1
..#..
.#..#
....#
.#...
.....

z=0
##...
##...
#....
....#
.###.

z=1
..#..
.#..#
....#
.#...
.....

z=2
.....
.....
..#..
.....
.....


After the full six-cycle boot process completes, 112 cubes are left in the active state.

Starting with your given initial configuration, simulate six cycles. How many cubes are left in the active state after the sixth cycle?

--- Part Two ---

For some reason, your simulated results don't match what the experimental energy source engineers expected. Apparently, the pocket dimension actually has four spatial dimensions, not three.

The pocket dimension contains an infinite 4-dimensional grid. At every integer 4-dimensional coordinate (x,y,z,w), there exists a single cube (really, a hypercube) which is still either active or inactive.

Each cube only ever considers its neighbors: any of the 80 other cubes where any of their coordinates differ by at most 1. For example, given the cube at x=1,y=2,z=3,w=4, its neighbors include the cube at x=2,y=2,z=3,w=3, the cube at x=0,y=2,z=3,w=4, and so on.

The initial state of the pocket dimension still consists of a small flat region of cubes. Furthermore, the same rules for cycle updating still apply: during each cycle, consider the number of active neighbors of each cube.

For example, consider the same initial state as in the example above. Even though the pocket dimension is 4-dimensional, this initial state represents a small 2-dimensional slice of it. (In particular, this initial state defines a 3x3x1x1 region of the 4-dimensional space.)

Simulating a few cycles from this initial state produces the following configurations, where the result of each cycle is shown layer-by-layer at each given z and w coordinate:

---- Part 2 ----
Before any cycles:
z=0, w=0
..#..
...#.
.###.
.....
.....

Round 1
z=-1, w=-1
.....
.#...
...#.
..#..
.....

z=-1, w=0
.....
.#...
...#.
..#..
.....

z=-1, w=1
.....
.#...
...#.
..#..
.....

z=0, w=-1
.....
.#...
...#.
..#..
.....

z=0, w=0
.....
.#.#.
..##.
..#..
.....

z=0, w=1
.....
.#...
...#.
..#..
.....

z=1, w=-1
.....
.#...
...#.
..#..
.....

z=1, w=0
.....
.#...
...#.
..#..
.....

z=1, w=1
.....
.#...
...#.
..#..
.....

Round 2
z=-2, w=-2
.....
.....
..#..
.....
.....

z=-2, w=-1
.....
.....
.....
.....
.....

z=-2, w=0
###..
##.##
#...#
.#..#
.###.

z=-2, w=1
.....
.....
.....
.....
.....

z=-2, w=2
.....
.....
..#..
.....
.....

z=-1, w=-2
.....
.....
.....
.....
.....

z=-1, w=-1
.....
.....
.....
.....
.....

z=-1, w=0
.....
.....
.....
.....
.....

z=-1, w=1
.....
.....
.....
.....
.....

z=-1, w=2
.....
.....
.....
.....
.....

z=0, w=-2
###..
##.##
#...#
.#..#
.###.

z=0, w=-1
.....
.....
.....
.....
.....

z=0, w=0
.....
.....
.....
.....
.....

z=0, w=1
.....
.....
.....
.....
.....

z=0, w=2
###..
##.##
#...#
.#..#
.###.

z=1, w=-2
.....
.....
.....
.....
.....

z=1, w=-1
.....
.....
.....
.....
.....

z=1, w=0
.....
.....
.....
.....
.....

z=1, w=1
.....
.....
.....
.....
.....

z=1, w=2
.....
.....
.....
.....
.....

z=2, w=-2
.....
.....
..#..
.....
.....

z=2, w=-1
.....
.....
.....
.....
.....

z=2, w=0
###..
##.##
#...#
.#..#
.###.

z=2, w=1
.....
.....
.....
.....
.....

z=2, w=2
.....
.....
..#..
.....
.....

After the full six-cycle boot process completes, 848 cubes are left in the active state.

Starting with your given initial configuration, simulate six cycles in a 4-dimensional space. How many cubes are left in the active state after the sixth cycle?

*/
use std::io;
use std::io::prelude::*;
use std::collections::HashSet;

#[derive(Debug, Hash, Eq, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl Point {
    fn len_reachable_active_3d(&self, active: &HashSet<Point>) -> usize {
        let mut len = 0;
        let mut ns = Vec::new();
        for i in -1..2 {
            for j in -1..2 {
                for k in -1..2 {
                    let point = Point {
                        x: self.x + i,
                        y: self.y + j,
                        z: self.z + k,
                        w: 0
                    };
                    if point != *self {
                        ns.push(point);
                    }
                    if point != *self && active.contains(&point) {
                        len += 1;
                    }
                }
            }
        }
        assert_eq!(ns.len(), 26);
        return len;
    }

    fn len_reachable_active_4d(&self, active: &HashSet<Point>) -> usize {
        let mut len = 0;
        let mut ns = Vec::new();
        for i in -1..2 {
            for j in -1..2 {
                for k in -1..2 {
                    for l in -1..2 {
                        let point = Point {
                            x: self.x + i,
                            y: self.y + j,
                            z: self.z + k,
                            w: self.w + l,
                        };
                        if point != *self {
                            ns.push(point);
                        }
                        if point != *self && active.contains(&point) {
                            len += 1;
                        }
                    }
                }
            }
        }
        assert_eq!(ns.len(), 80);
        return len;
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w;
    }
}

struct Conway {
    active3d: HashSet<Point>,
    active4d: HashSet<Point>,
    min_x3d: i32, max_x3d: i32,
    min_y3d: i32, max_y3d: i32,
    min_z3d: i32, max_z3d: i32,
    min_x4d: i32, max_x4d: i32,
    min_y4d: i32, max_y4d: i32,
    min_z4d: i32, max_z4d: i32,
    min_w4d: i32, max_w4d: i32,
}

impl Conway {
    fn iterate3d(&mut self) {
        let mut to_add = Vec::new();
        let mut to_remove = Vec::new();

        for z in self.min_z3d-1..self.max_z3d+2 {
            //println!("\nz={}", z);
            for x in self.min_x3d-1..self.max_x3d+2 {
                for y in self.min_y3d-1..self.max_y3d+2 {
                    let w = 0;
                    let point = Point {
                        x, y, z, w
                    };
                    let c = point.len_reachable_active_3d(&self.active3d);
                    if self.active3d.contains(&point) {
                        //print!("({:2})", c);
                        if c != 2 && c != 3 {
                            to_remove.push(point);
                        }
                    } else {
                        //print!("[{:2}]", c);
                        if c == 3 {
                            to_add.push(point);
                        }
                    }
                }
                //println!();
            }
        }

        for p in to_remove.iter() {
            self.active3d.remove(p);
        }

        for p in to_add.iter() {
            self.active3d.insert(*p);
            if self.min_x3d > p.x {
                self.min_x3d = p.x;
            }
            if self.max_x3d < p.x {
                self.max_x3d = p.x;
            }
            if self.min_y3d > p.y {
                self.min_y3d = p.y;
            }
            if self.max_y3d < p.y {
                self.max_y3d = p.y;
            }
            if self.min_z3d > p.z {
                self.min_z3d = p.z;
            }
            if self.max_z3d < p.z {
                self.max_z3d = p.z;
            }
        }

        //println!("removed: {} added {}", to_remove.len(), to_add.len());
    }

    fn iterate4d(&mut self) {
        let mut to_add = Vec::new();
        let mut to_remove = Vec::new();

        for w in self.min_w4d-1..self.max_w4d+2 {
            for z in self.min_z4d-1..self.max_z4d+2 {
                //println!("\nz={} w={}", z, w);
                for x in self.min_x4d-1..self.max_x4d+2 {
                    for y in self.min_y4d-1..self.max_y4d+2 {
                        let point = Point {
                            x, y, z, w
                        };
                        let c = point.len_reachable_active_4d(&self.active4d);
                        if self.active4d.contains(&point) {
                            //print!("({:2})", c);
                            if c != 2 && c != 3 {
                                to_remove.push(point);
                            }
                        } else {
                            //print!("[{:2}]", c);
                            if c == 3 {
                                to_add.push(point);
                            }
                        }
                    }
                    //println!();
                }
            }
        }

        for p in to_remove.iter() {
            self.active4d.remove(p);
        }

        for p in to_add.iter() {
            self.active4d.insert(*p);
            if self.min_x4d > p.x {
                self.min_x4d = p.x;
            }
            if self.max_x4d < p.x {
                self.max_x4d = p.x;
            }
            if self.min_y4d > p.y {
                self.min_y4d = p.y;
            }
            if self.max_y4d < p.y {
                self.max_y4d = p.y;
            }
            if self.min_z4d > p.z {
                self.min_z4d = p.z;
            }
            if self.max_z4d < p.z {
                self.max_z4d = p.z;
            }
            if self.min_w4d > p.w {
                self.min_w4d = p.w;
            }
            if self.max_w4d < p.w {
                self.max_w4d = p.w;
            }
        }

        //println!("removed: {} added {}", to_remove.len(), to_add.len());
    }

    fn _print3d(&self) {
        for z in self.min_z3d..self.max_z3d+1 {
            println!("\nz={}", z);
            for x in self.min_x3d..self.max_x3d+1 {
                for y in self.min_y3d..self.max_y3d+1 {
                    let w = 0;
                    let point = Point {
                        x, y, z, w
                    };
                    if self.active4d.contains(&point) {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                println!();
            }
        }
    }

    fn _print4d(&self) {
        for w in self.min_w4d..self.max_w4d+1 {
            for z in self.min_z4d..self.max_z4d+1 {
                println!("\nz={} w={}", z, w);
                for x in self.min_x4d..self.max_x4d+1 {
                    for y in self.min_y4d..self.max_y4d+1 {
                        let point = Point {
                            x, y, z, w
                        };
                        if self.active4d.contains(&point) {
                            print!("#");
                        } else {
                            print!(".");
                        }
                    }
                    println!();
                }
            }
        }
    }
}

fn main() {
    let mut buffer = String::new();
    let _n = io::stdin().read_to_string(&mut buffer);
    let input: Vec<String> = buffer.split("\n").map(|s| -> String {
        return String::from(s);
    }).collect();
    //println!("{}", buffer);
    //println!("{:?}", input);
    let mut conway = Conway {
        active3d: HashSet::new(),
        active4d: HashSet::new(),
        min_x3d: 0, max_x3d: input.len() as i32 -1,
        min_x4d: 0, max_x4d: input.len() as i32 -1,
        min_y3d: 0, max_y3d: input[0].len() as i32 -1,
        min_y4d: 0, max_y4d: input[0].len() as i32 -1,
        min_z3d: 0, max_z3d: 0,
        min_z4d: 0, max_z4d: 0,
        min_w4d: 0, max_w4d: 0
    };
    for (i, r) in input.iter().enumerate() {
        for (j, c) in r.chars().enumerate() {
            match c {
                '#' => {
                    let point = Point {
                        x: i as i32,
                        y: j as i32,
                        z: 0, w: 0
                    };
                    conway.active3d.insert(point);
                    conway.active4d.insert(point);
                },
                _ => {}
            }
        }
    }
    
    for _ in 0..6 {
        //conway._print3d();
        conway.iterate3d();
    }
    println!("{}", conway.active3d.len());

    for _ in 0..6 {
        //conway._print4d();
        conway.iterate4d();
    }
    println!("{}", conway.active4d.len());
}
