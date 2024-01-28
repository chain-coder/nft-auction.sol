import * as anchor from '@project-serum/anchor';
import { PublicKey } from '@solana/web3.js';

// Seed strings for creating program-derived addresses.
export const GLOBAL_AUTHORITY_SEED = "global-authority";
export const AUCTION_SEED = "auction";

// Auction account data size in bytes.
export const AUCTION_SIZE = 160;

// Data structure layout for an auction state.
export interface AuctionPool {
    seller: PublicKey,          // 32 bytes: Auction holder's public key
    nftMint: PublicKey,         // 32 bytes: NFT being auctioned public key
    nftCollection: PublicKey,   // 32 bytes: Corresponding NFT collection public key

    bidder: PublicKey,          // 32 bytes: Current highest bidder public key
    currentBid: anchor.BN,      // 8 bytes: Current highest bid

    startPrice: anchor.BN,      // 8 bytes: Auction's start price
    endTime: anchor.BN,         // 8 bytes: Auction's end time (unix timestamp)
}