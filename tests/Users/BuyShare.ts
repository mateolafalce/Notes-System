import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Solotery } from "/mnt/c/Users/Mateo/solotery/target/types/solotery";
import { AccountPk } from "/mnt/c/Users/Mateo/solotery/tests/Account";
import { CreatorPublicKey } from "/mnt/c/Users/Mateo/solotery/tests/Account";
import { wallet } from "/mnt/c/Users/Mateo/solotery/tests/Account";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";

describe("Buy Share", async () => {
  const program = anchor.workspace.Solotery as Program<Solotery>;
  it('Buying ...', async () => {
    const tx = await program.methods.buyShare(
        4,
        new anchor.BN(6000)
    ).accounts({
        solotery: AccountPk,
        from: wallet.publicKey,
        creator: CreatorPublicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      }).rpc();
      const Account = await program.account.soLotery.fetch(AccountPk);
      console.log("-----------------------------------------------------")
      console.log("Tx: ",tx);
      console.log("-----------------------------------------------------")
      console.log("Authority 1: ", Account.owner1.toString());
      console.log("-----------------------------------------------------")
      console.log("Best Proposal 1: ", ((Account.bestProposal1 / LAMPORTS_PER_SOL) .toString()), "SOL");
      console.log("-----------------------------------------------------")
      console.log("Authority 2: ", Account.owner2.toString());
      console.log("-----------------------------------------------------")
      console.log("Best Proposal 2: ", ((Account.bestProposal2 / LAMPORTS_PER_SOL) .toString()), "SOL");
      console.log("-----------------------------------------------------")
      console.log("Authority 3: ", Account.owner3.toString());
      console.log("-----------------------------------------------------")
      console.log("Best Proposal 3: ", ((Account.bestProposal3 / LAMPORTS_PER_SOL) .toString()), "SOL");
      console.log("-----------------------------------------------------")
      console.log("Authority 4: ", Account.owner4.toString());
      console.log("-----------------------------------------------------")
      console.log("Best Proposal 4: ", ((Account.bestProposal4 / LAMPORTS_PER_SOL) .toString()), "SOL");
      console.log("-----------------------------------------------------")
  });
});