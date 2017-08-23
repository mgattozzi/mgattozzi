# Lessons learned redesigning and refactoring a Rust Library

Published April 27, 2017

Recently I've completely rewritten my
[GitHub API library](https://github.com/mgattozzi/github-rs) from the ground up.
One of the big reasons was to start leveraging the upcoming Hyper 0.11 release,
however, the main reason I rewrote this was because I actually used my own
library outside of example code and I hated using it. About a month or two or
ago I started working on [Thearesia](https://www.github.com/mgattozzi/thearesia)
a bot to manage GitHub repositories (like bors or homu). While I thought the
library was okay the way I made it before, when I used it I realized how sorely
mistaken I was. It was awkward to use, unergonomic, and was just ugly and a pain
to work with. Here's a line from one request you could use:

```rust
// Where client was defined earlier
let user = client.get_user_following_username("mgattozzi");
```

While not the worst offender, the long function name can't be split up if it
causes the code to go over 80 chars (something I consider good style and easier
to read). Contrast that with the same request to the API using the newer code in
the library:

```rust
// Where client was defined earlier
let (headers, status, json) =
    client.get()
          .user()
          .following()
          .username("mgattozzi")
          .execute()
          .unwrap();
```

Not only is it easier on the eyes, but stylistically easier to manipulate. It
also passes back the status code which for some GitHub requests is the only way
to see if something worked or not. If you need access to the headers those are
available as well. Perfect if you want to use the ETag sent back for a cached
request that won't count against the Rate Limit possibly. There are more
enhancements, mostly under the hood, but the newer design accomplishes a few
things the older version did not have:

- Easier to maintain as a library
- Easier for an end user to use
- Easier to manipulate return types and useful data being returned
- Foolproof design, users can't make invalid requests (as long as the library
  logic is correct or the user doesn't mess up the custom endpoint function)

I'll be covering how I came up with the design, what I've learned implementing
the new design, and how it strikes the balance between simplicity for the
end user and maintainer.

## Things I kept in mind
I think one of the most important things I've learned as an engineer the past
year and half has been from non-engineers. Specifically from the Head of
Marketing and Head of Product where I work. Consistently they've nailed home the
point that every decision and everything done needs to keep the end user in
mind. Can we do this all the time due to engineering limitations? No, but it
should be done if possible. With this in mind I thought of users of my library
when building it, anticipating needs, making it easier to use, and avoiding
behavior that would cause an experience that could be unergonomic or unexpected.
I would say this mindset influenced a better design then if I had just gone at
it again to get it working like I originally had. It was less of a "can I do
this" like the first iteration, but "how should I do this" and that made all
the difference in building github-rs. While I did eventually make it easier for
myself to maintain later, my first goal was to make it easier for the end user,
even at the expense of maintenance for myself. Luckily Rust provides some
powerful tools in order to avoid the issues of maintenance if used properly
(spoiler alert: it's macros).

## Design
One of the main things I wanted to do was implement a builder pattern to make
requests. One would chain functions that would build up the request someone
wanted to execute and then run it. This meant passing around three things,
a handle to a tokio-core `Core` to execute a Hyper request, a Hyper `Client`,
and then the `Request` type which contains the Url for the website and other
relevant data. However, I wanted to limit these fields so they weren't
accessible outside the crate.

I ran into a problem though. I couldn't split up my types this way. I needed
access to those inner fields for certain methods. If I put them in different
modules in the crate then those fields would have to be public, meaning the end
user could access them at any time. This was counter to my goal of constructing
a request that worked every time and it exposed the internal workings in an
uncontrolled way. It also exposed the inner guts of the data types which wasn't
really needed for every user. Luckily `pub(restrict)` was chosen to be
stabilized in the next release (1.18). This was nice as the feature was only a
temporary blocker! Using this feature I could have everything split up into
submodules and make it easy to organize all of the code and not have the design
be compromised. Here's the layout right now in the src tree:

```bash
.
├── client.rs
├── gists
│   ├── delete.rs
│   ├── get.rs
│   ├── mod.rs
│   ├── patch.rs
│   ├── post.rs
│   └── put.rs
├── headers.rs
├── issues
│   ├── delete.rs
│   ├── get.rs
│   ├── mod.rs
│   ├── patch.rs
│   ├── post.rs
│   └── put.rs
├── lib.rs
├── macros.rs
├── misc
│   ├── delete.rs
│   ├── get.rs
│   ├── mod.rs
│   ├── patch.rs
│   ├── post.rs
│   └── put.rs
├── notifications
│   ├── delete.rs
│   ├── get.rs
│   ├── mod.rs
│   ├── patch.rs
│   ├── post.rs
│   └── put.rs
├── orgs
│   ├── delete.rs
│   ├── get.rs
│   ├── mod.rs
│   ├── patch.rs
│   ├── post.rs
│   └── put.rs
├── repos
│   ├── delete.rs
│   ├── get.rs
│   ├── mod.rs
│   ├── patch.rs
│   ├── post.rs
│   └── put.rs
├── search
│   ├── delete.rs
│   ├── get.rs
│   ├── mod.rs
│   ├── patch.rs
│   ├── post.rs
│   └── put.rs
├── teams
│   ├── delete.rs
│   ├── get.rs
│   ├── mod.rs
│   ├── patch.rs
│   ├── post.rs
│   └── put.rs
├── users
│   ├── delete.rs
│   ├── get.rs
│   ├── mod.rs
│   ├── patch.rs
│   ├── post.rs
│   └── put.rs
└── util.rs
```

Each submodule corresponds to a specific section in the GitHub API documentation
that you can find [here](https://developer.github.com/v3). For each one any of
the different groupings of requests that could be made to GitHub, a submodule
was created containing files for each CRUD method, making things easier to
maintain.

Awesome, but what's the point of all this? The entry point of the whole library
lies inside the `client` module. It contains the code for the `Github` struct
and from there all requests can be constructed. The way it crafts a request is
that you chain functions that return a new type. That new type has methods that
can be used to get another one and so on and when you're ready to fire off the
request you chain an execute function call to the end of it if that type can
execute a request. If this sounds sort of like a DAG it's because it is.
You can only construct requests in one direction! The return types are nodes
and the function calls are the edges that link them together! Because
of this a user can only make a valid request to GitHub. The library does all the
heavy lifting for them. The modules make it easier to reason about from
a developer perspective, but when compiled all the `GET` methods are linked
together in some way, same with `POST`, `PUT`, `PATCH`, and `DELETE`.

This was probably one of my favorite parts of striking the balance between
maintenance and usability. It manages to do both simultaneously creating
a win/win situation.

All of this comes together through a bunch of `impl From`, `impl`s of individual
`struct`s, and having the same fields for each `struct` as well as having
functions that add to the url if needed and taking input for the URL if needed.
This seems like a lot though right? How much boiler plate code would this be
especially when there are around 470 endpoints give or take? Well it would be a
lot, most of it is actually pretty repetitive. When I first started doing it, I
was doing all of it mostly by hand to get a proof of concept working, but after
that it was implemented with macros to do all of the codegen. Why rewrite the
same thing over and over again and make it harder to change things if needed?
This brings us to the first lesson learned.

## Don't fear the macro
Macros can seem scary to people who haven't used them before. The syntax is
alien compared to Rust normally, they seem magical in what they do, and it's not
something heavily used in day to day Rust code. However, they're powerful and
can be used effectively with great results. In my earlier version of the library
I had similar code being reused constantly with only slight consistent
variations. This was perfect for macro use. In fact I had done this for adding
pagination and error conversion before using error-chain. Mind you error-chain
wasn't a thing back in 2016 when I started. I'm sure I would have used it then
if it was available.

In the old library if I decided to change up how I made requests I would have to
change every single function over to the newer design possibly (there were cases
where I wouldn't need too). With a macro I only need to change one area and boom
, the whole application gains the new benefits. There's got to be some downsides
though right? Well, if you're refactoring macros or code tons of error messages
might get thrown up and drown out the useful ones (usually they'll be the same).
I avoided this problem when refactoring my macros by creating a new one and
testing it out slowly, switching all of them over to it, and then renaming the
macro again to the old name.

For example, I had changed the internal code for one field from `&mut Core` to
be `&Rc<RefCell<Core>>` and the amount of errors that were spat out was around
100 or so or at least it sure felt that way. They were all similar though so it
made it easy to nail down the fix at least.

The main reason for macros though is that it reduced boiler plate code by a ton.
Rather than having to write a real long repetitive `impl From` for the
conversion from one type to another, I wrote it once and just hand it the types
as well as the string to add to the Url if needed. Macros also help auto
generate the functions that call the conversions from one type to the other
or sets up how to execute the call.

This is maintainable for me as the library author and makes it easy for people
to contribute as well. Speaking of which I'm happy to mentor and guide new
contributions as there are a lot of easy endpoints to write, there's just a lot
of them! You won't need to know all of the guts of the library if you want
an endpoint added you just need to add a few identifiers and strings in the
right place and all of the work is done for you and boom, new endpoint to the
library added.

Now this approach isn't needed for every library and might not be the best
design wise for your library or someone else's. However, macros can help reduce
code bloat and add more maintainability to your code and I'd say it's worth
considering depending on your needs!

A small tip about macros and not exposing them to the world: if you have a file
`macros.rs` and you define your macros in it for use only to your library you
can do so by not adding any `#[macro_export]`s for the macro and then in your
`lib.rs` file before you declare any other modules put `#[macro_use] mod
macros;` and it will make your macros available to the rest of your library but
not to the end user. I found this out after realizing I had been making the
macros publicly available to the end user. They were useless outside of the
internal workings of the library and didn't need to be exposed. Once I did this
they couldn't see them but every file in the library could. Success!

Another interesting tidbit about Rust, since macro expansions happen early on
in the compile cycle they need to be defined before being used. Since those
modules have their macros expanded in each one in the order defined in `lib.rs`,
then if it's not defined by the time it gets there then it can't expand the
macro. If you put the macro definitions first then it works, because now rustc
knows what to do when it gets to that macro. Don't believe me? You can clone my
repo and place the macros import at the bottom of `lib.rs` and it'll fail
compilation but work when back in it's original spot. It's a subtle thing but
good to know if you're getting undefined issues for macros.

## Internal Mutability or: How I Learned to Stop Worrying and Love the Rc<RefCell\<T\>>
I've been reading the second edition of the [Rust Programming
Book](https://rust-lang.github.io/book/second-edition) by [Carol
Nichols](https://github.com/carols10cents) and
[Steve Klabnik](https://github.com/steveklabnik) which has been a joy to read. I
still remember the dark ages of Rust Documentation when 1.0 just come out. We
had arcane compiler error messages and the first version of the book had been
made before the 1.0 was even ready. As a result I've picked up a few things I
didn't understand before or had trouble learning or not seeing the utility of
things thanks to their amazing rewrite. I can't understate the care and love
that permeates every word. If you haven't read it do so, both old and new
Rustaceans have a lot to gain from it. I know I did. That's where I finally
understood the idea of Internal Mutability and it's what let me not
have to pass around an `&mut Core` everywhere. That means you the user can just
have an immutable `Github` client data structure everywhere! The chapter in
particular can be found
[here](https://rust-lang.github.io/book/second-edition/ch15-00-smart-pointers.html).
With some downtime during one of my National Guard Drill weekends I knocked out
an implementation with it and it works quite well!

If this is the first time you've heard about this concept it's one where there's
a trade off. We're going from compile time checks to runtime checks when it
comes to ownership. We can't have two things have a mutable reference to a data
type. Normally the compiler would stop it at compile time
if we did. With `RefCell` it does it at run time instead. As such great
care needs to be taken when dealing with this so that two mutable borrows don't
happen at the same time. Luckily `RefCell` has a method to return a `&mut T`
that's wrapped in a `Result`. If we try to do it at run time now we can handle
this error without causing the program to crash. The way my library works now it
handles this case and it's impossible to execute two queries simultaneously
since we can't send the client across threads (maybe one day I'll implement an
`Arc` version). That's cool and all but what were the benefits of this?

Before you would have to do something like:

```rust
let mut client = Github::new("Your API Key");
```

Which can be confusing to people. Why does it need to be mutable? What if I only
want to pass a reference to a client but now I have to do a mutable one to
execute queries? It ends up placing more constraints on the end user. Switching
the field to be an `Rc<RefCell<T>>` allowed the code to be this instead:

```rust
let client = Github::new("Your API Key");
```

The implementation is still hidden from the user and because of it's
immutability (at least from the user's perspective) the user can now do more by
being constrained less! The major lesson from this change (from my original
`&mut Core` implementation) is that it hides more from the user. The less that
the user knows about the internal workings the better. Their focus should be to
just use the library and less on how to get it to work. However, there are
exceptions to this.

## Opening up the hatch: Providing access to the guts of the API
Sometimes we as library authors can't predict what a user wants to do, or we
haven't covered every use case. While a lot can be done to make sure we do
sometimes it just isn't enough and having a few functions for the end user to
use should be available. For example I've exposed the `Core` to the end user
through a function call (made possible only through this internal mutability).
Why though? The tendency for Futures/Tokio code lately has been to hide that
`Core` from the end user. If we are using two Tokio based libraries like this
then that means we'll have spun up two Cores! If you want to then do async stuff
on your own you also need to create one. That's wasteful, but as it stands it's
not exactly easy to share a single core. This is probably my biggest gripe with
Tokio's design as it stands (I still love it though warts and all).  By
providing the end user a way to access the internal `Core` they can run their
own code on it and it won't interfere with the GitHub library. Will everyone
need this? Probably not and in fact the docs for the method to expose the core
have some pretty clear warnings to the user of it. Like Rust's `unsafe` it says
hey I hope you know what you're doing but I trust you.

This seems like a weird thing to do but sometimes the safety net of a library or
the need to hide things can be too restrictive. Finding what to expose to the
users with more niche needs while still making it "just work" for others is a
bit of a balancing act but it's not a hard one to strike when you figure out
what should be exposed and what should be hidden.

## Hiding failure till the end
You might notice that none of the function calls have a `Result` type until the
code is executed. That's a lie to the library user. Internally it's keeping
track of them. Even better it's chaining them all together so that upon
execution the user is getting a stack trace of sorts about what happened. This
is handled by the wonderful error-chain library. Why though? Each step might be
a failure if adding things to the URL or executing them and so on and so forth.
The library itself has a lot of places it could fail. Early iterations on the
design had each step returning a `Result<T>`. This is what it would look like if
the old code had remained:

```rust
fn foo() -> Result<StatusCode> {
    let (status, _, _) =
        client.get()?
              .repos()?
              .owner("mgattozzi")?
              .repo("github-rs")?
              .branches()?
              .execute()?;
    status
}
```

That's the clean version. Imagine doing this with `match` or `unwrap()` all over
the place. It would look horrible. Contrast that with how the library actually
does it:

```rust
fn foo() -> Result<StatusCode> {
    let (status, _, _) =
        client.get()
              .repos()
              .owner("mgattozzi")
              .repo("github-rs")
              .branches()
              .execute()?;
    status
}
```

Much cleaner and the user only has to deal with failure of execution once. This
hides most of the error handling from the user internally and just passes the
errors along until the end! It's simple for the user and that's what matters.
Now I will say this was one of those things where I had to trapeze through quite
a few hoops to get it working properly. However, I think this made things much
easier to deal with in the long run and has made the API a stable one to use as
well as still displaying pertinent error information.

While reading this you might of thought I could have done the `impl From`s as
a bunch of `Result` types. Doing that would have ended up with the same results
code wise as the current implementation, minus a few tweaks, and the
intermediate `? ` would not be needed, however the code to do a transformation
must not fail for `From` and `Into`. By putting the `Result` inside we've bent
the rules a bit as the struct transformation does happen, just the value inside
errors possibly and so the conversion doesn't break the rules. If it did return
a `Result` type we should use `TryFrom` or `TryInto` but they aren't stabilized
yet (surprisingly) and are on nightly Rust, which I want to avoid if possible.
Once that comes out I'll be switching the code over to that for sure. The types
will change but the code will run the same for the user.

## Conclusion
I hope you've learned a little something today, maybe about thinking about API
design, maybe macros, or maybe something else. I've walked you through my
thought process and what I learned coming up with a new user focused design for
[github-rs](https://github.com/mgattozzi/github-rs). It covered the thoughts
that drove the redesign, the design itself, the magic of macros, internal
mutability and it's advantages, as well as error handling in a builder pattern
like library. I encourage you to give the library a shot and let me know if I've
succeeded or if I could improve the ergonomics and usability even more.
Seriously, tell me if you have frustrations with it at at all. I promise I don't
bite :D. Even if you don't, poke around the code, maybe you'll find something
you can plunder for your own library!
