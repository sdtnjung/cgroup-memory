#![doc = include_str!("../README.md")]

pub const MEMORY_STAT: &str = "/sys/fs/cgroup/memory.stat";
pub const MEMORY_MAX: &str = "/sys/fs/cgroup/memory.max";

use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    fs::{self, OpenOptions},
    io::Read,
    num::ParseIntError,
};

/// Represents the memory statistics from the [MEMORY_STAT] file.
///
/// The fields were taken from a file as seen within a `gcr.io/distroless/cc-debian12` Docker container image.
/// No other images/linux distros were tested yet.
///
/// To increase compatibility with other distros, all fields are optional.
#[derive(Debug, Default)]
pub struct MemoryStat {
    pub anon: Option<u64>,
    pub file: Option<u64>,
    pub kernel: Option<u64>,
    pub kernel_stack: Option<u64>,
    pub pagetables: Option<u64>,
    pub sec_pagetables: Option<u64>,
    pub percpu: Option<u64>,
    pub sock: Option<u64>,
    pub vmalloc: Option<u64>,
    pub shmem: Option<u64>,
    pub zswap: Option<u64>,
    pub zswapped: Option<u64>,
    pub file_mapped: Option<u64>,
    pub file_dirty: Option<u64>,
    pub file_writeback: Option<u64>,
    pub swapcached: Option<u64>,
    pub anon_thp: Option<u64>,
    pub file_thp: Option<u64>,
    pub shmem_thp: Option<u64>,
    pub inactive_anon: Option<u64>,
    pub active_anon: Option<u64>,
    pub inactive_file: Option<u64>,
    pub active_file: Option<u64>,
    pub unevictable: Option<u64>,
    pub slab_reclaimable: Option<u64>,
    pub slab_unreclaimable: Option<u64>,
    pub slab: Option<u64>,
    pub workingset_refault_anon: Option<u64>,
    pub workingset_refault_file: Option<u64>,
    pub workingset_activate_anon: Option<u64>,
    pub workingset_activate_file: Option<u64>,
    pub workingset_restore_anon: Option<u64>,
    pub workingset_restore_file: Option<u64>,
    pub workingset_nodereclaim: Option<u64>,
    pub pgscan: Option<u64>,
    pub pgsteal: Option<u64>,
    pub pgscan_kswapd: Option<u64>,
    pub pgscan_direct: Option<u64>,
    pub pgscan_khugepaged: Option<u64>,
    pub pgsteal_kswapd: Option<u64>,
    pub pgsteal_direct: Option<u64>,
    pub pgsteal_khugepaged: Option<u64>,
    pub pgfault: Option<u64>,
    pub pgmajfault: Option<u64>,
    pub pgrefill: Option<u64>,
    pub pgactivate: Option<u64>,
    pub pgdeactivate: Option<u64>,
    pub pglazyfree: Option<u64>,
    pub pglazyfreed: Option<u64>,
    pub zswpin: Option<u64>,
    pub zswpout: Option<u64>,
    pub thp_fault_alloc: Option<u64>,
    pub thp_collapse_alloc: Option<u64>,
}

#[derive(Debug)]
pub enum ReadParseError {
    Io(std::io::Error),
    Parse(ParseIntError),
    /// The memory file's values evaluted to a non-zero sum
    Zero,
}

impl From<ParseIntError> for ReadParseError {
    fn from(e: ParseIntError) -> Self {
        ReadParseError::Parse(e)
    }
}

impl From<std::io::Error> for ReadParseError {
    fn from(e: std::io::Error) -> Self {
        ReadParseError::Io(e)
    }
}

impl Error for ReadParseError {}

impl Display for ReadParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ReadParseError::Io(io_error) => write!(f, "{}", io_error),
            ReadParseError::Zero => write!(
                f,
                "The memory statistics could not be evaluted to a non-zero sum"
            ),
            ReadParseError::Parse(parse_error) => write!(f, "{}", parse_error),
        }
    }
}

/// Reads and parses the memory statistics file.
///
/// # Optional fields
///
/// * The memory statistics file [MEMORY_STAT] may not contain all fields. In this case, the field is assumed to be zero.
/// * In case the field's value cannot be parsed to u64, the field is assumed to be zero (unlikely scenario).
///
/// # Errors
/// Returns an error if the memory statistics file [MEMORY_STAT] could not be read or parsed.
pub fn memory_stat() -> Result<MemoryStat, ReadParseError> {
    let memory_stat_string = fs::read_to_string(MEMORY_STAT).map_err(ReadParseError::Io)?;

    println!("{}", memory_stat_string);

    let mut ms = MemoryStat::default();

    // Parse the memory statistics
    for line in memory_stat_string.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            let key = parts[0];

            match key {
                "anon" => ms.anon = parts[1].parse().ok(),
                "file" => ms.file = parts[1].parse().ok(),
                "kernel" => ms.kernel = parts[1].parse().ok(),
                "kernel_stack" => ms.kernel_stack = parts[1].parse().ok(),
                "pagetables" => ms.pagetables = parts[1].parse().ok(),
                "sec_pagetables" => ms.sec_pagetables = parts[1].parse().ok(),
                "percpu" => ms.percpu = parts[1].parse().ok(),
                "sock" => ms.sock = parts[1].parse().ok(),
                "vmalloc" => ms.vmalloc = parts[1].parse().ok(),
                "shmem" => ms.shmem = parts[1].parse().ok(),
                "zswap" => ms.zswap = parts[1].parse().ok(),
                "zswapped" => ms.zswapped = parts[1].parse().ok(),
                "file_mapped" => ms.file_mapped = parts[1].parse().ok(),
                "file_dirty" => ms.file_dirty = parts[1].parse().ok(),
                "file_writeback" => ms.file_writeback = parts[1].parse().ok(),
                "swapcached" => ms.swapcached = parts[1].parse().ok(),
                "anon_thp" => ms.anon_thp = parts[1].parse().ok(),
                "file_thp" => ms.file_thp = parts[1].parse().ok(),
                "shmem_thp" => ms.shmem_thp = parts[1].parse().ok(),
                "inactive_anon" => ms.inactive_anon = parts[1].parse().ok(),
                "active_anon" => ms.active_anon = parts[1].parse().ok(),
                "inactive_file" => ms.inactive_file = parts[1].parse().ok(),
                "active_file" => ms.active_file = parts[1].parse().ok(),
                "unevictable" => ms.unevictable = parts[1].parse().ok(),
                "slab_reclaimable" => ms.slab_reclaimable = parts[1].parse().ok(),
                "slab_unreclaimable" => ms.slab_unreclaimable = parts[1].parse().ok(),
                "slab" => ms.slab = parts[1].parse().ok(),
                "workingset_refault_anon" => ms.workingset_refault_anon = parts[1].parse().ok(),
                "workingset_refault_file" => ms.workingset_refault_file = parts[1].parse().ok(),
                "workingset_activate_anon" => ms.workingset_activate_anon = parts[1].parse().ok(),
                "workingset_activate_file" => ms.workingset_activate_file = parts[1].parse().ok(),
                "workingset_restore_anon" => ms.workingset_restore_anon = parts[1].parse().ok(),
                "workingset_restore_file" => ms.workingset_restore_file = parts[1].parse().ok(),
                "workingset_nodereclaim" => ms.workingset_nodereclaim = parts[1].parse().ok(),
                "pgscan" => ms.pgscan = parts[1].parse().ok(),
                "pgsteal" => ms.pgsteal = parts[1].parse().ok(),
                "pgscan_kswapd" => ms.pgscan_kswapd = parts[1].parse().ok(),
                "pgscan_direct" => ms.pgscan_direct = parts[1].parse().ok(),
                "pgscan_khugepaged" => ms.pgscan_khugepaged = parts[1].parse().ok(),
                "pgsteal_kswapd" => ms.pgsteal_kswapd = parts[1].parse().ok(),
                "pgsteal_direct" => ms.pgsteal_direct = parts[1].parse().ok(),
                "pgsteal_khugepaged" => ms.pgsteal_khugepaged = parts[1].parse().ok(),
                "pgfault" => ms.pgfault = parts[1].parse().ok(),
                "pgmajfault" => ms.pgmajfault = parts[1].parse().ok(),
                "pgrefill" => ms.pgrefill = parts[1].parse().ok(),
                "pgactivate" => ms.pgactivate = parts[1].parse().ok(),
                "pgdeactivate" => ms.pgdeactivate = parts[1].parse().ok(),
                "pglazyfree" => ms.pglazyfree = parts[1].parse().ok(),
                "pglazyfreed" => ms.pglazyfreed = parts[1].parse().ok(),
                "zswpin" => ms.zswpin = parts[1].parse().ok(),
                "zswpout" => ms.zswpout = parts[1].parse().ok(),
                "thp_fault_alloc" => ms.thp_fault_alloc = parts[1].parse().ok(),
                "thp_collapse_alloc" => ms.thp_collapse_alloc = parts[1].parse().ok(),
                _ => {}
            }
        }
    }
    Ok(ms)
}

/// Calculates the net used memory in bytes.
///
/// Formula: `anon + file + kernel + kernel_stack + pagetables + percpu + slab_unreclaimable - slab_reclaimable`
///
/// In case one of the fields is not present in the memory statistics file, the field is assumed to be zero.
///
/// # Errors
/// Returns an error if the memory statistics file [MEMORY_STAT] could not be read or the formula evaluted to a zero sum.
///
/// # Overflow
/// In theory the arithmetic operation may overflow:
/// * sum of occupied memory fields is greater than `u64::MAX`. Given the current state of technology, it is very unlikely that this will happen as `u64` represents ~ 18446744073 GB.
/// * `slab_reclaimable` is greater than the sum of all other fields that represent occupied memory.
pub fn memory_net_used_calc(ms: &MemoryStat) -> Result<u64, ReadParseError> {
    let total_net_used_memory = ms.anon.unwrap_or(0)
        + ms.file.unwrap_or(0)
        + ms.kernel.unwrap_or(0)
        + ms.kernel_stack.unwrap_or(0)
        + ms.pagetables.unwrap_or(0)
        + ms.percpu.unwrap_or(0)
        + ms.slab_unreclaimable.unwrap_or(0)
        - ms.slab_reclaimable.unwrap_or(0);

    if total_net_used_memory == 0 {
        return Err(ReadParseError::Zero);
    }

    Ok(total_net_used_memory)
}

/// Returns the net used memory in bytes.
///
/// * Reads and parses [MEMORY_STAT] via [memory_stat]
/// * Calculates the net used memory via [memory_net_used_calc]
pub fn memory_net_used() -> Result<u64, ReadParseError> {
    let ms = memory_stat()?;
    memory_net_used_calc(&ms)
}

/// Parses the max memory line.
///
/// # Returns
///
/// - `None` if `line` is "max".
/// - `Some(u64)` if the line could be parsed to `u64`.
///
/// # Errors
///
/// Returns an error if the line could not be parsed to `u64` and it is not "max".
pub fn memory_max_parse(line: &str) -> Result<Option<u64>, ParseIntError> {
    if line == "max" {
        return Ok(None);
    }

    line.trim().parse::<u64>().map(Some)
}

/// Parses the line to u64.
///
/// # Safety
///
/// This function is unsafe because it expects the input to be either "max" or a valid u64.
///
/// # Panics
///
/// This function panics if the input is not "max" or a valid u64.
fn memory_max_parse_unsafe(line: &str) -> Option<u64> {
    if line == "max" {
        return None;
    }

    unsafe { Some(line.trim().parse::<u64>().unwrap_unchecked()) }
}

/// Reads and parses the memory max file.
///
/// Data source: [MEMORY_MAX]
///
/// # Errors
/// Returns an error if the memory max file [MEMORY_MAX] could not be read or parsed.
///
/// # Example
/// ```rust
/// match memory_max() {
///     Ok(Some(v)) => println!("Max memory: {v}"),
///     Ok(None) => println!("No max memory constraint"),
///     Err(e) => println!("Failed to read and parse memory files: {e}"),
/// }
/// ``````
pub fn memory_max() -> Result<Option<u64>, ReadParseError> {
    let mut file = OpenOptions::new().read(true).open(MEMORY_MAX)?;
    let mut buffer = [0; 4096];

    let bytes_read = file.read(&mut buffer)?;
    let content = std::str::from_utf8(&buffer[..bytes_read]).map_err(|_e| ReadParseError::Zero)?;
    Ok(memory_max_parse(content)?)
}

/// Reads and parses the memory max file using unsafe code.
///
/// # Safety
/// This function is unsafe because it uses `std::str::from_utf8_unchecked`, which can lead to undefined behavior if the content is not valid ASCII. However the [Kernel doc](https://www.kernel.org/doc/Documentation/filesystems/sysfs.txt) states that attributes should be ASCII text files.
/// 
/// If the file content exceeds the buffer size (1024 bytes), only part of the file will be read, potentially causing incorrect parsing.
///
/// 
///
/// # Errors
/// Returns an error if the memory max file could not be read.
///
/// # Example
/// ```rust
/// match memory_max_unsafe() {
///     Ok(Some(v)) => println!("Max memory: {v}"),
///     Ok(None) => println!("No max memory constraint"),
///     Err(e) => println!("Failed to read and parse memory files: {e}"),
/// }
/// ```
pub fn memory_max_unsafe() -> Result<Option<u64>, ReadParseError> {
    let mut file = OpenOptions::new()
        .read(true)
        .open(MEMORY_MAX)
        .map_err(ReadParseError::Io)?;
    let mut buffer = [0; 1024];
    let bytes_read = file.read(&mut buffer).map_err(ReadParseError::Io)?;

    // SAFETY: Linux guarantees that all of *sysfs* is valid ASCII.
    let content = unsafe { std::str::from_utf8_unchecked(&buffer[..bytes_read]) };
    Ok(memory_max_parse_unsafe(content))
}

/// Returns the available memory in bytes.
///
/// Formula: [memory_max()] - [memory_net_used()]
///
/// # Errors
/// Returns an error if the memory max file [MEMORY_MAX] or the memory statistics file [MEMORY_STAT] could not be read or parsed.
///
/// # Panics
/// This function will panic if any arithmetic operation (subtraction) overflows.
///
/// # Example
/// ```rust
/// match memory_available() {
///     Ok(Some(available)) => println!("Available memory: {} bytes", available),
///     Ok(None) => println!("No memory limit set"),
///     Err(e) => println!("Failed to read memory information: {}", e),
/// }
/// ```
pub fn memory_available() -> Result<Option<u64>, ReadParseError> {
    let max = match memory_max()? {
        Some(value) => value,
        None => return Ok(None),
    };
    let net_used = memory_net_used()?;
    Ok(Some(max - net_used))
}
