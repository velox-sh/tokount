#![expect(unsafe_code)]
use std::fs::File;
use std::io::Read;
#[cfg(unix)]
use std::os::unix::fs::FileTypeExt;
use std::path::Path;

#[cfg(unix)]
use memmap2::Advice;
use memmap2::Mmap;
use memmap2::MmapOptions;

// NUL byte detection window
const BINARY_CHECK_LEN: usize = 8 * 1024;

// mmap for files larger than this
// buffered read below (mmap overhead dominates on small files)
const MMAP_THRESHOLD: u64 = 64 * 1024;

// skip files larger than this — no source file is this big, and pseudo-files
// like /proc/kcore report enormous sizes that would cause OOM when mmap'd
const MAX_FILE_SIZE: u64 = 512 * 1024 * 1024; // 512 MiB

#[cfg(unix)]
fn is_special_file_type(ft: std::fs::FileType) -> bool {
    ft.is_block_device() || ft.is_char_device() || ft.is_fifo() || ft.is_socket()
}

#[cfg(not(unix))]
fn is_special_file_type(_ft: std::fs::FileType) -> bool {
    false
}

/// Reusable file reader with mmap support for large files
pub struct FileReader {
    buf: Vec<u8>,
    mmap: Option<Mmap>,
}

impl Default for FileReader {
    fn default() -> Self {
        Self::new()
    }
}

impl FileReader {
    fn read_file(&mut self, file: File, size: u64) -> Option<&[u8]> {
        if size > MAX_FILE_SIZE {
            return None;
        }

        if size >= MMAP_THRESHOLD {
            // we hold no other reference to this file's pages
            // the mmap is stored in self.mmap and dropped before the next read() call
            // len() skips the internal fstat64 that Mmap::map() would call
            // since we already have the size from metadata() above
            let map = unsafe { MmapOptions::new().len(size as usize).map(&file) }.ok()?;

            let check_len = map.len().min(BINARY_CHECK_LEN);
            if memchr::memchr(0, &map[..check_len]).is_some() {
                return None;
            }

            // scanning is sequential; hint the kernel to prefetch ahead
            #[cfg(unix)]
            map.advise(Advice::Sequential).ok();

            self.mmap = Some(map);
            self.buf.clear();
            return self.mmap.as_deref();
        }

        self.mmap = None;
        self.buf.clear();
        self.buf.reserve(size as usize + 1);

        let mut file = file;
        file.read_to_end(&mut self.buf).ok()?;
        if self.buf.is_empty() {
            return None;
        }

        let check_len = self.buf.len().min(BINARY_CHECK_LEN);
        if memchr::memchr(0, &self.buf[..check_len]).is_some() {
            return None;
        }

        Some(&self.buf)
    }

    /// Create a new file reader
    pub fn new() -> Self {
        Self {
            buf: Vec::with_capacity(64 * 1024),
            mmap: None,
        }
    }

    /// Read from an already-open file; skips the `open` + `stat` syscalls
    pub fn read_open(&mut self, file: File, size: u64) -> Option<&[u8]> {
        self.read_file(file, size)
    }

    /// Read a file into a reusable buffer; returns `None` for binary files, empty files, or errors
    pub fn read(&mut self, path: &Path) -> Option<&[u8]> {
        let file = File::open(path).ok()?;
        let metadata = file.metadata().ok()?;

        if !metadata.file_type().is_file() || is_special_file_type(metadata.file_type()) {
            self.mmap = None;
            return None;
        }

        let size = metadata.len();
        if size == 0 {
            self.mmap = None;
            return None;
        }

        self.read_file(file, size)
    }
}
