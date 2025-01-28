use crate::core::skip_list::SkipList;
use libc::mlock;
use libc::munlock;
use memoffset::offset_of;

/// TODO: This needs to be a proceedural macro to avoid duplication
#[cfg(any(
    target_os = "ios",
    target_os = "macos",
    target_os = "android",
    target_os = "linux"
))]
pub fn blocks_ptr(blocks: &SkipList) -> *const u8 {
    let base_ptr = blocks as *const SkipList as *const u8;
    let offset = offset_of!(SkipList, blocks); // Offset of the `blocks` field
    unsafe { base_ptr.add(offset) }
}
#[cfg(any(
    target_os = "ios",
    target_os = "macos",
    target_os = "android",
    target_os = "linux"
))]
pub fn pin_memory(ptr: *const u8, size: usize) -> Result<(), String> {
    let result = unsafe { mlock(ptr as *const _, size) };
    if result == 0 {
        Ok(())
    } else {
        Err(format!(
            "Failed to pin memory: {}",
            std::io::Error::last_os_error()
        ))
    }
}

#[cfg(any(
    target_os = "ios",
    target_os = "macos",
    target_os = "android",
    target_os = "linux"
))]
pub fn unpin_memory(ptr: *const u8, size: usize) -> Result<(), String> {
    let result = unsafe { munlock(ptr as *const _, size) };
    if result == 0 {
        Ok(())
    } else {
        Err(format!(
            "Failed to unpin memory: {}",
            std::io::Error::last_os_error()
        ))
    }
}
