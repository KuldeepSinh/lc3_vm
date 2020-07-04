use super::errno::Errno;
use super::time::TimeVal;
use super::Result;
use libc::{self, c_int};
use std::mem;
use std::os::unix::io::RawFd;
use std::ptr::null_mut;

use libc::FD_SETSIZE;

// FIXME: Change to repr(transparent) once it's stable
#[repr(C)]
#[derive(Clone, Copy)]
#[allow(missing_debug_implementations)]
pub struct FdSet(libc::fd_set);

impl FdSet {
    pub fn new() -> FdSet {
	let mut fdset = unsafe { mem::MaybeUninit::uninit().assume_init() };
        unsafe { libc::FD_ZERO(&mut fdset) };
        FdSet(fdset)
    }

    pub fn insert(&mut self, fd: RawFd) {
        unsafe { libc::FD_SET(fd, &mut self.0) };
    }

    pub fn remove(&mut self, fd: RawFd) {
        unsafe { libc::FD_CLR(fd, &mut self.0) };
    }

    pub fn contains(&mut self, fd: RawFd) -> bool {
        unsafe { libc::FD_ISSET(fd, &mut self.0) }
    }

    pub fn clear(&mut self) {
        unsafe { libc::FD_ZERO(&mut self.0) };
    }

    /// Finds the highest file descriptor in the set.
    ///
    /// Returns `None` if the set is empty.
    ///
    /// This can be used to calculate the `nfds` parameter of the [`select`] function.
    ///
    ///
    /// [`select`]: fn.select.html
    pub fn highest(&mut self) -> Option<RawFd> {
        for i in (0..FD_SETSIZE).rev() {
            let i = i as RawFd;
            if unsafe { libc::FD_ISSET(i, self as *mut _ as *mut libc::fd_set) } {
                return Some(i);
            }
        }

        None
    }
}

/// Monitors file descriptors for readiness
///
/// Returns the total number of ready file descriptors in all sets. The sets are changed so that all
/// file descriptors that are ready for the given operation are set.
///
/// When this function returns, `timeout` has an implementation-defined value.
///
/// # Parameters
///
/// * `nfds`: The highest file descriptor set in any of the passed `FdSet`s, plus 1. If `None`, this
///   is calculated automatically by calling [`FdSet::highest`] on all descriptor sets and adding 1
///   to the maximum of that.
/// * `readfds`: File descriptors to check for being ready to read.
/// * `writefds`: File descriptors to check for being ready to write.
/// * `errorfds`: File descriptors to check for pending error conditions.
/// * `timeout`: Maximum time to wait for descriptors to become ready (`None` to block
///   indefinitely).
///
/// # References
///
/// [select(2)](http://pubs.opengroup.org/onlinepubs/9699919799/functions/select.html)
///
/// [`FdSet::highest`]: struct.FdSet.html#method.highest
pub fn select<'a, N, R, W, E, T>(
    nfds: N,
    readfds: R,
    writefds: W,
    errorfds: E,
    timeout: T,
) -> Result<c_int>
where
    N: Into<Option<c_int>>,
    R: Into<Option<&'a mut FdSet>>,
    W: Into<Option<&'a mut FdSet>>,
    E: Into<Option<&'a mut FdSet>>,
    T: Into<Option<&'a mut TimeVal>>,
{
    let mut readfds = readfds.into();
    let mut writefds = writefds.into();
    let mut errorfds = errorfds.into();
    let timeout = timeout.into();

    let nfds = nfds.into().unwrap_or_else(|| {
        readfds
            .iter_mut()
            .chain(writefds.iter_mut())
            .chain(errorfds.iter_mut())
            .map(|set| set.highest().unwrap_or(-1))
            .max()
            .unwrap_or(-1)
            + 1
    });

    let readfds = readfds
        .map(|set| set as *mut _ as *mut libc::fd_set)
        .unwrap_or(null_mut());
    let writefds = writefds
        .map(|set| set as *mut _ as *mut libc::fd_set)
        .unwrap_or(null_mut());
    let errorfds = errorfds
        .map(|set| set as *mut _ as *mut libc::fd_set)
        .unwrap_or(null_mut());
    let timeout = timeout
        .map(|tv| tv as *mut _ as *mut libc::timeval)
        .unwrap_or(null_mut());

    let res = unsafe { libc::select(nfds, readfds, writefds, errorfds, timeout) };

    Errno::result(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::io::RawFd;

    #[test]
    fn fdset_insert() {
        let mut fd_set = FdSet::new();

        for i in 0..FD_SETSIZE {
            assert!(!fd_set.contains(i as RawFd));
        }

        fd_set.insert(7);

        assert!(fd_set.contains(7));
    }

    #[test]
    fn fdset_remove() {
        let mut fd_set = FdSet::new();

        for i in 0..FD_SETSIZE {
            assert!(!fd_set.contains(i as RawFd));
        }

        fd_set.insert(7);
        fd_set.remove(7);

        for i in 0..FD_SETSIZE {
            assert!(!fd_set.contains(i as RawFd));
        }
    }

    #[test]
    fn fdset_clear() {
        let mut fd_set = FdSet::new();
        fd_set.insert(1);
        fd_set.insert((FD_SETSIZE / 2) as RawFd);
        fd_set.insert((FD_SETSIZE - 1) as RawFd);

        fd_set.clear();

        for i in 0..FD_SETSIZE {
            assert!(!fd_set.contains(i as RawFd));
        }
    }

    #[test]
    fn fdset_highest() {
        let mut set = FdSet::new();
        assert_eq!(set.highest(), None);
        set.insert(0);
        assert_eq!(set.highest(), Some(0));
        set.insert(90);
        assert_eq!(set.highest(), Some(90));
        set.remove(0);
        assert_eq!(set.highest(), Some(90));
        set.remove(90);
        assert_eq!(set.highest(), None);

        set.insert(4);
        set.insert(5);
        set.insert(7);
        assert_eq!(set.highest(), Some(7));
    }
}
