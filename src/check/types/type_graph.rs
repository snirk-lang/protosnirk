//! A graph of type inferences.

use lex::Token;
use parse::{ScopedId, TypeId};
use identify::ConcreteType;
use check::types::InferenceSource;

use petgraph::Directed;
use petgraph::graph::{Graph, NodeIndex, EdgeIndex};
use petgraph::visit::Dfs;

use std::collections::HashMap;

/// Represents a node in the type inference graph, or
/// an rvalue in a type equation solver.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum TypeNode {
    /// Type is of a program variable with the given `ScopedId`
    VariableType(ScopedId),
    /// Type is of a concrete type written in the program, such as
    /// a primitive or a function
    ConcreteType(TypeId),
    /// Type is a temporary created from an expression.
    Expression
}

#[derive(Debug, Default)]
pub struct TypeGraph {
    /// Graph of types upon which to run unification/type inference
    graph: Graph<TypeNode, InferenceSource, Directed, u32>,
    /// TypeId -> NodeIndex
    known_type_ids: HashMap<TypeId, NodeIndex>,
    /// ScopedId -> NodeIndex
    variables: HashMap<ScopedId, NodeIndex>
}

impl TypeGraph {
    pub fn new() -> TypeGraph {
        TypeGraph::default()
    }

    pub fn add_type(&mut self, ty: TypeId) -> NodeIndex {
        if let Some(found_ix) = self.known_type_ids.get(&ty) {
            return *found_ix
        }
        let new_ix = self.graph.add_node(TypeNode::ConcreteType(ty));
        self.known_type_ids.insert(ty, new_ix);
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
    pub fn add_inference(&mut self, src: NodeIndex,
                                    dest: NodeIndex,
                                    source: InferenceSource) -> EdgeIndex {
        self.graph.add_edge(src, dest, source)
    }
    pub fn infer_type_of_var(&mut self, var: &ScopedId)
                                        -> Result<TypeId, Vec<NodeIndex>> {
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
            match &self.graph(found[0]) {
                &TypeNode::ConcreteType(ty_id) => Ok(ty_id),
                _ => unreachable!()
            }
        }
        else {
            Err(found)
        }
    }
}
