use uuid::Uuid;
use uuid::v1::{Timestamp, Context};
use std::convert::TryInto;

pub fn v1() -> String {
    let ts = std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap();
    let id = Uuid::new_v1(Timestamp::from_unix(
        Context::new(std::process::id().try_into().unwrap()),
        ts.as_secs(), 0),
        &[9, 9, 9, 9, 8, 7]
    ).unwrap();
    return id.to_hyphenated().to_string();
}

pub fn v3(namespace: &str, name: &str) -> String {
    let nsid = Uuid::parse_str(namespace).unwrap_or_default();
    let namebytes = name.as_bytes();
    let id = Uuid::new_v3(&nsid, namebytes);
    return id.to_hyphenated().to_string();
}

pub fn v4() -> String {
    let id = Uuid::new_v4();
    return id.to_hyphenated().to_string();
}

pub fn v5(namespace: &str, name: &str) -> String {
    let nsid = Uuid::parse_str(namespace).unwrap_or_default();
    let namebytes = name.as_bytes();
    let id = Uuid::new_v5(&nsid, namebytes);
    return id.to_hyphenated().to_string();
}