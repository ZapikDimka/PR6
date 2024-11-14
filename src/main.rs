use std::error::Error;
use std::fs::File;
use std::time::Duration;
use chrono::{DateTime, Utc};
use serde_json;
use url::Url;
use uuid::Uuid;
use toml;
use serde_yaml::to_string as to_yaml;
use toml::to_string as to_toml;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::de::{self, Visitor};
use std::fmt;
#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    email: String,
    birthdate: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PublicTariff {
    id: u32,
    price: u32,
    #[serde(with = "humantime_serde")]
    duration: Duration, // Duration
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PrivateTariff {
    client_price: u32,
    #[serde(with = "humantime_serde")]
    duration: Duration,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Stream {
    user_id: Uuid,
    is_private: bool,
    settings: u32,
    shard_url: Url, //url
    public_tariff: PublicTariff,
    private_tariff: PrivateTariff,
}

#[derive(Debug, Serialize, Deserialize)]
struct Gift {
    id: u32,
    price: u32,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Debug{
    #[serde(with = "humantime_serde")]
    duration: Duration,
    at: DateTime<Utc>, //data
}

#[derive(Debug, Serialize, Deserialize)]
enum RequestType {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failure")]
    Failure,
}

#[derive(Debug, Serialize, Deserialize)]
struct Request {
    #[serde(rename = "type")]
    request_type: RequestType,
    stream: Stream,
    gifts: Vec<Gift>,
    debug: Debug,
}

fn main() {
    let event = Event {
        name: "Event 1".to_string(),
        date: "2024-11-14".to_string(),
    };

    let json = serde_json::to_string(&event).unwrap();
    println!("\n{}", json);

    let des_event: Event = serde_json::from_str(&json).unwrap();
    println!("{:?}", des_event);


    // let user = User{
    //     name:"John".to_string(),
    //     email:"johndoe321@gmail.com".to_string(),
    //     birthdate:"5.06.97".to_string()
    // };
    //
    // let json = serde_json::to_string(&user)?;
    // println!("{}", json);
    //
    // let deser_user: User = serde_json::from_str(&json)?;
    // println!("{:?}", deser_user);

    // let file = File::open("request.json")?;
    // let request: Request = serde_json::from_reader(file)?;
    //
    // println!("{:#?}", request);
    //
    // println!("yaml: {}", to_yaml(&request)?);
    // println!("toml: {}", to_toml(&request)?);
    //
    // Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;
    use serde_json;
    use uuid::Uuid;
    use chrono::{DateTime, Utc};
    use std::time::Duration;
    use url::Url;

    #[test]
    fn test_1() {
        let mut file = File::open("request.json").unwrap();
        let mut json_str = String::new();
        file.read_to_string(&mut json_str).unwrap();

        let request: Request = serde_json::from_str(&json_str).unwrap();

        // Перевірка типу запиту
        //assert_eq!(request.request_type, RequestType::Success);

        // Перевірка user_id
        assert_eq!(
            request.stream.user_id,
            Uuid::parse_str("8d234120-0bda-49b2-b7e0-fbd3912f6cbf").unwrap()
        );

        // Перевірка значення is_private
        assert_eq!(request.stream.is_private, false);

        // Перевірка значення settings
        assert_eq!(request.stream.settings, 45345);

        // Перевірка shard_url
        assert_eq!(
            request.stream.shard_url,
            Url::parse("https://n3.example.com/sapi").unwrap()
        );

        // Перевірка полів public_tariff
        assert_eq!(request.stream.public_tariff.id, 1);
        assert_eq!(request.stream.public_tariff.price, 100);
        assert_eq!(request.stream.public_tariff.duration, Duration::from_secs(3600)); // "1h" -> 3600 секунд
        assert_eq!(request.stream.public_tariff.description, "test public tariff");

        // Перевірка полів private_tariff
        assert_eq!(request.stream.private_tariff.client_price, 250);
        assert_eq!(request.stream.private_tariff.duration, Duration::from_secs(60)); // "1m" -> 60 секунд
        assert_eq!(request.stream.private_tariff.description, "test private tariff");

        // Перевірка кількості подарунків
        assert_eq!(request.gifts.len(), 2);

        // Перевірка першого подарунка
        assert_eq!(request.gifts[0].id, 1);
        assert_eq!(request.gifts[0].price, 2);
        assert_eq!(request.gifts[0].description, "Gift 1");

        // Перевірка другого подарунка
        assert_eq!(request.gifts[1].id, 2);
        assert_eq!(request.gifts[1].price, 3);
        assert_eq!(request.gifts[1].description, "Gift 2");

        // Перевірка полів debug
        assert_eq!(request.debug.duration, Duration::from_millis(234)); // "234ms" -> 234 мілісекунди
        let expected_at: DateTime<Utc> = "2019-06-28T08:35:46+00:00".parse().unwrap();
        assert_eq!(request.debug.at, expected_at);
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Event {
    name: String,
    #[serde(serialize_with = "serialize_date", deserialize_with = "deserialize_date")]
    date: String,
}

fn serialize_date<S>(date: &str, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let formatted_date = format!("Date: {}", date);
    serializer.serialize_str(&formatted_date)
}

fn deserialize_date<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let data: &str = Deserialize::deserialize(deserializer)?;
    Ok(data.replace("Date: ", ""))
}