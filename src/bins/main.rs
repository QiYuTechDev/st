use structopt::StructOpt;

use st::StCli;

fn main() {
    let cli: StCli = StCli::from_args();
    cli.run()
}
