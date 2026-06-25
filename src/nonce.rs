use soroban_sdk::{contracttype, Address, Env};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NonceDataKey {
    Nonce(Address),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NonceValue {
    pub nonce: u64,
}

#[allow(dead_code)]
pub fn get_nonce(env: &Env, coordinator: &Address) -> u64 {
    let key = NonceDataKey::Nonce(coordinator.clone());
    if let Some(value) = env.storage().persistent().get::<_, NonceValue>(&key) {
        value.nonce
    } else {
        0
    }
}

pub fn consume_nonce(
    env: &Env,
    coordinator: &Address,
    provided_nonce: u64,
    _salt: soroban_sdk::Bytes,
    _signature: soroban_sdk::Bytes,
) -> Result<(), crate::ContractError> {
    let key = NonceDataKey::Nonce(coordinator.clone());
    let current_nonce = if let Some(value) = env.storage().persistent().get::<_, NonceValue>(&key) {
        value.nonce
    } else {
        0
    } + 1;

    if provided_nonce != current_nonce {
        return Err(crate::ContractError::InvalidNonce);
    }

    env.storage().persistent().set(&key, &NonceValue { nonce: current_nonce });
    Ok(())
}
