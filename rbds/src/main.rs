use std::{collections::HashMap, sync::Arc};

use log::info;
use parallel_event_emitter::ParallelEventEmitter;
use rbds_sdk::{IPlayer, Plugin, RBDSLogger, Server_Config};
use rust_raknet::RaknetListener;
use tokio::sync::Mutex;
use utils::get_files;
mod utils;

struct Server {
    config: Arc<Server_Config>,
    network: Arc<Network>,
    players: HashMap<IPlayer, Vec<u8>>,
}

impl Server {
    async fn start() -> Self {
        let server = Self {
            config: Arc::new(Server_Config::def_config()),
            network: Arc::new(Network::new(Server_Config::def_config()).await),
            players: HashMap::new(),
        };

        server
    }
    async fn on_message(mut self) -> Self {
        self.network.raknetlistener.lock().await.listen().await;
        while let Ok(socket) = self.network.raknetlistener.lock().await.accept().await {
            let buf = socket.recv().await.unwrap();
            if buf[0] == 0xfe {
                // 玩家进入游戏，TODO: 首先是解包
                println!("玩家进入游戏{:?}", buf);
                let iplayer: IPlayer = IPlayer {
                    addr: socket.peer_addr().unwrap(),
                    datapacket: buf.clone(),
                };
                //触发事件
            }
        }
        self
    }
}

struct Network {
    motd: Arc<Mutex<String>>,
    raknetlistener: Mutex<RaknetListener>,
}

impl Network {
    async fn new(server_confg: Server_Config) -> Network {
        let motd = server_confg.clone().getmotd();
        let ip_str = server_confg.get_ip_str();
        Self {
            motd: Arc::new(Mutex::new(motd.to_string())),
            raknetlistener: Mutex::new(RaknetListener::bind(&ip_str).await.unwrap()),
        }
    }
}


#[tokio::main]
async fn main() {
    
    

    //日志初始化
    let _ = RBDSLogger::init_log();
    //插件初始化
    let mut _plugins_list = Vec::new();

    let plugin_path = "./plugins/".to_owned();

    for plugin_name in get_files(plugin_path.clone()) {
        info!("检测到插件：{}", plugin_name);
        // 加载插件 DLL
        let lib = unsafe {
            libloading::Library::new(plugin_path.clone() + &plugin_name)
                .expect("Failed to load DLL")
        };
        // 获取插件函数指针
        unsafe {
            let postint: libloading::Symbol<unsafe extern "C" fn() -> Plugin> =
                lib.get(b"postint\0").expect("Failed to find function");
            // 累计插件主函数
            let plugin = postint();
            info!("加载插件：{}", plugin.name);
            _plugins_list.push(postint());
        }
    }
    //插件获取完毕
}
