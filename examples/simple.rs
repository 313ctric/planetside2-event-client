use planetside2_event_client::{
    census::CensusClient,
    data::{Character, Environment, World},
    event::{
        message::{EventRequest, EventRequestAction, Service},
        CallbackHolder, EventPayload, EventStreamingClient, EventType,
    },
};

use tokio;

const SERVICE_ID: &str = "example";

// fill in a username here
const USERNAME: &str = "something";
const ENVIRONMENT: Environment = Environment::PC;

fn handle_event_message(event: &EventPayload, character: Character) {
    println!("Received event message: {:?}", event);
    match event {
        EventPayload::Death(d) => {
            if d.character_id == character {
                println!("You just got killed");
            } else if d.attacker_character_id == character {
                println!("You just killed someone");
            }
            if d.is_headshot {
                println!("Kill was a headshot");
            }
        }
        EventPayload::GainExperience(e) => {
            if e.character_id == character {
                // you earned the experience
                println!("You just earned experience");

                if e.experience_id.0 == 1 {
                    println!("You just earned kill experience");
                }
            }
        }
        _ => (), //ignore all other events
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

    // add our callback to an object to call them
    let mut callbacks = CallbackHolder::new();
    callbacks.register_event_listener(move |e| handle_event_message(e, character));
    callbacks.register_event_listener(move |e| println!("{:?}", e));

    // create a client and connect it to the server
    let mut event_client = EventStreamingClient::new(ENVIRONMENT, SERVICE_ID.to_owned(), callbacks);
    event_client.connect().await.unwrap();

    // send thee request constructed earlier
    event_client.send_request(sub_request).await.unwrap();

    // run the client
    event_client.run().await;
}
