# Intro
This file describe how to impelement the event call on the smart contract

# A contract with event setting

A contract can provide a event with this event's information as args.

And this contract have a entrypoint `set_event_callback` exposed.

```rust
// ======
// set_event_callback
// args: 
// - call_back_contract_hash: ContractHash to formated string
// - call_back_contract_entry: String (entrypoint)
// - event: String (event name)
```

these setting can be stored like
```rust
event: Vec<(String, ContractHash, String)>
//          event                  entry
```

when a event happen
will call the `call_back_contract_entry` on the `call_back_contract_hash` with event args.

# Outro

This design's practicality is extensive.

# A event logger - Example

A counter contract with call back. and another contract from another user is a logger can log all user call the counter contract.