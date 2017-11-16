# Rust and the case for Web Assembly in 2018
Published November th, 2017

Every now and then a technology comes around that fundamentally changes the game
and how things work. Java and the JVM created a world where the idea of portable
code execution was possible, compile once run anywhere (as long as there was
a JVM available). JavaScript changed how the web worked and went from something
to help manipulate how web pages looked based off user interaction, to it's
modern incarnation of full applications built into the browser.

![xkcd comic number 1367 describing that web pages are apps now too][xkcd]

I think we're at that next technology now and it's staring us in the face, but
because it hasn't been used beyond hobbyist level interaction it's kind of hard
to see why one might use it for their own purposes yet. I'm talking about Web
Assembly (wasm for short). It's a big deal. We're talking an Internet unlike any
that's been seen before and it's going to change everything.

## What is wasm?

If you haven't heard of it wasm is a binary format to run programs at native
speeds in the browser. What? That's crazy you might say, well here's an example
of it [being used for a neural network to generate anime faces][moe_moe_kyun].
The code for that isn't written in JavaScript. Now while that may not be your
cup of tea you might be interested in the fact that someone put the
[Godot Game Engine][godot] into the browser. This is all early days kind of stuff
with wasm but it highlights an important thing, wasm is powerful, wasm is fast enough,
and soon we will be able to run any application on a single platform; the web.

This is a *big* deal. Just the other day
[wasm became supported on every major web browser][wasm_browser]. It's here now
and it's going to be around for a very long time.

Alright, so why is it a big deal?

1. It's a binary portable format supported in something everyone has, a web
   browser. It's very much like Java applets many years ago.
2. It's platform independent. Mac, Linux, Windows, whatever OS it doesn't matter
   as long as it has a web browser.
3. It's an open standard, so no one company owns it like Oracle does with Java.
4. It's fast, while JavaScript is great due to it's ubiquity and working
   everywhere on the web, it's interpreted so it has it's limits with how fast
   it can be (that being said the JIT compiling in browsers these days is usually
   good enough)

It's got the best parts of what made Java and Javascript the kings of the Web
and established them as languages of choice, but all the speed of C/C++/Rust
possibly.

## What does this mean for Rust?

We're poised to be *THE* language of choice for wasm. While C/C++ can be used it
has many things that can push Web Developers away from using it that Rust
doesn't suffer from, mainly:

1. Packaging libraries is a huge pain
2. Segfaults/memory unsafety/Undefined Behavior being easier to hit
3. Higher level constructs that are easier to reason about (it doesn't preclude
   the notion that something like C++ has nice things like iterators, but it
   might not make it worth using because it comes packaged with a language that
   has all the issues in 2)

Sounds kind of like what we already say to convince people to use Rust who do
systems programming to use Rust. Except, this time our target demographic is different.
It's not systems programmers, it's web developers. Generally speaking here's
what Web Developers want and need:

1. Easy packaging of libraries
2. Being able to just download a package and use it
3. Being able to call and use those libraries easily with good documentation
4. Easy integration into a build system like Webpack

We have most of these:

1. We have cargo to package up libraries
2. We also can again use cargo, but we need to integrate with npm somehow which
   I'll talk about later
3. Rustdoc is a wonderful tool and much easier to go through than a man page
4. I don't think we're here yet but I'll cover it later

We have a lot of what Web Developers want and can easily make the push to make
it a seamless experience for them, rather than making the mistake of letting
C/C++ becoming the go to choice again. Rust can and should become the Systems
Language of the web, much like C/C++ have been for computers.

You might have noticed I haven't mentioned programming languages like
Ruby/Python/Java etc. here that also cover the same things as Rust (and arguably
easier to learn) right now in terms of making it a good language for wasm. This
is because they're Gargbage Collected based languages. If the point of wasm is
to be fast, then having no runtime to worry about is a good thing. GCs add extra
overhead, would have to be shipped with each wasm package, and at that point you
might as well just use JS which is also GC based but is highly optimized for the
web.

Because of this that leaves us with Rust, C, or C++ as the only real contenders
for using wasm.

## How do we make it the language of choice?

While I've given up trying to convince die hard C/C++ people to use Rust when
they can (there are arguments not too, i.e. large code bases like Linux) as the
default, what we can do is convince a whole new generation of programmers to use
Rust. Lots of people are learning to code right now, especially for the first
time, and many of them are learning JS as their first language. Undoubtbly as
the years go on and wasm becomes more of a thing, because it will be, they will
come into contact with it and try it out or have to learn it. We want to
position Rust for that critical confluence of events that will occur which is:

1. wasm has matured and more people are using it
2. Since people are using it they need to learn the languages made to compile to
   wasm
3. They'll learn the easiest one, with the most documentation, and the generally
   accepted standard to use.

We need Rust to be point number 3 in the next few years or else we'll have
missed the boat. We're in a special position to do that now and the sooner we
start on this the better.

I'm going to break down the strategy for this into short, medium, and long term
plans that can help position Rust to be a huge player in the next big thing of
computing.

### Short Term
There's two important hurdles we need to clear short term and that's tooling and
compiling Rust to wasm. I'll cover compilation first.

Right now we can compile to wasm but that means you need emscripten and
I don't know if you've tried to set that up before but it's not as nice as
rustup. We now require two different tools to make a wasm module and that
doesn't seem like a good idea at all. The first priority from the compiler side
should be getting a wasm backend built out and working well. Well the good news
is that there is a [wasm backend][wasm_backend] target getting added. We're
already on our way there. Future work over the next year would be making sure it
works with `std` if it doesn't already, as well as optimization, and making sure
it produces wasm files that work.

Next up is getting the tooling working. This includes three major tools:

1. The RLS
2. webpack
3. NPM

Improving the RLS is going to be critical. I barely know web developers who code using
vim or emacs. They generally use VS Code, Atom, Sublime Text, and others that are more
graphical or are an IDE. They're used because the tooling is good for it. We also know
that people won't use Rust till there's an IDE available anyways, so continuing this work is
important for both getting the Web Developer demographic as well as getting others
who wouldn't use Rust without an IDE. Two birds with one stone! Most of this is
already underway anyways, but having a usable stable version in 2018 that we can
continue to improve will be a big boon. As an addendum to that making it easy to
integrate the RLS with other editors should be a snap as well. Making things
easier for end users is always a good thing for adoption, even if it means we
need to do a bit of extra maintenance to keep it working.

Next up we need to get integration with webpack working and from what I've seen
it's something that's [in progress][webpack]! Being able to drop in files and have
it Just Work™ is going to be important. Webpack has become the cargo for the web in
many ways, so being able to just hook in to that build system with no effort for people
developing their own modules is essential. I really think tooling is a huge driver of
adoption. I know I would hate using Rust without rustup or cargo even if I did
like the language.

Lastly NPM is the next big piece. What if someone writes a whole module in web assembly?
Where should they distribute it? While crates.io is great for other Rust modules it's
not used for storing code used in web development. NPM is where everyone who does web
development gets their modules from. Where am I going with this? Well the good thing about
wasm is that it's platform agnostic so we could compile a module to wasm and then upload
it to NPM to then be installed alongside other modules used for development. Being
able to do something like `cargo upload --npm` would be great. It'll let us still use
Rust tooling locally, but still allow us to hook into the wider Web Development ecosystem.

With webpack and NPM we'll need to work with the project maintainers on it but they also
either use Rust or already want it in their project so I'm sure it'll be easy to get help
with integrating into those systems.

### Medium Term

Tooling aside what's the next big piece of the puzzle further down the road? I think continued
ergonomic improvements like Non Lexical Lifetimes (NLL), more features like Const Generics and
Associated Type Constructors (ATCs) will help fix some of the pain points of Rust and leverage
it as a more powerful language in general.

Further work on documenting code, producing new tutorials, and continuously engaging with the
web development community and what their needs are when it comes to wasm. Some things will be
out of our reach, like influencing what wasm will be further down the line like with DOM
manipulation, but we can continue to improve the backend of rustc as more wasm features become
available, so that web developers can use them sooner.

Also begin developing more examples with Rust and wasm and showing it off to people. We need
to generate buzz, because it doesn't matter how good your stuff is if no one knows about it.
We'll need to start marketing Rust as a viable language for wasm as the more tricky technical
bits of using it are addressed.

### Long Term

I'm going to assume we've done the above two sections, but long term I think it's the same
goals of the community now, grow it, make it easy to use and join, and increase participation.
I think the important thing here is that by getting more people to use wasm we'll have a new
generation of systems programmers who will reach for Rust as their first choice. Maybe even
then we can have a full Rust Stack equivalent to MEAN. I'm thinking maybe something like WARD
(Web Assembly, Rocket, Diesel) but hey that's a far off dream. Still, longterm the goals will
help grow Rust, and enable users to do more cool things whether on the web, in embedded dev,
or something else.

## Why does any of this matter?

Now you might be looking at this whole article and say to yourself, "Look I like
Rust, but I don't do web development so why should I care or put effort into
helping out here?" I'm sure we all want to work on Rust in production or at
least want it to succeed, and for that we need more people using it. So why not
get more people to use it everywhere and make it more accessible? If we start
getting more people to use it, they help create more packages and write blog
posts, which gets more people to use it who can contribute in other ways, which
means now companies might start using it in production and so on and so forth.
Language choice is as much social as it is technical as to why things succeed.
While you might use Rust for other use cases, if it becomes a large enough
community, then you still stand to benefit due to having more hands create more
things and work on the compiler. We lose nothing by inviting more people into
the fold and we stand to gain everything by doing so.

## Rust 2018 Roadmap

At this point I think it would be good to point out that 2018 is the prime time
candidate for a big push in wasm and one of the main reasons I wanted to write
this article. I think it also coincides with what we've already been working on
already in 2017 as well. The ergonomics initiative can still be brought forward
into 2018 (NLL, ATCs, Incremental Compilation, and other nice things) because
making the language even friendlier and powerful is a good thing that still
benefits everyone, but also will benefit us by getting more Web Developers on
boarded to Rust. While some things will directly benefit the wasm only portion,
i.e. code generation/optimization, which will consume limited compiler team
resources, the longterm benefits far outweigh the initial cost.

The other thing that stands to benefit is documentation and how we onboard new
users. We've poured a lot of resources into this over the past two plus years
since 1.0 and it's payed off through improved error messages and more. Doing
that more will be a good thing regardless of use case.

Lastly the things we need to tackle in the short term seem like the biggest
technical things that we should do now to make further work on this easier.

More concretely as goals for 2018 I think Rust should focus on the following:

1. Continued building out and optimizing of the new wasm backend for rustc
2. Continued work on ergonomics such as NLL and others
3. Continued work on RLS and integrating it into other IDEs/Text Editors easily
4. Generate more documentation for beginners/improve current documentation
5. Work on Webpack integration
6. Work on npm integration

I've already detailed above why these items but I think this is a good goal
that's ambitious, but would have a huge impact, that also builds on existing
work, rather than throwing everything we have been working on now away just
because a new year has rolled around. It also has the added benefit of helping
everyone still while also advancing Rust as a language! I'd love to hear if
there's other things people would add or take away from this list, but this is
what I think is critically important for Rust's future.

## Conclusion

There's a lot to consider going forward but one thing I think is clear, we need
to act now. While we might not be able to manipulate the DOM with Rust and wasm
yet (allowing a pure Rust stack) we still need to position ourselves for that
future. We're short on time and first to market is a huge advantage in becoming
the default choice (how many times have you been told by someone that they would
make the next big social network that'll be different and wanted your help?).
Displacing that is hard and we even see that when talking about Rust to other
C/C++ devs. Once entrenched, even if one wants to use something else, it's hard
to not use those defaults. If we want to secure a position for Rust in the
future, this is it, and the sooner we get to making wasm a first class citizen
in Rust and for web tooling the better off we'll be when wasm stops being
something that a few people are working on and becomes the web.

[xkcd]: https://imgs.xkcd.com/comics/installing.png
[moe_moe_kyun]: http://make.girls.moe/#/
[godot]: https://twitter.com/reduzio/status/929330105050189824
[wasm_browser]: https://blog.mozilla.org/blog/2017/11/13/webassembly-in-browsers/
[webpack]: https://twitter.com/slsoftworks/status/930457337109843969
[wasm_backend]: https://github.com/rust-lang/rust/pull/45905
