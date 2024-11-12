use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Not init")]
    NotInit,
    #[msg("Invalid parameter")]
    InvalidParameter,
    #[msg("Repeated init")]
    RepeatedInit,
    #[msg("Not approved")]
    NotApproved,
    #[msg("Not support token_2022 mint extension")]
    NotSupportMint,
    #[msg("Missing tickarray bitmap extension account")]
    MissingTickArrayBitmapExtensionAccount,
    #[msg("Insufficient balance")]
    InsufficientBalance,
    #[msg("Insufficient mint balance")]
    InsufficientMintBalance,
    #[msg("Insufficient collection mint balance")]
    InsufficientCollectionMintBalance,
    #[msg("Not authorized")]
    NotAuthorized,
    #[msg("Time over")]
    TimeOver,
    #[msg("Time over stm")]
    TimeOverStm,
    #[msg("Repeated withdraw")]
    RepeatedWithdraw,
}
