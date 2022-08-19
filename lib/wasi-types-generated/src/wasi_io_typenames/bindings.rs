#[allow(clippy::all)]
pub mod wasi_io_typenames {
  #[allow(unused_imports)]
  use wit_bindgen_wasmer::{anyhow, wasmer};
  /// Timestamp in nanoseconds.
  pub type Timestamp = u64;
  /// Identifiers for clocks.
  #[repr(u8)]
  #[derive(Clone, Copy, PartialEq, Eq)]
  pub enum Clockid {
    /// The clock measuring real time. Time value zero corresponds with
    /// 1970-01-01T00:00:00Z.
    Realtime,
    /// The store-wide monotonic clock, which is defined as a clock measuring
    /// real time, whose value cannot be adjusted and which cannot have negative
    /// clock jumps. The epoch of this clock is undefined. The absolute time
    /// value of this clock therefore has no meaning.
    Monotonic,
  }
  impl core::fmt::Debug for Clockid {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
      match self {
        Clockid::Realtime => {
          f.debug_tuple("Clockid::Realtime").finish()
        }
        Clockid::Monotonic => {
          f.debug_tuple("Clockid::Monotonic").finish()
        }
      }
    }
  }
  /// Error codes returned by functions.
  /// Not all of these error codes are returned by the functions provided by this
  /// API; some are used in higher-level library layers, and others are provided
  /// merely for alignment with POSIX.
  #[repr(u8)]
  #[derive(Clone, Copy, PartialEq, Eq)]
  pub enum Errno {
    /// No error occurred. System call completed successfully.
    Success,
    /// Argument list too long.
    Toobig,
    /// Permission denied.
    Access,
    /// Address in use.
    Addrinuse,
    /// Address not available.
    Addrnotavail,
    /// Address family not supported.
    Afnosupport,
    /// Resource unavailable, or operation would block.
    Again,
    /// Connection already in progress.
    Already,
    /// Bad file descriptor.
    Badf,
    /// Bad message.
    Badmsg,
    /// Device or resource busy.
    Busy,
    /// Operation canceled.
    Canceled,
    /// No child processes.
    Child,
    /// Connection aborted.
    Connaborted,
    /// Connection refused.
    Connrefused,
    /// Connection reset.
    Connreset,
    /// Resource deadlock would occur.
    Deadlk,
    /// Destination address required.
    Destaddrreq,
    /// Mathematics argument out of domain of function.
    Dom,
    /// Reserved.
    Dquot,
    /// File exists.
    Exist,
    /// Bad address.
    Fault,
    /// File too large.
    Fbig,
    /// Host is unreachable.
    Hostunreach,
    /// Identifier removed.
    Idrm,
    /// Illegal byte sequence.
    Ilseq,
    /// Operation in progress.
    Inprogress,
    /// Interrupted function.
    Intr,
    /// Invalid argument.
    Inval,
    /// I/O error.
    Io,
    /// Socket is connected.
    Isconn,
    /// Is a directory.
    Isdir,
    /// Too many levels of symbolic links.
    Loop,
    /// File descriptor value too large.
    Mfile,
    /// Too many links.
    Mlink,
    /// Message too large.
    Msgsize,
    /// Reserved.
    Multihop,
    /// Filename too long.
    Nametoolong,
    /// Network is down.
    Netdown,
    /// Connection aborted by network.
    Netreset,
    /// Network unreachable.
    Netunreach,
    /// Too many files open in system.
    Nfile,
    /// No buffer space available.
    Nobufs,
    /// No such device.
    Nodev,
    /// No such file or directory.
    Noent,
    /// Executable file format error.
    Noexec,
    /// No locks available.
    Nolck,
    /// Reserved.
    Nolink,
    /// Not enough space.
    Nomem,
    /// No message of the desired type.
    Nomsg,
    /// Protocol not available.
    Noprotoopt,
    /// No space left on device.
    Nospc,
    /// Function not supported.
    Nosys,
    /// The socket is not connected.
    Notconn,
    /// Not a directory or a symbolic link to a directory.
    Notdir,
    /// Directory not empty.
    Notempty,
    /// State not recoverable.
    Notrecoverable,
    /// Not a socket.
    Notsock,
    /// Not supported, or operation not supported on socket.
    Notsup,
    /// Inappropriate I/O control operation.
    Notty,
    /// No such device or address.
    Nxio,
    /// Value too large to be stored in data type.
    Overflow,
    /// Previous owner died.
    Ownerdead,
    /// Operation not permitted.
    Perm,
    /// Broken pipe.
    Pipe,
    /// Protocol error.
    Proto,
    /// Protocol not supported.
    Protonosupport,
    /// Protocol wrong type for socket.
    Prototype,
    /// Result too large.
    Range,
    /// Read-only file system.
    Rofs,
    /// Invalid seek.
    Spipe,
    /// No such process.
    Srch,
    /// Reserved.
    Stale,
    /// Connection timed out.
    Timedout,
    /// Text file busy.
    Txtbsy,
    /// Cross-device link.
    Xdev,
    /// Extension: Capabilities insufficient.
    Notcapable,
  }
  impl Errno{
    pub fn name(&self) -> &'static str {
      match self {
        Errno::Success => "success",
        Errno::Toobig => "toobig",
        Errno::Access => "access",
        Errno::Addrinuse => "addrinuse",
        Errno::Addrnotavail => "addrnotavail",
        Errno::Afnosupport => "afnosupport",
        Errno::Again => "again",
        Errno::Already => "already",
        Errno::Badf => "badf",
        Errno::Badmsg => "badmsg",
        Errno::Busy => "busy",
        Errno::Canceled => "canceled",
        Errno::Child => "child",
        Errno::Connaborted => "connaborted",
        Errno::Connrefused => "connrefused",
        Errno::Connreset => "connreset",
        Errno::Deadlk => "deadlk",
        Errno::Destaddrreq => "destaddrreq",
        Errno::Dom => "dom",
        Errno::Dquot => "dquot",
        Errno::Exist => "exist",
        Errno::Fault => "fault",
        Errno::Fbig => "fbig",
        Errno::Hostunreach => "hostunreach",
        Errno::Idrm => "idrm",
        Errno::Ilseq => "ilseq",
        Errno::Inprogress => "inprogress",
        Errno::Intr => "intr",
        Errno::Inval => "inval",
        Errno::Io => "io",
        Errno::Isconn => "isconn",
        Errno::Isdir => "isdir",
        Errno::Loop => "loop",
        Errno::Mfile => "mfile",
        Errno::Mlink => "mlink",
        Errno::Msgsize => "msgsize",
        Errno::Multihop => "multihop",
        Errno::Nametoolong => "nametoolong",
        Errno::Netdown => "netdown",
        Errno::Netreset => "netreset",
        Errno::Netunreach => "netunreach",
        Errno::Nfile => "nfile",
        Errno::Nobufs => "nobufs",
        Errno::Nodev => "nodev",
        Errno::Noent => "noent",
        Errno::Noexec => "noexec",
        Errno::Nolck => "nolck",
        Errno::Nolink => "nolink",
        Errno::Nomem => "nomem",
        Errno::Nomsg => "nomsg",
        Errno::Noprotoopt => "noprotoopt",
        Errno::Nospc => "nospc",
        Errno::Nosys => "nosys",
        Errno::Notconn => "notconn",
        Errno::Notdir => "notdir",
        Errno::Notempty => "notempty",
        Errno::Notrecoverable => "notrecoverable",
        Errno::Notsock => "notsock",
        Errno::Notsup => "notsup",
        Errno::Notty => "notty",
        Errno::Nxio => "nxio",
        Errno::Overflow => "overflow",
        Errno::Ownerdead => "ownerdead",
        Errno::Perm => "perm",
        Errno::Pipe => "pipe",
        Errno::Proto => "proto",
        Errno::Protonosupport => "protonosupport",
        Errno::Prototype => "prototype",
        Errno::Range => "range",
        Errno::Rofs => "rofs",
        Errno::Spipe => "spipe",
        Errno::Srch => "srch",
        Errno::Stale => "stale",
        Errno::Timedout => "timedout",
        Errno::Txtbsy => "txtbsy",
        Errno::Xdev => "xdev",
        Errno::Notcapable => "notcapable",
      }
    }
    pub fn message(&self) -> &'static str {
      match self {
        Errno::Success => "No error occurred. System call completed successfully.",
        Errno::Toobig => "Argument list too long.",
        Errno::Access => "Permission denied.",
        Errno::Addrinuse => "Address in use.",
        Errno::Addrnotavail => "Address not available.",
        Errno::Afnosupport => "Address family not supported.",
        Errno::Again => "Resource unavailable, or operation would block.",
        Errno::Already => "Connection already in progress.",
        Errno::Badf => "Bad file descriptor.",
        Errno::Badmsg => "Bad message.",
        Errno::Busy => "Device or resource busy.",
        Errno::Canceled => "Operation canceled.",
        Errno::Child => "No child processes.",
        Errno::Connaborted => "Connection aborted.",
        Errno::Connrefused => "Connection refused.",
        Errno::Connreset => "Connection reset.",
        Errno::Deadlk => "Resource deadlock would occur.",
        Errno::Destaddrreq => "Destination address required.",
        Errno::Dom => "Mathematics argument out of domain of function.",
        Errno::Dquot => "Reserved.",
        Errno::Exist => "File exists.",
        Errno::Fault => "Bad address.",
        Errno::Fbig => "File too large.",
        Errno::Hostunreach => "Host is unreachable.",
        Errno::Idrm => "Identifier removed.",
        Errno::Ilseq => "Illegal byte sequence.",
        Errno::Inprogress => "Operation in progress.",
        Errno::Intr => "Interrupted function.",
        Errno::Inval => "Invalid argument.",
        Errno::Io => "I/O error.",
        Errno::Isconn => "Socket is connected.",
        Errno::Isdir => "Is a directory.",
        Errno::Loop => "Too many levels of symbolic links.",
        Errno::Mfile => "File descriptor value too large.",
        Errno::Mlink => "Too many links.",
        Errno::Msgsize => "Message too large.",
        Errno::Multihop => "Reserved.",
        Errno::Nametoolong => "Filename too long.",
        Errno::Netdown => "Network is down.",
        Errno::Netreset => "Connection aborted by network.",
        Errno::Netunreach => "Network unreachable.",
        Errno::Nfile => "Too many files open in system.",
        Errno::Nobufs => "No buffer space available.",
        Errno::Nodev => "No such device.",
        Errno::Noent => "No such file or directory.",
        Errno::Noexec => "Executable file format error.",
        Errno::Nolck => "No locks available.",
        Errno::Nolink => "Reserved.",
        Errno::Nomem => "Not enough space.",
        Errno::Nomsg => "No message of the desired type.",
        Errno::Noprotoopt => "Protocol not available.",
        Errno::Nospc => "No space left on device.",
        Errno::Nosys => "Function not supported.",
        Errno::Notconn => "The socket is not connected.",
        Errno::Notdir => "Not a directory or a symbolic link to a directory.",
        Errno::Notempty => "Directory not empty.",
        Errno::Notrecoverable => "State not recoverable.",
        Errno::Notsock => "Not a socket.",
        Errno::Notsup => "Not supported, or operation not supported on socket.",
        Errno::Notty => "Inappropriate I/O control operation.",
        Errno::Nxio => "No such device or address.",
        Errno::Overflow => "Value too large to be stored in data type.",
        Errno::Ownerdead => "Previous owner died.",
        Errno::Perm => "Operation not permitted.",
        Errno::Pipe => "Broken pipe.",
        Errno::Proto => "Protocol error.",
        Errno::Protonosupport => "Protocol not supported.",
        Errno::Prototype => "Protocol wrong type for socket.",
        Errno::Range => "Result too large.",
        Errno::Rofs => "Read-only file system.",
        Errno::Spipe => "Invalid seek.",
        Errno::Srch => "No such process.",
        Errno::Stale => "Reserved.",
        Errno::Timedout => "Connection timed out.",
        Errno::Txtbsy => "Text file busy.",
        Errno::Xdev => "Cross-device link.",
        Errno::Notcapable => "Extension: Capabilities insufficient.",
      }
    }
  }
  impl core::fmt::Debug for Errno{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
      f.debug_struct("Errno")
      .field("code", &(*self as i32))
      .field("name", &self.name())
      .field("message", &self.message())
      .finish()
    }
  }
  impl core::fmt::Display for Errno{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
      write!(f, "{} (error {})", self.name(), *self as i32)}
  }
  
  impl std::error::Error for Errno{}
  wit_bindgen_wasmer::bitflags::bitflags! {
    /// File descriptor rights, determining which actions may be performed.
    pub struct Rights: u32 {/// The right to invoke `fd_datasync`.
      /// 
      /// If `path_open` is set, includes the right to invoke
      /// `path_open` with `fdflags::dsync`.
      const FD_DATASYNC = 1 << 0;
      /// The right to invoke `fd_read` and `sock_recv`.
      /// 
      /// If `rights::fd_seek` is set, includes the right to invoke `fd_pread`.
      const FD_READ = 1 << 1;
      /// The right to invoke `fd_seek`. This flag implies `rights::fd_tell`.
      const FD_SEEK = 1 << 2;
      /// The right to invoke `fd_fdstat_set_flags`.
      const FD_FDSTAT_SET_FLAGS = 1 << 3;
      /// The right to invoke `fd_sync`.
      /// 
      /// If `path_open` is set, includes the right to invoke
      /// `path_open` with `fdflags::rsync` and `fdflags::dsync`.
      const FD_SYNC = 1 << 4;
      /// The right to invoke `fd_seek` in such a way that the file offset
      /// remains unaltered (i.e., `whence::cur` with offset zero), or to
      /// invoke `fd_tell`.
      const FD_TELL = 1 << 5;
      /// The right to invoke `fd_write` and `sock_send`.
      /// If `rights::fd_seek` is set, includes the right to invoke `fd_pwrite`.
      const FD_WRITE = 1 << 6;
      /// The right to invoke `fd_advise`.
      const FD_ADVISE = 1 << 7;
      /// The right to invoke `fd_allocate`.
      const FD_ALLOCATE = 1 << 8;
      /// The right to invoke `path_create_directory`.
      const PATH_CREATE_DIRECTORY = 1 << 9;
      /// If `path_open` is set, the right to invoke `path_open` with `oflags::create`.
      const PATH_CREATE_FILE = 1 << 10;
      /// The right to invoke `path_link` with the file descriptor as the
      /// source directory.
      const PATH_LINK_SOURCE = 1 << 11;
      /// The right to invoke `path_link` with the file descriptor as the
      /// target directory.
      const PATH_LINK_TARGET = 1 << 12;
      /// The right to invoke `path_open`.
      const PATH_OPEN = 1 << 13;
      /// The right to invoke `fd_readdir`.
      const FD_READDIR = 1 << 14;
      /// The right to invoke `path_readlink`.
      const PATH_READLINK = 1 << 15;
      /// The right to invoke `path_rename` with the file descriptor as the source directory.
      const PATH_RENAME_SOURCE = 1 << 16;
      /// The right to invoke `path_rename` with the file descriptor as the target directory.
      const PATH_RENAME_TARGET = 1 << 17;
      /// The right to invoke `path_filestat_get`.
      const PATH_FILESTAT_GET = 1 << 18;
      /// The right to change a file's size.
      /// If `path_open` is set, includes the right to invoke `path_open` with `oflags::trunc`.
      /// Note: there is no function named `path_filestat_set_size`. This follows POSIX design,
      /// which only has `ftruncate` and does not provide `ftruncateat`.
      /// While such function would be desirable from the API design perspective, there are virtually
      /// no use cases for it since no code written for POSIX systems would use it.
      /// Moreover, implementing it would require multiple syscalls, leading to inferior performance.
      const PATH_FILESTAT_SET_SIZE = 1 << 19;
      /// The right to invoke `path_filestat_set_times`.
      const PATH_FILESTAT_SET_TIMES = 1 << 20;
      /// The right to invoke `path_permissions_set`.
      const PATH_PERMISSIONS_SET = 1 << 21;
      /// The right to invoke `fd_filestat_get`.
      const FD_FILESTAT_GET = 1 << 22;
      /// The right to invoke `fd_filestat_set_size`.
      const FD_FILESTAT_SET_SIZE = 1 << 23;
      /// The right to invoke `fd_filestat_set_times`.
      const FD_FILESTAT_SET_TIMES = 1 << 24;
      /// The right to invoke `fd_permissions_set`.
      const FD_PERMISSIONS_SET = 1 << 25;
      /// The right to invoke `path_symlink`.
      const PATH_SYMLINK = 1 << 26;
      /// The right to invoke `path_remove_directory`.
      const PATH_REMOVE_DIRECTORY = 1 << 27;
      /// The right to invoke `path_unlink_file`.
      const PATH_UNLINK_FILE = 1 << 28;
      /// If `rights::fd_read` is set, includes the right to invoke `poll_oneoff` to subscribe to `eventtype::fd_read`.
      /// If `rights::fd_write` is set, includes the right to invoke `poll_oneoff` to subscribe to `eventtype::fd_write`.
      const POLL_FD_READWRITE = 1 << 29;
      /// The right to invoke `sock_shutdown`.
      const SOCK_SHUTDOWN = 1 << 30;
    }
  }
  
  impl core::fmt::Display for Rights{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
      f.write_str("Rights(")?;
      core::fmt::Debug::fmt(self, f)?;
      f.write_str(" (0x")?;
      core::fmt::LowerHex::fmt(&self.bits, f)?;
      f.write_str("))")?;
      Ok(())}
  }
  
  /// A file descriptor handle.
  pub type Fd = u32;
  /// The type of a file descriptor or file.
  #[repr(u8)]
  #[derive(Clone, Copy, PartialEq, Eq)]
  pub enum Filetype {
    /// The type of the file descriptor or file is unknown or is different from any of the other types specified.
    Unknown,
    /// The file descriptor or file refers to a block device inode.
    BlockDevice,
    /// The file descriptor or file refers to a character device inode.
    CharacterDevice,
    /// The file descriptor or file refers to a directory inode.
    Directory,
    /// The file descriptor or file refers to a regular file inode.
    RegularFile,
    /// The file descriptor or file refers to a datagram socket.
    SocketDgram,
    /// The file descriptor or file refers to a byte-stream socket.
    SocketStream,
    /// The file refers to a symbolic link inode.
    SymbolicLink,
    /// The file descriptor or file refers to a FIFO.
    Fifo,
  }
  impl core::fmt::Debug for Filetype {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
      match self {
        Filetype::Unknown => {
          f.debug_tuple("Filetype::Unknown").finish()
        }
        Filetype::BlockDevice => {
          f.debug_tuple("Filetype::BlockDevice").finish()
        }
        Filetype::CharacterDevice => {
          f.debug_tuple("Filetype::CharacterDevice").finish()
        }
        Filetype::Directory => {
          f.debug_tuple("Filetype::Directory").finish()
        }
        Filetype::RegularFile => {
          f.debug_tuple("Filetype::RegularFile").finish()
        }
        Filetype::SocketDgram => {
          f.debug_tuple("Filetype::SocketDgram").finish()
        }
        Filetype::SocketStream => {
          f.debug_tuple("Filetype::SocketStream").finish()
        }
        Filetype::SymbolicLink => {
          f.debug_tuple("Filetype::SymbolicLink").finish()
        }
        Filetype::Fifo => {
          f.debug_tuple("Filetype::Fifo").finish()
        }
      }
    }
  }
  /// File or memory access pattern advisory information.
  #[repr(u8)]
  #[derive(Clone, Copy, PartialEq, Eq)]
  pub enum Advice {
    /// The application has no advice to give on its behavior with respect to the specified data.
    Normal,
    /// The application expects to access the specified data sequentially from lower offsets to higher offsets.
    Sequential,
    /// The application expects to access the specified data in a random order.
    Random,
    /// The application expects to access the specified data in the near future.
    Willneed,
    /// The application expects that it will not access the specified data in the near future.
    Dontneed,
    /// The application expects to access the specified data once and then not reuse it thereafter.
    Noreuse,
  }
  impl core::fmt::Debug for Advice {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
      match self {
        Advice::Normal => {
          f.debug_tuple("Advice::Normal").finish()
        }
        Advice::Sequential => {
          f.debug_tuple("Advice::Sequential").finish()
        }
        Advice::Random => {
          f.debug_tuple("Advice::Random").finish()
        }
        Advice::Willneed => {
          f.debug_tuple("Advice::Willneed").finish()
        }
        Advice::Dontneed => {
          f.debug_tuple("Advice::Dontneed").finish()
        }
        Advice::Noreuse => {
          f.debug_tuple("Advice::Noreuse").finish()
        }
      }
    }
  }
  wit_bindgen_wasmer::bitflags::bitflags! {
    /// File descriptor flags.
    pub struct Fdflags: u8 {/// Append mode: Data written to the file is always appended to the file's end.
      const APPEND = 1 << 0;
      /// Write according to synchronized I/O data integrity completion. Only the data stored in the file is synchronized.
      const DSYNC = 1 << 1;
      /// Non-blocking mode.
      const NONBLOCK = 1 << 2;
      /// Synchronized read I/O operations.
      const RSYNC = 1 << 3;
      /// Write according to synchronized I/O file integrity completion. In
      /// addition to synchronizing the data stored in the file, the implementation
      /// may also synchronously update the file's metadata.
      const SYNC = 1 << 4;
    }
  }
  
  impl core::fmt::Display for Fdflags{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
      f.write_str("Fdflags(")?;
      core::fmt::Debug::fmt(self, f)?;
      f.write_str(" (0x")?;
      core::fmt::LowerHex::fmt(&self.bits, f)?;
      f.write_str("))")?;
      Ok(())}
  }
  
  /// File descriptor attributes.
  #[repr(C)]
  #[derive(Copy, Clone)]
  pub struct Fdstat {
    /// File type.
    pub fs_filetype: Filetype,
    /// File descriptor flags.
    pub fs_flags: Fdflags,
    /// Rights that apply to this file descriptor.
    pub fs_rights_base: Rights,
    /// Maximum set of rights that may be installed on new file descriptors that
    /// are created through this file descriptor, e.g., through `path_open`.
    pub fs_rights_inheriting: Rights,
  }
  impl core::fmt::Debug for Fdstat {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
      f.debug_struct("Fdstat").field("fs-filetype", &self.fs_filetype).field("fs-flags", &self.fs_flags).field("fs-rights-base", &self.fs_rights_base).field("fs-rights-inheriting", &self.fs_rights_inheriting).finish()}
  }
  /// Type of a subscription to an event or its occurrence.
  #[repr(u8)]
  #[derive(Clone, Copy, PartialEq, Eq)]
  pub enum Eventtype {
    /// The time value of clock `subscription_clock::id` has
    /// reached timestamp `subscription_clock::timeout`.
    Clock,
    /// File descriptor `subscription_fd_readwrite::fd` has data
    /// available for reading. This event always triggers for regular files.
    FdRead,
    /// File descriptor `subscription_fd_readwrite::fd` has capacity
    /// available for writing. This event always triggers for regular files.
    FdWrite,
  }
  impl core::fmt::Debug for Eventtype {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
      match self {
        Eventtype::Clock => {
          f.debug_tuple("Eventtype::Clock").finish()
        }
        Eventtype::FdRead => {
          f.debug_tuple("Eventtype::FdRead").finish()
        }
        Eventtype::FdWrite => {
          f.debug_tuple("Eventtype::FdWrite").finish()
        }
      }
    }
  }
  wit_bindgen_wasmer::bitflags::bitflags! {
    /// Flags determining how to interpret the timestamp provided in
    /// `subscription_clock::timeout`.
    pub struct Subclockflags: u8 {/// If set, treat the timestamp provided in
      /// `subscription_clock::timeout` as an absolute timestamp of clock
      /// `subscription_clock::id`. If clear, treat the timestamp
      /// provided in `subscription_clock::timeout` relative to the
      /// current time value of clock `subscription_clock::id`.
      const SUBSCRIPTION_CLOCK_ABSTIME = 1 << 0;
    }
  }
  
  impl core::fmt::Display for Subclockflags{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
      f.write_str("Subclockflags(")?;
      core::fmt::Debug::fmt(self, f)?;
      f.write_str(" (0x")?;
      core::fmt::LowerHex::fmt(&self.bits, f)?;
      f.write_str("))")?;
      Ok(())}
  }
  
  /// The contents of a `subscription` when type is `eventtype::clock`.
  #[repr(C)]
  #[derive(Copy, Clone)]
  pub struct SubscriptionClock {
    /// The clock against which to compare the timestamp.
    pub id: Clockid,
    /// The absolute or relative timestamp.
    pub timeout: Timestamp,
    /// The amount of time that the implementation may wait additionally
    /// to coalesce with other events.
    pub precision: Timestamp,
    /// Flags specifying whether the timeout is absolute or relative
    pub flags: Subclockflags,
  }
  impl core::fmt::Debug for SubscriptionClock {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
      f.debug_struct("SubscriptionClock").field("id", &self.id).field("timeout", &self.timeout).field("precision", &self.precision).field("flags", &self.flags).finish()}
  }
  /// Identifiers for preopened capabilities.
  #[repr(u8)]
  #[derive(Clone, Copy, PartialEq, Eq)]
  pub enum Preopentype {
    /// A pre-opened directory.
    Dir,
  }
  impl core::fmt::Debug for Preopentype {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
      match self {
        Preopentype::Dir => {
          f.debug_tuple("Preopentype::Dir").finish()
        }
      }
    }
  }
  
  /// Auxiliary data associated with the wasm exports.
  #[derive(Default)]
  pub struct WasiIoTypenamesData {
  }
  
  pub struct WasiIoTypenames {
    #[allow(dead_code)]
    env: wasmer::FunctionEnv<WasiIoTypenamesData>,
    func_fd_dummy_func: wasmer::TypedFunction<i32, ()>,
    func_fdstat_dummy_func: wasmer::TypedFunction<(i32,i32,i32,i32,), ()>,
    func_subscription_clock_dummy_func: wasmer::TypedFunction<(i32,i64,i64,i32,), ()>,
  }
  impl WasiIoTypenames {
    #[allow(unused_variables)]
    
    /// Adds any intrinsics, if necessary for this exported wasm
    /// functionality to the `ImportObject` provided.
    ///
    /// This function returns the `WasiIoTypenamesData` which needs to be
    /// passed through to `WasiIoTypenames::new`.
    fn add_to_imports(
    mut store: impl wasmer::AsStoreMut,
    imports: &mut wasmer::Imports,
    ) -> wasmer::FunctionEnv<WasiIoTypenamesData> {
      let env = wasmer::FunctionEnv::new(&mut store, WasiIoTypenamesData::default());
      env
    }
    
    /// Instantiates the provided `module` using the specified
    /// parameters, wrapping up the result in a structure that
    /// translates between wasm and the host.
    ///
    /// The `imports` provided will have intrinsics added to it
    /// automatically, so it's not necessary to call
    /// `add_to_imports` beforehand. This function will
    /// instantiate the `module` otherwise using `imports`, and
    /// both an instance of this structure and the underlying
    /// `wasmer::Instance` will be returned.
    pub fn instantiate(
    mut store: impl wasmer::AsStoreMut,
    module: &wasmer::Module,
    imports: &mut wasmer::Imports,
    ) -> anyhow::Result<(Self, wasmer::Instance)> {
      let env = Self::add_to_imports(&mut store, imports);
      let instance = wasmer::Instance::new(
      &mut store, module, &*imports)?;
      
      Ok((Self::new(store, &instance, env)?, instance))
    }
    
    /// Low-level creation wrapper for wrapping up the exports
    /// of the `instance` provided in this structure of wasm
    /// exports.
    ///
    /// This function will extract exports from the `instance`
    /// and wrap them all up in the returned structure which can
    /// be used to interact with the wasm module.
    pub fn new(
    store: impl wasmer::AsStoreMut,
    _instance: &wasmer::Instance,
    env: wasmer::FunctionEnv<WasiIoTypenamesData>,
    ) -> Result<Self, wasmer::ExportError> {
      let func_fd_dummy_func= _instance.exports.get_typed_function(&store, "fd-dummy-func")?;
      let func_fdstat_dummy_func= _instance.exports.get_typed_function(&store, "fdstat-dummy-func")?;
      let func_subscription_clock_dummy_func= _instance.exports.get_typed_function(&store, "subscription-clock-dummy-func")?;
      Ok(WasiIoTypenames{
        func_fd_dummy_func,
        func_fdstat_dummy_func,
        func_subscription_clock_dummy_func,
        env,
      })
    }
    /// Dummy function to expose type into generated code
    pub fn subscription_clock_dummy_func(&self, store: &mut wasmer::Store,dummy: SubscriptionClock,)-> Result<(), wasmer::RuntimeError> {
      let SubscriptionClock{ id:id0, timeout:timeout0, precision:precision0, flags:flags0, } = dummy;
      let flags1 = flags0;
      self.func_subscription_clock_dummy_func.call(store, id0 as i32, wit_bindgen_wasmer::rt::as_i64(timeout0), wit_bindgen_wasmer::rt::as_i64(precision0), (flags1.bits >> 0) as i32, )?;
      Ok(())
    }
    pub fn fd_dummy_func(&self, store: &mut wasmer::Store,dummy: Fd,)-> Result<(), wasmer::RuntimeError> {
      self.func_fd_dummy_func.call(store, wit_bindgen_wasmer::rt::as_i32(dummy), )?;
      Ok(())
    }
    pub fn fdstat_dummy_func(&self, store: &mut wasmer::Store,dummy: Fdstat,)-> Result<(), wasmer::RuntimeError> {
      let Fdstat{ fs_filetype:fs_filetype0, fs_flags:fs_flags0, fs_rights_base:fs_rights_base0, fs_rights_inheriting:fs_rights_inheriting0, } = dummy;
      let flags1 = fs_flags0;
      let flags2 = fs_rights_base0;
      let flags3 = fs_rights_inheriting0;
      self.func_fdstat_dummy_func.call(store, fs_filetype0 as i32, (flags1.bits >> 0) as i32, (flags2.bits >> 0) as i32, (flags3.bits >> 0) as i32, )?;
      Ok(())
    }
  }
  #[allow(unused_imports)]
  use wasmer::AsStoreMut as _;
  #[allow(unused_imports)]
  use wasmer::AsStoreRef as _;
}
