use crate::*;
use std::fmt;
use std::mem::{self, MaybeUninit};
use wasmer_derive::ValueType;
use wasmer_types::ValueType;
use wasmer_wasi_types_generated::wasi_snapshot0;

pub type __wasi_linkcount_t = u32;

#[derive(Copy, Clone)]
#[repr(C)]
pub union __wasi_subscription_u {
    pub clock: wasi_snapshot0::SubscriptionClock,
    pub fd_readwrite: __wasi_subscription_fs_readwrite_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __wasi_subscription_t {
    pub userdata: __wasi_userdata_t,
    pub type_: wasi_snapshot0::Eventtype,
    pub u: __wasi_subscription_u,
}

unsafe impl ValueType for __wasi_subscription_t {
    fn zero_padding_bytes(&self, bytes: &mut [MaybeUninit<u8>]) {
        macro_rules! field {
            ($($f:tt)*) => {
                &self.$($f)* as *const _ as usize - self as *const _ as usize
            };
        }
        macro_rules! field_end {
            ($($f:tt)*) => {
                field!($($f)*) + mem::size_of_val(&self.$($f)*)
            };
        }
        macro_rules! zero {
            ($start:expr, $end:expr) => {
                for i in $start..$end {
                    bytes[i] = MaybeUninit::new(0);
                }
            };
        }
        self.userdata
            .zero_padding_bytes(&mut bytes[field!(userdata)..field_end!(userdata)]);
        zero!(field_end!(userdata), field!(type_));
        self.type_
            .zero_padding_bytes(&mut bytes[field!(type_)..field_end!(type_)]);
        zero!(field_end!(type_), field!(u));
        match self.type_ {
            wasi_snapshot0::Eventtype::FdRead | wasi_snapshot0::Eventtype::FdWrite => unsafe {
                self.u.fd_readwrite.zero_padding_bytes(
                    &mut bytes[field!(u.fd_readwrite)..field_end!(u.fd_readwrite)],
                );
                zero!(field_end!(u.fd_readwrite), field_end!(u));
            },
            wasi_snapshot0::Eventtype::Clock => unsafe {
                self.u
                    .clock
                    .zero_padding_bytes(&mut bytes[field!(u.clock)..field_end!(u.clock)]);
                zero!(field_end!(u.clock), field_end!(u));
            },
        }
        zero!(field_end!(u), mem::size_of_val(self));
    }
}

pub type __wasi_whence_t = u8;
pub const __WASI_WHENCE_CUR: u8 = 0;
pub const __WASI_WHENCE_END: u8 = 1;
pub const __WASI_WHENCE_SET: u8 = 2;

#[derive(Copy, Clone, PartialEq, Eq, ValueType)]
#[repr(C)]
pub struct __wasi_filestat_t {
    pub st_dev: __wasi_device_t,
    pub st_ino: __wasi_inode_t,
    pub st_filetype: wasi_snapshot0::Filetype,
    pub st_nlink: __wasi_linkcount_t,
    pub st_size: __wasi_filesize_t,
    pub st_atim: __wasi_timestamp_t,
    pub st_mtim: __wasi_timestamp_t,
    pub st_ctim: __wasi_timestamp_t,
}

impl fmt::Debug for __wasi_filestat_t {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let convert_ts_into_time_string = |ts| {
            let tspec = ::time::OffsetDateTime::from_unix_timestamp_nanos(ts);
            format!("{} ({})", tspec.format("%a, %d %b %Y %T %z"), ts)
        };
        f.debug_struct("__wasi_filestat_t")
            .field("st_dev", &self.st_dev)
            .field("st_ino", &self.st_ino)
            .field(
                "st_filetype",
                &format!(
                    "{} ({})",
                    wasi_filetype_to_name(self.st_filetype),
                    self.st_filetype as u8,
                ),
            )
            .field("st_nlink", &self.st_nlink)
            .field("st_size", &self.st_size)
            .field(
                "st_atim",
                &convert_ts_into_time_string(self.st_atim as i128),
            )
            .field(
                "st_mtim",
                &convert_ts_into_time_string(self.st_mtim as i128),
            )
            .field(
                "st_ctim",
                &convert_ts_into_time_string(self.st_ctim as i128),
            )
            .finish()
    }
}

impl fmt::Debug for __wasi_subscription_t {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("__wasi_subscription_t")
            .field("userdata", &self.userdata)
            .field("type", &self.type_.to_str())
            .field(
                "u",
                match self.type_ {
                    wasi_snapshot0::Eventtype::Clock => unsafe { &self.u.clock },
                    wasi_snapshot0::Eventtype::FdRead | wasi_snapshot0::Eventtype::FdWrite => unsafe {
                        &self.u.fd_readwrite
                    },
                },
            )
            .finish()
    }
}
