# rust-docker-headless-chrome

This project is a PoC demonstrating how to run a Rust project using headless Chrome inside a Docker container.

## Run

```bash
docker run --rm -it $(docker build . --quiet)
```
