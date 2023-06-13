use anchor_lang::prelude::*;

declare_id!("Bas2BRWWw2hXSptzWfDWkBcegQ1MjM9RzqzespQLmDD7");

#[program]
pub mod hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
