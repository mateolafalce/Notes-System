import * as anchor from "@project-serum/anchor";
import { Solotery } from "/mnt/c/Users/Mateo/solotery/target/types/solotery";
import { wallet } from "/mnt/c/Users/Mateo/solotery/tests/Account";
import { PublicKey } from '@solana/web3.js';

describe("Creating PDA", () => {
  const program = anchor.workspace.Solotery as anchor.Program<Solotery>;
    it("Created", async () => {
      const [SOLotery, _bump] = await PublicKey
      .findProgramAddress(
        [
          anchor.utils.bytes.utf8.encode("SOLotery"),
        ],
        program.programId
      )
      const tx = await program.methods.createStake()
      .accounts({
          solotery: SOLotery,
          user: wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        }).rpc();
      console.log("---------------------------------------------")
      console.log("PDA: ", SOLotery.toBase58());
      console.log("---------------------------------------------") 
      console.log("Your transaction signature", tx);
      console.log("---------------------------------------------") 
    });
})