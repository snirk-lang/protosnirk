# Protosnirk modules

Protosnirk's source is split into the following modules:

## Lex

Contains the lexer which reads protosnirk syntax.

Alternate lexers may be used for languages with different syntax that can be
expressed in similar token streams (for example, using curly braces instead of tabbing)

Custom symbols can be _registered_ with the lexer.

## Parse

The parser produces syntax tree from the token stream provided by the lexer and
builds a list of constants and symbol table.

Most compile time analysis, transformations, and optimizations are done here.
Alternate parsers may be used for further optimizations, or to emit code for
a different runtime. Optimizers within a parser may be added or pipelined.

Parsing is roughly split into two stages:
1. `parse`: build the initial parse tree using TDOP parsers (Pratt parsing)
2. `verify`: confirm the semantics of the program (can't assign immutable variables)
and build supporting datasets (symbol table, constant list) to go with the parse tree.

## Compile

The `Compiler` produces a list of 3-address instructions designed to be executed on the VM.
The compiler pipeline will be expanded in the future to include an `Emitter` for writing compiled
programs to files.

## Run

The runner (VM) is a virtual machine which runs compiled code from the
parser or compiler and maintains the program state. Different virtual machines
can provide different amounts of optimization (such as emitting machine code)
or debugging features.

The VM may be expanded in the future to allow execution to be paused, program state inspected, or support REPL.
