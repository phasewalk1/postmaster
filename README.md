
# ✉ sb-messenger  ![GitHub Workflow Status (branch)](https://img.shields.io/github/workflow/status/phasewalk1/sb-messenger/Quickstart/main)

This is the gRPC server for the SB-Messenger service. It is responsible for handling client requests and storing/retrieving data from the database. Some methods implement server-side streaming, while others are unary. The server is implemented using the Tonic library, which provides a gRPC server implementation on top of the Tokio runtime.


## Workflow

To run integration tests against a client, run the following command

```bash
cargo forge
```

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

