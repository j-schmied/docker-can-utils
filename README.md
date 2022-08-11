# Docker can-utils

Use can-utils platform independent via Docker Container.

## Usage

* Build and Run Docker Container

```bash
docker build -t "can-utils" .
docker run -d --name can-utils can-utils
```

* via Python Client: `python src/can-utils.py <command> <command args>*`
* via Rust Client: `cargo build; ./target/main <command> <command args>*`

## Notes

* Linuxkit for Mac, Windows do not currently support CAN. 

## Credits

* docker_api: https://github.com/vv9k/docker-api-rs
