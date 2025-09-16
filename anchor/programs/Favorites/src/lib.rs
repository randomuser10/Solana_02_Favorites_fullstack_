use anchor_lang::prelude::*;

declare_id!("JAVuBXeBZqXNtS73azhBDAoYaaAFfo4gWXoZe2e7Jf8H");

#[program]
pub mod favorites {
    use super::*;

    pub fn set_favorites(_ctx: Context<SetFavorites>,
                               _number: u64,
                               _color: String,
                              _hobbies: Vec<String>,
                            ) -> Result<()> {
        msg!("GM!");
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct Favorites{
    pub _number: u64,

    #[max_len(50)]
    pub _color: String,

    #[max_len(5,50)]
    pub _hobbies: Vec<String>,


}
#[derive(Accounts)]
pub struct SetFavorites<'info > {
    #[account(mut)]
    pub user: Signer<'info >,

    #[account(
        init,
        payer = user,
        space = 8 + Favorites::INIT_SPACE,
        seeds = [b"favorites", user.key().as_ref()],
        bump,
    )]
    pub favorites : Account<'info, Favorites>,

    pub system_program : Program<'info, System>,


}
