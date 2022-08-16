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
    use crate::server::world::WorldInfo;

    use super::rocket;
    use rocket::http::{ContentType, Status};
    use rocket::local::blocking::Client;

    #[test]
    fn get_world_list() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!("/api/world/")).dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn push_get_single_world() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");

        let post_json = serde_json::to_string(&WorldInfo {
            world_uuid: "uuid".to_string(),
            ..WorldInfo::default()
        })
        .unwrap();
        ////////////// Push ////////////////
        let response = client
            .post(uri!("/api/world/uuid"))
            .header(ContentType::JSON)
            .body(post_json)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        ////////////// Get ////////////////
        let response = client
            .get(uri!("/api/world/uuid"))
            .header(ContentType::JSON)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn push_get_multi_world() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");

        let post_json = serde_json::to_string(&vec![
            WorldInfo {
                world_uuid: "uuid".to_string(),
                ..WorldInfo::default()
            },
            WorldInfo {
                world_uuid: "uuid1".to_string(),
                ..WorldInfo::default()
            },
        ])
        .unwrap();
        ////////////// Push ////////////////
        let response = client
            .post(uri!("/api/world"))
            .header(ContentType::JSON)
            .body(post_json)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        ////////////// Get ////////////////
        let response = client
            .get(uri!("/api/world"))
            .header(ContentType::JSON)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        let r = response.into_json::<Vec<WorldInfo>>().unwrap();
        assert_eq!(r.len(), 2);
    }
}
