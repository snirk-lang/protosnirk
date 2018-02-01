# Protosnirk

A programming language built iteratively.


## What is this?

This is a "toy"/"example"/"learning" programming language.
Instead of trying to build the next Java, I start with every
compilers 101 class's first assignment: the calculator with variables.

## Why the name?

Snirk isn't actually my name (neither is Immington). I've always wanted
to create a programming language, but I alone am not in the position to
create an amazing one. So I'll learn my way through it, iterating on
prototypes, until I can create a real language, Snirk.

## What is the status? What is proto?

Protosnirk will be "proto" until it's a _real_ language - this is an excuse to
design in a "move fast, break things" fashion with the `proto` name.
Each version will add core features that "real" languages have.

## Why is this special?

It's not really special right now. I think it will be special in the future, but
until then, there are a few other serious embedded Rust
languages right now that are worth checking out.

## What are some of the current features?
- Whitespace-significant, semicolon-free syntax
- Expression-based language
- Immutable-by-default variables
- Named-parameter calling convention
- Shorthands for "block" style declaraions (`if`, `fn`)

## What are some of the planned features?
- Static typing
- Garbage collection
- Will not have `null`
- Full LLVM JIT and statically-linked support
- Classes, traits, algebraic-data-type-`enum`s
- Object onwership model and thread safety
- Compile-time-constants

## Building and Running
- `LLVM_SYS_40_FFI_WORKAROUND=1` must be set as an environment variable