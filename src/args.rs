#[derive(clap::Parser)]
pub struct Args {
    /// Confirm or not.
    #[arg(short, long, default_value_t = false)]
    pub yes: bool, // confirm
    /// git add
    #[arg(short, long, default_value_t = false)]
    pub add_all: bool, // git add
    /// git push
    #[arg(short, long, default_value_t = false)]
    pub push: bool, // git push
}
