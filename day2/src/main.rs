/*
--- Day 2: Password Philosophy ---

Your flight departs in a few days from the coastal airport; the easiest way down to the coast from here is via toboggan.

The shopkeeper at the North Pole Toboggan Rental Shop is having a bad day. "Something's wrong with our computers; we can't log in!" You ask if you can take a look.

Their password database seems to be a little corrupted: some of the passwords wouldn't have been allowed by the Official Toboggan Corporate Policy that was in effect when they were chosen.

To try to debug the problem, they have created a list (your puzzle input) of passwords (according to the corrupted database) and the corporate policy when that password was set.

For example, suppose you have the following list:

1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc

Each line gives the password policy and then the password. The password policy indicates the lowest and highest number of times a given letter must appear for the password to be valid. For example, 1-3 a means that the password must contain a at least 1 time and at most 3 times.

In the above example, 2 passwords are valid. The middle password, cdefg, is not; it contains no instances of b, but needs at least 1. The first and third passwords are valid: they contain one a or nine c, both within the limits of their respective policies.

How many passwords are valid according to their policies?
*/

use std::io;
use std::io::prelude::*;
use std::str::FromStr;
use regex::Regex;

#[derive(Debug, PartialEq)]
struct PasswordEntry {
    min: u16,
    max: u16,
    character: char,
    password: String
}

impl FromStr for PasswordEntry {
    type Err = Box<dyn std::error::Error>;
    fn from_str(entry: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^(\d+)-(\d+) (\w{1}): (\w+)$").unwrap();
        let mut min: u16 = 0;
        let mut max: u16 = 0;
        let mut character: char = '0';
        let mut password: String = "0".to_string();
        for cap in re.captures_iter(entry) {
            min = u16::from_str(&cap[1])?;
            max = u16::from_str(&cap[2])?;
            character = cap[3].chars().nth(0).unwrap();
            password = cap[4].to_string();
            break;
        }
        return Ok(PasswordEntry {
                min, max, 
                character, 
                password
        });
    }
}

impl PasswordEntry {
    fn check(&self) -> bool {
        let mut count = 0;
        for c in self.password.chars() {
            if c == self.character {
                count += 1;
            }
        }
        //println!("{} {} {}", self.password, self.character, count);
        return count >= self.min && count <= self.max;
    }

    fn check_new(&self) -> bool {
        let f = self.password.chars().nth((self.min - 1).into()).unwrap() == self.character;
        let s = self.password.chars().nth((self.max - 1).into()).unwrap() == self.character; 
        let res = f ^ s;
        //println!("{} {} {} {}", self.password, f, s, res);
        return res
    }
}

fn main() {
    let stdin = io::stdin();
    let mut count = 0;
    let mut count_new = 0;
    for line in stdin.lock().lines() {
        let current_line = line.unwrap();
        match PasswordEntry::from_str(current_line.as_str()) {
            Ok(rgb) => {
                //println!("{:?}", rgb);
                if rgb.check() {
                    //println!("OK");
                    count += 1;
                }
                if rgb.check_new() {
                    count_new += 1;
                }
            },
            Err(_) => {
                println!("error parsing line");
            }
        }
    }
    println!("{}", count);
    println!("{}", count_new);
}
