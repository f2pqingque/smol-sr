use crate::util;
use msg::{PlayerBasicInfo, PlayerGetTokenScRsp, PlayerLoginFinishScRsp, PlayerLoginScRsp};
use prost::Message;

pub fn on_player_get_token(_req: Vec<u8>) -> Vec<u8> {
    PlayerGetTokenScRsp {
        msg: String::from("OK"),
        retcode: 0,
        uid: 1,
        ..Default::default()
    }
    .encode_to_vec()
}

pub fn on_player_login(_req: Vec<u8>) -> Vec<u8> {
    PlayerLoginScRsp {
        basic_info: Some(PlayerBasicInfo {
            nickname: String::from("smol"),
            level: 10,
            exp: 0,
            stamina: 240,
            mcoin: 1,
            hcoin: 1,
            scoin: 1,
            world_level: 1,
        }),
        server_timestamp_ms: util::cur_timestamp_ms(),
        retcode: 0,
        stamina: 240,
        ..Default::default()
    }
    .encode_to_vec()
}

pub fn on_player_login_finish(_req: Vec<u8>) -> Vec<u8> {
    PlayerLoginFinishScRsp { retcode: 0 }.encode_to_vec()
}
