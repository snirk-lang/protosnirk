//! Test runner for protosnirk tests

#![allow(unused_imports)] // Some imports needed for generated code

extern crate protosnirk;
#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate derive_integration_tests;

use std::path::Path;
use std::env;
use std::fs::File;
use std::io::{Read, Write};

use protosnirk::llvm::{Context};
use protosnirk::pipeline::{Runner, CompileRunner, CompilationError};

fn init_logs() {
    use env_logger::{Builder, Target};
    use log::{LevelFilter, Level};

    let mut builder = Builder::new();
    if let Ok(filter_module) = env::var("SNIRK_LOG_MODULE") {
        builder.filter_module(
            &format!("protosnirk::{}", filter_module), LevelFilter::Trace);
    }
    else {
        builder.filter_level(LevelFilter::Debug);
    }
    builder
        .target(Target::Stdout)
        .format(|buf, record|
            writeln!(buf, "{} {}:{}  {}",
                     record.level(),
                     record.file()
                        .and_then(|path| path.split_at(4).1.split(".rs").next())
                        .unwrap_or("<file>"),
                     record.line().unwrap_or(0),
                     record.args()))
        .try_init()
        .ok();
}

fn write_graph_files() -> Option<String> {
    ::std::env::var("SNIRK_WRITE_GRAPH_FILE").ok()
}

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
            else if name.ends_with("-known-issue") {
                TestMode::CompileFail
            }
            else {
                panic!("Invalid test name {}: must end with test mode specifier",
                    file_name.display())
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
    init_logs();

    info!("Test {} source:\n\n{}", test.name(), test.content());

    let graph_file_path = write_graph_files();

    let parse_result = Runner::from_string(test.content(),
                                           test.name().to_string())
        .parse();

    if let Err(parse_error) = parse_result {
        if test.mode() != TestMode::ParseFail {
            return Err(format!(
                "Failed to parse {}: {:#?}",
                test.path(),
                parse_error))
        }
        else {
            return Ok(()) // Test successful
        }
    }
    else if test.mode() == TestMode::ParseFail {
        return Err(format!("Test {} parsed unexpectedly", test.path()))
    }

    info!("Test parsed sucessfully.\n");

    let compile_result = parse_result.expect("Checked for bad parse result")
        .identify()
        .and_then(|identified| identified.check());

    if let Err(errors) =  compile_result {
        if test.mode() != TestMode::CompileFail {
            if let Ok(print_ast) = env::var("SNIRK_PRINT_AST") {
                let unit = match errors {
                    CompilationError::IdentificationError { ref unit, .. } => unit,
                    CompilationError::CheckingError { ref unit, .. } => unit
                };
                if print_ast.to_lowercase() == "full" {
                    info!("AST:\n{:#?}\n", unit);
                }
                else {
                    info!("AST:\n{:?}\n", unit);
                }
            }
            if let Some(file_path) = graph_file_path {
                if let CompilationError::CheckingError { ref graph, .. } = errors {
                    use std::path::{Path};
                    let mut path = Path::new(&file_path)
                        .join(test.path());
                    path.set_extension("svg");
                    info!("Writing graph to {}\n",
                        path.to_str().unwrap_or("????"));
                    graph.write_svg(path);
                }
            }
            return Err(format!(
                "Failed to compile {}: {:#?}",
                test.path(),
                errors
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

    info!("Code checked sucessfully.\n");

    {
        let context = Context::new();
        let mut compiler = CompileRunner::new(&context);
        let _module = compiler.compile(checked, false);
    }
    Ok(())
}

#[derive(IntegrationTests)]
struct _Placeholder;
