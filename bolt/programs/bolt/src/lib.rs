use anchor_lang::prelude::*;

declare_id!("GetPeJuUn9dabeSR98haGpQJEYdvvznQYD8yBuXwca7d");

#[program]
pub mod bolt {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
