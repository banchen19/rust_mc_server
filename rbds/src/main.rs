
use std::sync::Arc;

use log::info;
use rbds_sdk::{Event, EventManager, IPlayer, Plugin, RBDSLogger, Server_Config};
use rust_raknet::RaknetListener;
use tokio::sync::Mutex;
use utils::get_files;
mod utils;

struct Server {
    config: Arc<Server_Config>,
    network: Mutex<Network>,
    event_manage: Mutex<EventManager>,
}

impl Server {
    async fn start(event_manage: Mutex<EventManager>) -> Self {
        let server = Self {
            config: Arc::new(Server_Config::def_config()),
            network: Mutex::new(Network::new(Server_Config::def_config()).await),
            event_manage,
        };
        server
            .event_manage
            .lock()
            .await
            .trigger_event(Event::Start)
            .await;
        server
    }
    async fn on_message(self) -> Self {
        self.network.lock().await.raknetlistener.listen().await;
        while let Ok(socket) = self.network.lock().await.raknetlistener.accept().await {
            let buf = socket.recv().await.unwrap();
            if buf[0] == 0xfe {
                // 玩家进入游戏，TODO: 首先是解包
                println!("玩家进入游戏{:?}", buf);
                let iplayer: IPlayer = IPlayer {
                    addr: socket.peer_addr().unwrap().to_string(),
                    datapacket: buf.clone(),
                };
                self.event_manage
                    .lock()
                    .await
                    .add_subscribe_listener(Event::AcceptDatePacket, iplayer)
                    .await;
                //触发事件
                self.event_manage
                    .lock()
                    .await
                    .trigger_event(Event::AcceptDatePacket)
                    .await;
            }
        }
        self
    }
}

struct Network {
    motd: Arc<String>,
    raknetlistener: RaknetListener,
}

impl Network {
    async fn new(server_confg: Server_Config) -> Network {
        let motd = server_confg.clone().getmotd();
        let ip_str = server_confg.get_ip_str().parse().unwrap();
        let mut network= Network{
            motd: Arc::new(motd.to_string()),
            raknetlistener:RaknetListener::bind(&ip_str).await.unwrap(),
        };
        let _ = network.raknetlistener.set_full_motd(network.motd.clone().to_string());
        network
    }
    
}

#[tokio::main]
async fn main() {
    let event_manage = Mutex::new(EventManager::default());
    event_manage
        .lock()
        .await
        .add_subscribe_listener(Event::Start, IPlayer::default())
        .await;
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
                .expect("Failed to load plugin")
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

    info!("获取当前事件");
    let players = event_manage
        .lock()
        .await
        .get_players_for_event(Event::Start)
        .await.expect("msg");
    info!("Vec：Player ：{:?}", players);

    //启动服务端
    let server = Server::start(event_manage.lock().await.clone().into()).await;
    server.on_message().await;
}
