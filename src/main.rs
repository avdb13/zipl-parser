#![no_std]

use sections::default::Auto;

pub(crate) mod sections;

fn main() {}

#[derive(Clone, Debug, PartialEq)]
enum Section<'a> {
    Default(&'a str),
    Menu(&'a str),
    Auto(Auto<'a>),
}
