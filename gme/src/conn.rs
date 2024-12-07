use crate::handlers;
use crate::pk;
use msg::*;

macro_rules! dummy_handle {
    ($rsp_cmd:expr) => {
        pk::encode_packet($rsp_cmd, Vec::with_capacity(0))
    };
}

macro_rules! handle_packet {
    ($body:expr, $rsp_cmd:expr, $handler:path) => {
        pk::encode_packet($rsp_cmd, $handler($body))
    };
}

use handlers::{avatar, lineup, login, mission, player, scene};

pub fn ping_pong(cmd: u16, body: Vec<u8>) -> Vec<u8> {
    match cmd {
        PLAYER_GET_TOKEN_CS_REQ => {
            handle_packet!(body, PLAYER_GET_TOKEN_SC_RSP, login::on_player_get_token)
        }
        PLAYER_LOGIN_CS_REQ => {
            handle_packet!(body, PLAYER_LOGIN_SC_RSP, login::on_player_login)
        }
        PLAYER_LOGIN_FINISH_CS_REQ => {
            handle_packet!(
                body,
                PLAYER_LOGIN_FINISH_SC_RSP,
                login::on_player_login_finish
            )
        }
        PLAYER_HEART_BEAT_CS_REQ => {
            handle_packet!(body, PLAYER_HEART_BEAT_SC_RSP, player::on_player_heart_beat)
        }
        GET_BASIC_INFO_CS_REQ => {
            handle_packet!(body, GET_BASIC_INFO_SC_RSP, player::on_get_basic_info)
        }
        GET_AVATAR_DATA_CS_REQ => {
            handle_packet!(body, GET_AVATAR_DATA_SC_RSP, avatar::on_get_avatar_data)
        }
        GET_MULTI_PATH_AVATAR_INFO_CS_REQ => {
            dummy_handle!(GET_MULTI_PATH_AVATAR_INFO_SC_RSP)
        }
        GET_BAG_CS_REQ => {
            dummy_handle!(GET_BAG_SC_RSP)
        }
        GET_MISSION_STATUS_CS_REQ => {
            handle_packet!(
                body,
                GET_MISSION_STATUS_SC_RSP,
                mission::on_get_mission_status
            )
        }
        GET_CUR_LINEUP_DATA_CS_REQ => {
            handle_packet!(body, GET_CUR_LINEUP_DATA_SC_RSP, lineup::on_get_cur_lineup)
        }
        GET_ALL_LINEUP_DATA_CS_REQ => {
            handle_packet!(
                body,
                GET_ALL_LINEUP_DATA_SC_RSP,
                lineup::on_get_all_lineup_data
            )
        }
        GET_CUR_SCENE_INFO_CS_REQ => {
            handle_packet!(
                body,
                GET_CUR_SCENE_INFO_SC_RSP,
                scene::on_get_cur_scene_info
            )
        }
        _ => Vec::with_capacity(0),
    }
}
