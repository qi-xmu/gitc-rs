#[derive(clap::Parser)]
pub struct Args {
    #[arg(short, long, default_value_t = false)]
    pub yes: bool, // confirm
    #[arg(short, long, default_value_t = false)]
    pub add_all: bool, // git add
    #[arg(short, long, default_value_t = false)]
    pub push: bool, // git push
}
