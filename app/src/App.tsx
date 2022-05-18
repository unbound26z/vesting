import React, { FC, ReactNode, useMemo } from "react";
import logo from "./download.png";
import { CSSProperties } from "react";
import "./App.css";
import { useState } from "react";
import FormInput from "./components/FormInput";
import { Claim } from "./components/Claim";

import { WalletAdapterNetwork } from "@solana/wallet-adapter-base";
import {
  ConnectionProvider,
  WalletProvider,
  useAnchorWallet,
} from "@solana/wallet-adapter-react";
import {
  WalletModalProvider,
  WalletMultiButton,
} from "@solana/wallet-adapter-react-ui";
import {
  PhantomWalletAdapter,
  SolflareWalletAdapter,
} from "@solana/wallet-adapter-wallets";
import { clusterApiUrl, Connection, PublicKey } from "@solana/web3.js";
import { Program, AnchorProvider, web3, BN } from "@project-serum/anchor";
import ConnectWallet from "./components/ConnectWallet";
import idl from "./idl.json";

require("@solana/wallet-adapter-react-ui/styles.css");

//VARIABLES######################################
const programID = new PublicKey(idl.metadata.address);

//APP############################################

const App: FC = () => {
  return (
    <Context>
      <Content />
    </Context>
  );
};

export default App;

//CONTEXT########################################

const Context: FC<{ children: ReactNode }> = ({ children }) => {
  //set to 'devnet', 'testnet', or 'mainnet-beta'.
  const network = WalletAdapterNetwork.Devnet;

  // You can also provide a custom RPC endpoint.
  const endpoint = useMemo(() => clusterApiUrl(network), [network]);

  const wallets = useMemo(
    () => [new PhantomWalletAdapter(), new SolflareWalletAdapter({ network })],
    [network]
  );

  return (
    <ConnectionProvider endpoint={endpoint}>
      <WalletProvider wallets={wallets} autoConnect>
        <WalletModalProvider>{children}</WalletModalProvider>
      </WalletProvider>
    </ConnectionProvider>
  );
};

//CONTENT########################################

const Content: FC = () => {
  const [values, setValues] = useState({
    amount: "",
    cliff: "",
    period: "",
    beneficiary: "",
    num_of_periods: ""
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
      placeholder: "Period between payments (# of days)",
      errorMessage: "Must be a valid number!",
      label: "Period",
      pattern: `^[0-9]{1,20}$`,
      required: true,
    },
    {
      id: 4,
      name: "num_of_periods",
      type: "text",
      placeholder: "Number of periods in total.",
      errorMessage: "Must be a valid number!",
      label: "Period count",
      pattern: `^[0-9]{1,3}$`,
      required: true,
    },
    {
      id: 5,
      name: "beneficiary",
      type: "text",
      placeholder: "Pubkey that will recieve the tokens",
      errorMessage: "Must be a valid address!",
      label: "Beneficiary",
      pattern: `^[A-Za-z0-9]{32,44}$`,
      required: true,
    },
  ];

  const handleSubmit = (e) => {
    e.preventDefault();
  };

  const onChange = (e) => {
    setValues({ ...values, [e.target.name]: e.target.value });
  };

  const wallet = useAnchorWallet();

  // const [voteAccount, setVoteAccount] = useState({
  //   account: null,
  //   accountBump: null,
  // });

  function getProvider() {
    if (!wallet) {
      return null;
    }
    //create the provider and return it to the caller
    //network set to localnet
    const network = "http://127.0.0.1:8899";
    const connection = new Connection(network, "processed");

    const provider = new AnchorProvider(connection, wallet, {
      preflightCommitment: "processed",
    });
    return provider;
  }

  async function makeVestment() {
    const provider = getProvider();
    const vestment = web3.Keypair.generate(); //is this needed cuz its a pda
    if (!provider) {
      throw "Provider is null.";
    }

    //fixing some type of idl bug this way
    const a = JSON.stringify(idl);
    const b = JSON.parse(a);
    const program = new Program(b, idl.metadata.address, provider);

    const [vestmentPDA, vestmentBump] = await PublicKey.findProgramAddress(
      [Buffer.from("vestment")],
      programID
    );

    let amount = values.amount;
    let cliff = values.cliff;
    let period = values.period;
    let num_of_periods=values.num_of_periods;
    let beneficiary = values.beneficiary;

    try {
      await program.rpc.makeVestment(amount,cliff,period,beneficiary,num_of_periods,{
        accounts: {
          vestment: vestmentPDA,
          vestor: provider.wallet.publicKey,
          systemProgram: web3.SystemProgram.programId,
        }
      });
    } catch (err) {
      console.log("Transaction error: " + err);
    }
  }

  function claimVestment() {}

  return (
    <div className="app">
      <WalletMultiButton />
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
        <div className="vestbut">
          <button
            onClick={makeVestment}
            className="wallet-adapter-button wallet-adapter-button-trigger vestButton"
          >
            Vest
          </button>
        </div>
      </form>
      <div className="claim">
        <button
          onClick={claimVestment}
          className=" wallet-adapter-button wallet-adapter-button-trigger claimButton"
        >
          Claim tokens
        </button>
      </div>
    </div>
  );
};
