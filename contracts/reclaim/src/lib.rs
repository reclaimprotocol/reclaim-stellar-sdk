#![no_std]
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, vec, Address, BytesN, Env,
    String, Symbol, Vec,
};

#[contract]
pub struct ReclaimContract;

const CONFIG: Symbol = symbol_short!("CONFIG");
const EPOCH: Symbol = symbol_short!("EPOCH");

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Config {
    pub owner: Address,
    pub current_epoch: u128,
    pub exists: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Witness {
    pub address: BytesN<20>,
    pub host: String,
}

impl Witness {
    pub fn get_addresses(witness: Vec<Witness>) -> Vec<BytesN<20>> {
        let env = Env::default();
        let mut vec_addresses = vec![&env];
        for wit in witness {
            vec_addresses.push_back(wit.address);
        }
        return vec_addresses;
    }
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Epoch {
    pub id: u128,
    pub timestamp_start: u64,
    pub timestamp_end: u64,
    pub minimum_witness: u128,
    pub witnesses: Vec<Witness>,
}

fn generate_random_seed(bytes: Vec<u8>, offset: usize) -> u32 {
    let hash_slice = &bytes.slice(offset as u32..(offset + 4) as u32);
    let mut seed = 0u32;

    // for i in 0..hash_slice.len(){
    //     let byte = hash_slice.x;
    // }

    // for (i, &byte) in hash_slice.iter().enumerate() {
    //     seed |= u32::from(byte) << (i * 8);
    // }

    seed
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ReclaimError {
    OnlyOwner = 1,
    AlreadyInitialized = 2,
    HashMismatch = 3,
    LengthMismatch = 4,
    SignatureMismatch = 5,
}

#[contractimpl]
impl ReclaimContract {
    pub fn instantiate(env: Env, user: Address) -> Result<(), ReclaimError> {
        user.require_auth();

        let default_config = Config {
            owner: user.clone(),
            current_epoch: 0_u128,
            exists: false,
        };
        let mut config: Config = env
            .storage()
            .persistent()
            .get(&CONFIG)
            .unwrap_or(default_config);

        if config.exists == true {
            return Err(ReclaimError::AlreadyInitialized);
        }

        config = Config {
            owner: user,
            current_epoch: 0_u128,
            exists: true,
        };

        env.storage().persistent().set(&CONFIG, &config);

        let now = env.ledger().timestamp();
        let epoch = Epoch {
            id: 0_u128,
            timestamp_start: now,
            timestamp_end: now + 10000_u64,
            minimum_witness: 1_u128,
            witnesses: vec![&env],
        };

        env.storage().persistent().set(&EPOCH, &epoch);
        Ok(())
    }

    pub fn add_epoch(
        env: Env,
        witnesses: Vec<Witness>,
        minimum_witness: u128,
    ) -> Result<(), ReclaimError> {
        let mut config: Config = env.storage().persistent().get(&CONFIG).unwrap();
        let admin = config.owner.clone();
        admin.require_auth();

        let new_epoch_id = config.current_epoch + 1_u128;
        let now = env.ledger().timestamp();

        let epoch = Epoch {
            id: new_epoch_id,
            timestamp_start: now,
            timestamp_end: now + 10000_u64,
            minimum_witness,
            witnesses: witnesses,
        };

        env.storage().persistent().set(&EPOCH, &epoch);

        config.current_epoch = new_epoch_id;
        env.storage().persistent().set(&CONFIG, &config);

        Ok(())
    }

    // pub fn verify_proof(env: Env, signed_claim: SignedClaim) -> Result<(), ReclaimError> {
    //     Ok(())
    // }
}

mod test;
