#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

mod Error;

use std::collections::HashMap;
use std::env::args;
use std::process::exit;
use std::sync::{LazyLock, Mutex};
pub use Error::ArgumentError;

static ARGS: LazyLock<Vec<String>> = LazyLock::new(|| args().collect());
static HELPMAP: LazyLock<Mutex<HashMap<String, String>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
fn isFlag(arg: &str) -> bool {
    arg.starts_with("--")
}
fn isThisTheFlag(argToSearch: &str, flagYouWant: &str) -> bool {
    if !isFlag(argToSearch) {
        return false;
    }
    let flagToSearch = argToSearch.replace("--", "");
    flagToSearch == flagYouWant
}
pub fn displayHelp() {
    let _appName = ARGS.get(0).unwrap();
    let mut _helpList = HELPMAP
        .lock()
        .expect("You can't use this lib in parallel context.");
    println!("Usage of: {_appName}",);
    println!("--help: Show this message.");
    for (arg, help) in _helpList.iter() {
        println!("--{arg}: {help}");
    }
    print!("Example: {_appName} ");
    for (arg, _) in _helpList.iter() {
        print!(" --{arg} <value>");
    }
    println!();
}
fn collectArg(name: &str) -> (usize, bool) {
    let mut index = 0;
    let mut outOfRange = false;
    // TODO: Add the ability to add custom help messages
    if ARGS.len() > 1 && ARGS.get(1).unwrap() == "--help" {
        displayHelp();
        exit(42);
    }
    loop {
        if index + 1 >= ARGS.len() {
            outOfRange = true;
            break;
        }
        if isThisTheFlag(ARGS.get(index).unwrap(), name) {
            break;
        }
        index += 1;
    }
    (index, outOfRange)
}

fn parseNumerical<T: std::str::FromStr>(obj: typeFlag<T>) -> Result<T, ArgumentError> {
    let (index, outOfRange) = collectArg(&obj.name);

    // TODO: Find a better way to factor everything out
    if obj.required {
        if outOfRange {
            Err(ArgumentError::ARG_REQUIRED)
        } else {
            if isFlag(ARGS.get(index + 1).unwrap()) {
                Err(ArgumentError::ARG_REQUIRED)
            } else {
                if let Ok(i) = ARGS.get(index + 1).unwrap().parse::<T>() {
                    Ok(i)
                } else {
                    Err(ArgumentError::ARG_PROCESSING)
                }
            }
        }
    } else {
        if outOfRange {
            Ok(obj.value)
        } else {
            if isFlag(ARGS.get(index + 1).unwrap()) {
                Ok(obj.value)
            } else {
                if let Ok(i) = ARGS.get(index + 1).unwrap().parse::<T>() {
                    Ok(i)
                } else {
                    Err(ArgumentError::ARG_PROCESSING)
                }
            }
        }
    }
}
#[derive(Debug)]
pub struct typeFlag<T> {
    name: String,
    required: bool,
    value: T,
}

impl<T> typeFlag<T> {
    /// Sets the help message of the created flag.
    ///
    /// It accepts a `str` as an arguments.
    ///
    /// This function can only be called after `new` function.
    /// # Example:
    /// ```ignore
    /// let arg = SCLP::strFlag::new().setHelp("Arg help text.");
    /// ```
    ///

    pub fn setHelp(self, help: &str) -> Self {
        let mut _helpList = HELPMAP
            .lock()
            .expect("You can't use this lib in parallel context.");
        _helpList.insert(self.name.clone(), help.to_string());

        self
    }
    /// Sets the default value of the created flag.
    ///
    /// It accepts a `value` of type `T` as an arguments.
    ///
    /// This function is optional if not called the argument will be considered required and `parse` will exit and show the help message and `tryParse` will return `Err(ArgumentError)` in the case of user not providing the argument.
    ///
    /// This function can only be called after `new` function.
    /// # Example:
    /// ```ignore
    /// let arg = SCLP::strFlag::new().setHelp("Arg help text.").setDefault("abc");
    /// ```
    ///
    pub fn setDefault(mut self, value: T) -> Self {
        self.required = false;
        self.value = value;
        self
    }
}
pub trait Flag {
    type Value;
    fn new(name: &str) -> Self;
    /// Returns a `Result<T, ArgumentError>`.
    ///
    /// This function allows for better control in the case of handling required argument but not provided.
    ///
    ///# `ArgumentError`:
    /// `ARG_REQUIRED` will be returned in the case of the argument is requerd but the user didn't provide it.
    ///
    /// `ARG_PROCESSING` will be returned in the case of the argument parsing type error.
    /// # Example:
    /// ```ignore
    /// let arg =  SCLP::strFlag::new().setHelp("Arg help text.").setDefault("abc").tryParse().unwrap();
    ///
    /// ```
    ///
    fn tryParse(self) -> Result<Self::Value, ArgumentError>;
    ///Returns a `value` of type `T`
    ///
    /// This function is the simplest way and the easest to get the value of an argument.
    /// # IMPORTANT:
    /// Only call this function after you configured all the flags to generate the correct help message with all flags.
    /// # Example:
    /// ```ignore
    /// let name = SCLP::strFlag::new("name").setDefault("jhon".to_string()).setHelp("Your name.");
    /// let age = SCLP::intFlag::new("age").setDefault(22).setHelp("Your age.");
    /// let height = SCLP::floatFlag::new("height").setHelp("Your height.");
    /// let happy = SCLP::boolFlag::new("happy").setDefault(true).setHelp("Are you happy");
    /// println!(
    ///     "name: {} age: {} height: {} happy: {}",
    ///     name.parse(),
    ///     age.parse(),
    ///     height.parse(),
    ///     happy.parse()
    ///  );
    /// ```
    ///
    ///
    ///
    fn parse(self) -> Self::Value
    where
        Self: Sized,
    {
        if let Ok(s) = self.tryParse() {
            s
        } else {
            displayHelp();
            exit(1);
        }
    }
}
pub type strFlag = typeFlag<String>;

impl Flag for strFlag {
    type Value = String;
    fn new(name: &str) -> Self {
        let mut _helpList = HELPMAP
            .lock()
            .expect("You can't use this lib in parallel context.");
        _helpList.insert(name.to_string(), String::from("String flag."));
        strFlag {
            name: name.to_string(),
            value: String::new(),
            required: false,
        }
    }

    fn tryParse(self) -> Result<String, ArgumentError> {
        let (index, outOfRange) = collectArg(&self.name);
        if self.required {
            if outOfRange {
                Err(ArgumentError::ARG_REQUIRED)
            } else {
                if isFlag(ARGS.get(index + 1).unwrap()) {
                    Err(ArgumentError::ARG_REQUIRED)
                } else {
                    Ok(ARGS.get(index + 1).unwrap().to_string())
                }
            }
        } else {
            if outOfRange {
                Ok(self.value)
            } else {
                if isFlag(ARGS.get(index + 1).unwrap()) {
                    Ok(self.value)
                } else {
                    Ok(ARGS.get(index + 1).unwrap().to_string())
                }
            }
        }
    }
}

pub type intFlag = typeFlag<i32>;
impl Flag for intFlag {
    type Value = i32;

    fn new(name: &str) -> Self {
        let mut _helpList = HELPMAP
            .lock()
            .expect("You can't use this lib in parallel context.");
        _helpList.insert(name.to_string(), String::from("Integer flag."));
        intFlag {
            name: name.to_string(),
            value: 0,
            required: true,
        }
    }

    fn tryParse(self) -> Result<Self::Value, ArgumentError> {
        parseNumerical(self)
    }
}
pub type floatFlag = typeFlag<f32>;

impl Flag for floatFlag {
    type Value = f32;

    fn new(name: &str) -> Self {
        let mut _helpList = HELPMAP
            .lock()
            .expect("You can't use this lib in parallel context.");
        _helpList.insert(name.to_string(), String::from("Float flag."));
        floatFlag {
            name: name.to_string(),
            required: true,
            value: 0.0,
        }
    }

    fn tryParse(self) -> Result<Self::Value, ArgumentError> {
        parseNumerical(self)
    }
}

pub type boolFlag = typeFlag<bool>;

impl Flag for boolFlag {
    type Value = bool;

    fn new(name: &str) -> Self {
        let mut _helpList = HELPMAP
            .lock()
            .expect("You can't use this lib in parallel context.");
        _helpList.insert(name.to_string(), String::from("Boolean flag."));
        boolFlag {
            name: name.to_string(),
            required: true,
            value: false,
        }
    }

    fn tryParse(self) -> Result<Self::Value, ArgumentError> {
        for arg in ARGS.iter() {
            if isThisTheFlag(arg, &self.name) {
                return Ok(true);
            }
        }
        Ok(self.value)
    }
}

// TODO: tests ?
