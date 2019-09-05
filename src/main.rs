extern crate nom;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace0,
    number::complete::double,
    sequence::preceded,
    IResult,
};

#[derive(Debug)]
enum NodeValue {
    Numeric(f64),
    Operator(char),
}

#[derive(Debug)]
struct Node {
    pub element: NodeValue,
    pub left_child: Option<Box<Node>>,
    pub right_child: Option<Box<Node>>,
}

fn parse_number(s: &str) -> IResult<&str, f64> {
    double(s)
}

fn parse_op(s: &str) -> IResult<&str, &str> {
    alt((tag("+"), tag("-"), tag("/"), tag("*")))(s)
}


fn parse_expr(s: &str) -> Node {
    // Get the number and operator
    if s.len() == 0 {
        panic!("Received empty expression");
    }

    // Get number
    let (s, num) = preceded(multispace0, parse_number)(s).unwrap();

    // Make sure that there is an op left to parse (needed at the end of a string)
    if s.len() == 0 {
        return Node {
            element: NodeValue::Numeric(num),
            left_child: None,
            right_child: None,
        };
    }

    // Get operator
    let (s, op) = preceded(multispace0, parse_op)(s).unwrap();

    // Get the char
    let op = op.chars().next().unwrap();

    return Node {
        element: NodeValue::Operator(op),
        left_child: Some(Box::new(Node {
            element: NodeValue::Numeric(num),
            left_child: None,
            right_child: None,
        })),
        right_child: Some(Box::new(parse_expr(s))),
    };
}

fn main() {
    println!("{:#?}", parse_expr("1+ 2+3+ 4"));
}
