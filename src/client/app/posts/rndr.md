# RNDR - Rocket Nginx Diesel React
Published Jan 3rd, 2017

This holiday season I had decided to rewrite this website (again) to use
Rocket, a new Rust web framework. I had also
thought, "Why not learn some React as well?". While doing so I ran into
all kinds of learning moments and what I made was basically a static site but it was fun!
While there's much to improve I did come up with a fun
little app that I think can show off what Rocket do. To set it all up we'll need [Rocket](https://rocket.rs),
[Nginx](https://www.nginx.com/), [Diesel](http://diesel.rs/) and [React](https://facebook.github.io/react/). This isn't meant to be a definitive guide but
to show off a nice little stack that's pretty fast when loaded and walk
through the code on my site as it is now. I like to call it RNDR (Render).

## What we're going to walk through
It's a very simple page that will display a button, that when clicked
will tell you how many times it has been clicked by users. Pretty simple
but it shows off some of the capabilities of all of the components in
the stack

## The components in RNDR
1. Rocket is going to serve up our content and handle incoming requests
2. Nginx is our proxy server. Since Rocket doesn't have SSL right now
   I've used Nginx to make all requests forward from port 80 to port 443.
   The site uses [Let's Encrypt](https://letsencrypt.org/) for it's
   certificates. We won't cover setup for that in this walk through.
3. Diesel is accessing our PostgreSQL server and updating
   the count as well as retrieving it. Diesel also works for sqlite but
   I opted for Postgres here due to comfort and future plans.
4. React is our frontend. It's fast once loaded (getting the JS file
   small was tough) and it's a really interesting technology. I've
   found it to be easier to work in compared to Angular personally.

## Let's look through the code

## Conclusion
If you want to see it in action it's live [here]() and the code for all
of it is at this tag on my repo [here](). Hopefully this gave you
a look at what Rocket can do as well as Diesel two Rust projects that
I really like. Think my code could use some improvement? Found a mistake
somewhere on here? Just want to say hi? If so drop by
my [repo](https://github.com/mgattozzi/mgattozzi) and open up a PR or
issue! As always, I hope you learned something new today or got some
inspiration to try something out.
