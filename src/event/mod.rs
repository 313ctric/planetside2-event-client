const EVENT_BASE_URL: &str =
    "wss://push.planetside2.com/streaming?environment={env}&service-id=s:{service_id}";

use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream,
};

use serde_json::Value;

use crate::data::Environment;

use std::error::Error;

pub mod message;
use message::{
    EventRequest, EventResponse, ExternallyTaggedEventResponse, InternallyTaggedEventResponse,
};

pub use message::events::{EventPayload, EventType};

pub struct CallbackHolder {
    ps2_callbacks: Vec<Box<dyn FnMut(&EventResponse)>>,
    all_callbacks: Vec<Box<dyn FnMut(&Value)>>,
    event_callbacks: Vec<Box<dyn FnMut(&EventPayload)>>,
}
impl CallbackHolder {
    pub fn new() -> Self {
        Self {
            ps2_callbacks: vec![],
            all_callbacks: vec![],
            event_callbacks: vec![],
        }
    }

    /// add a listener that fires on all responses from the server that parse as valid PS2 responses
    pub fn register_ps2_response_listener<F>(&mut self, callback: F)
    where
        F: 'static + FnMut(&EventResponse),
    {
        self.ps2_callbacks.push(Box::new(callback));
    }
    /// add a listener that fires on all responses that parse as valid event payloads
    pub fn register_event_listener<F>(&mut self, callback: F)
    where
        F: 'static + FnMut(&EventPayload),
    {
        self.event_callbacks.push(Box::new(callback));
    }
    /// add a listener that fires on all responses from the server that are valid JSON, even if they cannot be parsed as valid messages for the PS2 api
    pub fn register_all_response_listener<F>(&mut self, callback: F)
    where
        F: 'static + FnMut(&Value),
    {
        self.all_callbacks.push(Box::new(callback));
    }

    fn call_ps2_response_callbacks(
        callbacks: &mut Vec<Box<dyn FnMut(&EventResponse)>>,
        response: &EventResponse,
    ) {
        for func in callbacks.iter_mut() {
            func(&response);
        }
    }
    fn call_event_callbacks(
        callbacks: &mut Vec<Box<dyn FnMut(&EventPayload)>>,
        response: &EventPayload,
    ) {
        for func in callbacks.iter_mut() {
            func(&response);
        }
    }
    fn call_all_reponse_callbacks(callbacks: &mut Vec<Box<dyn FnMut(&Value)>>, response: &Value) {
        for func in callbacks.iter_mut() {
            func(&response);
        }
    }

    /// parse an incoming message and call the relevant callback functions
    fn handle_message(&mut self, message: &[u8]) {
        if self.all_callbacks.len() > 0 {
            let basic_parse: serde_json::Value = serde_json::from_slice(&message).unwrap();
            Self::call_all_reponse_callbacks(&mut self.all_callbacks, &basic_parse);
        }

        match serde_json::from_slice::<EventResponse>(message) {
            Ok(resp) => {
                Self::call_ps2_response_callbacks(&mut self.ps2_callbacks, &resp);
                if let EventResponse::ExternallyTagged(
                    ExternallyTaggedEventResponse::Subscription(_s),
                ) = &resp
                {
                    // TODO
                    // we have a subscription callback, we could update to say we have seen the subscription returned, but like nah
                }
                if let EventResponse::InternallyTagged(
                    InternallyTaggedEventResponse::ServiceMessage {
                        service: _,
                        payload,
                    },
                ) = &resp
                {
                    // event callback
                    Self::call_event_callbacks(&mut self.event_callbacks, payload);
                }
                // we don't care about other message types
                // match resp {
                //     EventResponse::Untagged(UntaggedEventResponse::HelpResponse{..}) => (),
                //     EventResponse::Untagged(UntaggedEventResponse::HelpInfo{..}) => (),
                //     EventResponse::InternallyTagged(InternallyTaggedEventResponse::Heartbeat{..}) => (),
                //     EventResponse::InternallyTagged(InternallyTaggedEventResponse::ServiceStateChanged{..}) => (),
                //     EventResponse::InternallyTagged(InternallyTaggedEventResponse::ConnectionStateChanged{..}) => (),
                // }
            }
            Err(_) => (), // probably an echo response, just ignore it
        }
    }
}

pub struct EventStreamingClient {
    connect_url: String,
    websocket_stream: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,

    callbacks: CallbackHolder,
}
impl EventStreamingClient {
    pub fn new(environment: Environment, service_id: String, callbacks: CallbackHolder) -> Self {
        let url = EVENT_BASE_URL
            .replace("{env}", &environment.to_string())
            .replace("{service_id}", &service_id);

        Self {
            connect_url: url,
            websocket_stream: Option::None,

            callbacks,
        }
    }

    pub async fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        if self.websocket_stream.is_some() {
            return Ok(()); // already connected
        }
        let url = url::Url::parse(&self.connect_url)?;
        let (stream, _) = connect_async(url).await?;
        self.websocket_stream = Option::from(stream);
        Ok(())
    }

    pub async fn send_request(&mut self, request: EventRequest) -> Result<(), Box<dyn Error>> {
        let stream = self.websocket_stream.as_mut().ok_or("planetside2 event client not connected to websocket, make sure to call 'connect' first")?;
        let serialized = serde_json::to_string(&request)?;
        let msg: Message = Message::text(serialized);
        stream.send(msg).await?;
        Ok(())
    }

    pub async fn run(mut self) {
        let stream = match self.websocket_stream.as_mut() {
            Some(s) => s,
            None => {
                return;
            }
        };
        while let Some(msg) = stream.next().await {
            match msg {
                Ok(msg) => {
                    if msg.is_text() {
                        let msg_text = msg.into_data();

                        // sometimes a response contains multiple messages stuck together, split them up and handle each one in order
                        let mut msg_iter = msg_text.iter().enumerate();
                        let mut last = 0;
                        while let Some(position) = msg_iter.position(|(i, &x)| {
                            x == ('}' as u8)
                                && (msg_text.get(i + 1) == Some(&('{' as u8))
                                    || i + 1 == msg_text.len())
                        }) {
                            if let Some(short_msg) = msg_text.get(last..(last + position + 1)) {
                                self.callbacks.handle_message(&short_msg);
                            }
                            last += position + 1;
                        }
                    }
                }
                Err(_) => (), // just ignore the error
            }
        }
    }
}
