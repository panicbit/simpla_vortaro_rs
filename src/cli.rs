#[derive(clap::Parser)]
#[command(version, author)]
pub struct Cli {
    /// La vorton kion vi volas serĉi
    pub vorto: String,
    /// Akiras la difino de la VORTO anstataŭ serĉi ĝin
    #[clap(short = 'd', long = "difinu")]
    pub difinu: bool,
}
