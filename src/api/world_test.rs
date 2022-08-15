use crate::*;
use rocket::{Build, Rocket};

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount(
        "/api/world",
        routes![get_world_info, get_worlds, push_world_info, push_worlds],
    )
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn hello_world() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!("/api/world/")).dispatch();
        assert_eq!(response.status(), Status::Ok);
        // assert_eq!(response.into_string().unwrap(), "Hello, world!");
    }
}
