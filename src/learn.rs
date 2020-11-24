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
use uuid::Uuid;

#[path="gaid_helper.rs"] mod gaid_helper;

pub fn learn(id: &str) {
    // "Negative one" indicates an invalid numeric ID here.
    let is_numid: bool;
    let numid: BigUint;
    let _numid = BigUint::from_str(id);
    match _numid {
        Ok(x) => {numid = x; is_numid = true;}
        Err(_) => {numid = BigUint::zero(); is_numid = false}
    }
    if id.len() <= 21 && id.len() >= 10 && is_numid {println!("{}", snowflake(id, numid));}
    match Uuid::parse_str(id) {
        Ok(x) => {
            println!("{}", uuidlearn(x));
        }
        Err(_) => {} //do nothing
    }
    if id.len() >= 21 && is_numid {println!("{}", gaid(id));}
}

fn snowflake(_id: &str, numid: BigUint) -> String {
    //Assume it might be a snowflake
    let nid = numid.to_i128().unwrap();
    let tsnum = (nid >> 22) + 1420070400000;
    let workid = (nid & 0x3E0000) >> 17;
    let procid = (nid & 0x1F000) >> 12;
    let seqid = nid & 0xFFF;
    let tsutc = chrono::Utc.timestamp_millis(tsnum.try_into().unwrap());
    let tsloc = chrono::Local.timestamp_millis(tsnum.try_into().unwrap());
    return format!("-----\nPotential type: Discord Snowflake\nTime: {}\nTime: {}\nWorker ID: {}\nProcess ID: {}\nSequence number: {}", 
        tsutc, tsloc, workid, procid, seqid)
}

fn uuidlearn(id: Uuid) -> String {
    let mut result: String = "".to_owned();
    let vn = id.get_version_num();
    result += &format!("-----\nType: UUID\nVersion: {}",
        vn);
    //if there's anything else we can determine from a UUID,
    //get it here
    return result;
}

fn gaid(id: &str) -> String {
    //=== Version Code ===//
    let vc: &str;
    if id.starts_with("10") {vc = "10";}
    else if id.starts_with("11") {vc = "11";}
    else {vc = "unknown/invalid";}
    //=== Truncation Flag ===//
    let trunc: bool = if id.starts_with("111") {true} else {false};
    let tsnum = i128::from_str_radix(if id.starts_with("111") {id.split_at(41).1} 
    else if id.starts_with("110") {id.split_at(80).1}
    else /* if id.starts_with("10") */ {id.split_at(79).1}, 10).unwrap() + 1577836800;
    let tsutc = chrono::Utc.timestamp_millis(tsnum.try_into().unwrap());
    let tsloc = chrono::Local.timestamp_millis(tsnum.try_into().unwrap());
    return format!("-----\nPotential Type: Generic Anonymous ID (GAID)\nVersion Code: {vc}{tr}\nTimestamp: {ts}\nTime: {tu}\nTime: {tl}",
        vc = vc,
        ts = tsnum,
        tu = tsutc,
        tl = tsloc,
        tr = if gaid_helper::gaid_can_trunc(vc) && trunc {"\nTruncated: YES"} 
            else if gaid_helper::gaid_can_trunc(vc) {"\nTruncated: NO"} else {""}
    )
}