use std::sync::mpsc::{Receiver, Sender};
use std::{
    thread,
    time::{self, Duration},
};

use egui::{Context, Widget};
use log::{error, info};

use crate::{
    error::apperror::AppError,
    worker::message::{ToAppMessage, ToWorkerMessage},
};

use super::message::{ToApp, ToWorker};

pub struct Worker {
    sender: Sender<ToApp>,
    receiver: Receiver<ToWorker>,
    egui_ctx: Context,
}

impl Worker {
    pub fn new(
        sender: Sender<ToApp>,
        receiver: Receiver<ToWorker>,
        egui_ctx: eframe::egui::Context,
    ) -> Self {
        Self {
            sender,
            receiver,
            egui_ctx,
        }
    }

    pub fn init(&mut self) {
        info!("Worker starting up.");

        // Wait for <ToWorker> messages giving work to do.
        // Can send back <ToApp> messages to the GUI.
        // FIXME:
        // In case of a send error we can not "send" an AppError
        // to the app. We just log it. To be improved.
        loop {
            let maybe_message = self.receiver.recv();

            match maybe_message {
                Ok(message) => match message.message {
                    ToWorkerMessage::Ping => {
                        if self
                            .sender
                            .send(ToApp {
                                message: ToAppMessage::Pong,
                            })
                            .is_err()
                        {
                            error!("failed to send ToAppMessage::Pong");
                        }
                    }
                },
                Err(e) => {
                    if self
                        .sender
                        .send(ToApp {
                            message: ToAppMessage::Error(AppError::ChannelReceiveError),
                        })
                        .is_err()
                    {
                        error!("failed to send ToAppMessage::Error for {}", e);
                    };
                }
            }
        }
    }
}
