use chrono::{DateTime, Local};

///! Pseudologging

/// Get a timestamp
pub fn timestamp() -> String {
    let time: DateTime<Local> = Local::now();
    return format!("{}", time.format("%Y-%m-%d %H:%M:%S%.6f"));
}


// https://danielkeep.github.io/practical-intro-to-macros.html
// https://veykril.github.io/tlborm/decl-macros/macros-methodical.html

#[macro_export]
macro_rules! info {
    // Match: a repeating sequence $();
    // matching one or more times, separated by a comma: $(),*
    // which repeats an expression, captured as a variable "arg":
    // ($arg:expr)
    // -> ( $($arg:expr),+ )
    
    ( $($arg:expr),+  ) => { println!("[{}] {}", crate::log::timestamp(), format_args!($($arg),+) ) }
}
