use clap::{Arg, Command};
use crate::ipc::Ipc;
use crate::Arc;

pub fn get_matches() -> clap::ArgMatches {
    Command::new("RouilleSpy")
        .version("1.0")
        .author("Your Name <your@email.com>")
        .about("Displays user records from IPC memory")
        .arg(
            Arg::new("ipc_key")
                .short('k')
                .long("key")
                .value_name("IPC_KEY")
                .help("Sets the IPC key (e.g., 0x0000DEAD)")
                .default_value("0x0000DEAD")
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("gui")
                .short('g')
                .long("gui")
                .help("Run in GUI mode"),
        )
        .arg(
            Arg::new("verbosity")
                .short('v')
                .long("verbosity")
                .help("Sets the level of verbosity")
                .value_parser(["error", "warn", "info", "debug", "trace"])
                .default_value("info"),
        )
        .get_matches()
}

pub fn run_cli(ipc: Arc<Ipc>) {
    loop {
        let records = ipc.read_user_records();
        for record in records {
            println!(
                "Username: {}, Command: {}, Download Speed: {:.2} KB/s, Upload Speed: {:.2} KB/s",
                String::from_utf8_lossy(&record.username),
                String::from_utf8_lossy(&record.command),
                record.download_speed,
                record.upload_speed
            );
        }
        std::thread::sleep(std::time::Duration::from_secs(1)); // Update every 1 second
    }
}
