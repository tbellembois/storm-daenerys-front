use crate::error::apperror::AppError;

#[derive(Debug)]
pub struct ToApp {
    pub message: ToAppMessage,
}

#[derive(Debug)]
pub struct ToWorker {
    pub message: ToWorkerMessage,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum ToAppMessage {
    Pong,
    Error(AppError),
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum ToWorkerMessage {
    Ping,
}
