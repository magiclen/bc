/*!
# bc (An arbitrary precision calculator language)

Use `bc` in the Rust Programming Language.

## Examples

```rust
#[macro_use] extern crate bc;

let result = bc!("2 + 6");

assert_eq!("8", result.unwrap());
```

```rust
#[macro_use] extern crate bc;

let result = bc!("2.5 + 6");

assert_eq!("8.5", result.unwrap());
```

```rust
#[macro_use] extern crate bc;

let result = bc_timeout!("99^99");

assert_eq!("369729637649726772657187905628805440595668764281741102430259972423552570455277523421410650010128232727940978889548326540119429996769494359451621570193644014418071060667659301384999779999159200499899", result.unwrap());
```

```rust
#[macro_use] extern crate bc;

let result = bc_timeout!(20, "99^99");

assert_eq!("369729637649726772657187905628805440595668764281741102430259972423552570455277523421410650010128232727940978889548326540119429996769494359451621570193644014418071060667659301384999779999159200499899", result.unwrap());
```
*/

pub extern crate subprocess;

use std::path::Path;

use subprocess::{Exec, PopenError, Redirection, ExitStatus};

#[derive(Debug)]
pub enum BCError {
    PopenError(PopenError),
    NoResult,
    Timeout,
    /// Maybe it is a syntax error.
    Error(String),
}


/// Call `bc`.
pub fn bc<P: AsRef<Path>, S: AsRef<str>>(bc_path: P, statement: S) -> Result<String, BCError> {
    let process = Exec::cmd(bc_path.as_ref().as_os_str()).arg("-l").arg("-q")
        .stdin(format!("{}\n", statement.as_ref()).as_str())
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Pipe);

    let capture = process.capture().map_err(|err| BCError::PopenError(err))?;

    let stderr = capture.stderr_str();

    if stderr.is_empty() {
        let stdout = capture.stdout_str();

        if stdout.is_empty() {
            Err(BCError::NoResult)
        } else {
            Ok(handle_output(stdout))
        }
    } else {
        Err(BCError::Error(handle_output(stderr)))
    }
}


/// Call `bc` with `timeout`.
pub fn bc_timeout<PT: AsRef<Path>, P: AsRef<Path>, S: AsRef<str>>(timeout_path: PT, timeout_secs: u32, bc_path: P, statement: S) -> Result<String, BCError> {
    let process = Exec::cmd(timeout_path.as_ref().as_os_str()).arg(format!("{}s", timeout_secs)).arg(bc_path.as_ref().as_os_str()).arg("-l").arg("-q")
        .stdin(format!("{}\n", statement.as_ref()).as_str())
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Pipe);

    let capture = process.capture().map_err(|err| BCError::PopenError(err))?;

    if let ExitStatus::Exited(status) = capture.exit_status {
        if status == 124 {
            return Err(BCError::Timeout);
        }
    }

    let stderr = capture.stderr_str();

    if stderr.is_empty() {
        let stdout = capture.stdout_str();

        if stdout.is_empty() {
            Err(BCError::NoResult)
        } else {
            Ok(handle_output(stdout))
        }
    } else {
        Err(BCError::Error(handle_output(stderr)))
    }
}

fn handle_output(output: String) -> String {
    let len = output.len();

    let mut output = output.into_bytes();

    let output = unsafe {
        output.set_len(len - 1);

        String::from_utf8_unchecked(output)
    };

    match output.find("\\\n") {
        Some(index) => {
            let mut s = String::from(&output[..index]);

            s.push_str(&output[(index + 2)..].replace("\\\n", ""));

            s
        }
        None => output
    }
}

/// Call `bc`.
#[macro_export]
macro_rules! bc {
    ($statement:expr) => {
        ::bc::bc("bc", $statement)
    };
    ($bc_path:expr, $statement:expr) => {
        ::bc::bc($bc_path, $statement)
    };
}

/// Call `bc` with `timeout`.
#[macro_export]
macro_rules! bc_timeout {
    ($statement:expr) => {
        ::bc::bc_timeout("timeout", 15, "bc", $statement)
    };
    ($timeout:expr, $statement:expr) => {
        ::bc::bc_timeout("timeout", $timeout, "bc", $statement)
    };
    ($timeout_path:expr, $timeout:expr, $bc_path:expr, $statement:expr) => {
        ::bc::bc_timeout($timeout_path, $timeout, $bc_path, $statement)
    };
}