#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Address, String, Symbol, symbol_short};

#[contracttype]
#[derive(Clone)]
pub struct Ticket {
    pub ticket_id: u64,
    pub event_name: String,
    pub ticket_price: u64,
    pub owner: Address,
}

#[contracttype]
pub enum TicketKey {
    Ticket(u64)
}

const TICKET_COUNT: Symbol = symbol_short!("T_COUNT");

#[contract]
pub struct NFTTicketingContract;

#[contractimpl]
impl NFTTicketingContract {
    // Create a new ticket (NFT) for an event
    pub fn create_ticket(env: Env, creator: Address, event_name: String, ticket_price: u64) -> u64 {
        // Authenticate the creator
        creator.require_auth();

        if ticket_price == 0 {
            panic!("Invalid ticket price");
        }

        let mut ticket_count: u64 = env.storage().instance().get(&TICKET_COUNT).unwrap_or(0);
        ticket_count += 1;

        let ticket = Ticket {
            ticket_id: ticket_count,
            event_name: event_name.clone(),
            ticket_price,
            owner: creator.clone(),
        };

        env.storage().instance().set(&TicketKey::Ticket(ticket_count), &ticket);
        env.storage().instance().set(&TICKET_COUNT, &ticket_count);

        log!(&env, "Ticket created for event: {}", event_name);

        ticket_count
    }

    // Transfer ticket to another user
    pub fn transfer_ticket(env: Env, sender: Address, ticket_id: u64, new_owner: Address) -> bool {
        sender.require_auth();

        let mut ticket = Self::get_ticket(env.clone(), ticket_id);

        if ticket.owner != sender {
            log!(&env, "Only the owner can transfer this ticket");
            return false;
        }

        ticket.owner = new_owner.clone();
        env.storage().instance().set(&TicketKey::Ticket(ticket_id), &ticket);

        log!(&env, "Ticket {} transferred to {}", ticket_id, new_owner);

        true
    }

    // Get ticket info

pub fn get_ticket(env: Env, ticket_id: u64) -> Ticket {
    let key = TicketKey::Ticket(ticket_id);

    let dummy_address = Address::from_str(&env, "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF"); // Placeholder account

    env.storage().instance().get(&key).unwrap_or(Ticket {
        ticket_id: 0,
        event_name: String::from_str(&env, ""),
        ticket_price: 0,
        owner: dummy_address,
    })
}
}
