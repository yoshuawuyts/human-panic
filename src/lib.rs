#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]

extern crate termcolor;

use std::error::Error;
use std::panic;

/// Catch any error handlers that occur, and
// Cargo env vars available:
// https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates
pub fn catch_unwind<F: FnOnce() -> Result<(), Box<Error>>>(f: F) {
  panic::set_hook(Box::new(|_panic_info| {
    // TODO: create log report.
    if let Err(e) = print_msg() {
      eprintln!("Error generating panic message: {}", e);
    }
  }));

  match f() {
    Ok(_) => {}
    _ => { /* TODO: create log report. */ }
  }
}

use std::io::{Result as IoResult, Write};
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

fn print_msg() -> IoResult<()> {
  let stderr = BufferWriter::stderr(ColorChoice::Auto);
  let mut buffer = stderr.buffer();
  buffer.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;

  let _version = env!("CARGO_PKG_VERSION");
  let name = env!("CARGO_PKG_NAME");
  let authors = env!("CARGO_PKG_AUTHORS");
  let homepage = env!("CARGO_PKG_HOMEPAGE");

  writeln!(&mut buffer, "Well, this is embarrasing.\n")?;
  writeln!(&mut buffer, "{} had a problem and crashed. To help us diagnose the problem you can send us a crash report.\n", name)?;
  writeln!(&mut buffer, "We have generated a report file at \"<reports not generated yet>\". Submit an issue or email with the subject of \"{} Crash Report\" and include the report as an attachment.\n", name)?;

  if !homepage.is_empty() {
    writeln!(&mut buffer, "- Homepage: {}", homepage)?;
  }
  if !authors.is_empty() {
    writeln!(&mut buffer, "- Authors: {}", authors)?;
  }
  writeln!(&mut buffer, "\nWe take privacy seriously, and do not perform any automated error collection. In order to improve the software, we rely on people to submit reports.\n")?;
  writeln!(&mut buffer, "Thank you kindly!")?;

  stderr.print(&buffer).unwrap();

  Ok(())
}
