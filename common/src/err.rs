use std::process;

pub trait OrExit<T> {
    fn or_exit(self, message: &str) -> T;
}

pub trait LogErr<T> {
    fn log_err(self, message: &str);
}

impl<T, E> OrExit<T> for Result<T, E>
where
    E: std::fmt::Debug,
{
    fn or_exit(self, message: &str) -> T {
        if let Err(err) = self {
            eprintln!("{message}: {err:?}");
            process::exit(1);
        }

        self.unwrap()
    }
}

impl<T, E> LogErr<T> for Result<T, E>
where
    E: std::fmt::Debug,
{
    fn log_err(self, message: &str) {
        if let Err(err) = self {
            eprintln!("{message}: {err:?}");
        }
    }
}
