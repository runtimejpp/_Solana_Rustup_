use anchor_lang::prelude::*;

declare_id!("BSVD6Mz8KVpWHeBomeWFMd6sfX9arMxeYkWR8ub6QEpD");

#[program]
pub mod mycalculatordapp {
    use super::*;
    pub fn create(ctx: Context<Create>, init_message: String) -> String {
        let calculator = &mut ctx.account.calculator;
        calculator.greeting = init_message;
        Ok(());
    }
    pub fn add(ctx: Context<Addition>, num1:  i64, num2: i64,) -> ProgramResult{
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 + num2;
        ok(())
    }


}

#[derive(Accounts)]   
pub struct Create<'info>{
    #[account(init, payer=user, space=264)] 
    pub calculator: Account<'info, Calculator>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct Calculator{
    pub greeting: String,
    pub result: i64,
    pub remainder: i64 //
}