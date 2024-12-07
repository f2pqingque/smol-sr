use crate::util;
use msg::{GetBasicInfoScRsp, PlayerHeartBeatCsReq, PlayerHeartBeatScRsp};
use prost::Message;

pub async fn on_player_heart_beat(req: Vec<u8>) -> Vec<u8> {
    let req: &[u8] = &req;
    let dec = PlayerHeartBeatCsReq::decode(req).unwrap();

    PlayerHeartBeatScRsp {
        download_data: None,
        client_time_ms: dec.client_time_ms,
        server_time_ms: util::cur_timestamp_ms(),
        retcode: 0,
    }
    .encode_to_vec()
}

pub async fn on_get_basic_info(_req: Vec<u8>) -> Vec<u8> {
    GetBasicInfoScRsp {
        retcode: 0,
        is_gender_set: true,
        gender: 0,
        ..Default::default()
    }
    .encode_to_vec()
}
