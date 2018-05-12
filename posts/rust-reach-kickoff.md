# Increasing Rust's Reach Kickoff

Published May 11th, 2018

I have the great pleasure and privilege this year to be one of the mentors for Increasing
Rust's Reach. I'll be working with [Sarah][sarah] and [nano][nano] on WebAssembly and I'm really
excited to see what we accomplish over the next few months. Even after our first
meeting I just know they're gonna do some great things. Over the coming months
I'll be documenting their progress, but to kick things off the Rust Community
team is asking people to describe their story and how they contribute to Rust,
to show off the variety and breadth of our experiences and talent in a wide
variety of areas. This is my story.

## The Journey Begins

On June 7th, 2015 my [first public commit][commit] of Rust was posted to GitHub, but my story
leading up to this point actually starts back in 2013. Back around May in 2013
I came home to Boston. I had spent two years at Drexel University pursuing a degree in
Biomedical Engineering. I found at the time I didn't want to pursue that path any longer and
made the leap to Computer Science, with no idea whether I would like it, and
only the vague notion that I had always been "good" with computers and took AP CS in high school
and had really enjoyed coding. I then attended UMass Boston starting that Fall.
I didn't stay at Drexel because it was expensive and staying while
switching degrees two years in was just not going to be sustainable and so
a state school it was.

Fast forward to the end of the first school year at UMass and well it was
definitely a ride. Between Seasonal Affective Disorder (my brain can't make
enough seratonin during the Winter months), upending my life, and having only
just started to make friends at a primarily commuter school, I now also had to
figure out how I was going to pay for the upcoming school year. It was around
that time a coworker at the Stop and Shop Deli I worked at mentioned he was
joining the National Guard, troops that work for the State Government and
usually deployed for floods and things (with exceptions of course), and he
suggested I should take a look into it. It would pay for State School and
possibly net me other benefits like loan repayment etc.

I suggested this to my family and my parents were shocked to say the least. I can't even describe
their reaction but for context: I was a skinny boy, who hated doing sports
growing up, who did 6 years of choir and theater, and an absolute nerd. Not
exactly what you would consider a military type. More like a Steve Rogers before
becoming Captain America kind of deal.

On June 25th, 2014 I signed my enlistment papers and started getting ready for
Basic that October. After Basic was AIT (Advanced Individual Training, the
military really loves their acronyms), which was January of 2015. Here we
actually had laptops and phones, as well as a lot more free time than Basic. So
what's a bored private to do? I started learning Haskell, at a friend's behest, and D in my spare
time. It was a ton of fun learning something new!

Then on May 15th, 2015 Rust 1.0 was announced and my path to where I am today
really started.

## First Steps

I ended up graduating AIT the week after the release, but spent my free time
when we weren't prepping and I didn't have responsibilities consuming the Rust
Book 1st edition. It was sparse to say the least. Steve at a much later time
described it to me as writing the book for a language that hadn't even been
released. Also the compiler wasn't so nice at the time. I affectionately refer
to these as the dark ages of yore, with eldritch error messages that make
Future's errors seem tame in comparison.

I read, practiced and learned Rust when I could. Then I got home. I had an entire
summer before I went to classes. I spent the majority of it coding
[Rusty][shell], a now long abandoned terminal shell, in order to learn Rust.
It's nostalgic looking back on it. I wrote some great code:

```rust
///Read in Config
///Inner function used to pull in a default configuration file for parsing
///or the customized one if it exists
fn read_in_config() -> String{
    //Find a way to read from default if this doesn't work. let a = if else?
    let mut home_config = home_dir().expect("No Home directory");
    home_config.push(".rusty.toml");
    let default = File::open(home_config.as_path().to_str()
                             .expect("Should have a home directory to
                                     turn into a str"));
    let config = if default.is_err(){
        //Should be changed to location of git repo if compiling on your own machine
        File::open("./config/rusty.toml").ok().expect("No default file")
        } else {
            default.ok().expect("No files to open for config")
        };
    let mut reader = BufReader::new(&config);
    let mut buffer_string = String::new();
    reader.read_to_string(&mut buffer_string)
        .ok().expect("Failed to read in config");
    buffer_string
}
```

Look at that lack of understanding how to handle errors in Rust. My code has
definitely come a long way in three years.

Early Rust 1.x was a weird time, it was brand new, no one really knew if it would succeed,
the resources to learn it pretty much didn't exist, and overall there just wasn't
much of anything. I constantly used nightly for things that nowadays we just accept
as part of Rust. These days I hardly use nightly except for wasm which I think really
shows how much it has matured.

I spent that Summer in a mostly solitary manner obsessing over Rust. I probably
wouldn't recommend that to people now, let alone myself, anymore, but I was 22
and still naive in more ways than one. However, there was a Boston Rust Meetup
that started and I went to them, where I met Niko for the first time, and
started to interact more with the Rust community. My solitary work was slowly
becoming something I could do with other people.

## Becoming a Part of the Community

Over the next two years I really started to do more open source work and working
with Rust. I have a retrospective for years [1][one] and [2][two] if you're
interested as it covers most of what happened. Mainly I started my professional
career and met more people in the community. I would say around April of last
year was when that really kicked off. I went to RustFest Kyiv, I graduated in
May, and started to really get to know people in person from the community at
large. I also gave my first ever talk at RustConf, which made me happy when
people thought I had done it before. I get to thank my high school for few
things (I'm still mad I had to learn Latin as a graduation requirement), but all
the public speaking we had to do for English class finally paid off. As I grew
to know more people in the community IRL my responsibilities have also been
growing in turn, with contributions to a few projects and now being on the
wasm-wg and with that Increasing Rust's Reach.

It's been a wild few years, but the experiences the community has given me, the
people I've met, and the work we've accomplished are things I would never trade
away. Having watched the language grow as well as the amount of people involved
is heart warming.

## How do I contribute?

Well that's something that has changed over time but currently what I do is:

- Contribute to [wasm-pack][pack] as part of the wasm-wg. Getting to work with
  Ashley on this has been an utter joy. Mostly I end up doing more mentoring than
  actual coding for it, which I'm finding I like a lot more
- Working on the [rust-wasm book][book] and keeping it up to date, which is hard
  because we're moving fast and breaking things almost daily which is both
  a good and bad thing depending on who you ask, but wasm is extremely bleeding
  edge
- Now also mentoring for Increasing Rust's Reach which starts Monday!

This is what I do in a more "official" capacity, but I've contributed in other
ways over the year that aren't necessarily big and flashy, such as a lint for
rustc, and contributing documentation fixes to it as well as patches for other
crates and things. I would say a lot more of my contribution is less the code,
but more what I've talked and planned with people outside of it.

My main point is that there's a variety of ways you can contribute to the
community at large that aren't necessarily big and grand, but still help
everyone, and not all of it is code! Every little bit counts!

## Wrapping it all up

My path is a little unconventional, filled with a lot more anxiety, and doubt
than I'd have wished at the time, but being where I'm at now, seeing what Rust
can do, and where we can go makes me really excited. There's a lot going on and
I think I'll be contributing more to the community as much as I can because it's
one I really want to see grow more. It's filled with wonderful people and I'm
happy to meet more new amazing people year after year. I can't state how much
this community has had an impact on me and I'm really excited to see what other
changes for me it has in store.

[sarah]: https://twitter.com/meyerini
[nano]: https://twitter.com/nanoplink
[commit]: https://github.com/mgattozzi/Rusty/commit/9dd157da9ac6c104d46b1ac2fc3a00ae5a8485b3
[shell]: https://github.com/mgattozzi/Rusty
[one]: https://mgattozzi.com/1-year-of-rust
[two]: https://mgattozzi.com/2-years-of-rust
[pack]: https://github.com/ashleygwilliams/wasm-pack
[book]: https://rust-lang-nursery.github.io/rust-wasm
