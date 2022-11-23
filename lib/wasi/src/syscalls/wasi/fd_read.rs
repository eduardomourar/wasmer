use wasmer_vfs::AsyncReadExt;

use super::*;
use crate::syscalls::*;

/// ### `fd_read()`
/// Read data from file descriptor
/// Inputs:
/// - `Fd fd`
///     File descriptor from which data will be read
/// - `const __wasi_iovec_t *iovs`
///     Vectors where data will be stored
/// - `u32 iovs_len`
///     Length of data in `iovs`
/// Output:
/// - `u32 *nread`
///     Number of bytes read
///
pub fn fd_read<M: MemorySize>(
    mut ctx: FunctionEnvMut<'_, WasiEnv>,
    fd: WasiFd,
    iovs: WasmPtr<__wasi_iovec_t<M>, M>,
    iovs_len: M::Offset,
    nread: WasmPtr<M::Offset, M>,
) -> Result<Errno, WasiError> {
    trace!(
        "wasi[{}:{}]::fd_read: fd={}",
        ctx.data().pid(),
        ctx.data().tid(),
        fd
    );

    let offset = {
        let mut env = ctx.data();
        let state = env.state.clone();
        let inodes = state.inodes.clone();

        let fd_entry = wasi_try_ok!(state.fs.get_fd(fd));
        fd_entry.offset.load(Ordering::Acquire) as usize
    };

    fd_read_internal::<M>(ctx, fd, iovs, iovs_len, offset, nread)
}

/// ### `fd_pread()`
/// Read from the file at the given offset without updating the file cursor.
/// This acts like a stateless version of Seek + Read
/// Inputs:
/// - `Fd fd`
///     The file descriptor to read the data with
/// - `const __wasi_iovec_t* iovs'
///     Vectors where the data will be stored
/// - `size_t iovs_len`
///     The number of vectors to store the data into
/// - `Filesize offset`
///     The file cursor to use: the starting position from which data will be read
/// Output:
/// - `size_t nread`
///     The number of bytes read
pub fn fd_pread<M: MemorySize>(
    ctx: FunctionEnvMut<'_, WasiEnv>,
    fd: WasiFd,
    iovs: WasmPtr<__wasi_iovec_t<M>, M>,
    iovs_len: M::Offset,
    offset: Filesize,
    nread: WasmPtr<M::Offset, M>,
) -> Result<Errno, WasiError> {
    trace!(
        "wasi[{}:{}]::fd_pread: fd={}, offset={}",
        ctx.data().pid(),
        ctx.data().tid(),
        fd,
        offset
    );

    fd_read_internal::<M>(ctx, fd, iovs, iovs_len, offset as usize, nread)
}

/// ### `fd_pread()`
/// Read from the file at the given offset without updating the file cursor.
/// This acts like a stateless version of Seek + Read
/// Inputs:
/// - `Fd fd`
///     The file descriptor to read the data with
/// - `const __wasi_iovec_t* iovs'
///     Vectors where the data will be stored
/// - `size_t iovs_len`
///     The number of vectors to store the data into
/// - `Filesize offset`
///     The file cursor to use: the starting position from which data will be read
/// Output:
/// - `size_t nread`
///     The number of bytes read
fn fd_read_internal<M: MemorySize>(
    mut ctx: FunctionEnvMut<'_, WasiEnv>,
    fd: WasiFd,
    iovs: WasmPtr<__wasi_iovec_t<M>, M>,
    iovs_len: M::Offset,
    offset: usize,
    nread: WasmPtr<M::Offset, M>,
) -> Result<Errno, WasiError> {
    wasi_try_ok!(ctx.data().clone().process_signals_and_exit(&mut ctx)?);

    let mut env = ctx.data();
    let state = env.state.clone();
    let inodes = state.inodes.clone();

    let fd_entry = wasi_try_ok!(state.fs.get_fd(fd));
    let is_stdio = fd_entry.is_stdio;

    let bytes_read = {
        if is_stdio == false {
            if !fd_entry.rights.contains(Rights::FD_READ) {
                // TODO: figure out the error to return when lacking rights
                return Ok(Errno::Access);
            }
        }

        let is_non_blocking = fd_entry.flags.contains(Fdflags::NONBLOCK);
        let inode_idx = fd_entry.inode;

        let max_size = {
            let memory = env.memory_view(&ctx);
            let iovs_arr = wasi_try_mem_ok!(iovs.slice(&memory, iovs_len));
            let mut max_size = 0usize;
            for iovs in iovs_arr.iter() {
                let iovs = wasi_try_mem_ok!(iovs.read());
                let buf_len: usize =
                    wasi_try_ok!(iovs.buf_len.try_into().map_err(|_| Errno::Overflow));
                max_size += buf_len;
            }
            max_size
        };

        let bytes_read = {
            let inodes = inodes.read().unwrap();
            let inode = &inodes.arena[inode_idx];
            let mut guard = inode.write();
            match guard.deref_mut() {
                Kind::File { handle, .. } => {
                    if let Some(handle) = handle {
                        let handle = handle.clone();
                        drop(inode);
                        drop(guard);
                        drop(inodes);

                        let data = wasi_try_ok!(__asyncify(
                            &mut ctx,
                            if is_non_blocking {
                                Some(Duration::ZERO)
                            } else {
                                None
                            },
                            async move {
                                let mut handle = handle.write().unwrap();
                                if is_stdio == false {
                                    handle
                                        .seek(std::io::SeekFrom::Start(offset as u64))
                                        .await
                                        .map_err(map_io_err)?;
                                }

                                let mut data = Vec::with_capacity(max_size);
                                unsafe { data.set_len(max_size) };
                                let amt = handle.read(&mut data[..]).await.map_err(map_io_err)?;
                                unsafe { data.set_len(amt) };
                                Ok(data)
                            }
                        )
                        .map_err(|err| match err {
                            Errno::Timedout => Errno::Again,
                            a => a,
                        }));
                        env = ctx.data();

                        let memory = env.memory_view(&ctx);
                        let iovs_arr = wasi_try_mem_ok!(iovs.slice(&memory, iovs_len));
                        wasi_try_ok!(read_bytes(&data[..], &memory, iovs_arr))
                    } else {
                        return Ok(Errno::Inval);
                    }
                }
                Kind::Socket { socket } => {
                    let socket = socket.clone();

                    drop(guard);
                    drop(inodes);

                    let data = wasi_try_ok!(__asyncify(
                        &mut ctx,
                        if is_non_blocking {
                            Some(Duration::ZERO)
                        } else {
                            None
                        },
                        async move { socket.recv(max_size).await }
                    )
                    .map_err(|err| match err {
                        Errno::Timedout => Errno::Again,
                        a => a,
                    }));
                    env = ctx.data();

                    let data_len = data.len();
                    let mut reader = &data[..];
                    let memory = env.memory_view(&ctx);
                    let iovs_arr = wasi_try_mem_ok!(iovs.slice(&memory, iovs_len));
                    let bytes_read =
                        wasi_try_ok!(read_bytes(reader, &memory, iovs_arr).map(|_| data_len));
                    bytes_read
                }
                Kind::Pipe { pipe } => {
                    let mut pipe = pipe.clone();

                    drop(guard);
                    drop(inodes);

                    let data = wasi_try_ok!(__asyncify(
                        &mut ctx,
                        if is_non_blocking {
                            Some(Duration::ZERO)
                        } else {
                            None
                        },
                        async move {
                            let mut data = Vec::with_capacity(max_size);
                            unsafe { data.set_len(max_size) };
                            let amt = wasmer_vfs::AsyncReadExt::read(&mut pipe, &mut data[..])
                                .await
                                .map_err(map_io_err)?;
                            unsafe { data.set_len(amt) };
                            Ok(data)
                        }
                    )
                    .map_err(|err| match err {
                        Errno::Timedout => Errno::Again,
                        a => a,
                    }));
                    env = ctx.data();

                    let data_len = data.len();
                    let mut reader = &data[..];

                    let memory = env.memory_view(&ctx);
                    let iovs_arr = wasi_try_mem_ok!(iovs.slice(&memory, iovs_len));
                    let bytes_read =
                        wasi_try_ok!(read_bytes(reader, &memory, iovs_arr).map(|_| data_len));
                    bytes_read
                }
                Kind::Dir { .. } | Kind::Root { .. } => {
                    // TODO: verify
                    return Ok(Errno::Isdir);
                }
                Kind::EventNotifications {
                    counter: ref_counter,
                    is_semaphore: ref_is_semaphore,
                    wakers: ref_wakers,
                    ..
                } => {
                    let counter = Arc::clone(ref_counter);
                    let is_semaphore: bool = *ref_is_semaphore;
                    let wakers = Arc::clone(ref_wakers);

                    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
                    {
                        let mut guard = wakers.lock().unwrap();
                        guard.push_front(tx);
                    }

                    drop(ref_counter);
                    drop(ref_is_semaphore);
                    drop(ref_wakers);
                    drop(guard);
                    drop(inodes);

                    let ret;
                    loop {
                        let val = counter.load(Ordering::Acquire);
                        if val > 0 {
                            let new_val = if is_semaphore { val - 1 } else { 0 };
                            if counter
                                .compare_exchange(val, new_val, Ordering::AcqRel, Ordering::Acquire)
                                .is_ok()
                            {
                                let mut memory = env.memory_view(&ctx);
                                let reader = val.to_ne_bytes();
                                let iovs_arr = wasi_try_mem_ok!(iovs.slice(&memory, iovs_len));
                                ret = wasi_try_ok!(read_bytes(&reader[..], &memory, iovs_arr));
                                break;
                            } else {
                                continue;
                            }
                        }

                        // If its none blocking then exit
                        if is_non_blocking {
                            return Ok(Errno::Again);
                        }

                        // Yield until the notifications are triggered
                        let tasks_inner = env.tasks.clone();
                        rx = wasi_try_ok!(__asyncify(&mut ctx, None, async move {
                            let _ = rx.recv().await;
                            Ok(rx)
                        })
                        .map_err(|err| match err {
                            Errno::Timedout => Errno::Again,
                            a => a,
                        }));
                        env = ctx.data();
                    }
                    ret
                }
                Kind::Symlink { .. } => unimplemented!("Symlinks in wasi::fd_read"),
                Kind::Buffer { buffer } => {
                    let memory = env.memory_view(&ctx);
                    let iovs_arr = wasi_try_mem_ok!(iovs.slice(&memory, iovs_len));
                    wasi_try_ok!(read_bytes(&buffer[offset..], &memory, iovs_arr))
                }
            }
        };

        if is_stdio == false {
            // reborrow
            let mut fd_map = state.fs.fd_map.write().unwrap();
            let fd_entry = wasi_try_ok!(fd_map.get_mut(&fd).ok_or(Errno::Badf));
            fd_entry
                .offset
                .fetch_add(bytes_read as u64, Ordering::AcqRel);
        }

        bytes_read
    };

    let bytes_read: M::Offset = wasi_try_ok!(bytes_read.try_into().map_err(|_| Errno::Overflow));
    trace!(
        "wasi[{}:{}]::fd_read: bytes_read={}",
        ctx.data().pid(),
        ctx.data().tid(),
        bytes_read
    );

    let memory = env.memory_view(&ctx);
    let nread_ref = nread.deref(&memory);
    wasi_try_mem_ok!(nread_ref.write(bytes_read));

    Ok(Errno::Success)
}
