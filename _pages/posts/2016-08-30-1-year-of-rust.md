---
layout: post
title: One Year of Rust
---

I've been using Rust since 1.0 launched. I had heard of it before 1.0
but didn't pay much attention to it and spent some time flirting with
D before making the full switch to Rust. I fought with the compiler for
a while and had to retrain my mind to think about how to write programs
well but in the end it was all for the better and I find I'm way more
productive in Rust. I wanted to write a bit about what I've learned and
accomplished over the past year, document some of the community's achievements
, what we can do better, and what I had wished I knew when I started.

## What I've done
In no particular order I want to list what I've managed to accomplish
with Rust this year (this is mostly for myself).

- Started a [series for new Rustaceans](http://mgattozzi.github.io/archive/) to explain how you do various
  things in Rust
- Started [Alchemist](http://mgattozzi.github.io/2016/05/05/announcing-alchemist.html) a program to install packages from other
  distributions on your own by translating package names and installing
  them with your own distro's package manager
- Wrote a [Shell in Rust](https://github.com/mgattozzi/Rusty) (deprecated) to learn it and looking back at
  the code I've certaibly gotten better and removed many anti patterns
  and bad design decisions from my code. As far as I know of I'm the
  only one who has solved how to pipe Commands (I was PMed on Reddit
  about it recently actually) into each other. I would hope we had a
  better interface for it though.
- Started my [Github API Wrapper](https://github.com/mgattozzi/github-rs) and launched the initial 0.1.0 version on
  [crates.io](https://crates.io/crates/github-rs) which is my first crate.
- I was able to contribute some documentation to the Rust compiler which
  I was particularly excited about as this was my first PR for an FOSS
  project and to have it be for one of such caliber makes me happy.
- Gave a talk at Rust Boston on tooling in the ecosystem at the time.
  (multirust, rustfmt, racer, clippy, and rusty-tags)

Amongst the public things there have been some private side projects that
I'm happy with how they're moving along and my Rust has gotten way
better. I rarely run into Ownership or Lifetime issues at this point
which is a great feeling. If you're new to Rust it gets easier the more
you use it. Some of my projects are behind because of life and my job
coming close to 1.0 but I do want to work more on them when I have the
time, especially with helping new Rust users learn.

## How the community has changed
When I first started the only good documentation that existed was Steve
Klabnik's original version of the book, blog posts from pre 1.0 and the
[nomicon](https://doc.rust-lang.org/nomicon/) wasn't even done, nor was my particular favorite for new users
[Too Many Linked Lists](http://cglab.ca/~abeinges/blah/too-many-lists/book/). Multiple toolchains and multirust was alright
but needed a lot of work. Tooling was pretty much non existent. The
community had pushed for a 1.0 compiler and it kind of left other things
that languages need by the wayside like IDEs and editor plugins, though I will
say cargo was really nice and what I wished Haskell could have had at the time (I learned about stack later
when I started my job with it). Beyond that though libraries to build
things were sparse or didn't exist for a use case. Rust felt as a language
truly like a new frontier to explore and learn from and I'm glad I made the
decision to jump on then, but man those first few months were rough.

What have we gained in the past year as a community though?

- Better documentation. We now have a [docs team](https://www.rust-lang.org/en-US/team.html#Documentation-team), the [Rust Book](https://github.com/rust-lang/book)
  has been greatly improved thanks to the efforts of [Carol](https://github.com/carols10cents) and [Steve](https://github.com/steveklabnik) and
  is nearing completion, [This Week in Rust Docs](http://guillaumegomez.github.io/this-week-in-rust-docs/) covers changes that have
  been made and still need help on and is a boon to keep up to date with
  docs. Many more articles and users have made it better and right now
  I think it's one of the easiest times to start learning the language
  with the plethora of words available on the language that are from
  post 1.0. There's still more work to be done of course but Rust has
  some of the most solid documentation compared to other more mature
  languages I've used (I'm looking at you Haskell, I love you, but the
  types aren't the only things I need to understand how things work).
  Also it's super pretty and easy to navigate thanks to `rustdoc`.
- Better tooling. We now have tools like [rustup](https://github.com/rust-lang-nursery/rustup.rs), and [clippy](https://github.com/Manishearth/rust-clippy)
  has greatly improved, as have [racer](https://github.com/phildawes/racer) and [rustfmt](https://github.com/rust-lang-nursery/rustfmt). We're getting
  IDE support with [IntelliJ Rust](https://github.com/intellij-rust/intellij-rust) which is great for
  production environments and people who like that kind of thing
  (personally I'll stick with vim).
- Corporate support is now more than Mozilla. With companies like
  [Dropbox using it in production](https://blogs.dropbox.com/tech/2016/05/inside-the-magic-pocket/), [a page dedicated to companies who
  use it](https://www.rust-lang.org/en-US/friends.html) that keeps growing, and more and more people hearing about
  it in the wild it's clear Rust has gone from a small language to
  something big in the past year. I feel like it'll only get bigger from
  here on out.
- Better libraries. We now have libraries that we desperately needed or
  have been improved. [Serde](https://github.com/serde-rs/serde), [Diesel](https://github.com/diesel-rs/diesel), [Hyper](https://github.com/hyperium/hyper), [Iron](https://github.com/iron/iron), and [Ring](https://github.com/briansmith/ring)
  come to mind, but I know countless more projects exist now that
  libraries have gotten better. Not as many 1.0 projects as I would like
  to see in the ecosystem but it's a start and we're on our way there as
  a community.
- Good examples of what Rust is capable of now exist. We have an [OS built
  in Rust](https://github.com/redox-os/redox), a [Syn cookie generator](https://github.com/LTD-Beget/syncookied) to stop Syn flood attacks, and one of my favorites
  that I could not stop gushing to my coworkers about for a couple
  hours [Futures](https://aturon.github.io/blog/2016/08/11/futures/). Projects like these show the wide range of things
  Rust can do and do well. A year ago I could only really point to
  projects like [Servo](https://github.com/servo/servo). Sure we now have [Rust in Firefox](https://hacks.mozilla.org/2016/07/shipping-rust-in-firefox/) as well
  but we were hard pressed to find a non Mozilla project to point at,
  and thus the language felt like Mozilla's much like how Swift is
  Apple's language. Now though it doesn't and we can see people use it
  for so many cool things.
- A fantastic community. I've spent a year watching this community grow
  and consistently it has been a community that isn't toxic, is
  rational, and although we might get heated over things like ? vs
  try!() it always feels like it's because we want Rust to improve not
  because there's bad blood between us. The community has scaled well
  and attracts the kinds of people I would love to associate with
  offline.

We may have a long way to go in terms of these things but they've also
drastically improved in the last year alone which gives me hope about
the next year. With things like [MIR](https://blog.rust-lang.org/2016/04/19/MIR.html), [better error messages](https://blog.rust-lang.org/2016/08/10/Shape-of-errors-to-come.html), and
other things I truly believe Rust, it's tools, and community a year from
now will be completely different from what we know now and in a good
way.

## What can we do differently?
I feel like more emphasis on onboarding new users and making it more
accessible to them should be a priority. Often the complaint of it's
hard to learn is thrown around. We've made progress but we need to make
it even easier to learn. Further develop the tooling so that companies
will use it in production. Stabilizing more crates to 1.0 will greatly
help us as well. Having clear focused goals for the Rust community and
better communicating that to users so that they can understand what's
upcoming in terms of the project, what RFCs have passed, what RFCs that
have passed are actually having code developed and overall organizing
this information in an easy to access area for everyone to reference.
These are things I feel would greatly benefit Rust on the whole both
professionally and for the community as we grow. Also better support for
embedded development systems would go a long way as many people request
it but it doesn't seem to be a priority and Rust could really shine
here. Things like no std don't work without workarounds and that's not
good if we want embedded systems to also be first class citizens.

## What I wish I knew/had done while learning Rust
Here's a list of things I wish I had done earlier or knew earlier:

- Jump on #IRC. Everyone in the community loves to help out more so than
  any other place I've been and isn't toxic compared to places like
  Stackoverflow. You'll get an answer pretty much immediately and
  overall it's just a fun place to hang out on! I spent to much time
  shying away from asking questions to try and learn it on my own
  (partially stubbornness, partially because I wanted to struggle and
  learn it) that I missed that a lot of my code was bad Rust. Ask
  questions we love them and no question is stupid.
- I can't iterate this enough ask questions if you're stuck.
- Don't use clone to avoid dealing with the borrow checker. If you're
  using clone chances are your code could be done better and avoid it
  altogether. There are few instances where it's actually fine to use.
- Read the book throughly rather then jumping back to chapters to
  reference them after having skimmed the whole thing. I did that and
  I spent more time confused while referencing it.
- Document. Your. Crate. Well. While many have gotten better there were
  a few I used where I had to dig through old examples to get what
  I needed out of it. Write good examples, write good docs, and write
  good tests. It will help you and everyone around you. A solid library
  depends on them and tools like `rustdoc` make it so simple that you
  don't really have an excuse. I didn't and boy did I shoot myself in
  the foot sometimes while writing my code and coming back to it.
- Tests. Write so many tests. Rust makes it so simple compared to
  languages like Java that you should really write some if you haven't!
  (I know I still have some I should write)
- Learn good Rust design patterns. [For example](https://github.com/mgattozzi/Rusty/blob/70fe29ce78f70ca34f83c050ab522d4457abe6ca/src/process/stdproc.rs#L5-L19) if you find yourself
  using things like `is_some()` then unwrapping consider using an
  `if let Some` construct instead. We have constructs to reduce common
  use cases to small bits of code (try!() for example) and to make the
  code look cleaner and more readable.
- Don't unwrap() if you can. While it's fine to get some examples
  working it's really just an unsafe thing to do and overall a bad
  choice when building a solid library. Result exists for a reason and
  Option is there to denote the possibility of a missing value. Use
  them. They exist to be a type checker for your mind and to handle all
  use cases for something.
- Understand lifetimes and borrowing. While the compiler will greatly
  help you in this respect, the faster you learn this, and the less time
  you spend fighting the compiler the more productive you will be.
- Finally it just takes time. I'm still [learning new things](https://twitter.com/mgattozzi/status/755413395541753856) even
  a year out and I have a feeling I'm going to learn even more. While
  I'm past the basics and have a solid understanding of it now, it took
  close to a year to be this comfortable. It can be intimidating but
  Rust is a language that's worth the effort. You'll be frustrated (I
  yelled at my computer a lot) but it gets easier.

## Conclusion
I feel like I've accomplished a lot this year, as has Rust on the whole
and I'm excited to see where we go. I think we have a few things we can
improve on and I'm looking forward to writing about
it next year and to see what we all have done since then. It's definitely an
exciting time to learn and be a part of the Rust community!
