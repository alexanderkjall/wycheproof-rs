#[test]
fn test_aead_parsing() {
    for test in wycheproof::aead::TestName::all() {
        let _kat = wycheproof::aead::TestSet::load(test).unwrap();
    }
}

#[test]
fn test_cipher_parsing() {
    for test in wycheproof::cipher::TestName::all() {
        let _kat = wycheproof::cipher::TestSet::load(test).unwrap();
    }
}

#[test]
fn test_dsa_parsing() {
    for test in wycheproof::dsa::TestName::all() {
        let _kat = wycheproof::dsa::TestSet::load(test).unwrap();
    }
}

#[test]
fn test_ecdh_parsing() {
    for test in wycheproof::ecdh::TestName::all() {
        let _kat = wycheproof::ecdh::TestSet::load(test).unwrap();
    }
}

#[test]
fn test_ecdsa_parsing() {
    for test in wycheproof::ecdsa::TestName::all() {
        let _kat = wycheproof::ecdsa::TestSet::load(test).unwrap();
    }
}

#[test]
fn test_eddsa_parsing() {
    for test in wycheproof::eddsa::TestName::all() {
        let _kat = wycheproof::eddsa::TestSet::load(test).unwrap();
    }
}

#[test]
fn test_ec_curve_parsing() {
    for test in wycheproof::ec_curve::TestName::all() {
        let _kat = wycheproof::ec_curve::TestSet::load(test).unwrap();
    }
}

#[test]
fn test_fpe_str_parsing() {
    for test in wycheproof::fpe_str::TestName::all() {
        let _kat = wycheproof::fpe_str::TestSet::load(test).unwrap();
    }
}

#[test]
fn test_fpe_list_parsing() {
    for test in wycheproof::fpe_list::TestName::all() {
        let _kat = wycheproof::fpe_list::TestSet::load(test).unwrap();
    }
}

#[test]
fn test_hkdf_parsing() {
    for test in wycheproof::hkdf::TestName::all() {
        let _kat = wycheproof::hkdf::TestSet::load(test).unwrap();
    }
}

#[test]
fn test_keywrap_parsing() {
    for test in wycheproof::keywrap::TestName::all() {
        let _kat = wycheproof::keywrap::TestSet::load(test).unwrap();
    }
}

#[test]
fn test_mac_parsing() {
    for test in wycheproof::mac::TestName::all() {
        let _kat = wycheproof::mac::TestSet::load(test).unwrap();
    }
}

#[test]
fn test_mac_with_nonce_parsing() {
    for test in wycheproof::mac_with_nonce::TestName::all() {
        let _kat = wycheproof::mac_with_nonce::TestSet::load(test).unwrap();
    }
}

#[test]
fn test_primality_parsing() {
    for test in wycheproof::primality::TestName::all() {
        let _kat = wycheproof::primality::TestSet::load(test).unwrap();
    }
}

#[test]
fn test_rsa_oaep_parsing() {
    for test in wycheproof::rsa_oaep::TestName::all() {
        let _kat = wycheproof::rsa_oaep::TestSet::load(test).unwrap();
    }
}

#[test]
fn test_rsa_pkcs1_decrypt_parsing() {
    for test in wycheproof::rsa_pkcs1_decrypt::TestName::all() {
        let _kat = wycheproof::rsa_pkcs1_decrypt::TestSet::load(test).unwrap();
    }
}

#[test]
fn test_rsa_pkcs1_verify_parsing() {
    for test in wycheproof::rsa_pkcs1_verify::TestName::all() {
        let _kat = wycheproof::rsa_pkcs1_verify::TestSet::load(test).unwrap();
    }
}

#[test]
fn test_rsa_pss_verify_parsing() {
    for test in wycheproof::rsa_pss_verify::TestName::all() {
        let _kat = wycheproof::rsa_pss_verify::TestSet::load(test).unwrap();
    }
}

#[test]
fn test_xdh_parsing() {
    for test in wycheproof::xdh::TestName::all() {
        let _kat = wycheproof::xdh::TestSet::load(test).unwrap();
    }
}
