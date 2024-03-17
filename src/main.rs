// this is testing the ankiConnect ability, sending json from a POST request.
use serde::{Serialize, Deserialize};
use reqwest::Client;
use tokio;

#[derive(Serialize, Deserialize)]
struct Note {
    deckName: String,
    modelName: String,
    fields: Fields,
}

#[derive(Serialize, Deserialize)]
struct Fields {
    front: String,
    back: String,
}

#[derive(Serialize, Deserialize)]
struct Params {
    notes: Vec<Note>,
}

#[derive(Serialize, Deserialize)]
struct RequestBody {
    action: String,
    version: i32,
    params: Params,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let note = Note {
        deckName: "test".to_string(),
        modelName: "Basic-1e476".to_string(),
        fields: Fields {
            front: "can it be\n multiple??????".to_string(),
            back: "nope, but one thing at a time.".to_string(),
        },
    };

    let request_body = RequestBody {
        action: "addNotes".to_string(),
        version: 6,
        params: Params {
            notes: vec![note],
        },
    };

    let client = Client::new();
    let res = client.post("http://localhost:8765")
        .json(&request_body)
        .send()
        .await?;

    println!("Status: {}", res.status());
    Ok(())
}

