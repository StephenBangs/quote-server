# quote-server

Final Project - Quote server for CS 410P - Full Stack Rust

Github repository:

*https://github.com/StephenBangs/quote-server*

Stephen Bangs

CS 410P - Bart Massey

Quote Server Rust Webserver

Final Project Submission - Create a quote server webpage in a browser.

Written as an Axum Webservice with:
    -An Askama-templated HTML UI 
    -A Utoipa-documented REST API, with a documentation endpoint for swagger UI
    -A basic REST browser client UI written in Leptos

6/10/25

**This is a basic webserver written in Rust, with a sqlite database.**

# Currently working on Final submission: 

_Your course project is expected to be either a recipe service or a quote service, written in Axum with SQlite. You may have used SQLx or an ORM such as SeaORM. We will be looking for:

A database-driven Axum web service with:
An Askama-templated HTML UI
A Utoipa-documented REST API, with a documentation endpoint for at least Swagger-UI
A basic REST browser client UI written in Yew, Leptos or some other WASM-based frontend
A reasonably thorough README.md with your name, details of what you did, what did not go well or is still outstanding, and anything else you think we should know when looking at the project
Optional but desirable features include:

Authentication using the JWT scheme taught in class or something similar.
Ability to do authenticated writes to the database through one or more client UIs and/or the REST API
A working Dockerfile
Submission
You should once again submit your project URL; also let us know in Zulip if it has changed since the second checkpoint. We will pull your repo on the due date, and notify you if there is a problem with that.

The browser client (Yew, Leptos or whatever) will necessarily be in a second crate source directory. That crate source should either be part of your original repo, or included in that repo as a Git submodule in case it was developed separately.

Please do ask on Zulip if you have any questions._

# Steps to display single quote:

1) Download and unzip repository
2) Use commands:
    cargo build
    cargo run
3) Don't close program running, and open browser to visit localhost port 3000:
    http://127.0.0.1:3000/
4) Stare in awe and wonder upon life's mysteries while looking at the quote. Take a second to appreciate yourself and your own journey. Or don't! After all, you are the master of your own fate: you are the captain of your soul.


# Steps to create Database

I was having an incredibly hard time getting  `cargo sqlx migrate run` to work, but this seems to work on the command line:

`sqlx database setup --source .\migrations\`

`cargo sqlx prepare`

# Steps to use Swagger API

//Todo

# How to use Leptos

//Todo

# Misc info

My milestone 2 submission was done two days after the given due date of the 30th - It looks like somebody scraped and cloned my github repo on the 2nd of June, but I wanted to let you know that I needed a little more time to get it working!! Sorry!!