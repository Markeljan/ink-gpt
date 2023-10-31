#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract] mod incrementer { 
    use ink::storage::Mapping;

#[ink(event)]
pub struct Incremented {
    from: Option<AccountId>,
    value: u32,
}

#[ink(storage)]
pub struct Incrementer {
    value: u32,
    my_map: Mapping<AccountId, u32>,
}

impl Incrementer {
    #[ink(constructor)]
    pub fn new(init_value: u32) -> Self {
        let mut my_map = Mapping::default();
        let from = Self::env().caller();
        my_map.insert(&from, &init_value);

        Self {
            value: init_value,
            my_map,
        }
    }

    #[ink(constructor)]
    pub fn default() -> Self {
        Self::new(Default::default())
    }

    #[ink(message)]
    pub fn get(&self, key: AccountId) -> u32 {
        self.my_map.get(&key).unwrap_or(0)
    }

    #[ink(message)]
    pub fn inc(&mut self, by: u32) {
        let from = self.env().caller();
        let value = self.value.checked_add(by).expect("overflow");

        self.value = value;
        self.my_map.insert(from, &value);

        self.env().emit_event(Incremented {
            from: Some(from),
            value: value,
        });
    }

    #[ink(message)]
    pub fn dinc(&mut self, by: u32) {
        let from = self.env().caller();
        let value = self.value.checked_sub(by).expect("underflow");

        self.value = value;
        self.my_map.insert(from, &value);

        self.env().emit_event(Incremented {
            from: Some(from),
            value: value,
        });
    }

    #[ink(message)]
    pub fn get_value(&self) -> u32 {
        self.value
    }
}
}