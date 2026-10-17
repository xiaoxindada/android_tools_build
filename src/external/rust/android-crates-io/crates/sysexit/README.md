sysexit
=======

Rust module that provides exit codes for programs.


Synopsis
--------

Usage:

	use std::process;
	use sysexit;

	let exit_status = process::Command::new("sh")
	    .arg("-c").arg(format!("exit {}", 74))
	    .status()
	    .expect("failed to run sh(1)");
	let exit_code = sysexit::from_status(exit_status);
	println!("{}", exit_code);

This outputs:

	i/o error (74)


Description
-----------

The choice of an appropriate exit value is often ambigeous and
whilst it is impossible to provide an authoritative anthology that
applies under all circumstances, this crate attempts to collec the
most frequently recognised exit codes across Unix systems.

Exit statuses fall between 0 and 255 (inclusive), and codes greater than
zero indicate failure.  The range 125–128 is reserved shell-specific
statuses, including shell builtins and compound commands.  The range
129–154 is reserved fatal signals, explained below.

As a basis it encodes the exit codes of [sysexits(3)] from OpenBSD
(64–78), exit statuses used by [bash(1)], supplemented by codes
created by shells when the command is terminated by a fatal signal.
When the fatal signal is a number _N_, the latter follows bash’s
strategy of using the value 128 + _N_ as the exit status.  This means
that the `SIGHUP` (1) signal will be recognised as the exit code
for the number 129.


Interface
---------

You can see the full API documentation on <https://docs.rs/sysexit>.

### [`pub fn from_status(status: std::process::ExitStatus) -> sysexit::Code`]
Converts [`std::process::ExitStatus`] to [`sysexit::Code`].

### [`pub fn is_success(status: std::process::ExitStatus) -> bool`]
Determines if the provided [`std::process::ExitStatus`] was successful.

### [`pub fn is_error(status: std::process::ExitStatus) -> bool`]
Determines if the provided [`std::process::ExitStatus`] was jnsuccessful.

### [`pub fn is_reserved(n: i32) -> bool`]
Test if the provided exit code is reserved and has a special meaning.

### [`pub fn is_valid(n: i32) -> bool`]
Test if the provided exit code is valid, in other words that it is
within the 0-255 (inclusive) range.


See also
--------

[_exit(2)], [exit(3)], [sysexits(3)], [bash(1)]


History
-------

This library is based on the `sysexits.h` file that first appeared
in 4.0BSD for use by the delivermail utility, later renamed to
[sendmail(8)].  It was further expanded with fatal signals from
[bash(1)].

You can consult the [CHANGES.md][] file for a record of all notable
changes to the library.


Authors
-------

Eric Allman invented the `sysexits.h` file in 1980.  Much of the
documentation for this library is based on the [sysexits(3)] man
page written by Joerg Wunsch, based on Eric’s original comments.
The `is_reserved` and `is_valid` functions were written by Richard
Fussenegger.  The Rust crate was written by [Andreas Tolfsen].


Bugs
----

The choice of an appropriate exit value is often ambigeous.


[Andreas Tolfsen]: https://sny.no/
[bash(1)]: https://linux.die.net/man/1/bash
[_exit(2)]: https://man.openbsd.org/_exit.2
[exit(3)]: https://man.openbsd.org/exit.3
[`pub fn from_status(status: std::process::ExitStatus) -> sysexit::Code`]: https://docs.rs/sysexit/newest/sysexit/fn.from_status.html
[`pub fn is_error(status: std::process::ExitStatus) -> bool`]: https://docs.rs/sysexit/newest/sysexit/fn.is_error.html
[`pub fn is_reserved(n: i32) -> bool`]: https://docs.rs/sysexit/newest/sysexit/fn.is_reserved.html
[`pub fn is_success(status: std::process::ExitStatus) -> bool`]: https://docs.rs/sysexit/newest/sysexit/fn.is_success.html
[`pub fn is_valid(n: i32) -> bool`]: https://docs.rs/sysexit/newest/sysexit/fn.is_valid.html
[sendmail(8)]: https://man.openbsd.org/sendmail.8
[`std::process::ExitStatus`]: https://doc.rust-lang.org/std/process/struct.ExitStatus.html
[`sysexit::Code`]: https://docs.rs/sysexit/newest/enum.Code.html
[`sysexits(3)]: https://man.openbsd.org/sysexits.3
[`sysexit::Unknown`]: https://docs.rs/sysexit/newest/enum.Code.html#variant.Unknown
