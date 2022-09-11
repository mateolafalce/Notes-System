import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Solotery } from "/mnt/c/Users/Mateo/solotery/target/types/solotery";
import { AccountPk } from "/mnt/c/Users/Mateo/solotery/tests/Account";
import { wallet } from "/mnt/c/Users/Mateo/solotery/tests/Account";
import { Connection, LAMPORTS_PER_SOL } from "@solana/web3.js";

function timeConverter(UNIX_timestamp){
  var a = new Date(UNIX_timestamp * 1000);
  var months = ['Jan','Feb','Mar','Apr','May','Jun','Jul','Aug','Sep','Oct','Nov','Dec'];
  var year = a.getFullYear();
  var month = months[a.getMonth()];
  var date = a.getDate();
  var hour = a.getHours();
  var min = a.getMinutes();
  var sec = a.getSeconds();
  var time = date + ' ' + month + ' ' + year + ' ' + hour + ':' + min + ':' + sec ;
  return time;
}

describe("Ticket", async () => {
  const program = anchor.workspace.Solotery as Program<Solotery>;
  it('Ticket ...', async () => {
    const tx = await program.methods.ticket().accounts({
        solotery: AccountPk,
        from: wallet.publicKey,
        stake: AccountPk,
        systemProgram: anchor.web3.SystemProgram.programId,
      }).rpc();
      const Account = await program.account.soLotery.fetch(AccountPk);
      const connection = new Connection("https://api.devnet.solana.com");
      let balance = await connection.getBalance(AccountPk);
      console.log("-----------------------------------------------------")
      console.log("Tx: ",tx);
      console.log("-----------------------------------------------------")
      console.log("-----------------------------------------------------")
      console.log("Authority: ", Account.authority.toBase58());
      console.log("-----------------------------------------------------")
      console.log("PDA: ", AccountPk.toString());
      console.log("-----------------------------------------------------")
      console.log("Total stake: ", ((balance / LAMPORTS_PER_SOL)- 0.06830544).toString(), "SOL");
      console.log("-----------------------------------------------------")
      console.log("Total tickets: ", (Account.players.length).toString());
      console.log("-----------------------------------------------------")
      console.log("Timestamp Lock: ", timeConverter(Account.secureCheck).toString());
      console.log("-----------------------------------------------------")
      console.log("Original bump: ", Account.bumpOriginal.toString());
      console.log("-----------------------------------------------------")
      console.log("Winner state: ", Account.chooseWinnerOnlyOneTime.toString());
      console.log("-----------------------------------------------------")
      console.log("PDA Winner Pubkey: ", Account.winnerPublickey.toBase58());
      console.log("-----------------------------------------------------")
      console.log("Players: ", Account.players.toString());
      console.log("-----------------------------------------------------")
  });
});