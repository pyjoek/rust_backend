#[macro_use] extern crate rocket;

use rocket::serde::{Serialize, Deserialize, json::Json};

#[derive(Serialize)]
struct Message {
    message: String
}

#[derive(Deserialize)]
struct create {
    name: String
}

#[get("/")]
fn index() -> Json<Message> {
    Json(Message{
        message: "Server running".to_string()
    })
}

#[post("/add", format="json", data="<data>")]
fn createData(data: Json<create>) -> Json<Message> {
    Json(Message{
        message: format!("Received: {}", data.name)
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, createData])
}