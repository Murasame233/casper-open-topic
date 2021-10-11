#[cfg(test)]
mod tests {
    use std::io::Write;

    use casper_engine_test_support::{Code, Hash, SessionBuilder, TestContextBuilder};
    use casper_types::system::CallStackElement;
    use casper_types::{account::AccountHash, runtime_args, PublicKey, SecretKey, U512};
    use casper_types::{ContractHash, RuntimeArgs};

    const MY_ACCOUNT: [u8; 32] = [7u8; 32];
    const ANOTHER_ACCOUNT: [u8; 32] = [8u8; 32];
    const KEY: &str = "Caller";
    // Define `KEY` constant to match that in the contract.
    const CONTRACT_WASM: &str = "caller_recorder.wasm";
    const UPDATE_WASM: &str = "contract_caller.wasm";
    const UPDATE_STORED_WASM: &str = "contract_caller2.wasm";

    #[test]
    fn should_store_hello_world() {
        // Ready for account Context
        let secret_key = SecretKey::ed25519_from_bytes(MY_ACCOUNT).unwrap();
        let public_key = PublicKey::from(&secret_key);
        let account_addr = AccountHash::from(&public_key);
        let secret_key_ano = SecretKey::ed25519_from_bytes(ANOTHER_ACCOUNT).unwrap();
        let public_key_ano = PublicKey::from(&secret_key_ano);
        let another_account = AccountHash::from(&public_key_ano);

        let mut context = TestContextBuilder::new()
            .with_public_key(public_key, U512::from(500_000_000_000_000_000u64))
            .with_public_key(public_key_ano, U512::from(500_000_000_000_000_000u64))
            .build();

        // Deploy Contract
        let session_code = Code::from(CONTRACT_WASM);
        let session = SessionBuilder::new(session_code, runtime_args! {})
            .with_address(account_addr)
            .with_authorization_keys(&[account_addr])
            .build();

        context.run(session);

        // Update caller with direct call
        // user -> caller_recorder
        let hash: Hash = context
            .query(account_addr, &["caller_contract_hash".into()])
            .unwrap()
            .into_t()
            .unwrap();
        let code = Code::Hash(hash, "update_caller".into());
        let update_session = SessionBuilder::new(code, runtime_args! {})
            .with_address(account_addr)
            .with_authorization_keys(&[account_addr])
            .build();

        context.run(update_session);

        // query Result
        let s: String = context
            .query(account_addr, &[KEY.into()])
            .unwrap()
            .into_t()
            .unwrap();
        println!("{}", s);
        let mut s1 = std::fs::File::create("stack1.txt").unwrap();
        let v: Vec<CallStackElement> = context
            .query(account_addr, &["Call Stack".into()])
            .unwrap()
            .into_t()
            .unwrap();
        s1.write(format!("{:?}", v).as_bytes()).unwrap();
        s1.flush().unwrap();

        // Update caller with contract
        // user -> middle contract -> caller_recorder
        let session_code = Code::from(UPDATE_WASM);
        let session = SessionBuilder::new(
            session_code,
            runtime_args! {"hash"=>ContractHash::from(hash)},
        )
        .with_address(another_account)
        .with_authorization_keys(&[another_account])
        .build();
        context.run(session);

        let mut s2 = std::fs::File::create("stack2.txt").unwrap();
        let v: Vec<CallStackElement> = context
            .query(account_addr, &["Call Stack".into()])
            .unwrap()
            .into_t()
            .unwrap();
        s2.write(format!("{:?}", v).as_bytes()).unwrap();
        s2.flush().unwrap();

        // Update caller with contract
        // deploy contract
        let update_deploy_session_code = Code::from(UPDATE_STORED_WASM);
        let update_deploy_session = SessionBuilder::new(update_deploy_session_code, runtime_args! {})
            .with_address(another_account)
            .with_authorization_keys(&[another_account])
            .build();

        context.run(update_deploy_session);

        //run update contract
        // user -> stored contract -> caller_recorder
        let hash_update: Hash = context
            .query(another_account, &["cal_contract_hash".into()])
            .unwrap()
            .into_t()
            .unwrap();
        let code = Code::Hash(hash_update, "cal".into());
        let update_session =
            SessionBuilder::new(code, runtime_args! {"hash"=>ContractHash::from(hash)})
                .with_address(another_account)
                .with_authorization_keys(&[another_account])
                .build();
        context.run(update_session);

        // query Result
        let s: String = context
            .query(account_addr, &[KEY.into()])
            .unwrap()
            .into_t()
            .unwrap();
        println!("{}", s);
        let mut s3 = std::fs::File::create("stack3.txt").unwrap();
        let v: Vec<CallStackElement> = context
            .query(account_addr, &["Call Stack".into()])
            .unwrap()
            .into_t()
            .unwrap();
        s3.write(format!("{:?}", v).as_bytes()).unwrap();
        s3.flush().unwrap();
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
