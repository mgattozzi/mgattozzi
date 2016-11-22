---
layout: post
title: Announcing Alchemist
---

I've been working on a small project for a while now and it's with great
pleasure that I get to announce it now that I have a proof of concept.
This is Alchemist a program designed to transform package names for
other distributions into the equivalent package names in your own
distribution. It then will install them for you automatically.

I was tired of Arch Linux being treated as a second class citizen in
many Github projects where instructions only existed for Ubuntu
installation. This was a shame and raises a barrier to entry to getting
users set up to use the project. Why not create a tool that removes this
barrier that I consistently came against? Why not create it so that it
worked for all unix based platforms? I can't be the only one facing this
problem and lesser known or used distributions must have it worse than
I when it comes to this.

I began work on Alchemist in order to solve this problem, put package
names in and get the correct version for my platform out. Right now
I only have a few packages that map from Ubuntu to Arch (or vice versa)
and the db schema only works for Arch and Ubuntu right now. I want to
change this though and expand the functionality to other platforms and
add more packages for users to have. This is a huge undertaking and not
one I can do alone. Any help is appreciated no matter how small. Even
if it's just adding package mappings or adding support for more distros
all code changes are welcome.

Of course I can't just claim all of this and not give a demonstration.
Here's a video of me installing some packages for Ubuntu and getting
the Arch Linux equivalent:

<iframe src="https://player.vimeo.com/video/165509394" width="640" height="357" frameborder="0" webkitallowfullscreen mozallowfullscreen allowfullscreen></iframe>

In order to build it you'll need the binary and library packages for
postgresql and sqlite of your distro so that diesel will build and
the latest version of the nightly compiler due to the nature of
diesel as a library.

I hope that this will solve a problem for some that has bugged me for
quite some time and I'm looking forward to seeing what Alchemist will
become in the future.

You can find the code located on Github
[here](https://github.com/mgattozzi/Alchemist)
