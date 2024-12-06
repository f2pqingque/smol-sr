use ohkami::{Ohkami, Route};

include!("../out/_.rs");

async fn mdk_shield() -> String {
    String::from(MDK_SHIELD)
}

async fn login_granter() -> String {
    String::from(LOGIN_GRANTER)
}

async fn risky_api_check() -> String {
    String::from(RISKY_API_CHECK)
}

async fn query_dispatch() -> String {
    String::from(QUERY_DISPATCH)
}

async fn query_gateway() -> String {
    String::from(QUERY_GATEWAY)
}

fn main() {
    smol::block_on(async {
        Ohkami::new((
            "/query_dispatch".GET(query_dispatch),
            "/query_gateway".GET(query_gateway),
            "/account/risky/api/check".POST(risky_api_check),
            "/hkrpg_global/mdk/shield/api/login".POST(mdk_shield),
            "/hkrpg_global/mdk/shield/api/verify".POST(mdk_shield),
            "/hkrpg_global/combo/granter/login/v2/login".POST(login_granter),
        ))
        .howl("127.0.0.1:21000")
        .await
    })
}
