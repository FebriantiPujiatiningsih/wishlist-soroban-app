#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec,
};

#[contracttype]
#[derive(Clone, Debug)]
pub struct WishlistItem {
    id: u64,
    item_name: String,
    category: String,
    target_price: u64,
    priority: String,
    is_achieved: bool,
}

const WISHLIST_DATA: Symbol = symbol_short!("WISHES");

#[contract]
pub struct WishlistVaultContract;

#[contractimpl]
impl WishlistVaultContract {
    pub fn get_wishlist(env: Env) -> Vec<WishlistItem> {
        env.storage()
            .instance()
            .get(&WISHLIST_DATA)
            .unwrap_or(Vec::new(&env))
    }

    pub fn create_wishlist(
        env: Env,
        item_name: String,
        category: String,
        target_price: u64,
        priority: String,
    ) -> String {
        let mut wishlist: Vec<WishlistItem> = env
            .storage()
            .instance()
            .get(&WISHLIST_DATA)
            .unwrap_or(Vec::new(&env));

        let item = WishlistItem {
            id: env.prng().gen::<u64>(),
            item_name,
            category,
            target_price,
            priority,
            is_achieved: false,
        };

        wishlist.push_back(item);
        env.storage().instance().set(&WISHLIST_DATA, &wishlist);

        String::from_str(&env, "Wishlist item berhasil ditambahkan")
    }

    pub fn achieve_wishlist(env: Env, id: u64) -> String {
        let mut wishlist: Vec<WishlistItem> = env
            .storage()
            .instance()
            .get(&WISHLIST_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..wishlist.len() {
            let mut item = wishlist.get(i).unwrap();

            if item.id == id {
                item.is_achieved = true;
                wishlist.set(i, item);

                env.storage().instance().set(&WISHLIST_DATA, &wishlist);

                return String::from_str(&env, "Wishlist berhasil ditandai tercapai");
            }
        }

        String::from_str(&env, "Wishlist item tidak ditemukan")
    }

    pub fn update_priority(env: Env, id: u64, new_priority: String) -> String {
        let mut wishlist: Vec<WishlistItem> = env
            .storage()
            .instance()
            .get(&WISHLIST_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..wishlist.len() {
            let mut item = wishlist.get(i).unwrap();

            if item.id == id {
                item.priority = new_priority;
                wishlist.set(i, item);

                env.storage().instance().set(&WISHLIST_DATA, &wishlist);

                return String::from_str(&env, "Priority wishlist berhasil diperbarui");
            }
        }

        String::from_str(&env, "Wishlist item tidak ditemukan")
    }

    pub fn delete_wishlist(env: Env, id: u64) -> String {
        let mut wishlist: Vec<WishlistItem> = env
            .storage()
            .instance()
            .get(&WISHLIST_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..wishlist.len() {
            if wishlist.get(i).unwrap().id == id {
                wishlist.remove(i);

                env.storage().instance().set(&WISHLIST_DATA, &wishlist);

                return String::from_str(&env, "Wishlist item berhasil dihapus");
            }
        }

        String::from_str(&env, "Wishlist item tidak ditemukan")
    }
}

mod test;