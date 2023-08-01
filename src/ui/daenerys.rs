use std::thread;
use std::sync::mpsc::{Sender, Receiver, self};

use eframe::CreationContext;

use crate::error::apperror::AppError;
use crate::ui::pages::main;
use crate::worker::{message::{ToWorker, ToApp, ToAppMessage}, worker::Worker};

// Applications pages.
#[derive(Default)]
enum Page {
    #[default]
    Main,
}

#[derive(Default)]
pub struct DaenerysApp {
    // Current active page.
    page: Page,

    // Channels for communication beetween
    // application (GUI) and worker.
    pub sender: Option<Sender<ToWorker>>,
    receiver: Option<Receiver<ToApp>>,

    // Current error if one.
    pub current_error: Option<AppError>,
    // Current info if one.
    pub current_info: Option<String>,

     // Icons.
    pub storm_logo: Option<egui_extras::RetainedImage>,
}

impl DaenerysApp {

    pub fn new(cc: &CreationContext) -> Self {

        // Create application.
        let mut app = DaenerysApp { 
            storm_logo: Some(egui_extras::RetainedImage::from_svg_bytes(
            "storm.svg",
            include_bytes!("media/storm.svg")).unwrap()),
            ..Default::default()
        };

        // Create channels.
        let (app_tx, app_rx) = mpsc::channel();
        let (worker_tx, worker_rx) = mpsc::channel();

        let context = cc.egui_ctx.clone();

        tracing::info!("Spawning new worker.");
        
        // Spawn a thread with a new worker.
        thread::spawn(move || {
            Worker::new(worker_tx, app_rx, context).init();
        });

        tracing::info!("New worker spawned.");

        app.sender = Some(app_tx);
        app.receiver = Some(worker_rx);

        app

    }

}

impl eframe::App for DaenerysApp {

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        // Check for ToApp messages.
        if let Some(receiver) = &self.receiver {

            let maybe_message = match receiver.try_recv() {
                Ok(message) => Some(message),
                Err(e) => match e {
                    mpsc::TryRecvError::Empty => None,
                    mpsc::TryRecvError::Disconnected => {
                        self.current_error = Some(AppError::ChannelClosed);
                        None
                    },
                },
            };

            if let Some(message) = maybe_message { 
                println!("received = {:?}", message);
                match message.message {
                    ToAppMessage::Pong => self.current_info = Some("pong".to_string()),
                    ToAppMessage::Error(e) => self.current_error = Some(e), //FIXME: handle fatal errors
                }
            }

        }

        // Render page.
        match self.page {
            Page::Main => main::ui::update(self, ctx, frame),
        }
    }

}