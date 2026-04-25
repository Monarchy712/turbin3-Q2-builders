use anchor_lang::prelude::*;

declare_id!("3wPRRxVRnsYod51Qzm8Bgk8Sqam7PoPU4sh1ot1AciKb");

#[program]
pub mod test_hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
