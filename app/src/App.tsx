import React from "react";
import logo from "./logo.svg";
import "./App.css";
import { useState } from "react";
import FormInput from "./components/FormInput";
import ConnectWallet from "./components/ConnectWallet";
import { Claim } from "./components/Claim";

const App = () => {
  const [values, setValues] = useState({
    amount: "",
    cliff: "",
    period: ""
  });

  const inputs = [
    {
      id: 1,
      name: "amount",
      type: "text",
      placeholder: "Enter the amount of SOL tokens",
      errorMessage: "Must be a valid number!",
      label: "Amount (SOL)",
      pattern: "^[0-9]{1,20}$",
      required: true,
    },
    {
      id: 2,
      name: "cliff",
      type: "text",
      placeholder: "Token locking period (# of days)",
      errorMessage: "Must be a valid number!",
      label: "Cliff",
      pattern: "^[0-9]{1,20}$",
      required: true,
    },
    {
      id: 3,
      name: "period",
      type: "text",
      placeholder: "Period between payments",
      errorMessage: "Must be a valid number!",
      label: "Period",
      pattern: `^[0-9]{1,20}$`,
      required: true,
    }
  ];

  const handleSubmit = (e) => {
    e.preventDefault();
  };

  const onChange = (e) => {
    setValues({ ...values, [e.target.name]: e.target.value });
  };

  return (
    <div className="app">
      <ConnectWallet/>
      <form onSubmit={handleSubmit}>
        <h1>Vesting</h1>
        {inputs.map((input) => (
          <FormInput
            key={input.id}
            {...input}
            value={values[input.name]}
            onChange={onChange}
          />
        ))}
        <button>Vest</button>
      </form>
      <Claim/>
    </div>
  );
};

export default App;
