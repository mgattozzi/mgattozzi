---
layout: post
title: FFI with Haskell and Rust
---

I've had an idea percolating for a while. I love using Haskell for work.
It's functional, expressive, and easy to reason about. It's also
strongly typed and deals with immutable data (for the most part) which are
big pluses for writing good code. However, it's garbage collected,
and while it's not likely to run into performance issues for small
projects, on larger ones it can get in the way. Commonly with Haskell,
if one really needed speed, the FFI would be used with C code in order
to get the performance needed. The only problem with that is that C is
unsafe with undefined behavior being the norm. This is where Rust is
a great alternative C for those performance critical functions needed in
some Haskell problems. If you've ever used Haskell before you know that
the documentation is either outdated, unrelated to what you actually
want, or just plain missing. I spent hours digging through old Haskell and
Rust blog posts, stack overflow posts from 2009, GHC options, and god
knows how many compiler errors. Rather than having you dig through the depths
of the archaeological nightmare that is the Internet, I've decided to
write this for anyone trying to use Rust in Haskell. Let me be clear,
this article is not covering how to use Haskell in Rust. That's for some
other time.

Let's get started then.

### Tooling
This articles assumes you have the following installed:

- rustc - at least 1.12 stable
- cargo
- cabal
- GHC

Why not stack? It kept yelling at me for weird things. I like it but
this is a simple example and cabal will suffice. Besides stack is built
on top of cabal so you should already have it anyways.

### Setting up our Rust Code
First let's setup our project directory. Initialize it with the
following command:

```bash
cargo new haskellrs
```

This well setup a folder haskellrs that's laid out like this:

```
.
├── Cargo.toml
└── src
    └── lib.rs
```

Before we start setting up our Haskell code as well let's get our Rust
code straightened out. First we need to modify our Cargo.toml file. When
you open it up it should look something like this:

```toml
[package]
name = "haskellrs"
version = "0.1.0"
authors = ["Michael Gattozzi <mgattozzi@gmail.com>"]

[dependencies]
```

We need to modify it to look like this:

```toml
[package]
name = "haskellrs"
version = "0.1.0"
authors = ["Michael Gattozzi <mgattozzi@gmail.com>"]

[lib]
name = "rusty"
crate-type = ["staticlib"]

[dependencies]
```

If you've done FFI before you might have noticed I chose to statically
link this file rather than dynamically link it. I have yet to get an
example working with dylib or the new cdylib format for Rust and would love to
know how if you've done it before. The three new lines added let Rust
know a few things:

1. The output file will be a .a file called librusty.a (The common
convention for C code that's a library to start with lib and then the
name of the project).
2. This file can get statically linked into other programs because we've
defined it to be a static lib.
3. It knows how to setup the file differently with it's symbols for
other programs. Rust normally uses .rlib files to link Rust code to
other Rust code when you pull in external dependencies with Cargo. We
don't want that since Haskell would have no idea how to understand that
file format.

Okay so our project is setup! Let's start writing some Rust! Open up
your src/lib.rs file. It should look like this:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
```

We don't need this for our purposes so feel free to delete it. Instead
we're going to write two simple functions so that we can see how this
works. One with `CStr` and one with `i32`. First up we're going to write
a function called `double_input` that takes an `i32` and doubles it. It
should look like this:

```rust
#[no_mangle]
pub extern fn double_input(x: i32) -> i32 {
    2 * x
}
```

Let's go through each line:

1. The `#[no_mangle]` tells the Rust compiler not to do anything weird
with the symbols of this function when compiled because we need to be
able to call it from other languages. This is needed if you plan on
doing any FFI. Not doing so means you won't be able to reference it in
other languages
2. Now our actual function header. `pub` makes it available to use
elsewhere. `extern` means this is externally available outside our
library and tells the compiler to follow the C calling convention when
compiling. If you don't know what that means, it has to do with how code
leaves values available for functions that called it on the CPU level.
You can find more information about it
[here](https://en.wikipedia.org/wiki/X86_calling_conventions#cdecl) if
you are interested on learning more. We need to do this so Haskell knows
how to treat the Rust code. For all intents and purposes Haskell thinks
it's calling C compiled code and not Rust. Now we finish with the rest
of the function. `fn` tells us this is a function we are declaring,
`double_input` is the function name `(x: i32)` means it has an input `x` of
type `i32` and `-> i32` means we are returning an `i32`.
3. Our next line is simple take `x` and multiply it by 2. Since it's the
last line in the Rust function by omitting the ; at the end we're
telling the compiler return the value of the expression, much like how
the last expression of a `do` block in Haskell returns a value.

Pretty simple right? The only difference between FFI code and Rust only code
is the `extern` and `#[no_mangle]` statements put into the function
header.

Okay now let's write a function that prints Strings passed to it. Since
we're using FFI we need to use `CStr` so that both languages know how
to talk to the other. At the top of lib.rs add the following line:

```rust
use std::ffi::CStr;
use std::os::raw::c_char;
```

Now we're going to write our printing function:

```rust
#[no_mangle]
pub extern fn print_string(x: *const c_char) {
  unsafe {
    let cstring = CStr::from_ptr(x);
    if let Ok(input) = cstring.to_str() {
      println!("{}", input);
    } else {
      panic!("Unable to print input");
    }
  }
}
```

Like before we write out our `#[no_mangle]` and `pub extern` so that the
function will be exported. We've also stated that our input is
a pointer to what we'll turn into a `CStr`. Because a `CStr` isn't like a Rust
`String` we need to turn it into one if we want it to be able to print. This is
why we use the `to_str()` function. However, this has the possibility of failure. If
the program making the `CStr` fails in making it properly then we won't
be able to turn it into a Rust String. This is why `to_str()` returns
a `Result` type. If it's translated fine we'll print out the input string
and if not we panic and cause the program to abort because it failed in
some manner. We also have to wrap it in an unsafe because we're messing
around with raw pointers. While this is classified as unsafe by Rust we
can be pretty sure that the Haskell code will generate a correct `CString`
for us to use so wrapping it in `unsafe` here is alright.

All right so we have our two functions we want to use in Haskell setup.
I want to talk about what we're going to be doing with the Haskell code
and some problems I ran into before actually writing this code.

### A small digression
While we could do this in one file we're going to use two of them. One
containing the foreign functions imported from Rust and our main file that will
call them from the other module. Why? Because there's no documentation
about using foreign functions imported in one module to be used in
another and it was the most frustrating things to have to deal with. To
digress for a bit, I found the documentation for Rust FFI was alright but
not the best. I could figure out what I needed but it's out of date since
it lacks any examples with dylib/cdylib and using that in other
languages. It also assumes I only want to write and use it in C code. I don't
want to use C. That's why I use Rust. I want that embedded in other languages
not the other way around.

Haskell's documentation however was messy, disorganized, scattered across
various web pages and sites, and could use a lot of love in getting fixed up.
There's no good tutorials on dynamic vs statically linking libraries and how to
do it. It just said cabal and GHC manuals covered it without linking the relevant
pages. That's honestly pretty frustrating, it felt like saying that's an exercise
left to the reader with no basic knowledge to get started. Linking to it would have
saved me many hours of digging through the documentation. There's no well written
explanations of doing FFI. The information on how to do it is there, it's just
not good or easy to parse without time and effort. This is one of the times where,
no the types really can't speak for themselves, because it's not types
I'm dealing with. It's frustrating because I really do like Haskell as a language
but the lack of good documentation really hurts the language.

Alright. Enough about bad documentation. Let's actually code what
I spent forever figuring it out so you don't have to be as frustrated as
I was, and so I can contribute rather than just being angry about it.

### Setting up our Haskell code
We're going to create two files in our src directory. Main.hs and FLib.hs. Here's what FLib.hs
should look like:

```haskell
module FLib where

import Foreign.C.Types
import Foreign.C.String

foreign import ccall "double_input" doubleInput :: CInt -> CInt
foreign import ccall unsafe "print_string" printString :: CString -> IO ()

```

Here's what Main.hs looks like:

```haskell
import FLib
import Foreign.C.String (newCString)

main :: IO ()
main = do
  putStrLn $ show $ doubleInput 3 --This will print 6
  -- Technically newCString doesn't need the \0 but I found that
  -- Rust was eating up the last character regardless of what it was
  -- so to make sure it works I just null terminated it to get the right
  -- output.
  str <- newCString "Hello World\0"
  printString str

```

Now we need to setup our cabal file so that this actually can work:

```cabal
name:                haskellrs
version:             0.1.0.0
build-type:          Simple
cabal-version:       >=1.10

executable haskellrs-exec
  main-is:             Main.hs
  hs-source-dirs:      src
  build-depends:       base >= 4.7 && < 5
  default-language:    Haskell2010
  other-modules:       FLib
  extra-libraries:     rusty, pthread
  extra-lib-dirs:      target/release

library
  hs-source-dirs:      src
  exposed-modules:     FLib
  other-extensions:    ForeignFunctionInterface
  build-depends:       base >= 4.7 && < 5
  default-language:    Haskell2010
  extra-libraries:     rusty, pthread
  extra-lib-dirs:      target/release
```

Pretty simple file right? Before we dive into the Haskell code let's get
this example working so you can see for yourself:

```bash
cargo build --release
cabal run
```

You should see the following printed out on your screen:

```bash
6
Hello World
```

You just ran Rust code inside of Haskell! Let's dissect our Haskell
code, so that we can understand how all of this works.

If we take a look at FLib we have our statements for importing code:

```haskell
foreign import ccall "double_input" doubleInput :: CInt -> CInt
foreign import ccall unsafe "print_string" printString :: CString -> IO ()
```

`foreign import ccall` tells us that the function we're importing
follows the C calling convention and Haskell should treat it like that
in it's code. Anything in quotes is the name of the actual function
we're importing. The stuff after the quotes is the function header and
what we'll call the name in Haskell. For example our Rust function `double_input`
is being imported into Haskell as `doubleInput` and it takes a `CInt` and
returns a `CInt`.

Take a look at the second line though. Notice how I put unsafe? We need
this because of how `CStrings` work for one in Haskell (I couldn't get
anything to print if I didn't put unsafe) but two we are doing an IO
action. Unlike `doubleInput` this function isn't pure and we have side
effects (printing out a line) and so we need this to be in the `IO` monad
for it to work. Even if your input might be a `CInt` for another function
you write, if you implement any kind of IO with that function import it
with an unsafe call so that it's enforced that way in Haskell. Unlike
Rust, Haskell needs to keep track of it's IO actions so that it knows
how to execute things each time.

General rule of thumb:

- Import pure functions as if they were safe
- Import impure functions as unsafe

Okay now let's look at our Main.hs file

```haskell
import FLib
import Foreign.C.String (newCString)

main :: IO ()
main = do
  putStrLn $ show $ doubleInput 3
  str <- newCString "Hello World\0"
  printString str

```

This is the code that actually executes our Rust Functions. First we
import our library that contains our Rust code and we import a function
to make `CStrings`.

Now we setup our main function. If you've not used Haskell before then
the syntax might look different but we'll go through each line so you
can understand what's going on:

```haskell
main :: IO ()
```

This is our function declaration. Every executable has a main function
with a return type of of `IO ()`. Essentially we're saying this program
has side effects (because we are running a program on a computer) and it
returns the unit type `()` upon completion wrapped up in the `IO` monad.
If that was complete gibberish then just know that `IO` is there to tell
the compiler we're doing stuff that affects the computer and it needs to
properly keep track of it.

```haskell
main = do
  putStrLn $ show $ doubleInput 3
```

We've started our `do` block which is saying that all the actions after
this must be in a monad and the last statement returns the value of the
expected monad, in this case we're in the `IO` monad and the last
statement must return `IO ()`. Our next line contains `$`. It's a way of
saying do everything to the right of me first. That line is equivalent
to writing:

```haskell
(putStrLn (show (doubleInput 3)))
```

I just find that the `$` is a bit cleaner to read. So what's happening
here is we're doubling the input 3. However we want to display this in
the terminal, so `show` converts the `CInt` into a `String` and
`putStrLn` actually prints it out to the console!

Next up we have this line:

```haskell
str <- newCString "Hello World\0"
```

What we're saying is store the value of `newCString` into `str`. Why are we
doing this though? Couldn't we just do what we did with the other line?
Well no, see `newCString` returns an `IO CString` and what we want is
the value inside the `IO` which is a type `CString`. Our function
`printString` expects a `CString` not an `IO CString`. By doing this
step we can actually then use `str` as a `CString`!

That's why this line works:

```haskell
  printString str
```

It then prints our string and returns an `IO ()` like the do block
expects and so our code compiles! I've been hiding some of the magic of
how this works in the cabal file that contains some options and things
to get this working. Let's take a look at it again:

```cabal
name:                haskellrs
version:             0.1.0.0
build-type:          Simple
cabal-version:       >=1.10

executable haskellrs-exec
  main-is:             Main.hs
  hs-source-dirs:      src
  build-depends:       base >= 4.7 && < 5
  default-language:    Haskell2010
  other-modules:       FLib
  extra-libraries:     rusty, pthread
  extra-lib-dirs:      target/release

library
  hs-source-dirs:      src
  exposed-modules:     FLib
  other-extensions:    ForeignFunctionInterface
  build-depends:       base >= 4.7 && < 5
  default-language:    Haskell2010
  extra-libraries:     rusty, pthread
  extra-lib-dirs:      target/release
```

It's fairly straight forward. We've created an executable that imports
our FLib library (this is why it's listed under other-modules) and it
imports our rust library (remember we called it rusty). We've also stated
where rusty was located with the extra-lib-dirs that way Haskell could
link it in properly. Remember this means you've compiled the Rust
Library first otherwise the target/release folder won't exist at all. Now
we've also imported this library pthread. It was necessary to get the `CString`
example working. Our library entry is also pretty much the same but take
a look under other-extensions. It uses `ForeignFunctionInterface`. If we
didn't put this here we would have to put `{-# LANGUAGE
ForeignFunctionInterface #-}` at the top of every Haskell file that did
an import call. By putting it here it implicitly does that for every
file in the library. Together these allow Haskell to call our Rust code
as expected. Seems like a lot but overall there aren't that many lines
of code needed to get it working!

### Conclusion
If you want to take a look at the code I've put it up in a repository
[here](https://github.com/mgattozzi/haskellrs). This post covered how to
setup a project to use Rust inside of Haskell code. While this was
fairly simple to setup and do in terms of the overall code it still took
some work. It also covered the proper ways to call the code inside of
Rust. I've started a [project](https://github.com/mgattozzi/curryrs) to automate the process in
Rust and Haskell projects so the need to write all the boiler plate
yourself is unnecessary. It'll also provide mappings between the various
types in Rust and Haskell to make it easier to reason about. This will
take some time to get right but making this a seamless experience for
users would be great. I think Rust and Haskell compliment
each other well and making it easy to do that will be a boon to both
communities.

Thanks to [Caleb Jones](https://github.com/porglezomp) for making some minor
corrections to this article.
