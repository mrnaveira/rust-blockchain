use std::collections::HashMap;

use thiserror::Error;

use super::Address;

pub type Amount = u64;

#[derive(Error, PartialEq, Debug)]
pub enum AccountBalanceMapError {
    #[error("Sender account does not exist")]
    SenderAccountDoesNotExist,

    #[error("Insufficient funds")]
    InsufficientFunds,
}

#[derive(Debug, Default, Clone)]
pub struct AccountBalanceMap(HashMap<Address, Amount>);

impl AccountBalanceMap {
    pub fn add_amount(&mut self, recipient: &Address, amount: Amount) {
        let balance = self.get_recipient_balance(recipient);
        self.update_balance(recipient, balance + amount);
    }

    pub fn transfer(
        &mut self,
        sender: &Address,
        recipient: &Address,
        amount: Amount,
    ) -> Result<(), AccountBalanceMapError> {
        let sender_balance = self.get_sender_balance(sender)?;
        let recipient_balance = self.get_recipient_balance(recipient);

        if sender_balance < amount {
            return Err(AccountBalanceMapError::InsufficientFunds);
        }

        self.update_balance(sender, sender_balance - amount);
        self.update_balance(recipient, recipient_balance + amount);

        Ok(())
    }

    fn get_recipient_balance(&self, recipient: &Address) -> Amount {
        match self.0.get(recipient) {
            Some(amount) => *amount,
            None => 0,
        }
    }

    fn get_sender_balance(&self, sender: &Address) -> Result<Amount, AccountBalanceMapError> {
        match self.0.get(sender) {
            Some(balance) => Ok(*balance),
            None => Err(AccountBalanceMapError::SenderAccountDoesNotExist),
        }
    }

    fn update_balance(&mut self, address: &Address, new_balance: Amount) {
        let balance = self.0.entry(address.clone()).or_insert(0);
        *balance = new_balance;
    }
}
