---
layout: post
title: Understanding where clauses and trait constraints
---

Lately I've been working on a crate [functils](https://github.com/mgattozzi/functils) to make Rust a little more
functional and it's based off Stephen Diehl's
[Protolude](https://github.com/sdiehl/protolude) crate. It has some very
sensible defaults for Haskell after turning off the implicit Prelude
that I enjoy using in my Haskell code. It includes some cool functions
like the identity function:

```haskell
-- Given a type a return itself
id :: a -> a
id x = x
```

I implemented that in Rust no problem with generics!

``` rust
pub fn identity<A>(x:A) -> A {
    x
}
```

Well how about this function uncons from protolude? If you know Scheme
or LISP it's the opposite of cons, as I just found out in class today.
Well it's pretty easy in Haskell!

```haskell
-- Given a list of a return a tuple of the first item of the list and
-- the rest of the list if it exists. Otherwise return Nothing.
uncons :: [a] -> Maybe (a, [a])
-- Pattern matching on an empty list means we return nothing
uncons []     = Nothing
-- x is the head and xs is the rest of the list
uncons (x:xs) = Just (x, xs)
```

For those unfamiliar with Haskell, Maybe is equivalent to Option in
Rust. Just == Some and Nothing == None. If this is anything like
identity then it should be a breeze to implement in Rust right? Nope.
I was wrong.

## Iterating from 0

Here was my first wack at the code, a direct port of Haskell's code to
Rust:

``` rust
fn uncons<A>(x: [A]) -> Option<(A,[A])> {
    match x.length {
        0 => None,
        1 => Some((x[0], [])),
        _ => Some((x[0], x[1..(x.length - 1)])),
    }
}
```

If you've done generics with slices before or with slices with non
deterministic sizes you know what's going to happen. If you don't here's
the error message:

```
error: aborting due to 2 previous errors
error: the trait bound `[A]: std::marker::Sized` is not satisfied [--explain E0277]
 --> example.rs:5:1
  |>
5 |> fn uncons<A>(x: [A]) -> Option<(A,[A])> {
  |> ^
note: `[A]` does not have a constant size known at compile-time
note: required because it appears within the type `(A, [A])`
note: required by `std::option::Option`
```

What does this mean? Well slices need to know the size of their elements
and using generics won't let it know how much space to allocate. We could add
a `Sized` constraint but what if I want to use this on list like things like
`Vec` or `HashSet`? This implementation won't do anyways for them. Well what's
one thing that they have in common? Iterators! Let's use them to implement this
function. Here's the first pass I made at the code:

``` rust
// This isn't implicitly imported so we need to do so ourselves
use std::iter::FromIterator;

// We need B because it won't be something to iterate over but
// an item inside of A
pub fn uncons<A: IntoIterator + FromIterator<A>, B>(x: A) -> Option<(B,A)> {
    // Let's turn our input into an iterator to use!
    let mut iter = x.into_iter();
    // If it has a value let's get it and store it inside of one
    if let Some(one) = iter.next() {
        // We have a value. Wrap the tuple inside Some and use the
        // collect() function to turn it back into the collection
        // without the first value
        Some((one, iter.collect()))
    } else {
        // There's nothing in the iterator! We have an 'empty list'
        // therefore we need to return None.
        None
    }
}
```

This should work. This is perfect code. Nothing should go wrong. Oh boy
did it go wrong.

```
error: aborting due to previous error
error: mismatched types [--explain E0308]
  --> example.rs:16:15
   |>
16 |>         Some((one, iter.collect()))
   |>               ^^^ expected type parameter, found associated type
note: expected type `B`
note:    found type `<A as std::iter::IntoIterator>::Item`

error: the trait bound `A: std::iter::FromIterator<<A as std::iter::IntoIterator>::Item>` is not satisfied [--explain E0277]
  --> example.rs:16:25
   |>
16 |>         Some((one, iter.collect()))
   |>                         ^^^^^^^
help: consider adding a `where A: std::iter::FromIterator<<A as std::iter::IntoIterator>::Item>` bound
```

Uhhhhhhhhhhh what? The trait bound whatsit isn't satisfied? Add a `where` clause?
This is where I realized I had gone into new territory with Rust that I had not
understood yet. I was lost so I did what the compiler said to do. Let's add this `where`
to the example then.

``` rust
use std::iter::FromIterator;
pub fn uncons<A: IntoIterator + FromIterator<A>, B>(x: A) -> Option<(B,A)>
  where A: FromIterator<<A as IntoIterator>::Item> {
    let mut iter = x.into_iter();
    if let Some(one) = iter.next() {
        Some((one, iter.collect()))
    } else {
        None
    }
}
```

Okay so this didn't solve our problem. We get this error again instead:

```
error: mismatched types [--explain E0308]
  --> example.rs:11:15
   |>
11 |>         Some((one, iter.collect()))
   |>               ^^^ expected type parameter, found associated type
note: expected type `B`
note:    found type `<A as std::iter::IntoIterator>::Item`

error: aborting due to previous error
```

Let's back it up a bit. What exactly are we trying to tell the compiler?
A is a collection of some sort that can turn into and out of an iterator
and B is the item stored inside of A! If we look at the found type the
compiler is telling us it has this `Item` type but it doesn't know what to
do with it. We need to tell the compiler that this `Item` type is actually
B! How though? How? With a `where` clause actually!

## From confusion to understanding. Implementing the where clause.

Let's look at the `where` clause. `where` is used to add constraints to
generic types (like we had been doing) and giving the compiler the
information it needs to solve things! Here's what our final version
should look like and what I came to after a lot of work:

```rust
use std::iter::FromIterator;
pub fn uncons<A, B>(x: A) -> Option<(B,A)>
    where A: IntoIterator<Item = B> + FromIterator<B> {
    let mut iter = x.into_iter();
    if let Some(one) = iter.next() {
        Some((one, iter.collect()))
    } else {
        None
    }
}
```

No errors at all! Alright, lets take a look at the function definition
to see what's going on.

```rust
pub fn uncons<A,B>(x: A) -> Option<(B,A)>
```

This is pretty standard and looks more like what we've done before.
`uncons` takes an A type called x and turns it into an `Option<(B,A)>`. Awesome,
so let's look at this where clause because we haven't seen it look like
this before.

``` rust
where A: IntoIterator<Item = B> + FromIterator<B>
```

A is constrained with the traits `IntoIterator` and `FromIterator`. This is
nothing new but the stuff in the angle brackets is. Look at
`IntoIterator`. We are saying that the Items inside of it are of the type
B! This means if I did this with `Vec<String>` then A is `Vec` and B is
`String`. The compiler now understands that if A is being
iterated over, then the type inside of it is B for this function! In the
`FromIterator` we're doing a similar thing but saying that the inner
type implementing it is of type B. In this case it's basically saying
`A<B>: FromIterator` if that form makes more sense to you (but is
incorrect Rust). This then compiles and if you test it out works exactly as
you would expect!

If you don't believe me you can try this example to see it does:

```rust
let empty: Vec<i32> = vec![];
let vec1 = vec![1,2,3];
let vec2 = vec![2,3];
assert!(uncons(vec1).unwrap().eq(&(1,vec2)));
assert_eq!(uncons(empty), None)
```

## So what does where do?
`where ` allows us to define constraints for the types given as generics
and provide more information to the compiler. It allows us to make the
signature of the function look a little neater as well. Rather than
this:

```rust
pub fn example<A: Ord + Eq + PartialEq + PartialOrd, B: Iterator>(x: A, y: B) -> bool {
  //implementation
}
```

We can put:

```rust
pub fn example<A,B>(x: A, y: B) -> bool
  where A: Ord + Eq + PartialEq + PartialOrd
      , B: Iterator
{
  //implementation
}
```

This is a little more compact in my opinion and easier to read like
a list. My final form of the uncons function actually does work with the
inline notation. It just was really long and not as easy to read.

To answer the question: `where` is syntactic sugar for the inline form
of adding constraints to generics that is easier to read.

Wait what you say?! You said you solved it with a `where` clause but
it's really just sugar, not an actual solution. Well I did,
sort of. While it turns out to be unnecessary since `where` is
syntactic sugar it did lead me in the right direction of thinking about
how to solve the problem, mainly what am I trying to tell the compiler?
How do I tell the compiler B is inside of A?

## Conclusion
I've walked you through the problems I had while working with traits and
implementing something new and solving it using `where` and better trait
constraints. I struggled for quite some time before figuring this out because
the documentation was sparse on constraints and the error messages were helpful but
not in the way I thought they'd be and it took a while to reason through them.

I'd love to hear your feedback and corrections if you have any!
