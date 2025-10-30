# \# Recurring Stablecoin Micro-Subscription Engine

# 

# A complete Anchor smart contract for managing recurring subscriptions on Solana with USDC or SOL payments.

# 

# \## Features

# 

# ✅ \*\*Merchant Registration\*\* - Register products with price and subscription intervals  

# ✅ \*\*Subscription Management\*\* - Create, renew, and cancel subscriptions  

# ✅ \*\*Entitlement Checking\*\* - Query active subscription status  

# ✅ \*\*Token Payments\*\* - Support for USDC, SOL, or any SPL token  

# ✅ \*\*PDA-based Architecture\*\* - Secure account derivation using seeds  

# ✅ \*\*Timestamp-based Expiry\*\* - Automatic expiry tracking with renewal logic  

# 

# \## Account Structures

# 

# \### Merchant

# Stores merchant information and payment preferences.

# 

# ```rust

# pub struct Merchant {

# &nbsp;   pub authority: Pubkey,     // Merchant owner

# &nbsp;   pub token\_mint: Pubkey,    // Payment token (USDC, SOL, etc.)

# &nbsp;   pub bump: u8,              // PDA bump seed

# }

# ```

# 

# \*\*PDA Seeds:\*\* `\["merchant", authority, token\_mint]`

# 

# \### SubscriptionConfig

# Defines subscription pricing and terms for a product.

# 

# ```rust

# pub struct SubscriptionConfig {

# &nbsp;   pub merchant: Pubkey,         // Reference to merchant

# &nbsp;   pub price: u64,               // Price per period (in token base units)

# &nbsp;   pub interval\_days: u32,       // Subscription interval in days

# &nbsp;   pub product\_name: String,     // Product/service name (max 50 chars)

# &nbsp;   pub active: bool,             // Active status

# &nbsp;   pub bump: u8,                 // PDA bump seed

# }

# ```

# 

# \*\*PDA Seeds:\*\* `\["config", merchant, product\_name]`

# 

# \### Subscription

# Tracks individual user subscriptions.

# 

# ```rust

# pub struct Subscription {

# &nbsp;   pub subscriber: Pubkey,            // User address

# &nbsp;   pub merchant: Pubkey,              // Merchant address

# &nbsp;   pub subscription\_config: Pubkey,   // Config reference

# &nbsp;   pub start\_timestamp: i64,          // When subscription started

# &nbsp;   pub expiry\_timestamp: i64,         // When subscription expires

# &nbsp;   pub active: bool,                  // Active status

# &nbsp;   pub total\_paid: u64,               // Lifetime payment amount

# &nbsp;   pub bump: u8,                      // PDA bump seed

# }

# ```

# 

# \*\*PDA Seeds:\*\* `\["subscription", subscriber, subscription\_config]`

# 

# \## Instructions

# 

# \### 1. register\_merchant

# 

# Register a merchant and create a subscription configuration.

# 

# \*\*Parameters:\*\*

# \- `price: u64` - Subscription price in token base units (e.g., 1000000 = 1 USDC)

# \- `interval\_days: u32` - Subscription duration in days

# \- `product\_name: String` - Product identifier (max 50 chars)

# 

# \*\*Accounts:\*\*

# \- `merchant` - PDA to create (init)

# \- `subscription\_config` - PDA to create (init)

# \- `token\_mint` - SPL token mint for payments

# \- `authority` - Merchant owner (signer, payer)

# \- `system\_program` - System program

# 

# \*\*Example:\*\*

# ```typescript

# await program.methods

# &nbsp; .registerMerchant(

# &nbsp;   new BN(1\_000\_000),  // 1 USDC (6 decimals)

# &nbsp;   30,                 // 30 days

# &nbsp;   "Premium Plan"

# &nbsp; )

# &nbsp; .accounts({

# &nbsp;   merchant: merchantPda,

# &nbsp;   subscriptionConfig: configPda,

# &nbsp;   tokenMint: usdcMint,

# &nbsp;   authority: merchantKeypair.publicKey,

# &nbsp;   systemProgram: SystemProgram.programId,

# &nbsp; })

# &nbsp; .signers(\[merchantKeypair])

# &nbsp; .rpc();

# ```

# 

# \### 2. create\_subscription

# 

# Subscribe a user to a merchant's product.

# 

# \*\*Parameters:\*\* None (reads from config)

# 

# \*\*Accounts:\*\*

# \- `subscription` - PDA to create (init)

# \- `merchant` - Merchant PDA

# \- `subscription\_config` - Config PDA

# \- `subscriber` - User wallet (signer, payer)

# \- `subscriber\_token\_account` - User's token account

# \- `merchant\_token\_account` - Merchant's token account

# \- `token\_program` - SPL Token program

# \- `system\_program` - System program

# 

# \*\*Effects:\*\*

# \- Creates subscription account

# \- Transfers payment from subscriber to merchant

# \- Sets expiry timestamp based on interval

# 

# \*\*Example:\*\*

# ```typescript

# await program.methods

# &nbsp; .createSubscription()

# &nbsp; .accounts({

# &nbsp;   subscription: subscriptionPda,

# &nbsp;   merchant: merchantPda,

# &nbsp;   subscriptionConfig: configPda,

# &nbsp;   subscriber: userKeypair.publicKey,

# &nbsp;   subscriberTokenAccount: userTokenAccount,

# &nbsp;   merchantTokenAccount: merchantTokenAccount,

# &nbsp;   tokenProgram: TOKEN\_PROGRAM\_ID,

# &nbsp;   systemProgram: SystemProgram.programId,

# &nbsp; })

# &nbsp; .signers(\[userKeypair])

# &nbsp; .rpc();

# ```

# 

# \### 3. renew\_subscription

# 

# Renew an existing subscription.

# 

# \*\*Parameters:\*\* None

# 

# \*\*Accounts:\*\*

# \- `subscription` - Existing subscription PDA (mut)

# \- `merchant` - Merchant PDA

# \- `subscription\_config` - Config PDA

# \- `subscriber` - User wallet (signer)

# \- `subscriber\_token\_account` - User's token account (mut)

# \- `merchant\_token\_account` - Merchant's token account (mut)

# \- `token\_program` - SPL Token program

# 

# \*\*Effects:\*\*

# \- Extends expiry timestamp by interval

# \- Transfers payment from subscriber to merchant

# \- Updates total\_paid amount

# \- If expired, extends from current time; if active, extends from current expiry

# 

# \*\*Example:\*\*

# ```typescript

# await program.methods

# &nbsp; .renewSubscription()

# &nbsp; .accounts({

# &nbsp;   subscription: subscriptionPda,

# &nbsp;   merchant: merchantPda,

# &nbsp;   subscriptionConfig: configPda,

# &nbsp;   subscriber: userKeypair.publicKey,

# &nbsp;   subscriberTokenAccount: userTokenAccount,

# &nbsp;   merchantTokenAccount: merchantTokenAccount,

# &nbsp;   tokenProgram: TOKEN\_PROGRAM\_ID,

# &nbsp; })

# &nbsp; .signers(\[userKeypair])

# &nbsp; .rpc();

# ```

# 

# \### 4. cancel\_subscription

# 

# Cancel an active subscription.

# 

# \*\*Parameters:\*\* None

# 

# \*\*Accounts:\*\*

# \- `subscription` - Subscription PDA (mut)

# \- `subscriber` - User wallet (signer)

# 

# \*\*Effects:\*\*

# \- Sets `active` to false

# \- Does NOT refund payment

# 

# \*\*Example:\*\*

# ```typescript

# await program.methods

# &nbsp; .cancelSubscription()

# &nbsp; .accounts({

# &nbsp;   subscription: subscriptionPda,

# &nbsp;   subscriber: userKeypair.publicKey,

# &nbsp; })

# &nbsp; .signers(\[userKeypair])

# &nbsp; .rpc();

# ```

# 

# \### 5. check\_entitlement

# 

# Query if a subscription provides current entitlement.

# 

# \*\*Parameters:\*\* None

# 

# \*\*Accounts:\*\*

# \- `subscription` - Subscription PDA

# 

# \*\*Returns:\*\* Logs entitlement status

# \- `VALID` if active=true AND expiry > current\_time

# \- `INVALID` otherwise

# 

# \*\*Example:\*\*

# ```typescript

# await program.methods

# &nbsp; .checkEntitlement()

# &nbsp; .accounts({

# &nbsp;   subscription: subscriptionPda,

# &nbsp; })

# &nbsp; .rpc();

# 

# // Check logs for entitlement status

# ```

# 

# \### 6. update\_subscription\_config

# 

# Update subscription pricing or status (merchant only).

# 

# \*\*Parameters:\*\*

# \- `new\_price: Option<u64>` - New subscription price

# \- `new\_interval\_days: Option<u32>` - New interval duration

# \- `new\_active: Option<bool>` - Active/inactive status

# 

# \*\*Accounts:\*\*

# \- `subscription\_config` - Config PDA (mut)

# \- `merchant` - Merchant PDA

# \- `authority` - Merchant owner (signer)

# 

# \*\*Example:\*\*

# ```typescript

# await program.methods

# &nbsp; .updateSubscriptionConfig(

# &nbsp;   new BN(2\_000\_000),  // New price

# &nbsp;   null,               // Keep interval

# &nbsp;   null                // Keep active status

# &nbsp; )

# &nbsp; .accounts({

# &nbsp;   subscriptionConfig: configPda,

# &nbsp;   merchant: merchantPda,

# &nbsp;   authority: merchantKeypair.publicKey,

# &nbsp; })

# &nbsp; .signers(\[merchantKeypair])

# &nbsp; .rpc();

# ```

# 

# \## PDA Derivation

# 

# \### Merchant PDA

# ```typescript

# const \[merchantPda] = PublicKey.findProgramAddressSync(

# &nbsp; \[

# &nbsp;   Buffer.from("merchant"),

# &nbsp;   authority.toBuffer(),

# &nbsp;   tokenMint.toBuffer(),

# &nbsp; ],

# &nbsp; program.programId

# );

# ```

# 

# \### Subscription Config PDA

# ```typescript

# const \[configPda] = PublicKey.findProgramAddressSync(

# &nbsp; \[

# &nbsp;   Buffer.from("config"),

# &nbsp;   merchantPda.toBuffer(),

# &nbsp;   Buffer.from(productName),

# &nbsp; ],

# &nbsp; program.programId

# );

# ```

# 

# \### Subscription PDA

# ```typescript

# const \[subscriptionPda] = PublicKey.findProgramAddressSync(

# &nbsp; \[

# &nbsp;   Buffer.from("subscription"),

# &nbsp;   subscriber.toBuffer(),

# &nbsp;   configPda.toBuffer(),

# &nbsp; ],

# &nbsp; program.programId

# );

# ```

# 

# \## Error Codes

# 

# | Code | Message |

# |------|---------|

# | `InvalidPrice` | Price must be greater than 0 |

# | `InvalidInterval` | Interval must be greater than 0 days |

# | `ProductNameTooLong` | Product name max 50 characters |

# | `SubscriptionInactive` | Subscription is not active |

# | `SubscriptionAlreadyCanceled` | Subscription already canceled |

# | `Unauthorized` | Unauthorized access |

# | `MathOverflow` | Math overflow error |

# 

# \## Token Support

# 

# \### USDC Payments

# \- Use USDC mint address: `EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v`

# \- Price in base units: 1 USDC = 1,000,000 (6 decimals)

# 

# \### SOL Payments

# \- Use wrapped SOL mint or native SOL with token wrapper

# \- Price in lamports: 1 SOL = 1,000,000,000

# 

# \### Custom SPL Tokens

# \- Any SPL token mint can be used

# \- Ensure both merchant and subscriber have token accounts

# 

# \## Security Considerations

# 

# 1\. \*\*PDA Validation\*\* - All accounts use proper seed validation

# 2\. \*\*Owner Checks\*\* - Token account ownership verified on transfers

# 3\. \*\*Mint Validation\*\* - Token accounts must match merchant's configured mint

# 4\. \*\*Authorization\*\* - Only subscriber can cancel their subscription

# 5\. \*\*Merchant Control\*\* - Only merchant authority can update configs

# 6\. \*\*Math Safety\*\* - All arithmetic uses checked operations to prevent overflow

# 

# \## Building and Testing

# 

# \### Build

# ```bash

# anchor build

# ```

# 

# \### Test

# ```bash

# anchor test

# ```

# 

# \### Deploy

# ```bash

# anchor deploy

# ```

# 

# \## Usage Flow

# 

# 1\. \*\*Merchant Setup:\*\*

# &nbsp;  - Call `register\_merchant` with product details

# &nbsp;  - Create token account for receiving payments

# 

# 2\. \*\*User Subscription:\*\*

# &nbsp;  - User calls `create\_subscription`

# &nbsp;  - Payment transfers automatically

# &nbsp;  - Subscription becomes active

# 

# 3\. \*\*Access Control:\*\*

# &nbsp;  - Your app calls `check\_entitlement` before granting access

# &nbsp;  - Verify subscription is active and not expired

# 

# 4\. \*\*Renewal:\*\*

# &nbsp;  - User calls `renew\_subscription` when needed

# &nbsp;  - Extends expiry and processes payment

# 

# 5\. \*\*Cancellation:\*\*

# &nbsp;  - User calls `cancel\_subscription` to opt out

# &nbsp;  - No future charges, but no refund for current period

# 

# \## Example: Monthly USDC Newsletter

# 

# ```typescript

# // 1. Merchant registers newsletter subscription

# await program.methods.registerMerchant(

# &nbsp; new BN(5\_000\_000),  // 5 USDC/month

# &nbsp; 30,                 // Monthly

# &nbsp; "Premium Newsletter"

# ).rpc();

# 

# // 2. User subscribes

# await program.methods.createSubscription().rpc();

# 

# // 3. Check access before showing content

# const subscription = await program.account.subscription.fetch(subscriptionPda);

# const hasAccess = subscription.active \&\& 

# &nbsp;                 subscription.expiryTimestamp.toNumber() > Date.now() / 1000;

# 

# // 4. User renews after 30 days

# await program.methods.renewSubscription().rpc();

# ```

# 

# \## License

# 

# MIT



