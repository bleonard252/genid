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
use sha2::{Sha256, Digest};
use std::ops::Add;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use num_bigint::BigUint;
use num_traits::Num;
pub fn new(real: &str, trunc: bool, vc: i8) -> String {
    if trunc || vc == 10 {return vc11(real, trunc);}
    else {return vc10(real);};
}

fn vc10(real: &str) -> String {
    const VERSION: i32 = 10;
    // {version}{digest16 in base10 (len: 78)}{timestamp}
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH.add(Duration::from_millis(1577836800))).unwrap();
    //let mut digest: generic_array::GenericArray<u8, generic_array::typenum::UInt<_, _>> = generic_array::arr![u8;];
    let digest = Sha256::digest((real.to_owned() + &format!("{}", timestamp.as_millis())).as_bytes());
    let digest10 = BigUint::from_str_radix(&format!("{:x}", digest), 16).unwrap();
    return format!("{version}{digest10:077}{timestamp}", version = VERSION, digest10 = digest10, timestamp = timestamp.as_millis());
    //let bytes = String::from_utf8(digest.to_vec()).unwrap();
}
fn vc11(real: &str, trunc: bool) -> String {
    const VERSION: i32 = 11;
    // {version}{digest16 in base10 (len: 78 or 39)}{timestamp}
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH.add(Duration::from_millis(1577836800))).unwrap();
    let digest = Sha256::digest((real.to_owned() + &format!("{}", timestamp.as_millis())).as_bytes());
    let digest10 = BigUint::from_str_radix(&format!("{:x}", digest), 16).unwrap();
    let digest11 = &mut String::from(format!("{digest10:077}", digest10 = digest10));
    let truncdigit = if trunc {1} else {0};
    if trunc {
        digest11.truncate(38);
    }
    return format!("{version}{truncdigit}{digest11}{timestamp}", version = VERSION, truncdigit = truncdigit, digest11 = digest11, timestamp = timestamp.as_millis());
}