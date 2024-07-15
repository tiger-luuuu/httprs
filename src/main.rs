use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "tiger_lu")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post),
}

struct Get {
    #[clap(short, long)]
    id: Option<String>,
}

struct Post {
    #[clap(short, long)]
    id: Option<String>,
}

fn main() {
    println!("Hello, world!");
}
