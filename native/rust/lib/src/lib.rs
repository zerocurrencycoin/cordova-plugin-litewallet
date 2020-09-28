#[macro_use]
extern crate lazy_static;

use std::sync::{Mutex, Arc};
use std::cell::RefCell;

use base64::{encode, decode};
use bip39::{Mnemonic, Language};
use rand::{Rng, rngs::OsRng};

extern crate serde;
#[macro_use] extern crate serde_json;
extern crate serde_derive;
use serde_derive::Deserialize;

use zecwalletlitelib::{commands, lightclient::{LightClient, LightClientConfig}};


#[derive(Deserialize, Clone)]
pub struct JsAddressParameters {
    pub coin_type: String,
    pub hrp_sapling_extended_spending_key: String,
    pub hrp_sapling_extended_full_viewing_key: String,
    pub hrp_sapling_payment_address: String,
    #[serde(with = "hex_serde")]
    pub b58_pubkey_address_prefix: [u8; 2],
    #[serde(with = "hex_serde")]
    pub b58_script_address_prefix: [u8; 2],
}


pub fn set_address_params(mut config: LightClientConfig, params: &str) -> LightClientConfig {

    let js_params: JsAddressParameters = match serde_json::from_str(&params) {
        Ok(js) => js,
        Err(_) => {return config}
    };

    let ct: u32 = match js_params.coin_type.parse() {
        Ok(s) => s,
        Err(_) => return config
    };

    if js_params.hrp_sapling_extended_spending_key.len() == 0 {return config}
    if js_params.hrp_sapling_extended_full_viewing_key.len() == 0 {return config}
    if js_params.hrp_sapling_payment_address.len() == 0 {return config}
    if js_params.b58_pubkey_address_prefix.len() != 2 {return config}
    if js_params.b58_script_address_prefix.len() != 2 {return config}

    LightClientConfig::set_coin_type(&mut config, ct);
    LightClientConfig::set_hrp_sapling_extended_spending_key(&mut config, js_params.hrp_sapling_extended_spending_key);
    LightClientConfig::set_hrp_sapling_extended_full_viewing_key(&mut config, js_params.hrp_sapling_extended_full_viewing_key);
    LightClientConfig::set_hrp_sapling_payment_address(&mut config, js_params.hrp_sapling_payment_address);
    LightClientConfig::set_b58_pubkey_address_prefix(&mut config, js_params.b58_pubkey_address_prefix);
    LightClientConfig::set_b58_script_address_prefix(&mut config, js_params.b58_pubkey_address_prefix);

    config
}


// We'll use a MUTEX to store a global lightclient instance,
// so we don't have to keep creating it. We need to store it here, in rust
// because we can't return such a complex structure back to JS
lazy_static! {
    static ref LIGHTCLIENT: Mutex<RefCell<Option<Arc<LightClient>>>> = Mutex::new(RefCell::new(None));
}
pub fn init_new(server_uri: String, params: String, sapling_output_b64: String, sapling_spend_b64: String) -> String {
    let server = LightClientConfig::get_server_or_default(Some(server_uri));
    let (mut config, latest_block_height) = match LightClientConfig::create(server) {
        Ok((c, h)) => (c, h),
        Err(e) => {
            return format!("Error: {}", e);
        }
    };

    //Set Encoding Specifications
    config = set_address_params(config, params.as_str());

    let lightclient = match LightClient::new(&config, latest_block_height) {
        Ok(mut l) => {
            match l.set_sapling_params(&decode(&sapling_output_b64).unwrap(), &decode(&sapling_spend_b64).unwrap()) {
                Ok(_) => l,
                Err(e) => return format!("Error: {}", e)
            }
        },
        Err(e) => {
            return format!("Error: {}", e);
        }
    };

    let seed = match lightclient.do_seed_phrase() {
        Ok(s) => s.dump(),
        Err(e) => {
            return format!("Error: {}", e);
        }
    };

    LIGHTCLIENT.lock().unwrap().replace(Some(Arc::new(lightclient)));

    seed
}

pub fn init_from_seed(server_uri: String, params: String, seed: String, birthday: u64, sapling_output_b64: String, sapling_spend_b64: String) -> String {
    let server = LightClientConfig::get_server_or_default(Some(server_uri));
    let (mut config, _latest_block_height) = match LightClientConfig::create(server) {
        Ok((c, h)) => (c, h),
        Err(e) => {
            return format!("Error: {}", e);
        }
    };

    //Set Encoding Specifications
    config = set_address_params(config, params.as_str());

    let lightclient = match LightClient::new_from_phrase(seed, &config, birthday, false) {
        Ok(mut l) => {
            match l.set_sapling_params(&decode(&sapling_output_b64).unwrap(), &decode(&sapling_spend_b64).unwrap()) {
                Ok(_) => l,
                Err(e) => return format!("Error: {}", e)
            }
        },
        Err(e) => {
            return format!("Error: {}", e);
        }
    };

    let seed = match lightclient.do_seed_phrase() {
        Ok(s) => s.dump(),
        Err(e) => {
            return format!("Error: {}", e);
        }
    };

    LIGHTCLIENT.lock().unwrap().replace(Some(Arc::new(lightclient)));

    seed
}

pub fn init_from_b64(server_uri: String, params: String, base64_data: String, sapling_output_b64: String, sapling_spend_b64: String) -> String {
    let server = LightClientConfig::get_server_or_default(Some(server_uri));
    let (mut config, _latest_block_height) = match LightClientConfig::create(server) {
        Ok((c, h)) => (c, h),
        Err(e) => {
            return format!("Error: {}", e);
        }
    };

    //Set Encoding Specifications
    config = set_address_params(config, params.as_str());

    let decoded_bytes = match decode(&base64_data) {
        Ok(b) => b,
        Err(e) => { return format!("Error: Decoding Base64: {}", e); }
    };

    let lightclient = match LightClient::read_from_buffer(&config, &decoded_bytes[..]) {
        Ok(mut l) => {
            match l.set_sapling_params(&decode(&sapling_output_b64).unwrap(), &decode(&sapling_spend_b64).unwrap()) {
                Ok(_) => l,
                Err(e) => return format!("Error: {}", e)
            }
        },
        Err(e) => {
            return format!("Error: {}", e);
        }
    };

    let seed = match lightclient.do_seed_phrase() {
        Ok(s) => s.dump(),
        Err(e) => {
            return format!("Error: {}", e);
        }
    };

    LIGHTCLIENT.lock().unwrap().replace(Some(Arc::new(lightclient)));

    seed
}

pub fn save_to_b64() -> String {
    // Return the wallet as a base64 encoded string
    let lightclient: Arc<LightClient>;
    {
        let lc = LIGHTCLIENT.lock().unwrap();

        if lc.borrow().is_none() {
            return format!("Error: Light Client is not initialized");
        }

        lightclient = lc.borrow().as_ref().unwrap().clone();
    };

    match lightclient.do_save_to_buffer() {
        Ok(buf) => encode(&buf),
        Err(e) => {
            format!("Error: {}", e)
        }
    }
}

pub fn execute(cmd: String, args_list: String) -> String {
    let resp: String;
    {
        let lightclient: Arc<LightClient>;
        {
            let lc = LIGHTCLIENT.lock().unwrap();

            if lc.borrow().is_none() {
                return format!("Error: Light Client is not initialized");
            }

            lightclient = lc.borrow().as_ref().unwrap().clone();
        };

        let args = if args_list.is_empty() { vec![] } else { vec![args_list.as_ref()] };
        resp = commands::do_user_command(&cmd, &args, lightclient.as_ref()).clone();
    };

    resp
}

pub fn check_seed_phrase(seed_phrase: &str) ->String {
    match Mnemonic::from_phrase(seed_phrase.to_string(), Language::English) {
        Ok(_) => {
            let data = json!({"checkSeedPhrase": "Ok"});
            return serde_json::to_string(&data).unwrap()
        },
        Err(_) => {
            let data = json!({"checkSeedPhrase": "Error"});
            return serde_json::to_string(&data).unwrap()
        }
    };
}

pub fn get_seed_phrase() -> String {

    let mut seed_bytes = [0u8; 32];
    let mut system_rng = OsRng;
            system_rng.fill(&mut seed_bytes);

    let data = json!({
        "seedPhrase": Mnemonic::from_entropy(&seed_bytes,Language::English,).unwrap().phrase().to_string()
    });

    serde_json::to_string(&data).unwrap()
}
