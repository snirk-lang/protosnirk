//! A graph of type inferences.

use lex::Token;
use ast::{ScopedId, CallArgument};
use identify::ConcreteType;
use identify::types::InferenceSource;

use petgraph::Directed;
use petgraph::graph::{Graph, NodeIndex, EdgeIndex};
use petgraph::visit::Dfs;

use std::collections::HashMap;

/// Represents a node in the type inference graph, or
/// an rvalue in a type equation solver.
#[derive(Debug, PartialEq, Clone)]
enum TypeNode {
    /// Type is of a program variable with the given `ScopedId`
    VariableType(ScopedId),
    /// Type is of a concrete type written in the program, such as
    /// a primitive or a function
    ConcreteType(ScopedId),
    /// Type is a temporary created from an expression.
    Expression,
    /// Type is the argument of a given function
    CallArg(CallArgSpecifier, NodeIndex),
    /// Type is the return type of a given function.
    CallReturn(NodeIndex),
}

/// How an argument to a function is specified
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum CallArgSpecifier {
    Index(usize),
    Name(String)
}

/// The type of `petgraph::Graph` used by the `TypeGraph`
type DirectedTypeGraph = Graph<TypeNode, InferenceSource, Directed, u32>;

/// HM type unification graph.
///
/// This data structure contains "equations" for HM type inference.
/// Instead of performing a standard unification algorithm, we instead
/// represent type constraints in a DAG and use graph traversal algorithms
/// to unify the types.
#[derive(Debug, Default)]
pub struct TypeGraph {
    /// Graph of types upon which to run unification/type inference
    graph: Graph<TypeNode, InferenceSource, Directed, u32>,
    /// TypeId -> NodeIndex
    types: HashMap<ScopedId, NodeIndex>,
    /// ScopedId -> NodeIndex
    variables: HashMap<ScopedId, NodeIndex>
}

pub const PRIMITIVE_TYPE_NAMES: &[&'static str] = &[
    "()",
    "float",
    "bool",
];

impl TypeGraph {
    pub fn with_primitives() -> TypeGraph {
        let mut curr_id = ScopedId::default().pushed().incremented();

        let mut graph = TypeGraph::default();

        for _ in 0 .. PRIMITIVE_TYPE_NAMES.len() {
            graph.add_type(curr_id.clone());
            curr_id.increment();
        }

        graph
    }

    pub fn get_type(&self, ty: &ScopedId) -> Option<NodeIndex> {
        self.types.get(ty).cloned()
    }

    pub fn get_variable(&self, var: &ScopedId) -> Option<NodeIndex> {
        self.variables.get(var).cloned()
    }

    pub fn add_type(&mut self, ty: ScopedId) -> NodeIndex {
        if let Some(found_ix) = self.types.get(&ty) {
            return *found_ix
        }
        let new_ix = self.graph.add_node(TypeNode::ConcreteType(ty.clone()));
        self.types.insert(ty, new_ix);
        new_ix
    }

    pub fn add_variable(&mut self, var: ScopedId) -> NodeIndex {
        if let Some(found_ix) = self.variables.get(&var) {
            return *found_ix
        }
        let new_ix = self.graph.add_node(TypeNode::VariableType(var.clone()));
        self.variables.insert(var, new_ix);
        new_ix
    }

    pub fn add_expression(&mut self) -> NodeIndex {
        self.graph.add_node(TypeNode::Expression)
    }

    pub fn add_named_call_arg(&mut self, name: String,
                                         fn_index: NodeIndex)
                                         -> NodeIndex {
        self.graph.add_node(TypeNode::CallArg(
            CallArgSpecifier::Name(name), fn_index))
    }

    pub fn add_call_arg(&mut self, index: usize,
                                   fn_index: NodeIndex) -> NodeIndex {
        self.graph.add_node(TypeNode::CallArg(
            CallArgSpecifier::Index(index), fn_index))
    }

    pub fn add_call_return_type(&mut self, function: NodeIndex) -> NodeIndex {
        self.graph.add_node(TypeNode::CallReturn(function))
    }

    pub fn add_inference(&mut self, src: NodeIndex,
                                    dest: NodeIndex,
                                    source: InferenceSource) -> EdgeIndex {
        self.graph.add_edge(src, dest, source)
    }
    pub fn infer_type_of_var(&mut self, var: &ScopedId)
                                        -> Result<(NodeIndex, ScopedId),
                                                   Vec<NodeIndex>> {
        let var_ix = self.variables.get(var)
            .expect("TypeGraph: asked to infer type of unknown variable");
        let mut dfs = Dfs::new(&self.graph, *var_ix);
        let mut found = Vec::new();

        while let Some(next_ix) = dfs.next(&self.graph) {
            let node = &self.graph[next_ix];
            if let &TypeNode::ConcreteType(_) = node {
                found.push(next_ix);
            }
        }
        if found.len() == 1 {
            self.graph.add_edge(var_ix.clone(), found[0],
                InferenceSource::Inferred);
            let found_ix = found[0];
            match &self.graph[found_ix] {
                &TypeNode::ConcreteType(ref id) => {
                    return Ok((found_ix, id.clone()))
                }
                _ => unreachable!("Did not add non concrete types to search")
            }
        }
        else {
            Err(found)
        }
    }
}

#[cfg(test)]
mod tests {

    use ast::*;
    use super::*;

    use petgraph::prelude::*;
    use petgraph::dot::*;

    use std::io::{Read, Write};
    use std::path::Path;
    use std::fs::{File, OpenOptions};
    use std::process::{Command, Stdio};

    use parse::tests as parse_tests;

    /// Call `dot -Tsvg` on the given file
    fn write_graph_svg<P: AsRef<Path>>(graph: &DirectedTypeGraph, path: P) {
        let dot = Dot::with_config(graph, &[Config::EdgeIndexLabel]);

        let mut dot_cmd = Command::new("dot")
                                  .arg("-Tsvg")
                                  .stdin(Stdio::piped())
                                  .stdout(Stdio::piped())
                                  .spawn()
                                  .expect("Did not have `dot` installed from graphviz");

        { // Lock stdin
            let mut stdin = dot_cmd.stdin.as_mut().expect("Couldn't get an stdin");
            write!(&mut stdin, "{:?}", dot).expect("Could not write graph");
        }
        let output = dot_cmd.wait_with_output().expect("Could not wait for dot");

        let mut output_file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
            .expect("Could not create file for svg");
        output_file.write_all(&output.stdout).expect("Could not write file for svg");
    }

    #[ignore]
    #[test]
    fn create_type_graph() {
        use identify::*;
        use check::ErrorCollector;

        ::env_logger::Builder::new().parse("TRACE").init();

        let unit = parse_tests::parser(parse_tests::FACT_AND_HELPER)
            .parse_unit()
            .expect("Failed to parse FACT_AND_HELPER");

        let mut errors = ErrorCollector::new();
        let mut name_builder = NameScopeBuilder::new();
        let mut type_builder = TypeScopeBuilder::with_primitives();
        let mut graph = TypeGraph::with_primitives();
        debug!("Running ASTIdentifier");
        ASTIdentifier::new(&mut name_builder, &mut type_builder, &mut errors).visit_unit(&unit);
        debug!("Running ASTTypeChecker");
        ASTTypeChecker::new(&mut type_builder, &mut errors, &mut graph).visit_unit(&unit);

        write_graph_svg(&graph.graph, "/tmp/type-graph.svg");
    }

}
