---
layout: post
title: Schrodinger's Bug Adventures in Asynchronous Debugging
---

Lately I've been writing integration tests for [Elsen](https://elsen.co)
using node.js to verify our Haskell backend is indeed returning to us
what we expect to be there. Using a combination of the Mocha, Chai, and
Supertest frameworks I've managed to make a comprehensive set of tests.
It's a simple process: Mocha runs the tests and outputs if everything
passed or not, Supertest makes the actual requests to our API, and Chai
verifies the JSON we get back is what we would expect. One of our tests
would look like this:

```javascript
//This is our context that certain endpoints are wrapped in
context('Functions', function() {
  //This is the more specific context and where we start usin endpoints
  context('Syntax Checking', function() {
    var endpoint = '/v2/syntaxcheck';
    var syntax, syntax_res;

    //This is one of the tests. It makes the request with API
    it('Syntax is valid', function(done) {
      syntax = 'function succ(x) { return (x+1); }';
      API.post(endpoint)
        .set('Authorization',LOGIN_TOKEN)
        .send(syntax)
        .expect(200)
        .expect('Content-Type', 'application/json; charset=utf-8')
        .end(function(err,result) {
          //Here we make the actual comparisons
          //Done is used for all the calls because they're asynchronous
          if (err) return done(err);
            syntax_res = result.body;
            syntax_res.should.to.equal(true);
            done();
        });
    });

    //This is like above but we have an error here that I spent about
    //two days debugging. Look at the previous one and see if you
    //can figure it out at all.
    it('Syntax is invalid', function() {
      syntax = 'function succ(x) { return (x+ }';
      API.post(endpoint)
        .set('Authorization',LOGIN_TOKEN)
        .send(syntax)
        .expect(200)
        .expect('Content-Type', 'application/json; charset=utf-8')
        .end(function(err,result) {
          if (err) return done(err);
            syntax_res = result.body;
            syntax_res.should.to.eql(
              { msg: 'Parse error at line 1, col 31', position: { line: 1, col: 31 } }
            );
          done();
      });
  });
});
```

Note that these are only a small section of the thousands of lines of
code written to test the API. One error here though made me frustrated
to no end. If you ran these tests as they're written above then they
both pass without issue.

I discovered the issue when I was adding some more tests. I tested
whether or not the JSON I got back even it existed. The test said it
did. Then I tried to console.log() the result. Nothing was printed out.
I tested if it didn't exist as well. That test also passed. How can the
JSON I'm getting back both exist and not exist simultaneously? This
made absolutely no sense! Much like Schrodingers famous thought
experiment I had something that existed and didn't until I tried to
observe it.

Now what? The test I was doing was a POST as well much like my syntax
checker endpoint. I also had some more that did a POST. I changed each
of them to make them wrong and fail. I verified the correct results
using curl to hit the endpoints manually. I ran the tests again. All
the ones I changed failed except the syntax checker. This was where I
started to go crazy. How come it worked for some of them but not all of
them? I added a console.log() to the syntax checker as well just to see
if I got anything unlike the first time. That's when I really started
to lose my mind. I got a result back this time. What I didn't realize
then though was that the result should have printed out where my test
executed not later on in the middle of another test.

Visually it looked like this:

```
#Syntax Test executes here
Pass
#More Tests
Pass
#More Tests
Pass
#Syntax Test Result Outputs here
{ JSON }

#It should have been like this:
#Syntax Test executes here
Pass
#Syntax Test Result Outputs here
{ JSON }
#More Tests
Pass
#More Tests
Pass
```

I spent many hours checking documentation and checking to see if it was
a bug in versions of the testing frameworks. Long story short none of
that helped in the slightest. I was chasing a red herring. That's when
I commented everything out but the login test (in order to authenticate
our connections) and the syntax check test. This test still had the
console.log() statement. Guess what? Nothing printed this time! This
was the second day I was working on this test and still nothing. My
boss Zac came in at this point to help me out and we spent a while
trying to figure it out. We lloked at the docs again and other working
bits of my code. That's when we figured it out. The fact one word was
not used caused the tests to pass.

I had forgoten one word in the whole process that would cause it all to
work as intended: done.

Remember this line of code?

```
it('Syntax is invalid', function() {
  //Other stuff is in here
});
```

The anonymous function is missing the word done as a parameter. The
tests normally use it in those .end() calls I make to tell the test to
wait for whatever is in there to complete because it's asynchronus.
However without that done statement the .end() function wasn't even
being entered into causing the tests to pass otherwise. When a result
was logged later it was because it had finally been processed. When it
wasn't, that was because the test framework had already completed
running all of it's tests and so didn't bother to print the results!

Needless to say I was relieved to finally have the tests working as
expected. I learned quite a bit about debugging asynchronous code,
something I've never really touched upon. I've normally dealt with
either synchronous code or just sequential execution making this an eye
opening experience. The main take away is that if you are doing
asynchronous code make sure that you have your callback defined if you
want it to wait for the result!
