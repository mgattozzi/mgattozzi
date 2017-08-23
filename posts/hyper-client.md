# Using an Async Hyper Client

Published March 9th, 2017

Lately I've been revamping my [GitHub
API Library](https://github.com/mgattozzi/github-rs) to be both more
ergonomic and to use the upcoming 0.11 release of
[Hyper](https://github.com/hyperium/hyper) which is asynchronous
using [Futures](https://github.com/alexcrichton/futures-rs) and
[Tokio](https://tokio.rs/) under the hood. Mainly this has been due to my
experiences using my library in my GitHub bot
[Thearesia](https://github.com/mgattozzi/thearesia). I figured if I'm already
going to be redoing how my library works might as well upgrade to the new
version of Hyper as well and provide some explanations to those wishing
to upgrade their own libraries. I'll be using Hyper at
[this commit](https://github.com/hyperium/hyper/tree/e871411627cab5caf00d8ee65328da9ff05fc53d)
for today's example. The docs are good enough for now if you want to dig
into it, but you might need to fish around for what you need. Good news
is this seems to be the [last issue
open](https://github.com/hyperium/hyper/issues/805) before release!

Before we begin I'm assuming you have a cursory knowledge of Futures and
Tokio. If you need an introduction to it I'd highly recommend reading
Andrew Hobden's post over on Asquera's blog called [The Future with
Futures](http://asquera.de/blog/2017-03-01/the-future-with-futures/).
It's an informative read and should cover enough of what we need to know
for this example!

Today, we'll go through making a request to the GitHub API asking for ourselves
as a user (in this case I'll be making a request using Thearesia's token
but follow along using [your own](https://help.github.com/articles/creating-a-personal-access-token-for-the-command-line/)). This means we'll need HTTPS support so
we'll be importing the hyper-tls library as well. This is a more
involved example than what is in the Hyper repo currently and should
help cover a good few use cases for people.

Let's get started by creating a new project:

```bash
cargo new --bin ghub
```

Then open up our new Cargo.toml file and add these lines:

```toml
hyper = { git = "https://github.com/hyperium/hyper" }
hyper-tls = { git = "https://github.com/hyperium/hyper-tls" }
tokio-core = "0.1"
futures = "0.1"
```

This will give use the newest version of hyper and hyper-tls since it'll
be using the git dependency (if you're from the future and following
along try 0.11 as the version to use instead if it's out and the
examples below are failing). Your Cargo.toml should look something like this
now:

```toml
[package]
name = "ghub"
version = "0.1.0"
authors = ["Michael Gattozzi <mgattozzi@gmail.com>"]

[dependencies]
hyper = { git = "https://github.com/hyperium/hyper" }
hyper-tls = { git = "https://github.com/hyperium/hyper-tls" }
tokio-core = "0.1"
futures = "0.1"
```

Cool we've specified all the dependencies we'll actually need! Now let's setup
the imports in our program. Open up your main.rs file and add the following
lines at the top:

```rust
extern crate hyper_tls;
extern crate futures;
extern crate tokio_core;

use tokio_core::reactor::Core;
use futures::{Future, Stream};
use futures::future;

use hyper::{Url, Method, Error};
use hyper::client::{Client, Request};
use hyper::header::{Authorization, Accept, UserAgent, qitem};
use hyper::mime::Mime;
use hyper_tls::HttpsConnector;
```

Seems like a lot right? I thought so too, but Hyper is a low level HTTP library
and we need this level of granularity to make sure our requests are setup right
for the GitHub API. The good news is that it's not scary! Let's start
building up our request so you can see where all these imports fit in.

First up let's begin crafting the request we'll need. In order to do
this we'll need
a [Url](https://hyper.rs/hyper/master/hyper/struct.Url.html)
to point the request at:

```rust
fn main() {
    let url = Url::parse("https://api.github.com/user").unwrap();
```
Pretty self explanatory, pass it in a string and that becomes the `Url`
struct. This functionality and all of it's methods was just like before in
Hyper so you can easily extend the url dynamically or get other
information from it. What, we care about here is that we have it
pointing at the end point we want to use to [get data on
ourselves](https://developer.github.com/v3/users/#get-the-authenticated-user).

Sweet! Now let's use that to make
a [Request](https://hyper.rs/hyper/master/hyper/client/struct.Request.html)
struct. This is what we'll use to set the headers to what we want for the API

```rust
    let mut req = Request::new(Method::Get, url);
```

It takes a `Method`, an enum representing all the different types of
requests you can make like GET, POST, PUT, DELETE, PATCH, etc., and a `Url`.
You have access to the handle and headers from this struct via function
calls so that you can change aspects of it that you want to
change. Alright let's get our
[Mime](https://hyper.rs/mime.rs/mime/struct.Mime.html) value and authorization
token setup for the headers

```rust
    let mime: Mime = "application/vnd.github.v3+json".parse().unwrap();
    let token = String::from("token {Your_Token_Here}");
```

Why is this media type (`Mime`) needed? Well if you look at the
[GitHub docs](https://developer.github.com/v3/media/) you can set what
you want to receive back from the API. Usually we would want JSON, so we
ask for that but we also set which version of the API to use with the
`vnd.github.v3+` part. We're telling GitHub to use version 3 of the API
because we don't want anything to break if all of a sudden they switch
to version 4 for some reason.

We also need our token to be in the header. From trial and error when
I first used Hyper in the library I realized that GitHub is expecting
input of the form `token {Your_Token_Here}` for their `Authorization`
header. It's a bit weird when I first tried to figure it out. Originally
I thought I was supposed to use Hyper's `Bearer` struct since it had a token
value inside of it but that was not the case apparently.

Let's change the headers of our `Request` now:

```rust
    req.headers_mut().set(UserAgent(String::from("ghub-example")));
    req.headers_mut().set(Accept(vec![qitem(mime)]));
    req.headers_mut().set(Authorization(token));
```

I'm doing this with the `headers_mut().set()` way due to some borrowing
errors I ran into and moved values. Meaning I couldn't do:

```rust
let mut headers = req.headers_mut();
headers.set()
```

And then using `req` later, as `req` didn't exist anymore. Not sure
if this was a rust or a Hyper issue but this works just fine. If you
figure out a more ergonomic way to do it let me know!

First up we need
a [UserAgent](https://hyper.rs/hyper/master/hyper/header/struct.UserAgent.html)
in our headers. Why? According to the
[docs](https://developer.github.com/v3/#user-agent-required) GitHub will reject
any request without it! You'll get a `403` when you try to make the
request.

Next up we are going to change our
[Accept](https://hyper.rs/hyper/master/hyper/header/struct.Accept.html) header
to utilize that Media type we had made earlier. We pass it to
[qitem](https://hyper.rs/hyper/master/hyper/header/fn.qitem.html)
which wraps it in
a [QualityItem](https://hyper.rs/hyper/master/hyper/header/struct.QualityItem.html)
type that `Accept` is expecting and then we put it in a `Vec` since
`Accept` might hold multiple `QualityItem` values in the header of
a request. We don't have multiple values here but it does need to be in a
`Vec`.

Lastly we set our
[Authorization](https://hyper.rs/hyper/master/hyper/header/struct.Authorization.html)
with our token by just passing it in to an `Authorization` struct. Boom
we've setup all of our headers and crafted the request we need. Now
let's start dealing with Futures.

```rust
    let mut event_loop = Core::new().unwrap();
    let handle = event_loop.handle();
```

First up we need to setup an event loop
([Core](https://docs.rs/tokio-core/0.1.4/tokio_core/reactor/struct.Core.html))
that will handle processing our Future when we need it. We'll also need a
[Handle](https://docs.rs/tokio-core/0.1.4/tokio_core/reactor/struct.Handle.html)
to that event loop so that our
[Client](https://hyper.rs/hyper/master/hyper/client/struct.Client.html) and
`HttpsConnector` know which event loop to be processed on.

Alright let's set the `Client` up so we can make connections:

```rust
    let client = Client::configure()
        .connector(HttpsConnector::new(4,&handle))
        .build(&handle);
```

Since we're not using the default version of the client which only does
HTTP we call the `configure()` function so that we can change the
connector. In this case we're using `HttpsConnector` from the hyper-tls
library, but presumably anything that implements the `Connect` trait
should work. This might allow for requests by other protocols if I'm not
mistaken. You might be wondering what that number four is for, well I had
to look at the source code originally since there were no online docs
for it yet. Here's what the relevant comment said, "Takes number of DNS
worker threads." Four is what had been in an older example in the Hyper
repo so I just went with that. You can change that to the number of your
liking. We then tell it to build itself and we now have a `Client` with HTTPS
support! We're almost done. Let's actually make our `Future`:

```rust
    let work = client.request(req)
        .and_then(|res| {
            println!("Response: {}", res.status());
            println!("Headers: \n{}", res.headers());

            res.body().fold(Vec::new(), |mut v, chunk| {
                v.extend(&chunk[..]);
                future::ok::<_, Error>(v)
            }).and_then(|chunks| {
                let s = String::from_utf8(chunks).unwrap();
                future::ok::<_, Error>(s)
            })
        });
```

The thing with futures is that it's always expecting some kind of future
to pass on to the next function call chained to it and eventually it
will pass on a value where it's completed when you run the future. So you can
have futures in futures. If you look at the above code it's exactly what we did.
First we tell our client to make a request and pass it our `Request` struct from
earlier. This gives us a
[FutureResponse](https://hyper.rs/hyper/master/hyper/client/struct.FutureResponse.html)
which resolves to
a [Response](https://hyper.rs/hyper/master/hyper/client/struct.Response.html) if
it works out. When we call `and_then()` we're saying once you get the
response back do this. In this case we're saying print out the status
(did we get a 200, 403, 404 or something else?) and the headers from the
response. We then call `res.body()` which creates another `Future`
called a [Body](https://hyper.rs/hyper/master/hyper/struct.Body.html),
which is a stream of
[Chunks](https://hyper.rs/hyper/master/hyper/struct.Chunk.html),
where a `Chunk` is basically a vector of bytes (`Vec<u8>`). If you look at the
first part after `body()` we're getting each `Chunk` and folding the values
into a single vector and putting it into a future `ok` so we can chain
another computation. After that we want it to take that vector and turn it into
a `String` and return that value in a `Future`! When run it'll return
either an error or the JSON String from the call.

All right, let's run the future to completion and print out the result.

```rust
    let user = event_loop.run(work).unwrap();
    println!("We've made it outside the request! \
              We got back the following from our \
              request:\n");
    println!("{}", user);
}
```

We pass in the future to the event loop and get back the value from it,
in this case a `String` and then print it out. Sweet. Let's see it in
action then!

Save the file then do:

```bash
cargo run
```

You'll get output similar to this:

```bash
Response: 200 OK
Headers: 
Server: GitHub.com
Date: Thu, 09 Mar 2017 18:59:58 GMT
Content-Type: application/json; charset=utf-8
Content-Length: 1450
Status: 200 OK
X-RateLimit-Limit: 5000
X-RateLimit-Remaining: 4999
X-RateLimit-Reset: 1489089598
Cache-Control: private, max-age=60, s-maxage=60
Vary: Accept, Authorization, Cookie, X-GitHub-OTP
Vary: Accept-Encoding
ETag: "a6fbebdd7e3ea78f873e2531b6af2562"
Last-Modified: Wed, 15 Feb 2017 16:43:54 GMT
X-OAuth-Scopes: admin:gpg_key, admin:org, admin:org_hook, admin:public_key, admin:repo_hook, delete_repo, gist, notifications, repo, user
X-Accepted-OAuth-Scopes: 
X-GitHub-Media-Type: github.v3; format=json
Access-Control-Expose-Headers: ETag, Link, X-GitHub-OTP, X-RateLimit-Limit, X-RateLimit-Remaining, X-RateLimit-Reset, X-OAuth-Scopes, X-Accepted-OAuth-Scopes, X-Poll-Interval
Access-Control-Allow-Origin: *
Content-Security-Policy: default-src 'none'
Strict-Transport-Security: max-age=31536000; includeSubdomains; preload
X-Content-Type-Options: nosniff
X-Frame-Options: deny
X-XSS-Protection: 1; mode=block
X-Served-By: 02ea60dfed58b2a09106fafd6ca0c108
X-GitHub-Request-Id: 8572:356D:62252BA:747B25E:58C1A62E

We've made it outside the request! We got back the following from our request:

{"login":"thearesia","id":25337282,"avatar_url":"https://avatars1.githubusercontent.com/u/25337282?v=3","gravatar_id":"","url":"https://api.github.com/users/thearesia","html_url":"https://github.com/thearesia","followers_url":"https://api.github.com/users/thearesia/followers","following_url":"https://api.github.com/users/thearesia/following{/other_user}","gists_url":"https://api.github.com/users/thearesia/gists{/gist_id}","starred_url":"https://api.github.com/users/thearesia/starred{/owner}{/repo}","subscriptions_url":"https://api.github.com/users/thearesia/subscriptions","organizations_url":"https://api.github.com/users/thearesia/orgs","repos_url":"https://api.github.com/users/thearesia/repos","events_url":"https://api.github.com/users/thearesia/events{/privacy}","received_events_url":"https://api.github.com/users/thearesia/received_events","type":"User","site_admin":false,"name":"Thearesia \"Sword Saint\" van Astrea","company":null,"blog":"https://github.com/mgattozzi/thearesia","location":"Kingdom of Lugnica","email":null,"hireable":null,"bio":"I'm a Github bot maintained by @mgattozzi","public_repos":0,"public_gists":0,"followers":1,"following":0,"created_at":"2017-01-25T03:25:48Z","updated_at":"2017-02-15T16:43:54Z","private_gists":0,"total_private_repos":0,"owned_private_repos":0,"disk_usage":0,"collaborators":0,"two_factor_authentication":true,"plan":{"name":"free","space":976562499,"collaborators":0,"private_repos":0}}
```

Awesome it all worked out perfectly! Here's what the code looks like all
together:

```rust
extern crate hyper;
extern crate hyper_tls;
extern crate futures;
extern crate tokio_core;

use tokio_core::reactor::Core;
use futures::{Future, Stream};
use futures::future;

use hyper::{Url, Method, Error};
use hyper::client::{Client, Request};
use hyper::header::{Authorization, Accept, UserAgent, qitem};
use hyper::mime::Mime;
use hyper_tls::HttpsConnector;

fn main() {
    let url = Url::parse("https://api.github.com/user").unwrap();
    let mut req = Request::new(Method::Get, url);
    let mime: Mime = "application/vnd.github.v3+json".parse().unwrap();
    let token = String::from("token {Your_Token_Here}");
    req.headers_mut().set(UserAgent(String::from("github-rs")));
    req.headers_mut().set(Accept(vec![qitem(mime)]));
    req.headers_mut().set(Authorization(token));

    let mut event_loop = Core::new().unwrap();
    let handle = event_loop.handle();
    let client = Client::configure()
        .connector(HttpsConnector::new(4,&handle))
        .build(&handle);
    let work = client.request(req)
        .and_then(|res| {
            println!("Response: {}", res.status());
            println!("Headers: \n{}", res.headers());

            res.body().fold(Vec::new(), |mut v, chunk| {
                v.extend(&chunk[..]);
                future::ok::<_, Error>(v)
            }).and_then(|chunks| {
                let s = String::from_utf8(chunks).unwrap();
                future::ok::<_, Error>(s)
            })
        });
    let user = event_loop.run(work).unwrap();
    println!("We've made it outside the request! \
              We got back the following from our \
              request:\n");
    println!("{}", user);
}
```

# Conclusion
Future's is changing the game in the Rust world and Hyper is stepping up
to the plate. Once I wrapped my head around it worked it became really
easy to work with. It really helps if you understand how futures work
and if you plan on upgrading to this I'd recommend having a solid understanding
how tokio and futures work together here with Hyper. Hopefully you've
gotten a better understanding how to use the library and come up with
some even more cool or complex things beyond this. I encourage you to
try it out and start prepping your projects for the eventual upgrade.
I've also posted the code on [GitHub](https://github.com/mgattozzi/ghub) for
you if you want to just clone the repo. It won't work at all till you
add your token though, so don't try to run it as is!
