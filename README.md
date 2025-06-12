# quote-server

Final Project - Quote server for CS 410P - Full Stack Rust

Github repository:

*https://github.com/StephenBangs/quote-server*

Stephen Bangs

CS 410P - Bart Massey

Quote Server Rust Webserver

Final Project Submission - Create a quote server webpage in a browser.

Written as an Axum Webservice with:
    -An Askama-templated HTML UI w/ CSS 
    -A Utoipa-documented REST API, with a documentation endpoint for swagger UI/redoc/rapidoc. I was uncertain if we should implement all of them, but did anyway.
    -A basic REST browser client UI written in Leptos

6/10/25

**This is a basic webserver written in Rust, with a sqlite database.**

# Currently working on Final submission: 

**Your course project is expected to be either a recipe service or a quote service, written in Axum with SQlite. You may have used SQLx or an ORM such as SeaORM. We will be looking for:

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

Please do ask on Zulip if you have any questions.**

# Steps to display single quote:

1) Download and unzip repository

2) Create Database and API calls. The joke database URI is `sqlite://db/knock-knock.db`, and there may be some issues with creating the db/ folder in the database. I have a .env file in the root, as well as a parser that has the DATABASE_URL, but the commands below sometimes cannot create a /db/ folder in the root by themselves. In that case, please create one yourself.

    Use commands:
    `sqlx database setup --source .\migrations\`

    `cargo sqlx prepare`

3) Use commands:

    `cargo build`

    `cargo run --release -- --init-from assets/static/quotes.json`

    *Make sure you run the --init-from assets/static/quotes.json upon initial setup!*


4) Don't close server running, and open browser to visit localhost port 3000:

    `http://127.0.0.1:3000/`

    or:

    `localhost:3000`

5) Stare in awe and wonder upon life's mysteries while looking at the quote. Take a second to appreciate yourself and your own journey. Or don't! After all, you are the master of your own fate: you are the captain of your soul.

Note: Server will 404 if no jokes in database right now

# TODO

1) ~~Get the random quote button to work on page, not to go to `http://localhost:3000/api/quotes/random?`~~

2) Get Leptos up and working

# Steps to create Database

I was having an incredibly hard time getting  `cargo sqlx migrate run` to work, but this seems to work on the command line:

`sqlx database setup --source .\migrations\`

`cargo sqlx prepare`

# Steps to use Swagger API

Run server with `cargo run --release`

Go to `http://localhost:3000/swagger-ui` and click around! you can try every API method inside it.

# Rapidoc

Run server with `cargo run --release`

Go to `http://localhost:3000/rapidoc` and try out the API!

# Redoc

Run server with `cargo run --release`

Go to `http://localhost:3000/redoc` and try out the API!

# How to use Leptos

run regular server in directory `quote-server` with `cargo run --release`

cd into `quote-server/leptos-frontend`

run `trunk serve --open`

__Marvel in wonder at uplifting quotes. This step is mandatory.__

# Problems

    Leptos is being a little squirrely, and I'm not sure I'm going to be able to get it working in time for submission at 11:00 on 6/11/25. Regular server seems to work pretty well.

# Misc info on second check in

My 2nd check in submission was done two days after the given due date of the 30th - It looks like somebody scraped and cloned my github repo on the 2nd of June, but I wanted to let you know that I needed a little more time to get it working!! Sorry!! I hope that two days after submission is not too, too late.