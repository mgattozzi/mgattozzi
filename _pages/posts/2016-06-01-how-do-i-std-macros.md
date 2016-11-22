---
layout: post
title: How do I use the Standard Library Macros in Rust? Part 1
---

This is the second post in a series of blog posts dedicated to answering
the simple question "How do I X in Rust?" After last week's post
[/u/Breaking-Away](https://www.reddit.com/r/rust/comments/4l71qw/how_do_i_convert_a_str_to_string_the_beginning_in/d3mg0dk)
asked about how to use the macros available in the Standard Library.
Since there are quite a few I'll breaking up the posts into at least two
parts. This first post will cover the most common macros used in the
language. The second post will cover ones that are available, but not
used as often.

## Why would we want to use Standard Library Macros?
Do you like being able to print out text to the console? If you do so
using println!() then you're using a Standard Library macro! Maybe
you've seen code that uses try!() in Rust? If you have it's using
a Standard Library macro! These macros available to us cover common
functionality that people need, but where a function will not suffice.
A macro takes inputs much like a function does, however, when it's being
compiled it's expanded out to actual code based off of the rules it
matches and what is given as input. This makes macros pretty powerful
and flexible. Even writing a macro could be it's own series of posts.
Today though we're going to focus on how to use the Standard Library's
macros and how you can use them effectively. In a future post I'll cover
writing your own macro

The ones we'll cover in this post are:

- assert!()
- assert_eq!()
- panic!()
- print!()
- println!()
- try!()
- unimplemented!()
- unreachable!()
- vec!()
- write!()
- writeln!()

The macros we'll cover in part two are:

- cfg!()
- column!()
- concat!()
- debug_assert!()
- debug_assert_eq!()
- env!()
- file!()
- format!()
- format_args!()
- include!()
- include_bytes!()
- include_str!()
- line!()
- module_path!()
- option_env!()
- stringify!()
- thread_local!()

You'll see the first group of macros more often then not. It should be
noted that you don't need to import any of these macros as they're
implicitly included in each .rs file you create unless you set that the
file should not have the standard library available.

Alright let's jump in and learn us some macros!

### assert!()

```rust
fn main() {
  //These will pass and the code will execute the next line
  assert!(true)
  assert!(!false)
  //This will not pass and cause the code to abort
  assert!(false)
}
```

assert!() takes an input that tests for some value that gives true or
false. If true it'll continue code execution and if false will abort the
code and let you know why. This macro is great for testing as well as
for making guarantees (sometimes known as [contract
  programming](https://en.wikipedia.org/wiki/Design_by_contract)) in your code and causing it to abort if something
does not go right.

### assert_eq!()

```rust
fn main() {
  //These will pass and the code will execute the next line
  assert_eq!(1,1);
  assert_eq!("Hello","Hello");
  assert_eq!("there","there");
  assert_eq!(true,true);
  assert_eq!(false,false);
  //This will not pass and cause the code to abort
  assert_eq!(true,false);
}
```

assert_eq!() takes two parameters and compares them for equality. If
they are equal it moves on with the execution of code, if not it aborts
the code and lets you know what happened. Notice how you can pass any
two types there? The macro is able to take a variety of different inputs
unlike a function in Rust which has to`be given a type signature. This
allows us to test for all kinds of different types of equalities using
one macro! Can you imagine having to test for equality with a whole
bunch of functions designed for specific types? It would be a real pain
to deal with and that's why this macro is real nice for asserting things
are equal. It's greatly used in testing or guaranteeing two things are
equal in your code as it executes.

### panic!()

```rust
fn main() {
  let test = String::from("Hello");

  // This is a trivial example but the code here will
  // always panic.
  if test.is_empty() {
    println!("String is empty");
  } else {
    panic!("Uh oh the String was not empty. Aborting");
  }
}
```

panic!() is a macro used to deallocate all resources gracefully and exit
your program. Usually you'll use panic!() if you want code to abort
execution in your program in a defined safe way. This is what is
called if you try to unwrap a None value or an Err value for instance.
You can pass it a string much like println!() meaning you can do
something like this:

```rust
// Where we define e as an Error earlier in the code
panic!("This is my error: {}", e);
```

e will be passed into the {} as part of the string. You can have any
number of {} or {:?} (the debug variant of {}). Just make sure you
supply enough arguments after to fill in those spots in the string.
The string passed to the macro shows up as a message after the code
aborts running.

### print!()

```rust
use std::io::{stdout,Write};

fn main() {
  let num = 5;
  let num2 = 9;
  print!("This is output to stdout without a newline char");
  print!("\nI told it to print a newline twice \n");
  print!("The variable num has a value of: {}", num);
  //Debug output version
  print!("\nThe variable num has a value of: {:?}", num);
  //Outputs The variable num has a value of: 5 Also num2: 9
  print!("\nThe variable num has a value of: {} Also num2: {}", num, num2);
  //Makes sure print!() items get flushed to stdout for the user to see
  stdout().flush();
}
```

print!() is a macro that outputs text to stdout without adding a new
line char at the end of the string that it prints out. It can take any
number of inputs, one for each {} or {:?}. One thing that should be
noted is that print!() doesn't flush output to stdout automatically like
println!() does for whatever reason. Since it doesn't if you want the
output there immediately you can add a call to flush it like in the
above example.


### println!()

```rust
fn main() {
  let num = 5;
  let num2 = 9;
  println!("This is output to stdout with a newline char");
  println!("The variable num has a value of: {}", num);
  //Debug output version
  println!("The variable num has a value of: {:?}", num);
  //Outputs The variable num has a value of: 5 Also num2: 9
  println!("The variable num has a value of: {} Also num2: {}", num, num2);
}
```

println!() is a macro that outputs text to stdout and adds a new line
char at the end of the string that it prints out. It can take any
number of inputs, one for each {} or {:?}. Unlike print!(), println!()
automatically flushes to stdout on calling it. Generally you'll see this
more than print being used for reasons like this.

### try!()

```rust
use std::fs::File;

fn main() {
	let _unused_result = you_made_this();
	let _unused_result = why_did_you_make_this();
}

// Trys to make a file, returns an io error if it failed
// Ok with the () return type (essentially not a type, but
// it is and that's a discussion for a different day)
fn you_made_this() -> std::io::Result<()> {
    let my_file = try!(File::create("I_Made_This.txt"));
    Ok(())
}

// Same code but way more verbose!
fn why_did_you_make_this() -> std::io::Result<()> {
   let my_file = File::create("foo.txt");

    let _result = match my_file {
        Ok(t) => t,
        Err(e) => return Err(e),
    };

    Ok(())
}
```

I'll be honest, up until about two weeks ago I didn't know how this
worked, and I've been using Rust since 1.0 came out. That's why I'm
writing this, so you don't have to fumble around with it. try!() is
the match statement in the second function when expanded out. Because
of this your function needs a Result return type (in this case the IO
kind) matching what it's being used on to work. try!() is a nice way
to unwrap results where you want the error to quit the function and
allow you to handle the error explicitly. If you don't care about the
error being returned or still want to do things in the function even
if an error occurs don't use try!(). Otherwise feel free to use it
liberally so you don't have crazy nesting trying to deal with it
explicitly.

### unimplemented!()

```rust
fn main() {
    // What we care about with this made up function is that it returns an
    // Option<u32> no need to actually show an implementation
    let x = fib_seq(10);
    match x {
        Some(y) = println!("{}",y),
        None    = unimplemented!(),
    }
}
```

In the above example I've set the None branch to say it's unimplemented.
This macro lets the user continue to code and have things typecheck as 
if you had written things there and lets you or someone looking at the 
code base know that it's incomplete. However, this also means the code 
will panic if the macro code is reached in the flow of execution. In 
this case fib_seq() returned a None then this would case the code to fail. 
This macro is great to create stub functions or things you'll need in order
to get the compiler to shut up about it and know you still need to fix it.

### unreachable!()

```rust
fn main() {
    let x = squared(5);
    if x == 0 {
        println!("x was 0");
    } else if x > 0 {
        prinln!("x^2 is: {}", x);
    } else {
        unreachable!("If you got here then either math and logic is broken
        , or you should tell the compiler team");
    }
}

fn squared(n: i32) -> i32 {
    n * n
}
```

The above code will take a number and square it and print out either 0 or it's 
value. I added a trivial else statement, but if it's reached it tells us two 
things, either our squared function is broken or the compiler is having some 
problems. Either way, unreachable!() is a macro you use to let the compiler know 
that a certain branch of execution is impossible and it allows it to optimize it
away. However, if you use it incorrectly and it's reached the code will abort. Use
this if you can guarantee the code can't be reached. Otherwise you're better off
providing better error handling if you're uncertain if it can be reached or not.

### vec!()

```rust
fn main() {
    let x = vec![1,2,3,4,5];
    for i in x.iter() {
        println!("{}",i);
    }
    // Prints out
    // 1
    // 2
    // 3
    // 4
    // 5
    
    // Makes a vec of over 9000 twos.
    let y = vec![2; 9001]
}
```

vec![] is a macro that takes a slice (notice the [] not () used in the macro
call) and constructs a vector out of the the values you pass too it. Much nicer
than instantiating a Vector and pushing the individual data types into it one
call to push() at a time. If you know some of the values you need inside it 
ahead of time this is a real nice vector to use. It's also good to make a Vec
requiring repeated values a set number of times as show for the second part of
the example above.

### write!()

```rust
use std::io::Write;
use std::str;

fn main() {
	// This is what a byte string is actually, a slice of u8's
	// I've explicitly stated the type here for readability
	// even if the code itself didn't need it
	let mut buffer: Vec<u8> = Vec::new();
	write!(&mut buffer, "Words!").expect("Didn't write to buffer!");
	// You can also use formatted arguments like println!()
	
	// We had to use debug here since Vec<u8> doesn't have
	// the display trait implemented
	println!("{:?}", &buffer); // [87, 111, 114, 100, 115, 33]
	println!("{}", str::from_utf8(&buffer).unwrap()); // Words!
}
```

write!() is useful for creating bytestrings from input rather than
having to figure out the individual numbers and pushing them into a
Vec<u8> or having to do the conversion one at a time. It's like print!(),
but for a buffer that you can then use to do various other things with!

### writeln!()

```rust
use std::io::Write;
use std::str;

fn main() {
	// Same thing but now with new lines!
	let mut buffer: Vec<u8> = Vec::new();
	writeln!(&mut buffer, "Words!").expect("Didn't write to buffer!");
	// You can also use formatted arguments like println!()
	
	// Both print with the extra new line but only in the first one do you
	// see it manifest as a visible number (it's the 10)
	println!("{:?}", &buffer); // [87, 111, 114, 100, 115, 33, 10]
	println!("{}", str::from_utf8(&buffer).unwrap()); // Words!
}
```

writeln!() is useful for creating bytestrings from input rather than
having to figure out the individual numbers and pushing them into a
Vec<u8> or having to do the conversion one at a time and it automagically
adds a newline for you! It's like println!(), but for a buffer that you
can then use to do various other things with!

## Conclusion

Here's the quick rundown of all the macros we just covered:

- assert!() = Assert a boolean statement is true, abort if false
- assert_eq!() = Assert an equality is true, abort if false
- panic!() = Abort the code, release resources, and print out the string
  to the console that was passed to it.
- print!() = Print the string out to stdout without a newline character
- println!() = Print the string out to stdout with a newline character
- try!() = Attempt to unwrap a Result, returns an error if present.
  Requires the method to return a Result of some type if used.
- unimplemented!() = Useful for stub functions to let the code compile
  and pass the type checker. Will abort the code if used and it comes to
  that macro somehow.
- unreachable!() = Let's the compiler know that this branch of the code
  is impossible to reach to allow for optimizations. If it's reached
  somehow during execution the code will panic and abort.
- vec!() = Takes a slice of some type and turns it into a Vector of the
  type of the items in the slice.
- write!() = Write a line to a buffer without a newline character. This
  is usually used to write to a file of some sort.
- writeln!() = Write a line to a buffer with a newline character. This
  is usually a file of some sort.

These are the most common ones you'll see used in Rust code, but you
might see the other ones that I'll cover in the next post out in the
wild. This should give you a good grasp of what is available to you as
a user of the language and a taste of how powerful macros can be.

If you have a burning question that you want answered but just can't
seem to find the answer you need, shoot me an email at
mgattozzi@gmail.com and I'll write a post to answer your question! No
question is stupid. Chances are if you're having trouble with it so is
someone else so ask away!

If you're a more experienced Rust user and want to help with the wording
or give suggestions send me a PR or reach out to me and I'll include
changes if they're good. You'll also get credit for your
contributions of course!
