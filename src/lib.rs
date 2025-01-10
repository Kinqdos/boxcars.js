mod typescript;
mod utils;

use boxcars::{CrcCheck, NetworkParse, ParserBuilder};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = JSON, js_name = parse)]
    fn json_parse(text: String) -> JsValue;
}

#[wasm_bindgen]
pub struct BoxcarsParser {
    data: Box<[u8]>,
    crc_check: Option<CrcCheck>,
    network_parse: Option<NetworkParse>,
}

#[wasm_bindgen]
impl BoxcarsParser {
    #[wasm_bindgen(constructor)]
    pub fn new(data: Box<[u8]>) -> Self {
        BoxcarsParser {
            data,
            crc_check: None,
            network_parse: None,
        }
    }

    #[wasm_bindgen(js_name = "setCrcCheck")]
    pub fn set_crc_check(mut self, check: typescript::CrcCheck) -> Self {
        self.crc_check = Some(match check.as_string().unwrap().as_str() {
            "ALWAYS" => CrcCheck::Always,
            "NEVER" => CrcCheck::Never,
            "ON_ERROR" => CrcCheck::OnError,
            _ => panic!("invalid crc_check"),
        });
        self
    }

    #[wasm_bindgen(js_name = "setNetworkParse")]
    pub fn set_network_parse(mut self, parse: typescript::NetworkParse) -> Self {
        self.network_parse = Some(match parse.as_string().unwrap().as_str() {
            "ALWAYS" => NetworkParse::Always,
            "NEVER" => NetworkParse::Never,
            "IGNORE_ON_ERROR" => NetworkParse::IgnoreOnError,
            _ => panic!("invalid network parse"),
        });
        self
    }

    pub fn parse(self) -> typescript::Replay {
        let mut builder = ParserBuilder::new(&*self.data);
        if self.crc_check.is_some() {
            builder = builder.with_crc_check(self.crc_check.unwrap());
        };
        if self.network_parse.is_some() {
            builder = builder.with_network_parse(self.network_parse.unwrap());
        };
        let replay = builder.parse().unwrap();
        let json = serde_json::to_string(&replay).unwrap();
        json_parse(json).into()
    }
}
