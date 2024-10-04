use anchor_lang::prelude::*;

#[account]
#[derive(Default, Debug)]
pub struct Crank {
    pub bump: u8,
    pub signer: Pubkey,
    pub fee: u64,
}

impl Crank {
    pub const PREFIX: &'static str = "CRANK";

    pub const SIZE: usize = 8 + /* discriminator */
        std::mem::size_of::<u8>() + /* bump */
        std::mem::size_of::<Pubkey>() + /* signer */
        std::mem::size_of::<u64>() + /* fee */
        64; /* padding */
}
