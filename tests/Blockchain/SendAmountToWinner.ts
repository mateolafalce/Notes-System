import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Solotery } from "/mnt/c/Users/Mateo/solotery/target/types/solotery";
import { AccountPk } from "/mnt/c/Users/Mateo/solotery/tests/Account";
import { wallet } from "/mnt/c/Users/Mateo/solotery/tests/Account";
import { CreatorPublicKey } from "/mnt/c/Users/Mateo/solotery/tests/Account";

describe("Send amount to winner", async () => {
  const program = anchor.workspace.Solotery as Program<Solotery>;
  it("Winner", async () => {
    const PDAccount = await program.account.soLotery.fetch(AccountPk);
    const tx = await program.methods.sendAmountToWinner()
    .accounts({
        solotery: AccountPk,
        creatorPublickey: CreatorPublicKey,
        winnerPublickey: PDAccount.winnerPublickey,
        soloteryAuthority: wallet.publicKey,
      }).rpc();
    console.log("---------------------------------------------")
    console.log("Successfully sent: ", tx);
    console.log("---------------------------------------------")
  });
});