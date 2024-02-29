use anchor_lang::error_code;

#[error_code]
pub enum ProgramError {
    #[msg("You are not a holder")]
    NotABagHolder,
    #[msg("You do not have a profit")]
    NoPofit,
    #[msg("You are not an administrator")]
    NotAnAdmin,
    #[msg("It is in initial phase")]
    IsInitialPhase,
    #[msg("It is not in initial phase")]
    IsPostInitialPhase,
    #[msg("You sent less token than required")]
    SentLessToken,
    #[msg("You do not have enough funds")]
    InsufficientBalance,
    #[msg("Already Initialized Account")]
    AlreadyInitialized,
    #[msg("To address doesn't match generated to account")]
    InvalidToAccount,
    #[msg("Signer isn't Owner")]
    NotOwner,
}
