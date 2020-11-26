pub mod fields;
pub mod access;
pub mod object;
pub mod owner;

pub type UserPrivateKey = [u8; 32];
pub type UserPublicKey = u32;
pub type RoomId = u64;
