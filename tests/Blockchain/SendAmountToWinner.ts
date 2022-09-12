import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Solotery } from "/mnt/c/Users/Mateo/solotery/target/types/solotery";
import { AccountPk } from "/mnt/c/Users/Mateo/solotery/tests/Account";
import { wallet } from "/mnt/c/Users/Mateo/solotery/tests/Account";

describe("Send amount to winner", async () => {
  const program = anchor.workspace.Solotery as Program<Solotery>;
  it("Winner", async () => {
    const PDAccount = await program.account.soLotery.fetch(AccountPk);
    const tx = await program.methods.sendAmountToWinner()
    .accounts({
        solotery: AccountPk,
        owner1: PDAccount.owner1,
        owner2: PDAccount.owner2,
        owner3: PDAccount.owner3,
        owner4: PDAccount.owner4,
        owner5: PDAccount.owner5,
        owner6: PDAccount.owner6,
        owner7: PDAccount.owner7,
        winnerPublickey: PDAccount.winnerPublickey,
        soloteryAuthority: wallet.publicKey,
      }).rpc();
    console.log("---------------------------------------------")
    console.log("Successfully sent: ", tx);
    console.log("---------------------------------------------")
  });
});