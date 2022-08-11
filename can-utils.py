#!/usr/bin/env python
import docker
import sys

VALID_CMDS: set = {'cansniffer',  # display CAN data content differences
                   'candump',  # display, filter and log CAN data to files
                   'cansend',  # send a single frame
                   'cangen',  # generate (random) CAN traffic
                   'cansequence',  # send and check sequence of CAN frames with incrementing payload
                   'canplayer',  # replay CAN logfiles
                   'canlogserver',  # log CAN frames from a remote/local host
                   'bcmserver',  # interactive BCM configuration (remote/local)
                   'socketcand',  # use RAW/BCM/ISO-TP sockets via TCP/IP sockets
                   'cannelloni',  # UDP/SCTP based SocketCAN tunnel
                   'cangw',  # CAN gateway userspace tool for netlink configuration
                   'canbusload',  # calculate and display the CAN busload
                   'can-calc-bit-timing',  # userspace version of in-kernel bitrate calculation
                   'canfdtest',  # Full-duplex test program (DUT and host part)
                   'isotpdump',  # 'wiretap' and interpret CAN messages (CAN_RAW)
                   'isotpperf',  # ISO15765-2 protocol performance visualisation
                   'isotprecv',  # receive ISO-TP PDU(s)
                   'isotpsend',  # send a single ISO-TP PDU
                   'isotpsniffer',  # 'wiretap' ISO-TP PDU(s)
                   'isotpserver',  # IP server for simple TCP/IP <-> ISO 15765-2 bridging (ASCII HEX)
                   'isotptun',  # create a bi-directional IP tunnel on CAN via ISO-TP
                   'j1939acd',  # address claim daemon
                   'j1939cat',  # take a file and send and receive it over CAN
                   'j1939spy',  # spy on J1939 messages using SOC_J1939
                   'j1939sr',  # send/recv from stdin or to stdout
                   'testj1939',  # send/receive test packet
                   'asc2log',  # convert ASC logfile to compact CAN frame logfile
                   'log2asc',  # convert compact CAN frame logfile to ASC logfile
                   'log2long',  # convert compact CAN frame representation into user readable
                   'slcan_attach',  # userspace tool for serial line CAN interface configuration
                   'slcand',  # daemon for serial line CAN interface configuration
                   'slcanpty',  # creates a pty for applications using the slcan ASCII protocol
                   'whoami'
                   }


def parse_args() -> list:
    args: list = list()

    for arg in sys.argv:
        args.append(arg)

    return args


def main():
    target_name: str = 'can-utils'
    target_exists: bool = False

    # Connect to local Docker Daemon
    try:
        client = docker.from_env()
    except docker.errors.APIError:
        print(f"[!] Error connection to Docker. Is the Docker Daemon running?")
        exit(1)

    print(f"[*] Using {client.version()['Platform']['Name']}")

    # Check if CL arguments are supplied
    if len(sys.argv) < 2:
        print("[i] Usage: ./can-utils.py <Command> <Command Args>*")
        exit(0)

    # Get CLI Args: $1: command, $2- args
    cli_args: list = parse_args()
    cmd: str = cli_args[1]

    print(f"[*] Chosen command: {cmd}")

    cmd_args: list = list()

    # Check if command is a valid can-utils command
    if cmd not in VALID_CMDS:
        print("[!] Invalid command.")
        exit(1)

    for i in range(1, len(sys.argv)):
        cmd_args.append(sys.argv[i])

    # Check if target container exists
    for container in client.containers.list():
        if container.name == target_name:
            target_exists = True
            break

    if not target_exists:
        print(f"[!] Error: target {target_name} does not exist.")
        exit(1)

    container = client.containers.get(target_name)

    # Execute command
    output = container.exec_run(cmd_args, tty=True)

    if output[0] != 0:
        print(f"[!] Error (exit code: {output[0]}): {output[1].decode('utf-8')}")
        exit(1)

    print(f"[*] Output:\n{output[1].decode('utf-8')}")

    exit(0)


if __name__ == '__main__':
    main()
