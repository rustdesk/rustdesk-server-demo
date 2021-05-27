use hbb_common::{
    protobuf::Message as _,
    rendezvous_proto::*,
    tcp::{new_listener, FramedStream},
    tokio,
    udp::FramedSocket,
};

#[tokio::main(basic_scheduler)]
async fn main() {
    let mut socket = FramedSocket::new("0.0.0.0:21116").await.unwrap();
    let mut listener = new_listener("0.0.0.0:21116", false).await.unwrap();
    let mut rlistener = new_listener("0.0.0.0:21117", false).await.unwrap();
    let mut id_map = std::collections::HashMap::new();
    let relay_server = std::env::var("IP").unwrap();
    let mut saved_stream = None;
    loop {
        tokio::select! {
            Some(Ok((bytes, addr))) = socket.next() => {
                if let Ok(msg_in) = RendezvousMessage::parse_from_bytes(&bytes) {
                    match msg_in.union {
                        Some(rendezvous_message::Union::register_peer(rp)) => {
                            id_map.insert(rp.id, addr);
                        }
                        Some(rendezvous_message::Union::register_pk(_)) => {
                            let mut msg_out = RendezvousMessage::new();
                            msg_out.set_register_pk_response(RegisterPkResponse {
                                result: register_pk_response::Result::OK.into(),
                                ..Default::default()
                            });
                            socket.send(&msg_out, addr).await.ok();
                        }
                        _ => {}
                    }
                }
            }
            Ok((stream, _)) = listener.accept() => {
                let mut stream = FramedStream::from(stream);
                if let Some(Ok(bytes)) = stream.next_timeout(3000).await {
                    if let Ok(msg_in) = RendezvousMessage::parse_from_bytes(&bytes) {
                        match msg_in.union {
                            Some(rendezvous_message::Union::punch_hole_request(ph)) => {
                                if let Some(addr) = id_map.get(&ph.id) {
                                    let mut msg_out = RendezvousMessage::new();
                                    msg_out.set_request_relay(RequestRelay {
                                        relay_server: relay_server.clone(),
                                        ..Default::default()
                                    });
                                    socket.send(&msg_out, addr.clone()).await.ok();
                                    saved_stream = Some(stream);
                                }
                            }
                            Some(rendezvous_message::Union::relay_response(_)) => {
                                let mut msg_out = RendezvousMessage::new();
                                msg_out.set_relay_response(RelayResponse {
                                    relay_server: relay_server.clone(),
                                    ..Default::default()
                                });
                                if let Some(mut stream) = saved_stream.take() {
                                    stream.send(&msg_out).await.ok();
                                    if let Ok((stream_a, _)) = rlistener.accept().await {
                                        let mut stream_a = FramedStream::from(stream_a);
                                        stream_a.next_timeout(3_000).await;
                                        if let Ok((stream_b, _)) = rlistener.accept().await {
                                            let mut stream_b = FramedStream::from(stream_b);
                                            stream_b.next_timeout(3_000).await;
                                            relay(stream_a, stream_b).await;
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}

async fn relay(stream: FramedStream, peer: FramedStream) {
    let mut peer = peer;
    let mut stream = stream;
    peer.set_raw();
    stream.set_raw();
    loop {
        tokio::select! {
            res = peer.next() => {
                if let Some(Ok(bytes)) = res {
                    stream.send_bytes(bytes.into()).await.ok();
                } else {
                    break;
                }
            },
            res = stream.next() => {
                if let Some(Ok(bytes)) = res {
                    peer.send_bytes(bytes.into()).await.ok();
                } else {
                    break;
                }
            },
        }
    }
}
