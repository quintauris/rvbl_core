#[macro_export]
macro_rules! line {
    () => (println!());
    ($color:tt, $fmt:expr) => {
        {
            use colored::Colorize;
            (println!("{}", $fmt.$color()))
        }
    };
    ($color:tt, $fmt:expr, $($arg:tt)*) => {
        {
            use colored::Colorize;
            (println!("{}", format!($fmt, $($arg)*).$color()))
        }
    };
}

pub(crate) use line;

macro_rules! info {
    () => (println!());
    ($fmt:expr) => (line!(bright_blue, $fmt));
    ($fmt:expr, $($arg:tt)*) => (line!(bright_blue, $fmt, $($arg)*));
}

pub(crate) use info;

macro_rules! spawning {
    () => (println!());
    ($fmt:expr) => (line!(green, $fmt));
    ($fmt:expr, $($arg:tt)*) => (line!(green, $fmt, $($arg)*));
}

pub(crate) use spawning;

macro_rules! warning {
    () => (println!());
    ($fmt:expr) => (line!(yellow, $fmt));
    ($fmt:expr, $($arg:tt)*) => (line!(yellow, $fmt, $($arg)*));
}

pub(crate) use warning;
