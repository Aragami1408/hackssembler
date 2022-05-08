use std::env;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, digit1, multispace0};
use nom::combinator::{opt, rest, value};
use nom::IResult;
use nom::Parser;
use nom::{
    multi::many_till,
    sequence::{pair, tuple},
};

mod utils;



fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args); 

    let instruction_list: Vec<String> = utils::file_to_vec(&args[1]).unwrap();

}
