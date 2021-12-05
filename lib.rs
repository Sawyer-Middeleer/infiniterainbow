use anchor_lang::prelude::*;

declare_id!("CX3B1Cy76tCXBWsf1njTHHvKivicW2NgPBATKp6Dqs82");

#[program]
pub mod infiniterainbow {
  use super::*;
  pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
    // Get a reference to the account.
    let base_account = &mut ctx.accounts.base_account;
    // Initialize total_colors.
    base_account.total_colors = 0;
    Ok(())
  }


pub fn add_color(ctx: Context<AddColor>, color_code: String) -> ProgramResult {
    // Get a reference to the account and increment total_colors.
    let base_account = &mut ctx.accounts.base_account;
    let user = &mut ctx.accounts.user;

    // Build the struct.
    let item = ItemStruct {
      color_code: color_code.to_string(),
      user_address: *user.to_account_info().key,
    };
		
	// Add it to the color_list vector.
    base_account.color_list.push(item);
    base_account.total_colors += 1;
    Ok(())
  }
}

// Attach certain variables to the StartStuffOff context.
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 9001)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

#[derive(Accounts)]
pub struct AddColor<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub color_code: String,
    pub user_address: Pubkey,
}

// Tell Solana what we want to store on this account.
#[account]
pub struct BaseAccount {
    pub total_colors: u64,
    pub color_list: Vec<ItemStruct>,
}
