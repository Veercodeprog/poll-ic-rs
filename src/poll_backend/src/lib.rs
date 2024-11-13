use candid::{CandidType, Deserialize, Principal};
// use ic_cdk::export::Principal;
use std::cell::RefCell;
use std::collections::BTreeMap;

type VoteStore = BTreeMap<String, u64>;

thread_local! {
    static VOTES: RefCell<VoteStore> = RefCell::new(BTreeMap::new());
    static QUESTION: RefCell<String> = RefCell::new(String::from("What is your favorite programming language?"));
}

#[ic_cdk::query]
fn get_question() -> String {
    QUESTION.with(|q| q.borrow().clone())
}

#[ic_cdk::query]
fn get_votes() -> Vec<(String, u64)> {
    VOTES.with(|votes| votes.borrow().clone().into_iter().collect())
}

#[ic_cdk::update]
fn vote(option: String) -> Vec<(String, u64)> {
    VOTES.with(|votes| {
        let mut votes = votes.borrow_mut();
        let count = votes.entry(option).or_insert(0);
        *count += 1;
        votes.clone().into_iter().collect()
    })
}

#[ic_cdk::update]
fn reset_votes() -> Vec<(String, u64)> {
    VOTES.with(|votes| {
        let mut votes = votes.borrow_mut();
        votes.clear();
        votes.insert("Rust".to_string(), 0);
        votes.insert("Motoko".to_string(), 0);
        votes.insert("TypeScript".to_string(), 0);
        votes.insert("Python".to_string(), 0);
        votes.clone().into_iter().collect()
    })
}
