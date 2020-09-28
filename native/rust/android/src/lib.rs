#![cfg(target_os="android")]
#![allow(non_snake_case)]

//#[macro_use] extern crate log;
extern crate android_logger;

use log::Level;
use android_logger::Config;

use std::ffi::{CString, CStr};
use jni::JNIEnv;
use jni::objects::{JObject, JString};
use jni::sys::{jstring};


#[no_mangle]
pub unsafe extern fn Java_com_rust_litewalletjni_LiteWalletJni_initlogging(env: JNIEnv, _: JObject) -> jstring {
    android_logger::init_once(
        Config::default().with_min_level(Level::Trace),
    );

    let ok = format!("OK");
    let output = env.new_string(ok.as_str()).unwrap();
    return output.into_inner();
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_rust_litewalletjni_LiteWalletJni_getseedphrase(env: JNIEnv) -> jstring {

    let results = rustlib::get_seed_phrase();
    //Create string to be passed back to Java
    let output = env.new_string(format!("{}", results)).expect("Couldn't create java string!");
    // Finally, extract the raw pointer to return.
    output.into_inner()
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_rust_litewalletjni_LiteWalletJni_checkseedphrase(env: JNIEnv, _: JObject, input: JString) -> jstring {

    let input: String = env.get_string(input).expect("Couldn't get java string!").into();
    //generate key
    let results = rustlib::check_seed_phrase(input.as_str());
    //Create string to be passed back to Java
    let output = env.new_string(format!("{}", results)).expect("Couldn't create java string!");
    // Finally, extract the raw pointer to return.
    output.into_inner()
}

#[no_mangle]
pub unsafe extern fn Java_com_rust_litewalletjni_LiteWalletJni_initnew(env: JNIEnv, _: JObject, j_serveruri: JString, j_params: JString,
        j_sapling_output: JString, j_sapling_spend: JString) -> jstring {
    let server_uri = CString::from(
        CStr::from_ptr(
            env.get_string(j_serveruri).unwrap().as_ptr()
        )
    ).into_string().unwrap();

    let params = CString::from(
        CStr::from_ptr(
            env.get_string(j_params).unwrap().as_ptr()
        )
    ).into_string().unwrap();

    let sapling_output = CString::from(
        CStr::from_ptr(
            env.get_string(j_sapling_output).unwrap().as_ptr()
        )
    ).into_string().unwrap();
    let sapling_spend = CString::from(
        CStr::from_ptr(
            env.get_string(j_sapling_spend).unwrap().as_ptr()
        )
    ).into_string().unwrap();

    let seed = rustlib::init_new(server_uri, params, sapling_output, sapling_spend);

    let output = env.new_string(seed.as_str()).unwrap();
    output.into_inner()
}


#[no_mangle]
pub unsafe extern fn Java_com_rust_litewalletjni_LiteWalletJni_initfromseed(env: JNIEnv, _: JObject, j_serveruri: JString, j_params: JString,
        j_seed: JString, j_birthday: JString, j_sapling_output: JString, j_sapling_spend: JString) -> jstring {
    let server_uri = CString::from(
        CStr::from_ptr(
            env.get_string(j_serveruri).unwrap().as_ptr()
        )
    ).into_string().unwrap();

    let params = CString::from(
        CStr::from_ptr(
            env.get_string(j_params).unwrap().as_ptr()
        )
    ).into_string().unwrap();

    let seed = CString::from(
        CStr::from_ptr(
            env.get_string(j_seed).unwrap().as_ptr()
        )
    ).into_string().unwrap();

    let birthday = CString::from(
        CStr::from_ptr(
            env.get_string(j_birthday).unwrap().as_ptr()
        )
    ).into_string().unwrap().parse::<u64>().unwrap();

    let sapling_output = CString::from(
        CStr::from_ptr(
            env.get_string(j_sapling_output).unwrap().as_ptr()
        )
    ).into_string().unwrap();
    let sapling_spend = CString::from(
        CStr::from_ptr(
            env.get_string(j_sapling_spend).unwrap().as_ptr()
        )
    ).into_string().unwrap();

    let seed = rustlib::init_from_seed(server_uri, params, seed, birthday, sapling_output, sapling_spend);

    let output = env.new_string(seed.as_str()).unwrap();
    output.into_inner()
}


#[no_mangle]
pub unsafe extern fn Java_com_rust_litewalletjni_LiteWalletJni_initfromb64(env: JNIEnv, _: JObject, j_serveruri: JString, j_params: JString,
        j_base64: JString, j_sapling_output: JString, j_sapling_spend: JString) -> jstring {
    let base64 = CString::from(
        CStr::from_ptr(
            env.get_string(j_base64).unwrap().as_ptr()
        )
    ).into_string().unwrap();

    let server_uri = CString::from(
        CStr::from_ptr(
            env.get_string(j_serveruri).unwrap().as_ptr()
        )
    ).into_string().unwrap();

    let params = CString::from(
        CStr::from_ptr(
            env.get_string(j_params).unwrap().as_ptr()
        )
    ).into_string().unwrap();

    let sapling_output = CString::from(
        CStr::from_ptr(
            env.get_string(j_sapling_output).unwrap().as_ptr()
        )
    ).into_string().unwrap();
    let sapling_spend = CString::from(
        CStr::from_ptr(
            env.get_string(j_sapling_spend).unwrap().as_ptr()
        )
    ).into_string().unwrap();

    let seed = rustlib::init_from_b64(server_uri, params, base64, sapling_output, sapling_spend);

    let output = env.new_string(seed.as_str()).unwrap();
    output.into_inner()
}

#[no_mangle]
pub unsafe extern fn Java_com_rust_litewalletjni_LiteWalletJni_save(env: JNIEnv, _: JObject) -> jstring {
    let encoded = rustlib::save_to_b64();
    let output = env.new_string(encoded.as_str()).unwrap();
    output.into_inner()
}

#[no_mangle]
pub unsafe extern fn Java_com_rust_litewalletjni_LiteWalletJni_execute(env: JNIEnv, _: JObject, j_command: JString, j_argslist: JString) -> jstring {
    let cmd = CString::from(
        CStr::from_ptr(
            env.get_string(j_command).unwrap().as_ptr()
        )
    ).into_string().unwrap();

    let args_list = CString::from(
        CStr::from_ptr(
            env.get_string(j_argslist).unwrap().as_ptr()
        )
    ).into_string().unwrap();

    let resp = rustlib::execute(cmd, args_list);

    let output = env.new_string(resp.as_str()).unwrap();
    output.into_inner()
}
