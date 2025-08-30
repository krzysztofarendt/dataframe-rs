/// Prints a formatted debug message to standard output, prefixed with `"DEBUG:"`.
///
/// This macro works like [`println!`], but automatically adds a `"DEBUG: "` prefix
/// to the output. It accepts the same syntax as [`format!`], including
/// format strings with `{}` placeholders and arguments.
///
/// # Examples
///
/// ```
/// use dataframe::dbg_print;
/// dbg_print!("x = {}", 42);
/// // Prints: DEBUG: x = 42
///
/// let a = 1;
/// let b = 2;
/// dbg_print!("a = {}, b = {}", a, b);
/// // Prints: DEBUG: a = 1, b = 2
/// ```
#[macro_export]
macro_rules! dbg_print {
    ( $( $arg:tt )* ) => {
        println!("DEBUG: {}", format!($( $arg )*));
    }
}
