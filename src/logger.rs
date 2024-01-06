#[macro_export]
macro_rules! rwlog_info {
    ($($arg:tt)*) => {{
        use chrono::Local;
        let time_txt = Local::now().format("%F %T%.3f").to_string();
        println!("{} {}", time_txt, format_args!($($arg)*));
    }}
}

#[macro_export]
macro_rules! rwlog_error {
    ($($arg:tt)*) => {{
        use chrono::Local;
        use colored::*;
        let time_txt = Local::now().format("%F %T%.3f").to_string();
        println!("{} {}", time_txt, format_args!($($arg)*).to_string().red());
    }}
}

#[macro_export]
macro_rules! rwlog_debug {
    ($($arg:tt)*) => {{
        if std::env::var("RUN_DEBUG").is_ok() {
            use chrono::Local;
            use colored::*;
            let time_txt = Local::now().format("%F %T%.3f").to_string();
            println!("{} {}", time_txt, format_args!($($arg)*).to_string().green());
        }
    }}
}
