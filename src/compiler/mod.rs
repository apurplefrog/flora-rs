pub mod compiler_types;

use crate::compiler::compiler_types::*;
use crate::tokenizer;

#[derive(Default)]
pub struct Compiler {
    flags: u32,
    version: CompilerVersion,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            flags: CompilerFlags::StandardOutput | CompilerFlags::OptimizationLevel1,
            version: CompilerVersion::current_version(),
        }
    }

    pub fn compile(&self, chars: Vec<char>) -> CompilationResult {
        println!("{:#?}", tokenizer::tokenize(chars));
        CompilationResult::CompilationSuccess
    }
}
