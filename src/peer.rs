use tracing::{debug, info, instrument};

use crate::config::Config;
use crate::event::Event;
use crate::event_queue::EventQueue;
use crate::state::State;
use crate::connection::Connection;

#[derive(Debug)]
pub struct Peer {
    state: State,
    event_queue: EventQueue,
    tcp_connection: Option<Connection>,
    config: Config,
}

impl Peer {
    pub fn new (config: Config) -> Self {
        let state = State::Idle;
        let event_queue = EventQueue::new();
        Self {
            state,
            event_queue,
            tcp_connection: None::<Connection>,
            config,
        }
    }

    #[instrument]
    pub fn start(&mut self) {
        info!("peer is started.");

        self.event_queue.enqueue(Event::ManualStart);
    }

    #[instrument]
    pub async fn next(&mut self) {
        if let Some(event) = self.event_queue.dequeue() {
            info!("event is occured, event={:?}.", event);
            self.handle_event(event).await;
        }
    }

    async fn handle_event(&mut self, event: Event) {
        match &self.state {
            State::Idle => {
                match event {
                    Event::ManualStart => {
                        self.tcp_connection = Connection::connect(&self.config).await.ok();
                        if self.tcp_connection.is_some() {
                            self.event_queue.enqueue(Event::TcpConnectionEstablished);
                        } else {
                            panic!("TCP connection is failed. {:?}", self.config);
                        }
                        self.state = State::Connect;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;
    #[tokio::test]
    async fn peer_can_transition_to_connect_state() {
        let config: Config = "64512 127.0.0.1 65413 127.0.0.2 active".parse().unwrap();
        let mut peer = Peer::new(config);
        peer.start();

        tokio::spawn(async move {
            let remote_config = "64513 127.0.0.2 65412 127.0.0.1 passive".parse().unwrap();
            let mut remote_peer = Peer::new(remote_config);
            remote_peer.start();
            remote_peer.next().await;
        });

        tokio::time::sleep(Duration::from_secs(1)).await;
        peer.next().await;
        assert_eq!(peer.state, State::Connect);
    }
}
