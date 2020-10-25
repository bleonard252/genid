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
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use rand;

pub fn snowflake(epoch: &str) -> u64 {
    let _epoch = Duration::from_millis(u64::from_str_radix(epoch, 10).unwrap());
    #[allow(unused_assignments)]
    let mut result: u64 = 0;
    let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    result = ((ts - _epoch).as_millis() as u64) << 22;
    result |= (rand::random::<u64>()) << 17;
    result |= (rand::random::<u64>()) << 12;
    result |= rand::random::<u64>();
    return result;
}