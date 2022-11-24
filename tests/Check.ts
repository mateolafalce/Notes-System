import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Solotery } from "/mnt/c/Users/Mateo/solotery/target/types/solotery";
import { AccountPk } from "/mnt/c/Users/Mateo/solotery/tests/Account";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";

describe("SOLotery", () => {
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
    const program = anchor.workspace.Solotery as Program<Solotery>;
    it("Check the state of the lotery", async () => {
        const Account = await program.account.soLotery.fetch(AccountPk);
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
      console.log("Total tickets 2: ", Account.players2.length.toString());
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