
# ✉ postmaster [![CodeFactor](https://www.codefactor.io/repository/github/phasewalk1/postmaster/badge)](https://www.codefactor.io/repository/github/phasewalk1/postmaster) ![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/phasewalk1/postmaster/quickstart.yml)
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white) ![Postgres](https://img.shields.io/badge/postgres-%23316192.svg?style=for-the-badge&logo=postgresql&logoColor=white)
>>> checking, checking!

## Workflow

__Integration tests__

Before running our integration tests, we need to set up the environment. Bash scripts are included for this purpose. We can quickly and easily spin up our docker container by running:
```bash
bash scripts/docker_spin.sh
```
Once we have the local postgres container running, we can launch the service (both the gRPC server and the HTTP server) with the following script:
```bash
bash scripts/launch_service.sh
```
Under the hood, this script will first source the environment variables from 'scripts/environ.sh', which will set 'RUST_LOG', 'DATABASE_URL', and a few other relevant environment variables for development.

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



## API Reference

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
