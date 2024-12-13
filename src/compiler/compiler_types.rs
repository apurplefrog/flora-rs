#[repr(u32)]
pub enum CompilerFlags {
    MinimalOutput = 1,
    StandardOutput = 2,
    VerboseOutput = 4,
    OptimizationLevel0 = 8,
    OptimizationLevel1 = 16,
    OptimizationLevel2 = 32,
    OptimizationLevelMax = 64,
}

pub enum CompilationResult {
    CompilationFailure,
    CompilationSuccess,
}

impl std::ops::BitOr for CompilerFlags {
    type Output = u32;
    fn bitor(self, rhs: Self) -> Self::Output {
        self as u32 | rhs as u32
    }
}

#[derive(Default)]
pub struct CompilerVersion(String);

impl CompilerVersion {
    pub fn current_version() -> Self {
        CompilerVersion("v1.0.0".to_string())
    }

    pub fn get_version(&self) -> String {
        self.0.clone()
    }
}

