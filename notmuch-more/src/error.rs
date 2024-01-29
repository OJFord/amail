use thiserror::Error;

#[derive(Debug, Error)]
pub enum NotmuchMoreError {
    #[error(transparent)]
    Infallible(#[from] std::convert::Infallible),
    #[error(transparent)]
    IoError {
        #[from]
        source: std::io::Error,
    },
    #[error(transparent)]
    LettreAddressError(#[from] lettre::address::AddressError),
    #[error(transparent)]
    LettreError(#[from] lettre::error::Error),
    #[error(transparent)]
    LettreSmtpError(#[from] lettre::transport::smtp::Error),
    #[error(transparent)]
    NotMuchError(#[from] notmuch::Error),
    #[error(transparent)]
    ParseError(#[from] mailparse::MailParseError),
    #[error(transparent)]
    MimeError(#[from] email::results::ParsingError),
    #[error(transparent)]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
