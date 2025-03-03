_:
  @just --list

test-client:
  cargo run --bin client

test-server:
  cargo run --bin server
