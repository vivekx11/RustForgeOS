//! Capability-based permission system

use bitflags::bitflags;

bitflags! {
    pub struct Capability: u32 {
        const READ = 1 << 0;
        const WRITE = 1 << 1;
        const EXECUTE = 1 << 2;
        const NETWORK = 1 << 3;
        const FILESYSTEM = 1 << 4;
    }
}
