# Words are Power: Writing Authoritative Documentation
Published Feb 8th, 2017

One of the hardest problems in both Closed and Open Source Software is
documentation. How do we convey what the software does? How do we keep it up to
date? I could list more but the problems in that space alone are enormous. With
this post I want to talk about something I've noticed recently in various bits
of documentation and how you can avoid it in your own work.

Often we use words like "Sort of", "Could", "maybe", etc. in our every day
speech. It's what I like to describe as hazy language. Yes it implies that
things are like that, but it doesn't authoritatively say "This is how it is."

When it comes to every day speech that's totally fine! Your thoughts might be
buffering in a conversation, you might actually be describing possibilities, and
things that could be, but that's not the language you should be using with
documentation.

Your documentation of your projects or those you contribute to are "The Source
of All Truth." It's the gospel. When people look at them and read them they
should know that "This is it, there's no doubt in my mind that I have to do it
this way."

However, many of us write how we talk, we say things how we would describe
something to someone if they were in front of us. They're not though! These are
documents read by developers at different times of the day and guess what?
You're not there. These docs are your proxy to talk to other developers and
convey how to use your work!

How do you write your documentation in such a way that it's clear, concise, and
authoritative? I've included a few tips below that should help:

- Avoid non committed answers. "Sort of", "maybe", "well actually", things like
  this should be avoided. What you are writing either is or is not. Either you
  do it this way or you should not do it this way. There should be no doubt in
  your users mind as to how to use it!
- If you find that you're saying things like "well sometimes" then this means
  there's an odd use case that should be documented and well, possibly with it's
  own section! It means that there's some caveat the caller should be aware of
  and they need to know. Example:

  "Well sometimes if you call this method fail because another thread has a lock
  on the data." vs. "If you call this method you need to make sure no other
  thread has a lock on the data or else it might fail and panic."

  The first way is flaky and doesn't give context or the failure state, the
  second does.
- Clearly define what you're documenting. Avoid flowery language that doesn't
  add to the sentence and only causes more confusion. Example:

  "If you use this method a really cool thing will happen. The event loop will
  start up and what do you know, you have asynchronous programming available to
  you!" vs "Calling this method activates the event loop allowing you to spawn
  asynchronous events"

  The first is better for a blog post walking through things, the second is
  authoritative and better for documentation.
- If you are describing a process for a project, such as this is how PRs are
  done, you absolutely must use authoritative language. If you require your
  users to fill out a form to file a bug report you must say so in your
  documentation somewhere. If you say you "should maybe fill out this form and
  I'll get back to you" it's implying you might not need to, that this process
  is optional. This places a burden on you, the maintainer, and on the user who
  then files a bug report incorrectly. Document. Your. Processes. It'll spare
  you the headache in the long run.
- Read your documentation out loud to yourself. Does it sound awkward, wishy
  washy or meh? You probably need to rewrite it. Dog food your documentation. If
  you feel like it's not descriptive change it!
- Have someone else look it as well. They'll probably catch something you
  haven't caught and can tell you when it feels to casual and not descriptive
  enough.

If you keep these in mind you should be in good shape. Documentation is hard.
Writing docs that empower your users is even harder, but being authoritative on
the library with documentation is the easiest win you can have.

Remember, words are there to convey meaning. Documentation is also meant to
precisely convey that meaning, so take that tone with the words you choose as
well!
