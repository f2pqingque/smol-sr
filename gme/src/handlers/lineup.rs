use msg::{
    AmountInfo, AvatarType, GetAllLineupDataScRsp, GetCurLineupDataScRsp, LineupAvatar, LineupInfo,
};
use prost::Message;

pub async fn on_get_cur_lineup(_req: Vec<u8>) -> Vec<u8> {
    GetCurLineupDataScRsp {
        retcode: 0,
        lineup: Some(LineupInfo {
            name: String::from("smolteam"),
            avatar_list: vec![LineupAvatar {
                id: 1201,
                hp: 10000,
                slot_type: 0,
                satiety: 0,
                sp: Some(AmountInfo {
                    cur_amount: 0,
                    max_amount: 10000,
                }),
                avatar_type: AvatarType::AvatarFormalType.into(),
            }],
            plane_id: 20101,
            ..Default::default()
        }),
    }
    .encode_to_vec()
}

pub async fn on_get_all_lineup_data(_req: Vec<u8>) -> Vec<u8> {
    GetAllLineupDataScRsp {
        lineup_list: vec![LineupInfo {
            name: String::from("smolteam"),
            avatar_list: vec![LineupAvatar {
                id: 1201,
                hp: 10000,
                slot_type: 0,
                satiety: 0,
                sp: Some(AmountInfo {
                    cur_amount: 0,
                    max_amount: 10000,
                }),
                avatar_type: AvatarType::AvatarFormalType.into(),
            }],
            plane_id: 20101,
            ..Default::default()
        }],
        cur_index: 0,
        retcode: 0,
    }
    .encode_to_vec()
}
