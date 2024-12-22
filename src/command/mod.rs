use std::{env, fs, path::Path};

use crate::compiler::compiler::Compiler;

pub fn initiate_command() {
    let args: Vec<String> = env::args().collect();
    let mut compiler = Compiler::new();
    let mut arg_index = 1;
    let mut flags = Vec::new();
    let mut file_contents;

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

            "-o" | "--output" => {
                arg_index += 1;
                let output_file = args.get(arg_index).unwrap();
                let output_path = Path::new(output_file);
                compiler.set_output_file(output_path);
            }

            name => {
                let path = Path::new(name);
                if let Ok(contents) = fs::read(path) {
                    file_contents = contents.iter().map(|&u| u as char).collect::<Vec<char>>();
                    let compilation_succeeded = compiler.compile(file_contents);
                    use super::compiler::compiler_types::CompilationResult::*;
                    match compilation_succeeded {
                        Failure => println!("Compilation Failed!"),
                        Success => println!("Compiltation Succeeded!"),
                    }
                } else {
                    panic!();
                }
            }
        }
        arg_index += 1;
    }
}
