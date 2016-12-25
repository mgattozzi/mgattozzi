# Russian Dolls and clean Rust code
<div class="subtitle">Published November 28th, 2016</div>

Recently I started porting my website from [Github pages
](https://mgattozzi.github.io) to this domain. I wanted to make my own
static site using Iron and code it all myself as a challenge rather then use
a Jekyll template. The fact you're reading this means that
succeeded! However, that's an article for another time. Today I wanted
to talk a little about making your Rust code more readable as well as resources
for better patterns and cleaner code.

## The Russian Doll Problem
As part of the site I use a toml configuration file and I read certain
values from it. Thing is with an `enum` representing toml values with Rust
values inside and some of those being `BTreeMaps` representing toml tables,
well, it got pretty hairy in terms of unwrapping values I actually wanted.
Since everything was using `Option` I had a lot of matching and
unwrapping of values going on. My code was slowly devolving into
this:

<img class="center-block img-responsive" src="/static/images/russian_dolls.jpg" alt="Russian Dolls">

Here's what it looked like before in fact. Not one of my prouder coding
moments:

```rust
// type Config = BTreeMap<String, toml::Value>

pub fn css(conf: &Config) -> Option<PreProc> {
    match conf.get("css") {
        Some(&Value::Table(ref tab))=> {
            match tab.get("pre_processor") {
                Some(&Value::Boolean(pp)) => {
                    if pp {
                        match tab.get("css_processor") {
                            Some(&Value::String(ref css_proc)) => {
                                if css_proc == "sass" {
                                    Some(PreProc::Sass)
                                } else if css_proc == "less" {
                                    Some(PreProc::Less)
                                } else {
                                    None
                                }
                            },
                            _ => None,
                        }
                    } else {
                        None
                    }
                },
                _ => None,
            }
        },
        _ => None,
    }
}

pub fn update_duration(conf: &Config) -> u64 {
    let sleep_default = 5;

    match conf.get("site"){
        Some(&Value::Table(ref tab)) => {
            match tab.get("sleep_update") {
                Some(&Value::Integer(val)) => {
                    val as u64
                },
                _ => sleep_default,
            }
        },
        _ => sleep_default,
    }
}
```

Gross right? Worst part is that was the *cleaner version*. Repeated patterns,
just a bunch of unwrapping through matches, it's got a whole load of code
smells. It was bothering me at work all day today. It was just sitting there
giving me that feeling you get when you've committed a code sin but
you're unsure how to atone for it. That all changed when I took a look at
the `Option` docs on the train ride home. Upon finding the function I needed
I had kicked the code smell out to the curb on my commute.

## The Mr.Wolf of Option: and_then

If you're unfamiliar with Mr.Wolf from Pulp Fiction, he's a no nonsense
man who can get you out of a bind. In this case I was deep in the code
smell surrounded with nothing but Russian dolls and I needed all the
help I could get.

<div class="video-container">
  <iframe width="auto" height="auto" src="https://www.youtube.com/embed/IgzFPOMjiC8" frameborder="0" allowfullscreen></iframe>
</div>

Man did
[and_then](https://doc.rust-lang.org/std/option/enum.Option.html#method.and_then) get me to
clean up my code. I went from the "clean" monstrosity you saw before to this:

```rust
pub fn css(conf: &Config) -> Option<PreProc> {
    conf.get("css")
        .and_then(Value::as_table)
        .and_then(|x| x.get("css_processor"))
        .and_then(Value::as_str)
        .and_then(|css_proc|
            if css_proc == "sass" {
                Some(PreProc::Sass)
            } else if css_proc == "less" {
                Some(PreProc::Less)
            } else {
                None
            })
}

pub fn update_duration(conf: &Config) -> u64 {
    let sleep_default = 5;

    conf.get("site")
        .and_then(Value::as_table)
        .and_then(|x| x.get("sleep_update"))
        .and_then(Value::as_integer)
        .unwrap_or(sleep_default) as u64
}
```

`and_then` works by taking in a function and acting on an `Option`. If it's
`Some` it extracts the value and uses the function you passed to it,
then rewraps it in a `Some` or it returns `None` if your passed in function does.
If it's `None` it just passes back `None`.
What's neat is that this allows us to chain together handling of
`Option` values and change them with a function. In my case it was
perfect because I wanted to transform these values if I was able to find
them in the configuration but use `None` if I couldn't find the value.

Not only does this make the code not look like a Russian doll getting
opened up, but it's more readable, and I can clearly see what each
transformation might do.

## When in doubt Std Lib out
The standard library contains a ton of great functions for dealing with
things like `Option` or `Result`. I'm honestly surprised I hadn't looked
them up first when I was having the problem. A new rule of thumb: If it
feels awkward and wrong there's probably a function in the standard
library to make it easier.

I've often found this to be the case when I've written Rust code and done
some awkward god awful things. For example this is a pattern I did
sometimes from when I first learned Rust:

```rust
let unwrapped: i32;

if value.is_some() {
  unwrapped = value.unwrap()
} else {
  unwrapped = 5;
}

```

The better way would have been this:

```rust
let unwrapped = value.unwrap_or(5);
```

If it feels weird or like an anti pattern there's probably a better way
to do it. If you can't find a function in the standard library to resolve
your issue I recommend looking at the [patterns repo](https://github.com/rust-unofficial/patterns)
which can help with this.

## Conclusion
Clean code is hard. It's easier to write things that just work without
having a regard for what it looks like, especially for personal projects.
However, this doesn't benefit anyone, including yourself. The good thing
is there are resources like the [patterns repo](https://github.com/rust-unofficial/patterns) or
[clippy](https://github.com/Manishearth/rust-clippy) that can catch
common mistakes to help you develop more rustic code.

If you're a newer coder or just new to Rust, I encourage you to use those resources (and
the compiler!) to write better cleaner looking code. Spend the time
making mistakes like this, to learn what not to do. As for the rest of
the community, there's always more to learn. Over a year and a half of
Rust and I'm still learning new things and I'm sure you are too! I would
also encourage contributing to the pattern repo if you've been around
for a bit. It could use some love and having examples of good patterns
of rustic code for new users will be a major boon.

The good news is this site is currently running with the non Russian
doll version! I hope this post encourages you to look at your code
for areas you could clean up or make more readable. I know I will be!
