use soroban_sdk::{Env, Address, Symbol, IntoVal};

pub fn mint_nft(
    env: &Env,
    nft_contract: Address,
    to: Address,
) {
    env.invoke_contract::<()>(
        &nft_contract,
        &Symbol::new(env, "mint"),
        soroban_sdk::vec![env, to.into_val(env)],
    );
}
