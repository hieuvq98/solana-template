#[cfg(feature = "mainnet")]
pub const ROOT_KEYS: &[&str] = &[
  "5CLctYx1inGCDnQCVmicx9uHPaqwghEsSEeFYZnk2L6x",
];

#[cfg(all(not(feature = "mainnet"), not(feature = "devnet")))]
pub const ROOT_KEYS: &[&str] = &[
  "1RbBCDnDt7NkrjndCnwjMuJ9vJbx81pKT1ts1x8SeQq",
];

#[cfg(feature = "mainnet")]
pub const FEE_OWNER: &str = "FSskoaLzbUjbrvkB1DLg6fvayd2vYQYSvNWmZbTtH5Zy";

#[cfg(all(not(feature = "mainnet"), not(feature = "devnet")))]
pub const FEE_OWNER: &str = "1RbBCDnDt7NkrjndCnwjMuJ9vJbx81pKT1ts1x8SeQq";

pub const LAUNCHPAD_SEED_1: &[u8] = &[8, 201, 24, 140, 93, 100, 30, 148];
pub const LAUNCHPAD_PURCHASE_SEED_1: &[u8] = &[68, 70, 141, 93, 102, 104, 120, 59, 54];
pub const SIGNER_SEED_1: &[u8] = &[2, 151, 229, 53, 244, 77, 229, 7];
pub const USER_PROFILE_SEED_1: &[u8] = &[133, 177, 201, 78, 13, 152, 198, 180];
pub const WHITELIST_TOKEN_SEED_1: &[u8] = &[237, 187, 186, 94, 223, 196, 119, 229];
