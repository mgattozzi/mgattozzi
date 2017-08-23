# Schemers - Parsers
<div class="subtitle">Published December 16th, 2016</div>

Ready for the next lesson? We'll be writing our parser now, the Read in
the REPL. We'll be using the `nom` library for parsing. It's what's
known as a parser combinator library. By building small parsers that do
one thing like parse a number or one that parses a string we can build
them up into larger ones, for instance one that could parse a string or
a number. It uses macros to build parsers. We'll start small and
parse a procedure something like:

```scheme
(+ 3 4 5)
```

Into a data structure we can use. By no means will our parser be
complete but it will give us an introduction to building one and
using it to place data into a data structure. We'll also
likely being changing these data structures as needed but it's a start!

We'll be covering some of the following things in our code:

- Enums
- Structs
- Macros
- nom
- Modules
- Deriving Traits
- Tests

Alright then, let's get started by first making our code from before
more idiomatic Rust!

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

We'll be using two new reserved keywords to accomplish this, `loop` and
`break`. `loop` does just that, loop the code inside it forever. For
example:

```rust
loop {
  println!("I'm a loop looping forever");
}
```

Will keep printing that forever till either your terminal crashes or you
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

This code does the exact same thing as before except it uses `loop`
where the `while` statement was, it gets rid of the variable `done`, and
replaces the statements where it was changed to say `true` with `break`
statements instead. Why would we do this? Why is this more Rust like?

For one, the use of `loop` explicitly tells us this will continue on for
some time and keep executing everything inside. Uses of `break` tell us
that here and only here are the conditions to cause this loop to exit no
matter what. Rust code tends to explicit and
clearly stating your intention in your code is cleaner and easier to
understand. This also removes an unnecessary mutable variable from the
scope of the program. Limiting the amount of variables that are mutable
is a good thing. You'll find that often you won't need to have mutable
data to solve your problems. While this was a small amount of code I could
have changed the `true` to `false` by accident and the code would still compile
and it becomes a runtime bug, something not even Rust can capture. Using `break`
makes it harder to change it's functionality without the compiler complaining or
you explicitly changing it on purpose.

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

## Getting nom setup
We need another library! That means we need to open up `Cargo.toml`
again. It should look sort of like this right now:

```toml
[package]
name = "schemers"
version = "0.1.0"
description = "Scheme interpreter written in Rust!"
authors = ["Michael Gattozzi <mgattozzi@gmail.com>"]
readme = "README.md"
license = "MIT OR Apache-2.0"
homepage = "https://github.com/mgattozzi/schemers"
repository = "https://github.com/mgattozzi/schemers"

[dependencies]
rustyline = "1.0.0"
```

Add this line under `rustyline`:

```toml
nom = "~2.0.0"
```

This is saying, "Use the `nom` library v2.0.0 but if there is any version
between v2.0.0 and v2.1.0 (not including v2.1.0) use that". This means we get
some bug fixes and we won't use a new minor version that might break our code a little.
Cool! Now let's import it by putting this at the top of our `main.rs` file:

```rust
#[macro_use]
extern crate nom;
```

This is saying import the `nom` crate and it tells the compiler
we'll also be using the macros that are exported from it. If you don't
place `#[macro_use]` there then we can't use them and then writing a parser
with `nom` would be possible but incredibly hard. Keep your `main.rs` file
open still. We need to do one last thing.

## Mods mods mods get your mods here
Code usually is easier to reason about if we split it up in to separate
files. In this case we'll be building our parser in a separate file
called `parser.rs`. However, we need to tell the compiler we'll be using
this in our project. This is where modules come in! By defining a module
in our main file we can then import that module with use statements
anywhere else in our program. Neat huh? First create an empty file under
`src` called `parser.rs`. Now in `main.rs` add this line somewhere near
the top:

```rust
mod parser;
```

This lets Rust know to look for the `parser.rs` file when compiling. It
now will treat anything inside the file as part of the `parser` module.
I generally will put my module definitions after my external imports and
before `use` statements in a file but it doesn't matter what the order
is since it really just comes down to personal preference. With all of
that done the header of your `main.rs` file should look like this:

```rust
extern crate rustyline;
#[macro_use]
extern crate nom;

mod parser;
```

If you want to make sure it works compile it using `cargo build` in fact
if you want to see it fail delete `parser.rs` and build the project. The
compiler will spit out a warning saying it was unable to find the
file. If everything is building then we're ready to actually start
building our parser!

## Macros and Parsing
Remember when I briefly touched on macros for `println!()`? Well we're
about to use a lot of macros and going a little more in depth about how
they work will be beneficial. We won't be writing macros which can be
kind of hard if you're unfamiliar with all of the syntax. We'll just be
covering how they work at a higher level!

What is a macro exactly? It's code that generates code at compile time.
Weird right? You can think of it as a set of rules that given input
expands out to the proper form in code. Let's look at a simple example,
the `try!()` macro. `try!()` works by taking a `Result` type as input. If
it's not an error it unwraps the data and if it is then it returns it
early. This means your function will also implement `Result` and the
error type used when using `try!()`. We'll use opening and reading from
a file as an example and we'll look at the macro and expanded version.

Let's assume we have a file with some text in it, we'll have it be
"Hello!" for this example. Here's a simple program that uses a function
`read_file` that is given a `&str`, a reference to a `String` or a non
growable `String` is a way to think about it, and opens up the file and
reads it into a `String` and returns that `String`.

```rust
use std::path::Path;
use std::fs::File;
use std::io::{Read, Error};

fn main() {
    let output = read_file("example");
    match output {
        Ok(x) => println!("The file contained: {}", x),
        Err(e) => println!("Failed to read file. Error was: {}", e),
    }
}

fn read_file(file: &str) -> Result<String, Error> {
    let path = Path::new(file);
    let mut handle = try!(File::open(path));
    let mut buffer = String::new();
    try!(handle.read_to_string(&mut buffer));
    Ok(buffer)
}
```

You'll notice I used `try!()` twice here. Think of these as operations
that could fail. In this case they're both I/O errors that could occur.
Maybe the file got deleted while reading, isn't there, or there was some kind of
corruption. The `Error` type from `std::io` will contain the appropriate
error that occurred if it does. That means it won't execute anything
else if it fails. Why though? Couldn't we just do this with regular Rust
code? Yes we can! Here is what it looks like all expanded out:

```rust
use std::path::Path;
use std::fs::File;
use std::io::{Read, Error};

fn main() {
    let output = read_file("example");
    match output {
        Ok(x) => println!("{}", x),
        Err(e) => println!("Failed to read file. Error was: {}", e),
    }
}

fn read_file(file: &str) -> Result<String, Error> {
    let mut buffer = String::new();
    let path = Path::new(file);
    let mut handle = match File::open(path) {
        Ok(a) => a,
        Err(e) => return Err(e),
    };

    match handle.read_to_string(&mut buffer) {
        Ok(a) => a,
        Err(e) => return Err(e),
    };

    Ok(buffer)
}
```

Both now do a match on the function. In each of them we use `return`
to tell Rust to stop execution and return this value. Often you
shouldn't need it since the last expression without a semicolon is the
return value. However, in this case we do need it. If we had to do this
expansion for every single `Result` type though it would be a pain.
That's why we have `try!()`.

You might have heard about the new `?` syntax that can be used in place
of `try!()`. It looks like this if being used:

```rust
fn read_file(file: &str) -> Result<String, Error> {
    let mut buffer = String::new();
    let path = Path::new(file);
    let mut handle = File::open(path)?;
    handle.read_to_string(&mut buffer)?;

    Ok(buffer)
}
```

I personally like it, others do not, really just use whichever one works
for you or is more readable since it does the same thing. It was added
syntax though because this pattern was so common in Rust programs.

Macros allow flexible code generation based off sets of rules defined in
them meaning we can have it work differently depending on the context
and inputs! Macros are quite powerful and if you're willing to learn
them you can do some real neat things. We won't be writing our own, for
the near future at least. Instead let's use some to start parsing!

# Rustacean's First Parser

Let's write a simple parser that takes input and returns a `String` type.
In `nom` everything is read in as raw bytes, represented as an array of
bytes or &[u8]. Let's actually start writing some code.

First you need to put this at the top of `parser.rs` so that it can use
functions from `nom` that aren't macros:

```rust
use nom::*;
```

Now we can write our first parser:

```rust
named!(string<&[u8], String>,
    do_parse!(
        word: ws!(alphanumeric) >>
        (String::from_utf8(word.to_vec()).unwrap())
    )
);
```

You might be confused now. That's totally fine. We're going through each
part step by step. First off the `named!()` macro. Our first input is
saying what our parsing function will be called. We'll call it
`string` and the next part between the `<>` is the type returned. The
&[u8] is saying to nom, "Look, this will be used and combined with others
and so it might actually have more data to parse. Store it as part of
the return  value!" The other part is the type of data returned from being
parsed so in this case we're returning a `String`.

If that seems a bit confusing, that's okay. It took me a bit to
understand that much myself. It might be easier for you to understand
when we make a test case to make sure this parser is working. Next up
we're using the `do_parse!()` macro. If you're familiar with Haskell it
acts loosely like do notation. If you don't know what that is don't
worry. We can use this macro to store the values of parsed results for
use later or just completely eat up a certain portion of input and move
on. In this case take a look at this line here:

```rust
word: ws!(alphanumeric) >>
```

This is saying store the result of the parsing that is caused by everything
between `:` and `>>` and store it into `word`. We can call this variable
whatever we want but for what's going on here this makes the most sense.
Now what's this `ws!()` macro? Well it just eats up white space and
returns the result of what's inside. `alphanumeric` is an inbuilt nom
parser that will return the characters A-Z, a-z, and 0-9. Because of the
`ws!()` it will stop once it hits a whitespace character or a non
alphanumeric character, whichever comes first.

The `>>` is telling the `do_parse!()` macro, "Hey this is the end of this
parser go to the next line." The stuff inside of the `()` on this line

```rust
(String::from_utf8(word.to_vec()).unwrap())
```

is what gets returned by the function the macro will generate! In this
case we're saying take whatever was stored in word turn it from a `&[u8]` to
a vector of bytes, `Vec<u8>` (`Vec` uses slices inside of it so the
promotion from `[]` to `Vec` is guaranteed to work, no `Option` or `Result`
needed!), and then turn that into a `String` and unwrap it even if it might fail.
Now we'll make this a little bit more robust later but in this case since we've been
transforming data between compatible types it's not likely to fail.
I wouldn't normally recommend that you do this. It's just easier to do to
work on an example. Cool so this should take data in and turn it into
a `String`. Let's make some tests to make sure it works!

We're going to utilize a really cool aspect of Rust. You can write
in file unit tests as well as integration tests or documentation tests
and they'll run when you use the `cargo test` command. It's amazing to
be able to do that. We're going to write a test to make sure our string
parser works. It should take in any input, get rid of the white space
and turn the first word into a string.

Put this at the bottom of your file:

```rust
#[test]
fn string_parser() {
    let comp_string = String::from("hi");

    match string(b"hi") {
        IResult::Done(_,s) => assert_eq!(comp_string, s),
        _ => panic!("Failed to parse string")
    }

    match string(b"   hi    ") {
        IResult::Done(_,s) => assert_eq!(comp_string, s),
        _ => panic!("Failed to parse string")
    }

    match string(b"hi      ") {
        IResult::Done(_,s) => assert_eq!(comp_string, s),
        _ => panic!("Failed to parse string")
    }

    match string(b"        hi") {
        IResult::Done(_,s) => assert_eq!(comp_string, s),
        _ => panic!("Failed to parse string")
    }
}
```

Let's walk through it so you know what's going on. First off look at the
`#[test]` annotation. What it's telling the Rust compiler is,"Hey, look
this function below me is a test and so you should ignore it if you're
not running tests." However if it is running tests it compiles it and
runs it. If nothing causes the function to terminate unexpectedly then
it passes otherwise it fails. It will also fail if any assertions do. We
have three types of assertions:

- `assert_eq!()` - it compares two statements for equality
- `assert_ne!()` - a recent addition, it compares two statements for inequality
- `assert!()` - a more general form, it checks for a true boolean
  statement

With them you can check that the behavior of your program is working.
Let's look at the four test cases here. They're all testing that the input
is equal to a `String` with the value of "hi". There are four different
variations of "hi" being input to our `string` function. You might notice
the b before the quotation marks. This tells Rust that this is a byte
string not an `&str`. All of the parsers are expecting input as bytes.
Now we check that it's parsed successfully. `nom` has an `IResult` enum
and that's how we know if something parsed successfully or not. We only
want it to pass if it finished parsing. So we match it on the `Done`
variant of `IResult`. Since it has two types we only want the part
parsed as a `String`. We use the `_` symbol to discard the other return
type and then assert that the string returned from the parser is indeed
just "hi". On any other type of `IResult` we've failed and so we tell
the compiler to panic and fail that test.

Our four cases vary with no white space, white space on both sides of
the word, on just the right side and just the left side. Let's see if
our parser works like we expected. Run `cargo test`. You should see
something like this:

```bash
michael@eva-01 ~/Code/Tutorials/schemers (git)-[parser] % cargo test
   Compiling schemers v0.1.0 (file:///home/michael/Code/Tutorials/schemers)
    Finished debug [unoptimized + debuginfo] target(s) in 1.33 secs
     Running target/debug/deps/schemers-9f33802606b3ac13

running 1 test
test parser::string_parser ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

Sweet! Our string parser is working and it ignores whitespace! Now we
need to build parsers for procedures we can call. There are a few
different types in scheme, Primitives, Special Forms, and then a User
procedure. What we're doing here is setting up parsers for them. The
functions and how they act are part of the Evaluator. Now we could parse
things like "if" or "+" as a string but that's not as easy to reason
about and harder to make sure we match all possibilities of a possible
special form for instance further on down the line. So let's create a way to represent
them. We'll make an `enum` for procedures which we'll call `Op` and it
will have values that will store other enums. Seems weird? Let me show
you what I mean:

```rust
#[derive(Debug, PartialEq, Eq)]
enum Op {
    Primitive(Prim),
    SpecialForm(SForm),
    User(String),
}

#[derive(Debug, PartialEq, Eq)]
enum Prim {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq, Eq)]
enum SForm {
    If,
    Begin,
    Define,
    Lambda,
    Let,
}
```

We've defined the enum Op and it has three possible values:

- Primitive
- SpecialForm
- User

You might have noticed this at the top of each of these:

```rust
#[derive(Debug, PartialEq, Eq)]
```

These are traits that can be generically derived. Rather than me having
to write out how to test for equality the Rust Compiler can figure it out
for me by implementing `PartialEq` and `Eq`. We'll be going more into
traits when we implement our own error handling but if you're familiar
with Object Oriented Programming they're like interfaces. You can read
more about them [here](https://doc.rust-lang.org/beta/book/traits.html)
if you're interested.

Remember `Option` and `Result`? Those were enums! This means we can use
match on an `Op` enum and have it act differently depending on what
needs to be done later on! Now each of those contains some internal
type. For `Primitive` there's an enum `Prim` that contains a list of types to
represent the different Scheme primitives, in this case +, -, * and /.
This isn't all of the primitives in Scheme but it's small enough of
a list to get the point across of how it works. The same thing with
`SpecialForm`. It contains an enum `SForm` that could be any of the ones
below it. In this case it's just a capitalized version of how you would
type it out in Scheme. For the `User` though we don't really know what
they would have it be. We can't really type check it so we'll just make
it a `String` for now. Right now we're focusing on parsing and not how
the evaluation works. We'll figure out how to deal with these values
later! Right now we just need a way to represent them. Let's build three
different parsers for each type in `Op` and have them return
a corresponding `Op` value. Here's what you should add to your code:

```rust
named!(specialform<&[u8], Op>,
    alt!(
        map!(tag!("if"),     |_| Op::SpecialForm(SForm::If))     |
        map!(tag!("begin"),  |_| Op::SpecialForm(SForm::Begin))  |
        map!(tag!("define"), |_| Op::SpecialForm(SForm::Define)) |
        map!(tag!("lambda"), |_| Op::SpecialForm(SForm::Lambda)) |
        map!(tag!("let"),    |_| Op::SpecialForm(SForm::Let))
    )
);

named!(primitive<&[u8], Op>,
    alt!(
        map!(tag!("*"), |_| Op::Primitive(Prim::Mul)) |
        map!(tag!("+"), |_| Op::Primitive(Prim::Add)) |
        map!(tag!("-"), |_| Op::Primitive(Prim::Sub)) |
        map!(tag!("/"), |_| Op::Primitive(Prim::Div))
    )
);

named!(user<&[u8], Op>,
    do_parse!(
        user_proc: string >>
        (Op::User(user_proc))
    )
);
```

What's this `map!()` stuff? What's a `tag!()`? What's this `|_|` and
`|`? Also `alt!()`? Really the only thing that looks like what we built
before is the `user` parser! Let's break these new macros and syntax
down. Do note the syntax inside these macros is specific to `nom` and not
Rust if that wasn't clear.

Let's start with the `user` parser. It's like `string` except we're
actually using the `string` parser here to store a `String` into `user_proc`!
We then create an `Op` of the `User` type with `user_proc` as it's internal
value! If you look `user` returns an `Op` here. What's really cool is
we used a parser we wrote in another parser we wrote! This is what it
means to use a parser *combinator* library. We're not done yet though.

Let's look at `primitive`. It's also returning an `Op` but briefly
looking over the code we can see it's returning a `Primitive` rather
than a `User`. Well what's the `alt!()` doing? It's matching on alternate
possibilities! A primitive could be a `*` or a `+` or a `-` or a `/` and so it
matches on whichever one it is. `tag!()` is just matching on some string
put in it. In this case it's single characters. What's this
`map!()` though? Well it's simply saying, if matched by the first
argument do the function on the right. We're passing it a lambda with no
inputs denoted by `|_|` and transforming the value into something else.
In this case we're just returning the value on the right of the lambda
with no transformations or anything. You might have noticed the `|` this
is how `alt!()` knows how to separate the alternate parser choices.
Knowing that let's look at `specialform`. It's acting exactly the same
as `primitive` but it's instead matching on special form keywords and
returning the value corresponding to those keywords.

Make sense? Well let's write some test cases so you can see them in
action and maybe it'll make more sense if you're a bit confused.

```rust
#[test]
fn user_op_parser() {
    match user(b"userop") {
        IResult::Done(_,s) => assert_eq!(Op::User(String::from("userop")), s),
        _ => panic!("Failed to parse userop")
    }
}

#[test]
fn specialform_op_parser() {
    match specialform(b"if") {
        IResult::Done(_,a) => assert_eq!(Op::SpecialForm(SForm::If), a),
        _ => panic!("Failed to parse special form")
    }
    match specialform(b"begin") {
        IResult::Done(_,a) => assert_eq!(Op::SpecialForm(SForm::Begin), a),
        _ => panic!("Failed to parse special form")
    }
    match specialform(b"define") {
        IResult::Done(_,a) => assert_eq!(Op::SpecialForm(SForm::Define), a),
        _ => panic!("Failed to parse special form")
    }
    match specialform(b"lambda") {
        IResult::Done(_,a) => assert_eq!(Op::SpecialForm(SForm::Lambda), a),
        _ => panic!("Failed to parse special form")
    }
    match specialform(b"let") {
        IResult::Done(_,a) => assert_eq!(Op::SpecialForm(SForm::Let), a),
        _ => panic!("Failed to parse special form")
    }
}

#[test]
fn primitive_op_parser() {
    match primitive(b"+") {
        IResult::Done(_,a) => assert_eq!(Op::Primitive(Prim::Add), a),
        _ => panic!("Failed to parse primitive")
    }
    match primitive(b"*") {
        IResult::Done(_,a) => assert_eq!(Op::Primitive(Prim::Mul), a),
        _ => panic!("Failed to parse primitive")
    }
    match primitive(b"-") {
        IResult::Done(_,a) => assert_eq!(Op::Primitive(Prim::Sub), a),
        _ => panic!("Failed to parse primitive")
    }
    match primitive(b"/") {
        IResult::Done(_,a) => assert_eq!(Op::Primitive(Prim::Div), a),
        _ => panic!("Failed to parse primitive")
    }
}

```

Given specific inputs we're expecting certain outcomes like before. If you run the
code you should get something like this:

```bash
michael@eva-01 ~/Code/Tutorials/schemers (git)-[parser] % cargo test
   Compiling schemers v0.1.0 (file:///home/michael/Code/Tutorials/schemers)
    Finished debug [unoptimized + debuginfo] target(s) in 5.4 secs
     Running target/debug/deps/schemers-9f33802606b3ac13

running 4 tests
test parser::string_parser ... ok
test parser::user_op_parser ... ok
test parser::specialform_op_parser ... ok
test parser::primitive_op_parser ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured
```

Looks like our mapping and use of `alt!()` worked out really well. Our
code now parses those words as expected. Now what if we actually wanted
to just parse for all types of operations simultaneously? Let's write
a parser that combines all of them!

```rust
named!(op<&[u8], Op>,
    alt!(
        ws!(specialform) |
        ws!(primitive)   |
        ws!(user)
    )
);
```

This parser tries the three we just made to return an `Op` and ignores
whitespace since someone might do something like this which is valid
syntax:

```scheme
(    +
  1 2 3
)
```

Let's create a test case for them.

```rust
#[test]
fn op_parser() {
    match op(b"  +   ") {
        IResult::Done(_,a) => assert_eq!(Op::Primitive(Prim::Add), a),
        _ => panic!("Failed to parse primitive")
    }
    match op(b"   let   ") {
        IResult::Done(_,a) => assert_eq!(Op::SpecialForm(SForm::Let), a),
        _ => panic!("Failed to parse special form")
    }
    match op(b"   myprocedure   ") {
        IResult::Done(_,a) => assert_eq!(Op::User(String::from("myprocedure")), a),
        _ => panic!("Failed to parse user op")
    }
}
```

Notice that we're only using the `op` parser here to test some of the
things we just did with individual parsers and we added whitespace just
to make sure the `ws!()` invocation was working. Let's test it out!

```bash
michael@eva-01 ~/Code/Tutorials/schemers (git)-[parser] % cargo test
   Compiling schemers v0.1.0 (file:///home/michael/Code/Tutorials/schemers)
    Finished debug [unoptimized + debuginfo] target(s) in 5.70 secs
     Running target/debug/deps/schemers-9f33802606b3ac13

running 5 tests
test parser::op_parser ... ok
test parser::primitive_op_parser ... ok
test parser::specialform_op_parser ... ok
test parser::string_parser ... ok
test parser::user_op_parser ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured
```

It works! I'm hoping you're beginning to see the power of being able to
build larger parsers from smaller ones. Eventually we'll just want to
call one parser on all input to have it build out our Abstract Syntax
Tree, commonly referred to as AST, so that we can then evaluate it.

We're almost done for this portion of the tutorial let's actually get
a parser built for a procedure! We're going to use a data structure
that's not completely how we're going to represent things in the future. For now
it's just to show you the results of everything we just built all
combined together!

First off we'll need a `struct`. It's a way to have multiple fields
wrapped up into one and they can have different types. Let's take a look
at what we'll be using here:

```rust
#[derive(Debug, PartialEq, Eq)]
struct Procedure {
    op: Op,
    args: Vec<String>
}
```

We're defining a `struct` and calling it a `Procedure`. It has two
fields `op` and `args`. If we make a Procedure we need `op` to be of type
`Op` and `args` to be a vector of the `String` type.

Now let's build our parser for it:

```rust
named!(procedure<&[u8], Procedure>,
    do_parse!(
        tag!("(")   >>
        op_type: op >>
        arguments: ws!(many0!(string)) >>
        tag!(")")   >>
        (Procedure { op: op_type, args: arguments  })
    )
);
```

Our `procedure` parser returns a `Procedure` `struct`. Here's how it
works. First it eats the opening `(`. Notice how we didn't store the
value at all here in this `do_parse!()`? Now we use our `op` parser and
store the value in `op_type`. What about this line starting with arguments?
Well first we want to get rid of any whitespace but what's this
`many0!()`? It's saying we're going to use this parser on the inside as
many times as possible but sometimes it might not return anything which
is the 0 part. If we use `many1!()` we are saying match the parser on
the inside once and possibly more then that. Since something like

```scheme
(my-no-argument-procedure)
```

is possible in Scheme we need to account for the fact there might not
always be arguments in a procedure call. Now we eat up the last thing in
a procedure which is a `)` then we create a `Procedure` `struct` and
return it! Let's test it out to see if it works!

```rust
#[test]
fn procedure_test() {

    let procedure_num = Procedure {
        op: Op::Primitive(Prim::Add),
        args: vec!["1".to_string(), "2".to_string(), "3".to_string(), "4".to_string()]
    };

    let procedure_user = Procedure {
        op: Op::User(String::from("myprocedure")),
        args: Vec::new()
    };

    match procedure(b"(+ 1 2 3 4)") {
        IResult::Done(_,a) => assert_eq!(procedure_num, a),
        _ => panic!("Failed to parse primitive")
    }

    match procedure(b"(myprocedure)") {
        IResult::Done(_,a) => assert_eq!(procedure_user, a),
        _ => panic!("Failed to parse primitive")
    }
}
```

First we'll create our comparison objects to be used against the parser
value and then test both of them. One has arguments the other does not
and so we should be able to tell if the `many0!()` call worked as
expected. Let's run those tests again!

```bash
michael@eva-01 ~/Code/Tutorials/schemers (git)-[parser] % cargo test
   Compiling schemers v0.1.0 (file:///home/michael/Code/Tutorials/schemers)
    Finished debug [unoptimized + debuginfo] target(s) in 1.80 secs
     Running target/debug/deps/schemers-9f33802606b3ac13

running 6 tests
test parser::procedure_test ... ok
test parser::specialform_op_parser ... ok
test parser::primitive_op_parser ... ok
test parser::op_parser ... ok
test parser::user_op_parser ... ok
test parser::string_parser ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured
```

It worked! We've built the beginning of our parser that actually can
take a procedure and turn it into a Rust data type that we can then use
to manipulate and evaluate later on. Pretty cool right?

## Exercises

1. We didn't implement all of the special form or primitive key words. Add
  the lists below to the proper `enum` and write test cases for them:

  Primitive:
  - `cons`
  - `car`
  - `cdr`
  - `list`

  Special Form:
  - `call/cc`
  - `let*`

  These aren't all of the rest of them but it should be enough for you to
  get a grasp of enums and writing test cases.

2. `Procedure` has a field `args` that's just a vector of strings. This
   isn't really what we would want in a parser. There can be multiple
   types following it like numbers or other procedures. Create an enum
   `Value` that can handle `String`, `Op`, `Procedure` or integers (make it hold an `i64` type).
   Modify `Procedure`'s `args` field to hold a `Vec<Value>` instead and modify the parser to do
   that.

## Conclusion
We covered a lot here. It might take you some time to digest that and
it's fine. We covered Rust's `struct` and `enum` types, wrote test
cases, built the beginnings of a parser and covered other smaller
concepts along the way. I recommend hand typing these out rather then
copying and pasting or just skimming over so you can get a feel for it.
Also do the exercises. Two might be a bit more difficult but try it.
You'd be surprised what you can accomplish. It's the only way you'll be
able to learn a new language.

If you have questions you can either open up an issue on the [repo itself](https://github.com/mgattozzi/schemers)
or you can ping me on Twitter or by email. I would also suggest the
\#rust-beginners irc channel as well. If you found a flaw in the
article you can open up a pull request on [my website repo](https://github.com/mgattozzi/mgattozzi)
with the changes. If you do place a note that you made corrections if
you want the credit.

All of this article's code is available at this [commit](https://github.com/mgattozzi/schemers/tree/Parser_1). You can find
the previous article [here](/scheme-ex1).
