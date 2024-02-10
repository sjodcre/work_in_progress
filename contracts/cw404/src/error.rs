use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Generic Error")]
    Generic {},

    #[error("Not found!")]
    NotFound {},

    #[error("Already Exists!")]
    AlreadyExists {},

    #[error("Invalid recipient!")]
    InvalidRecipient {},

    #[error("Invalid Sender!")]
    InvalidSender {},

    #[error("Unsafe Recipient!")]
    UnsafeRecipient {},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
