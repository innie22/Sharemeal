#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Address, Env, Vec,
};

#[derive(Clone)]
#[contracttype]
pub struct Participant {
    pub addr: Address,
    pub amount_owed: i128,
    pub paid: bool,
}

#[derive(Clone)]
#[contracttype]
pub struct Meal {
    pub meal_id: u64,
    pub creator: Address,
    pub participants: Vec<Participant>,
    pub settled: bool,
}

#[contract]
pub struct BillSplitter;

#[contractimpl]
impl BillSplitter {
    // ✅ Create a new meal
    pub fn create_meal(env: Env, meal_id: u64, creator: Address) {
        // Prevent overwriting existing meal
        if env.storage().instance().has(&meal_id) {
            panic!("Meal already exists");
        }

        let meal = Meal {
            meal_id,
            creator,
            participants: Vec::new(&env),
            settled: false,
        };

        env.storage().instance().set(&meal_id, &meal);
    }

    // ✅ Add participant
    pub fn add_participant(
        env: Env,
        meal_id: u64,
        addr: Address,
        amount: i128,
    ) {
        let mut meal: Meal = env
            .storage()
            .instance()
            .get(&meal_id)
            .expect("Meal not found");

        // Prevent duplicate participants
        for p in meal.participants.iter() {
            if p.addr == addr {
                panic!("Participant already exists");
            }
        }

        let participant = Participant {
            addr,
            amount_owed: amount,
            paid: false,
        };

        meal.participants.push_back(participant);
        env.storage().instance().set(&meal_id, &meal);
    }

    // ✅ Mark participant as paid
    pub fn mark_paid(env: Env, meal_id: u64, addr: Address) {
        let mut meal: Meal = env
            .storage()
            .instance()
            .get(&meal_id)
            .expect("Meal not found");

        let mut updated = Vec::new(&env);

        for p in meal.participants.iter() {
            if p.addr == addr {
                updated.push_back(Participant {
                    addr: p.addr.clone(),
                    amount_owed: p.amount_owed,
                    paid: true,
                });
            } else {
                updated.push_back(p);
            }
        }

        meal.participants = updated;

        // Check if all participants paid
        let mut all_paid = true;
        for p in meal.participants.iter() {
            if !p.paid {
                all_paid = false;
            }
        }

        meal.settled = all_paid;

        env.storage().instance().set(&meal_id, &meal);
    }

    // ✅ Get full meal info
    pub fn get_meal(env: Env, meal_id: u64) -> Meal {
        env.storage()
            .instance()
            .get(&meal_id)
            .expect("Meal not found")
    }

    // ✅ Get unpaid participants (for reminders)
    pub fn get_unpaid(env: Env, meal_id: u64) -> Vec<Participant> {
        let meal: Meal = env
            .storage()
            .instance()
            .get(&meal_id)
            .expect("Meal not found");

        let mut unpaid = Vec::new(&env);

        for p in meal.participants.iter() {
            if !p.paid {
                unpaid.push_back(p);
            }
        }

        unpaid
    }

    // ✅ Remove a participant (optional but useful)
    pub fn remove_participant(env: Env, meal_id: u64, addr: Address) {
        let mut meal: Meal = env
            .storage()
            .instance()
            .get(&meal_id)
            .expect("Meal not found");

        let mut updated = Vec::new(&env);

        for p in meal.participants.iter() {
            if p.addr != addr {
                updated.push_back(p);
            }
        }

        meal.participants = updated;

        env.storage().instance().set(&meal_id, &meal);
    }
}