use soroban_sdk::{contracttype, String};

#[contracttype]
#[derive(Clone)]
pub struct RewardId(pub String);

#[contracttype]
#[derive(Clone)]
pub struct PlayerId(pub String);

#[contracttype]
#[derive(Clone)]
pub struct StakeId(pub String);

#[contracttype]
#[derive(Clone)]
pub struct BetId(pub String);
