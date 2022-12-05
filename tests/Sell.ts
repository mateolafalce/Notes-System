import * as anchor from "@project-serum/anchor";
import { Commonmarket } from "../target/types/commonmarket";
import { MainPDA, wallet } from "../tests/Account";
import { PublicKey, LAMPORTS_PER_SOL } from '@solana/web3.js';

describe("Initialize a sell", async () => {
  const program = anchor.workspace.Commonmarket as anchor.Program<Commonmarket>;
  it('Sell ...', async () => {
    const PDA = await program.account.mainAccount.fetch(MainPDA);
    const [SellPDA, _bump] = await PublicKey
    .findProgramAddress(
      [
        new anchor.BN(PDA.offers.toNumber()).toArrayLike(Buffer, "be", 8)
      ],
      program.programId
    )
    const tx = await program.methods.sell(
      "ñ",
      "ñ",
      new anchor.BN(1),
      new anchor.BN(5000),
      "ñ"
    )
    .accounts({
        mainAccount: MainPDA,
        sell: SellPDA,
        user: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      }).rpc();
      const Account = await program.account.sell.fetch(SellPDA);
      console.log("-----------------------------------------------------")
      console.log("Tx: ",tx);
      console.log("-----------------------------------------------------")
      console.log("PDA: ", SellPDA.toString());
      console.log("-----------------------------------------------------")
      console.log("Original bump: ", Account.bumpOriginal.toString());
      console.log("-----------------------------------------------------")
      console.log("Product name: ", Account.product.toString());
      console.log("-----------------------------------------------------")
      console.log("Description: ", Account.description.toString());
      console.log("-----------------------------------------------------")
      console.log("Supply: ", Account.supply.toString());
      console.log("-----------------------------------------------------")
      console.log("Price: ", (Account.price.toNumber() / LAMPORTS_PER_SOL).toString(), "SOL");
      console.log("-----------------------------------------------------")
      console.log("IPFS url: ", Account.ipfsUrl.toString());
      console.log("-----------------------------------------------------")
      console.log("Offer ID: ", PDA.offers.toString());
      console.log("-----------------------------------------------------")
  });
});