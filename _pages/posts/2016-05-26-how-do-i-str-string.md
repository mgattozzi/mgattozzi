---
layout: post
title: How do I convert a &str to a String in Rust?
---

This is the first in a whole series of blog posts to answer the simple
question "How do I X in Rust?" Many times as a user coming into the
Rust language there are growing pains and some things that seem
intuitive after using the language for a while aren't for a new user.
This is aimed at those new users, however even an experienced Rust user
might learn a new trick or two.

With that in mind let's jump right in!

## What's the difference between &str and String?

Before we talk about conversion it'll help to know what
a [&str](https://doc.rust-lang.org/std/primitive.str.html) and
a [String](https://doc.rust-lang.org/std/string/struct.String.html) are and why we'd want to convert from a &str to a String.
A &str is an immutable reference to a String. This means you can't change
the string at all or manipulate it in any way. However if you have
a String you have the option to do that as well as having some methods
that &str doesn't have to do text manipulation. You can look at the
methods available to the different data types that are linked above for
what you can do exactly with each type. Generally speaking:

- You want &str if you don't want to change the string
- You want String if you want to change the string or need ownership of
  the data

Sometimes you'll get a &str type from a method, but need a String. The
following methods we'll discuss will show you how to convert to a String
from an &str.

*Thanks to [@LeoUnglaub](https://twitter.com/LeoUnglaub/status/735942665569767424) for suggesting to add this section*

## Converting a &str to a String

Surprisingly, there are quite a few ways to do this and depending on what
you're trying to do one of the methods laid out below will be more
appropriate.

### to_string()

```rust
fn main() {
    let new_string = "Hello World!".to_string();
}
```

to_string() turns an &str into a String. Back before [this pr in version
1.9](https://github.com/rust-lang/rust/pull/32586) of the compiler this
was actually slower than the next method I'll be showing you. It went
through crazy amounts of machinery to turn it into a String whereas the
newer compiler version optimized this for &str -> String making it
equivalent to the next method speed wise. This machinery is all the code
that can parse the data type and turn it into a String. It should be
noted that this and the following methods are copying data under the
hood so that you can have ownership of the String. There is no speed
difference between to_string(), to_owned(), and String::from().

There was a question on the subreddit as to whether this means .to_owned() would be deprecated. [/u/Roaneno answers it well here.](https://www.reddit.com/r/rust/comments/4l71qw/how_do_i_convert_a_str_to_string_the_beginning_in/d3mc6jy?context=3)

### to_owned()

```rust
fn main() {
    let new_string = "Hello World!".to_owned();
}
```

Since the &str is an immutable reference to a String using to_owned()
turns it into a String you own and can do things with! This is now the
same as to_string(), but was the preferred way prior to 1.9 .to_owned()
is used for other data types to promote an immutable reference into a
data type you own.

### String::from()

```rust
fn main() {
    let new_string = String::from("Hello World!");
}
```

When you are using to_string() in any compiler after 1.9 you're actually
using this method. to_string() ends up being syntactic sugar for it.
You hand an &str into the from method and from there it constructs
a String type for you to use.

### String::push_str()

```rust
fn main() {
    let mut new_string = String::new(); //Create an empty string
    new_string.push_str("Hello");
    new_string.push_str(" World!");
    println!("{}", new_string); //Prints Hello World!
}
```

Since a String is a growable data type you can add to it by pushing &str
into it. While this is a bit verbose for the example I gave above you
could use it to concatenate a variety of &str into one String that you
can then manipulate to do other things.

### format!()

```rust
fn main() {
    let world_var= "world";
    //Becomes "Hello World!"
    let new_string = format!("Hello {}!", world_var);
}
```
*Thanks to [/u/horsefactory](https://www.reddit.com/r/rust/comments/4l71qw/how_do_i_convert_a_str_to_string_the_beginning_in/d3n258e) for pointing out the typo that was here in a previous version*

You can also use the format!() macro in order to do some more complex
formatting with &str as input. It's like println!() but instead of
outputting to stdout it instead returns a String. This is simpler to use
than .push_str() if you know what text you want in your string and don't
want lines of code just to set it up. Given the above example with
.push_str() it would be:

```rust
fn main() {
    let world_var = "world";
    let mut new_string = String::new();
    new_string.push_str("Hello ");
    new_string.push_str(world_var);
    new_string.push_str("!");
}
```

This is way more verbose than is needed. That's why a case like this is
more suited to format!() to have the code be more expressive and clean
looking. However, there is a cost. Remember that crazy machinery I was
talking about for .to_string()? That was using this under the hood and
as such was less efficient. Use this method with caution if speed is
a concern.

*Tip courtesy of [/u/MrJillHace](https://www.reddit.com/r/rust/comments/4l71qw/how_do_i_convert_a_str_to_string_the_beginning_in/d3kxqhk)*

### into()

```rust
fn main() {
    let new_string = "Hello world!".into();
}
```

This is a less well known way to do it, but since &str implements
Into it has this method available to it. Think of it as saying I'm
turning this into that. It's a trait for conversions which you can find
[here in the docs](https://doc.rust-lang.org/std/convert/trait.Into.html) if you want to read more about it. While it's a
little less clear as to what it's doing you have this method available
to you as well. It's not guaranteed to be fast according to the
definition of the trait, just that you are guaranteed the conversion
itself. String::from() implements the From trait which implements Into.
You're probably better off using that method rather than this, but
you're more than free to do so.

*Tip courtesy of
[@nj_snav_lin](https://twitter.com/nj_snav_lin/status/735923508467924997?s=09) and [/u/cramert](https://www.reddit.com/r/rust/comments/4l71qw/how_do_i_convert_a_str_to_string_the_beginning_in/d3lgmdi)*

## What's the idiomatic Rust way?

Most code you'll see will use .to_owned() or String::from() since that
was faster than .to_string(). Now though any of those 3 will be the most
common way to do conversions. This is one of those things that has so
many ways to do things that there's no one standard for the community.
I personally think .to_string() or String::from() are good choices since
they emphasize what they're doing more so than .to_owned() or any of the
other methods. Others might disagree, but at the end of the day it's
your code so choose the method that's best suited for what you're doing.

## We have to go back Marty!

Sometimes you'll need to go back from a String to &str. All you'll have
to do is:

```rust
fn main() {
    let my_string = String::from("Hello World!");
    let my_immutable_string = &my_string; //This is a &String type
    let my_str: &str = &my_string; //This is an &str type
}
```

&String can be used anywhere there is a need for &str. It automatically
turns into it if the method calls for it.

Simple enough right? If you want to know why you can read about [auto deref here](https://doc.rust-lang.org/book/deref-coercions.html).

*Thanks to [@k0pernicus](https://twitter.com/k0pernicus/status/736154776166109185) for suggesting to add this to the post*


## Conclusion

Above are the various ways you can make a String from a &str and each
serves a different purpose:

- .to_string() if you want a simple conversion and show you're making
  a String
- .to_owned() if you want a simple conversion and to show you're taking
  ownership of it.
- String::from() a more explicit way to say what you're doing
- String::push_str() if you need to append to a String
- format!() if you have predefined text that you wanted formatted in
  a specific way and don't care about the speed cost
- .into() if you want a simple conversion and don't care about the
  ambiguity of it

You can also easily go from a String to a &str:

- &String types turn into &str if the method call needs it
- let my_str: &str = &my_string; if you want to explicitly specify
  a &str type

If you have a burning question that you want answered but just can't
seem to find the answer you need, shoot me an email at
mgattozzi@gmail.com and I'll write a post to answer your question! No
question is stupid. Chances are if you're having trouble with it so is
someone else so ask away!

If you're a more experienced Rust user and want to help with the wording
or give suggestions send me a PR or reach out to me and I'll include
changes if they're good. You'll also get credit for your
contributions of course!
