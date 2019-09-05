extern crate nom;

use nom::{
    branch::alt, bytes::complete::tag, character::complete::multispace0, number::complete::double,
    IResult,
};

#[derive(Debug)]
enum NODE_VALUE {
    numeric(f64),
    operator(char),
}

#[derive(Debug)]
struct Node {
    pub element: NODE_VALUE,
    pub left_child: Option<Box<Node>>,
    pub right_child: Option<Box<Node>>,
}

fn parse_number(s: &str) -> IResult<&str, f64> {
    double(s)
}

fn parse_op(s: &str) -> IResult<&str, &str> {
    alt((tag("+"), tag("-"), tag("/"), tag("*")))(s)
}

fn remove_whitespace(s: &str) -> IResult<&str, &str> {
    multispace0(s)
}

fn parse_expr(s: &str) -> Node {
    // Get the number and operator
    if s.len() == 0 {
        panic!("Received empty expression");
    }

    let s = remove_whitespace(s).unwrap().0;
    let (s, num) = parse_number(s).unwrap();

    // Make sure that there is an op left to parse (needed at the end of a string)
    if s.len() == 0 {
        return Node {
            element: NODE_VALUE::numeric(num),
            left_child: None,
            right_child: None,
        };
    }

    let s = remove_whitespace(s).unwrap().0;
    let (s, op) = parse_op(s).unwrap();

    let op = op.chars().next().unwrap();

    return Node {
        element: NODE_VALUE::operator(op),
        left_child: Some(Box::new(Node {
            element: NODE_VALUE::numeric(num),
            left_child: None,
            right_child: None,
        })),
        right_child: Some(Box::new(parse_expr(s))),
    };
}

fn main() {
    println!("{:#?}", parse_expr("1+ 2+3+ 4"));
}
