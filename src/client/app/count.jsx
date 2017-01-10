import React from 'react';

class Count extends React.Component {

  constructor() {
    super();
    this.state = { count: 0, disabled: false};
    fetch('https://mgattozzi.com/count')
      .then((response) => response.json())
      .then((responseJson) => {
        this.setState({count: responseJson.count});
      })
      .catch((error) => {
        console.error(error);
      });

    this.clickMe = this.clickMe.bind(this);
  }

  clickMe() {
    this.setState({disabled: true});
    fetch('https://mgattozzi.com/count/', {method: 'PUT'})
      .then((response) => response.json())
      .then((responseJson) => {
        this.setState({count: responseJson.count, disabled: false});
      })
  }

  render () {
    return(
      <div>
        <div className="panel panel-primary">
          <div className="panel-heading">
            Number of times the button has been clicked
          </div>
          <div className="panel-body text-center">{ this.state.count }</div>
        </div>
        <div>
          <button onClick={this.clickMe}
                  className='btn btn-danger btn-lg center-block'
                  disabled = {this.state.disabled}>
            Click Me!
          </button>
        </div>
      </div>
    );
  }

}

export default Count;
