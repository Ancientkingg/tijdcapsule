use serde::{Deserialize, Serialize};
use chrono::{DateTime, NaiveDate, Utc};
use rand::{distributions::{Alphanumeric, DistString}, thread_rng, RngCore};

const LOCAL_EPOCH: DateTime<Utc> = match NaiveDate::from_ymd_opt(2024, 5, 11) {
    Some(date) => match date.and_hms_opt(14, 42,53) {
        Some(datetime) => datetime.and_utc(),
        None => panic!("Error building epoch!"),
    },
    None => panic!("Error building epoch!"),
};

#[derive(Debug, Deserialize, Serialize)]
pub struct CapsuleId(String);

impl CapsuleId {
    pub fn generate() -> CapsuleId {
        let entropy = (thread_rng().next_u32() as u64) << 48;
        let ms_since_epoch = ((Utc::now().to_utc()
                                .signed_duration_since(LOCAL_EPOCH)
                                .num_milliseconds() as u64) << 16) >> 16;
        let number_representation = entropy | ms_since_epoch;
        let id = base62::encode(number_representation);
        CapsuleId(id)
    }
}

impl From<String> for CapsuleId {
    fn from(s: String) -> Self {
        CapsuleId(s)
    }
}

impl From<&str> for CapsuleId {
    fn from(s: &str) -> Self {
        CapsuleId(s.to_string())
    }
}

impl From<std::option::Option<std::string::String>> for CapsuleId {
    fn from(s: std::option::Option<std::string::String>) -> Self {
        CapsuleId(s.unwrap())
    }
}

impl std::fmt::Display for CapsuleId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub mod key {
    use rand::distributions::{Alphanumeric, DistString};

    const FRAGMENT_LENGTH: usize = 6;
    const KEY_LENGTH: usize = 32;

    pub fn generate() -> (String, String) {
        let key_fragment = Alphanumeric.sample_string(&mut rand::thread_rng(), FRAGMENT_LENGTH);

        let key = {
            let mut key = key_fragment.repeat(KEY_LENGTH / FRAGMENT_LENGTH + 1);
            key.truncate(KEY_LENGTH);
            key
        };

        (key_fragment, key)
    }

    pub fn to_key(key_fragment: &str) -> String {
        

        {
            let mut key = key_fragment.repeat(KEY_LENGTH / FRAGMENT_LENGTH + 1);
            key.truncate(KEY_LENGTH);
            key
        }
    }
}

pub fn generate_client_id() -> String {
    let client_id = Alphanumeric.sample_string(&mut thread_rng(), 32);
    client_id
}