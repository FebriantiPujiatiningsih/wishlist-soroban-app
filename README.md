# Stellar Wishlist Vault DApp

Stellar Wishlist Vault DApp is a decentralized personal wishlist and target tracking application built using Soroban Smart Contract on Stellar Testnet.

## Project Description

Stellar Wishlist Vault DApp helps users store and manage their personal wishlist items on the blockchain. Users can record desired items such as gadgets, books, skincare, travel goals, or personal targets.

Each wishlist item contains an item name, category, target price, priority level, and achievement status. The data is stored using Soroban smart contract storage, making the wishlist more transparent and decentralized.

## Project Vision

The vision of Stellar Wishlist Vault DApp is to help individuals organize their personal goals and desired items in a secure and decentralized way.

This project combines personal target tracking with blockchain technology, allowing users to keep meaningful wishlist records without relying on a centralized database.

## Key Features

- Create a personal wishlist item
- Add item category
- Add target price
- Add priority level
- View all wishlist items
- Mark wishlist item as achieved
- Update wishlist priority
- Delete wishlist item by ID
- Store wishlist data using Soroban smart contract storage

## Smart Contract Functions

### get_wishlist

Returns all wishlist items stored in the smart contract.

### create_wishlist

Creates a new wishlist item with item name, category, target price, and priority.

### achieve_wishlist

Marks a wishlist item as achieved based on its ID.

### update_priority

Updates the priority level of a wishlist item.

### delete_wishlist

Deletes a wishlist item based on its ID.

## Data Structure

```rust
pub struct WishlistItem {
    id: u64,
    item_name: String,
    category: String,
    target_price: u64,
    priority: String,
    is_achieved: bool,
}

## Deployment Information

Network: Stellar Testnet  
Smart Contract ID: `CDYUV5R2XHQE3T7ID3TRJLUX22GXP725RRXYMEL6AFMLXHGU65YM4Z2F`
