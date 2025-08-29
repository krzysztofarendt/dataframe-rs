/// Prints a formatted debug message to standard output, prefixed with `"DEBUG:"`.
///
/// This macro works like [`println!`], but automatically adds a `"DEBUG: "` prefix
/// to the output. It accepts the same syntax as [`format!`], including
/// format strings with `{}` placeholders and arguments.
///
/// # Examples
///
/// ```
/// debug!("x = {}", 42);
/// // Prints: DEBUG: x = 42
///
/// let a = 1;
/// let b = 2;
/// debug!("a = {}, b = {}", a, b);
/// // Prints: DEBUG: a = 1, b = 2
/// ```
macro_rules! debug_print {
    ( $( $arg:tt )* ) => {
        println!("DEBUG: {}", format!($( $arg )*));
    }
}

pub(crate) use debug_print;
