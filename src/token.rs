use soroban_sdk::{contract, contractimpl, Env, Address, BytesN};

#[contract]
pub struct CertificateToken;

#[contractimpl]
impl CertificateToken {

    pub fn mint(env: Env, owner: Address, cert_hash: BytesN<32>) {
        let key = (owner.clone(), cert_hash.clone());

        env.storage().persistent().set(&key, &true);

        env.events().publish(
            ("certificate_token", "minted"),
            cert_hash
        );
    }
}