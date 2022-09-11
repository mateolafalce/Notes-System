import * as anchor from "@project-serum/anchor";
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
export const wallet = provider.wallet as anchor.Wallet;

export const AccountPk = new anchor.web3.PublicKey(
    "zsxpPUaJdScwXWa3ETcuRwxP8Bwm7oWaosfZdZRywmn"
);
export const CreatorPublicKey = new anchor.web3.PublicKey(
    "HiHyNHnwSzFprUdMERFbUJNTcuoD375oE6qDfuytmUYL"
);
export const Owner1 = new anchor.web3.PublicKey(
    "NqXx91Lk2qn9V1W3kEvBVmp1fzXLEGTSGkd9yungtk8"
);
export const Owner2 = new anchor.web3.PublicKey(
    "NqXx91Lk2qn9V1W3kEvBVmp1fzXLEGTSGkd9yungtk8"
);
export const Owner3 = new anchor.web3.PublicKey(
    "NqXx91Lk2qn9V1W3kEvBVmp1fzXLEGTSGkd9yungtk8"
);
export const Owner4 = new anchor.web3.PublicKey(
    "NqXx91Lk2qn9V1W3kEvBVmp1fzXLEGTSGkd9yungtk8"
);