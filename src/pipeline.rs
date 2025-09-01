use std::sync::Mutex;

use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, WeakUnboundedSender, unbounded_channel};
use take_once::TakeOnce;

#[derive(thiserror::Error, Debug)]
pub enum PipelineError {
    #[error("Pipeline output already taken")]
    OutputAlreadyTaken,
    #[error("Pipeline input already closed")]
    InputAlreadyClosed,
}

#[derive(Debug)]
pub struct Pipeline<T> {
    keep_alive: bool,
    input: Mutex<Sender<T>>,
    output: TakeOnce<UnboundedReceiver<T>>,
}

#[derive(Debug)]
pub enum Sender<T> {
    Strong(UnboundedSender<T>),
    Weak(WeakUnboundedSender<T>),
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        match self {
            Sender::Strong(sender) => Sender::Strong(sender.clone()),
            Sender::Weak(sender) => Sender::Weak(sender.clone()),
        }
    }
}

impl<T> Pipeline<T> {
    pub fn new(keep_alive: bool) -> Self {
        let (input, output) = unbounded_channel();
        Self {
            keep_alive,
            input: Mutex::new(Sender::Strong(input)),
            output: TakeOnce::new_with(output),
        }
    }

    pub fn input(&self) -> Result<UnboundedSender<T>, PipelineError> {
        let mut input = self.input.lock().unwrap();
        let sender = match &*input {
            Sender::Strong(sender) => sender.clone(),
            Sender::Weak(sender) => match sender.upgrade() {
                Some(strong_sender) => strong_sender,
                None => Err(PipelineError::InputAlreadyClosed)?,
            },
        };

        if !self.keep_alive && matches!(*input, Sender::Strong(_)) {
            *input = Sender::Weak(sender.downgrade());
        };

        Ok(sender)
    }

    pub fn output(&self) -> Result<UnboundedReceiver<T>, PipelineError> {
        let output = self.output.take();
        match output {
            Some(receiver) => Ok(receiver),
            None => Err(PipelineError::OutputAlreadyTaken.into()),
        }
    }
}
