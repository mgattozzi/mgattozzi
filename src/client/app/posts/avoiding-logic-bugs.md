# Avoiding Logic Bugs in Rust with Traits and Types
Published July 14th, 2017

The other day I saw someone comment how a bug could have been avoided if Rust
was used. This was incorrect as it was a logic bug not a memory bug. Rust
guarantees memory safety, but whether your logic is correct is a whole different
story. That being said we can use Rust's type system to make our code work and
avoid logic bugs.

Rust has some nice abstractions to avoid these kinds of errors if used properly.
Is it perfect? No, if they're implemented incorrectly then you're still going to
have logic bugs. If your implementation is correct though it'll make it harder
to shoot oneself in the foot later on.

To show you what I mean we'll start making a library for dealing with units, in
this case we'll just do temperature, how to handle conversions, implementing
traits to make using the library easier, and we'll write some tests to make sure
it works as expected. Before that though let's take a look at a fragile
implementation of this library that could easily be used improperly:

```rust
type Kelvin = f64;
type Celsius = f64;
type Fahrenheit = f64;

pub fn fahrenheit_to_celsius(f: Fahrenheit) -> Celsius {
    ((f-32.0) * (5.0/9.0) )
}

pub fn fahrenheit_to_kelvin(f: Fahrenheit) -> Kelvin {
    (f + 459.67) * (5.0/9.0)
}

pub fn kelvin_to_celsius(k: Kelvin) -> Celsius {
    (k - 273.15)
}

pub fn kelvin_to_fahrenheit(k: Kelvin) -> Fahrenheit {
    ((k * (9.0/5.0)) - 459.67 )
}

pub fn celsius_to_fahrenheit(c: Celsius) -> Fahrenheit {
    ((c * (9.0/5.0)) + 32.0 )
}

pub fn celsius_to_kelvin(c: Celsius) -> Kelvin {
    (c + 273.15)
}
```

Pretty simple right? We have three different types and it has all the
conversions. All you need to do is put the right type in and you're good to go.
Here's the problem though:

```rust
let x: Fahrenheit = 32.0;
println!("{}", celsius_to_fahrenheit(x)); // This works D:
```

`type` defines a type alias. This means `Celsius` and `Fahrenheit` are both
actually `f64`. `rustc` will gladly build this code and run it, despite us
having said that this is a `Fahrenheit` type. Our code has a logic bug! Our
current implementation is very weak against logic issues and we can't add things
together without a lot of work. We'd have to keep track of the types, how they
were converted from what unit to what, and then use the right method for the
conversion. That's a lot of work, a lot of methods, and they're too error prone.
Let's make our implementation more robust and easier to use:

```rust
use std::ops::Add;
use std::fmt;
use Temperature::*;

#[derive(Debug, PartialEq, Copy, Clone)]
/// An enum representing the different units of Temperature
pub enum Temperature {
    Kelvin(f64),
    Celsius(f64),
    Fahrenheit(f64),
}
```

First we've imported the `Add` trait which we'll get to later, but it will let
us add units together using `+` when we implement it! We've also imported `fmt`
since we'll be handrolling an implementation of `Display` for the `Temperature`
enum we've created. We'll be able to allow people to print out units with the
correct number and unit tacked on at the end! The meat of this bit is this
`Temperature`  type. We imported all of it's variants into the file (the
`use Temperature::*;`)so we don't have to keep saying `Temperature::Kelvin` or
`Temperature::Celsius`.As you can see we have three different unit types, all
with an `f64` value as an internal field. We've also derived `Debug` for debug
printing, `PartialEq` to compare values, `Clone` and `Copy`. Because `f64` is a
`Copy` the compiler will let the enum act as a `Copy` type which is nice for
math and dealing with ownership in Rust. You might be wondering why we couldn't
get `Eq` to auto derive here. `f64` does not have it implemented for it at all
since it is a floating point type. Floating points are tricky to test for
equality so you can only really get a "close enough" kind of answer.

Alright we have our type and imports lets start implementing some traits. Let's
start off with the `Display` trait so that we can print out the type with the
right unit attached to it:

```rust
impl fmt::Display for Temperature {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match *self {
             Kelvin(k) => write!(fmtr, "{}K", k),
             Celsius(c) => write!(fmtr, "{}°C", c),
             Fahrenheit(f) => write!(fmtr, "{}°F", f),
        }
    }
}
```

Our `Display` implementation is fairly simple. First we match on `*self` (it
just means we don't have to put an `&` in front of each field as we're
dereferencing it), and write out the inner value with the correct unit tacked on
at the end! We couldn't do this in our old implementation because you can't
implement it for types outside your own library. Neat we got some small benefits
just by switching to an `enum` like this.

Alright let's actually implement a few functions for the `Temperature` enum
itself:

```rust
impl Temperature {
    /// Convert whatever `Temperature` unit there is into `Celsius`
    pub fn to_celsius(self) -> Temperature {
        match self {
            Kelvin(k) => Celsius(k - 273.15),
            c @ Celsius(_) => c,
            Fahrenheit(f) => Celsius( (f-32.0) * (5.0/9.0) ),
        }
    }

    /// Convert whatever `Temperature` unit there is into `Fahrenheit`
    pub fn to_fahrenheit(self) -> Temperature {
        match self {
            Kelvin(k) =>  Fahrenheit( (k * (9.0/5.0)) - 459.67 ),
            Celsius(c) => Fahrenheit( (c * (9.0/5.0)) + 32.0 ),
            f @ Fahrenheit(_) => f,
        }
    }

    /// Convert whatever `Temperature` unit there is into `Kelvin`
    pub fn to_kelvin(self) -> Temperature {
        match self {
            k @ Kelvin(_) => k,
            Celsius(c) => Kelvin(c + 273.15),
            Fahrenheit(f) => Kelvin( (f + 459.67) * (5.0/9.0) ),
        }
    }
}
```

Remember how we had six different methods for temperature conversion and it was
prone to converting units from the wrong type into the wrong number for the
return type? No more! We can now convert all of the temperatures into the right
type and if we try to convert `Kelvin` to `Kelvin` then there's no problem!
It'll just return the type as is. If you've never seen the `@` symbol used
before it just means that the value of the whole pattern to the right of it is
assigned to the identifier to the left. For instance if I called the code:

```rust
let x = Kelvin(100.0).to_kelvin();
```

Then in the method `to_kelvin` `k` becomes the value `Kelvin(100.0)`.
It is not always needed but in this case it helped make our code a bit cleaner.
Awesome! We now have no fear temperature conversion. What if we wanted to add
32°F to 100K though? As it stands we could do the conversion, then take the
values out then add them together but that's a bit of a pain for an end user.
Why not make it easy for them and make sure that when adding units together it
turns out correctly?

```rust
impl Add for Temperature {

    type Output = Temperature;

    /// Add the Temperature units together with automatic conversion.
    /// The RHS will be converted into the unit on the left.
    fn add(self, rhs: Temperature) -> Self::Output {
        match (self, rhs) {
            (Celsius(a), b @ _) => {
                match b.to_celsius() {
                    Celsius(b) => Celsius(a + b),
                    _ => unreachable!(),
                }
            },
            (Fahrenheit(a), b @ _) => {
                match b.to_fahrenheit() {
                    Fahrenheit(b) => Fahrenheit(a + b),
                    _ => unreachable!(),
                }
            },
            (Kelvin(a), b @ _) => {
                match b.to_kelvin() {
                    Kelvin(b) => Kelvin(a + b),
                    _ => unreachable!(),
                }
            },
        }
    }
}
```

There's a lot to digest here so let's start with that `type` that shows up at
the beginning. The `type Output = Temperature` is known as an Associated Type in
Rust. You can read more about it in the book
[here](https://doc.rust-lang.org/book/first-edition/associated-types.html). In
this case we refer to it as the return type for `add` using `Self::Output`.

After that is our function `add` which the compiler uses. When we do something
like `1 + 2` in Rust this is syntactic sugar for `add(1,2)`. That's why we need
to define the `add` function for the `Temperature` `enum`. It is what allows us
to use that syntactic sugar! `self` refers to our left hand side value and `rhs`
is the right hand side value of the operation. We've wrapped it in a tuple
`(self, rhs)` and we're pattern matching against it. This makes makes sure that
we have have to match against every possible permutation of units on the left
and right side! Weird but there's only 3 statements in the match statement
right?

Here's the cool thing, because of how we implemented our conversion function we
can just call the function that converts the unit on the right to the unit on
the left and assume the value we get back is the right unit type. We then take
the converted (or not!) values inner number and add it to the value from the
unit on the left and return the proper unit. Since we know what value `b` will
be for the inner `match` statements we can just say any other value of
`Temperature` is unreachable. If it does get reached for whatever reason we
either have an implementation bug (more likely) or a compiler bug (less likely).

Now we can do things like:

```rust
let x = Kelvin(100.0) + Fahrenheit(32.0);
```

and Rust will handle not only the conversions but make sure the proper functions
are used to do it. No more problems for the end user! Let's write some tests
though. We need to make sure it works:

```rust
#[test]
fn add_test() {
    let k1 = Kelvin(0.0);
    let k2 = Kelvin(100.0);

    let c1 = Celsius(0.0);
    let c2 = Celsius(100.0);

    let f1 = Fahrenheit(0.0);
    let f2 = Fahrenheit(100.0);

    // Added to itself it should be the same unit
    assert_eq!(Kelvin(100.0), k1 + k2);
    assert_eq!(Celsius(100.0), c1 + c2);
    assert_eq!(Fahrenheit(100.0), f1 + f2);

    // Added to another unit it should be the conversion of the right
    // into the unit on the left added together. Remember we are using
    // floating point so there will be some margin of error for fractions
    // that occur
    assert_eq!(Kelvin(273.15), k1 + c1);
    assert_eq!(Kelvin(255.3722222222222223), k1 + f1);
    assert_eq!(Celsius(-273.15), c1 + k1);
    assert_eq!(Celsius(-17.77777777777778), c1 + f1);
    assert_eq!(Fahrenheit(32.0), f1 + c1);
    assert_eq!(Fahrenheit(-459.67), f1 + k1);

    // Testing multiple unit types added together
    assert_eq!(Fahrenheit(-427.67), f1 + k1 + c1);
    assert_eq!(Celsius(-290.92777777777775), c1 + k1 + f1);
    assert_eq!(Kelvin(528.5222222222222), k1 + f1 + c1);
}

#[test]
fn format_test() {
    assert_eq!(format!("{}", Kelvin(528.0)), "528K".to_owned());
    assert_eq!(format!("{}", Celsius(100.0)), "100°C".to_owned());
    assert_eq!(format!("{}", Fahrenheit(32.0)), "32°F".to_owned());
}

```

Now we can run the tests and you'll see that it all works! Remember when we
derived `Copy` earlier? We needed it for something like this. Now we only need
to define the unit once and every time we use it the value is copied over.
Without the copy we would have had to define each variable above multiple times.
Not really ergonomic or fun in this case.

It works and that's pretty cool right? We could easily extend this code to work
with things like multiplying, the `+=` operator, or other things that we might
want to do. Here's all of the code put together and not split up:

```rust
use std::ops::Add;
use std::fmt;
use Temperature::*;

#[derive(Debug, PartialEq, Copy, Clone)]
/// An enum representing the different units of Temperature
pub enum Temperature {
    Kelvin(f64),
    Celsius(f64),
    Fahrenheit(f64),
}

impl fmt::Display for Temperature {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match *self {
             Kelvin(k) => write!(fmtr, "{}K", k),
             Celsius(c) => write!(fmtr, "{}°C", c),
             Fahrenheit(f) => write!(fmtr, "{}°F", f),
        }
    }
}

impl Temperature {
    /// Convert whatever Temperature unit there is into Celsius
    pub fn to_celsius(self) -> Temperature {
        match self {
            Kelvin(k) => Celsius(k - 273.15),
            c @ Celsius(_) => c,
            Fahrenheit(f) => Celsius( (f-32.0) * (5.0/9.0) ),
        }
    }

    /// Convert whatever Temperature unit there is into Fahrenheit
    pub fn to_fahrenheit(self) -> Temperature {
        match self {
            Kelvin(k) =>  Fahrenheit( (k * (9.0/5.0)) - 459.67 ),
            Celsius(c) => Fahrenheit( (c * (9.0/5.0)) + 32.0 ),
            f @ Fahrenheit(_) => f,
        }
    }

    /// Convert whatever Temperature unit there is into Kelvin
    pub fn to_kelvin(self) -> Temperature {
        match self {
            k @ Kelvin(_) => k,
            Celsius(c) => Kelvin(c + 273.15),
            Fahrenheit(f) => Kelvin( (f + 459.67) * (5.0/9.0) ),
        }
    }
}

impl Add for Temperature {

    type Output = Temperature;

    /// Add the Temperature units together with automatic conversion.
    /// The RHS will be converted into the unit on the left.
    fn add(self, rhs: Temperature) -> Self::Output {
        match (self, rhs) {
            (Celsius(a), b @ _) => {
                match b.to_celsius() {
                    Celsius(b) => Celsius(a + b),
                    _ => unreachable!(),
                }
            },
            (Fahrenheit(a), b @ _) => {
                match b.to_fahrenheit() {
                    Fahrenheit(b) => Fahrenheit(a + b),
                    _ => unreachable!(),
                }
            },
            (Kelvin(a), b @ _) => {
                match b.to_kelvin() {
                    Kelvin(b) => Kelvin(a + b),
                    _ => unreachable!(),
                }
            },
        }
    }
}

#[test]
fn add_test() {
    let k1 = Kelvin(0.0);
    let k2 = Kelvin(100.0);

    let c1 = Celsius(0.0);
    let c2 = Celsius(100.0);

    let f1 = Fahrenheit(0.0);
    let f2 = Fahrenheit(100.0);

    // Added to itself it should be the same unit
    assert_eq!(Kelvin(100.0), k1 + k2);
    assert_eq!(Celsius(100.0), c1 + c2);
    assert_eq!(Fahrenheit(100.0), f1 + f2);

    // Added to another unit it should be the conversion of the right
    // into the unit on the left added together. Remember we are using
    // floating point so there will be some margin of error for fractions
    // that occur
    assert_eq!(Kelvin(273.15), k1 + c1);
    assert_eq!(Kelvin(255.3722222222222223), k1 + f1);
    assert_eq!(Celsius(-273.15), c1 + k1);
    assert_eq!(Celsius(-17.77777777777778), c1 + f1);
    assert_eq!(Fahrenheit(32.0), f1 + c1);
    assert_eq!(Fahrenheit(-459.67), f1 + k1);

    // Testing multiple unit types added together
    assert_eq!(Fahrenheit(-427.67), f1 + k1 + c1);
    assert_eq!(Celsius(-290.92777777777775), c1 + k1 + f1);
    assert_eq!(Kelvin(528.5222222222222), k1 + f1 + c1);
}

#[test]
fn format_test() {
    assert_eq!(format!("{}", Kelvin(528.0)), "528K".to_owned());
    assert_eq!(format!("{}", Celsius(100.0)), "100°C".to_owned());
    assert_eq!(format!("{}", Fahrenheit(32.0)), "32°F".to_owned());
}
```

## Can we make this stricter?

You could make this more strict by making each field in the enum an individual
type using something like this:

```rust
pub struct Celsius(f64);
pub struct Fahrenheit(f64);
pub struct Kelvin(f64);
```

and then implementing how each one works with each other. This method allows
you to be a bit more strict about what can be taken as inputs for functions. For
instance you could make a function that calculates the volume of gas using the
Ideal Gas Law and having `Celsius` be the only input. If we used `Temperature`
like we had previously defined then we'd have to cast it to `Celsius` first
inside our function then get the value out of it to calculate the volume.
Forgetting to do the cast would mean our function only works for 1/3 of the
input. It is an example of the type system failing to enforce the logic. Like it
was stated earlier, types can be used to help enforce logic but it is not
flawless. If it was just the `Celsius` `struct` from above though then
compilation would fail if we tried to put in a `Kelvin` or `Fahrenheit` value.

Whether using `enums`, `structs`, or both they have their own overheads. Enums
are a bit more flexible, but might need a few checks every now and then to make
sure things work. Structs are a bit more rigid, but you'll probably have to
write more implementations to have different unit types work together. Really it
comes down to your needs and how you structure your API. Regardless of which
you choose you should be using types to enforce logic where you can.

## Conclusion

I hope you got a good idea of how you can use types to not only make your code
easier to use but also prevent logic bugs from creeping in. I can tell you from
personal experience this works. I've used these techniques in a much larger
Haskell code base to do conversions of different currency types for historical
financial data. Dealing with units like `USD/Share` and `Yen` and then working
out how to multiply and divide them properly all leveraged the techniques
I showed above. If we hadn't done any of that I'm almost positive logic
bugs would have crept into the program with some values being incorrect as
a result.

If you find yourself using things like `&str` or `i32` to represent values
consider wrapping them in an `enum` or `struct` to better represent what you're
trying do and make it easier to work with. I guarantee you that it will make
your library or program more robust. That being said no implementation is
perfect and no program will be free of logic bugs, but you can make it much
harder for them to appear.

If you want an example of a more fully featured unit based library I'd recommend
taking a look at [paholg's dimensioned
library](https://github.com/paholg/dimensioned). It is a really impressive
code base and does all the unit type checking at compile time! I've also pushed
all the code from this blog post to GitHub and you can find that
[here](https://github.com/mgattozzi/sci-units) if you want to fork it or work
with it at all.
