# Intro

This file describe how to impelement the event call on the smart contract

# A contract with event setting

A contract can provide a event with this event's information as args.

And this contract have a entrypoint `set_event_callback` exposed.

```rust
// ======
// set_event_callback
// args:
// - call_back_contract_hash: ContractHash
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

## usage

test:

```
make test
```

the test will have output for let user know how it work simply.

and see the test code for more information.

## how it work - step

the counter contract will have two entry after deploy

- count
- set_event_callback

logger contract will call `set_event_callback` on the first contract when deploy.

counter countract will add it to callback list.

every time user call the counter's count entry.

counter contract will send event by iterating the callback list.