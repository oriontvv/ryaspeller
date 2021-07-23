#[derive(Debug, Serialize, Deserialize)]
pub struct SpellerError {}

#[derive(Debug, Error)]
pub enum SpellerError {
    #[error("bad api respose")]
    ResponseError,

    #[error("http error")]
    HttpError(reqwest::Error),
}
