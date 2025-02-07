use libc::_SC_PAGESIZE;
use libc::{madvise, MADV_SEQUENTIAL};
use memmap2::MmapMut;
use std::fs::OpenOptions;
use std::path::Path;

use crate::core::skip_list::SkipList;
use libc::mlock;
use libc::munlock;
use libc::sysconf;
use memmap2::MmapOptions;
use memoffset::offset_of;
use std::io::Result;

/// TODO: This needs to be a proceedural macro to avoid duplication
#[cfg(any(target_os = "ios", target_os = "macos", target_os = "android", target_os = "linux"))]
pub fn blocks_ptr(blocks: &SkipList) -> *const u8 {
    let base_ptr = blocks as *const SkipList as *const u8;
    let offset = offset_of!(SkipList, blocks); // Offset of the `blocks` field
    unsafe { base_ptr.add(offset) }
}
#[cfg(any(target_os = "ios", target_os = "macos", target_os = "android", target_os = "linux"))]
pub fn pin_memory(ptr: *const u8, size: usize) -> std::result::Result<(), String> {
    let result = unsafe { mlock(ptr as *const _, size) };
    if result == 0 {
        return Ok(());
    } else {
        return Err(format!("Failed to pin memory: {}", std::io::Error::last_os_error()));
    }
}

#[cfg(any(target_os = "ios", target_os = "macos", target_os = "android", target_os = "linux"))]
pub fn unpin_memory(ptr: *const u8, size: usize) -> std::result::Result<(), String> {
    let result = unsafe { munlock(ptr as *const _, size) };
    if result == 0 {
        return Ok(());
    } else {
        return Err(std::io::Error::last_os_error().to_string());
    }
}

pub fn get_page_size() -> usize {
    return unsafe { sysconf(_SC_PAGESIZE) as usize };
}

///
/// Creates new segment file and memory maps it
///  Memory map a file with 16 KB pages
/// 16 KB pages are used to optimize for sequential access
pub fn mmap_opt(file_path: &str) -> Result<MmapMut> {
    let file = OpenOptions::new().read(true).write(true).create(true).open(Path::new(file_path))?;
    let page_size = get_page_size();
    let mut mmap = unsafe { MmapOptions::new().len(page_size).map_mut(&file)? };
    unsafe { madvise(mmap.as_mut_ptr() as *mut libc::c_void, page_size, MADV_SEQUENTIAL) };
    return Ok(mmap);
}

