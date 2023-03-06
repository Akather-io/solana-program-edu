use anchor_lang::prelude::*;

declare_id!("3WnkE7WM2yM6fPL3FtV4geVXxFq6FLm86kMvP9mrwdg8");

#[program]
pub mod solana_program_edu {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
