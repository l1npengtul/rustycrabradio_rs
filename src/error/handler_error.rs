use snafu::{ensure, Backtrace, ErrorCompat, ResultExt, Snafu};

#[derive(Debug, Snafu)]
pub enum HandlerError {
    #[snafu(display("Could not handler, serenity returned: {}", why))]
    HandlerGetError{
        why : String,
    }
}