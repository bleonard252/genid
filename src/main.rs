use clap;
//use uuid::Uuid;
mod uuids;

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
        Some(&_) => {}
        None => {}
    }
}
