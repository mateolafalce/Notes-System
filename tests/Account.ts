import * as anchor from "@project-serum/anchor";
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
export const wallet = provider.wallet as anchor.Wallet;

export const AccountPk = new anchor.web3.PublicKey(
    "DupSpXvCQ3yANQzxVLFPbvZFiq3A4ue8HQvTWyybybh2"
);

export const PDAWinner = new anchor.web3.PublicKey(
    "9WT3UnC2m98wgJm7TmPsfbcoN98CaNfEDmghzsSkdX3B"
);
export const Winner = new anchor.web3.PublicKey(
    "Ciz4goiuoUCDcLea6fEJGhVpc8MGCeHDnczDm44Nqkbq"
);
export const CreatorPublicKey = new anchor.web3.PublicKey(
    "HiHyNHnwSzFprUdMERFbUJNTcuoD375oE6qDfuytmUYL"
);