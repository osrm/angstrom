use rand::thread_rng;
use secp256k1::{PublicKey, Secp256k1, SecretKey};

pub(crate) fn generate_node_keys(number_nodes: u64) -> Vec<(PublicKey, SecretKey)> {
    let mut rng = thread_rng();

    (0..number_nodes)
        .map(|_| {
            let sk = SecretKey::new(&mut rng);
            let pub_key = sk.public_key(&Secp256k1::default());
            (pub_key, sk)
        })
        .collect()
}
