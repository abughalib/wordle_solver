import "./App.css";
import Navbar from "./header/navbar";
import { useRef } from "react";
import Home from "./home";

const App: React.FC = () => {

  const options = () => {
    let options = [];
    for (let i = 5; i < 21; i++) {
      options.push(<option value={i}>{i}</option>);
    }
    return options;
  };

  return (
    <div className="App">
      <Navbar />
      <div className="md-top-10 center">
        <h2>Select Word Length: </h2>
        <select>{options()}</select>
      </div>
    </div>
  );
};

export default App;
