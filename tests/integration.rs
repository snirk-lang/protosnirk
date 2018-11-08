//! Test runner for protosnirk tests

#![allow(unused_imports)] // Some imports needed for generated code

extern crate protosnirk;
#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate derive_integration_tests;

use std::path::Path;
use std::fs::File;
use std::io::{Read, Write};

use protosnirk::llvm::{Context};
use protosnirk::pipeline::{Runner, CompileRunner};

/// Flags for which parts of the compile pipeline tests must go through.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TestMode {
    /// Test must parse okay
    ParseOk,
    /// Test must compile and check properly
    CompileOk,
    /// Test must fail to parse
    ParseFail,
    /// Test must fail checking or compiling
    CompileFail
}

impl TestMode {
    pub fn is_ok(&self) -> bool {
        use TestMode::*;
        match self {
            ParseOk | CompileOk => true,
            ParseFail | CompileFail => false
        }
    }
    pub fn is_err(&self) -> bool {
        !self.is_ok()
    }
}

#[derive(Debug)]
pub struct Test {
    name: String,
    path: String,
    content: String,
    mode: TestMode
}

impl Test {
    pub fn new(file_name: &AsRef<Path>, content: String) -> Test {
        let file_name = file_name.as_ref();
        let name = file_name.file_stem()
            .expect("Bad file name given to Test::new")
            .to_string_lossy().to_string();

        let mode =
            if name.ends_with("parse-ok") {
                TestMode::ParseOk
            }
            else if name.ends_with("parse-fail") {
                TestMode::ParseFail
            }
            else if name.ends_with("-ok") {
                TestMode::CompileOk
            }
            else if name.ends_with("-bad") {
                TestMode::CompileFail
            }
            else {
                panic!("Invalid test name {}", file_name.display())
            };

        Test {
            name,
            path: file_name.to_string_lossy().into(),
            content,
            mode
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn mode(&self) -> TestMode {
        self.mode
    }
}

type TestResult = Result<(), String>;

fn compile_runner(test: Test) -> TestResult {
    use env_logger::Target;
    use log::LevelFilter;
    println!("Attempting to initialize logger");
    let result = env_logger::Builder::new()
        .filter_level(LevelFilter::Debug)
        .target(Target::Stdout)
        .try_init();
    println!("Init result: {:?}", result);

    let parse_result = Runner::from_string(test.content(),
                                           test.name().to_string())
        .parse();

    if parse_result.is_err() {
        if test.mode() != TestMode::ParseFail {
            return Err(format!(
                "Failed to parse {}: {:#?}",
                test.path(),
                parse_result.expect_err("Checked for bad parse result")))
        }
        else {
            return Ok(()) // Test successful
        }
    }
    else if test.mode() == TestMode::ParseFail {
        return Err(format!("Test {} parsed unexpectedly", test.path()))
    }

    println!("Code parsed successfuly");

    let compile_result = parse_result.expect("Checked for bad parse result")
        .identify()
        .and_then(|identified| identified.check());

    if compile_result.is_err() {
        if test.mode() != TestMode::CompileFail {
            return Err(format!(
                "Failed to compile {}: {:#?}",
                test.path(),
                compile_result.expect_err("Checked for bad compile result")
            ))
        }
        else {
            return Ok(())
        }
    }
    else if test.mode() == TestMode::CompileFail {
        return Err(format!("Test {} compiled unexpectedly", test.path()))
    }

    let checked = compile_result.expect("Checked for bad compile result");

    {
        let context = Context::new();
        let mut compiler = CompileRunner::new(&context);
        let _module = compiler.compile(checked, false);
    }
    Ok(())
}

#[derive(IntegrationTests)]
struct _Placeholder;
