use std::collections::HashMap;

use thiserror::Error;

use super::Address;
use spec::types::Coin;

#[derive(Error, PartialEq, Eq, Debug)]
pub enum AccountDatabaseError {
    #[error("Sender account does not exist")]
    SenderAccountDoesNotExist,

    #[error("Insufficient funds")]
    InsufficientFunds,
}

#[derive(Debug, Default, Clone)]
pub struct AccountDatabase(HashMap<Address, Coin>);

impl AccountDatabase {
    pub fn add_funds(&mut self, address: &Address, new_funds: Coin) {
        let current_balance = self.get_account_balance(address).unwrap_or(0);
        self.update_funds(address, current_balance + new_funds);
    }

    pub fn transfer(
        &mut self,
        sender: &Address,
        recipient: &Address,
        amount: Coin,
    ) -> Result<(), AccountDatabaseError> {
        let sender_balance = self.get_sender_balance(sender)?;
        let recipient_balance = self.get_recipient_balance(recipient);

        if sender_balance < amount {
            return Err(AccountDatabaseError::InsufficientFunds);
        }

        self.update_funds(sender, sender_balance - amount);
        self.update_funds(recipient, recipient_balance + amount);

        Ok(())
    }

    pub fn get_account_balance(&self, address: &Address) -> Option<Coin> {
        self.0.get(address).cloned()
    }

    fn get_recipient_balance(&self, address: &Address) -> Coin {
        match self.0.get(address) {
            Some(amount) => *amount,
            None => 0,
        }
    }

    fn get_sender_balance(&self, address: &Address) -> Result<Coin, AccountDatabaseError> {
        match self.0.get(address) {
            Some(balance) => Ok(*balance),
            None => Err(AccountDatabaseError::SenderAccountDoesNotExist),
        }
    }

    fn update_funds(&mut self, address: &Address, new_balance: Coin) {
        let balance = self.0.entry(address.clone()).or_insert(0);
        *balance = new_balance;
    }
}
