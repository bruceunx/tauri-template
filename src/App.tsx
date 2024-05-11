import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    setGreetMsg(await invoke("stream_client", { name }));
  }

  async function onStop() {
    await invoke("greet_client", { name });
  }

  useEffect(() => {
    listen("greet", (e: any) => {
      setGreetMsg(() => e.payload.message);
    });
  }, []);

  return (
    <div className="h-screen w-full flex flex-row justify-center bg-gray-800 text-white space-x-10 p-10">
      <form
        className="w-1/2 flex flex-row space-x-5 h-fit"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button
          type="submit"
          className="bg-blue-500 p-2 rounded hover:drop-shadow-lg hover:bg-blue-700 w-full"
        >
          Greet
        </button>
      </form>

      <div className="w-1/2">
        <button
          onClick={onStop}
          className="bg-rose-500 p-2 rounded hover:drop-shadow-lg hover:bg-rose-700 w-full"
        >
          stop
        </button>
        <p>{greetMsg}</p>
      </div>
    </div>
  );
}

export default App;
