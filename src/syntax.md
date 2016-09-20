# Protosnirk syntax

Protosnirk is a project which will rapidly evolve over time, before solidifying
on a "complete" standard. As such all syntax is completely up for grabs and may
not reflect the end goals of the language.

Protosnirk is in a very early phase. I am working on incrementally creating a
programming language. This syntax will be entirely changed in future updates.

## Alpha 1: Numbers
*Protosnirk is not Turing-complete yet*

Protosnirk v0.1 is little more than a toy calculator that compilers 101 students
would write. A file declares variables and sets them to numerical values, or
the result of numerical expressions. Variables can be altered if declared with
`mut` and the last expression (`return` optional) should be printed to stdout.

The interpreter should be able to run programs, and maintain internal state to
allow for a REPL. The only type is `f64`. There are no conditionals yet.

Compiler errors: invalid syntax, unknown variable, can't assign to immutable,
divide by zero (constant).

Runtime errors: None?

```
// Comments with 2 slashes
let mut foo = 0 // no semicolons

foo = foo + 3 * 4 // Unified number type (f64) to start

let evenResult = foo % 2 // Modulo operator as well

foo // Program must end with "return statement"
```

This program should print `12`.
