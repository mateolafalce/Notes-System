import * as anchor from "@project-serum/anchor";
import { Solotery } from "/mnt/c/Users/Mateo/solotery/target/types/solotery";
import { AccountPk, wallet } from "/mnt/c/Users/Mateo/solotery/tests/Account";
import { LAMPORTS_PER_SOL } from "@solana/web3.js"; 

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
  const program = anchor.workspace.Solotery as anchor.Program<Solotery>;
  it('Ticket ...', async () => {

    const Account = await program.account.soLotery.fetch(AccountPk);
    const tx = await program.methods.ticket().accounts({
        solotery: AccountPk,
        from: wallet.publicKey,
        stake: AccountPk,
        winnerPublickey: Account.winnerPublickey,
        systemProgram: anchor.web3.SystemProgram.programId,
      }).rpc();
      console.log("-----------------------------------------------------")
      console.log("Tx: ",tx);
      console.log("-----------------------------------------------------")
      console.log("PDA: ", AccountPk.toString());
      console.log("-----------------------------------------------------")
      console.log("Original bump: ", Account.bumpOriginal.toString());
      console.log("-----------------------------------------------------")
      console.log("Actual Stake: ", Account.playersState);
      console.log("-----------------------------------------------------")
      console.log("Total amount 1: ", (Account.players1.length * 7777777 / LAMPORTS_PER_SOL), "SOL");
      console.log("-----------------------------------------------------")
      console.log("Total tickets 1: ", Account.players1.length.toString());
      console.log("-----------------------------------------------------")
      console.log("Total amount 1: ", (Account.players2.length * 7777777 / LAMPORTS_PER_SOL), "SOL");
      console.log("-----------------------------------------------------")
      console.log("Total tickets 2: ", Account.players2.length);
      console.log("-----------------------------------------------------")
      console.log("Winner Pubkey: ", Account.winnerPublickey.toBase58());
      console.log("-----------------------------------------------------")
      console.log("Timestamp Lock: ", timeConverter(Account.timeCheck).toString());
      console.log("-----------------------------------------------------")
      console.log("Tickets sold: ", Account.ticketsSold.toString());
      console.log("-----------------------------------------------------")
      /*console.log("Players in 1: ", Account.players1);
      console.log("-----------------------------------------------------")
      console.log("Players in 2: ", Account.players2);
      console.log("-----------------------------------------------------")*/
  });
});