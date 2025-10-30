use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("Bm4wfAHNqdaWBN7HyFfiGx8PShFpVmaKyBMRyLxxnBor");

const SECONDS_PER_DAY: i64 = 86400;

#[program]
pub mod subwave {
    use super::*;

    /// Register a merchant and create a subscription configuration
    pub fn register_merchant(
        ctx: Context<RegisterMerchant>,
        price: u64,
        interval_days: u32,
        product_name: String,
    ) -> Result<()> {
        require!(price > 0, ErrorCode::InvalidPrice);
        require!(interval_days > 0, ErrorCode::InvalidInterval);
        require!(product_name.len() <= 50, ErrorCode::ProductNameTooLong);

        let merchant = &mut ctx.accounts.merchant;
        merchant.authority = ctx.accounts.authority.key();
        merchant.token_mint = ctx.accounts.token_mint.key();
        merchant.bump = ctx.bumps.merchant;

        let config = &mut ctx.accounts.subscription_config;
        config.merchant = merchant.key();
        config.price = price;
        config.interval_days = interval_days;
        config.product_name = product_name.clone();
        config.active = true;
        config.bump = ctx.bumps.subscription_config;

        msg!("Merchant registered with product: {}", product_name);
        msg!("Price: {} tokens, Interval: {} days", price, interval_days);

        Ok(())
    }

    /// Create a new subscription for a user
    pub fn create_subscription(
        ctx: Context<CreateSubscription>,
    ) -> Result<()> {
        let config = &ctx.accounts.subscription_config;
        require!(config.active, ErrorCode::SubscriptionInactive);

        let subscription = &mut ctx.accounts.subscription;
        let clock = Clock::get()?;
        
        // Calculate expiry timestamp
        let interval_seconds = config.interval_days as i64 * SECONDS_PER_DAY;
        let expiry = clock.unix_timestamp
            .checked_add(interval_seconds)
            .ok_or(ErrorCode::MathOverflow)?;

        subscription.subscriber = ctx.accounts.subscriber.key();
        subscription.merchant = ctx.accounts.merchant.key();
        subscription.subscription_config = config.key();
        subscription.start_timestamp = clock.unix_timestamp;
        subscription.expiry_timestamp = expiry;
        subscription.active = true;
        subscription.total_paid = config.price;
        subscription.bump = ctx.bumps.subscription;

        // Transfer payment from subscriber to merchant
        let cpi_accounts = Transfer {
            from: ctx.accounts.subscriber_token_account.to_account_info(),
            to: ctx.accounts.merchant_token_account.to_account_info(),
            authority: ctx.accounts.subscriber.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, config.price)?;

        msg!("Subscription created for subscriber: {}", subscription.subscriber);
        msg!("Expires at: {}", expiry);

        Ok(())
    }

    /// Renew an existing subscription
    pub fn renew_subscription(
        ctx: Context<RenewSubscription>,
    ) -> Result<()> {
        let subscription = &mut ctx.accounts.subscription;
        let config = &ctx.accounts.subscription_config;
        
        require!(subscription.active, ErrorCode::SubscriptionInactive);
        require!(config.active, ErrorCode::SubscriptionInactive);

        let clock = Clock::get()?;
        
        // Extend from current expiry or from now if expired
        let base_time = if subscription.expiry_timestamp > clock.unix_timestamp {
            subscription.expiry_timestamp
        } else {
            clock.unix_timestamp
        };

        let interval_seconds = config.interval_days as i64 * SECONDS_PER_DAY;
        let new_expiry = base_time
            .checked_add(interval_seconds)
            .ok_or(ErrorCode::MathOverflow)?;

        subscription.expiry_timestamp = new_expiry;
        subscription.total_paid = subscription.total_paid
            .checked_add(config.price)
            .ok_or(ErrorCode::MathOverflow)?;

        // Transfer payment from subscriber to merchant
        let cpi_accounts = Transfer {
            from: ctx.accounts.subscriber_token_account.to_account_info(),
            to: ctx.accounts.merchant_token_account.to_account_info(),
            authority: ctx.accounts.subscriber.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, config.price)?;

        msg!("Subscription renewed until: {}", new_expiry);

        Ok(())
    }

    /// Cancel an active subscription
    pub fn cancel_subscription(
        ctx: Context<CancelSubscription>,
    ) -> Result<()> {
        let subscription = &mut ctx.accounts.subscription;
        
        require!(subscription.active, ErrorCode::SubscriptionAlreadyCanceled);
        
        subscription.active = false;

        msg!("Subscription canceled for: {}", subscription.subscriber);

        Ok(())
    }

    /// Check if a subscription is currently valid and provides entitlement
    pub fn check_entitlement(
        ctx: Context<CheckEntitlement>,
    ) -> Result<()> {
        let subscription = &ctx.accounts.subscription;
        let clock = Clock::get()?;

        let has_entitlement = subscription.active 
            && subscription.expiry_timestamp > clock.unix_timestamp;

        if has_entitlement {
            msg!("Entitlement VALID - Expires: {}", subscription.expiry_timestamp);
        } else {
            msg!("Entitlement INVALID - Active: {}, Expiry: {}", 
                subscription.active, 
                subscription.expiry_timestamp);
        }

        // Return the entitlement status in account data
        // Callers can check the logs or implement custom return handling
        
        Ok(())
    }

    /// Update subscription configuration (merchant only)
    pub fn update_subscription_config(
        ctx: Context<UpdateSubscriptionConfig>,
        new_price: Option<u64>,
        new_interval_days: Option<u32>,
        new_active: Option<bool>,
    ) -> Result<()> {
        let config = &mut ctx.accounts.subscription_config;

        if let Some(price) = new_price {
            require!(price > 0, ErrorCode::InvalidPrice);
            config.price = price;
            msg!("Price updated to: {}", price);
        }

        if let Some(interval) = new_interval_days {
            require!(interval > 0, ErrorCode::InvalidInterval);
            config.interval_days = interval;
            msg!("Interval updated to: {} days", interval);
        }

        if let Some(active) = new_active {
            config.active = active;
            msg!("Active status updated to: {}", active);
        }

        Ok(())
    }
}

// ============================================================================
// Account Structures
// ============================================================================

#[account]
pub struct Merchant {
    /// The merchant's authority (owner)
    pub authority: Pubkey,
    /// Token mint for payments (USDC, SOL, etc.)
    pub token_mint: Pubkey,
    /// PDA bump
    pub bump: u8,
}

impl Merchant {
    pub const LEN: usize = 8 + // discriminator
                          32 + // authority
                          32 + // token_mint
                          1;   // bump
}

#[account]
pub struct SubscriptionConfig {
    /// Reference to the merchant
    pub merchant: Pubkey,
    /// Price per subscription period (in token base units)
    pub price: u64,
    /// Subscription interval in days
    pub interval_days: u32,
    /// Product/service name
    pub product_name: String,
    /// Whether this subscription is active
    pub active: bool,
    /// PDA bump
    pub bump: u8,
}

impl SubscriptionConfig {
    pub const LEN: usize = 8 +   // discriminator
                          32 +   // merchant
                          8 +    // price
                          4 +    // interval_days
                          4 + 50 + // product_name (String with max 50 chars)
                          1 +    // active
                          1;     // bump
}

#[account]
pub struct Subscription {
    /// The subscriber's address
    pub subscriber: Pubkey,
    /// The merchant's address
    pub merchant: Pubkey,
    /// Reference to subscription config
    pub subscription_config: Pubkey,
    /// When the subscription started
    pub start_timestamp: i64,
    /// When the subscription expires
    pub expiry_timestamp: i64,
    /// Whether the subscription is active
    pub active: bool,
    /// Total amount paid over lifetime
    pub total_paid: u64,
    /// PDA bump
    pub bump: u8,
}

impl Subscription {
    pub const LEN: usize = 8 +  // discriminator
                          32 +  // subscriber
                          32 +  // merchant
                          32 +  // subscription_config
                          8 +   // start_timestamp
                          8 +   // expiry_timestamp
                          1 +   // active
                          8 +   // total_paid
                          1;    // bump
}

// ============================================================================
// Context Structures
// ============================================================================

#[derive(Accounts)]
#[instruction(price: u64, interval_days: u32, product_name: String)]
pub struct RegisterMerchant<'info> {
    #[account(
        init,
        payer = authority,
        space = Merchant::LEN,
        seeds = [b"merchant", authority.key().as_ref(), token_mint.key().as_ref()],
        bump
    )]
    pub merchant: Account<'info, Merchant>,

    #[account(
        init,
        payer = authority,
        space = SubscriptionConfig::LEN,
        seeds = [b"config", merchant.key().as_ref(), product_name.as_bytes()],
        bump
    )]
    pub subscription_config: Account<'info, SubscriptionConfig>,

    /// Token mint for subscription payments
    pub token_mint: AccountInfo<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateSubscription<'info> {
    #[account(
        init,
        payer = subscriber,
        space = Subscription::LEN,
        seeds = [
            b"subscription",
            subscriber.key().as_ref(),
            subscription_config.key().as_ref()
        ],
        bump
    )]
    pub subscription: Account<'info, Subscription>,

    #[account(
        seeds = [b"merchant", merchant.authority.as_ref(), merchant.token_mint.as_ref()],
        bump = merchant.bump
    )]
    pub merchant: Account<'info, Merchant>,

    #[account(
        seeds = [b"config", merchant.key().as_ref(), subscription_config.product_name.as_bytes()],
        bump = subscription_config.bump
    )]
    pub subscription_config: Account<'info, SubscriptionConfig>,

    #[account(mut)]
    pub subscriber: Signer<'info>,

    #[account(
        mut,
        constraint = subscriber_token_account.owner == subscriber.key(),
        constraint = subscriber_token_account.mint == merchant.token_mint
    )]
    pub subscriber_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = merchant_token_account.mint == merchant.token_mint
    )]
    pub merchant_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RenewSubscription<'info> {
    #[account(
        mut,
        seeds = [
            b"subscription",
            subscriber.key().as_ref(),
            subscription_config.key().as_ref()
        ],
        bump = subscription.bump,
        constraint = subscription.subscriber == subscriber.key(),
    )]
    pub subscription: Account<'info, Subscription>,

    #[account(
        seeds = [b"merchant", merchant.authority.as_ref(), merchant.token_mint.as_ref()],
        bump = merchant.bump
    )]
    pub merchant: Account<'info, Merchant>,

    #[account(
        seeds = [b"config", merchant.key().as_ref(), subscription_config.product_name.as_bytes()],
        bump = subscription_config.bump
    )]
    pub subscription_config: Account<'info, SubscriptionConfig>,

    #[account(mut)]
    pub subscriber: Signer<'info>,

    #[account(
        mut,
        constraint = subscriber_token_account.owner == subscriber.key(),
        constraint = subscriber_token_account.mint == merchant.token_mint
    )]
    pub subscriber_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = merchant_token_account.mint == merchant.token_mint
    )]
    pub merchant_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct CancelSubscription<'info> {
    #[account(
        mut,
        seeds = [
            b"subscription",
            subscriber.key().as_ref(),
            subscription.subscription_config.as_ref()
        ],
        bump = subscription.bump,
        constraint = subscription.subscriber == subscriber.key() @ ErrorCode::Unauthorized
    )]
    pub subscription: Account<'info, Subscription>,

    pub subscriber: Signer<'info>,
}

#[derive(Accounts)]
pub struct CheckEntitlement<'info> {
    #[account(
        seeds = [
            b"subscription",
            subscription.subscriber.as_ref(),
            subscription.subscription_config.as_ref()
        ],
        bump = subscription.bump
    )]
    pub subscription: Account<'info, Subscription>,
}

#[derive(Accounts)]
pub struct UpdateSubscriptionConfig<'info> {
    #[account(
        mut,
        seeds = [b"config", merchant.key().as_ref(), subscription_config.product_name.as_bytes()],
        bump = subscription_config.bump,
        constraint = subscription_config.merchant == merchant.key()
    )]
    pub subscription_config: Account<'info, SubscriptionConfig>,

    #[account(
        seeds = [b"merchant", authority.key().as_ref(), merchant.token_mint.as_ref()],
        bump = merchant.bump,
        constraint = merchant.authority == authority.key() @ ErrorCode::Unauthorized
    )]
    pub merchant: Account<'info, Merchant>,

    pub authority: Signer<'info>,
}

// ============================================================================
// Error Codes
// ============================================================================

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid price: must be greater than 0")]
    InvalidPrice,
    #[msg("Invalid interval: must be greater than 0 days")]
    InvalidInterval,
    #[msg("Product name too long: max 50 characters")]
    ProductNameTooLong,
    #[msg("Subscription is not active")]
    SubscriptionInactive,
    #[msg("Subscription already canceled")]
    SubscriptionAlreadyCanceled,
    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("Math overflow")]
    MathOverflow,
}
