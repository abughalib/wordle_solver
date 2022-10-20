import "./App.css";
import React, { useState } from "react";

async function fetchWords(): Promise<[]> {
  let resp = await fetch("http://localhost:8080/words", {
    method: "POST",
    body: JSON.stringify({
      length: 19,
    }),
    headers: {
      "Content-Type": "application/json",
    },
  });
  if (resp.status === 200) {
    let json_resp = await resp.json();
    return json_resp;
  } else {
    return await Promise.reject(resp);
  }
}

function App() {
  const [words, setWords] = useState<[]>([]);

  React.useEffect(() => {
    fetchWords().then((words) => {
      if (words) {
        setWords(words);
      }
    });
  });

  return (
    <div className="App">
      <header className="App-header">
        {words.map((word) => {
          return <p key={word}>{word}</p>;
        })}
      </header>
    </div>
  );
}

export default App;
