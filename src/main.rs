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
use clap;
//use uuid::Uuid;
mod uuids;
mod gaid;
mod snowflake;
mod learn;

fn main() {
    //println!("Hello, world!");
    let _yml = clap::load_yaml!("args.yml");
    let app = clap::App::from_yaml(_yml);
    let matches = app.get_matches();
    match matches.subcommand_name() {
        Some("uuid1") => {print!("{}", uuids::v1());}
        Some("uuid3") => {
            let argm = matches.subcommand_matches("uuid3").unwrap();
            print!("{}", uuids::v3(
                argm.value_of("namespace").expect("FATAL: Namespace not provided!"),
                argm.value_of("name").expect("FATAL: Name not provided!"),
            ));
        }
        Some("uuid4") => {print!("{}", uuids::v4());}
        Some("uuid5") => {
            let argm = matches.subcommand_matches("uuid5").unwrap();
            print!("{}", uuids::v5(
                argm.value_of("namespace").expect("FATAL: Namespace not provided!"),
                argm.value_of("name").expect("FATAL: Name not provided!"),
            ));
        }
        Some("gaid") => {
            let argm = matches.subcommand_matches("gaid").unwrap();
            print!("{}", gaid::new(
                /* Real ID */  argm.value_of("realid").expect("FATAL: Real ID not provided!"), 
                /* truncate */ argm.is_present("truncate"),
                /* force 11 */ if argm.is_present("version-code-11") {11} else {10}
            ));
        }
        Some("snowflake") => {
            let argm = matches.subcommand_matches("snowflake").unwrap();
            const DISCORD_EPOCH: &str = "1420070400000";
            print!("{}", snowflake::snowflake(argm.value_of("epoch").unwrap_or(&format!("{}", DISCORD_EPOCH))));
        }
        Some("learn") => {
            let argm = matches.subcommand_matches("learn").unwrap();
            learn::learn(argm.value_of("id").unwrap());
        }
        Some(&_) => {}
        None => {}
    }
}
