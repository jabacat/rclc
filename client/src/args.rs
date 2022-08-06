use structopt::StructOpt;

#[derive(StructOpt)]
enum Command {
    Chat {},
}

pub fn parse_args() {
    match Command::from_args() {
        Command::Chat {} => {
            println!("Chat!");
        }
    }
}
