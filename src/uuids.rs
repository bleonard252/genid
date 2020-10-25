/*
   Copyright 2020 Blake Leonard

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/
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
    let nsid = Uuid::parse_str(namespace).expect("Invalid namespace UUID");
    let namebytes = name.as_bytes();
    let id = Uuid::new_v3(&nsid, namebytes);
    return id.to_hyphenated().to_string();
}

pub fn v4() -> String {
    let id = Uuid::new_v4();
    return id.to_hyphenated().to_string();
}

pub fn v5(namespace: &str, name: &str) -> String {
    let nsid = Uuid::parse_str(namespace).expect("Invalid namespace UUID");
    let namebytes = name.as_bytes();
    let id = Uuid::new_v5(&nsid, namebytes);
    return id.to_hyphenated().to_string();
}