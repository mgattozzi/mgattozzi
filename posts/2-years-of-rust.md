# Two Years of Rust
Published May 14th, 2017

Last year I published my [One Year of Rust](https://mgattozzi.com/1-year-of-rust)
article to document how the community has changed, what I managed to accomplish
for myself, and what we could do better. This year I want to reflect on these
topics again as well as a few other things. With Rust's second birthday (and my
second year of Rust) I figured this is a good time to look at the
accomplishments of everyone involved as well as reflecting on how I've
personally grown as a Rust Developer.

## What I've done
For me this has been an incredible year for Rust.

- I've finally started to use and learn about some of Rust's more advanced
  features in a way like
  [AsRef](https://twitter.com/mgattozzi/status/854703245347913729) or others
  buried in the `std` docs.
- I started to figure out how to use
  [Rust in Haskell](https://mgattozzi.com/haskell-rust) and if you really want
  how to use [Haskell in Rust](https://mgattozzi.com/rust-haskell).
  Thanks to all who have contributed to
  [curryrs](https://github.com/mgattozzi/curryrs) to help with expanding the
  capability. I hope to devote more time to this when possible.
- [github-rs](https://github.com/mgattozzi/github-rs) has undergone a
  [major refactoring](https://mgattozzi.com/refactor-rust) and will also soon
  have a more human readable crate for function names. This came about since
  those I've talked to about it were split on which way they would like to use
  the code. Now you'll be able to choose what suits you and still get the
  benefits of how it's written.
- I blogged quite a bit more this year and wrote some articles to give back the
  knowledge I've gained to the community. I'm hoping this year I can tackle some
  more challenging stuff and write some nifty things for others to be able to
  utilize.
- I managed to write a
  [compiler error](https://github.com/rust-lang/rust/pull/39116) that was
  finally released in
  [Rust 1.17](https://blog.rust-lang.org/2017/04/27/Rust-1.17.html) and was
  prominently featured in the release notes. It made all of the
  work on it really worth it and I hopes users of dynamic languages avoid the
  pitfalls that strings can cause. Big thanks to
  [Manishearth](https://github.com/Manishearth) for mentoring
  and [nrc](https://github.com/nrc) for reviewing the PR!
- I was lucky enough to go to Kyiv for RustFest 2017 thanks to Yann at 1Aim. It
  was a wonderful weekend of technical and non-technical talks and Rust.
  However, the best part of the whole weekend was finally meet many fellow
  Rustaceans in the flesh including but not limited to
  [killercup](https://github.com/killercup),
  [Manishearth](https://github.com/Manishearth),
  [steveklabnik1](https://github.com/steveklabnik),
  [ag_dubs](https://twitter.com/ag_dubs),
  [llogiq](https://github.com/llogiq),
  [gcouprie](https://twitter.com/gcouprie),
  [pe_meunier](https://twitter.com/pe_meunier),
  [hoverbear](https://github.com/Hoverbear),
  [NGC_3572](https://twitter.com/NGC_3572),
  [Alex Chricton](https://github.com/alexcrichton) (thanks for turning me onto
  stroopwafel), [fnichol](https://twitter.com/fnichol) and many others whose
  online profiles I do not know. It was great meeting all of you! You guys made
  my first programming conference ever a complete blast and really brought the
  principles of the Rust community offline and made it a completely enjoyable
  experience. I really hope to see you guys again sooner rather than later.

## How the community has/hasn't changed
It's certainly felt like it has gotten bigger. I've seen more people at the
Boston Rust meetups,
[we've had more people
respond](https://twitter.com/jntrnr/status/861305270852399105) to the 2017
community survey in the first 72 hours than the whole of 2016's, and overall a
bigger influx of articles and questions in [/r/rust](https://reddit.com/r/rust)
and other places where discussion occurs. One thing that hasn't changed though
(FWIW) is the positivity from the community as well as the willingness and
ability to laugh about ourselves and not take everything too seriously. Just
take a look at the [Fireflowers incident](https://brson.github.io/fireflowers/).
We spent a good couple days of discussing what Rust is then topped it off by
shitposting Rust memes (at least in [/r/rust](https://reddit.com/r/rust)) to
cool ourselves down and blow off some end of year steam. This is the kind of
stuff I like to see because it means we aren't an Evangelical Strike Force but
just a bunch of geeks who love Rust.

I think that what's amazing is that even as we grow that positivity is
infectious and it's not slowing down.
[Others notice](https://twitter.com/sehurlburt/status/858616307708592130) that
kind of community attitude. It's easy to be mean but it's a lot harder to nice
all the time. While I have seen cases of more mean spirited things (hey, we're
not perfect) the
excellent moderation teams and an enforced Code of Conduct limit the scope of
that kind of damage. Too many developers don't submit code if they perceive
the community as toxic. Why would you if all you'll get is snubbed? I'm hoping
this trend continues as we invite more people into the fold.

Here's an interesting tidbit for the Rust compiler. Contributors to rustc from
last year to now has gone from 1466 to 1949 up 483 users or ~32% from last year!
The command I used for this data is
`git shortlog -s -n | cut -f2 | uniq | wc -l` it may be a bit off if someone
used different names when committing. This was from May 15th, 2016 to May 14th,
2017 when I published this article. There have also been 11 releases of Rust
since then (1.9 to 1.17)!

It's hard to quantify some things with numbers though and some of this is just a
gut feeling, but for the first time Rust just feels more noticed and mainstream.
Now, this is my perception mind you so I don't have hard data to back this up,
just a feeling. Two years ago and even last year alone it really felt like Rust
was in this precarious place, that it was do or die in terms of being accepted
by a larger audience of programmers. Now though, I just get the feeling that the
most precarious part of Rust's journey is over. Ask me again next year though
and we'll see if I'm wrong or not.

## What can we do differently?
With that being said what could we as a community do this year? Where can we go
to from here? These are my opinions of what I think we can do to really help the
grow Rust further:

- Scaling the sub teams. We're getting more people and the teams need to account
  for that growth. Open source is tiring to do with only a few people. If you
  have more trusted individuals to help out it reduces the burden on the more
  important people on the teams for bigger tasks and I think the sooner we do
  this the better. It's better to be prepared for this kind of thing.
- In the same vein, onboarding new compiler contributors I think would be
  important to sustain Rust's growth. The compiler team
  can only do so much in a release. Mentoring and growing new contributors and
  keeping them on for the long haul can free up the main compiler team to work
  on bigger changes like a better type system. Part of this is through
  documentation (I'm working on getting
  [rust-forge](https://github.com/rust-lang-nursery/rust-forge) to have the
  compiler docs hosted there) as well as finding smaller things for people to do
  that aren't crazy hard but just need some explanation. Many hands make light
  work and I think we as a community can easily scale this up by aggregating
  knowledge and investing in contributors more vigorously.
- Docs, docs, docs. I will say this time and time again. One of Rust's greatest
  strengths is the documentation, but for less used functions/types/modules
  we're still lacking. Steve and the rest of the docs team can only do so much
  in a given day! I hope the community will rally around this and document more
  of Rust for others. Especially the more quirky parts or more difficult parts
  like items in `std::sync`. It's really easy to send in a PR for docs to rustc
  as well! Ping me if you want pointers on how to do it!

Overall I think the core teams have done a great job of growing and sustaining
the project up to this point. I think we just need to consider long term where
Rust will be and we'll need the people and infrastructure to do it. Better to
have it in place now then have it be too late. Otherwise, the community is
awesome, supportive, and coming up with all kinds of really neat stuff, as well
as improving everyone's day to day work flow. We should keep that up!

## My wish list of lang features
Here's a non exhaustive list of what I would like to see be added or worked on
over the next year (even if the priorities are set by the 2017 survey):

- Time spent on faster compile times, most likely accomplished through
  incremental compilation, though that has been in the work for some time.
- Higher Kinded Types so I can use Monads in Rust. I also came up with a
  nifty Finite State Machine that needs this and stabilizing `impl Trait` being
  used inside traits for it to work.
- Macros 2.0 (because I really just love macros)
- `TryFrom` and `TryInto` because more often than not my conversions will
  possibly fail
- `Box` syntax stabilized
- `FnBox()` stabilized

There's probably more but these are the things I've wanted in stable for some
time now/to even be implemented.

## Conclusion
Rust has changed so radically in the past year that as I've written this and
looked back it just blows my mind on all that has been accomplished as well as
what's being planned. It's crazy to see and I'm super excited to see where we'll
be next year. Hopefully more great people, talks, crates, language items, and
tooling. I know I can't wait to look back at this next year just to see how much
has changed between now and then.
