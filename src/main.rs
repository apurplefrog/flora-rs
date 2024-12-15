#![allow(dead_code)]
#![allow(unused)]

mod command;
mod compiler;
mod tokenizer;

fn main() {
    crate::command::initiate_command();
}
