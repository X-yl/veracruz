//! PSA Attestation-specific material
//!
//! ## Authors
//!
//! The Veracruz Development Team.
//!
//! ## Licensing and copyright notice
//!
//! See the `LICENSE.markdown` file in the Veracruz root directory for
//! information on licensing and copyright.

use crate::error::*;
use lazy_static::lazy_static;
use rand::Rng;
use std::{collections::HashMap, sync::Mutex};
use std::io::Write;

use nitro_enclave_token::NitroToken;

static AWS_NITRO_ROOT_CERTIFICATE: [u8; 533] = [
    0x30, 0x82, 0x02, 0x11, 0x30, 0x82, 0x01, 0x96, 0xa0, 0x03, 0x02, 0x01, 0x02, 0x02, 0x11, 0x00,
    0xf9, 0x31, 0x75, 0x68, 0x1b, 0x90, 0xaf, 0xe1, 0x1d, 0x46, 0xcc, 0xb4, 0xe4, 0xe7, 0xf8, 0x56,
    0x30, 0x0a, 0x06, 0x08, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x04, 0x03, 0x03, 0x30, 0x49, 0x31, 0x0b,
    0x30, 0x09, 0x06, 0x03, 0x55, 0x04, 0x06, 0x13, 0x02, 0x55, 0x53, 0x31, 0x0f, 0x30, 0x0d, 0x06,
    0x03, 0x55, 0x04, 0x0a, 0x0c, 0x06, 0x41, 0x6d, 0x61, 0x7a, 0x6f, 0x6e, 0x31, 0x0c, 0x30, 0x0a,
    0x06, 0x03, 0x55, 0x04, 0x0b, 0x0c, 0x03, 0x41, 0x57, 0x53, 0x31, 0x1b, 0x30, 0x19, 0x06, 0x03,
    0x55, 0x04, 0x03, 0x0c, 0x12, 0x61, 0x77, 0x73, 0x2e, 0x6e, 0x69, 0x74, 0x72, 0x6f, 0x2d, 0x65,
    0x6e, 0x63, 0x6c, 0x61, 0x76, 0x65, 0x73, 0x30, 0x1e, 0x17, 0x0d, 0x31, 0x39, 0x31, 0x30, 0x32,
    0x38, 0x31, 0x33, 0x32, 0x38, 0x30, 0x35, 0x5a, 0x17, 0x0d, 0x34, 0x39, 0x31, 0x30, 0x32, 0x38,
    0x31, 0x34, 0x32, 0x38, 0x30, 0x35, 0x5a, 0x30, 0x49, 0x31, 0x0b, 0x30, 0x09, 0x06, 0x03, 0x55,
    0x04, 0x06, 0x13, 0x02, 0x55, 0x53, 0x31, 0x0f, 0x30, 0x0d, 0x06, 0x03, 0x55, 0x04, 0x0a, 0x0c,
    0x06, 0x41, 0x6d, 0x61, 0x7a, 0x6f, 0x6e, 0x31, 0x0c, 0x30, 0x0a, 0x06, 0x03, 0x55, 0x04, 0x0b,
    0x0c, 0x03, 0x41, 0x57, 0x53, 0x31, 0x1b, 0x30, 0x19, 0x06, 0x03, 0x55, 0x04, 0x03, 0x0c, 0x12,
    0x61, 0x77, 0x73, 0x2e, 0x6e, 0x69, 0x74, 0x72, 0x6f, 0x2d, 0x65, 0x6e, 0x63, 0x6c, 0x61, 0x76,
    0x65, 0x73, 0x30, 0x76, 0x30, 0x10, 0x06, 0x07, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x02, 0x01, 0x06,
    0x05, 0x2b, 0x81, 0x04, 0x00, 0x22, 0x03, 0x62, 0x00, 0x04, 0xfc, 0x02, 0x54, 0xeb, 0xa6, 0x08,
    0xc1, 0xf3, 0x68, 0x70, 0xe2, 0x9a, 0xda, 0x90, 0xbe, 0x46, 0x38, 0x32, 0x92, 0x73, 0x6e, 0x89,
    0x4b, 0xff, 0xf6, 0x72, 0xd9, 0x89, 0x44, 0x4b, 0x50, 0x51, 0xe5, 0x34, 0xa4, 0xb1, 0xf6, 0xdb,
    0xe3, 0xc0, 0xbc, 0x58, 0x1a, 0x32, 0xb7, 0xb1, 0x76, 0x07, 0x0e, 0xde, 0x12, 0xd6, 0x9a, 0x3f,
    0xea, 0x21, 0x1b, 0x66, 0xe7, 0x52, 0xcf, 0x7d, 0xd1, 0xdd, 0x09, 0x5f, 0x6f, 0x13, 0x70, 0xf4,
    0x17, 0x08, 0x43, 0xd9, 0xdc, 0x10, 0x01, 0x21, 0xe4, 0xcf, 0x63, 0x01, 0x28, 0x09, 0x66, 0x44,
    0x87, 0xc9, 0x79, 0x62, 0x84, 0x30, 0x4d, 0xc5, 0x3f, 0xf4, 0xa3, 0x42, 0x30, 0x40, 0x30, 0x0f,
    0x06, 0x03, 0x55, 0x1d, 0x13, 0x01, 0x01, 0xff, 0x04, 0x05, 0x30, 0x03, 0x01, 0x01, 0xff, 0x30,
    0x1d, 0x06, 0x03, 0x55, 0x1d, 0x0e, 0x04, 0x16, 0x04, 0x14, 0x90, 0x25, 0xb5, 0x0d, 0xd9, 0x05,
    0x47, 0xe7, 0x96, 0xc3, 0x96, 0xfa, 0x72, 0x9d, 0xcf, 0x99, 0xa9, 0xdf, 0x4b, 0x96, 0x30, 0x0e,
    0x06, 0x03, 0x55, 0x1d, 0x0f, 0x01, 0x01, 0xff, 0x04, 0x04, 0x03, 0x02, 0x01, 0x86, 0x30, 0x0a,
    0x06, 0x08, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x04, 0x03, 0x03, 0x03, 0x69, 0x00, 0x30, 0x66, 0x02,
    0x31, 0x00, 0xa3, 0x7f, 0x2f, 0x91, 0xa1, 0xc9, 0xbd, 0x5e, 0xe7, 0xb8, 0x62, 0x7c, 0x16, 0x98,
    0xd2, 0x55, 0x03, 0x8e, 0x1f, 0x03, 0x43, 0xf9, 0x5b, 0x63, 0xa9, 0x62, 0x8c, 0x3d, 0x39, 0x80,
    0x95, 0x45, 0xa1, 0x1e, 0xbc, 0xbf, 0x2e, 0x3b, 0x55, 0xd8, 0xae, 0xee, 0x71, 0xb4, 0xc3, 0xd6,
    0xad, 0xf3, 0x02, 0x31, 0x00, 0xa2, 0xf3, 0x9b, 0x16, 0x05, 0xb2, 0x70, 0x28, 0xa5, 0xdd, 0x4b,
    0xa0, 0x69, 0xb5, 0x01, 0x6e, 0x65, 0xb4, 0xfb, 0xde, 0x8f, 0xe0, 0x06, 0x1d, 0x6a, 0x53, 0x19,
    0x7f, 0x9c, 0xda, 0xf5, 0xd9, 0x43, 0xbc, 0x61, 0xfc, 0x2b, 0xeb, 0x03, 0xcb, 0x6f, 0xee, 0x8d,
    0x23, 0x02, 0xf3, 0xdf, 0xf6,
];

#[derive(Clone)]
struct NitroAttestationContext {
    firmware_version: String,
    challenge: [u8; 32],
}

lazy_static! {
    static ref ATTESTATION_CONTEXT: Mutex<HashMap<i32, NitroAttestationContext>> =
        Mutex::new(HashMap::new());
}

pub fn start(firmware_version: &str, device_id: i32) -> TabascoResponder {
    let mut challenge: [u8; 32] = [0; 32];
    let mut rng = rand::thread_rng();

    rng.fill(&mut challenge);

    let attestation_context = NitroAttestationContext {
        firmware_version: firmware_version.to_string(),
        challenge: challenge.clone(),
    };
    {
        let mut ac_hash = ATTESTATION_CONTEXT.lock()?;
        ac_hash.insert(device_id, attestation_context);
    }
    let serialized_attestation_init =
        colima::serialize_psa_attestation_init(&challenge, device_id)?;
    Ok(base64::encode(&serialized_attestation_init))
}

pub fn attestation_token(body_string: String) -> TabascoResponder {
    let _ignore = std::io::stdout().flush();

    let received_bytes = base64::decode(&body_string)
        .map_err(|err| {
            println!("tabasco::attestation::nitro::attestation_token failed to decode base64:{:?}", err);
            let _ignore = std::io::stdout().flush();
            err
        })?;

    let parsed = colima::parse_tabasco_request(&received_bytes)
        .map_err(|err| {
            println!("tabasco::attestation::nitro::attestation_token failed to parse tabasco request:{:?}", err);
            let _ignore = std::io::stdout().flush();
            err
        })?;
    if !parsed.has_native_psa_attestation_token() {
        println!("tabasco::attestation::psa::attestation_token received data is incorrect.");
        let _ignore = std::io::stdout().flush();
        return Err(TabascoError::MissingFieldError(
            "native_psa_attestation_token",
        ));
    }
    let (token, device_id) =
        colima::parse_native_psa_attestation_token(&parsed.get_native_psa_attestation_token());

    let attestation_document = NitroToken::authenticate_token(&token, &AWS_NITRO_ROOT_CERTIFICATE).map_err(|err| {
        println!("Tabasco::nitro::attestation_token authenticate_token failed:{:?}", err);
        let _ignore = std::io::stdout().flush();
        TabascoError::CborError(format!("parse_nitro_token failed to parse token data:{:?}", err))
    })?;

    let attestation_context = {
        let ac_hash = ATTESTATION_CONTEXT.lock()
            .map_err(|err| {
                println!("Tabasco::nitro::attestation_token failed to obtain lock on ATTESTATION_CONTEXT:{:?}", err);
                let _ignore = std::io::stdout().flush();
                err
            })?;
        println!("ac_hash:{:?}", ac_hash[&device_id].firmware_version);
        if ac_hash.contains_key(&device_id) {
            let context = &ac_hash[&device_id];
            context.clone()
        } else {
            println!("Tabasco::nitro::attestation_token device not found. device_id:{:?}", device_id);
            let _ignore = std::io::stdout().flush();
            return Err(TabascoError::NoDeviceError(device_id));
        }
    };

    // check the nonce of the attestation document
    match attestation_document.nonce {
        None => {
            println!("tabasco::attestation::nitro::attestation_token attestation document did not contain a nonce. We require it.");
            let _ignore = std::io::stdout().flush();
            return Err(TabascoError::MissingFieldError("nonce"));
        },
        Some(nonce) => {
            if nonce != attestation_context.challenge {
                println!("Challenge failed to match. Wanted:{:02x?}, got:{:02x?}", nonce, attestation_context.challenge);
                let _ignore = std::io::stdout().flush();
                return Err(TabascoError::MismatchError {
                    variable: "nonce/challenge",
                    expected: attestation_context.challenge.to_vec(),
                    received: nonce,
                });
            }
        },
    }
    

    let expected_enclave_hash: Vec<u8> = {
        let connection = crate::orm::establish_connection()?;
        let hash_option = crate::orm::get_firmware_version_hash(
            &connection,
            &"nitro".to_string(),
            &attestation_context.firmware_version,
        )
            .map_err(|err| {
                println!("tabasco::attestation::nitro::attestation_token get_firmware_version_hash failed:{:?}", err);
                let _ignore = std::io::stdout().flush();
                err
            })?;
        match hash_option {
            None => {
                println!("tabasco::attestation::nitro_attestation_token firmware version hash not found in database");
                let _ignore = std::io::stdout().flush();
                return Err(TabascoError::MissingFieldError("firmware version"));
            },
            Some(hash) => hash,
        }
    };
    let received_enclave_hash = &attestation_document.pcrs[0];
    if expected_enclave_hash != *received_enclave_hash {
        let debug_mode_wrapper = crate::server::DEBUG_MODE.lock()
            .map_err(|err| {
                println!("tabasco::attestation::nitro_attestation_token failed to obtain lock on DEBUG_MODE mutex");
                let _ignore = std::io::stdout().flush();
                TabascoError::MutexError(format!("tabasco::attestation::nitro::attestation_token failed to objtain lock on DEBUG_MODE:{:?}", err))
            }
            )?;
        if *debug_mode_wrapper {
            println!("Comparison between expected_enclave_hash:{:02x?} and received_enclave_hash:{:02x?} failed", expected_enclave_hash, *received_enclave_hash);
            println!("This is debug mode, so this is expected, so we're not going to fail you, but you should feel bad.");
            let _ignore = std::io::stdout().flush();
        } else {
            println!("tabasco::attestation::nitro_attestation_token debug mode is off, so we're gonna return an error for this mismatch");
            let _ignore = std::io::stdout().flush();

            return Err(TabascoError::MismatchError {
                variable: "received_enclave_hash",
                expected: expected_enclave_hash,
                received: received_enclave_hash.to_vec(),
            });
        }
    }

    let digest = match attestation_document.public_key {
        Some(public_key) => ring::digest::digest(&ring::digest::SHA256, &public_key),
        None => {
            return Err(TabascoError::MissingFieldError("public_key"));
        },
    };
    let pubkey_hash = digest.as_ref();

    // TODO: Get a real enclave name (or just get rid of the enclave names entirely)
    let enclave_name: String = "bob".to_string();

    let connection = crate::orm::establish_connection()?;
    crate::orm::update_or_create_device(
        &connection,
        device_id,
        &pubkey_hash.to_vec(),
        enclave_name,
    ).map_err(|err| {
        println!("nitro:: failed to add device to database:{:?}", err);
        let _ignore = std::io::stdout().flush();
        err
    })?;
    Ok("Pass".to_string())
}
