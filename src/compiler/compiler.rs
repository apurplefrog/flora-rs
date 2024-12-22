use super::compiler_types::*;
use crate::tokenizer;
use std::{fs, path::Path};

#[derive(Debug)]
pub struct Compiler<'a> {
    flags: CompilerFlags,
    version: CompilerVersion,
    output_file: &'a Path,
}

impl<'a> Compiler<'a> {
    pub fn new() -> Self {
        Self {
            flags: CompilerFlags {
                flags: CompilerFlag::StandardOutput | CompilerFlag::OptimizationLevel1,
            },
            version: CompilerVersion::get_current_version(),
            output_file: Path::new("output.asm"),
        }
    }

    pub fn set_flags(&mut self, flags: CompilerFlags) {
        self.flags = flags;
    }

    pub fn get_flags(&self) -> &CompilerFlags {
        &self.flags
    }

    pub fn get_version(&self) -> &CompilerVersion {
        &self.version
    }

    pub fn set_output_file(&mut self, path: &'a Path) {
        self.output_file = &path;
    }

    pub fn get_output_file(&self) -> &Path {
        &self.output_file
    }

    pub fn compile(&self, chars: Vec<char>) -> CompilationResult {
        let advanced_tokens = tokenizer::tokenize(chars);
        println!("{:#?}", advanced_tokens);
        let mut asm = "_start:\n\tjmp main\n".to_string();
        let mut token_index = 0;
        while token_index < advanced_tokens.len() {
            use crate::tokenizer::token_types::AdvancedToken::*;
            match advanced_tokens[token_index] {
                Function => {
                    token_index += 1;
                    if let Identifier(function_name) = &advanced_tokens[token_index] {
                        asm += format!("{function_name}:\n").as_str();
                    }
                }
                _ => (),
            }
            token_index += 1;
        }
        let mut file = fs::write(self.get_output_file(), asm);
        CompilationResult::Success
    }
}
