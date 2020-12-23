/*
--- Day 16: Ticket Translation ---

As you're walking to yet another connecting flight, you realize that one of the legs of your re-routed trip coming up is on a high-speed train. However, the train ticket you were given is in a language you don't understand. You should probably figure out what it says before you get to the train station after the next flight.

Unfortunately, you can't actually read the words on the ticket. You can, however, read the numbers, and so you figure out the fields these tickets must have and the valid ranges for values in those fields.

You collect the rules for ticket fields, the numbers on your ticket, and the numbers on other nearby tickets for the same train service (via the airport security cameras) together into a single document you can reference (your puzzle input).

The rules for ticket fields specify a list of fields that exist somewhere on the ticket and the valid ranges of values for each field. For example, a rule like class: 1-3 or 5-7 means that one of the fields in every ticket is named class and can be any value in the ranges 1-3 or 5-7 (inclusive, such that 3 and 5 are both valid in this field, but 4 is not).

Each ticket is represented by a single line of comma-separated values. The values are the numbers on the ticket in the order they appear; every ticket has the same format. For example, consider this ticket:

.--------------------------------------------------------.
| ????: 101    ?????: 102   ??????????: 103     ???: 104 |
|                                                        |
| ??: 301  ??: 302             ???????: 303      ??????? |
| ??: 401  ??: 402           ???? ????: 403    ????????? |
'--------------------------------------------------------'

Here, ? represents text in a language you don't understand. This ticket might be represented as 101,102,103,104,301,302,303,401,402,403; of course, the actual train tickets you're looking at are much more complicated. In any case, you've extracted just the numbers in such a way that the first number is always the same specific field, the second number is always a different specific field, and so on - you just don't know what each position actually means!

Start by determining which tickets are completely invalid; these are tickets that contain values which aren't valid for any field. Ignore your ticket for now.

For example, suppose you have the following notes:

class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12

It doesn't matter which position corresponds to which field; you can identify invalid nearby tickets by considering only whether tickets contain values that are not valid for any field. In this example, the values on the first nearby ticket are all valid for at least one field. This is not true of the other three nearby tickets: the values 4, 55, and 12 are are not valid for any field. Adding together all of the invalid values produces your ticket scanning error rate: 4 + 55 + 12 = 71.

Consider the validity of the nearby tickets you scanned. What is your ticket scanning error rate?

--- Part Two ---

Now that you've identified which tickets contain invalid values, discard those tickets entirely. Use the remaining valid tickets to determine which field is which.

Using the valid ranges for each field, determine what order the fields appear on the tickets. The order is consistent between all tickets: if seat is the third field, it is the third field on every ticket, including your ticket.

For example, suppose you have the following notes:

class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9

Based on the nearby tickets in the above example, the first position must be row, the second position must be class, and the third position must be seat; you can conclude that in your ticket, class is 12, row is 11, and seat is 13.

Once you work out which field is which, look for the six fields on your ticket that start with the word departure. What do you get if you multiply those six values together?

*/
use std::io;
use std::io::prelude::*;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();

    let re_rules = Regex::new(r"(?P<name>.+): (?P<r1>.+) or (?P<r2>.+)").unwrap();
    //println!("{:?}", re_rules);
    let re_range = Regex::new(r"(\d+)-(\d+)").unwrap();
    //println!("{:?}", re_range);

    let mut rules = HashMap::new();
    let mut rule_order = Vec::new();
    let mut reading_rules = true;
    let mut reading_own_ticket = false;
    let mut reading_other_tickets = false;

    let mut invalid = Vec::new();
    let mut tickets = Vec::new();
    for line in stdin.lock().lines() {
        let sline = line.unwrap();
        //println!("{:?}", sline);
        if sline == "" {
            if !reading_own_ticket && !reading_other_tickets {
                reading_rules = false;
            }
            continue;
        }
        if sline == "your ticket:" {
            reading_own_ticket = true;
            continue;
        }
        if sline == "nearby tickets:" {
            reading_other_tickets = true;
            continue;
        }
        if reading_rules {
            let cap = re_rules.captures(&sline).unwrap();
            //println!("{:?}", cap);
            let r1 = cap.name("r1").unwrap();
            let r2 = cap.name("r2").unwrap();
            let name = cap.name("name").unwrap();
            //println!("{:?} {:?}", r1.as_str(), r2.as_str());
            let cap_r1 = re_range.captures(r1.as_str()).unwrap();
            //println!("{:?}", cap_r1);
            let r1_s = cap_r1.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let r1_e = cap_r1.get(2).unwrap().as_str().parse::<i32>().unwrap();

            
            let cap_r2 = re_range.captures(r2.as_str()).unwrap();
            //println!("{:?}", cap_r1);
            let r2_s = cap_r2.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let r2_e = cap_r2.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let sname = String::from(name.as_str());
            rules.insert(sname, ((r1_s, r1_e), (r2_s, r2_e)));
            rule_order.push(String::from(name.as_str()));
        }
        if reading_own_ticket {
            reading_own_ticket = false;
            tickets.push(sline.split(",").map(|s| -> i32 { return s.parse::<i32>().unwrap(); }).collect::<Vec<i32>>());
            continue;
        }
        if reading_other_tickets {
            let mut numbers_valid = Vec::new();
            let ticket = sline.split(",").map(|s| -> i32 { return s.parse::<i32>().unwrap(); }).collect::<Vec<i32>>();
            for n in ticket.iter() {
                let flag: bool = rules.values().map(|r| -> bool {
                    let (r1, r2) = r;
                    let (r1s, r1e) = r1;
                    let (r2s, r2e) = r2;
                    return (*n >= *r1s && *n <= *r1e) || (*n >= *r2s && *n <= *r2e);
                 }).all(|v| -> bool {
                     return !v;
                 });
                 numbers_valid.push(!flag);
                 //println!("{:?}", flag);
                 if flag {
                    invalid.push(*n);
                 }
            }
            //println!("{:?}\n{:?}", ticket, numbers_valid);
            if numbers_valid.iter().all(|v| -> bool { return *v; }) {
                tickets.push(ticket);
            }
        }
    }
    //println!("{:?}", rules);
    //println!("{:?}", invalid);
    println!("{}", invalid.iter().sum::<i32>());
    //println!("{:?}", tickets);
    
    let mut rule_idx = HashMap::new();
    let mut values = HashMap::new();
    for i in 0..tickets[0].len() {
        values.insert(i, Vec::new());
        rule_idx.insert(i, Vec::new());
    }
    for ticket in tickets.iter().skip(1) {
        for (idx, val) in ticket.iter().enumerate() {
            values.get_mut(&idx).unwrap().push(*val);
        }
    }

    //println!("{:?}", values);
    for i in 0..tickets[0].len() {
        let valuev = values.get(&i).unwrap();
        for rname in rule_order.iter() {   
            let rule = rules.get(rname).unwrap();
            //print!("{:?} {:?}", rule, valuev);
            let flag = valuev.iter().map(|v| -> bool {
                let (r1, r2) = rule;
                let (r1s, r1e) = r1;
                let (r2s, r2e) = r2;
                return (*v >= *r1s && *v <= *r1e) || (*v >= *r2s && *v <= *r2e);
            }).all(|v| -> bool { v });
            if flag {
                rule_idx.get_mut(&i).unwrap().push(rname);
                //println!(" taken");
            }
            //println!();
        }
    }
    assert_eq!(rule_idx.len(), rules.len());
    let mut options_order = rule_idx.iter().map(|e| -> (usize, &Vec<&String>) {
        let (idx, names) = e;
        return (*idx, names);
    }).collect::<Vec<(usize, &Vec<&String>)>>();
    options_order.sort_by_key(|k| k.1.len());

    //println!("{:?}", options_order);

    let mut final_indexes = HashMap::new();
    for (i, names) in options_order {
        let mut final_name = String::new();
        for name in names.iter() {
            if !final_indexes.contains_key(*name) {
                final_name = String::from(name.as_str());
                break;
            }
        }
        final_indexes.insert(final_name, i);
    }
    //println!("{:?}", final_indexes);
    assert_eq!(final_indexes.len(), rules.len());
    //println!{"{:?}", tickets[0]};
    
    let mut product = 1i64;
    for (name, idx) in final_indexes {
        //println!("{} {} {}", name, idx, tickets[0][idx]);
        if name.contains("departure") {
            product *= tickets[0][idx] as i64;
        }
    }
    println!("{}", product);
    
}
