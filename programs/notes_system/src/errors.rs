use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("The text size you want to enter is too long")]LenghtError,
    #[msg("Error of authority you are not the school")]AuthorityError,
}