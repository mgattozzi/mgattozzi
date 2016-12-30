# Rust is its community
Published Dec 29th, 2016

I've been thinking a lot about what Rust means to me as well as what
Rust is beyond safety and speed. Recently [Steve Klabnik's article](http://words.steveklabnik.com/rust-is-more-than-safety),
[Insanity Bit's](http://insanitybit.github.io/2016/12/28/why-rust-introduction) response to Steve, [Dave Herman](https://thefeedbackloop.xyz/safety-is-rusts-fireflower/) as well as [Graydon's](http://graydon2.dreamwidth.org/247406.html)
responses have got me thinking about just what Rust is as a language
and what can be done to get people on board when the safety and speed
pitch fail, because I've seen that happen.

My coworkers know that I love Rust. So much so that I got a Rust programming
book for our Secret Santa. It's a lovely language and fun to use. We use
Haskell at work though, so a lot of the things Rust has so does Haskell
and arguably more. Haskell has a strongly typed system like Rust, it has
Higher Kinded types which Rust doesn't have (yet), and due to its
functional nature can easily do things in parallel that libraries like
rayon do in Rust. If anything the only clear advantage of Rust
comparing the languages is speed in my opinion. When I mention speed and safety as a reason to use it my boss/CTO
says, "Well I can do this in modern C++ fine and I have all these libraries
so what advantage do I get using Rust? I also get this with Haskell when
optimized correctly." He's right though. Speed and safety are important but for the most part people
just want something that has libraries and things they need, and if it's good
enough and you already know it why bother switching? I think it's a bit naive
to say you won't face safety problems in C++, but getting people to switch
languages is hard to do, especially if faced with the fact that the way
they write code could actually be wrong or unsafe. No one likes being wrong.
I feel like too often talking about Rust we talk about the features and
things and not the people who make up the community. By removing the human
factor and focusing on the features we lose out, because languages are
made and worked on by people.

Rust is great as a language and I could go on about how it makes me
productive, or a better programmer, etc. but I got a lot of that from
Haskell. Rust, however, has a better community by far. If you've read
anything about Haskell or been to meetups, it always has this air of
academia that you can't escape from no matter if it's production or type
theory things. Don't get me wrong, I love that kind of stuff, but for
a lot of people they don't care about Lambda Calculus or Monads or
Lenses or Functors. They just want stuff that works well or know how to
use things. With the Haskell community it's hard to teach or learn it
without coming into contact with these kinds things or have it be talked
about. It gives it an air of elitism that, while not always intended,
can scare people away from it. I've seen tutorials that start
explaining things like you're in a higher level math class and it scares
people off. I guess the best way I can describe it is the community can
feel cold and uninviting.

Contrast that with Rust. I've been using both languages for about the
same amount of time. With Rust it is always warm. While there are
disagreements they never feel vicious (or like HN elitist level). Asking
for help feels easy to do because it's actively encouraged by the
community. There aren't stupid questions here, but answers you haven't
learned yet. There's clearly a lot of love for the language and what it
means is different to each person involved but it's clear that everyone
wants Rust to succeed. I have yet to see someone who dove into Rust who
goes, "This isn't for me", though I bet there are a few. More often then
not I've seen "Wow" as the response of new users and consistently they
bring up the community. This is Rust's greatest strength. Sure our
tooling is getting better with things like rustup 1.0 and we have some novel things
like the borrow checker but the language doesn't push anything beyond
what languages before have established really. It is the community that has
grown the language and just shared its enthusiasm with others that has
made it so successful with those who've gotten to use the language so far. I think
that safety and speed will grab a few people in a pitch but really the
thing that has caused people to stick around is the sense of community
that Rust fosters. It is easy to get help. It is easy to find good documentation
(at least now) that the community spends time working on. It is the community
coming up with cool new libraries and spending their free time developing them.
It is the community working on posts explaining how things work or showing off
other's work. It is the core developers being open to the community at
large, letting us all be part of its design through the RFC process and
actively and selflessly helping users as well as encouraging people to
contribute. It is all of these things and more because of the people
who work on it.

Rust is more then just a language. It is the community involved in
fostering its growth in a way that I've rarely seen and one that makes
me want to stick around and get others to use it because I know they
will enjoy working with some incredible, welcoming, and warm people that
will make it worth their time to and effort to invest in Rust.
