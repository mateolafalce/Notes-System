use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("The text size you want to enter is too long")]LenghtError,
    #[msg("The price can never be 0")]PriceError,
    #[msg("u16 = 0 - 65535")]U16Error,
    #[msg("Supply is equal to zero")]SupplyError,
    #[msg("You are demanding more than what exists")]OfferError,
    #[msg("The signing pubkey is different from the pubkey of the CommonMarket account")]PubkeyError,
}