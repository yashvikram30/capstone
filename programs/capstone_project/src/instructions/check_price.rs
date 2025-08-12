use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};
 
#[derive(Accounts)]
pub struct CheckPrice<'info> {
    
    pub price_update: Account<'info, PriceUpdateV2>,
}

impl <'info> CheckPrice <'info> {
    
    pub fn check_price(&mut self) -> Result<()>{

    let price_update = &self.price_update;

    // get_price_no_older_than will fail if the price update is more than 30 seconds old
    let maximum_age: u64 = 60;

    // get_price_no_older_than will fail if the price update is for a different price feed.
    // This string is the id of the BTC/USD feed. See https://docs.pyth.network/price-feeds/price-feeds for all available IDs.
    let feed_id: [u8; 32] = get_feed_id_from_hex("0xef0d8b614545449452e9f8d4623e34ade2ba2ac67362100e27457bf6fc8894c4")?;

    let price = price_update.get_price_no_older_than(&Clock::get()?, maximum_age, &feed_id)?;
    
    // Sample output:
    // The price is (7160106530699 ± 5129162301) * 10^-8
    msg!("The price is ({} ± {}) * 10^{}", price.price, price.conf, price.exponent);
 
    Ok(())
    }
}