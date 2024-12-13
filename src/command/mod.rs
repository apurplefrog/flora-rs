use std::{env, fs, path::Path};

use crate::compiler;

pub fn initiate_command() {
    let compiler = compiler::Compiler::new();
    let args: Vec<String> = env::args().collect();
    let mut arg_index = 1;
    let mut flags = Vec::new();
    let mut file_name = String::new();
    let mut file_contents = Vec::new();

    while let Some(arg) = args.get(arg_index) {
        match arg.as_str() {
            "-f" | "--flags" => {
                arg_index += 1;
                while let Some(arg) = args.get(arg_index) {
                    if !arg.starts_with('-') {
                        break;
                    };

                    flags.push(arg.to_string());
                    arg_index += 1;
                }
            }

            name => {
                let path = Path::new(name);
                if let Ok(contents) = fs::read(path) {
                    file_contents = contents.iter().map(|&u| u as char).collect::<Vec<char>>();
                } else {
                    panic!();
                }
            }
        }
        arg_index += 1;
    }
}
