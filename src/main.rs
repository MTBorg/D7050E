extern crate nom;

use nom::{
    IResult,
    bytes::complete::{tag},
    number::complete::double,
    branch::alt
};

enum NODE_VALUE{
    numeric(f64),
    operator(char)
}

struct Node{
    pub element: NODE_VALUE,
    pub left_child: Option<Box<Node>>,
    pub right_child: Option<Box<Node>>
}

fn parse_number(s: &str) -> IResult<&str, f64>{
    double(s)
}

fn parse_op(s: &str) -> IResult<&str, &str>{
    alt((
        tag("+"),
        tag("-"),
        tag("/"),
        tag("*")
    ))(s)
}

fn parse_expr(s: &str, root: Node, last_op: Option<char>) -> Node{
    // Get the number and operator
    if s.len() == 0{
        panic!("Received empty expression");
    }

    let (s, num)= parse_number(s).unwrap();

    println!("num:{}", num);
    // Make sure that there is an op left to parse (needed at the end of a string)
    if s.len() == 0{
        println!("EMPTY STRING");
        return root
    }
    let (s, op)=parse_op(s).unwrap();
    println!("op:{}", op);

    let op = op.chars().next().unwrap();

    // // Check if we can close the parenthesis
    // if (op == '*' || op == '/') && (last_op == Some('+') || last_op == Some('-')){
    //     root.right_child = parse_expr 
    // }

    // let last_op = Some(op);

    parse_expr(s, root, Some(op))
}

fn main() {
    let ast_root: Node = Node{
        element: NODE_VALUE::operator('*'),
        left_child: None,
        right_child: None
    };
    parse_expr("10*1*3+4", ast_root, None);
}
