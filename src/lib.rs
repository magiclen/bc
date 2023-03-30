/*!
# bc (An arbitrary precision calculator language)

Use `bc` in the Rust Programming Language.

## Examples

```rust
let result = bc::bc!("2 + 6");

assert_eq!("8", result.unwrap());
```

```rust
let result = bc::bc!("2.5 + 6");

assert_eq!("8.5", result.unwrap());
```

```rust
let result = bc::bc_timeout!("99^99");

assert_eq!("369729637649726772657187905628805440595668764281741102430259972423552570455277523421410650010128232727940978889548326540119429996769494359451621570193644014418071060667659301384999779999159200499899", result.unwrap());
```

```rust
let result = bc::bc_timeout!(20, "99^99");

assert_eq!("369729637649726772657187905628805440595668764281741102430259972423552570455277523421410650010128232727940978889548326540119429996769494359451621570193644014418071060667659301384999779999159200499899", result.unwrap());
```
*/

use std::{
    error::Error,
    fmt::{Display, Error as FmtError, Formatter},
    io,
    path::Path,
    process::Stdio,
};

use execute::{command_args, Execute};

#[derive(Debug)]
pub enum BCError {
    IOError(io::Error),
    NoResult,
    Timeout,
    /// Maybe it is a syntax error.
    Error(String),
}

impl From<io::Error> for BCError {
    #[inline]
    fn from(err: io::Error) -> Self {
        BCError::IOError(err)
    }
}

impl From<String> for BCError {
    #[inline]
    fn from(err: String) -> Self {
        BCError::Error(err)
    }
}

impl Display for BCError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        match self {
            BCError::IOError(err) => Display::fmt(err, f),
            BCError::NoResult => f.write_str("There is no result of BC."),
            BCError::Timeout => f.write_str("The BC calculation has timed out."),
            BCError::Error(text) => f.write_str(text.as_str()),
        }
    }
}

impl Error for BCError {}

/// Call `bc`.
pub fn bc<P: AsRef<Path>, S: AsRef<str>>(bc_path: P, statement: S) -> Result<String, BCError> {
    let mut command = command_args!(bc_path.as_ref(), "-l", "-q");

    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());

    let output = command.execute_input_output(format!("{}\n", statement.as_ref()).as_bytes())?;

    if output.stderr.is_empty() {
        if output.stdout.is_empty() {
            Err(BCError::NoResult)
        } else {
            Ok(handle_output(unsafe { String::from_utf8_unchecked(output.stdout) }))
        }
    } else {
        Err(BCError::Error(handle_output(unsafe { String::from_utf8_unchecked(output.stderr) })))
    }
}

/// Call `bc` with `timeout`.
pub fn bc_timeout<PT: AsRef<Path>, P: AsRef<Path>, S: AsRef<str>>(
    timeout_path: PT,
    timeout_secs: u32,
    bc_path: P,
    statement: S,
) -> Result<String, BCError> {
    let mut command = command_args!(
        timeout_path.as_ref(),
        format!("{}s", timeout_secs),
        bc_path.as_ref(),
        "-l",
        "-q"
    );

    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());

    let output = command.execute_input_output(format!("{}\n", statement.as_ref()).as_bytes())?;

    if let Some(code) = output.status.code() {
        if code == 124 {
            return Err(BCError::Timeout);
        }
    }

    if output.stderr.is_empty() {
        if output.stdout.is_empty() {
            Err(BCError::NoResult)
        } else {
            Ok(handle_output(unsafe { String::from_utf8_unchecked(output.stdout) }))
        }
    } else {
        Err(BCError::Error(handle_output(unsafe { String::from_utf8_unchecked(output.stderr) })))
    }
}

fn handle_output(mut output: String) -> String {
    let length = output.len();

    unsafe {
        output.as_mut_vec().set_len(length - 1);
    }

    match output.find("\\\n") {
        Some(index) => {
            let mut s = String::from(&output[..index]);

            s.push_str(&output[(index + 2)..].replace("\\\n", ""));

            s
        },
        None => output,
    }
}

/// Call `bc`.
#[macro_export]
macro_rules! bc {
    ($statement:expr) => {
        $crate::bc("bc", $statement)
    };
    ($bc_path:expr, $statement:expr) => {
        $crate::bc($bc_path, $statement)
    };
}

/// Call `bc` with `timeout`.
#[macro_export]
macro_rules! bc_timeout {
    ($statement:expr) => {
        $crate::bc_timeout("timeout", 15, "bc", $statement)
    };
    ($timeout:expr, $statement:expr) => {
        $crate::bc_timeout("timeout", $timeout, "bc", $statement)
    };
    ($timeout_path:expr, $timeout:expr, $bc_path:expr, $statement:expr) => {
        $crate::bc_timeout($timeout_path, $timeout, $bc_path, $statement)
    };
}
