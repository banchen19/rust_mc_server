use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::net::SocketAddr;

#[derive(Debug, Eq, Hash, PartialEq)]
enum Event {
    Start,
    Stop,
}

#[derive(Default)]
struct EventManager {
    eventlist: Arc<Mutex<HashMap<Event, Vec<IPlayer>>>>, // 存储事件和相应的玩家列表
}

impl EventManager {
    // 添加事件监听，将玩家添加到指定事件下
    async fn add_subscribe_listener(&self, event: Event, player: IPlayer) {
        let mut eventlist = self.eventlist.lock().await; // 获取事件列表的可变引用
        eventlist.entry(event).or_insert(vec![]).push(player); // 如果事件不存在，创建并插入玩家；否则，将玩家添加到事件下
    }

    // 触发事件，对于指定事件，通知事件下的所有玩家
    async fn trigger_event(&self, event: Event) {
        let eventlist = self.eventlist.lock().await; // 获取事件列表的不可变引用
        if let Some(players) = eventlist.get(&event) {
            for player in players {
                // 处理事件，例如发送数据给玩家
                println!("Event triggered for player: {}", player.addr);
            }
        }
    }

    // 获取指定事件下的所有玩家
    async fn get_players_for_event(&self, event: Event) -> Option<Vec<IPlayer>> {
        let eventlist = self.eventlist.lock().await;
        if let Some(players) = eventlist.get(&event) {
            let cloned_players: Vec<IPlayer> = players.iter().cloned().collect();
            Some(cloned_players)
        } else {
            None
        }
    }
    
}