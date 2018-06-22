//! Test runner for protosnirk tests

extern crate colored;
extern crate workerpool;
extern crate num_cpus;

use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::sync::mpsc::channel;
use std::path::{Path, PathBuf};

use colored::Colorize;
use workerpool::{Pool, thunk::{Thunk, ThunkWorker}};

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
                    tests.push(Test::new(entry.path().as_path(), buffer));
                }
            }
        }
        Ok(tests)
    }

    pub fn test_all(
            &self,
            tests: Vec<Test>,
            runner: &'static (Fn(Test) + Send + Sync))
            -> Result<(), ()> {
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
                let name = test.test_name().to_string();
                runner(test);
                thunk_tx.send(name).unwrap();
            }));
        }
        drop(tx);

        let mut tested_count = 0;

        while let Ok(tested) = rx.recv() {
            tested_count += 1;

            writeln!(io::stderr(), "file {} ... {}",
                tested, "ok".green());
        }

        writeln!(io::stderr(),
            "\nintegration tests: ok. {} passed; {} failed; 0 ignored;",
            tested_count, total_tests - tested_count);

        Ok(())
    }
}

#[derive(Debug)]
pub enum TestMode {
    Pass,
    Fail
}

#[derive(Debug)]
pub struct Test {
    test_name: String,
    file_ext: String,
    content: String
}

impl Test {
    pub fn new(file_name: &Path, content: String) -> Test {
        let name = file_name.file_stem()
            .expect("Bad file name given to Test::new")
            .to_string_lossy().to_string();
        let extension = file_name.extension()
            .expect("Bad file name given to Test::new")
            .to_string_lossy().into();
        Test {
            test_name: name,
            file_ext: extension,
            content
        }
    }

    pub fn test_name(&self) -> &str {
        &self.test_name
    }

    pub fn file_name(&self) -> String {
        format!("{}.{}", self.test_name, self.file_ext)
    }

    pub fn mode(&self) -> TestMode {
        if self.test_name.ends_with("ok") {
            TestMode::Pass
        }
        else {
            TestMode::Fail
        }
    }
}

fn lex_runner(test: Test) {

}

fn parse_runner(test: Test) {

}

fn compile_runner(test: Test) {

}

fn lint_runner(test: Test) {

}

const INTEGRATION_TEST_DIRS: &[(&str, fn(Test))] = &[
    ("lex", lex_runner),
    ("parse", parse_runner),
    ("compile", compile_runner),
    ("lint", lint_runner),
    ("run", lint_runner)
];

#[test]
fn integration_tests() {
    for (stage, runner) in INTEGRATION_TEST_DIRS {
        let tester = Tester::new(stage);
        let files = tester.create_tests()
            .expect("Unable to find test files");
        tester.test_all(files, runner).expect("lex tests failed");
    }

}
