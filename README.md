# env_plus <br /> 
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/Jint3x/env_plus)
![Crates.io](https://img.shields.io/crates/v/env_loader)

### A very simple and highly costumizeable env variable loader. You can specify your own files that you want to use or use its default settings.
## <br />



# Usage

Add this to your Cargo.toml:

```toml
[dependenices]
env_plus = "0.1.0"
```

### .env_plus
```
// This is a comment!
SECRET=YOUR_SECRET
```

### main<nolink>.rs
```rust
use env_plus::EnvLoader;

fn main() {
    EnvLoader::new()
    activate();

    let secret = std::env::var("SECRET").unwrap();
    assert_eq!(secret, String::from("YOUR_SECRET"))
}
```

### The default settings for env_plus are:
* File: .env_plus
* Comments: //
* Value delimiter: =
* Overwrite existing variables with the same name: false

### However, you are allowed to fully costumize which files to use and how to parse them.

## <br />

# Advanced Usage

### special_file.extension
```
## I want to use this style as a comment!
## == will be the new value delimiter

SECRET==YOUR_SECRET
```

### main<nolink>.rs
```rust
use env_plus::EnvLoader;

fn main() {
    std::env::set_var("SECRET", "MY_SECRET");

    EnvLoader::new()
    .change_file(String::from("./special_file.extension"))
    .change_delimiter(String::from("=="))
    .change_comment(String::from("##"))
    .overwrite_envs(true)
    .activate();


    let secret = std::env::var("SECRET").unwrap();

    // SECRET has been overwritten from MY_SECRET to YOUR_SECRET
    assert_eq!(secret, String::from("YOUR_SECRET"))
}
```