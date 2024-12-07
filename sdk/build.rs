use msg::{Dispatch, GateServer, RegionInfo};
use prost::Message;
use serde_json::json;
use std::fs::{create_dir_all, write};
use std::path::Path;

const OUTPUT_DIR: &str = "out/";
const OUTPUT_FILE: &str = "_.rs";

fn main() {
    if !Path::new(OUTPUT_DIR).exists() {
        create_dir_all(OUTPUT_DIR).expect("Failed to create output directory");
    }

    let mdk_shield = json!({
        "data": {
            "account": {
                "area_code": "**",
                "email": "yuvlian@naver.com",
                "country": "ID",
                "is_email_verify": "1",
                "token": "x",
                "uid": "1"
            },
            "device_grant_required": false,
            "reactivate_required": false,
            "realperson_required": false,
            "safe_mobile_required": false
        },
        "message": "OK",
        "retcode": 0
    });

    let login_granter = json!({
        "data": {
            "account_type": 1,
            "combo_id": "1",
            "combo_token": "x",
            "data": "{\"guest\":false}",
            "heartbeat": false,
            "open_id": "1"
        },
        "message": "OK",
        "retcode": 0
    });

    let risky_api_check = json!({
        "data": {},
        "message": "OK",
        "retcode": 0
    });

    let query_dispatch = rbase64::encode(
        &Dispatch {
            retcode: 0,
            msg: String::from("OK"),
            region_list: vec![RegionInfo {
                name: String::from("smol"),
                display_name: String::from("smol"),
                title: String::from("smol"),
                env_type: String::from("2"),
                msg: String::from("OK"),
                dispatch_url: String::from("http://127.0.0.1:21000/query_gateway"),
            }],
            ..Default::default()
        }
        .encode_to_vec(),
    );

    let query_gateway = rbase64::encode(
        &GateServer {
            retcode: 0,
            ip: String::from("127.0.0.1"),
            port: 7000,
            // epic bruteforce
            use_tcp: true,
            watermark_enable: true,
            enable_video_bundle_version_update: true,
            close_redeem_code: true,
            forbid_recharge: true,
            enable_design_data_bundle_version_update: true,
            network_diagnostic: true,
            android_middle_package_enable: true,
            event_tracking_open: true,
            enable_save_replay_file: true,
            enable_upload_battle_log: true,
            ejcaokobhbg: true,
            nhehajgmjnj: true,
            ios_exam: true,
            mtp_switch: false,
            ..Default::default()
        }
        .encode_to_vec(),
    );    

    let constants = format!(
        "const MDK_SHIELD: &str = r#\"{}\"#;\n\
         const LOGIN_GRANTER: &str = r#\"{}\"#;\n\
         const RISKY_API_CHECK: &str = r#\"{}\"#;\n\
         const QUERY_DISPATCH: &str = r#\"{}\"#;\n\
         const QUERY_GATEWAY: &str = r#\"{}\"#;\n",
        mdk_shield, login_granter, risky_api_check, query_dispatch, query_gateway
    );

    let output_path = format!("{}{}", OUTPUT_DIR, OUTPUT_FILE);
    write(&output_path, constants).expect("Failed to write output file");
}
