use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "CrcCheck")]
    pub type CrcCheck;

    #[wasm_bindgen(typescript_type = "NetworkParse")]
    pub type NetworkParse;

    #[wasm_bindgen(typescript_type = "Replay")]
    pub type Replay;
}

#[wasm_bindgen(typescript_custom_section)]
pub const CRC_CHECK_ENUM: &'static str = r#"
export enum CrcCheck {
    ALWAYS = "ALWAYS",
    NEVER = "NEVER",
    ON_ERROR = "ON_ERROR"
}
"#;

#[wasm_bindgen(typescript_custom_section)]
pub const NETWORK_PARSE_ENUM: &'static str = r#"
export enum NetworkParse {
  ALWAYS = "ALWAYS",
  NEVER = "NEVER",
  IGNORE_ON_ERROR = "IGNORE_ON_ERROR"
}
"#;
