#![allow(dead_code)]

mod command;
mod compiler;
mod tokenizer;

fn main() {
    crate::command::initiate_command();
}
