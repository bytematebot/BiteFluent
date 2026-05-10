// Database/user persistence adapters.
//
// Concrete adapters will live here, for example:
// - memory
// - sqlx
// - surreal
// - redis-backed user/account adapter, if ever needed
#[cfg(feature = "memory")]
pub mod memory;
#[cfg(feature = "memory")]
pub use memory::MemoryAuthAdapter;
