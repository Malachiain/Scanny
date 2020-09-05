# Scanny

Destined to be a fully featured port scanner that can be deployed and run as a binary. A project for me to practice the Rust that I have been learning. The plan is contribte small but frequent updates and improvments as I learn more.

Building or installing from source will need the standard Rust toolchain installed with `rustup`.

# Features

Will scan ports within the specified range on localhost

## Building

`cargo build`

## Install form source

`cargo install --path ./`

## Running

`cargo run -- --from {port_from} --to {port_to}` 

