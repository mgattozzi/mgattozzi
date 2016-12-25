import React from 'react';

class Resume extends React.Component {

  render () {
    return(
      <div>
        <h1>Education</h1>
        <hr></hr>
        <h2>University of Massachusetts Boston</h2>
        <h3>Bachelor of Science (B.S.), Computer Science</h3>
        2013 - 2014, 2015 - 2017
        <p>
          Between 2014 and 2015 I received my Initial Entry
          Training for the Army. My education was put
          on hold for a year in order to complete it.
        </p>
        <h2>Drexel University</h2>
        <h3>Bachelor of Science (B.S.), Biomedical Engineering</h3>
        2011 - 2013
        <p>
          While attending Drexel I realized that I was passionate
          about computers and made the switch to Computer Science
          and back to my hometown of Boston to pursue that degree
          instead.
        </p>
        <h2>Boston Latin School</h2>
        <h3>High School Diploma</h3>
        2006 - 2011
        <p>
        Boston Latin School goes from 7th through 12th grade.
        </p>
        <h1>Experience</h1>
        <hr></hr>
        <h2>Junior Fullstack Engineer</h2>
        <h3>Elsen, Inc.</h3>
        September 2015 - Present
        <p>
          Elsen develops future-forward FinTech allowing anyone to harness, understand
          and make quick decisions using vast quantities of data without a team
          of mathematicians or programmers.
        </p>
        What I've done here:
        <ul>
          <li>Use Haskell to develop a backend platform for Factor Analysis Models</li>
          <li>Develop API Integration tests using Node.js and Haskell</li>
          <li>Work on automation of code deployment to ensure consistent delivery</li>
          <li>Use Angular to develop new features for the web app powered by the backend</li>
        </ul>
        <h2>Signal Support Systems Specialist</h2>
        <h3>Massachusetts National Guard</h3>
        June 2014 - Present
        <ul>
            <li>Ensure equipment is not defective and operating properly</li>
            <li>Assist command with fixing problems arising from equipment use</li>
            <li>Distribute and catalog equipment for field exercises</li>
            <li>Help set up communications for operations including the Boston Marathon and 4th of July</li>
        </ul>
        <h1>Technical Experience</h1>
        <hr></hr>
        <p>
          Languages:
        </p>
        <ul>
          <li>Haskell</li>
          <li>Rust</li>
          <li>JavaScript (Vanilla, Node.js, Angular, React)</li>
          <li>Python</li>
          <li>C/C++</li>
          <li>Java</li>
        </ul>
        Technologies:
        <ul>
          <li>Git</li>
          <li>Github/Gitlab/Bitbucket</li>
          <li>Windows</li>
          <li>MacOSX</li>
          <li>Linux</li>
        </ul>
        <h1>Articles</h1>
        <hr></hr>
        <h2>Schemers</h2>
        <h3>An Introduction to Rust</h3>
        <p>
          An ongoing series for newcomers to the Rust Language through
          a project where they build a Scheme Interpreter. It's goal is to make
          the language accessible by explaining harder concepts in terms that
          anyone can understand and through hands on experience. Exercises and
          answers are provided to guide the user along and to let them try out
          features as they're introduced.
        </p>
        <h1>Notable Personal Projects</h1>
        <hr></hr>
        <h2>Curryrs</h2>
        <h3>A library for easy Rust and Haskell FFI</h3>
        <p>
          The goal of this library is to make using Rust in Haskell and Haskell in
          Rust effortless. It came about after much personal frustration over trying to
          get it to work and the lack of any documentation.
        </p>
        <h2>Alchemist</h2>
        <h3>A Unix Platform Agnostic Installation Tool</h3>
        <p>
          Often I found that many code bases provide instructions for needed
          tools for specific platforms, usually Ubuntu, MacOS, and Windows, but
          neglect other distributions. The goal of Alchemist is to make that
          problem go away. With a database of package names it translates them
          into the ones for your platform and installs them with your system
          package manager.
        </p>
      </div>
    );
  }
}

export default Resume;
