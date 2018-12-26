//! Runner for compiling projects.

use lex::IterTokenizer;
use parse::{Parser, ParseError};
use ast::{Unit, visit::UnitVisitor};
use identify::{
    NameScopeBuilder, TypeScopeBuilder, ASTIdentifier, ASTTypeChecker, TypeGraph};
use check::{ErrorCollector, TypeConcretifier, TypeMapping};
use compile::{ModuleCompiler, SimpleModuleProvider};
use llvm::{Context, Builder};

use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::str::Chars;
use std::io::{self, Read};

#[derive(Debug)]
pub enum CompilationError {
    IdentificationError {
        name_builder: NameScopeBuilder,
        type_builder: TypeScopeBuilder,
        errors: ErrorCollector
    },
    CheckingError {
        type_builder: TypeScopeBuilder,
        graph: TypeGraph,
        errors: ErrorCollector
    }
}

#[derive(Debug)]
pub struct Runner<'input> {
    iter: IterTokenizer<Chars<'input>>,
    name: String
}

impl<'input> Runner<'input> {
    pub fn from_string(text: &'input str, name: String) -> Runner<'input> {
        Runner {
            iter: IterTokenizer::new(text.chars()),
            name
        }
    }
    pub fn from_file<P: AsRef<Path>>(path: P, buffer: &'input mut String)
                                     -> io::Result<Runner<'input>> {
        let name = path.as_ref().to_string_lossy().into();
        let mut file = try!(File::open(path));
        try!(file.read_to_string(buffer));
        Ok(Runner::from_string(buffer, name))
    }

    pub fn parse(self) -> Result<IdentifyRunner, ParseError> {
        let mut parser = Parser::new(self.iter);
        let unit = try!(parser.parse_unit());
        Ok(IdentifyRunner::new(unit, self.name))
    }
}

#[derive(Debug)]
pub struct IdentifyRunner {
    name: String,
    unit: Unit,
    errors: ErrorCollector,
    name_builder: NameScopeBuilder,
    type_builder: TypeScopeBuilder,
    graph: TypeGraph
}

impl IdentifyRunner {
    fn new(unit: Unit, name: String) -> IdentifyRunner {
        IdentifyRunner {
            unit, name,
            errors: ErrorCollector::new(),
            name_builder: NameScopeBuilder::new(),
            type_builder: TypeScopeBuilder::with_primitives(),
            graph: TypeGraph::with_primitives()
        }
    }

    pub fn identify(mut self) -> Result<CheckRunner, CompilationError> {
        ASTIdentifier::new(&mut self.name_builder,
                           &mut self.type_builder,
                           &mut self.errors)
            .visit_unit(&self.unit);
        if !self.errors.errors().is_empty() {
            error!("IdentifyRunner: failed ASTIdentifer");
            return Err(CompilationError::IdentificationError {
                name_builder: self.name_builder,
                type_builder: self.type_builder,
                errors: self.errors
            })
        }
        ASTTypeChecker::new(&mut self.type_builder,
            &mut self.graph,
            &mut self.errors)
            .visit_unit(&self.unit);
        if !self.errors.errors().is_empty() {
            error!("IdentifyRunner: failed ASTTypeChecker");
            Err(CompilationError::CheckingError {
                type_builder: self.type_builder,
                graph: self.graph,
                errors: self.errors
            })
        }
        else {
            Ok(CheckRunner::new(self))
        }
    }
}

#[derive(Debug)]
pub struct CheckRunner {
    unit: Unit,
    name: String,
    errors: ErrorCollector,
    name_builder: NameScopeBuilder,
    type_builder: TypeScopeBuilder,
    graph: TypeGraph
}

impl CheckRunner {
    fn new(runner: IdentifyRunner) -> CheckRunner {
        CheckRunner {
            unit: runner.unit,
            name: runner.name,
            errors: runner.errors,
            name_builder: runner.name_builder,
            type_builder: runner.type_builder,
            graph: runner.graph
        }
    }

    pub fn check(mut self) -> Result<CheckedUnit, CompilationError> {
        let results = {
            let mut tc = TypeConcretifier::new(&self.type_builder,
                                               &mut self.errors,
                                               &mut self.graph);
            tc.visit_unit(&self.unit);
            tc.into_results()
        };
        if !self.errors.errors().is_empty() {
            error!("CheckRunner: failed to type concretify");
            Err(CompilationError::CheckingError {
                type_builder: self.type_builder,
                graph: self.graph,
                errors: self.errors
            })
        }
        else {
            Ok(CheckedUnit::new(self.unit, self.name, results))
        }
    }
}

#[derive(Debug)]
pub struct CheckedUnit {
    unit: Unit,
    name: String,
    map: TypeMapping
}
impl CheckedUnit {
    fn new(unit: Unit, name: String, map: TypeMapping) -> CheckedUnit {
        CheckedUnit { unit, name, map }
    }

    pub fn unit(&self) -> &Unit {
        &self.unit
    }

    pub fn type_map(&self) -> &TypeMapping {
        &self.map
    }
}

pub struct CompileRunner<'ctx> {
    context: &'ctx Context
}
impl<'ctx> CompileRunner<'ctx> {
    pub fn new(context: &'ctx Context) -> CompileRunner<'ctx> {
        CompileRunner { context }
    }

    pub fn compile(&mut self, unit: CheckedUnit, optimizations: bool)
                   -> SimpleModuleProvider<'ctx> {
        let module = self.context.new_module(&unit.name);
        {
            let builder = Builder::new(&self.context);
            let mut ir_code = Vec::new();
            let mut scopes = HashMap::new();
            {
                let module_provider = SimpleModuleProvider::new(module,
                                                                false);
                let mut compiler = ModuleCompiler::new(unit.map,
                    module_provider,
                    &self.context,
                    &builder,
                    &mut ir_code,
                    &mut scopes,
                    optimizations);
                compiler.visit_unit(&unit.unit);

                let (provider, _types) = compiler.decompose();
                provider
            }
        }
    }
}
