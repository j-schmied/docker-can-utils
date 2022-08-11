#![allow(dead_code)]

use std::process::exit;
use std::str;

use clap::Parser;
use docker_api::{conn::TtyChunk, Docker, Result, Exec};

#[cfg(unix)]
pub fn new_docker() -> Result<Docker> {
    Ok(Docker::unix("/var/run/docker.sock"))
}

#[cfg(not(unix))]
pub fn new_docker() -> Result<Docker> {
    Docker::new("tcp://127.0.0.1:8080")
}

pub fn print_chunk(chunk: TtyChunk) {
    match chunk {
        TtyChunk::StdOut(bytes) => {
            println!("Stdout: {}", str::from_utf8(&bytes).unwrap_or_default())
        }
        TtyChunk::StdErr(bytes) => {
            eprintln!("Stdout: {}", str::from_utf8(&bytes).unwrap_or_default())
        }
        TtyChunk::StdIn(_) => unreachable!(),
    }
}

#[derive(Parser)]
pub struct Opts {
    #[clap(subcommand)]
    subcmd: Cmd,
}

#[derive(Parser)]
enum Cmd {
    /// Run a command in container and inspect it
    Inspect {
        /// The container to run the command in.
        container: String,
        /// Command to run.
        cmd: Vec<String>,
    },
    /// Resize the TTY session used by an exec instance.
    Resize {
        exec: String,
        width: u64,
        height: u64,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let valid_cmds = [ "cansniffer",  // display CAN data content differences
                                    "candump",  // display, filter and log CAN data to files
                                    "cansend",  // send a single frame
                                    "cangen",  // generate (random) CAN traffic
                                    "cansequence",  // send and check sequence of CAN frames with incrementing payload
                                    "canplayer",  // replay CAN logfiles
                                    "canlogserver",  // log CAN frames from a remote/local host
                                    "bcmserver",  // interactive BCM configuration (remote/local)
                                    "socketcand",  // use RAW/BCM/ISO-TP sockets via TCP/IP sockets
                                    "cannelloni",  // UDP/SCTP based SocketCAN tunnel
                                    "cangw",  // CAN gateway userspace tool for netlink configuration
                                    "canbusload",  // calculate and display the CAN busload
                                    "can-calc-bit-timing",  // userspace version of in-kernel bitrate calculation
                                    "canfdtest",  // Full-duplex test program (DUT and host part)
                                    "isotpdump",  // 'wiretap' and interpret CAN messages (CAN_RAW)
                                    "isotpperf",  // ISO15765-2 protocol performance visualisation
                                    "isotprecv",  // receive ISO-TP PDU(s)
                                    "isotpsend",  // send a single ISO-TP PDU
                                    "isotpsniffer",  // 'wiretap' ISO-TP PDU(s)
                                    "isotpserver",  // IP server for simple TCP/IP <-> ISO 15765-2 bridging (ASCII HEX)
                                    "isotptun",  // create a bi-directional IP tunnel on CAN via ISO-TP
                                    "j1939acd",  // address claim daemon
                                    "j1939cat",  // take a file and send and receive it over CAN
                                    "j1939spy",  // spy on J1939 messages using SOC_J1939
                                    "j1939sr",  // send/recv from stdin or to stdout
                                    "testj1939",  // send/receive test packet
                                    "asc2log",  // convert ASC logfile to compact CAN frame logfile
                                    "log2asc",  // convert compact CAN frame logfile to ASC logfile
                                    "log2long",  // convert compact CAN frame representation into user readable
                                    "slcan_attach",  // userspace tool for serial line CAN interface configuration
                                    "slcand",  // daemon for serial line CAN interface configuration
                                    "slcanpty",  // creates a pty for applications using the slcan ASCII protocol
                                    "whoami"
    ];

    env_logger::init();
    let opts: Opts = Opts::parse();
    let docker = new_docker()?;

    match opts.subcmd {
        Cmd::Inspect { container, cmd } => {
            use docker_api::opts::ExecContainerOpts;
            use futures::StreamExt;

            if !valid_cmds.contains(&cmd[0].as_str()) {
                eprintln!("Invalid command: {}", cmd[0]);
                exit(1);
            }

            // Create Opts with specified command
            let opts = ExecContainerOpts::builder()
                .cmd(cmd)
                .attach_stdout(true)
                .attach_stderr(true)
                .build();

            let exec = Exec::create(docker, &container, &opts).await?;

            println!("{:#?}", exec.inspect().await?);

            let mut stream = exec.start();

            stream.next().await;

            println!("{:#?}", exec.inspect().await?);
        }
        Cmd::Resize {
            exec,
            width,
            height,
        } => {
            use docker_api::opts::ExecResizeOpts;
            let exec = Exec::get(docker, &exec);

            // Resize its window with given parameters
            let resize_opts = ExecResizeOpts::builder()
                .width(width)
                .height(height)
                .build();
            exec.resize(&resize_opts).await?;
        }
    }

    Ok(())
}
