use anchor_lang::prelude::*;

// Anchor macros are used for account struct definitions
// `#[account]` means that this is an account struct.

// This struct represents the global state of the contract,
// stored in a Solana account marked by the GlobalPool
#[account]
#[derive(Default)]
pub struct GlobalPool {
    // super_admin stores the Pubkey (public key) of the admin user.
    pub super_admin: Pubkey, // 32 bytes
}

// An AuctionPool represents each auction on this contract
#[account(zero_copy)] // zero_copy for faster deserialization of the data
pub struct AuctionPool {
    // The seller's public key
    pub seller: Pubkey, // 32 bytes

    // The minter's (creator's) public key of the NFT
    pub nft_mint: Pubkey, // 32 bytes

    // The collection's public key in which NFT belongs
    pub nft_collection: Pubkey, // 32 bytes

    // The highest bidder's public key for this auction
    pub bidder: Pubkey, // 32 bytes

    // The amount of the highest bid for this auction
    pub current_bid: u64, // 8 bytes

    // The starting price of the auction
    pub start_price: u64, // 8 bytes

    // The timestamp of when the auction ends
    pub end_time: u64, // 8 bytes
}

// Provide a `default` constructor for `AuctionPool`
// This is the state of an `AuctionPool` when it is first created
impl Default for AuctionPool {
    #[inline]
    fn default() -> AuctionPool {
        // By default, all pubkeys are set to the default pubkey (all zeros),
        // bidding-related fields are set to zero
        AuctionPool {
            seller: Pubkey::default(),

            nft_mint: Pubkey::default(),
            nft_collection: Pubkey::default(),

            bidder: Pubkey::default(),
            current_bid: 0,

            start_price: 0,
            end_time: 0,
        }
    }
}
