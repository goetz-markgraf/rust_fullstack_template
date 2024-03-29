# Rust Fullstack Template

This repository contains a template for a fullstack Rust application.
It is based on these technologies:

- [leptos](https://leptos.dev) for the `frontend`, including:
    - reqwest for HTTP requests
    - serde and serde_json for JSON serialization
    - anyhow to easily handle errors
- [rocket](https://rocket.rs) for the `backend`, including:
    - serde and serde_json for JSON serialization

Additionally, the project contains a `shared` library that is used by both the `frontend` and the `backend`.
In this library you can define shared data structures, like data transfer objects, and functions.
It also uses serde and serde_json for JSON serialization.

The whole project it configured as one workspace containing all three projects.

# install nightly rust toolchain and wasm target

The frontend uses nightly features and compiles to WebAssembly, so you have to install the nightly toolchain and the
wasm target, if you haven't already.

```sh
rustup toolchain install nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

The same commands can be used to update the toolchain to the newest version.

# set workspace to nightly build

The frontend uses nightly features, so you have to set the workspace to nightly build.

```sh
rustup override set nightly
```

# how to run

You have to start the frontend and the backend separately.

## run backend

Go into the `backend` directory and run the following command:

```sh
cd backend
cargo run
```

It will launch the backend using the Rocket library. The default port is 8000.

If you want hot reloading, you can run the backend with the following command:

```sh
cd backend
cargo watch -x run
```

## run frontend

Go into the `frontend` directory and run the following command:

```sh
cd frontend
trunk serve
```

It will launch the development server for the frontend using the leptos library. The default port is 8080.
In the file `Trunk.toml`, the proxy for the dev server is configured such that every
request to `/api` is forwarded to `http://localhost:8000/api`.

The frontend is run with hot reload.

Open the browser and go to [`http://localhost:8080`](http:localhost:8080).
You should see the text "Hello from backend" that is fetched from the backend via an HTTP request.
