
# ✉ sb-messenger  [![CodeFactor](https://www.codefactor.io/repository/github/phasewalk1/sb-messenger/badge)](https://www.codefactor.io/repository/github/phasewalk1/sb-messenger) ![GitHub Workflow Status (branch)](https://img.shields.io/github/workflow/status/phasewalk1/sb-messenger/Quickstart/main)

This is the monorepo for the SB-Messenger service. It is responsible for handling messages in transit and storing/retrieving messages from the database. Some of the RPC methods implement server-side streaming, while others are unary. The gRPC server is implemented using the Tonic library, which provides a gRPC server implementation on top of the Tokio runtime. The HTTP server is implemented using the Rocket framework. Both servers share common protobuf schemas and diesel structs for composability.


## Workflow

To run integration tests against a gRPC client, run the following command.

```bash
cargo forge
```
This will ensure all conversion methods from Diesel to PROST!, and vice versa, are implemented correctly.

To view the documentation for the server implementation, run
```bash
cargo view
```

You can run the server by issuing the following,
```bash
cargo serve
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

