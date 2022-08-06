use structopt::StructOpt;

#[derive(StructOpt)]
enum Command {
    Chat {},
}

fn parse_args() {
    match Command::from_args() {
        Command::Chat {} => {
            println!("Chat!");
        }
    }
}

fn main() {
    parse_args();
}
