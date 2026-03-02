mod app;
mod metrics;
mod ui;
mod docker;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long, default_value = "RackTop")]
    title: String,

    #[arg(long)]
    auto_tab: Option<u64>,
}

fn main() {
    let args = Args::parse();
    app::run(args.title, args.auto_tab);
}