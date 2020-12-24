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

--- Part Two ---

You manage to answer the child's questions and they finish part 1 of their homework, but get stuck when they reach the next section: advanced math.

Now, addition and multiplication have different precedence levels, but they're not the ones you're familiar with. Instead, addition is evaluated before multiplication.

For example, the steps to evaluate the expression 1 + 2 * 3 + 4 * 5 + 6 are now as follows:

1 + 2 * 3 + 4 * 5 + 6
  3   * 3 + 4 * 5 + 6
  3   *   7   * 5 + 6
  3   *   7   *  11
     21       *  11
         231

Here are the other examples from above:

    1 + (2 * 3) + (4 * (5 + 6)) still becomes 51.
    2 * 3 + (4 * 5) becomes 46.
    5 + (8 * 3 + 9 + 3 * 4 * 3) becomes 1445.
    5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4)) becomes 669060.
    ((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2 becomes 23340.

What do you get if you add up the results of evaluating the homework problems using these new rules?

*/
use std::io;
use std::io::prelude::*;
#[macro_use] extern crate lalrpop_util;
pub mod ast;

lalrpop_mod!(pub grammar);
lalrpop_mod!(pub grammar2);

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
    // part 1 grammar check
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

    // part 2 grammar check
    let exp1 = grammar2::ExprParser::new().parse("1 + 2 * 3 + 4 * 5 + 6").unwrap();
    let v1 = calculate(&exp1);
    //println!("{:?} = {}", exp1, v1);
    assert_eq!(v1, 231);

    let exp1 = grammar2::ExprParser::new().parse("1 + (2 * 3) + (4 * (5 + 6))").unwrap();
    let v1 = calculate(&exp1);
    //println!("{:?} = {}", exp1, v1);
    assert_eq!(v1, 51);

    let exp1 = grammar2::ExprParser::new().parse("2 * 3 + (4 * 5)").unwrap();
    let v1 = calculate(&exp1);
    //println!("{:?} = {}", exp1, v1);
    assert_eq!(v1, 46);

    let exp1 = grammar2::ExprParser::new().parse("5 + (8 * 3 + 9 + 3 * 4 * 3)").unwrap();
    let v1 = calculate(&exp1);
    //println!("{:?} = {}", exp1, v1);
    assert_eq!(v1, 1445);

    let exp1 = grammar2::ExprParser::new().parse("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))").unwrap();
    let v1 = calculate(&exp1);
    //println!("{:?} = {}", exp1, v1);
    assert_eq!(v1, 669060);

    let exp1 = grammar2::ExprParser::new().parse("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))").unwrap();
    let v1 = calculate(&exp1);
    //println!("{:?} = {}", exp1, v1);
    assert_eq!(v1, 669060);
    
    let exp1 = grammar2::ExprParser::new().parse("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2").unwrap();
    let v1 = calculate(&exp1);
    //println!("{:?} = {}", exp1, v1);
    assert_eq!(v1, 23340);

    let mut total1 = 0u128;
    let mut total2 = 0u128;

    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let sline = String::from(line.unwrap().as_str());
        let exp1 = grammar::ExprParser::new().parse(sline.as_str()).unwrap();
        let v1 = calculate(&exp1);
        //println!("{:?} = {}", exp1, v1);
        let exp2 = grammar2::ExprParser::new().parse(sline.as_str()).unwrap();
        let v2 = calculate(&exp2);
        //println!("{:?} = {}", exp2, v2);

        total1 += v1;
        total2 += v2;
    }

    println!("{}", total1);
    println!("{}", total2);
}
