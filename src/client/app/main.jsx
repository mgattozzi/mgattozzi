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
              <li><Link to="/hyper-async">Building an Asynchronous Hyper Server</Link></li>
              <li><Link to="/diesel-powered-rocket">Diesel Powered Rocket</Link></li>
              <li><Link to="/rust-is">Rust is its community</Link></li>
              <li><Link to="/pipers">Announcing Pipers</Link></li>
              <li><Link to="/scheme-parser">Schemers - Parser 1</Link></li>
              <li><Link to="/russian-dolls">Russian Dolls and clean Rust code</Link></li>
              <li><Link to="/scheme-ex1">Schemers - Exercise 1</Link></li>
              <li><Link to="/scheme-input">Schemers - Input</Link></li>
            </ul>
          </div>
        </div>
      );
    } else {
      return(this.props.children);
    }
  }
  render () {
    const divStyle = {
      fontFamily: 'Georgia, Serif',
      fontSize: '16px',
      lineHeight: '1.5',
      maxWidth: '42em',
      hyphens: 'auto',
      WebkitHyphens: 'auto',
    };

    return(
    <div>
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
                        <li><Link to="/about"
                                  className="glyphicon glyphicon-user"
                                  activeClassName="active">
                                  &nbsp;About
                        </Link></li>
                        <li><Link to="/archive"
                                  className="glyphicon glyphicon-pencil"
                                  activeClassName="active">
                                   &nbsp;Archive
                        </Link></li>
                        <li><Link to="/contact"
                                  className="glyphicon glyphicon-envelope"
                                  activeClassName="active">
                                  &nbsp;Contact
                        </Link></li>
                        <li><Link to="/resume"
                                  className="glyphicon glyphicon-list"
                                  activeClassName="active">
                                  &nbsp;Resume
                        </Link></li>
                        <li><a className="glyphicon glyphicon-console"
                               href="https:/github.com/mgattozzi">
                               &nbsp;Github
                        </a></li>
                      </ul>
                  </div>
              </div>
          </nav>
      </div>
      <div className="container" style={divStyle}>
        {this.containerData()}
      </div>
    </div>
    );
  }
}

export default Main;
