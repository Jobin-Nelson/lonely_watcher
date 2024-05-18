use clap::Parser;

mod cli;

fn main() {
    let args = cli::Args::parse();

    match &args.command {
        cli::Commands::Log { log_file } => {
            sys_monitor::log_perf(log_file);
        }
        cli::Commands::Stat { log_file } => {
            todo!()
        }
    }
}
