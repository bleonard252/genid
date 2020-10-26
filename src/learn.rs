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
use num_bigint::{BigUint};
use std::str::FromStr;
use num_traits::{ToPrimitive, Zero};
use std::convert::TryInto;
use chrono;
use chrono::TimeZone;

pub fn learn(id: &str) {
    // "Negative one" indicates an invalid numeric ID here.
    let is_numid: bool;
    let numid: BigUint;
    let _numid = BigUint::from_str(id);
    match _numid {
        Ok(x) => {numid = x; is_numid = true;}
        Err(_) => {numid = BigUint::zero(); is_numid = false}
    }
    if id.len() <= 21 && id.len() >= 10 && is_numid {
        //Assume it might be a snowflake
        let nid = numid.to_i128().unwrap();
        let tsnum = (nid >> 22) + 1420070400000;
        let workid = (nid & 0x3E0000) >> 17;
        let procid = (nid & 0x1F000) >> 12;
        let seqid = nid & 0xFFF;
        //let tsdur = std::time::Duration::from_millis(tsnum.try_into().unwrap());
        //let tsdur = chrono::Duration::milliseconds(tsnum.try_into().unwrap());
        let tsutc = chrono::Utc.timestamp_millis(tsnum.try_into().unwrap());
        let tsloc = chrono::Local.timestamp_millis(tsnum.try_into().unwrap());
        println!("-----\nPotential type: Discord Snowflake\nTime: {}\nTime: {}\nWorker ID: {}\nProcess ID: {}\nSequence number: {}", 
            tsutc, tsloc, workid, procid, seqid)
    }
}