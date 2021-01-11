use structopt::StructOpt;

use st_cli::StCli;

fn main() {
    let cli: StCli = StCli::from_args();
    cli.run()
}
