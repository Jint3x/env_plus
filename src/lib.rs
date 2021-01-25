//! A library which easily allows you to set ENV variables in your process through a file.
//! 
//! The library is extremly simple, but also customizeable and it allows you to set your own value delimiter and comment style.
//! You can use external config files such as .dotenv too. All you have to do is specify which delimiter and comment style need to be used.


//! # Example
//! 
//! ### Cargo.toml
//! ```toml
//! [dependenices]
//! env_plus = "0.1.0"
//! ```
//! ### .env_plus
//! ```
//! // This is a comment!
//! SECRET=YOUR_SECRET
//! ```

//! ### main<nolink>.rs
//! ```rust
//! use env_plus::EnvLoader;
//! 
//! fn main() {
//!     EnvLoader::new()
//!     activate();
//!
//!     let secret = std::env::var("SECRET").unwrap();
//!     assert_eq!(secret, String::from("YOUR_SECRET"))
//! }
//! ```
//! 
//! For more advanced usage, please look at the documentation for each method 
//! on the EnvLoader struct. There're plenty of examples of how to use this 
//! library <br />


use std::fs;
mod tests;


/// The entry point of the library
/// 
/// The EnvLoader uses few default values which are listed below
/// * A file - '.env_plus' which is relative to the current directory. You can specify your own file with fn change_file
/// * A comment style - '//' which makes the program ignore everything after it. By putting it on the beginning of the line, the whole line is marked as a comment
/// * Delimiter - '=' once placed in each line, everything on the left is marked as a key and on the right as value (only the first = is used). Change with fn change_delimiter
/// * Overwrite - bool (default false) which specifies if the already exisiting ENVs will be replaced or not if you have a var with the same name. Change with fn overwrite_envs
#[derive(Clone)]
pub struct EnvLoader {
    file: String,
    comment: String,
    value_delimiter: String,
    overwrite: bool
}


impl EnvLoader {

    /// Create a new EnvLoader instance.
    /// 
    /// # Examples
    /// 
    /// ```
    /// // .env_plus
    /// 
    /// // Double slash is used for commenting, equal sign is used for assiging a value by default.
    /// SECRET=YOUR_SECRET
    /// ```
    /// 
    /// ``` 
    /// // main.rs
    /// use env_plus::EnvLoader;
    /// 
    /// fn main() {
    ///     EnvLoader::new()
    ///     .activate();
    /// 
    ///     let secret = std::env::var("SECRET").unwrap();
    ///     assert_eq!(secret, String::from("YOUR_SECRET"))
    /// }
    /// ```
    /// 
    pub fn new() -> EnvLoader {
        EnvLoader {
            file: String::from("./.env_plus"),
            comment: String::from("//"),
            value_delimiter: String::from("="),
            overwrite: false,
        }
    }

    /// Changes the file which will be used to parse ENVs from. Any file can be used as long as you set its 
    /// comment style and delimiter. 
    /// 
    /// # Examples
    /// 
    /// ```
    /// // my_special_file.extension
    /// 
    /// SECRET=YOUR_SECRET
    /// ```
    /// 
    /// ```
    /// // main.rs
    /// use env_plus::EnvLoader;
    /// 
    /// fn main() {
    ///     EnvLoader::new()
    ///     .change_file(String::from("./my_special_file.extension"))
    ///     .activate();
    /// 
    ///     let secret = std::env::var("SECRET").unwrap();
    /// 
    ///     // YOUR_SECRET will be in your special file which we loaded above. 
    ///     assert_eq!(secret, String::from("YOUR_SECRET"));
    /// }
    /// ```
    pub fn change_file(mut self, path: String) -> Self {
        self.file = path;

        self
    }

    /// Sets a new value to be marked as a comment in the file and not 
    /// be loaded.
    /// 
    /// # Examples
    /// 
    /// ```
    /// // .env_plus 
    /// 
    /// --This is a now a comment
    /// SECRET=YOUR_SECRET
    /// ```
    /// 
    /// ```
    /// // main.rs
    /// use env_plus::EnvLoader;
    /// 
    /// fn main() {
    ///     EnvLoader::new()
    ///     .change_comment(String::from("--"))
    ///     .activate();
    /// 
    ///     let secret = std::env::var("SECRET").unwrap();
    ///     assert_eq!(secret, String::from("YOUR_SECRET"));
    /// }
    /// ```
    pub fn change_comment(mut self, comment: String) -> Self {
        self.comment = comment;

        self
    }

    /// Change the delimiter that will be used to parse the file lines.
    /// The default delimiter is =
    /// 
    /// # Examples
    /// 
    /// ```
    /// // .env_plus
    /// 
    /// SECRET===YOUR_SECRET
    /// ```
    /// 
    /// ```
    /// // main.rs 
    /// use env_plus::EnvLoader;
    /// 
    /// fn main() {
    ///     EnvLoader::new()
    ///     .change_delimiter(String::from("==="))
    ///     .activate();
    /// 
    ///     let secret = std::env::var("SECRET").unwrap();
    ///     assert_eq!(secret, String::from("YOUR_SECRET"))
    /// }
    /// ```
    pub fn change_delimiter(mut self, delimiter: String) ->  Self {
        self.value_delimiter = delimiter;

        self
    }

    /// If true is passed, all current ENV vars that have the same names as the ones in 
    /// the file will be overwritten, otherwise they won't.
    /// 
    /// # Examples
    /// 
    /// ```
    /// // .env_plus
    /// 
    /// SECRET=YOUR_SECRET
    /// ```
    /// 
    /// ```
    /// // main.rs
    /// use env_plus::EnvLoader;
    /// 
    /// fn main() {
    ///     std::env::set_var("SECRET", "MY_SECRET")
    /// 
    ///     EnvLoader::new()
    ///     .overwrite_envs(true)
    ///     .activate();
    /// 
    ///     let secret = std::env::var("SECRET").unwrap();
    ///     assert_eq!(secret, String::from("YOUR_SECRET"));
    /// }
    /// ```
    pub fn overwrite_envs(mut self, overwrite: bool) -> Self {
        self.overwrite = overwrite;

        self
    }


    /// Activate the module and load your ENV file.
    /// 
    /// # Examples 
    /// 
    /// ```
    /// // special.env 
    /// 
    /// @ I really love my comment design.
    /// SECRET||YOUR_SECRET
    /// ```
    /// 
    /// ```
    /// // main.rs 
    /// use env_plus::EnvLoader;
    /// 
    /// fn main() {
    ///     EnvLoader::new()
    ///     .change_delimiter(String::from("||"))
    ///     .change_comment(String::from("@"))
    ///     .change_file(String::from("./special.env"))
    ///     .activate();
    /// 
    ///     let secret = std::env::var("SECRET").unwrap();
    ///     assert_eq!(secret, String::from("YOUR_SECRET"));
    /// }
    /// ```
    pub fn activate(self) {
        let files_loaded = load_file(self);

        if !files_loaded {
            eprintln!("An error has occured")
        }
    }
}


fn load_file(envs: EnvLoader) -> bool {
    let file = fs::read_to_string(envs.file); // Add a correct error handling

    if file.is_err() {
        println!("{:?}", file.err());
        false

    } else {
        let unwraped_file = file.unwrap();
        let file_lines = unwraped_file.lines();

        for (ind, line) in file_lines.enumerate() {
            load_line(line, &envs.comment, &envs.value_delimiter, &envs.overwrite, ind);
        };

        true
    }

}


fn load_line(line: &str, comment: &String, delimiter: &String, overwrite: &bool, ind: usize) {
    if line.trim().starts_with(comment) || line.trim() == "" { return };

    let split_line: Vec<&str> = line.split(comment).collect();
    let main_line = split_line[0];

    let key_value: Vec<&str> = main_line.splitn(2, delimiter).collect();
    if key_value.len() < 2 { panic!("Line {} with content '{}' does not appear to be formatted properly.", ind + 1, line) };

    let key = key_value[0];
    let value = key_value[1];


    let env_exists = std::env::var(key);

    if env_exists.is_err() {
        std::env::set_var(key, value)
    } else {
        if !overwrite { return }

        std::env::set_var(key, value)
    }
}
