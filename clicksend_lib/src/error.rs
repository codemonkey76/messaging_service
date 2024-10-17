pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    ClickSendApiError(String),
}
