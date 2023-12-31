pub mod hashing {
    use crate::base16;
    use ripemd::{Digest as RIPEDigest, Ripemd160};
    use sha256::digest;
    use sha3::{Digest, Keccak256};

    // bitcoin
    pub fn hash_sha256(input: &[u8]) -> String {
        let r = digest(input);
        return r;
    }
    pub fn hash_ripemd160(input: &[u8]) -> String {
        let mut hasher = Ripemd160::new();
        hasher.update(input);
        let out = hasher.finalize();
        return base16::encode_bytes(&out);
    }

    // ethereum
    pub fn hash_keccak256(input: &[u8]) -> String {
        let mut hasher = Keccak256::default();
        hasher.input(input);
        let out = hasher.result();
        let r = base16::encode_bytes(&out).to_uppercase();
        return r;
    }
    pub fn hash_keccak256_str(input: &String) -> String {
        let mut hasher = Keccak256::default();
        hasher.input(&input.clone().into_bytes());
        let out = hasher.result();
        let r = base16::encode_bytes(&out).to_uppercase();
        return r;
    }
}

pub mod bitcoin {
    use crate::base16;
    use crate::base58;
    use crate::crypto::hashing;
    use primitive_types::U512;

    pub fn encode_compressed_pr_key(pr_key: &str) -> String {
        let mut origin = format!("80{}01", &pr_key);
        attach_check_sum(&mut origin);
        return base58::convert_hex_to_base58(&origin);
    }
    pub fn attach_check_sum(s: &mut String) {
        let h1 = hashing::hash_sha256(&base16::decode_string(&s));
        let h2 = hashing::hash_sha256(&base16::decode_string(&h1));

        let check_sum = &h2[0..8].to_uppercase();
        s.push_str(&check_sum);
    }
    pub fn get_compressed_public_key_prefix(y: &str) -> String {
        let divisor = U512::from_big_endian(&[2]);
        let zero = U512::from_big_endian(&[0]);

        if U512::from_str_radix(y, 16)
            .expect("")
            .checked_rem(divisor)
            .expect("")
            == zero
        {
            return String::from("02");
        } else {
            return String::from("03");
        }
    }
    pub fn derive_compressed_address(pub_key: &str) -> String {
        let pub_key_x = String::from(&pub_key[2..66]).to_uppercase();
        let pub_key_y = String::from(&pub_key[66..130]).to_uppercase();

        let origin = format!(
            "{}{}",
            &get_compressed_public_key_prefix(&pub_key_y),
            pub_key_x
        );

        let h1 = hashing::hash_sha256(&base16::decode_string(&origin));
        let h2 = hashing::hash_ripemd160(&base16::decode_string(&h1));

        let mut pre_encode = format!("00{}", &h2);
        attach_check_sum(&mut pre_encode);

        return format!("1{}", &base58::convert_hex_to_base58(&pre_encode));
    }
}

pub mod ethereum {
    use crate::base16;
    use crate::crypto::hashing::{self, hash_keccak256_str};
    use std::iter::zip;
    use std::u8;

    use super::hashing::hash_keccak256;

    pub fn derive_address(pub_key: &str) -> String {
        let pub_key_x = String::from(&pub_key[2..66]).to_uppercase();
        let pub_key_y = String::from(&pub_key[66..130]).to_uppercase();

        let origin = format!("{}{}", pub_key_x, pub_key_y);
        let uncompressed_pub_hash = hash_keccak256(&base16::decode_string(&origin));

        let non_check_summed_address =
            format!("0x{}", &uncompressed_pub_hash[24..64]).to_lowercase();

        let address = check_sum(&non_check_summed_address);

        return address;
    }

    pub fn check_sum(address: &String) -> String {
        assert!(address.len() == 42);

        let ad = String::from(&address[2..]).to_lowercase();
        let h = hash_keccak256_str(&ad);
        let h = h.as_str();

        let mut r = String::from("");

        for (c, flag) in zip(ad.chars().into_iter(), h.chars().into_iter()) {
            if c.is_alphabetic() && u8::from_str_radix(flag.to_string().as_str(), 16).unwrap() > 8 {
                r.push(c.to_ascii_uppercase());
            } else {
                r.push(c);
            }
        }

        return format!("0x{}", r);
    }
}

pub mod secp256k1 {
    use crate::base16;
    use secp256k1::{PublicKey, Secp256k1, SecretKey};
    use std::str::FromStr;

    pub fn get_public_key(pr: &str) -> String {
        let secp = Secp256k1::new();
        let pr_key = SecretKey::from_str(pr).expect("pr-key");
        let pub_key = PublicKey::from_secret_key(&secp, &pr_key);
        return base16::encode_bytes(&pub_key.serialize_uncompressed());
    }
}

#[cfg(test)]
mod tests {
    use crate::crypto::bitcoin;
    use crate::crypto::ethereum;
    use crate::crypto::secp256k1;

    #[test]
    fn bitcoin_private_key_compression() {
        let pr_n = "51bb0a7f49284110c62e4268baa3cfad4a81edcd6e6ec3b2a8ef97f1e3754491";
        let e = "KyxarfAkZKd5ga5bKCaMpTPo3MZpYZkd6iHpxUjkXhsPGaFkwS3i";

        let r = bitcoin::encode_compressed_pr_key(pr_n);
        assert_eq!(e, r);
    }

    #[test]
    fn bitcoin_public_key_compression() {
        let pr_n = "51bb0a7f49284110c62e4268baa3cfad4a81edcd6e6ec3b2a8ef97f1e3754491";
        let pub_key = secp256k1::get_public_key(pr_n);
        let r = bitcoin::derive_compressed_address(&pub_key);

        let e = "1BkbDWm7jXoygx3iED9dPf3PoovNfJ4qGQ";
        assert_eq!(e, r);
    }

    #[test]
    fn ethereum_check_sum() {
        let ad = String::from("0xfb6916095ca1df60bb79ce92ce3ea74c37c5d359");
        let r = ethereum::check_sum(&ad);

        let e = String::from("0xfB6916095ca1df60bB79Ce92cE3Ea74c37c5d359");
        assert_eq!(e, r);
    }

    #[test]
    fn ethereum_address() {
        let pr_n = "51bb0a7f49284110c62e4268baa3cfad4a81edcd6e6ec3b2a8ef97f1e3754491";
        let pub_key = secp256k1::get_public_key(pr_n);
        let r = ethereum::derive_address(&pub_key);

        let e = "0x7aa6D878Ac2d1271fCD010802f7e09fAcd8528bf";
        assert_eq!(e, r);
    }
}
