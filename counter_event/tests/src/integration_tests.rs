#[cfg(test)]
mod tests {

    use casper_engine_test_support::{Code, SessionBuilder, TestContext, TestContextBuilder};
    use casper_types::{
        account::AccountHash, runtime_args, AsymmetricType, ContractHash, PublicKey, RuntimeArgs,
        U512,
    };
    const ACCOUNT_A: [u8; 32] = [3u8; 32];
    const ACCOUNT_B: [u8; 32] = [6u8; 32];
    const ACCOUNT_C: [u8; 32] = [9u8; 32];

    fn get_count(context: &TestContext, account_hash: AccountHash) -> u32 {
        context
            .query(account_hash, &["count".into()])
            .unwrap()
            .into_t()
            .unwrap()
    }
    #[test]
    fn test_count() {
        let account = PublicKey::ed25519_from_bytes(&ACCOUNT_A).unwrap();
        let account_hash = account.to_account_hash();

        let mut context = TestContextBuilder::new()
            .with_public_key(account, U512::from(100_000_000_000_000u64))
            .build();

        // Deploy contract
        let code = Code::from("counter.wasm");
        let session = SessionBuilder::new(code, runtime_args! {})
            .with_address(account_hash)
            .with_authorization_keys(&[account_hash])
            .build();
        context.run(session);
        let count: u32 = get_count(&context, account_hash);
        assert_eq!(count, 0);
        // Get contract hash
        let hash: ContractHash = context
            .query(account_hash, &["counter_hash".into()])
            .unwrap()
            .into_t()
            .unwrap();

        let count = Code::Hash(hash.value(), "count".into());
        let count_session = SessionBuilder::new(count, runtime_args! {})
            .with_address(account_hash)
            .with_authorization_keys(&[account_hash])
            .build();
        context.run(count_session);

        let count = get_count(&context, account_hash);
        assert_eq!(count, 1);
    }

    #[test]
    fn test_callback() {
        let account = PublicKey::ed25519_from_bytes(&ACCOUNT_A).unwrap();
        let account_hash = account.to_account_hash();

        let account_logger = PublicKey::ed25519_from_bytes(&ACCOUNT_B).unwrap();
        let logger_hash = account_logger.to_account_hash();

        let account_user = PublicKey::ed25519_from_bytes(&ACCOUNT_C).unwrap();
        let user_hash = account_user.to_account_hash();

        let mut context = TestContextBuilder::new()
            .with_public_key(account, U512::from(100_000_000_000_000u64))
            .with_public_key(account_logger, U512::from(100_000_000_000_000u64))
            .with_public_key(account_user, U512::from(100_000_000_000_000u64))
            .build();

        let code = Code::from("counter.wasm");
        let session = SessionBuilder::new(code, runtime_args! {})
            .with_address(account_hash)
            .with_authorization_keys(&[account_hash])
            .build();
        context.run(session);
        let count: u32 = get_count(&context, account_hash);
        assert_eq!(count, 0);

        println!("counter contract deployed.");

        let hash: ContractHash = context
            .query(account_hash, &["counter_hash".into()])
            .unwrap()
            .into_t()
            .unwrap();

        // Deploy logger

        let code = Code::from("logger.wasm");
        let session = SessionBuilder::new(
            code,
            runtime_args! {
                "hash"=> hash.to_formatted_string()
            },
        )
        .with_address(logger_hash)
        .with_authorization_keys(&[logger_hash])
        .build();
        context.run(session);

        println!("logger contract deployed, event should be set");

        let callbacks: Vec<(String, ContractHash, String)> = context
            .query(account_hash, &["callbacks".into()])
            .unwrap()
            .into_t()
            .unwrap();
        assert!(!callbacks.is_empty());
        println!("Callbacks: {:?}", callbacks);

        // count

        let count = Code::Hash(hash.value(), "count".into());
        let count_session = SessionBuilder::new(count, runtime_args! {})
            .with_address(user_hash)
            .with_authorization_keys(&[user_hash])
            .build();
        context.run(count_session);

        let count = get_count(&context, account_hash);
        assert_eq!(count, 1);

        println!("A count be called, and event listener should be call, the logger's log should be update.");

        let log: Vec<(String, u32)> = context
            .query(logger_hash, &["log".into()])
            .unwrap()
            .into_t()
            .unwrap();
        assert!(!log.is_empty());
        println!("log: {:?}", log);
        assert_eq!(log[0].1, 1);
    }
}
fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
