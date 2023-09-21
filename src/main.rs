use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};

// 异步
use std::{
    sync::{Arc, Mutex},
    thread,
};

//彩色log输出?
use colored::*;


// io处理

use std::io;

// 配置
mod config;
use config::server_config::{inti_config, Config};

mod network;
use network::udp_server::start_udp_server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    inti_play().await;
    Ok(())
}

async fn inti_play() {
    match inti_config() {
        Ok(config) => {
            let ws_server_task: tokio::task::JoinHandle<tokio::task::JoinHandle<()>> =
                tokio::spawn(start_udp_server(config));
            let _handle_input: () = handle_input(ws_server_task).await.to_owned();
        }
        Err(err) => {
            println!("{}", "配置文件与程序冲突，请使用最新的配置文件运行");
        }
    }
    // 等待线程完成
    thread::sleep(std::time::Duration::from_secs(1));
}

// 启动网络服务
//server



//命令
async fn handle_input(start_server_udp_task: tokio::task::JoinHandle<tokio::task::JoinHandle<()>>) {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("请输入正确指令");
        // 去除输入字符串两端的空格和换行符
        let command = input.trim();
        match command {
            "stop" => {
                start_server_udp_task.abort();
                println!("{}", "server_be: stop".red());
                std::process::exit(0);
            }
            _ => {}
        }
    }
}
