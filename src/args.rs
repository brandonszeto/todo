use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct TodoArgs {
    /// Current project
    project: String,

    /// Current task
    task: String,
}
