// Session store implementations.
//
// Concrete session stores will live here, for example:
// - memory
// - database
// - redis
// - tower-sessions adapter
#[cfg(feature = "memory")]
pub mod memory;
#[cfg(feature = "memory")]
pub use crate::sessions::memory::MemorySessionStore;
