# Schemers - Syntax Checking and Errors
<div class="subtitle">Published November 9th, 2016</div>

Ready for the next lesson? We'll be writing a simple parsing method that
just checks that all of the parentheses have a matching brace. The meat
and guts of this tutorial will be covering enums and writing your own
error type! However, we'll be touching on lots of important tangential
subjects. Here are all the things this tutorial covers:

- Writing custom Error types
- Writing a test suite
- Borrowing and references
- Iterators
- Deriving
- Documentation
- Comments


Alright then, let's get started!

## Making our code a little more Rusty

If you remember our code from Exercise 1 it looked like this:

```rust
extern crate rustyline;
fn main() {
    let mut done = false;
    let mut reader = rustyline::Editor::<()>::new();
    while !done {
        match reader.readline(">> ") {
            Ok(line) =>
                if line.trim() == "(exit)" {
                    done = true;
                } else {
                    println!("{}",line)
                },
            Err(e) => {
                use rustyline::error::ReadlineError::*;
                match e {
                    Eof | Interrupted => done = true,
                    _ => println!("Couldn't readline. Error was: {}", e),
                }
            }
        }
    }
}
```

This has one thing that really isn't Rust like and it's the use of
`done` and the `while` loop. Since we know this is a loop we want
to go forever unless certain conditions are met we should structure
it that way rather then having it do a boolean comparison each time
to figure it out!

We'll be using a two new reserved keywords to accomplish this, `loop` and
`break`. `loop` does just that, loop the code inside it forever. For
example:

```rust
loop {
  println!("I'm a loop looping forever");
}
```

Will keep printing that till either your terminal crashes or you
forcefully stop the program from executing. `break` causes whatever
type of loop it's in to stop executing and go to the next instruction in
your code. With that in mind let's rewrite what we have so far:

```rust
extern crate rustyline;
fn main() {
    let mut reader = rustyline::Editor::<()>::new();
    loop {
        match reader.readline(">> ") {
            Ok(line) =>
                if line.trim() == "(exit)" {
                    break;
                } else {
                    println!("{}",line)
                },
            Err(e) => {
                use rustyline::error::ReadlineError::*;
                match e {
                    Eof | Interrupted => break,
                    _ => println!("Couldn't readline. Error was: {}", e),
                }
            }
        }
    }
}
```

This code does the exact same thing as before, instead it uses `loop`
where the `while` statement was. It gets rid of the variable `done`, and
replaces the statements where it was changed to say `true` with `break`
statements instead. Why would we do this? Why is this more Rust like?

For one the use of `loop` explicitly tells us this will continue on for
some time and keep executing everything inside. Uses of `break` tell us
that here and only here are the conditions to cause this loop to exit no
matter what. Rust code tends to more often then not be explicit and
clearly stating your intention in your code is cleaner and easier to
understand. This also removes an unnecessary mutable variable from the
scope of the program. Limiting the amount of variables that are mutable
is a good thing. You'll find that often you won't need to have mutable
data to solve your problems. The last reason is that you're not leaving
it up to you forgetting why you need to change `done` to `true` to get it to
exit. While this was a small amount of code I could change the true to
false and the code would still compile and it becomes a runtime bug,
something not even Rust can capture. Using `break` makes it hard to
change it's functionality without the compiler complaining.

I'm going to do one more thing to make the code a bit more clear:

```rust
match e {
    // Close the program on a Ctrl-C or Ctrl-D
    Eof | Interrupted => break,
    _ => println!("Couldn't readline. Error was: {}", e),
}
```

The `//` is how we do comments in Rust. Anything after them on the line
are ignored. I've added this one here to make it a bit more clear as to
what could trigger an `Eof` or `Interrupted` error.

Cool we've cleaned up our code so let's create our error type to handle
syntax errors. For that we'll need enums and traits!

## Building a new Error type
The cool thing about Rust is that we can actually define our own types
that the compiler checks are right! This grants us a great deal of
flexibility because we can define things like our own errors for `Result`
types and we can be sure that it's always the correct error type each
time. Let's start by writing our own. One of the first things we'll be
doing is making sure the syntax in our code is actually correct. If it's
not then why bother interpreting it? In later tutorials we'll use some
advance libraries to parse all of the internal types inside parentheses.
For now though let's just write a small function to check that each
parenthese has one that it's paired up with.

For example in Scheme:

```scheme
(define x 5) ; This is valid, no unmatched parentheses
(define x 5  ; This is invalid, this has an unmatched one
```

Let's write up a general parsing error `enum` and have mismatched
parentheses be part of it. We briefly touched on enumerated lists
in the last section but we'll be diving into it more here.

At the bottom of the file you'll want to put this (the comment box is
optional):

```rust
//-----------------------------------------------------------------------------//
//                                                                             //
//                                   Errors                                    //
//                                                                             //
//-----------------------------------------------------------------------------//

enum ParseError {
    MisMatchedParen
}
```

This means we have a type `ParseError` that can contain different
values. In this case it's just one of them. Each `enum` can contain any
number of items inside of it and each item can contain some kind of data
type inside of it. Say we wanted to store the column number of the
mismatched parenthese we could write `ParseError` like this instead:

```rust
enum ParseError {
    MisMatchedParen(i32)
}
```

That means if you make a MisMatchedParen it now needs a number inside
like so:

```rust
MisMatchedParen(40)
```

We won't be doing that here so don't change it to contain a number.
I wanted to let you know that you can store data inside of it and in
fact you've been using that property with `Ok` and `Err` from the
`Result` enum this whole time!

Alright so we have a new type but that doesn't make it an error does it?
Nope. In order to count as an error that Result can use it needs to
implement the `Error` trait. Traits are like interfaces if you're used
to Object Oriented Programming. If that term is unfamiliar then you can
think of traits as certain requirements that must be implemented in
order for a data type to have the trait. If they aren't implemented then
the compilation fails. Using traits allows us to create powerful
generalized abstractions for things. In the case of `Error` it doesn't
care how I implement it for `ParseError`. It just needs the required
methods with the expected input and output types. That way when methods
that calls those functions from the `Error` trait on our `ParseError`
type it won't fail.
