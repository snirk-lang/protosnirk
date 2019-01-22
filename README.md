# Protosnirk

The beginnings of a programming language.

## What is this?

This repository contains the protosnirk compiler, which is written in Rust.
If you're interested in compilers (or Rust?) maybe check it out.
The language itself, as the name suggests, is rather sparse right now, but
you can check out the [tests][protosnirk-tests] folder to see what
it can do.

## What is the status? What is proto?

Languages change a lot during development. However, it can be hard to take a smaller
language seriously if it has a lot of churn or instability in its APIs,
or if it goes through large syntax changes.
Rust, in particular, went through enough change in its infancy that at the time they
were stabilizing for 1.0, there were still StackOverflow questions that would reach
the top of searches about old and disused features (such as `@mut`).

I'd like to develop protosnirk to the point of it being a serious "proof of concept" for
some of its novel features and systems. Once the basic experience of writing code and the
shape of most programs seems to be stable and useful enough, we'll roll over from
`protosnirk x.y` to `snirk 0.x.y`.

Because of this, there are parts of protosnirk that I'm not giving attention to.
For example, it's still just a [library][protosnirk-cargotoml] and the main "frontend" I
use are the [integration-tests][protosnirk-tests].

## Why is this special? Why make another programming language?

I'm tired of seeing errors pop up during runtime which could have been avoided if a
programmer could write a more clear API or if the compiler could check a few things
before compiling code.

There's not much more I can say right now, this is still "proto" after all.

## Why the name Snirk?

There are many types of names for programming languages, such as
- "Improvement upon" names like C, C++, D, C#, ObjectiveC
- Cool people names like Ada or Haskell
- Features names like Clojure, OCaml, Scala, or Smalltalk
- Cool things/fun words names like Rust, Lua, Elm, Crystal, or Boo
- Marketing names like JavaScript, Swift, NoSQL, or Go

Snirk was chosen as a kind of "fun word" category - you can't confuse it
with anything else yet and there's no need to add "lang" to the name to
avoid confusion (unless you are using [Hungarian notation][wiki-hungarian-notation]).

This means we will name source code `.snirk`, libraries `.snirklib`,
and the compiler `snirkc`.

`snirk` is ["a treasured and carefully-guarded point"][xkcd-about] in the space of
five-character strings.

## Why is this special?

It's not really special right now.

## What are some of the current features?

- I think the syntax looks nice
- Expression-based language
- Immutable-by-default variables (excitingly, are broken on `master`)
- Named-parameter calling conventions

## What are some of the planned features?

- Imperative, object oriented, multi-paradigm, etc. (the apple doesn't fall too far from the C)
- Strong static typing (no casts, coersions, or `null`s) which also enforces immutability
- Traits, sum types
- Opt-in garbage collection
- Ownership model to understand the lifetimes of data
- Freedom from data races
- Asynchronous-first I/O without red/blue functions
- Object capability system to fully understand what resources your code is using
- Compile to binary or run in interpreter
- First-class `const` data

[wiki-hungarian-notation]: https://en.wikipedia.org/wiki/Hungarian_notation
[wiki-earlang]: https://en.wikipedia.org/wiki/Erlang_(programming_language)#History
[xkcd-about]: https://xkcd.com/about/
[protosnirk-tests]: https://github.com/snirk-lang/protosnirk/tree/master/tests
[protosnirk-cargotoml]: https://github.com/snirk-lang/protosnirk/tree/master/Cargo.toml
