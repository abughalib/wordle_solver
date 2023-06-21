import React from "react";
import "./navbar.css";

export default class Navbar extends React.Component<{}, {}> {
  render() {
    return (
      <div className="navbar">
        <div className="navbar__title">Word Search</div>
      </div>
    );
  }
}
