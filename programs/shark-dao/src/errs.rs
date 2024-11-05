use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Not init")]
    NotInit,
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
    #[msg("Not authorized")]
    NotAuthorized,
}
