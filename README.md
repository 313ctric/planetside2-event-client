# planetside2-event-client

A client for the [Planetside 2 event streaming api](http://census.daybreakgames.com/#what-is-websocket).

This client is incomplete, however it has a lot of basic functions that may be useful.

## Usage

The best introduction is to read the [simple example](/examples/simple.rs).

In short:

1. You create and register callback functions for the client to call whenever it receives a new message.
```rust
let mut callbacks = CallbackHolder::new();
callbacks.register_event_listener(move |e| { println!("{:?}", e) });

// also available for more customization
callbacks.register_ps2_response_listener();
callbacks.register_all_response_listener();
```

2. You create an event client, then connect it.
```rust
let mut event_client = EventStreamingClient::new(ENVIRONMENT, SERVICE_ID.to_owned(), callbacks);
event_client.connect().await.unwrap();
```

3. You create and send a subscription request
```rust
let sub_request = EventRequest {
	action:EventRequestAction::Subscribe,
	service: Service::Event,
	characters:Some(character_ids),
	worlds:Some(worlds),
	event_names:Some(events),
	all: None,
	list_characters: None,
	payload: None
};
event_client.send_request(sub_request).await.unwrap();
```

4. You start the client
```rust
event_client.run().await;
```

## Examples

### Simple

A simple use case, listening to events then printing out information about them.

### Passing In Channel

Uses a channel to pass the events to another task for processing, allowing the processing thread to call the async functions provided to get additional information from the census api.

## Structure

### Event

The main client for connecting to the event websocket service

### Census

A small set of helper functions that interact with the census rest api to get additional information about the data from the event streaming api.

### Data

Data types common to both the census and event client, most of them are just simple wrappers around primitive types.

## Contributing

Don't.