# Protosnirk syntax

Protosnirk is a project which will rapidly evolve over time, before solidifying
on a "complete" standard. As such all syntax is completely up for grabs and may
not reflect the end goals of the language.

Protosnirk is in a very early phase. This syntax may be entirely changed in future updates.

## Alpha 1: Numbers
*Protosnirk is not Turing-complete yet*

Protosnirk v0.1 is little more than a toy calculator that compilers 101 students
would write. A file declares variables and sets them to numerical values, or
the result of numerical expressions. Variables can be altered if declared with
`mut` and the last expression (`return` keyword optional) should be printed to stdout.

The interpreter should be able to run programs, and maintain internal state to
allow for a REPL. The only type is `f64` ("inferred"). There are no conditionals yet.

Compiler errors: invalid syntax, unknown variable, can't assign to immutable.

Runtime errors: Dividing by zero is a run time error.

```
// Comments with 2 slashes
let mut foo = 0 // no semicolons

foo = foo + 3 * 4 // Unified number type (f64) to start

let evenResult = foo % 2 // Modulo operator as well

foo // Final expression
```

This program should print `12`.

## Variables

Variables are declared as a sequence Unicode letters, numbers, and `_`. Variable names
cannot start with a number. `camelCase` names are encouraged.

Good:
`someValue`, `lúcio`, `myVariableName`, `constant2`

Allowed:
`_num_vars`, `LOUD_VARIABLE`, `l337KiLlZcOuNt`

Invalid:
`12redHens`, `$foo`

## Declarations

Variables are declared via `let`. Mutable variables can be declared with `let mut`. Mutable
variables can later be reassigned.

```
let x = 0
let mut y = 12
y = 13
```

## Values

All values in protosnirk are 64 bit floating point.
Constants are interpreted via Rust's `f64::parse()`.

`0`, `12.2`, `-0`, `223e5`, `2e3.15`, `11.5e2.45`

I forgot to parse `NaN` (and `-NaN`). I will add it.

## Expressions

Many operations in programming languages are expressions: here I mean "things that have value".
Here are some expressions:
`0`, `x`, `y`, `y / 2`, `(3 + x) / 5 % y`

Because expressions "have value", you can use them when declaring variables.
You can put any of those to the right of `let myVar =` and your program will compile.

**Statements** on the other hand, are operations that are done which you can't get any external
"value from" - for example an assignment, such as `let x = 0` or `x += 5`.

Some programs let you do this (i.e. C with its `if (x = foo())`). Right now, protosnirk does not.

Statements are kind of a superset of expressions - a program consists of a list of `statement`s,
some of which can be `expression`s. The last `statement` of a program must be an expression, however.

You can use the `return` keyword to immediately return that value to the interpreter/print it.

## Operators

Protosnirk has these mathy operators: `+`, `-`, `*`, `/`, `%`

The `%` operator is the _remainder_ operator.

You can also use them with `=` to reassign the value of a mutable variable: `y *= 4` desugars to `y = y * 4`.
