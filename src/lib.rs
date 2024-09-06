use near_sdk;
use near_sdk::{near, env};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

#[near(contract_state)]
#[derive(Default)]
pub struct Contract {
}

#[near(serializers = [borsh, json])]
pub struct Participant {
    pub name: String,
    pub weight: u32,
}

#[near]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {}
    }

    pub fn weighted_raffle(&self, participants: Vec<Participant>, winners: u32) -> Vec<String> {
        if participants.len() < winners as usize {
            env::panic_str("Number of winners is greater than the number of participants");
        }

        let mut total_weight: u32 = participants.iter().map(|p| p.weight).sum();
        let seed: [u8; 32] = env::random_seed().try_into().unwrap();
        let mut rng = ChaCha8Rng::from_seed(seed);
        let mut selected_winners = Vec::with_capacity(winners as usize);

        let mut remaining_participants = participants;

        for _ in 0..winners {
            let winning_ticket = rng.gen_range(0..total_weight);
            let mut cumulative_weight = 0;

            let winner_index = remaining_participants
                .iter()
                .position(|p| {
                    cumulative_weight += p.weight;
                    cumulative_weight > winning_ticket
                })
                .unwrap();

            let winner = remaining_participants.swap_remove(winner_index);
            selected_winners.push(winner.name);
            total_weight -= winner.weight;
        }

        selected_winners
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, AccountId};

    #[test]
    fn test_weighted_raffle() {
        let context = VMContextBuilder::new()
            .current_account_id(AccountId::new_unchecked("contract.testnet".to_string()))
            .build();
        testing_env!(context);

        let contract = Contract::new(AccountId::new_unchecked("owner.testnet".to_string()));

        let participants = vec![
            Participant { name: "Alice".to_string(), weight: 5 },
            Participant { name: "Bob".to_string(), weight: 3 },
            Participant { name: "Charlie".to_string(), weight: 2 },
        ];

        let winners = contract.weighted_raffle(participants, 2);
        assert_eq!(winners.len(), 2);
        assert!(winners.iter().all(|w| ["Alice", "Bob", "Charlie"].contains(&w.as_str())));
    }
}