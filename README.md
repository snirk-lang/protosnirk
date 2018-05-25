# Protosnirk

The beginnings of a programming language.

## What is this?

This repository contains the protosnirk compiler, which is written in Rust.
If you're interested in compilers (or Rust?) maybe check it out.
The language itself, as the name suggests, is rather sparse right now, but
you can check out the [test][protosnirk-tests] folder to see what 
it can do.

## What is the status? What is proto?

Protosnirk will be "proto" until it's a "real" language. 
We'll keep adding and tweaking the basics, and once it
seems there's a good vision behind the language we'll roll over to Snirk 0.1.
Then we can really get to business and figure out how things should work.

## Why is this special? Why make another programming language?

I want to empower programmers to understand and express the constraints and
promises of their APIs - if it's going to cause undefined behavior, why not
express it in your API instead of in your docs?

There's not much more I can say right now, this is still "proto" after all.

## Why the name Snirk?

There are many types of names for programming languages, such as
- "Improvement upon" names like C, C++, D, C#, ObjectiveC
- Cool people like Ada or Haskell
- Features like Clojure, OCaml, Smalltalk
- Cool things like Rust, Lua, Elm, Boo
- Marketing like JavaScript, Swift, NoSQL, or Go

Snirk was chosen as a kind of "fun word" category - you can't confuse it
with anything else yet and there's no need to add "lang" to the name to
avoid confusion (unless you are using [Hungarian notation][wiki-hungarian-notation]).

This means we can name source code `.snirk`, libraries `.snirklib`,
and the compiler `snirkc`.

`snirk` is ["a treasured and carefully-guarded point"][xkcd-about] in the space of
five-character strings.

My handle is currently Snirk Immington - I'm certainly willing to change this
in the future if it seems like I'm naming the language after myself.

## Why is this special?

It's not really special right now. 
I think it will be special in the future, but until then, 
there are a few other serious embedded Rust languages right now 
that are worth checking out.

## What are some of the current features?

- I think the syntax looks nice
- Expression-based language
- Immutable-by-default variables
- Named-parameter calling conventions

## What are some of the planned features?

- Strong static typing (no casts, coersions, or `null`s)
- Objects, traits, sum types
- Garbage collection
- Ownership model to understand the liftimes of data
- Freedom from data races
- Asynchronous-first I/O without red/blue functions
- Object capability system to fully understand what
resources your code is using
- Compile to binary or run in interpreter

[wiki-hungarian-notation]: https://en.wikipedia.org/wiki/Hungarian_notation
[wiki-earlang]: https://en.wikipedia.org/wiki/Erlang_(programming_language)#History
[xkcd-about]: https://xkcd.com/about/
[protosnirk-tests]: https://github.com/immington-industries/protosnirk/tree/master/tests
