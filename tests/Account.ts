import * as anchor from "@project-serum/anchor";
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
export const wallet = provider.wallet as anchor.Wallet;

export const MainPDA = new anchor.web3.PublicKey(
    "7SBishkzH9QMKuesquRms9xXuE8vzD4s3saw34qLUk3W"
);
export const Offer = new anchor.web3.PublicKey(
    "5oV4nTsni89mHnqDFqp89yiL39NKKvrmpKmk3eNzZ6F9"
);