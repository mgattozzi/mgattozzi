---
layout: post
title: Schemers - Input
---

In my CS undergrad right now I've been taking an interesting course on the
*Structure of Higher Level Languages*. It's quite the eye opening class
and one of the things we are doing there is implementing a Scheme
interpreter in Scheme. I thought it would be fun to do something like a
tutorial such as [Write Yourself a Scheme in 48 Hours](https://en.wikibooks.org/wiki/Write_Yourself_a_Scheme_in_48_Hours)
to teach others some Rust and create a Scheme interpreter. This series of
blog posts is going to be aimed at people completely new to the language
who have never touched it before. We'll be implementing the [R5RS](http://www.schemers.org/Documents/Standards/R5RS/HTML/)\* version of Scheme since it's considered the good version. Don't worry too much
about all the language in the specification. It's just verbose so we
know *exactly* how to define it in our implementation. The main thing is
that we learn some Rust and have some fun doing it!

## A little history on Scheme
Scheme is a derivative of LISP or LISt Processor (jokingly referred to as
'Lots of Irritating Stupid Parentheses' as you will soon come to realize).
LISP was one of the first programming languages created by an MIT professor
name John McCarthy back in the 1950's and first implemented on an old
computer known as the IBM 704. Scheme was a variant developed at MIT in the
1970's. The language itself is simple enough to understand but the implications,
and the things that can be learned from it apply to languages both young and
old today. I'd recommend learning it a bit so you can be familiar with it when
we go through each tutorial. [Here](https://mitpress.mit.edu/sicp/) is a great
place to start learning it. It's the text I've been using in class and provides
great examples to learn from as well as exercises you can use to learn it.

## What we're covering
We won't be writing any Scheme in this tutorial. All we're going to do
is get the beginning of a REPL (Read Eval Print Loop) setup for us to
use. It'll keep taking input from us and printing out what we typed in
and loop until we put in `(exit)` as input.

Here are the Rust topics we'll cover:

- Setting up a Rust Environment using Rustup
- Compiling your first program
- Cargo.toml files
- External imports
- Rust `while` loops
- Rust `if else` statements
- Rust `match` statements
- Rust `Result` types
- Mutability vs. Immutability

We'll cover topics in Rust as we need them. By the end of this tutorial
you should have a basic Read Print Loop that we can exit from. With that
in mind let's setup our project.

## Let's get setup
Rust has two main tools to actually build your code, `rustc` which is
the Rust compiler and `cargo` it's package manager and build system.
We'll need both installed if we want to get anything done. We'll need
`cargo` to setup our project and have it build our project without us
having to mess around with getting dependencies, linking them, and how
to compile it all. We'll need `rustc` to actually build the project. The
easiest way, and the one that will be the supported way in the future is
using `rustup`. Don't let it's beta status scare you. It's fairly
stable! `rustup` gets us all of these components and installs them for
us. On top of that it also allows us to switch between the three release
channels (stable, beta, and nightly). We'll be working with the stable
compiler for this tutorial. You should still know what all the release
channels are for though! Stable is the last compiler version that the
dev team has marked as, well stable. It shouldn't cause any breakage or
errors. If it does that's really not good and you should file a bug.
Beta is for features that will be stabilized in the next release and
this lets us test them out as a release candidate just to make sure we
can fix any possible breaking changes in our code. Nightly is where the
kiddie gloves come off. It can break at any time, however here you get
to test all the crazy new features before they get considered for
stabilization. Some projects even only work with nightly because they
depend on certain features. Release cycles are every six weeks so you
get to have new features on a consistent quick schedule. It's great!

Alright, so you know about the tools we use and need so let's actually
install them! First up we need to get `rustup` installed. Without it we
won't be able to get anything working. We'll be doing most of our work
from the command line for this part. Open up your terminal and put this
in:

```bash
curl https://sh.rustup.rs -sSf | sh
```

Just follow the on screen instructions. At the very least you'll want
the stable channel installed and to be your default.

Run the following commands to make sure

```bash
rustc --version
cargo --version
```

`Cargo` might be output as a nightly with it's version. If `rustc` comes
out without a beta or nightly tag in it's name then this is the version
of `cargo` that goes with it. What we're really checking for is that the
compiler and `cargo` are there and that we can use the tools. At the
time of this writing `rustc` is on version 1.12.1 but any stable version
after it should work as well. If it's not the stable version then type
the following:

```bash
rustup default stable
```

Then check that it's the stable version. Cool we've got all of our tools now!

## Initialize the project
Now that we have our tools installed let's start using them. First thing
we're going to do is create a project for a binary. We're going to call
our project `schemers` in this tutorial but you can name your project
whatever you want! You'll need to run this command:

```bash
cargo new --bin schemers
```

We're telling `cargo` to create a new project that is a binary that we
can run, thus the bin flag, and we're going to call it `schemers`. If
you drop the bin flag it automatically creates a library project. We
won't be covering that in this tutorial but essentially it's a packages
of functions you can write and publish that others can use. We'll be
using one of these libraries in our own project! That's later though.
Let's take a look inside our project directory. It'll look like this:

```
.
├── Cargo.toml
├── .gitignore
├── .git
│   └── //ommitted
└── src
    └── main.rs
```

Let's start with the `git` portion. `cargo` automatically initializes
the directory as a git repository and contains a `.gitignore` file that
contains common things that Rust generates that you wouldn't want to
check into source control. It differs between the binary and the
library. In our case `.gitignore` only contains the `target` directory
which is where everything that's compiled is put in.

`Cargo.toml` is our configuration file where we list our dependencies as
well as provide metadata about the package itself such as the license
used or it's name. We'll come back to this later but if you're curious
take a peak!

The last part is our `src` directory. This is where our source code
lives and where `cargo` checks for files to compile so it can invoke `rustc` to
actually make the binary. `main.rs` is the entry point for any binary
in Rust and `lib.rs` is the entry point for any library.

## I fought immutability and immutability won
The code is actually setup to do the classic "Hello world!" already.
Just use `cargo run` in the terminal and it'll compile and print it out. Let's face it
though. It's an old and boring example. Let's actually learn
something by running into compiler errors! I'm not kidding. One of the
fastest ways to learn Rust is to face failure with it head on. You'll
learn more from that, especially since we have some shiny new error
messages!

Okay first let's setup a loop with a variable `done` that lets us loop
until we change done to true.

Open up your `main.rs` file with whatever editor you want. You should
change it to look like this:

```rust
fn main() {
  let done = false;
  while !done {
    done = true;
  }
}
```

Let's go through each line before we run this. First you'll see our
function header `main`. `fn` tells us that whatever comes next is
a function. `main()` tells us it's name is `main` and has no parameters
as input (the `()`) and we denote the body of the function with an
opening `{`. These function headers can be more involved to say
different things and we'll cover them eventually, but for now this is
how you declare a basic function. Now let's look at the next line:

```rust
let done = false;
```

`let` is the keyword that Rust uses to declare a variable. Although Rust
is strongly typed, unlike languages like Java, we don't need to declare
the type every time we declare a variable. The compiler is pretty smart to
infer what type it is. Sometimes though you'll need to let it know but
those times are few and far between. The `=` lets us know this is an
assignment of a value on the right to the variable on the left. `false`
is how we show a boolean in Rust (`true` being the opposite value). We
denote statements have ended with semicolons like many other C like syntax
languages.

```rust
  while !done {
    done = true;
  }
```

This is the syntax for a `while` loop. No parentheses needed, just
`while` then the boolean statement you want to evaluate, in this case
`not done`. Then we assign true to done! Let's give it a whirl and
compile it. Run the command `cargo run` and it'll attempt to compile the
code.

Except, it's going to fail. Let's take a look at the error message:

```
error[E0384]: re-assignment of immutable variable `done`
 --> src/main.rs:4:9
  |
2 |     let done = false;
  |         ---- first assignment to `done`
3 |     while !done {
4 |         done = true;
  |         ^^^^^^^^^^^ re-assignment of immutable variable
```

I didn't include a crucial detail about this code. When you assign
variable names with `let` then Rust makes the variable immutable.
Meaning no matter what you do, if you try to do something to change it,
Rust will throw an error. This means if you mean to mutate a variable
you'll get an error if you forgot to make it mutable. You'll also get
a warning if you make a variable mutable and don't actually change it!
This might seem weird if you've never dealt with languages where things
are immutable by default (like Haskell), but it's a great thing trust
me! It clearly lets you know what's changing, what's allowed to change,
and will warn you in the event something that shouldn't happen does. The
compiler is the debugger for your mind. It reduces the need for you to
think about the state of the program and let you do the important things
like writing new features. If you run into errors writing Rust, it'll feel
frustrating at first, but you'll soon see that you'll run into them less
over time. In fact you might even come to love them!

Good points aside, you're probably wondering how to fix this whole problem.
Must be hard right? Nope! We just need to make `done` a mutable variable.
Change the `let` statement to say this instead:

```rust
let mut done = false;
```

The `mut` keyword is how we tell Rust that a variable is mutable and so
we can reassign values to it if we need too. Now if you use `cargo run`
again you'll see that it compiled and ran. Our program doesn't actually
do anything right now. It's quite useless. Let's change that by getting
ourselves some input from users to print out!

## Depend on dependable dependencies
We need a way to get input from users, but we also want them to move
the cursor around, and later on we might want to save that history so
they can go back and use an old command again. We could write our own
implementation, but dealing with display buffers and terminal keys just
doesn't seem like a fun time. I want to write my program not reimplement
what's been implemented! Luckily for us we have
[crates.io](https://crates.io) a site for Rust programmers to upload
packages they've written and a place where others can download them to
use the code in their own! Did I mention you can publish a library using
`cargo` and that you can also use it to pull in dependencies for you
automatically? Pretty neat huh? Let's get this into our code then!

First up we are going to modify our `Cargo.toml` file so we can list our
new dependency. We'll be using a library called `rustyline` a library
based off of the famous `readline`, minus all of the C code. It has all of
the features I mentioned before and works really well for our REPL. If
you open up your file you'll see something that looks like this:

```toml
[package]
name = "schemers"
version = "0.1.0"
authors = ["Michael Gattozzi <mgattozzi@gmail.com>"]

[dependencies]
```

This is where your metadata and dependencies live. The configuration
file is much more human readable compared to something like JSON or
XML. There's a ton of options you can configure this file with and
I encourage you to look through the documentation
[here](http://doc.crates.io/manifest.html). However, we
won't be covering it right now. What I want to teach you is how to put
in a new dependency and get it in your code! Add this line to your file:

```toml
[dependencies]
rustyline = "1.0.0"
```

What we're saying is "Cargo add rustyline as a dependency to our code
and use the 1.0.0 version of the package." It makes it really easy to
specify versions and dependencies and when you do `cargo run` again
it'll pull it automatically from `crates.io` for you and link it in
automagically. Okay but putting it in our file isn't enough really. It
won't actually be in our codebase. Let's open up `main.rs` again and add
this to the top of the file:

```rust
extern crate rustyline;
```

Let's look at this a little bit. First `extern` means we're using code
outside of Rust or we might be exporting code outside of Rust to be
called from other languages or vice versa. In this case though check the
next word `crate`. This means we'll be using an external library (or
crate in Rust parlance) and that it's name is `rustyline`. In most of
your use cases you'll be using `extern crate` to define dependencies.
Don't worry too much about the FFI bit. That's more of a "for your
knowledge" then anything else and to let you know it's possible! Now
do `cargo run` again. You might notice this time it's pulling in
`rustyline` and it's dependencies, then compiling it, then your code,
then running it! Neat huh? You didn't even have to tell `cargo` how to
get it, compile it, or link it to your code. This is one of it's many
strengths that can be leveraged to write code without worrying about the
small details. Let's actually write some code using our new library we
imported!

Change your `main.rs` to look like this:

```rust
extern crate rustyline;
fn main() {
    let mut done = false;
    let mut reader = rustyline::Editor::<()>::new();
    while !done {
        match reader.readline(">> ") {
            Ok(line) =>
                if line == "(exit)" {
                    done = true;
                } else {
                    println!("{}",line);
                },
            Err(e) => println!("Couldn't readline. Error was: {}", e),
        }
    }
}
```

Okay, what? There are a lot of new things I just threw in your face. Good
news is this is the rest of the code we'll be writing for this tutorial.
Before we run it though let's break it down so you can understand what's
going on, line by line.

```rust
extern crate rustyline;
fn main() {
```

We covered this before but let's reiterate the point:

- We're importing a crate `rustyline` for use in this file
- We've declared a `main` function so that when we execute the binary we
  know where to start our program's execution

```rust
let mut done = false;
let mut reader = rustyline::Editor::<()>::new();
```

We've seen that first line before, we've created a boolean variable
called `done` that can change it's value. We've also set it to `false`.
What's that next line though? Well we've declared a variable `reader` that
is mutable. Okay, that makes sense, what's all the stuff on the right?
Well Rust borrows this syntax style from C++. What we're telling the
compiler is how to find the method we want it to use. `rustyline::` is
saying look in the `rustyline` crate. `Editor` is saying use a function
from the `Editor struct`. A `struct` in Rust is a way to store various data type
in fields we can access. We can also implement functions that manipulate
or create these `structs` we've designed. We'll cover this more in depth
in the future but this should be enough to understand what's going on
now. `::<()>::` is a type declaration for the `rustyline` editor. This
isn't all that important right now for what we're doing, we just need it
there to compile. It might seem like this is a bit hand wavy saying not
to worry about it, really I'm trying to focus on the syntax here and
giving you a basic understanding of concepts and elaborating on them as
we progress. Bear with me in this respect. Overloading you with too much
too fast won't help and only serves to cause frustration. `new()` is the
important bit here. This function is the one that actually creates an
`Editor struct` that we can use to get input from the user and assigns
it to the variable `reader`!

```rust
while !done {
    match reader.readline(">> ") {
```

We covered the `while` loop earlier in the article. As long as `!done`
evaluates to `true` the code inside will continue to loop. The interesting
bit is this `match` statement here. What is it? If you're familiar with
`switch` or `case` statements in other languages it's like that except
slightly more powerful because we can implement pattern matching,
a powerful concept that languages like Haskell have! In this statement
it's saying, use the `readline` function from our `Editor struct` and
make it's prompt look like `>> `. When a user types in input and presses
enter we'll pattern match on whatever data it returns! Nifty huh? Let's
look at the final block of code to understand what happens once we get
user input.

```rust
Ok(line) =>
    if line == "(exit)" {
        done = true;
    } else {
        println!("{}",line);
    },
Err(e) => println!("Couldn't readline. Error was: {}", e),
```

The function `readline` has a return type of `Result<String, ReadLineError>`.
`ReadLineError` is a custom error from the `rustyline` library. If you've
used Haskell before `Result` is like the `Either` monad. If you haven't
then this should help explain things. When we compute things sometimes
we get errors, for instance you divide a number by 0 or something like
that. If the program panicked and crashed every time something like that
happened then as a programmer it would be really hard to deal with
something like that. We wouldn't have control over how to handle the
error. What if the calculation was successful? Then we need to be able
to return that value. However, if a function could fail or work then we
can't return one type because it could be either a failure we would want
to handle ourselves, or the result of a successful computation. That's
where `Result` comes in! It keeps track of whether it was a failure or
a success. We'll know what type of failure it could be or what type of
success it will be if it returns a value with `Result`. By pattern
matching on it we can determine what to do if there is an error or if it
worked. We also can unwrap the values inside an `Ok` (successful
computation) or an `Err` (unsuccessful computation) and use that value
in another computation! Let's look at the easier case here, `Err(e)`.
What we're saying is if the `Result` type returns an error bind the
inner data of the error to the variable `e`. `=>` is the syntax we use
to say "if we match this pattern do what's to the right of me." The , at
the end is how you end each pattern in a `match` statement. Now let's look at what
happens if we get an error. `println!` is what's known as a macro in
Rust. They can get pretty crazy with what they can do but in this case
it figures out the input and what to print out to the console! Our first
argument is a `String`. We'll dive into the nitty gritty of `Strings` in
the future since it can be a confusing topic to new users of Rust. For
now you need to know that your first argument of input will be printed.
Note the `{}` in the text. This is where the value of `e` will be placed
when printed out! Each `{}` that's in the first argument corresponds to
a variable added as an argument. To make that clear:

```rust
let a = "Hello";
let b = "world";
println!("{} {}!", a ,b);
// This prints out "Hello world!"
println!("{} {}!", b ,a);
// This prints out "world Hello!"
```

You can add as few or as many `{}` as you want in your printed output,
you'll just need a corresponding variable to fill it. Let's look at our
use case again:

```rust
println!("Couldn't readline. Error was: {}", e)
```

Any time we get an error when getting input for `readline` it'll print
out "Couldn't readline. Error was: " and it will include the error as
part of the output. You'll run an example to see what this looks like
yourself soon enough!

Let's look at the `Ok(line)` statement.

```rust
Ok(line) =>
    if line == "(exit)" {
        done = true;
    } else {
        println!("{}",line);
    },
```

What's going on here is "we got input from the user and we've stored it
into the variable line". The statement following it is an `if else`
statement. Like `while`, Rust doesn't use the parentheses around what you
want to evaluate as a boolean statement like some other languages. I find
this to be more readable and reminds me of Python's syntax. What we're
saying here is, "If the user has input the string (exit) then make done true
so that the program will exit on the next loop, otherwise print out that
value to the command prompt and get the next input from them." Simple right?

Let's actually run it and try it out! Type type in `cargo run` and watch
what happens. After a lot of output you should see `>> ` pop up on your
screen! Type in some things and press enter and watch it get put on to
your screen!

```
>> hello
hello
>> goodbye
goodbye
```

Let's see if we can get it to print out our error message. Hit Ctrl-c
then Ctrl-d. You should see the following outputs:

```
>>
Couldn't readline. Error was: Interrupted
>>
Couldn't readline. Error was: EOF
```

Now type in (exit) your program should close out!

```
>> (exit)
michael@kotetsujo ~/Code/schemers (git)-[master] %
```

Cool huh? You've written your first program in Rust and it can read and
evaluate input! I'm going to give you some exercises to try out between
now and the next post. Unlike many other "exercise left to the reader"
tutorials I'll give you an answer in the next post on how to do it in
case you couldn't figure it out. You should really try though because
that's how you'll learn. If you run into difficulty ask on the
\#rust-beginners irc channel for help! The community is great and loves
to help new users.

## Exercises
Here's what I want you to do:

- Modify the `Err` line so that the program exits gracefully on an EOF
  or Interrupted, but prints an error out like before otherwise.
  (Hint: You'll need to modify `done` here and check for the error
  somehow)
- What happens when you modify ">> " to be something else? If you
  understand what is happening when you change it, modify it to be
  something you like!
- What happens when I put in something like "      (exit)      " to the
  interpreter? What method in the [standard
  library](https://doc.rust-lang.org/std/string/struct.String.html) would get rid of
  the whitespace? Find the method in the linked documentation and use it
  in the interpreter so that "    (exit)", "(exit)    ", and "(exit)"
  all cause the program to exit.

## Conclusion
We've covered a lot, like getting Rust all setup, setting up a project,
a little bit of Rust syntax, mutability, and how to get dependencies
into your project. This is but a sample of what we can do with Rust. We
grazed over some topics because there are situations where it will be
more relevant and worth diving into in depth. Don't worry if you run
into errors or feel frustrated. The community loves helping new users
and we want to make this easier for you to understand by helping you
over that initial hump. The irc channels are a great way to learn and
ask questions on rather than a post and waiting for an answer. It's
a resource you should really use. Ask, we were all Rust beginners once
and we know it can be hard at times. We want to make this experience
easier for you.

Hopefully you learned some things here and will continue to learn Rust!
The next article will be implementing a parser to actually be able to
turn our input into something that we can later evaluate as Scheme code.
We'll also be checking to make sure that the code doesn't have any
syntax errors (at least in terms of `()` not lining up properly). If you
have comments or questions feel free to ping me on Twitter or open up
issues on the repository itself where all of this code is hosted!

You can find all of the code from this article in the schemers repo [here](https://github.com/mgattozzi/schemers/tree/Input).
The next article can be found [here](https://mgattozzi.github.io/2016/11/09/scheme-ex1.html).

\* It should be noted prior to writing this article I had no idea that
schemers was the name of the website. This project is not associated
with that website at all. We're just using it to reference R5RS.
