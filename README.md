
# ✉ postmaster [![CodeFactor](https://www.codefactor.io/repository/github/phasewalk1/postmaster/badge)](https://www.codefactor.io/repository/github/phasewalk1/postmaster) ![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/phasewalk1/postmaster/quickstart.yml)
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white) ![Postgres](https://img.shields.io/badge/postgres-%23316192.svg?style=for-the-badge&logo=postgresql&logoColor=white)
>>> checking, checking!

# About
`postmaster` is a microservice that enables messenging between users on the SB music platform, a home for producers and artists alike where they can share works and find each other. The messenging service serves a simple role in allowing users to reach out to one another to discuss their services and or products. The `postmaster` service is implemented both in gRPC and HTTP. The gRPC server provides an interface for other services to communicate with `postmaster` through use of protobuf, although primarily, the service will be used through the main HTTP gateway of SB. The backend of `postmaster` communicates with a PostgreSQL database, and a default docker container is provided for development and testing purposes.

## Workflow

__Running in production__
To run the service in production, we only need to set the `DATABASE_URL` environment variable. This should point to the production data store that holds message data. Once that's set, we can spin up the service via the following:
```bash
bash scripts/launch_service.sh --prod
```

__Integration tests__

Before running our integration tests, we need to set up the environment. Bash scripts are included for this purpose. We can quickly and easily spin up our docker container by running:
```bash
bash scripts/docker_spin.sh
```
Once we have the local postgres container running, we can launch the service (both the gRPC server and the HTTP server) in _development mode_ by running the same `launch_service` script used for production (but this time, with no arguments passed at runtime):
```bash
bash scripts/launch_service.sh
```
Under the hood, this script will first source the environment variables from `scripts/environ.sh`, which will set `RUST_LOG`, `ROCKET_PORT`, `TONIC_PORT`, as well as pointing `DATABASE_URL` to the local docker container.

__Running integration tests on the service as a whole__

```bash
cargo forge
```
This will ensure all conversion methods from Diesel to PROST!, and vice versa, are implemented correctly, as well as running ALL tests from all 3 crates in the workspace.

__Integration tests on individual crates__
```bash
cargo forge-grpc
```
```bash
cargo forge-http
```

__Viewing the docs for the core library__
```bash
cargo view
```

__Running the HTTP server__
```bash
cargo serve
```
__gRPC server__
```bash
cargo interop
```

# API Reference

#### Send a Msg

```Rust
  client.send_msg(<msg>)
```

| Parameter | Type     | Description                |
| :-------- | :------- | :------------------------- |
| `request` | `Msg` | **Required**. The Protobuf encoded Msg to send |

#### Get Msg by message_id

```Rust
  client.get_msg(<message_id>)
```

| Parameter | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| `message_id`      | `string` | **Required**. id of Msg to fetch |

#### Get All Msgs Sent by User

```Rust
  client.get_sent_msgs(<sender>)
```

| Parameter | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| `sender`      | `string` | **Required**. the id of the sender whose Msgs to fetch |

#### Get all Msgs Received by User

```Rust
  client.get_received_msgs(<recipient>)
```

| Parameter | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| `recipient`      | `string` | **Required**. the id of the recipient whose Msgs to fetch |
