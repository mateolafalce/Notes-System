import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Solotery } from "/mnt/c/Users/Mateo/solotery/target/types/solotery";
import { AccountPk } from "/mnt/c/Users/Mateo/solotery/tests/Account";
import { wallet } from "/mnt/c/Users/Mateo/solotery/tests/Account";

describe("The winner is...", () => {
  const program = anchor.workspace.Solotery as Program<Solotery>;
  it("Winner", async () => { 
    const tx = await program.methods.chooseWinner()
    .accounts({
        solotery: AccountPk,
        soloteryAuthority: wallet.publicKey,
    }).rpc();
    const PDAAccount = await program.account.soLotery.fetch(AccountPk);
    console.log("---------------------------------------------")
    console.log("PDA Winner: ", PDAAccount.winnerPublickey.toBase58())
    console.log("---------------------------------------------")
    console.log("Your transaction signature", tx);
    });
});