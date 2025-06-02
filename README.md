# quote-server

Quote server for CS 410P - Full Stack Rust

Stephen Bangs

CS 410P - Bart Massey

Quote Server Rust Webserver

Milestone 1 - Display a single joke in a browser using Axum and Askama (and Tokio)

4/27/28

**This is a basic webserver written in Rust that will develop into a more fully "feature complete" webserver as we continue in the class.**

# Currently working on Milestone 1: 

_"Make a Rust project in the repository, and catch up with the class to the point of having a basic templated server-side website that displays a single quote or recipe. Build your webserver using Axum and Askama, following the patterns of our server."_


# Steps to display single quote:

1) Download and unzip repository
2) Use commands:
    cargo build
    cargo run
3) Don't close program running, and open browser to visit localhost port 3000:
    http://127.0.0.1:3000/
4) Stare in awe and wonder upon life's mysteries while looking at the quote. Take a second to appreciate yourself and your own journey. Or don't! After all, you are the master of your own fate: you are the captain of your soul.


# Steps to create Database

I was having an incredibly hard time getting  `cargo sqlx migrate run` to work, but this seems to:

sqlx database setup --source .\migrations\