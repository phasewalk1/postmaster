
# ✉ postmaster [![CodeFactor](https://www.codefactor.io/repository/github/phasewalk1/postmaster/badge)](https://www.codefactor.io/repository/github/phasewalk1/postmaster) ![GitHub Workflow Status (branch)](https://img.shields.io/github/workflow/status/phasewalk1/sb-messenger/Quickstart/main)
>>> checking, checking!

## Workflow

__Integration tests__

```bash
cargo forge
```
This will ensure all conversion methods from Diesel to PROST!, and vice versa, are implemented correctly.

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
