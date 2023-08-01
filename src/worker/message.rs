use crate::error::apperror::AppError;

#[derive(Debug)]
pub struct ToApp {
    pub message: ToAppMessage,
}

#[derive(Debug)]
pub struct ToWorker {
    pub message: ToWorkerMessage,
}

#[derive(Debug)]
pub enum ToAppMessage {
    Pong,
    Error(AppError),
}

#[derive(Debug)]
pub enum ToWorkerMessage {
    Ping,
}