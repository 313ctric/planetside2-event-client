use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_bool_from_anything;
use serde_json::Value;

use crate::data::World;

pub mod events;
use events::{EventPayload, EventType};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Service {
    Event,
    Push,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum EventRequestAction {
    Echo,
    Subscribe,
    ClearSubscribe,
    Help,
    RecentCharacterIds,
    RecentCharacterIdsCount,
}

#[derive(Serialize)]
pub struct EventRequest {
    /// the service you are using, always "event"
    pub service: Service,
    // the action you are requesting to do
    pub action: EventRequestAction,

    /// used for the clearSubscribe request to specify to clear all subscriptions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_characters: Option<bool>,

    /// used to send the data to echo for an echo request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Value>,

    /// the numerical ids of the characters you are subscribing to events for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub characters: Option<Vec<String>>,

    /// the numerical ids of all the worlds you are subscribing to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worlds: Option<Vec<World>>,

    /// the names of the events you are subscribing to
    #[serde(skip_serializing_if = "Option::is_none", rename = "eventNames")]
    pub event_names: Option<Vec<EventType>>,
}
impl Default for EventRequest {
    fn default() -> Self {
        EventRequest {
            service: Service::Event,
            action: EventRequestAction::Help,

            all: Option::None,
            list_characters: Option::None,

            payload: Option::None,
            characters: Option::None,
            worlds: Option::None,
            event_names: Option::None,
        }
    }
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionInfoEventResponse {
    character_count: Option<i32>,
    characters: Option<Vec<String>>,
    event_names: Vec<EventType>,
    logical_and_characters_with_worlds: bool,
    worlds: Vec<String>, // TODO: fix world deserializing
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum InternallyTaggedEventResponse {
    Heartbeat {
        service: Service,
        online: Value,
    },
    /// a message with data about an event you have subscribed to
    ServiceMessage {
        service: Service,
        payload: EventPayload,
    },
    ServiceStateChanged {
        service: Service,
        #[serde(deserialize_with = "deserialize_bool_from_anything")]
        online: bool,
        detail: String,
    },
    ConnectionStateChanged {
        service: Service,
        #[serde(deserialize_with = "deserialize_bool_from_anything")]
        connected: bool,
    },
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ExternallyTaggedEventResponse {
    Subscription(SubscriptionInfoEventResponse),
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", untagged)]
pub enum UntaggedEventResponse {
    HelpResponse {
        #[serde(rename = "example event service message payloads")]
        payload_examples: Value,
        #[serde(rename = "example messages to event service")]
        request_examples: Value,
    },
    HelpInfo {
        #[serde(rename = "send this for help")]
        help_payload: Value,
    },
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum EventResponse {
    InternallyTagged(InternallyTaggedEventResponse),
    ExternallyTagged(ExternallyTaggedEventResponse),
    Untagged(UntaggedEventResponse),
}
