import React from "react";
import "./main.css";

interface Length {
  length: number;
}

export default class Main extends React.Component<Length> {
  constructor(props: Length) {
    super(props);
    this.state = {
      length: 5,
    };
  }

  render() {
    return (
      <div className="main">
        <h1>Selected Word Length: {this.props.length}</h1>
      </div>
    );
  }
}
