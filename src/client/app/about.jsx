import React from 'react';

class About extends React.Component {

  render () {
      const authorStyle = {
        maxWidth: '500px',
        width: '100%',
        height: 'auto',
      };

      const flowerStyle = {
        maxWidth: '100%',
        height: 'auto',
      };
    return(
    <div>
      <img src="/static/images/author.jpg" alt="Author's image" className="center-block" style={authorStyle}></img>
      <p>
        My name is Michael Gattozzi. I'm finishing up my CS Undergrad at Umass Boston
        this year. I work at a Financial Tech Startup called Elsen, Inc. where I get to write
        Haskell in production! In my spare time I write a lot Rust code and
        lately I've been bridging the gap between the two languages. I also have the
        pleasure of serving in the MA National Guard where I get to work on
        radios and computers in the S6 of the Battalion I work at.
      </p>
      <p>
        I love tweaking my laptop setup in order to get it just the way
        I want it too. Currently I use Arch Linux with bspwm as my window manager. You can find
        my configuration of it <a href="https://github.com/mgattozzi/bspwm_config">here </a>
        and you can find my dotfile configurations
        <a href="https://github.com/mgattozzi/dotfiles">here </a>.
      </p>
      <div>
        Current Tech interests include:
        <ul>
          <li>Rust</li>
          <li>Haskell</li>
          <li>Machine Learning</li>
          <li>Fin Tech</li>
          <li>Metasploit</li>
          <li>Block Chain Technology</li>
        </ul>
      </div>
      <p>
        When I'm not working on code I enjoy looking for new delicious coffee
        places in Boston, catching up on newer TV and Anime shows as well as
        playing newer video games when I get the chance (the current ones being
        The Witcher 3 and Civilization 6). I also enjoy taking photographs in my
        spare time and particularly enjoy macro photography.
      </p>
      <p>
        Here's an example of one from my trip to Europe during the summer of 2015:
      </p>
      <img src="/static/images/macro.jpg" alt="Macro photograph of a flower" className="center-block" style={flowerStyle}></img>
    </div>
    );
  }
}

export default About;
