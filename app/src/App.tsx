import React, {FC, ReactNode, useMemo} from "react";
import logo from "./logo.svg";
import { CSSProperties } from "react";
import "./App.css";
import { useState } from "react";
import FormInput from "./components/FormInput";
import { Claim } from "./components/Claim";

import { WalletAdapterNetwork } from '@solana/wallet-adapter-base'
import { ConnectionProvider, WalletProvider } from "@solana/wallet-adapter-react";
import { WalletModalProvider, WalletMultiButton } from '@solana/wallet-adapter-react-ui'
import { PhantomWalletAdapter, SolflareWalletAdapter } from '@solana/wallet-adapter-wallets'
import { clusterApiUrl } from '@solana/web3.js'
import ConnectWallet from "./components/ConnectWallet";
import { useAnchorWallet } from "@solana/wallet-adapter-react"
import idl from './idl.json'

require('@solana/wallet-adapter-react-ui/styles.css');



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
      <WalletMultiButton/>
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
        <button className="wallet-adapter-button wallet-adapter-button-trigger vestButton">Vest</button>
      </form>
      <Claim/>
    </div>
  );
};

export default App;
