# Making WASM Executable on Linux
Published March th, 2018

It started with a tweet.

<a href="https://twitter.com/mgattozzi/status/974765243988574209">
  <img class="center-block img-responsive"
      src="/static/images/wasm-executable-tweet1.png"
      alt="What if I made a Linux kernel module to execute wasm directly? 🤔">
</a>

Which was quickly followed up by some discussion and a possible solution.

<a href="https://twitter.com/badboy_/status/974767682850574337">
  <img class="center-block img-responsive"
      src="/static/images/wasm-executable-tweet2.png"
      alt="I mean you wouldn’t even need a kernel module to make `./your-app.wasm`
      work: https://blog.cloudflare.com/using-go-as-a-scripting-language-in-linux/
      … (referring to your original tweet)">
</a>

Which led to this tweet:

<a href="https://twitter.com/mgattozzi/status/974810805043679233">
  <img class="center-block img-responsive"
      src="/static/images/wasm-executable-tweet3.png"
      alt="Look I'm not saying I just made wasm files executable on linux, but I just made wasm files executable on linux">
</a>

This post is going to look at how it was accomplished, discuss the necessary code, how you can do it
on your own system, and look at the current limitations of this solution that outweigh its benefits
currently, as well as what we can do to solve these and move WASM forward as an executable file
format.

## WASM Background

You may have heard the buzz around Web Assembly (WASM) and not been convinced, but to me it's a [big
deal](https://mgattozzi.com/rust-wasm). I've been working with the Rust WASM Working Group in order
to explore the space and see just what's possible with it. The main thing to understand about WASM
is that it's a portable binary format designed to let code run at native speeds on the web.
This byte code is interpreted by the browser and then executed. This is perfect for CPU-bound tasks
on the web and has led to some significant improvements in things like [source map
parsing](https://hacks.mozilla.org/2018/01/oxidizing-source-maps-with-rust-and-webassembly/).

Now I bet some of you are thinking, "If this is for the web, why are you trying to run it on your
computer?" Well think of it -- we finally have a cross platform open binary format that's not
controlled by any one corporation. It's an open standard! The Sun Microsystem people were ahead of
their time with the JVM and Java Web Applets. Now, though, it's a possibility that's unfettered by
corporations like Oracle. It's something anyone can implement an interpreter for and run!

I did this because I could, but really, the main reason is to get people excited about this
possibility. Compile once and run anywhere! Oh, this library is written in C and this one is written
in Rust? No problem. Compiling to the same format means it's easier to link and load things.
Multi-language projects? Sweet, less pain with FFI to deal with. No longer will we have to be constrained
by something like the C ABI! Instead we can just target WASM and define something better together!

Let's cover just how to make WASM executable then we can move on to its limitations and the future
where they can be removed.

## WASM Executables

Here's all the code needed to do this. Like most code, it's built on the shoulders of giants. I
used [Parity Tech's WASM interpreter](https://github.com/paritytech/wasmi) in order to interpret
the byte code.

```rust
extern crate wasmi;

use std::env::args;
use std::fs::File;
use wasmi::{ModuleInstance, NopExternals, RuntimeValue, ImportsBuilder, Module};

fn load_from_file(filename: &str) -> Module {
    use std::io::prelude::*;
    let mut file = File::open(filename).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    Module::from_buffer(buf).unwrap()
}

fn main() {
    let args: Vec<_> = args().collect();
    if args.len() != 2 {
        println!("Usage: {} <wasm file>", args[0]);
        return;
    }

    // Here we load module using dedicated for this purpose
    // `load_from_file` function (which works only with modules)
    let module = load_from_file(&args[1]);
    let main = ModuleInstance::new(&module, &ImportsBuilder::default())
        .expect("Failed to instantiate module")
        .run_start(&mut NopExternals)
        .expect("Failed to run start function in module");
    let res = main.invoke_export("main", &[RuntimeValue::I32(0), RuntimeValue::I32(0)], &mut NopExternals);
    match res {
        Ok(Some(RuntimeValue::I32(i))) => println!("Return value: {}", i),
        Ok(Some(RuntimeValue::I64(i))) => println!("Return value: {}", i),
        Ok(Some(RuntimeValue::F32(i))) => println!("Return value: {}", i),
        Ok(Some(RuntimeValue::F64(i))) => println!("Return value: {}", i),
        Err(e) => println!("Failed to execute wasm. Error was: {}", e),
        _ => println!("Uhhhh woops?"),
    }
}
```

This works by calling the `main` function from a Rust executable after instantiating
the module `start` function with `run_start`. While hidden in most Rust code, it's expecting a value
of `isize` for `argc` and a `*const *const u8` pointer for `argv`. In this case, I didn't make any
assumptions as to what they would be in the interpreter so we just pass in two values of 0 to
satisfy the program. It will execute the program and get a return value. Normally, it would just be
`0` for rust programs that execute correctly without error. You could run this program on a WASM
file and it would execute it just fine if it had a `main` function. In my testing case,
it did because I was using Rust's `wasm32-unknown-uknown` target. The end result is no fun. So how
did I get that value of ten in the tweet above? For that we'll need to look at the code used in
`wasm-add`:

```rust
#![feature(start)]

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let x = 5 + 5;
    x
}
```

Normally Rust's `main` function looks like this:

```rust
fn main() {
  // Your code here
}
```

But we're using the `start` feature. While normally implied in Rust programs, we can actually specify
what function we use to start a binary program! In this case we slap it on to `main` so we can make
it return a value. The compiled WASM will then add 5 and 5 then return the value, none the wiser as
to what it ran on. Neat, so we can return a number! Okay, cool, so we can run the interpreter on
a WASM file and return number values; what about execution? How do we make it go `./wasm-add.wasm`
and not `cargo run -- wasm-add.wasm`? The [Cloudflare blog
post](https://blog.cloudflare.com/using-go-as-a-scripting-language-in-linux/) describes this in more
detail, but it uses feature known as `binfmt_misc`. This allows Linux to learn how to run files other than
besides ELF files. Because WASM in its compiled form can't use the `#!` syntax to let the shell
know how to execute it, we need to use this method to teach Linux what to do when it encounters this
file type. The incantation for that is this:

```bash
$ sudo sh -c 'echo :webasm:E::wasm::/Path/to/the/interpreter:OC | tee /proc/sys/fs/binfmt_misc/register'
```

This instructs the kernel to register a thing called `webasm` that will execute files with the `E`xtension `wasm` using
the program specified here, using the permission/owner info on the given binary itself, not the
interpreter.

Alright, so with a binary and an interpreter we can now do this:

```bash
$ ./wasm-add.wasm
Return value: 10
```

Okay, so that's the fun and cool part. If you were thinking, "all I can return is numbers?" unfortunately
for now, yes. I'd like to discuss those limitations before you consider porting all your code to compile
to WASM to make it executable.

## WASM Limitations

If you, right now, create a program with `println!()` in Rust, compile it to WASM, then run it, you
will only have the return value printed out. The only reason anything could be printed is that the
interpreter could do something with a return value, not because it knew how to run print statements
inside the code. How could it? WASM has a [spec](https://webassembly.github.io/spec/) that lets it
work and interact with the web and to call things like `console.log`. However, the spec says nothing
about a computer!

How would you define that? Syscalls? Syscalls are different between Windows, Mac, and Linux. They
don't even necessarily work the same way. So we can't compile our WASM code with the assumption
we're on a specific host. We don't know whether the code itself is on the web or an interpreted
environment! If we don't know that and have no standard way to define input and output, then
essentially our programs will just run and not be interesting! In order for computers to be
interesting and useful, they can't be "pure" functions they need to interact with things.
Unfortunately, as it stands today the WASM spec doesn't have a way for us to define dealing with
input and output outside of web browsers and JS.

Here's the other limitation of this interpreter. If I compiled C code, it uses the same function
signature for `main` essentially so it'll be fine. What about if we got some other language to
compile and work on WASM? What if it called its entry function `execute`? That's not
the case right now, based off of what can compile to WASM, but you can see the problem here. We invoked
the executable saying it would have a function `main`; if it doesn't it won't work. If the program
doesn't have a `start` function here it won't work and the interpreter would fail. We could modify
the code to handle that but we want to make assumptions about binaries not if it's a library. You
can't execute libraries on computers so how can we make that distinction if it's compiled to WASM?

The other limitation is that we can only return one number! I've been trying to get it to work as
a casted pointer to a String or something this weekend, but to no avail. If you can do this, let me know!

To sum all of this up, though, this means one thing: we can run WASM on our computers but it's
essentially useless. How can we fix it?

## WASM Standards Expansion

The solution to this problem is that we need to expand the spec. If we as programmers want this as
a possibility, then we need to sit down and define just how we want WASM to signal to the interpreter,
"Hey I want to print stuff," or, "Hey I want to save a file," and so on and so on. There's a lot of
space here and to be honest I'm left with more questions than answers. How would we define it? How
would we make it so that we have something that can work across all platforms if implemented correctly?
I don't really have the answer here. It's a tough question that deserves well-thought-out responses
and a well-written specification based off of that. Unfortunately, as we all know or eventually come to
know, once the spec is out there, changing it becomes hard because you need to maintain
backwards compatibility. Just look at Windows' path handling for a good example of all the crazy
stuff done to maintain that compatibility while still letting new things work.

However, with all of that being said, I really think there's something here that's a big deal. I'm
not on the committee to change these things, but I think we should encourage those who are to
consider this going forward and keep an eye toward evolving the spec beyond the initial MVP offering. There's big
potential here to really make WASM something truly unifying in terms of programming and computers in
general. I'm really hoping more work will be done to explore this possibility in the future.

## Conclusion

There's a big unexplored space of what's possible. I'm hoping to work more with the [Rust WASM
Working Group](https://github.com/rust-lang-nursery/rust-wasm) on exploring this space to see what's
possible from our end to make things compatible with both execution on the web and on a computer, as
well as figuring out if we can get the interpreter to know what environment it's on and handle code
accordingly without breaking the spec. If you want to figure that out as well, please come by and
discuss! I've opened up this issue [here](https://github.com/rust-lang-nursery/rust-wasm/issues/91)
to discuss it more!
