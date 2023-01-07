use criterion::{black_box, criterion_group, criterion_main, Criterion};
use postmaster::prelude::{MessengerClient, MsgInTransit};
use tokio;
use tonic::{transport::Channel, Request};

async fn send_msg(client: &mut MessengerClient<Channel>) -> Result<(), tonic::Status> {
    let req = Request::new(MsgInTransit {
        sender: "prttyw1sh".into(),
        recipient: "Jane".into(),
        text: "Hello, Jane!".into(),
    });

    let _ = client.send_msg(req).await?.into_inner();

    Ok(())
}

fn criterion_benchmark(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let mut client = rt
        .block_on(MessengerClient::connect("http://[::1]:50051"))
        .unwrap();

    c.bench_function("send_msg", |b| {
        b.iter(|| rt.block_on(black_box(send_msg(&mut client))))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
