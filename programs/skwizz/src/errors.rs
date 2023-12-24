use anchor_lang::error_code;

#[error_code]
pub enum ProgramError {
    #[msg("BP must be less than or equal to 10000")]
    InvalidBasisPoints,
    #[msg("UninitializedAccount")]
    UninitializedAccount,
    #[msg("PublicKeyMismatch")]
    PublicKeyMismatch,
    #[msg("IncorrectOwner")]
    IncorrectOwner,
    #[msg("You are only allowed to place one bet at a time")]
    MultipleBetsNotAllowed,
    #[msg("Bet amounts must be greater than 0")]
    InvalidBetAmount,
    #[msg("Number of flips must equal 1")]
    InvalidNumFlips,
    #[msg("Invalid value for bets (may not match on-chain data)")]
    InvalidBets,
    #[msg("Invalid auction house authority")]
    InvalidAuctionHouseAuthority,
    #[msg("Account is already initialized")]
    AlreadyInitialized,
}
