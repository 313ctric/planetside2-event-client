use planetside2_event_client::{
    census::CensusClient,
    data::{Character, Environment, World},
    event::{
        message::{EventRequest, EventRequestAction, Service},
        CallbackHolder, EventPayload, EventStreamingClient, EventType,
    },
};

use tokio::{self, sync::mpsc};

const SERVICE_ID: &str = "example";

// fill in a username here
const USERNAME: &str = "something";
const ENVIRONMENT: Environment = Environment::PC;

async fn handle_event_message(
    event: EventPayload,
    character: Character,
    census_client: &mut CensusClient,
) {
    println!("Received event message: {:?}", event);
    match event {
        EventPayload::Death(d) => {
            if d.character_id == character {
                if let Some(other) = d.attacker_character_id.info(census_client).await {
                    println!("You just got killed by {}", other.name.first);
                } else {
                    println!("You just got killed");
                }
            } else if d.attacker_character_id == character {
                if let Some(other) = d.character_id.info(census_client).await {
                    println!("You just killed {}", other.name.first);
                } else {
                    println!("You just killed someone");
                }
            }
            if d.is_headshot {
                println!("Kill was a headshot");
            }
        }
        EventPayload::GainExperience(e) => {
            if e.character_id == character {
                // you earned the experience
                println!("You just earned experience");
                if let Some(info) = e.experience_id.info(census_client).await {
                    if info.is_squad() {
                        println!("You just earned squad experience");
                    }
                    if info.is_revive() {
                        println!("You just earned revive experience");
                    }
                } else {
                    if e.experience_id.0 == 1 {
                        println!("You just earned kill experience");
                    }
                }
            }
            if e.other_id == character {
                if let Some(info) = e.experience_id.info(census_client).await {
                    if info.is_revive() {
                        println!("You just got revived");
                    }
                }
            }
        }
        _ => (), //ignore all other events
    }
}

async fn listen_to_events(
    mut chan_receive: mpsc::UnboundedReceiver<EventPayload>,
    mut census_client: CensusClient,
    character: Character,
) {
    while let Some(e) = chan_receive.recv().await {
        handle_event_message(e, character, &mut census_client).await;
    }
}

#[tokio::main]
async fn main() {
    // get the id of a character from their name
    let mut census_client = CensusClient::new(SERVICE_ID.to_owned());
    let character = Character::from_name(USERNAME.to_owned(), &mut census_client)
        .await
        .unwrap();

    let events = vec![EventType::GainExperience, EventType::Death];
    let worlds = vec![World::Miller];
    let characters = vec![character];
    let character_ids: Vec<String> = characters.iter().map(|c| c.to_string()).collect();

    // construct a request to send to the server
    let sub_request = EventRequest {
        action: EventRequestAction::Subscribe,
        service: Service::Event,
        characters: Some(character_ids),
        worlds: Some(worlds),
        event_names: Some(events),
        all: None,
        list_characters: None,
        payload: None,
    };

    // create a channel to pass received events through
    let (chan_send, chan_receive) = tokio::sync::mpsc::unbounded_channel::<EventPayload>();

    // add our callback to an object to call them
    let mut callbacks = CallbackHolder::new();
    callbacks.register_event_listener(move |e| chan_send.send(e.clone()).unwrap());

    // create a client and connect it to the server
    let mut event_client = EventStreamingClient::new(ENVIRONMENT, SERVICE_ID.to_owned(), callbacks);
    event_client.connect().await.unwrap();

    // send thee request constructed earlier
    event_client.send_request(sub_request).await.unwrap();

    // run the client and the channel listener
    tokio::join!(
        event_client.run(),
        listen_to_events(chan_receive, census_client, character)
    );
}
