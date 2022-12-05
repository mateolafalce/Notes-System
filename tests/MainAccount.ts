import * as anchor from "@project-serum/anchor";
import { Commonmarket } from "../target/types/commonmarket";
import { wallet } from "./Account";
import { PublicKey } from '@solana/web3.js';

describe("Creating PDA", () => {
  const program = anchor.workspace.Commonmarket as anchor.Program<Commonmarket>;
    it("Created", async () => {
      const [Data, _bump] = await PublicKey
      .findProgramAddress(
        [
          anchor.utils.bytes.utf8.encode("Main Account"),
        ],
        program.programId
      )
      const tx = await program.methods.mainAccount()
      .accounts({
          mainAccount: Data,
          user: wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        }).rpc();
      console.log("---------------------------------------------")
      console.log("PDA: ", Data.toBase58());
      console.log("---------------------------------------------") 
      console.log("Your transaction signature", tx);
      console.log("---------------------------------------------") 
    });
})