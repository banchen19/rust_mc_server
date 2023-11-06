
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct IPlayer{
    pub addr:String,
    pub datapacket:Vec<u8>,
} 

impl IPlayer {
   pub fn default()->IPlayer {
        IPlayer{
            addr:"".to_owned(),
            datapacket:Vec::new()
        }
    }
}