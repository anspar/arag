use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "arag", about = "Anspar DApp builder")]
#[structopt(version = option_env!("ARAG_VERSION").unwrap_or(env!("CARGO_PKG_VERSION")))]
pub struct Opt {
    /// Path to custom templates directory containing index.html
    #[structopt(short, long)]
    pub entry: Option<String>,

    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(StructOpt, Debug)]
pub enum Command {
    #[structopt(about = "Create new project")]
    New {
        /// Project name
        name: String,
    },
    /// Serve the compiled html to default browser
    Show,
    /// Package everything into a single html and remove dev dependencies
    Release,
}

impl Opt {
    pub fn get_args() -> Self {
        Self::from_args()
    }
}
