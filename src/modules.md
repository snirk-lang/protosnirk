# Modules of protosnirk

Protosnirk is split into the following modules:

## Lex

Contains the lexer which reads protosnirk syntax.

Alternate lexers may be used for languages with different syntax that can be
expressed in similar parse trees.

Any compile-time validation (type checks, complex arithmetic, etc.)
should _not_ be done in the lexer.

## Parse

The parser applies transformations to the syntax tree provided by the lexer and
emits a parsed program tree and instruction lists for functions.

Any compile time analysis, transformations, or optimizations are done here.
Alternate parsers may be used for further optimizations, or to emit code for
a different runtime. Optimizers within a parser may be added or pipelined.

## Compile

The compiler module is primarily used to read and write the protosnirk chunk
format. It is mostly unused in the interpreter. Alternate compilers may be used
to read and write different binary formats.

## Run

The runner (vm) contains a virtual machine which runs compiled code from the
parser or compiler and maintains the program state. Different virtual machines
can provide different amounts of optimization (such as emitting machine code)
or debugging features.
