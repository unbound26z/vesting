import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Vesting } from "../target/types/vesting";
import * as assert from "assert";
import * as bs58 from "bs58";

describe("vesting", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Vesting as Program<Vesting>;

  it("can make a vestment", async () => {
    // Before sending the transaction to the blockchain.
    const vestment = anchor.web3.Keypair.generate();

    await program.rpc.makeVestment(100, 12, 1,{
      accounts: {
        vestment: vestment.publicKey,
        vestor: program.provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [vestment],
    });

    // Fetch the account details of the created vestment.
    const vestmentAccount = await program.account.vestment.fetch(vestment.publicKey);

    //insure it has the right data
    assert.equal(
      vestmentAccount.vestor.toBase58(),
      program.provider.wallet.publicKey.toBase58()
    );
    assert.equal(vestmentAccount.amount, 100);
    assert.equal(vestmentAccount.cliff, 12);
    assert.equal(vestmentAccount.period, 1);
    assert.ok(vestmentAccount.timestamp);
  });
});
