import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Vesting } from "../target/types/vesting";
import * as assert from "assert";
import * as bs58 from "bs58";
import { TOKEN_PROGRAM_ID } from "@project-serum/anchor/dist/cjs/utils/token";
import {
  LAMPORTS_PER_SOL,
  PublicKey,
  Transaction,
  SYSVAR_RENT_PUBKEY,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import {
  createMint,
  createAssociatedTokenAccount,
  mintTo,
} from "@solana/spl-token";

describe("vesting", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Vesting as Program<Vesting>;

  const connection = anchor.getProvider().connection;

  it("#01 can make a vestment", async () => {
    // Before sending the transaction to the blockchain.
    const vestor = anchor.web3.Keypair.generate();
    const beneficiary = anchor.web3.Keypair.generate();

    const tx1 = await connection.requestAirdrop(
      vestor.publicKey,
      LAMPORTS_PER_SOL
    );
    const tx2 = await connection.requestAirdrop(
      beneficiary.publicKey,
      LAMPORTS_PER_SOL
    );

    await connection.confirmTransaction(tx1);
    await connection.confirmTransaction(tx2);

    const tokenMint = await createMint(
      connection,
      vestor,
      vestor.publicKey,
      null,
      0
    );

    const vestorTokenAccount = await createAssociatedTokenAccount(
      connection,
      vestor,
      tokenMint,
      vestor.publicKey
    );

    const beneficiaryTokenAccount = await createAssociatedTokenAccount(
      connection,
      beneficiary,
      tokenMint,
      beneficiary.publicKey
    );

    await mintTo(
      connection,
      vestor,
      tokenMint,
      vestorTokenAccount,
      vestor,
      1000
    );

    //console.log(await connection.getTokenAccountBalance(vestorTokenAccount));

    const [vestedTokens] = await PublicKey.findProgramAddress(
      [Buffer.from("vested-tokens"), beneficiary.publicKey.toBuffer()],
      program.programId
    );

    const [vestment, vestmentBump] = await PublicKey.findProgramAddress(
      [Buffer.from("vestment"), vestedTokens.toBuffer()],
      program.programId
    );

    await program.rpc.makeVestment(new anchor.BN(100), 15, 15, 4, {
      accounts: {
        vestment: vestment,
        vestor: vestor.publicKey,
        vestorTokenAccount: vestorTokenAccount,
        beneficiary: beneficiary.publicKey,
        vestedTokens: vestedTokens,
        vestedTokensMint: tokenMint,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [vestor],
    });

    //console.log(await program.account.vestment.fetch(vestment));
    console.log(await connection.getTokenAccountBalance(vestedTokens));

    await program.rpc.claimVestment({
      accounts: {
        vestment: vestment,
        beneficiary: beneficiary.publicKey,
        beneficiaryTokenAccount: beneficiaryTokenAccount,
        vestedTokens: vestedTokens,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [beneficiary],
    });

    console.log(
      await connection.getTokenAccountBalance(beneficiaryTokenAccount)
    );

    console.log(await connection.getTokenAccountBalance(vestedTokens));
  });
});
