#[cfg(test)]
mod tests {
    use super::*;
    use rocket::local::blocking::Client;
    use rocket::http::Status;

    #[test]
    fn test_hi_endpoint_with_special_characters() {
        let client = Client::blocking(rocket::build().mount("/", routes![hi]));
        let response = client.get("/hi/?special=characters").dispatch();
        assert_eq!(response.status(), 200);
        assert_eq!(response.into_string().unwrap(), "Hi Rocket, updating!");
    }

    #[test]
    fn test_hi_endpoint_with_empty_special_characters() {
        let client = Client::blocking(rocket::build().mount("/", routes![hi]));
        let response = client.get("/hi/?special=").dispatch();
        assert_eq!(response.status(), 200);
        assert_eq!(response.into_string().unwrap(), "Hi Rocket, updating!");
    }

    #[test]
    fn test_hi_endpoint_with_special_characters_in_name() {
        let client = Client::blocking(rocket::build().mount("/", routes![hi, name]));
        let response = client.get("/hi/?name=special%20characters").dispatch();
        assert_eq!(response.status(), 200);
        assert_eq!(response.into_string().unwrap(), "Hi Rocket, updating!");
    }

    #[test]
    fn test_hi_endpoint_with_empty_special_characters_in_name() {
        let client = Client::blocking(rocket::build().mount("/", routes![hi, name]));
        let response = client.get("/hi/?name=").dispatch();
        assert_eq!(response.status(), 200);
        assert_eq!(response.into_string().unwrap(), "Hi Rocket, updating!");
    }

    #[test]
    fn test_hi_endpoint_with_special_characters_in_url() {
        let client = Client::blocking(rocket::build().mount("/", routes![hi]));
        let response = client.get("/hi/?url=special%20characters").dispatch();
        assert_eq!(response.status(), 200);
        assert_eq!(response.into_string().unwrap(), "Hi Rocket, updating!");
    }
}