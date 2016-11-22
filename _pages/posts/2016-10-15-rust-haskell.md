---
layout: post
title: Using Haskell in Rust
---

After my article on putting [Rust in Haskell](http://mgattozzi.github.io/2016/10/01/haskell-rust.html) I
set out on getting Haskell into Rust as part of my test suite for
[curryrs](https://github.com/mgattozzi/curryrs) which is supposed to
make this much easier. I was having some trouble getting this to work
because Haskell FFI only supports exporting for C. I tried to get this
to work directly with Rust but it didn't work at all. The main issue
being that we need to initialize and end the Haskell runtime when we use
our Haskell functions and closing it when we're done. Linking libraries
also ended up being a problem to overcome to get it done right. It is
possible though! We're going to setup a program from scratch and we're
going to use `curryrs` to make the types easier between the two.

### Tooling
This articles assumes you have the following installed:

- rustc - at least 1.12 stable
- cargo
- stack
- GHC 8.0.1
- gcc
- make

### Setting up the project
First we need to get our Rust code all set up! Initialize a new binary
project like so:

```bash
cargo new rushs --bin
```

Inside the new `rushs` directory we need to create a new Haskell project.
We'll call it `hs2rs` and you can initialize it like so:

```bash
stack new hs2rs simple-library
```

Alright let's get all of our Haskell code done first before we put it
into our Rust code.

### Haskell
If you look inside `src/Lib.hs` in the `hs2rs` project directory you'll
see that it looks like this:

```haskell
module Lib
    ( someFunc
    ) where

someFunc :: IO ()
someFunc = putStrLn "someFunc"
```

We're going to change it so that it looks like this instead:

```haskell
module Lib where

import Types

triple :: I32 -> I32
triple x = 3 * x

foreign export ccall triple :: I32 -> I32
```

We're importing the types from `curryrs` which contain aliases
for all of the types making it easier to write code between both
languages. In this case we're using the `I32` type which is `i32` in Rust
and `Int32` in Haskell. We've also defined a function `triple` that takes
a value and multiplies it by 3. We've then exported it with the
C calling convention for use in other languages.

Next up we need to import the Haskell FFI headers as part of the library so
we can properly initialize and end the Haskell runtime in our C glue
code.

Open up a file in `src` called `wrapper.c` and put this in it:

```c
#include <HsFFI.h>
```

That's all we need here. Don't compile anything yet since none of this
is truly ready for FFI. We'll need to modify our cabal file next. Open
up `hs2rs.cabal`

It will look something like this:

```cabal
name:                hs2rs
version:             0.1.0.0
synopsis:            Initial project template from stack
description:         Please see README.md
homepage:            https://github.com/githubuser/hs2rs#readme
license:             BSD3
license-file:        LICENSE
author:              Author name here
maintainer:          example@example.com
copyright:           2016 Author name here
category:            Web
build-type:          Simple
-- extra-source-files:
cabal-version:       >=1.10

library
  hs-source-dirs:      src
  exposed-modules:     Lib
  build-depends:       base >= 4.7 && < 5
  default-language:    Haskell2010

source-repository head
  type:     git
  location: https://github.com/githubuser/hs2rs
```

We're going to add some options, import some dependencies, and make sure
our c wrapper code is included. Your cabal file will look something like
this when you modify it:

```cabal
name:                hs2rs
version:             0.1.0.0
synopsis:            Use Haskell in Rust!
description:         Please see README.md
homepage:            https://github.com/mgattozzi/rushs
license:             BSD3
license-file:        LICENSE
author:              Michael Gattozzi
maintainer:          mgattozzi@gmail.com
copyright:           2016 Michael Gattozzi
category:            FFI
build-type:          Simple
-- extra-source-files:
cabal-version:       >=1.10

library
  hs-source-dirs:      src
  exposed-modules:     Lib
  other-extensions:    ForeignFunctionInterface
  ghc-options:         -dynamic -fPIC -shared -lHSrts-ghc8.0.1 -o libhs.so
  build-depends:       base >= 4.7 && < 5
                     , curryrs
  c-sources:           src/wrapper.c
  default-language:    Haskell2010

source-repository head
  type:     git
  location: https://github.com/mgattozzi/rushs
```

The important things to note here are that we need to set
`other-extensions` to have the `ForeignFunctionInterface` used for GHC.
Also look at the flags used:

- `-dynamic` tells GHC that we want a dynamic library
- `-fPIC` is also necessary because of the need for Position Independent Code
- `-lHSrts-ghc8.0.1` is telling GHC to link in the rts library which we
  need for our code to work in other places. It's tied to the version of
  the compiler you use. Just change the last few numbers to the version
  you're using for this to work. However, at the time of writing
  I haven't tested this against other versions.
- `-o libhs.so` is just telling the compiler to stick it in the output file
  `libhs.so` in the main top of the library directory. This is nice for
  reproducible builds in our code rather than digging through
  `.stack-work` for the library.

Okay one last thing for the code to work. Open up stack.yaml and change
the line extra-deps to the following:

```yaml
extra-deps: [ "curryrs-0.1.1.0" ]
```

This is so we can import the `curryrs` library since it's only on Hackage
and not in the current stack LTS.

Now you can compile your code:

```bash
stack build
```

You should see a `libhs.so` file show up! We now need to write some C code
to act as our intermediary for Rust and Haskell. Open up a file in this
directory called `inter.c` and place the following in it:

```c
#include <HsFFI.h>

void init(void) {
  static char *argv[] = { "libhs.so", 0 }, **argv_ = argv;
  static int argc = 1;

  hs_init(&argc, &argv_);
}

void fin(void) {
  hs_exit();
}
```

Now run the following:

```bash
gcc -shared -o libinter.so inter.c libhs.so -fPIC
```

This creates a shared library `inter.so` that we can use that's linked with
`libhs.so` using Position Independent Code. We compile it using gcc as a C
compiler so that it can locate all the haskell runtime libraries. `inter.c` is
just used as a wrapper around `hs_init` and `hs_exit` to make it easier to start
and stop the Haskell runtime in Rust. Alright now let's write some Rust!

### Rust
First up open up Cargo.toml and change it to have a build file and to
add `curryrs` as a dependency:

```toml
[package]
name = "rushs"
version = "0.1.0"
authors = ["Michael Gattozzi <mgattozzi@gmail.com>"]
build = "build.rs"

[dependencies]
curryrs = "^0.1.0"
```

Next up open up build.rs in the top level of the library and put the
following in:

```rust
fn main() {
    println!("cargo:rustc-link-search=native=hs2rs");
    println!("cargo:rustc-link-lib=dylib=inter");
    println!("cargo:rustc-link-lib=dylib=hs");
}
```

This tells cargo that Rust needs to look inside `hs2rs` for our
`inter.so` and  `libhs.so` files! When we import them now we don't need
to specify which files need to be linked specifically in the file. Alright
we're finally ready to get our main file all setup. Open up `src/main.rs` and change
it to look like this:

```rust
extern crate curryrs;
use curryrs::types::I32;

extern {
    pub fn init();
    pub fn fin();
    pub fn triple(x: I32) -> I32;
}

fn main() {
    unsafe { init(); }
    println!("Tripled value: {}", unsafe{triple(50)});
    unsafe { fin(); }
}

```

We're first importing the `I32` type from `curryrs`. We then import our
functions we created earlier. This includes our initialization and
exiting functions for the Haskell run time. These are absolutely
necessary or the code won't work. Failure to close the runtime would be
undefined behavior and could hog up resources. Make sure to close it up
when you're done!

Now in our main function we initialize the runtime. We make our call to
triple to make the number 150 then we end the runtime. Alright let's run
the code!

```bash
cargo run
```

You should see "Tripled value: 150" printed out! Congrats you've now
successfully run Haskell inside of Rust!

### Make
This is great but what about someone working on building the project?
What about tests? If you try to run the code without having compiled
anything else the whole thing fails. Let's set up a quick easy Makefile
to avoid this problem:

```make
build: hs cargo

hs:
	@(cd hs2rs && stack build && gcc -shared -o libinter.so inter.c libhs.so -fPIC)

cargo:
	@cargo build

run: hs
	@cargo run

test: build
	@cargo test

doc:
	@cargo doc
	@(cd hs2rs && stack haddock)

clean:
	@cargo clean
	@(cd hs2rs && stack clean && rm *.so)
```

This is a pretty simple file and could easily be expanded to be more
robust. Now you won't need to worry about it not working and can just
get it running with `make run`! No more worrying about getting all the
flags done right now.

### Limitations of this example
This is only the most basic of examples. I'm still unsure of the best
practices of getting data structures passed between the two. If you have
examples of passing them between Rust and Haskell let me know! It would
be great if `curryrs` could support passing more complex data rather than
just basic primitives.

### Conclusion
I've walked you through the basics of using Haskell in Rust. This
includes the setup of your project, a little on how to use `curryrs`, how
to setup the C files you'll need to interface with Rust and how to
export functions for use in other languages. I've also shown you how to
setup your Rust project, what you would need to include in a build
script, as well as how to call Haskell properly from inside Rust. I
also covered a basic Makefile you can use to make it easier to build the
dependencies for your Rust code.

I'm hoping that examples like this will allow Rust users in the future
to leverage the power Haskell has, such as infinite lists, in their code
or vice versa and allowing Haskell to have a type safe fast language
when speed truly is critical.

If you're interested on bridging that gap as well let me know!

Thanks to [Caleb Jones](https://github.com/porglezomp) for making some
corrections to this article.
