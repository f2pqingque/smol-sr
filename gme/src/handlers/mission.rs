use msg::{GetMissionStatusCsReq, GetMissionStatusScRsp, Mission, MissionStatus};
use prost::Message;

pub async fn on_get_mission_status(req: Vec<u8>) -> Vec<u8> {
    let req: &[u8] = &req;
    let dec = GetMissionStatusCsReq::decode(req).unwrap();

    GetMissionStatusScRsp {
        finished_main_mission_id_list: dec.main_mission_id_list,
        sub_mission_status_list: dec
            .sub_mission_id_list
            .iter()
            .map(|id| Mission {
                id: *id,
                progress: 1,
                status: MissionStatus::MissionFinish.into(),
            })
            .collect(),
        ..Default::default()
    }
    .encode_to_vec()
}
