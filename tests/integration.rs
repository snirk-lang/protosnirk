//! Test runner for protosnirk tests

extern crate colored;
extern crate num_cpus;
extern crate protosnirk;
extern crate workerpool;

#[macro_use]
extern crate derive_integration_tests;

use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::sync::mpsc::channel;
use std::path::{Path, PathBuf};

use colored::Colorize;
use workerpool::{Pool, thunk::{Thunk, ThunkWorker}};

use protosnirk::llvm::{Context};
use protosnirk::pipeline::{Runner, CompileRunner};

/// Represents an object which can run Snirk tests.
#[derive(Debug)]
pub struct Tester {
    root_path: PathBuf
}

impl Tester {
    pub fn new<P: AsRef<Path>>(folder: P) -> Tester {
        // Cargo places test files in a path like
        // target/debug/deps/file-{hex-mangling?}
        //    .. /  .. / .. /tests
        writeln!(io::stderr(), "\n     {} tests/{}\n",
            "Integration tests".green().bold(), folder.as_ref().display());

        let arg = env::args().next().expect("$0");
        let source_path = Path::new(&arg);
        let full_path = source_path
            .parent().expect("Bad test dir") // deps
            .parent().expect("Bad test dir") // debug/release
            .parent().expect("Bad test dir") // target
            .parent().expect("Bad test dir")
            .join("tests")
            .join(folder);

        Tester { root_path: full_path }
    }

    pub fn create_tests(&self) -> io::Result<Vec<Test>> {
        let mut tests = Vec::new();
        let mut dirs = Vec::new();
        dirs.push(self.root_path.clone());
        while let Some(dir) = dirs.pop() {
            for entry_result in try!(fs::read_dir(dir)) {
                let entry = try!(entry_result);
                if entry.path().is_dir() {
                    dirs.push(entry.path());
                }
                else {
                    let mut buffer = String::new();
                    let mut file = try!(File::open(entry.path()));
                    try!(file.read_to_string(&mut buffer));
                    //tests.push(Test::new(entry.path().as_path(), buffer));
                }
            }
        }
        Ok(tests)
    }

    pub fn test_all(
            &self,
            tests: Vec<Test>,
            runner: &'static (Fn(Test) -> TestResult + Send + Sync))
            -> Result<(), usize> {
        let name = self.root_path.file_stem()
            .expect("Checked expect")
            .to_string_lossy();

        let total_tests = tests.len();

        writeln!(io::stderr(), "found {} test files\n", total_tests);

        let pool = Pool::<ThunkWorker<()>>::with_name(name.into(),
            num_cpus::get());

        let (tx, rx) = channel();
        for test in tests {
            let thunk_tx = tx.clone();
            pool.execute(Thunk::of(move || {
                let name = test.name().to_string();
                let result = runner(test);
                thunk_tx.send((name, result)).unwrap();
            }));
        }
        drop(tx);

        let mut pass_count: usize = 0;

        while let Ok((tested, result)) = rx.recv() {
            if result.is_ok() {
                pass_count += 1;
                writeln!(io::stderr(), "file {} ... {}",
                    tested, "ok".green());
            }
            else {
                writeln!(io::stderr(), "file {} ... {}",
                    tested, "fail".red());

                writeln!(io::stderr(), "> {}",
                    result.expect_err("Checked expect"));
            }

        }

        writeln!(io::stderr(),
            "\nintegration tests: ok. {} passed; {} failed; 0 ignored;",
            pass_count, total_tests - pass_count);

        if total_tests - pass_count > 0 {
            Err(total_tests - pass_count)
        }
        else {
            Ok(())
        }
    }
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
    let parse_result = Runner::from_string(
            test.content(), test.name().to_string())
            .parse();

    if parse_result.is_err() {
        if test.mode() != TestMode::ParseFail {
            return Err(format!(
                "Failed to parse {}: {:#?} ",
                test.path(), parse_result.expect_err("Checked for bad parse result")))
        }
        else {
            return Ok(())// Test successful
        }
    }
    else if test.mode() == TestMode::ParseFail {
        return Err(format!("Test {} parsed unexpectedly", test.path()))
    }

    let compile_result = parse_result.expect("Checked for bad parse result")
        .identify()
        .and_then(|identified| identified.check());

    if compile_result.is_err() {
        if test.mode() != TestMode::CompileFail {
            return Err(format!(
                "Failed to compile {}: {:#?}",
                test.path(), compile_result.expect_err("Checked for bad compile result")
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

fn lint_runner(test: Test) -> TestResult {
    Ok(())
}

const INTEGRATION_TEST_DIRS: &[(&str, fn(Test) -> TestResult)] = &[
    ("compile", compile_runner),
    //("lint", lint_runner),
    //("run", lint_runner)
];

fn integration_tests() {
    let mut fail_count = 0;
    for (stage, runner) in INTEGRATION_TEST_DIRS {
        let tester = Tester::new(stage);
        let files = tester.create_tests()
            .expect("Unable to find test files");
        match tester.test_all(files, runner) {
            Ok(()) => {},
            Err(failed) => {
                fail_count += failed;
            }
        }
    }
    write!(io::stderr(), "\n").ok();

    if fail_count > 0 {
        panic!("{} total integration tests failed", fail_count);
    }
}

#[derive(IntegrationTests)]
struct _Placeholder;
