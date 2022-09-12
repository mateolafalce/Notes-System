import * as anchor from "@project-serum/anchor";
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
export const wallet = provider.wallet as anchor.Wallet;
export const AccountPk = new anchor.web3.PublicKey(
    "HjLsDdMNLPtAfNSwctqQYnQi6KZ4rNRf1ymrFwKeo5tp"
);
export const CreatorPublicKey = new anchor.web3.PublicKey(
    "HiHyNHnwSzFprUdMERFbUJNTcuoD375oE6qDfuytmUYL"
);