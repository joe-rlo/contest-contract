# Weighted Raffle Smart Contract

This NEAR smart contract implements a weighted raffle system. Participants can enter the raffle with different weights, giving them proportionally different chances of winning.

Based off the contract from https://github.com/NEAR-DevHub/race-of-sloths/

## Contract Overview

The contract consists of two main structures:

1. `Contract`: Stores the owner of the contract.
2. `Participant`: Represents a raffle participant with a name and a weight.

The main function `weighted_raffle` selects winners based on the weights of the participants.

## Functions

### `new(owner: AccountId) -> Self`

Initializes the contract with the given owner.

### `weighted_raffle(participants: Vec<Participant>, winners: u32) -> Vec<String>`

Conducts a weighted raffle and returns the names of the winners.

- `participants`: A list of `Participant` objects, each with a name and weight.
- `winners`: The number of winners to select.

## How to Use

1. Deploy the contract to a NEAR account.
2. Call the `weighted_raffle` function with a list of participants and the number of winners.

Example using NEAR CLI:

```
near call <contract_account_id> weighted_raffle '{"participants": [{"name": "Alice", "weight": 5}, {"name": "Bob", "weight": 3}, {"name": "Charlie", "weight": 2}], "winners": 2}' --accountId <your_account_id>
```


Replace `<contract_account_id>` with the account where the contract is deployed, and `<your_account_id>` with your NEAR account.

## Building and Testing

1. Ensure you have Rust and the NEAR CLI installed.
2. Clone this repository.
3. Build the contract:
   ```
   cargo build --target wasm32-unknown-unknown --release
   ```
4. Deploy the contract using NEAR CLI.
5. Run tests:
   ```
   cargo test
   ```

## Notes

- The contract uses a random number generator seeded with `env::random_seed()` for fairness.
- Participants with higher weights have a higher chance of winning.
- The number of winners must not exceed the number of participants.

## License

MIT License
