use {clap::Parser, std::path::PathBuf};

#[derive(Parser, Debug)]
pub struct Config {
    #[arg(short, long)]
    pub automaton_path: PathBuf,

    #[arg(short, long, allow_hyphen_values = true)]
    pub input: String,
}

pub fn args() -> Config {
    Config::parse()
}
