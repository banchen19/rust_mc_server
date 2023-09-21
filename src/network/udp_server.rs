use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use rust_raknet::{RaknetListener, RaknetSocket, Reliability};

use crate::config::{self, server_config::Config};

pub async fn start_udp_server(config: Config) -> tokio::task::JoinHandle<()> {
    let start_server_udp_task: tokio::task::JoinHandle<()> = tokio::spawn(async move {
        let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), config.ipv4_port);
        let mut server = RaknetListener::bind(&socket).await.expect("端口被占用");
        server
            .set_motd(
                &config.server_name,
                config.player_max,
                "495",
                &config.version,
                &config.gamemode,
                config.ipv4_port,
            )
            .await;
        server.listen().await;
        loop {
            let socket = server.accept().await.unwrap();
            let buf = socket.recv().await.unwrap();
            if buf[0] == 0xfe {
                //do something
            }
            println!("客户端地址{:?}", socket.peer_addr());

            let socket = RaknetSocket::connect(&socket.peer_addr().unwrap())
                .await
                .unwrap();

            println!("收到客户端发送的消息{:?}", buf);
            
        }
    });
    start_server_udp_task
}
