/*
--- Day 7: Handy Haversacks ---

You land at the regional airport in time for your next flight. In fact, it looks like you'll even have time to grab some food: all flights are currently delayed due to issues in luggage processing.

Due to recent aviation regulations, many rules (your puzzle input) are being enforced about bags and their contents; bags must be color-coded and must contain specific quantities of other color-coded bags. Apparently, nobody responsible for these regulations considered how long they would take to enforce!

For example, consider the following rules:

light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.

These rules specify the required contents for 9 bag types. In this example, every faded blue bag is empty, every vibrant plum bag contains 11 bags (5 faded blue and 6 dotted black), and so on.

You have a shiny gold bag. If you wanted to carry it in at least one other bag, how many different bag colors would be valid for the outermost bag? (In other words: how many colors can, eventually, contain at least one shiny gold bag?)

In the above rules, the following options would be available to you:

    A bright white bag, which can hold your shiny gold bag directly.
    A muted yellow bag, which can hold your shiny gold bag directly, plus some other bags.
    A dark orange bag, which can hold bright white and muted yellow bags, either of which could then hold your shiny gold bag.
    A light red bag, which can hold bright white and muted yellow bags, either of which could then hold your shiny gold bag.

So, in this example, the number of bag colors that can eventually contain at least one shiny gold bag is 4.

How many bag colors can eventually contain at least one shiny gold bag? (The list of rules is quite long; make sure you get all of it.)
*/
use std::io;
use std::io::prelude::*;
use regex::Regex;
use std::collections::{HashMap, HashSet, LinkedList};

#[derive(Debug)]
struct Graph {
    nodes_map: Vec<String>,
    nodes: HashMap<String, usize>,
    edges: HashMap<String, LinkedList<usize>>
}

impl Graph {
    fn dfs(&self, source: usize) -> HashSet<usize> {
        
        let mut stack: LinkedList<usize> = LinkedList::new();
        stack.push_front(source);

        let mut visited = HashSet::new();
        let mut reachable = HashSet::new();
        while stack.len() > 0 {
            let node = stack.pop_front().unwrap();
            if visited.insert(node) {
                reachable.insert(node);
            }
            let color = &self.nodes_map[node];
            for i in self.edges.get(color).unwrap().iter() {
                stack.push_front(*i);
            }
        }
        return reachable;
    }
}

fn main() {
    let stdin = io::stdin();
    let rc_re = Regex::new(
        r"((?P<color>\w+ \w+) bag[s]*)+"
    ).unwrap();
    let mut graph = Graph {
        nodes: HashMap::new(),
        nodes_map: Vec::new(),
        edges: HashMap::new()
    };
    let mut i = 0;
    for line in stdin.lock().lines() {
        let sline = line.unwrap().to_owned();
        //println!("{}", sline);
        let mut first = true;
        let mut root = 0;
        for cap in rc_re.captures_iter(&sline) {
            let color = cap["color"].trim().to_owned();
            if color != "no other" {
                if ! graph.nodes.contains_key(&color) {
                    //print!(".");
                    graph.nodes_map.push(color.clone());
                    graph.nodes.insert(color.clone(), i);
                    i += 1;
                }   
                if first {
                    root = *graph.nodes.get(&color).unwrap();
                    first = false;
                    //println!("\t({})", color);
                    graph.edges.insert(color.clone(), LinkedList::new());
                } else {
                    let root_str = &graph.nodes_map[root];
                    let color_i = *graph.nodes.get(&color).unwrap();
                    //println!("\t\t[{} {}]->[{} {}]", root_str, root, color, color_i);
                    graph.edges.get_mut(root_str).unwrap().push_back(color_i);
                }
            }
        }
    }
    assert_eq!(graph.nodes.len(), graph.nodes_map.len());
    //println!("{:?}", graph);
    let target = *graph.nodes.get("shiny gold").unwrap();
    //println!("{}", target);
    
    let mut can_reach = Vec::new();
    for i in 0..graph.nodes.len() {
        if i != target { // dont run the dfs from the target as it will yeild wrong results
            let reachable = graph.dfs(i);
            if reachable.contains(&target) {
                //println!("{} {:?}", i, reachable);
                can_reach.push(i);
            }
        }
    }
    //println!("{:?}", can_reach);
    println!("{}", can_reach.len());

}
