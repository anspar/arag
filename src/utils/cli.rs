use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "arag", about = "Anspar DApp builder")]
pub struct Opt {
    /// Serve the packaged html
    #[structopt(short, long)]
    pub show: bool,

    /// Package everything into a single html
    #[structopt(short, long)]
    pub pkg: bool,

    /// Path to custom entry html file
    #[structopt(short, long)]
    pub entry: Option<String>,

    #[structopt(subcommand)]
    pub cmd: Option<Command>,
}

#[derive(StructOpt, Debug)]
#[structopt(about = "Create new project")]
pub enum Command {
    New {
        /// Project name
        name: String,
    },
}

pub fn get_args() -> Opt {
    Opt::from_args()
}
