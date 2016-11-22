My Personal Blog written with Rust!

I've begun work on porting over the code from
[mgattozzi.github.io]("https://github.com/mgattozzi/mgattozzi.github.io")
so that I can serve files from a personal server and domain eventually.
I didn't want to use jekyll and thought it would be fun to write my own
server to do it. This code right now renders posts automatically in
\_posts then places them in their own folder under site which is where
everything is served from.

Currently a WIP that could eventually turn into it's own Rust based
static site generator.

Legal Notes:

Any code I've written is dual licensed under the MIT and Apache-2.0
licenses. Any contributions to this code are assumed to be under these
unless otherwise stated.

This site also uses code from highlight.js for rendering code blocks
with colored themes. It uses BSD-3 as a license and a copy of it
is located under docs/licenses for viewing.
