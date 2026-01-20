use anchor_lang::prelude::*;

#[derive(InitSpace)]
#[account]
pub struct Offer {
    // 订单 id
    pub id: u64,
    // 订单创建者
    pub maker: Pubkey,
    // 创建者拥有的 a mint 代币
    pub token_mint_a: Pubkey,
    // 创建者要兑换的 b mint 代币
    pub token_mint_b: Pubkey,
    // 创建者想要兑换的 b 代币的数量
    pub token_b_wanted_amount: u64,
    // pad bump
    pub bump: u8,
}