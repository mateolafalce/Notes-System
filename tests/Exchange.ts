import * as anchor from "@project-serum/anchor";
import { Commonmarket } from "../target/types/commonmarket";
import { wallet, Offer, MainPDA } from "./Account";

describe("Exchange", () => {
  const program = anchor.workspace.Commonmarket as anchor.Program<Commonmarket>;
    it("...", async () => {
      let amount = new anchor.BN(1);
      const tx = await program.methods.exchange(
        amount
      )
      .accounts({
          mainAccount: MainPDA,
          offer: Offer,
          buyer: wallet.publicKey,
          seller: wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        }).rpc();
    //const AccountData = await program.account.accountModel.fetch(Account);
    console.log("---------------------------------------------") 
    console.log("Your transaction signature", tx);
    console.log("---------------------------------------------")
    console.log("You buy: ", amount.toNumber().toString(), "of",Offer.toBase58().toString())
    console.log("---------------------------------------------")
    });
})