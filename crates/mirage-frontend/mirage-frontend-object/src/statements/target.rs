use crate::stringify::Stringify;

/// A target is a combination of os, arch and compiler.
/// Syntax: target <os>-<arch>-<compiler>;
/// Example: target linux-x86_64-gcc;
/// Example: target windows-x86-msvc;
///
/// Target is used to specify the target platform for the compiler.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Target(TargetType);

impl Target {
    pub fn new(os: Os, arch: Arch, compiler: Compiler) -> Self {
        Self(TargetType::new(os, arch, compiler))
    }

    pub fn from(os: &str, arch: &str, compiler: &str) -> Self {
        Self(TargetType::from(os, arch, compiler))
    }

    pub fn parse(target: &str) -> Self {
        Self(TargetType::parse(target))
    }
}

impl Stringify for Target {
    fn to_string(&self) -> String {
        format!("target {};", self.0.to_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TargetType {
    pub os: Os,
    pub arch: Arch,
    pub compiler: Compiler,
}

impl TargetType {
    pub fn new(os: Os, arch: Arch, compiler: Compiler) -> Self {
        Self {
            os,
            arch,
            compiler,
        }
    }

    pub fn from(os: &str, arch: &str, compiler: &str) -> Self {
        Self {
            os: Os::new(os),
            arch: Arch::new(arch),
            compiler: Compiler::new(compiler),
        }
    }

    pub fn parse(target: &str) -> Self {
        let mut os = Os::Unknown;
        let mut arch = Arch::Unknown;
        let mut compiler = Compiler::Unknown;
        let mut target = target.split('-');
        if let Some(os_str) = target.next() {
            os = Os::new(os_str);
        }
        if let Some(arch_str) = target.next() {
            arch = Arch::new(arch_str);
        }
        if let Some(compiler_str) = target.next() {
            compiler = Compiler::new(compiler_str);
        }
        Self {
            os,
            arch,
            compiler,
        }
    }

    pub fn to_str(&self) -> String {
        format!("{}-{}-{}", self.os.to_str(), self.arch.to_str(), self.compiler.to_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Os {
    Linux,
    Windows,
    MacOs,
    Android,
    Ios,
    Unknown,
}

impl Os {
    pub fn new(os: &str) -> Self {
        match os {
            "linux" => Self::Linux,
            "windows" => Self::Windows,
            "macos" => Self::MacOs,
            "android" => Self::Android,
            "ios" => Self::Ios,
            _ => Self::Unknown,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Self::Linux => "linux",
            Self::Windows => "windows",
            Self::MacOs => "macos",
            Self::Android => "android",
            Self::Ios => "ios",
            Self::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Arch {
    X86,
    X86_64,
    Arm,
    Arm64,
    Unknown,
}

impl Arch {
    pub fn new(arch: &str) -> Self {
        match arch {
            "x86" => Self::X86,
            "x86_64" => Self::X86_64,
            "arm" => Self::Arm,
            "arm64" => Self::Arm64,
            _ => Self::Unknown,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Self::X86 => "x86",
            Self::X86_64 => "x86_64",
            Self::Arm => "arm",
            Self::Arm64 => "arm64",
            Self::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Compiler {
    Gcc,
    Clang,
    Msvc,
    Unknown,
    All
}

impl Compiler {
    pub fn new(compiler: &str) -> Self {
        match compiler {
            "gcc" => Self::Gcc,
            "clang" => Self::Clang,
            "msvc" => Self::Msvc,
            "all" => Self::All,
            _ => Self::Unknown,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Self::Gcc => "gcc",
            Self::Clang => "clang",
            Self::Msvc => "msvc",
            Self::Unknown => "unknown",
            Self::All => "all",
        }
    }
}
