import React from 'react';
import { Link } from 'react-router';
import hljs from 'highlight.js';

class Main extends React.Component {
  containerData() {
    if (this.props.location.pathname === '/') {
      return(
        <div>
          <p>
            Thoughts and musings from a functional programmer who drinks too much coffee.
          </p>
          <p>
            Welcome! This site contains a variety of things including how to get in
            contact with me, my resume, who I am, as well as a articles I have written
            regarding technical topics usually related to Rust or Haskell.
          </p>
          <p>
            If you're a recruiter I'm currently not looking for a job right now, but
            I encourage you to get in contact with me if I ever do decide I'm
            looking for a new job.
          </p>

          <div>
            Recent Articles
            <ul>
              <li><a href="/posts/pipers.html">Announcing Pipers</a></li>
              <li><a href="/posts/scheme-parser.html">Schemers - Parser 1</a></li>
              <li><a href="/posts/russian-dolls.html">Russian Dolls and clean Rust code</a></li>
              <li><a href="/posts/scheme-ex1.html">Schemers - Exercise 1</a></li>
              <li><a href="/posts/scheme-input.html">Schemers - Input</a></li>
            </ul>
          </div>
        </div>
      );
    } else {
      return(this.props.children);
    }
  }
  render () {
    return(
    <div>
         <link rel="stylesheet" href="/static/css/styles/github.css"></link>
         <div>
          <nav className="navbar navbar-default">
              <div className="container-fluid">
                <div className="navbar-header">
                      <button type="button" className="navbar-toggle" data-toggle="collapse" data-target="#nav-top-data">
                      <span className="sr-only">Toggle navigation</span>
                      <span className="icon-bar"></span>
                      <span className="icon-bar"></span>
                      <span className="icon-bar"></span>
                    </button>
                    <Link to="/" className="navbar-brand">Barely Functional</Link>
                  </div>
                  <div className="collapse navbar-collapse" id="nav-top-data">
                      <ul className="nav navbar-nav">
                          <li><Link to="/about" activeClassName="active">About</Link></li>
                          <li><Link to="/archive" activeClassName="active">Archive</Link></li>
                          <li><Link to="/contact" activeClassName="active">Contact</Link></li>
                          <li><Link to="/resume" activeClassName="active">Resume</Link></li>
                      </ul>
                  </div>
              </div>
          </nav>
      </div>
      <div className="container">
        {this.containerData()}
      </div>
    </div>
    );
  }
}

export default Main;
