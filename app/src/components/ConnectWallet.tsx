import { useState } from "react";
import React from 'react';
import "./formInput.css";
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui'

const ConnectWallet = () => {
  return (
    <div className="connectWallet">
      <WalletMultiButton/>
    </div>
  )
}

export default ConnectWallet;