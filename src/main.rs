#[macro_use] extern crate rocket;

use rocket::serde::{Serialize, Deserialize, json::Json};

#[derive(Serialize)]
struct Message {
    message: String
}

#[derive(Deserialize)]
struct CreateMessage {
    name: String
}

#[get("/")]
fn root() -> Json<Message> {
    Json(Message {
        message: "Hello server testing".to_string()
    }
    )
}

#[post("/add", format = "json", data = "<data>")]
fn create(data: Json<CreateMessage>) -> Json<Message> {
    Json(Message{
        message: format!("Recieved: {}", data.name)
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![root, create])
}