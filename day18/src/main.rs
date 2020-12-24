/*
--- Day 18: Operation Order ---

As you look out the window and notice a heavily-forested continent slowly appear over the horizon, you are interrupted by the child sitting next to you. They're curious if you could help them with their math homework.

Unfortunately, it seems like this "math" follows different rules than you remember.

The homework (your puzzle input) consists of a series of expressions that consist of addition (+), multiplication (*), and parentheses ((...)). Just like normal math, parentheses indicate that the expression inside must be evaluated before it can be used by the surrounding expression. Addition still finds the sum of the numbers on both sides of the operator, and multiplication still finds the product.

However, the rules of operator precedence have changed. Rather than evaluating multiplication before addition, the operators have the same precedence, and are evaluated left-to-right regardless of the order in which they appear.

For example, the steps to evaluate the expression 1 + 2 * 3 + 4 * 5 + 6 are as follows:

1 + 2 * 3 + 4 * 5 + 6
  3   * 3 + 4 * 5 + 6
      9   + 4 * 5 + 6
         13   * 5 + 6
             65   + 6
                 71

Parentheses can override this order; for example, here is what happens if parentheses are added to form 1 + (2 * 3) + (4 * (5 + 6)):

1 + (2 * 3) + (4 * (5 + 6))
1 +    6    + (4 * (5 + 6))
     7      + (4 * (5 + 6))
     7      + (4 *   11   )
     7      +     44
            51

Here are a few more examples:

    2 * 3 + (4 * 5) becomes 26.
    5 + (8 * 3 + 9 + 3 * 4 * 3) becomes 437.
    5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4)) becomes 12240.
    ((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2 becomes 13632.

Before you can help with the homework, you need to understand it yourself. Evaluate the expression on each line of the homework; what is the sum of the resulting values?

*/
use std::io;
use std::io::prelude::*;
#[macro_use] extern crate lalrpop_util;
pub mod ast;

lalrpop_mod!(pub grammar);

use ast::{Expr, Opcode};

fn calculate(expr: &Expr) -> u128 {
    match *expr {
        Expr::Number(n) => return n as u128,
        Expr::Op(ref l, op, ref r) => {
            match op {
                Opcode::Add => return calculate(&l) + calculate(&r),
                Opcode::Mul => return calculate(&l) * calculate(&r),
            }
        }
    }
}
fn main() {
    let exp1 = grammar::ExprParser::new().parse("1 + 2 * 3 + 4 * 5 + 6").unwrap();
    let v1 = calculate(&exp1);
    //println!("{:?} = {}", exp1, v1);
    assert_eq!(v1, 71);
    
    let exp1 = grammar::ExprParser::new().parse("1 + (2 * 3) + (4 * (5 + 6))").unwrap();
    let v1 = calculate(&exp1);
    //println!("{:?} = {}", exp1, v1);
    assert_eq!(v1, 51);

    let exp1 = grammar::ExprParser::new().parse("2 * 3 + (4 * 5)").unwrap();
    let v1 = calculate(&exp1);
    //println!("{:?} = {}", exp1, v1);
    assert_eq!(v1, 26);

    let exp1 = grammar::ExprParser::new().parse("5 + (8 * 3 + 9 + 3 * 4 * 3)").unwrap();
    let v1 = calculate(&exp1);
    //println!("{:?} = {}", exp1, v1);
    assert_eq!(v1, 437);

    let exp1 = grammar::ExprParser::new().parse("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))").unwrap();
    let v1 = calculate(&exp1);
    //println!("{:?} = {}", exp1, v1);
    assert_eq!(v1, 12240);

    let exp1 = grammar::ExprParser::new().parse("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2").unwrap();
    let v1 = calculate(&exp1);
    //println!("{:?} = {}", exp1, v1);
    assert_eq!(v1, 13632);

    let mut total = 0u128;

    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let exp = grammar::ExprParser::new().parse(line.unwrap().as_str()).unwrap();
        let v = calculate(&exp);
        //println!("{:?} = {}", exp, v);

        total += v;
    }

    println!("{}", total);
}
