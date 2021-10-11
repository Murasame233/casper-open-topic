# Intro

This markdown file will describe about the erc-20 standard on the casper blockchain. We cannot just implement ERC-20 on the casper, and we should make a erc-20-casper standard that makes the token compitable with casper blockchain.

# The problem

## casper-erc20 implement problem

When dev some smart contract, there's a problem about authorization.
the casper-erc20 package use `get_immediate_call_stack_item` to get the caller.

```rust
fn get_immediate_call_stack_item() -> Option<CallStackElement> {
    let call_stack = runtime::get_call_stack();
    call_stack.into_iter().rev().nth(1) // get the direct caller.
}
```

## Conditions

And there is three condition to call a erc-20 contract.

> These three conditions I am already do some research about it.
> You can `cd test_getcaller` and `make test-caller` to get three file on the `tests/result` folder.

- user call erc-20 contract.(`stack1.txt`)
- user use code from a contract, run as a session, this contract will call erc-20 contract.(`stack2.txt`)
- user call a stored contract, this entrypoint will call the erc-20 contract.(`stack3.txt`)

With the first and second way, the `get_immediate_call_stack_item` will get the user's AccountHash,
And the third way, the `get_immediate_call_stack_item` will get the stored contract's contractPackageHash,

## What hacker can do

If there is a hacker, he can make a fake contract and user deploy it. And this contract will work like the second condition, let erc-20 contract think that's a user call. The hacker can transfer all token to anywhere.

# Solve

There is two way to solve.

## Add new `CallStackElement`

- unstored contract
- unstored session

This will make the first condition still can use `get_immediate_call_stack_item` to get the caller.

And on the second condition will get a unstored session or contract.

And third is still stored contract.

## Use secret to valid user

That's why we cannot just simply implement ERC-20 to casper.

We need add guards to some function. These guard will valid user's secret on every call to prevent the call not from the user.

Add if user want to use a erc-20 token. They have to set a secret to be able to use this token.

And secret will valid by using keccak256 or other hash function

We need a new `set_secret` entrypoint, with two parameter:

```
old_secret: String (valid secret)
new_hash: String (Hex format)
```
