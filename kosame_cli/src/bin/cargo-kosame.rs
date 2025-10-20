use clap::{Args, Parser};

#[derive(Parser)]
#[command(
    name = "kosame",
    bin_name = "kosame",
    about = "Kosame: Macro-based Rust ORM focused on developer ergonomics"
)]
enum Root {
    Fmt(Fmt),
    Introspect(Introspect),
}

#[derive(Args)]
#[command(version, about = "Format the content of Kosame macro invocations in Rust source files", long_about = None)]
struct Fmt {
    #[arg(short, long)]
    file: Option<std::path::PathBuf>,
}

#[derive(Args)]
#[command(version, about = "Introspects a database and generates a matching Kosame schema", long_about = None)]
struct Introspect {}

fn main() {
    Root::parse();
    println!("kek :>");
}
