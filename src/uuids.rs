use uuid::Uuid;
use uuid::v1::{Timestamp, Context};
use std::convert::TryInto;

pub fn v1() -> String {
    let ts = std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap_or_default();
    let id = Uuid::new_v1(Timestamp::from_unix(
        Context::new(std::process::id().try_into().unwrap_or_default()),
        ts.as_secs(), ts.as_nanos().try_into().unwrap_or_default()),
        &[std::process::id().try_into().unwrap_or_default()]
    ).unwrap_or_default();
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