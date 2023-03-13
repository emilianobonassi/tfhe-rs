use std::fmt::Debug;
use crate::typed_api::prelude::*;
use crate::typed_api::{ConfigBuilder, FheUint2};
use crate::typed_api::FheBool;
use crate::typed_api::FheUint8;
use crate::typed_api::{generate_keys};
use crate::typed_api::{ClientKey, PublicKey};

fn assert_that_public_key_encryption_is_decrypted_by_client_key<FheType, ClearType>(
    clear: ClearType,
    pks: &PublicKey,
    cks: &ClientKey,
) where
    ClearType: Copy + Eq + Debug,
    FheType: FheTryEncrypt<ClearType, PublicKey> + FheDecrypt<ClearType> {
    let encrypted = FheType::try_encrypt(clear, pks).unwrap();
    let decrypted: ClearType = encrypted.decrypt(cks);
    assert_eq!(clear, decrypted);
}

#[test]
fn test_boolean_public_key() {
    let config = ConfigBuilder::all_disabled().enable_default_bool().build();

    let (cks, _sks) = generate_keys(config);

    let pks = PublicKey::new(&cks);

    assert_that_public_key_encryption_is_decrypted_by_client_key::<FheBool, bool>(
        false, &pks, &cks
    );
    assert_that_public_key_encryption_is_decrypted_by_client_key::<FheBool, bool>(
        true, &pks, &cks
    );
}

#[test]
fn test_shortint_public_key() {
    let config = ConfigBuilder::all_disabled().enable_default_uint2().build();

    let (cks, _sks) = generate_keys(config);

    let pks = PublicKey::new(&cks);

    assert_that_public_key_encryption_is_decrypted_by_client_key::<FheUint2, u8>(
        0, &pks, &cks
    );
    assert_that_public_key_encryption_is_decrypted_by_client_key::<FheUint2, u8>(
        1, &pks, &cks
    );
    assert_that_public_key_encryption_is_decrypted_by_client_key::<FheUint2, u8>(
        2, &pks, &cks
    );
    assert_that_public_key_encryption_is_decrypted_by_client_key::<FheUint2, u8>(
        3, &pks, &cks
    );
}

#[test]
fn test_integer_public_key() {
    let config = ConfigBuilder::all_disabled().enable_default_uint8().build();

    let (cks, _sks) = generate_keys(config);

    let pks = PublicKey::new(&cks);

    assert_that_public_key_encryption_is_decrypted_by_client_key::<FheUint8, u8>(
        235, &pks, &cks
    );
}

#[test]
fn test_with_context() {
    let config = ConfigBuilder::all_disabled().enable_default_bool().build();

    let (cks, sks) = generate_keys(config);

    let a = FheBool::encrypt(false, &cks);
    let b = FheBool::encrypt(true, &cks);

    let (r, _) = crate::typed_api::with_server_key_as_context(sks, move || a & b);
    let d = r.decrypt(&cks);
    assert_eq!(d, false);
}