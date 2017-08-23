# Global Uninitialized Statics in Rust
Published May 30th, 2017

Before we jump into things I want to put the following disclaimer:

> Uninitialized Global Statics are generally not what you want to have in Rust.
> In fact if this is the first thing you're reaching for you might want to
> rethink your design.

Why though? If it's uninitialized you then need to give it some value later
meaning the static variable needs to be mutable which is not thread safe if
accessed later after being created. If a static variable changes throughout the
lifetime of the program then Rust can't guarantee you won't have different
results.

If you need things to be static but immutable and the only way to initialize
them is through non constant functions I would highly recommend the
`lazy_static` crate. It allows you to create things like `HashMap`s and things
where you already know everything that will be in them and won't change. You can
see some examples on it's repo
[here](https://github.com/rust-lang-nursery/lazy-static.rs). Generally you could
use this and a value wrapped in a `Mutex` in order to [have mutability with
statics](https://github.com/rust-lang-nursery/lazy-static.rs/issues/39).
Sometimes though that doesn't work. I couldn't use this crate this time since
`ucontext_t` is not `Send` because it has raw pointers in it's fields. When we
don't use don't use a `Mutex` here that `Send` restriction is gone. Instead we
use `static mut` which as mentioned before is `unsafe`.

That being said there are times where having an uninitialized global static
might be useful, usually when dealing with C libraries. I came onto this issue
of creating uninitialized statics recently while using the context switching
functions from `libc`. I wanted to get the hang of them by just doing a one to
one port of C and later making a more Rust like interface for my library some
other time. The code I was porting came from
[this](http://pubs.opengroup.org/onlinepubs/009695399/functions/makecontext.html)
site and was an example of how the `makecontext` function worked. Here's the
code I ended coming up with to get it all working:

```rust
extern crate libc;

use libc::{c_char, swapcontext, makecontext, getcontext, ucontext_t, c_void};
use std::mem;

// All of the uninitialized context types we'll need later, where a context is
// a point in the code we can swap back to with all of the registers and values
// restored as if we had never jumped to some other point in the code.
static mut CTX_0: Option<ucontext_t> = None;
static mut CTX_1: Option<ucontext_t> = None;
static mut CTX_2: Option<ucontext_t> = None;

pub fn main() {
    println!("Start Main");
    unsafe {

        // We need empty arrays for the context types later. These act as a
        // stack to store values
        let mut st1: [c_char; 8192] = [mem::zeroed(); 8192];
        let mut st2: [c_char; 8192] = [mem::zeroed(); 8192];

        // This code could be done without this variable but I wanted
        // to make it close to the port. Think of it as a context that
        // doesn't point to anywhere, allowing us to come back to where
        // we left off later.
        CTX_0 = Some(mem::uninitialized());

        // We're creating an empty context here.
        let mut ctx_1_tmp: ucontext_t = mem::uninitialized();

        // We pass the raw pointer to the context to initialize it
        getcontext(&mut ctx_1_tmp as *mut ucontext_t);

        // We now assign it to the global static
        CTX_1 = Some(ctx_1_tmp);

        // We set up a stack for registers and keeping track of stuff
        // As well as the size of the stack.
        ctx_1().uc_stack.ss_sp = st1.as_mut_ptr() as *mut c_void;
        ctx_1().uc_stack.ss_size = mem::size_of_val(&st1);

        // Once we finish with this part of the code where do we go next?
        ctx_1().uc_link = ctx_0() as *mut ucontext_t;

        // We now point the context to a specific function. In this case the
        // function f1
        makecontext(ctx_1() as *mut ucontext_t, f1, 0);

        // This part is much like before except we have a different stack and
        // have it go to CTX_1 after completion rather than CTX_0
        let mut ctx_2_tmp: ucontext_t = mem::uninitialized();
        getcontext(&mut ctx_2_tmp as *mut ucontext_t);
        CTX_2 = Some(ctx_2_tmp);
        ctx_2().uc_stack.ss_sp = st2.as_mut_ptr() as *mut c_void;
        ctx_2().uc_stack.ss_size = mem::size_of_val(&st2);
        ctx_2().uc_link = ctx_1() as *mut ucontext_t;
        makecontext(ctx_2() as *mut ucontext_t, f2, 0);

        // Now we start by going from where we're at to whatever CTX_2 points
        // to in this case that's the function f2
        swapcontext(ctx_0() as *mut ucontext_t, ctx_2() as *const ucontext_t);
    }
    println!("Finished Main");
}

// The function given for CTX_1
extern "C" fn f1() {
    println!("Start f1");
    unsafe {
        swapcontext(ctx_1() as *mut ucontext_t, ctx_2() as *const ucontext_t)
    };
    println!("Finish f1");
}

// The function given for CTX_2
extern "C" fn f2() {
    println!("Start f2");
    unsafe {
        swapcontext(ctx_2() as *mut ucontext_t, ctx_1() as *const ucontext_t)
    };
    println!("Finish f2");
}

// Convenience function to access the variable inside CTX_0
unsafe fn ctx_0() -> &'static mut ucontext_t {
    match CTX_0 {
        Some(ref mut x) => &mut *x,
        None => panic!(),
    }
}

// Convenience function to access the variable inside CTX_1
unsafe fn ctx_1() -> &'static mut ucontext_t {
    match CTX_1 {
        Some(ref mut x) => &mut *x,
        None => panic!(),
    }
}

// Convenience function to access the variable inside CTX_2
unsafe fn ctx_2() -> &'static mut ucontext_t {
    match CTX_2 {
        Some(ref mut x) => &mut *x,
        None => panic!(),
    }
}
```

If you run the above code you get the following output:

```
Start Main
Start f2
Start f1
Finish f2
Finish f1
Finish Main
```

This might be a bit confusing since the control flow goes all over the place but
it has its place for things like coroutines. However, this code is an example
for uninitialized global statics in Rust something that I didn't find documented
specifically anywhere and had to be pulled from disparate information sources.
If you don't care or understand about the context switching that's okay!
Let's take a look at the important bits above so you can understand how to do
what I described:

```rust
static mut CTX_0: Option<ucontext_t> = None;
static mut CTX_1: Option<ucontext_t> = None;
static mut CTX_2: Option<ucontext_t> = None;
```

Here we're declaring our variables and assigning them a value of `None`. This
satisfies the constraints for statics in Rust. They need an initial value at
compile time. We use `Option` in Rust to represent the possibility of a value
existing. Here we assign it a value of `None` to let the compiler know that
there is no value right now! This will change later but the compiler trusts
we'll know what we're doing.

How do we access the inner value mutably though? As you might have noticed we
have three different functions to access each variable:

```rust
unsafe fn ctx_0() -> &'static mut ucontext_t {
    match CTX_0 {
        Some(ref mut x) => &mut *x,
        None => panic!(),
    }
}
unsafe fn ctx_1() -> &'static mut ucontext_t {
    match CTX_1 {
        Some(ref mut x) => &mut *x,
        None => panic!(),
    }
}
unsafe fn ctx_2() -> &'static mut ucontext_t {
    match CTX_2 {
        Some(ref mut x) => &mut *x,
        None => panic!(),
    }
}
```

We're extracting those values and making a mutable reference to them and
claiming that their lifetime is static. This can't be elided right now so we
need to add the `'static` lifetime annotation to the return type. We've also
marked the function as `unsafe` since we're accessing a global static mutably,
which as I've mentioned before is an incredibly bad idea unless you absolutely
need to do this.

> N.B. You could do something else other than `panic!()` here and probably
> should, but this is for an example where I knew how everything would work out.
> That might not be the case though in other code.

Now if we ever need to assign a value to it we access the variable and assign it
a value of `Some(T)`. In this case we do it in the code in three separate
places:

```rust
// This function is also unsafe. Be careful!
CTX_0 = Some(mem::uninitialized());

let mut ctx_1_tmp: ucontext_t = mem::uninitialized();
getcontext(&mut ctx_1_tmp as *mut ucontext_t);
CTX_1 = Some(ctx_1_tmp);

// Code omitted

let mut ctx_2_tmp: ucontext_t = mem::uninitialized();
getcontext(&mut ctx_2_tmp as *mut ucontext_t);
CTX_2 = Some(ctx_2_tmp);

// Code omitted
```

We first make an uninitialized `ucontext_t` then use the `libc` function
`getcontext` to give it a value then assign them to the variables. After that if
we ever want access to the inner content we just call the helper functions we
created earlier.

## Conclusion
I can't stress how much you really shouldn't be doing this unless you can do all
of the book keeping and making sure everything works correctly. This is a useful
technique to have when dealing with languages like C where this behavior is
normal. This is more something you'd do with FFI than anything else. In Rust you
shouldn't really be using uninitialized statics. You can generally do what you
want through some other way in Rust.

That being said I hope you learned a little bit of what's possible with `unsafe`
Rust and can use this as a reference if you ever do need a little of this black
magic. Big thanks to [Manishearth](https://github.com/Manishearth) for
proofreading and offering suggestions on a few things here.
