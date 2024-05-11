import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

import GaugeChart from "react-gauge-chart";
import { LineChart, Line, CartesianGrid, XAxis, YAxis } from "recharts";
import { data } from "./data";

// sqlite
// const db = await Database.load("sqlite:test.db");
// mysql
// const db = await Database.load("mysql://user:pass@host/database");

// await db.execute("INSERT INTO ...");

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    setGreetMsg(await invoke("stream_client", { name }));
  }

  async function onStop() {
    await invoke("greet_client", { name });
  }

  async function onInsert() {
    await invoke("insert_data", { entry: { name, age: 32 } });
  }

  useEffect(() => {
    listen("greet", (e: any) => {
      setGreetMsg(() => e.payload.message);
    });
  }, []);

  return (
    <div className="h-screen w-full flex flex-col justify-start bg-gray-800 text-white px-10">
      <div className="flex flex-row justify-center space-x-10 mt-7">
        <form
          className="w-1/2 flex flex-row space-x-5 h-fit"
          onSubmit={(e) => {
            e.preventDefault();
            greet();
          }}
        >
          <input
            className="p-2 rounded-xl text-black"
            onChange={(e) => setName(e.currentTarget.value)}
            placeholder="Enter a device"
          />
          <button
            type="submit"
            className="bg-blue-500 p-2 rounded hover:drop-shadow-lg hover:bg-blue-700 w-full"
          >
            Start
          </button>
        </form>

        <div className="w-1/2 flex flex-row space-x-3">
          <button
            onClick={onStop}
            className="bg-rose-500 p-2 rounded hover:drop-shadow-lg hover:bg-rose-700 w-full"
          >
            Stop
          </button>
          <button
            onClick={onInsert}
            className="bg-green-500 p-2 rounded hover:drop-shadow-lg hover:bg-green-700 w-full"
          >
            DB_insert
          </button>
          <p>{greetMsg}</p>
        </div>
      </div>
      <div className="flex flex-row border border-blue-500 mt-2 rounded-xl w-full">
        <GaugeChart className="w-1/2" nrOfLevels={20} percent={0.86} />
        <GaugeChart className="w-1/2" nrOfLevels={20} percent={0.26} />
      </div>
      <div className="flex flex-row justify-evenly align-middle border h-full border-blue-500 mt-2 rounded-xl w-full p-5 mb-7">
        <LineChart width={600} height={300} data={data}>
          <Line type="monotone" dataKey="uv" stroke="#8884d8" />
          <CartesianGrid stroke="#ccc" />
          <XAxis dataKey="name" />
          <YAxis />
        </LineChart>

        <LineChart width={600} height={300} data={data}>
          <Line type="monotone" dataKey="uv" stroke="#8884d8" />
          <CartesianGrid stroke="#ccc" />
          <XAxis dataKey="name" />
          <YAxis />
        </LineChart>
      </div>
    </div>
  );
}

export default App;
