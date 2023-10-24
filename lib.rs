#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod membership {
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;

    #[ink(event)]
    pub struct Transfer {
        from: Option<AccountId>,
        to: Option<AccountId>,
        value: Balance,
    }

    #[ink(event)]
    pub struct MembershipGranted {
        account_id: AccountId,
    }

    #[ink(event)]
    pub struct MembershipRevoked {
        account_id: AccountId,
    }

    #[ink(storage)]
    pub struct Membership {
        members: Vec<AccountId>,
        balances: Mapping<AccountId, Balance>,
    }

    impl Membership {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self {
                members: Vec::new(),
                balances: Mapping::default(),
            };

            let caller = Self::env().caller();
            let value: Balance = Default::default();
            instance.balances.insert(caller, &value);

            instance
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new()
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<(), bool> {
            self.ensure_member()?;
            let from = self.env().caller();
            self.transfer_from_to(from, to, value)
        }

        #[ink(message)]
        pub fn grant_membership(&mut self, to: AccountId) -> Result<(), bool> {
            self.ensure_member()?;

            if self.members.contains(&to) {
                return Err(false);
            }

            self.members.push(to.clone());
            self.env().emit_event(MembershipGranted { account_id: to });

            Ok(())
        }

        #[ink(message)]
        pub fn revoke_membership(&mut self, to: AccountId) -> Result<(), bool> {
            self.ensure_member()?;

            let index = match self.members.iter().position(|&r| r == to) {
                Some(index) => index,
                None => return Err(false),
            };

            self.members.swap_remove(index);

            self.env().emit_event(MembershipRevoked { account_id: to });

            Ok(())
        }

        #[ink(message)]
        pub fn members(&self) -> Vec<AccountId> {
            self.members.clone()
        }

        #[ink(message)]
        pub fn balance_of(&self, of: AccountId) -> Balance {
            self.balances.get(&of).unwrap_or(0)
        }

        fn ensure_member(&self) -> Result<(), bool> {
            let caller = self.env().caller();
            if !self.members.contains(&caller) {
                return Err(false);
            }
            Ok(())
        }

        fn transfer_from_to(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<(), bool> {
            let from_balance = self.balance_of(from);
            if from_balance < value {
                return Err(false);
            }

            self.balances.insert(from, &(from_balance - value));

            let to_balance = self.balance_of(to);
            self.balances.insert(to, &(to_balance + value));

            self.env().emit_event(Transfer {
                from: Some(from),
                to: Some(to),
                value,
            });

            Ok(())
        }
    }
}
