use anchor_lang::prelude::*;

declare_id!("W9kWQdPdHQLQ3PBES4pYMEfPQKdsMqRSDSv8zCsZQWe");

pub const ORDER_STATUS_PENDING: u8 = 0;
pub const ORDER_STATUS_SETTLED: u8 = 1;
pub const ORDER_STATUS_CANCELLED: u8 = 2;

#[program]
pub mod chispazo_ledger {
    use super::*;

    pub fn initialize_merchant(
        ctx: Context<InitializeMerchant>,
        merchant_code: [u8; 16],
        clabe_hash: [u8; 32],
    ) -> Result<()> {
        let merchant = &mut ctx.accounts.merchant_config;
        merchant.authority = ctx.accounts.authority.key();
        merchant.merchant_code = merchant_code;
        merchant.clabe_hash = clabe_hash;
        merchant.is_active = true;
        merchant.order_count = 0;
        merchant.bump = ctx.bumps.merchant_config;
        Ok(())
    }

    pub fn create_payment_order(
        ctx: Context<CreatePaymentOrder>,
        module_type: u8,
        amount_cents: u64,
        spei_ref_hash: [u8; 32],
    ) -> Result<()> {
        require!(amount_cents > 0, ChispazoError::InvalidAmount);
        require!(module_type <= 4, ChispazoError::InvalidModule);

        let merchant = &mut ctx.accounts.merchant_config;
        require!(merchant.is_active, ChispazoError::MerchantInactive);

        let order_seq = merchant
            .order_count
            .checked_add(1)
            .ok_or(ChispazoError::OrderOverflow)?;
        merchant.order_count = order_seq;

        let clock = Clock::get()?;
        let order = &mut ctx.accounts.payment_order;
        order.merchant = merchant.key();
        order.authority = merchant.authority;
        order.order_seq = order_seq;
        order.module_type = module_type;
        order.amount_cents = amount_cents;
        order.spei_ref_hash = spei_ref_hash;
        order.status = ORDER_STATUS_PENDING;
        order.created_at = clock.unix_timestamp;
        order.settled_at = 0;
        order.bump = ctx.bumps.payment_order;

        Ok(())
    }

    pub fn update_order_status(
        ctx: Context<UpdateOrderStatus>,
        new_status: u8,
        spei_ref_hash: [u8; 32],
    ) -> Result<()> {
        let order = &mut ctx.accounts.payment_order;
        require!(
            order.status == ORDER_STATUS_PENDING,
            ChispazoError::OrderNotPending
        );
        require!(
            new_status == ORDER_STATUS_SETTLED || new_status == ORDER_STATUS_CANCELLED,
            ChispazoError::InvalidStatusTransition
        );

        let clock = Clock::get()?;
        order.status = new_status;
        order.spei_ref_hash = spei_ref_hash;
        if new_status == ORDER_STATUS_SETTLED {
            order.settled_at = clock.unix_timestamp;
        }

        Ok(())
    }

    pub fn close_payment_order(ctx: Context<ClosePaymentOrder>) -> Result<()> {
        let order = &ctx.accounts.payment_order;
        require!(
            order.status == ORDER_STATUS_SETTLED || order.status == ORDER_STATUS_CANCELLED,
            ChispazoError::OrderStillPendingCannotClose
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeMerchant<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + MerchantConfig::INIT_SPACE,
        seeds = [b"merchant", authority.key().as_ref()],
        bump
    )]
    pub merchant_config: Account<'info, MerchantConfig>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreatePaymentOrder<'info> {
    #[account(
        mut,
        seeds = [b"merchant", authority.key().as_ref()],
        bump = merchant_config.bump,
        has_one = authority @ ChispazoError::UnauthorizedAuthority
    )]
    pub merchant_config: Account<'info, MerchantConfig>,

    #[account(
        init,
        payer = authority,
        space = 8 + PaymentOrder::INIT_SPACE,
        seeds = [
            b"order",
            merchant_config.key().as_ref(),
            &merchant_config.order_count.checked_add(1).unwrap_or(0).to_le_bytes()
        ],
        bump
    )]
    pub payment_order: Account<'info, PaymentOrder>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateOrderStatus<'info> {
    #[account(
        seeds = [b"merchant", authority.key().as_ref()],
        bump = merchant_config.bump,
        has_one = authority @ ChispazoError::UnauthorizedAuthority
    )]
    pub merchant_config: Account<'info, MerchantConfig>,

    #[account(
        mut,
        seeds = [
            b"order",
            merchant_config.key().as_ref(),
            &payment_order.order_seq.to_le_bytes()
        ],
        bump = payment_order.bump,
        has_one = authority @ ChispazoError::UnauthorizedAuthority,
        constraint = payment_order.merchant == merchant_config.key() @ ChispazoError::OrderMerchantMismatch
    )]
    pub payment_order: Account<'info, PaymentOrder>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ClosePaymentOrder<'info> {
    #[account(
        seeds = [b"merchant", authority.key().as_ref()],
        bump = merchant_config.bump,
        has_one = authority @ ChispazoError::UnauthorizedAuthority
    )]
    pub merchant_config: Account<'info, MerchantConfig>,

    #[account(
        mut,
        close = authority,
        seeds = [
            b"order",
            merchant_config.key().as_ref(),
            &payment_order.order_seq.to_le_bytes()
        ],
        bump = payment_order.bump,
        has_one = authority @ ChispazoError::UnauthorizedAuthority,
        constraint = payment_order.merchant == merchant_config.key() @ ChispazoError::OrderMerchantMismatch
    )]
    pub payment_order: Account<'info, PaymentOrder>,

    #[account(mut)]
    pub authority: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct MerchantConfig {
    pub authority: Pubkey,
    pub merchant_code: [u8; 16],
    pub clabe_hash: [u8; 32],
    pub is_active: bool,
    pub order_count: u64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct PaymentOrder {
    pub merchant: Pubkey,
    pub authority: Pubkey,
    pub order_seq: u64,
    pub module_type: u8,
    pub amount_cents: u64,
    pub spei_ref_hash: [u8; 32],
    pub status: u8,
    pub created_at: i64,
    pub settled_at: i64,
    pub bump: u8,
}

#[error_code]
pub enum ChispazoError {
    #[msg("La autoridad firmante no corresponde al comercio")]
    UnauthorizedAuthority,
    #[msg("La orden no pertenece al comercio indicado")]
    OrderMerchantMismatch,
    #[msg("El monto debe ser mayor a cero")]
    InvalidAmount,
    #[msg("Modulo comercial invalido")]
    InvalidModule,
    #[msg("El comercio no se encuentra activo")]
    MerchantInactive,
    #[msg("Desbordamiento en el consecutivo de ordenes")]
    OrderOverflow,
    #[msg("Solo se pueden actualizar ordenes en estado pendiente")]
    OrderNotPending,
    #[msg("Transicion de estado no permitida")]
    InvalidStatusTransition,
    #[msg("No se puede cerrar una orden pendiente; debe estar liquidada o cancelada")]
    OrderStillPendingCannotClose,
}