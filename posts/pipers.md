# Announcing Pipers - A small library for piping commands
<div class="subtitle">Published December 23, 2016</div>

Back when I first started using Rust during 1.0 I had set about to learn
it by writing a shell program. While the code itself is really not
idiomatic (I cringed today fixing up what I took from it) it has one
thing that works well. Piping commands. I've received inquiries every
now and then from an [old Reddit
post](https://www.reddit.com/r/rust/comments/3azfie/how_to_pipe_one_process_into_another/)
of mine and while I did find the answer to the thread was locked so
I couldn't respond leading to this scenario for some programmers.

<img src="http://imgs.xkcd.com/comics/wisdom_of_the_ancients.png" class="center-block img-responsive">

Well finals are over, this still isn't in the Standard Library for some
reason and I wanted to try something out by creating a way for me to do
a point free style like library where you can chain commands together.

Pipe only works on Unix since it's using some Unix specific code from
the library. If at any point a command fails for some reason it's placed into the final result.
In fact if the command fails at any point along the way it just passes it down the chain
and doesn't execute the other commands. The code itself is only 100 lines long and that's including
comments and documentation. It's meant to be really simple in how it
works and I think this example sums it up quite well:

```rust
let out = Pipe::new("ls /")      // Put in your first command
              .then("grep usr")  // Choose the command you want to pipe into
              .then("head -c 1") // Keep chaining the pipes
              .finally()         // Turn the Pipe into a Result<Child>
              .expect("Commands did not pipe")
              .wait_with_output()
              .expect("Failed to wait on child");

assert_eq!("u", &String::from_utf8(out.stdout).unwrap());
```

It's a trivial example but it shows exactly what the library does.
Nothing more. Nothing less. The dependency is fairly light weight and
helps abstract all the boiler plate away from piping commands and I hope
you'll use it in your projects where you need this kind of
functionality.

If you want to check out the code itself it's available in the
[repo](https://github.com/mgattozzi/pipers) and it's available right now
on [crates.io](https://crates.io/crates/pipers) as well. To add it to
your code just add the following to your Cargo.toml file:

```toml
[dependencies]
pipers = "1.0.1"
```

Then just add the following:

```rust
extern crate pipers;
use pipers::Pipe;
```

Now you're all set and can start piping all of your shell commands!

