//! This module provides exit codes for programs.
//!
//! The choice of an appropriate exit value is often ambigeous and whilst it is
//! impossible to provide an authoritative anthology that applies under all
//! circumstances, this module attempts to collect the most frequently
//! recognised exit codes across Unix systems.
//!
//! Exit statuses fall between 0 and 255 (inclusive), and codes greater than
//! zero indicate failure.  The range 125–128 is reserved shell-specific
//! statuses, including shell builtins and compound commands.  The range
//! 129–154 is reserved fatal signals, explained below.
//!
//! Usage:
//!
//! ```
//! use std::process;
//! use sysexit;
//!
//! let exit_status = process::Command::new("sh")
//!     .arg("-c").arg(format!("exit {}", 65))
//!     .status()
//!     .expect("failed to run sh(1)");
//! let exit_code = sysexit::from_status(exit_status);
//! println!("{}", exit_code);
//! ```
//!
//! This outputs:
//!
//! ```text
//! i/o error (74)
//! ```
//!
//! As a basis it encodes the exit codes of [sysexits(3)] from OpenBSD (64–78), exit statuses used by [bash(1)],
//! supplemented by codes created by shells when the command is terminated
//! by a fatal signal.  When the fatal signal is a number _N_, the latter
//! follows bash’s strategy of using the value 128 + _N_ as the exit status.
//! This means that the `SIGHUP` (1) signal will be recognised as the exit code
//! for the number 129.
//!
//! It should be pointed out that numeric exit codes are an absolute
//! abomination, but we are stuck with them.
//!
//! [sysexits(3)]: https://man.openbsd.org/sysexits.3

#![allow(unknown_lints, cast_lossless, doc_markdown, match_same_arms)]

extern crate libc;

use std::fmt;
use std::i8;
use std::process;
use std::io;

const SIGBASE: i32 = i8::MAX as i32 + 1;

/// A successful exit is always indicated by a status of 0, or
/// [`exit::Success`].  Exit codes greater than zero indicates failure.
///
/// [`exit::Success`]: enum.Code.html#variant.Success
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum Code {
    /// The process exited successfully.
    Success = 0,

    /// Generic failure.
    Failure = 1,

    /// Catch-all exit code when the process exits for an unknown reason.
    Unknown = 2,

    /// The command was used incorrectly, e.g. with the wrong number of
    /// arguments, a bad flag, bad syntax in a parameter, or whatever.
    Usage = 64,

    /// The input data was incorrect in some way.  This should only be used for
    /// user’s data and not system files.
    DataErr = 65,

    /// An input file (not a system file) did not exist or was not readable.
    /// This could also include erros like “No message” to a mailer (if it
    /// cared to catch it).
    NoInput = 66,

    /// The user specified did not exist.  This might be used for mail adresses
    /// or remote logins.
    NoUser = 67,

    /// The host specified did not exist.  This is used in mail addresses or
    /// network requests.
    NoHost = 68,

    /// A service is unavailable.  This can occur if a support program or file
    /// does not exist.  This can also be used as a catch-all message when
    /// something you wanted to do doesn’t work, but you don’t know why.
    Unavailable = 69,

    /// An internal software error has been detected.  This should be limited
    /// to non-operating system related errors if possible.
    Software = 70,

    /// An operating system error has been detected.  This is intended to be
    /// used for such things as “cannot fork”, or “cannot create pipe”.  It
    /// includes things like [getuid(2)] returning a user that does not exist
    /// in the passwd file.
    ///
    /// [getuid(2)]: https://man.openbsd.org/getuid.2
    OsErr = 71,

    /// Some system file (e.g. _/etc/passwd_, _/var/run/utmp_) does not exist,
    /// cannot be opened, or has some sort of error (e.g. syntax error).
    OsFile = 72,

    /// A (user specified) output file cannot be created.
    CantCreat = 73,

    /// An error occurred while doing I/O on some file.
    IoErr = 74,

    /// Temporary failure, indicating something that is not really an error.
    /// For example that a mailer could not create a connection, and the
    /// request should be reattempted later.
    TempFail = 75,

    /// The remote system returned something that was “not possible” during a
    /// protocol exchange.
    Protocol = 76,

    /// You did not have sufficient permission to perform the operation.  This
    /// is not intended for file system problems, which should use `NoInput` or
    /// `CantCreat`, but rather for high level permissions.
    NoPerm = 77,

    /// Something was found in an unconfigured or misconfigured state.
    Config = 78,

    /// Command was found but is not executable by the shell.
    NotExecutable = 126,

    /// Usually indicates that the command was not found by the shell, or that
    /// the command is found but that a library it requires is not found.
    NotFound = 127,

    /// The `SIGHUP` signal is sent to a process when its controlling terminal
    /// is closed.
    SIGHUP = SIGBASE + libc::SIGHUP,

    /// The `SIGINT` signal is sent to a process by its controlling terminal
    /// when a user wishes to interrupt the process.
    SIGINT = SIGBASE + libc::SIGINT,

    /// The `SIGKILL` signal is sent to a process to cause it to terminate
    /// immediately.  In contrast to `SIGTERM` and `SIGINT`, this signal cannot
    /// be caught or ignored, and the receiving process cannot perform any
    /// clean-up upon receiving this signal.
    SIGKILL = SIGBASE + libc::SIGKILL,

    /// The `SIGPIPE` signal is sent to a process when it attempts to write to
    /// a pipe without a process connected to the other end.
    SIGPIPE = SIGBASE + libc::SIGPIPE,

    /// The `SIGALRM` signal is sent to a process when the time limit specified
    /// in a call to a preceding alarm setting function (such as `setitimer`)
    /// elapses.
    SIGALRM = SIGBASE + libc::SIGALRM,

    /// The `SIGTERM` signal is sent to a process to request its termination.
    /// Unlike the `SIGKILL` signal, it can be caught and interpreted or
    /// ignored by the process.
    SIGTERM = SIGBASE + libc::SIGTERM,

    /// The `SIGUSR1` signal, like `SIGUSR2`, is sent to a process to indicate
    /// a user-defined condition.
    SIGUSR1 = SIGBASE + libc::SIGUSR1,

    /// The `SIGUSR2` signal, like `SIGUSR1`, is sent to a process to indicate
    /// a user-defined condition.
    SIGUSR2 = SIGBASE + libc::SIGUSR2,

    /// The `SIGVTALRM` signal is sent to a process when the time limit
    /// specified for the virtual alarm elapses.
    SIGVTALRM = SIGBASE + libc::SIGVTALRM,
}

/// Converts an `i32` primitive integer to an exit code.
impl From<i32> for Code {
    fn from(n: i32) -> Self {
        use self::Code::*;

        match n {
            0 => Success,
            1 => Failure,
            2 => Unknown,

            64 => Usage,
            65 => DataErr,
            66 => NoInput,
            67 => NoUser,
            68 => NoHost,
            69 => Unavailable,
            70 => Software,
            71 => OsErr,
            72 => OsFile,
            73 => CantCreat,
            74 => IoErr,
            75 => TempFail,
            76 => Protocol,
            77 => NoPerm,
            78 => Config,

            126 => NotExecutable,
            127 => NotFound,

            _ if n == SIGBASE + libc::SIGHUP => SIGHUP,
            _ if n == SIGBASE + libc::SIGINT => SIGINT,
            _ if n == SIGBASE + libc::SIGKILL => SIGKILL,
            _ if n == SIGBASE + libc::SIGUSR1 => SIGUSR1,
            _ if n == SIGBASE + libc::SIGUSR2 => SIGUSR2,
            _ if n == SIGBASE + libc::SIGPIPE => SIGPIPE,
            _ if n == SIGBASE + libc::SIGALRM => SIGALRM,
            _ if n == SIGBASE + libc::SIGTERM => SIGTERM,
            _ if n == SIGBASE + libc::SIGVTALRM => SIGVTALRM,

            _ => Unknown,
        }
    }
}

impl From<Option<i32>> for Code {
    fn from(maybe_n: Option<i32>) -> Self {
        match maybe_n {
            Some(n) => Code::from(n),
            None => Code::Unknown,
        }
    }
}

/// Converts [`std::process::ExitStatus`] to an exit code by looking at its
/// [`ExitStatus::code()`] value.
///
/// On Unix, if the process was terminated by a fatal signal, the corresponding
/// signal exit code is returned.  If the passed exit status cannot be
/// determined, `exit::Unknown` (2) is returned.
///
/// [`std::process::ExitStatus`]:
/// https://doc.rust-lang.org/std/process/struct.ExitStatus.html
/// [`ExitStatus::code()`]:
/// https://doc.rust-lang.org/std/process/struct.ExitStatus.html#method.code
impl From<process::ExitStatus> for Code {
    fn from(status: process::ExitStatus) -> Self {
        let n = platform_exit_code(status).unwrap_or(Code::Unknown as i32);
        From::from(n)
    }
}

impl From<io::ErrorKind> for Code {
    fn from(kind: io::ErrorKind) -> Self {
        use io::ErrorKind::*;
        match kind {
            NotFound => Code::OsFile,
            PermissionDenied => Code::NoPerm,
            AddrInUse | AddrNotAvailable => Code::Unavailable,
            ConnectionRefused | ConnectionReset | ConnectionAborted | NotConnected | BrokenPipe => {
                Code::Protocol
            }
            AlreadyExists => Code::CantCreat,
            InvalidInput | InvalidData => Code::DataErr,
            _ => Code::IoErr,
        }
    }
}

/// Provides a user-friendly explanation of the exit code.
impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Code::*;

        let reason = match *self {
            Success => "success",
            Failure => "failure",
            Unknown => "unknown",
            Usage => "usage",
            DataErr => "data",
            NoInput => "no input",
            NoUser => "no user",
            NoHost => "no host",
            Unavailable => "unavailable",
            Software => "software",
            OsErr => "os err",
            OsFile => "os file",
            CantCreat => "cannot create",
            IoErr => "i/o error",
            TempFail => "temporary failure",
            Protocol => "protocol",
            NoPerm => "permission denied",
            Config => "config",

            NotExecutable => "not executable",
            NotFound => "not found",

            SIGHUP => "hangup signal",
            SIGINT => "terminal interrupt signal",
            SIGKILL => "kill signal",
            SIGPIPE => "write on a pipe with no one to read it signal",
            SIGALRM => "alarm clock signal",
            SIGTERM => "termination signal",
            SIGUSR1 => "user-defined signal 1",
            SIGUSR2 => "user-defined signal 2",
            SIGVTALRM => "virtual timer expired signal",
        };

        write!(f, "{} ({})", reason, *self as i32)
    }
}

#[cfg(target_family = "unix")]
fn platform_exit_code(status: process::ExitStatus) -> Option<i32> {
    use std::os::unix::process::ExitStatusExt;
    status.code().or_else(|| status.signal())
}

#[cfg(not(target_family = "unix"))]
fn platform_exit_code(status: process::ExitStatus) -> Option<i32> {
    status.code()
}

pub use self::Code::*;

/// Converts [`std::process::ExitStatus`] to [`sysexit::Code`].
///
/// On Unix, if the process was terminated by a fatal signal, the corresponding
/// signal exit code is returned.  If the passed exit status cannot be
/// determined, [`sysexit::Unknown`] (2) is returned.
///
/// [`std::process::ExitStatus`]: https://doc.rust-lang.org/std/process/struct.ExitStatus.html
/// [`sysexit::Code`]: enum.Code.html
/// [`sysexit::Unknown`]: enum.Code.html#variant.Unknown
pub fn from_status(status: process::ExitStatus) -> Code {
    Code::from(status)
}

/// Determines if the provided [`std::process::ExitStatus`] was successful.
///
/// Example:
///
/// ```
/// use std::process;
/// use sysexit;
///
/// let exit_status = process::Command::new("true")
///     .status()
///     .expect("failed to run true(1)");
/// assert!(sysexit::is_success(exit_status));
/// ```
///
/// [`std::process::ExitStatus`]: https://doc.rust-lang.org/std/process/struct.ExitStatus.html
pub fn is_success(status: process::ExitStatus) -> bool {
    Code::from(status) == Success
}

/// Determines if the provided [`std::process::ExitStatus`] was unsuccessful.
///
/// Example:
///
/// ```
/// use std::process;
/// use sysexit;
///
/// let exit_status = process::Command::new("false")
///     .status()
///     .expect("failed to run false(1)");
/// assert!(sysexit::is_error(exit_status));
/// ```
///
/// [`std::process::ExitStatus`]: https://doc.rust-lang.org/std/process/struct.ExitStatus.html
pub fn is_error(status: process::ExitStatus) -> bool {
    !is_success(status)
}

/// Tests if the provided exit code is reserved, and has a special meaning in
/// shells.
pub fn is_reserved(n: i32) -> bool {
    (Success as i32 <= n && n <= Unknown as i32) || (Usage as i32 <= n && n <= Config as i32)
        || (NotExecutable as i32 <= n && n <= SIGVTALRM as i32)
}

/// Test if provided exit code is valid, that is within the 0–255 (inclusive)
/// range.
pub fn is_valid(n: i32) -> bool {
    0 <= n && n <= 255
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sigbase() {
        assert_eq!(SIGBASE, 128);
    }

    #[test]
    fn from_i32() {
        assert_eq!(Code::from(0), Success);
        assert_eq!(Code::from(1), Failure);
        assert_eq!(Code::from(2), Unknown);

        assert_eq!(Code::from(64), Usage);
        assert_eq!(Code::from(65), DataErr);
        assert_eq!(Code::from(66), NoInput);
        assert_eq!(Code::from(67), NoUser);
        assert_eq!(Code::from(68), NoHost);
        assert_eq!(Code::from(69), Unavailable);
        assert_eq!(Code::from(70), Software);
        assert_eq!(Code::from(71), OsErr);
        assert_eq!(Code::from(72), OsFile);
        assert_eq!(Code::from(73), CantCreat);
        assert_eq!(Code::from(74), IoErr);
        assert_eq!(Code::from(75), TempFail);
        assert_eq!(Code::from(76), Protocol);
        assert_eq!(Code::from(77), NoPerm);
        assert_eq!(Code::from(78), Config);

        assert_eq!(Code::from(126), NotExecutable);
        assert_eq!(Code::from(127), NotFound);

        assert_eq!(Code::from(129), SIGHUP);
        assert_eq!(Code::from(130), SIGINT);
        assert_eq!(Code::from(137), SIGKILL);
        assert_eq!(Code::from(138), SIGUSR1);
        assert_eq!(Code::from(140), SIGUSR2);
        assert_eq!(Code::from(141), SIGPIPE);
        assert_eq!(Code::from(142), SIGALRM);
        assert_eq!(Code::from(143), SIGTERM);
        assert_eq!(Code::from(154), SIGVTALRM);

        assert_eq!(Code::from(-1), Unknown);
        assert_eq!(Code::from(128), Unknown);
        assert_eq!(Code::from(162), Unknown);
    }

    fn exit_status(code: i32) -> process::ExitStatus {
        process::Command::new("sh")
            .arg("-c")
            .arg(format!("exit {}", code))
            .status()
            .expect("failed to run sh(1)")
    }

    #[test]
    fn from_exitstatus() {
        assert_eq!(Code::from(exit_status(0)), Success);
        assert_eq!(Code::from(exit_status(1)), Failure);
        assert_eq!(Code::from(exit_status(2)), Unknown);

        assert_eq!(Code::from(exit_status(64)), Usage);
        assert_eq!(Code::from(exit_status(65)), DataErr);
        assert_eq!(Code::from(exit_status(66)), NoInput);
        assert_eq!(Code::from(exit_status(67)), NoUser);
        assert_eq!(Code::from(exit_status(68)), NoHost);
        assert_eq!(Code::from(exit_status(69)), Unavailable);
        assert_eq!(Code::from(exit_status(70)), Software);
        assert_eq!(Code::from(exit_status(71)), OsErr);
        assert_eq!(Code::from(exit_status(72)), OsFile);
        assert_eq!(Code::from(exit_status(73)), CantCreat);
        assert_eq!(Code::from(exit_status(74)), IoErr);
        assert_eq!(Code::from(exit_status(75)), TempFail);
        assert_eq!(Code::from(exit_status(76)), Protocol);
        assert_eq!(Code::from(exit_status(77)), NoPerm);
        assert_eq!(Code::from(exit_status(78)), Config);

        assert_eq!(Code::from(exit_status(126)), NotExecutable);
        assert_eq!(Code::from(exit_status(127)), NotFound);

        assert_eq!(Code::from(exit_status(129)), SIGHUP);
        assert_eq!(Code::from(exit_status(130)), SIGINT);
        assert_eq!(Code::from(exit_status(137)), SIGKILL);
        assert_eq!(Code::from(exit_status(138)), SIGUSR1);
        assert_eq!(Code::from(exit_status(140)), SIGUSR2);
        assert_eq!(Code::from(exit_status(141)), SIGPIPE);
        assert_eq!(Code::from(exit_status(142)), SIGALRM);
        assert_eq!(Code::from(exit_status(143)), SIGTERM);
        assert_eq!(Code::from(exit_status(154)), SIGVTALRM);
    }

    #[test]
    fn success() {
        assert!(is_success(exit_status(0)));
        assert!(!is_success(exit_status(1)));
    }

    #[test]
    fn error() {
        assert!(is_error(exit_status(1)));
        assert!(!is_error(exit_status(0)));
    }

    #[test]
    fn reserved() {
        for n in 0..512 {
            println!("{}", n);
            match n {
                0...2 => assert!(is_reserved(n)),
                64...78 => assert!(is_reserved(n)),
                126...154 => assert!(is_reserved(n)),
                n => assert!(!is_reserved(n)),
            }
        }
    }

    #[test]
    fn valid() {
        for n in 0..512 {
            match n {
                0...255 => assert!(is_valid(n)),
                _ => assert!(!is_valid(n)),
            }
        }
    }
}
