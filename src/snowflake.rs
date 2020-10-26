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
use rand;
use rand::Rng;
use chrono::{Utc, TimeZone};
use num_traits::FromPrimitive;

pub fn snowflake(epoch: &str) -> u64 {
    //let _epoch = Duration::from_millis(u64::from_str_radix(epoch, 10).unwrap());
    let _epoch = Utc.timestamp_millis(i64::from_str_radix(epoch, 10).unwrap());
    #[allow(unused_assignments)]
    let mut result: u64;
    let ts = Utc::now();
    println!("{}", ts.timestamp_millis());
    result = u64::from_i64(ts.timestamp_millis() - _epoch.timestamp_millis()).unwrap() << 22;
    result |= rand::thread_rng().gen_range::<u64, u64, u64>(0, 1024) << 17;
    result |= rand::thread_rng().gen_range::<u64, u64, u64>(0, 32) << 12;
    result |= rand::thread_rng().gen_range::<u64, u64, u64>(0, 4096);
    return result;
}