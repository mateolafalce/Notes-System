import * as anchor from "@project-serum/anchor";
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
export const wallet = provider.wallet as anchor.Wallet;

export const AccountPk = new anchor.web3.PublicKey(
    "5dLQavaewtQCTWJvPW6giWsYgZACLrNknBKt29n8563J"
);