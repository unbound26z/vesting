import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Vesting } from "../target/types/vesting";

describe("vesting", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Vesting as Program<Vesting>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
