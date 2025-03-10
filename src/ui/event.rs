// Event Handling
//
// This file handles terminal events (keyboard, resize, etc.)

use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use crossterm::event::{self, Event as CrosstermEvent, KeyCode, KeyEvent, KeyModifiers};

/// Terminal events
#[derive(Debug, Clone, Copy)]
pub enum Event {
    /// Key press
    Key(KeyEvent),
    /// Terminal resize
    Resize(u16, u16),
    /// Tick event for animations
    Tick,
}

/// Event handler
pub struct EventHandler {
    /// Event sender
    sender: mpsc::Sender<Event>,
    /// Event receiver
    receiver: mpsc::Receiver<Event>,
    /// Event handler thread
    handler: Option<thread::JoinHandle<()>>,
}

impl EventHandler {
    /// Create a new event handler with the given tick rate
    pub fn new(tick_rate: Duration) -> Self {
        let (sender, receiver) = mpsc::channel();
        let handler = {
            let sender = sender.clone();
            thread::spawn(move || {
                let mut last_tick = Instant::now();
                loop {
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(Duration::from_secs(0));

                    if event::poll(timeout).expect("Failed to poll for events") {
                        match event::read().expect("Failed to read event") {
                            CrosstermEvent::Key(key) => {
                                if key.code == KeyCode::Char('c')
                                    && key.modifiers == KeyModifiers::CONTROL
                                {
                                    // Exit on Ctrl+C
                                    break;
                                }
                                sender.send(Event::Key(key)).expect("Failed to send event");
                            }
                            CrosstermEvent::Resize(width, height) => {
                                sender
                                    .send(Event::Resize(width, height))
                                    .expect("Failed to send event");
                            }
                            _ => {}
                        }
                    }

                    if last_tick.elapsed() >= tick_rate {
                        sender.send(Event::Tick).expect("Failed to send tick event");
                        last_tick = Instant::now();
                    }
                }
            })
        };

        Self {
            sender,
            receiver,
            handler: Some(handler),
        }
    }

    /// Receive the next event
    pub fn next(&self) -> Result<Event, mpsc::RecvError> {
        self.receiver.recv()
    }
}

impl Drop for EventHandler {
    fn drop(&mut self) {
        if let Some(handler) = self.handler.take() {
            handler.join().expect("Failed to join event handler thread");
        }
    }
}
