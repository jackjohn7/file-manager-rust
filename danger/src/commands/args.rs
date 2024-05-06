use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct DangerArgs {
    #[arg(short, long, action)]
    pub full_paths: bool,

    #[arg(short, long, action)]
    pub paths_relative: bool,

    #[arg(short, long, default_value_t=String::from("."))]
    pub input: String,

    #[arg(short, long, action)]
    pub source: bool,
}
