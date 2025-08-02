# YapHouse Anchor Program v1

> **A decentralized audio platform** smart contract module, powering user onboarding, room management, reward claims, and NFT badges.

---

## 🎯 Program Overview

The `yaphouse` Anchor program defines core on-chain logic for:

1. **User Initialization** (`initialize_user`) — onboard new users
2. **Subscription Purchase** (`subscription_purchase`) — mint Creator Subscription Pass NFT
3. **Room Lifecycle**

   * **Create Room** (`create_room`)
   * **Start Room** (`start_room`)
   * **Close Room** (`close_room`)
4. **Rewards & Badges**

   * **Claim Rewards** (`claim_reward`)
   * **Mint Badge NFT** (`mint_badge_nft`)

---

## 🏗️ Architectural Layout

```rust
use anchor_lang::prelude::*;
declare_id!("6K2rTd7Lmdhks4vDaSYDvbSG43HoVbtRX2tXj86rRG2r");
#[program]
pub mod yaphouse { /* ... */ }

// Account structs:
#[account] pub struct User { … }
#[account] pub struct YapRoom { … }
// Custom Data Types:
#[derive(AnchorSerialize, AnchorDeserialize)] pub struct SpaceAttendance { … }
```

All PDAs use **deterministic seeds** + bumps for address derivation and rent exemption.

---

## 🔨 Instructions & Accounts

| Instruction                                                                             | Accounts                                                                                                                              | Status        |
| --------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------- | ------------- |
| `initialize_user`                                                                       | - `payer: Signer`<br>- `UserCreated: PDA<User>`<br>- `system_program`                                                                 | ✅ Complete    |
| **Purpose:** Create a `User` account with profile data and initial stats.               |                                                                                                                                       |               |
| `subscription_purchase`                                                                 | - `creator: Signer`<br>- `CSP mint/account`: TBD<br>- `token_program`, `system_program`, `rent`                                       | ⚙️ Incomplete |
| **Purpose:** Mint a Creator Subscription Pass NFT.                                      |                                                                                                                                       |               |
| `create_room`                                                                           | - `payer: Signer`<br>- `yap_room: PDA<YapRoom>`<br>- `system_program`                                                                 | ✅ Complete    |
| **Purpose:** Initialize a new room with title, ID, start time, duration, and host info. |                                                                                                                                       |               |
| `start_room`                                                                            | - `yap_room: PDA<YapRoom>`<br>- `system_program`                                                                                      | ✅ Complete    |
| **Purpose:** Flip the room `active` flag to allow participation logging.                |                                                                                                                                       |               |
| `close_room`                                                                            | - TBD (should require `host` & `yap_room` PDAs)                                                                                       | ⚙️ Incomplete |
| **Purpose:** Mark room as ended; record closure timestamp.                              |                                                                                                                                       |               |
| `claim_reward`                                                                          | - `user: Signer`<br>- `user_profile: PDA<User>`<br>- `token_mint`<br>- `user_token_account`<br>- `token_program`<br>- `clock`         | ⚙️ Incomplete |
| **Purpose:** Distribute accrued YAP tokens to users for participation time.             |                                                                                                                                       |               |
| `mint_badge_nft`                                                                        | - `user: Signer`<br>- `badge_mint & account`: PDA/Mint<br>- `metadata_program`<br>- `token_program`<br>- `system_program`<br>- `rent` | ⚙️ Incomplete |
| **Purpose:** Mint achievement badges as NFTs (e.g., milestones).                        |                                                                                                                                       |               |

---

## 📐 Account Structures

### `User`

* **PDA Seed:** `[b"newuserinthetown", user_username]`
* **Fields:**

  * `user_pubkey: Pubkey`
  * `user_name: String` (max 32)
  * `user_username: String` (max 32)
  * `rooms_created: u64`
  * `rooms_attended: u64`
  * `spoken_time: u64`
  * `space_attended_and_time: Vec<SpaceAttendance>` (max 100)
  * `bump: u8`

### `YapRoom`

* **PDA Seed:** `[b"Room_Creation", room_id.to_le_bytes()]`
* **Fields:**

  * `host_pubkey: Pubkey`
  * `room_title: String` (max 32)
  * `room_id: u64`
  * `duration: u64`
  * `start_time: i64`
  * `active: bool`
  * `bump: u8`

### `SpaceAttendance` (Data Type)

* `room_id: u64`
* `time: i64`

---

## 🚧 Next Steps & To‐Do

1. **Complete `subscription_purchase`**

   * Define mint PDA, token account, and CPI to SPL Program.
2. **Implement `close_room`**

   * Add `host` signer guard, record closure timestamp.
3. **Build `claim_reward` logic**

   * Use SPL MintTo CPI with a reward vault PDA.
4. **Integrate `mint_badge_nft`**

   * Leverage Metaplex metadata CPI for NFT minting.
5. **Write Unit Tests**

   * For each instruction: success/failure cases, PDA derivations.
6. **Generate IDL & Client Snippets**

   * For Next.js + Anchor JS integration.

---

> *YapHouse v1 on-chain spec sets the foundation for a Social‑Fi ecosystem. This README captures the current state, guiding future implementations of reward and NFT mechanics!*
