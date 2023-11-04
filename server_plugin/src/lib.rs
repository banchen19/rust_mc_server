use rbds_sdk::Plugin;

#[no_mangle]
pub extern "C" fn postint() -> Plugin {
    Plugin{
        name: "死亡掉落保护2".to_owned(),
        version: (0,0,1),
        authors: "作者信息".to_owned(),
        repository: "无仓库".to_owned(),
        keywords: vec!["死亡".to_string()],
    }
    
}

#[no_mangle]
pub extern "C" fn on_event() ->  &'static str {
    "获取事件"
}