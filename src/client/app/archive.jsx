import React from 'react';
import {Link} from 'react-router'

class Archive extends React.Component {

  render () {
    return(
      <div>
        <h1>Archive</h1>
          <p>
            You'll find collections of articles I've written here that can be
            grouped together by topic
          </p>
        <h2>Alchemist</h2>
          <p>
            Articles regarding my package name conversion tool.
          </p>
          <ul>
            <li><Link to="/announcing-alchemist">
              Announcing Alchemist: v0.1.0
            </Link></li>
          </ul>
        <h2>Rust</h2>
          <p>
            Articles about various topics in Rust
          </p>
          <ul>
            <li><Link to="/blog-about-rust">
              Why you should be blogging about Rust
            </Link></li>
            <li><Link to="/1-year-of-rust">
              One year of Rust
            </Link></li>
            <li><Link to="/understanding-where-clauses">
              Understanding where clauses and trait constraints
            </Link></li>
            <li><Link to="/russian-dolls">
              Russian Dolls and clean Rust code
            </Link></li>
            <li><Link to="/pipers">
              Announcing Pipers
            </Link></li>
            <li><Link to="/rust-is">
              Rust is its community
            </Link></li>
            <li><Link to="/diesel-powered-rocket">
              Diesel Powered Rocket
            </Link></li>
            <li><Link to="/hyper-async">
                Building an Asynchronous Hyper Server
            </Link></li>
          </ul>
        <h2>Schemers</h2>
          <p>
            A series of articles aimed at new Rust users to learn the language
            by writing a scheme interpreter. These are listed in order.
          </p>
          <ul>
            <li><Link to="/scheme-input">
              Input
            </Link></li>
            <li><Link to="/scheme-ex1">
              Exercise 1
            </Link></li>
            <li><Link to="/scheme-parser">
              Parser 1
            </Link></li>
          </ul>
        <h2>Haskell and Rust</h2>
          <p>
            Articles doing FFI with Haskell and Rust
          </p>
          <ul>
            <li><Link to="/haskell-rust">
              FFI with Haskell and Rust
            </Link></li>
            <li><Link to="/rust-haskell">
              Using Haskell in Rust
            </Link></li>
          </ul>
        <h2>How do I X in Rust?</h2>
          <p>
            A collection of articles aimed at new Rust users to avoid growing pains
            normally associated with learning the language.
          </p>
          <ul>
            <li><Link to="/how-do-i-str-string">
              How do I convert a &amp;str to String in Rust?
            </Link></li>
            <li><Link to="/how-do-i-std-macros">
              How do I use the Standard Library macros in Rust?
            </Link></li>
          </ul>
        <h2>JS</h2>
          <p>
            Articles regarding my usage of JavaScript.
          </p>
          <ul>
            <li><Link to="/schrodingers-bug">
              Schrodinger's Bug: Adventures in Asynchronous Debugging
            </Link></li>
          </ul>
      </div>
    );
  }
}

export default Archive;
