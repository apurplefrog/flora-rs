use std::path::Path;

use crate::tokenizer::tokenize;

#[repr(u32)]
#[derive(Debug)]
pub enum CompilerFlag {
    MinimalOutput = 1,
    StandardOutput = 2,
    VerboseOutput = 4,
    OptimizationLevel0 = 8,
    OptimizationLevel1 = 16,
    OptimizationLevel2 = 32,
    OptimizationLevelMax = 64,
}

#[derive(Debug)]
pub struct CompilerFlags {
    pub flags: u32,
}

#[derive(Debug)]
pub enum CompilationResult {
    Failure,
    Success,
}

impl std::ops::BitOr for CompilerFlag {
    type Output = u32;
    fn bitor(self, rhs: Self) -> Self::Output {
        self as u32 | rhs as u32
    }
}

#[derive(Default, Debug)]
pub struct CompilerVersion {
    current_version: String,
}

const CURRENT_VERSION: &str = "Alpha v1.0.0";

impl CompilerVersion {
    pub fn get_current_version() -> CompilerVersion {
        Self {
            current_version: CURRENT_VERSION.to_string(),
        }
    }
}
