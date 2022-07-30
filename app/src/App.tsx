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
import { clusterApiUrl, Connection } from "@solana/web3.js";
import { Program, AnchorProvider, web3, BN } from "@project-serum/anchor";
import ConnectWallet from "./components/ConnectWallet";
import idl from "./idl.json";

require("@solana/wallet-adapter-react-ui/styles.css");

const App: FC = () => {
  return (
    <Context>
      <Content />
    </Context>
  );
};

export default App;

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

const Content: FC = () => {
  const [values, setValues] = useState({
    amount: "",
    cliff: "",
    period: "",
    beneficiary: ""
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
      name: "beneficiary",
      type: "text",
      placeholder: "PubKey that will recieve the tokens",
      errorMessage: "Must be a valid address!",
      label: "Beneficiary",
      pattern: `^[A-Z0-9]{32,44}$`,
      required: true
    }
  ];

  const handleSubmit = (e) => {
    e.preventDefault();
  };

  const onChange = (e) => {
    setValues({ ...values, [e.target.name]: e.target.value });
  };

  const wallet = useAnchorWallet();

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

<<<<<<< Updated upstream
=======
  //onclick make vestment button do this
  async function makeVestment() {
    const provider = getProvider();
    if (!provider) {
      throw "Provider is null.";
    }

    //fixing some type of idl bug this way, but basically importing the program into the constant so we have access to the func
    const a = JSON.stringify(idl);
    const b = JSON.parse(a);
    const program = new Program(b, idl.metadata.address, provider);

    try {
      //take the values from the form
      let amount = values.amount;
      let cliff = values.cliff;
      let period = values.period;
      let num_of_periods = values.num_of_periods;
      let beneficiary = new PublicKey(values.beneficiary);

      //this is our token mint
      let tokenMint = "6bscZfAt91RAfqAsUTu9gSve6gALhiUChJFsLXjVbJZS";
      let tokenMintKey = new PublicKey(tokenMint);
      //PDA: Token account of the vestment
      const [vestedTokens] = await PublicKey.findProgramAddress(
        //token acc from the vestment
        [Buffer.from("vested-tokens"), beneficiary.toBuffer()],
        program.programId
      );

      //PDA: The actual vestment account
      const [vestment, vestmentBump] = await PublicKey.findProgramAddress(
        [Buffer.from("vestment"), vestedTokens.toBuffer()],
        programID
      );

      const [vestmentLedger] = await PublicKey.findProgramAddress(
        [Buffer.from("vestment-ledger"), beneficiary.toBuffer(), Buffer.from("vestment"), tokenMintKey.toBuffer()],
        programID
      );

      //gaining access to the users tokens
      const vestorTokenAcc =
        await provider.connection.getParsedTokenAccountsByOwner(
          wallet!.publicKey,
          { mint: new PublicKey(tokenMint) }
        );

      //passing the arguments into our instruction so we can put it in the transaction
      const mV = program.instruction.makeVestment(
        new anchor.BN(amount), //BN=big number
        new anchor.BN(cliff),
        new anchor.BN(period),
        num_of_periods,
        {
          accounts: {
            vestmentLedger: vestmentLedger,
            vestment: vestment,
            vestor: provider.wallet.publicKey,
            vestorTokenAccount: vestorTokenAcc.value[0].pubkey, //its an array for some reason
            vestedTokens: vestedTokens,
            vestedTokensMint: new PublicKey(tokenMint),
            beneficiary,
            tokenProgram: TOKEN_PROGRAM_ID,
            rent: SYSVAR_RENT_PUBKEY,
            systemProgram: web3.SystemProgram.programId,
          },
        }
      );

      //taking the most recent blockhash
      const recentBlockhash = (await provider.connection.getLatestBlockhash())
        .blockhash;
      //making our transaction
      const tx1 = new Transaction({
        feePayer: wallet!.publicKey,
        recentBlockhash: recentBlockhash,
      });
      //passing our instruction into the transaction
      tx1.add(mV);

      //sending the transaction = interacting with the blockchain
      if (wallet) {
        await sendTransaction({
          transaction: tx1,
          connection: connection,
          wallet: wallet,
        });
      }

      toast.success("Vesting succesful!");
    } catch (err) {
      console.log("Transaction error: " + err);
      toast.error("Failed to vest!");
    }
  }

  async function claimVestment() {
    let tokenMint = "6bscZfAt91RAfqAsUTu9gSve6gALhiUChJFsLXjVbJZS";
    let tokenMintKey = new PublicKey(tokenMint);

    const provider = getProvider();
    //const vestment = web3.Keypair.generate(); //is this needed cuz its a pda
    if (!provider) {
      throw "Provider is null.";
    }

    //fixing some type of idl bug this way
    const a = JSON.stringify(idl);
    const b = JSON.parse(a);
    const program = new Program(b, idl.metadata.address, provider);

    try {
      //our token mint
      let tokenMint = "6bscZfAt91RAfqAsUTu9gSve6gALhiUChJFsLXjVbJZS";
      //taking the wallet of the user that claims
      let beneficiary = provider.wallet.publicKey;
      //PDA: Token account of vestment
      const [vestedTokens] = await PublicKey.findProgramAddress(
        [Buffer.from("vested-tokens"), beneficiary.toBuffer()],
        program.programId
      );
      //PDA: Vestment
      const [vestment, vestmentBump] = await PublicKey.findProgramAddress(
        [Buffer.from("vestment"), vestedTokens.toBuffer()],
        programID
      );
      //Token account of user that wants to claim
      const beneTokenAcc = await provider.connection.getParsedTokenAccountsByOwner(
        wallet!.publicKey,
        { mint: new PublicKey(tokenMint) }
      );

      const [vestmentLedger] = await PublicKey.findProgramAddress(
        [Buffer.from("vestment-ledger"), beneficiary.toBuffer(), Buffer.from("vestment"), tokenMintKey.toBuffer()],
        programID
      )

      //taking our instruction and passing arguments
      const cV = program.instruction.claimVestment({
        accounts: {
          vestmentLedger: vestmentLedger,
          vestment: vestment,
          beneficiary: beneficiary,
          beneficiaryTokenAccount: beneTokenAcc.value[0].pubkey,
          vestedTokens: vestedTokens,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: web3.SystemProgram.programId,
        },
      });
      //taking recent blockhash
      const recentBlockhash = (await provider.connection.getLatestBlockhash())
        .blockhash;
      //making our transaction
      const tx2 = new Transaction({
        feePayer: wallet!.publicKey,
        recentBlockhash: recentBlockhash,
      });
      //passing our instruction to our transaction
      tx2.add(cV);
      //sending our instruction
      if (wallet) {
        await sendTransaction({
          transaction: tx2,
          connection: connection,
          wallet: wallet,
        });
      }
      toast.success("Successful claim!");
    } catch (err) {
      console.log("Transaction error: " + err);
      toast.error("Failed to claim!");
    }
  }

>>>>>>> Stashed changes
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
        <button className="wallet-adapter-button wallet-adapter-button-trigger vestButton">
          Vest
        </button>
      </form>
      <Claim />
    </div>
  );
};
