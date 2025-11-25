use elliptic_curve::rand_core::{OsRng, RngCore};
use secp256k1::{SecretKey, PublicKey, Secp256k1};
use sha2::{Digest, Sha256};
use ripemd::{Ripemd160, Digest as RipemdDigest};
use hex;
pub trait Wallet{
    fn new()->Self;
    fn get_address(&self)->&str;
    fn get_priv_key(&self)->&str;
    fn get_public_key(&self)->&str;
}
pub struct Wart_key{
    priv_key:String,
    public_key:String,
    address:String
}
impl Wallet for Wart_key{
    fn new()->Self{
        let mut sk_bytes = [0u8; 32];
        let mut rng = OsRng;
        rng.fill_bytes(&mut sk_bytes);
        //println!("private_key: {}", hex::encode(&sk_bytes));
        let secp = Secp256k1::new();
        let sk = SecretKey::from_slice(&sk_bytes)
            .expect("Error creating secret key from slice");
        let pub_key = PublicKey::from_secret_key(&secp, &sk);
        //println!("public key: {}", hex::encode(pubkey.serialize()));
        let sha = Sha256::digest(&pub_key.serialize());
        let addr_raw = Ripemd160::digest(&sha);
        let mut hasher_checksum = Sha256::new();
        hasher_checksum.update(&addr_raw);
        let checksum = &hasher_checksum.finalize()[..4];
        let addr = [&addr_raw[..], checksum].concat();
        //println!("address: {}", hex::encode(addr));
        Wart_key{
            priv_key:hex::encode(&sk_bytes),
            public_key:hex::encode(pub_key.serialize()),
            address:hex::encode(addr)
        }
    }
    fn get_address(&self) -> &str {
        &self.address
    }
    fn get_priv_key(&self) -> &str {
        &self.priv_key
    }
    fn get_public_key(&self) -> &str {
        &self.public_key
    }
}

