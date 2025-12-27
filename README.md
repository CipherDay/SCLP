<div align="center">
<h1>SCLP</h1>

---

<strong>📦 Simple Command Line Parser</strong>
</div>
<br />

**SCLP** is very simple lightweight rust command line parser with **Zero** external dependencies.  
SCLP was built as learning project for me to learn the programing language [Rust](https://rust-lang.org/).  
SCLP is inspired by the Golang [flag](https://pkg.go.dev/flag) package.

## Example:

```rust
use SCLP::*;

fn main() {
    let name = SCLP::strFlag::new("name")
        .setDefault("john".to_string())
        .setHelp("Your name.");
    let age = SCLP::intFlag::new("age")
        .setDefault(22)
        .setHelp("Your age.");

    // height here is required     
    let height = SCLP::floatFlag::new("h")
        .setHelp("Your height.");

    let happy = SCLP::boolFlag::new("happy")
        .setHelp("Are you happy.");

    println!(
        "name: {} age: {} height: {} happy: {}",
        name.parse(),
        age.parse(),
        height.parse(),
        happy.parse()
    );
}
```

```shell
./app --name user1 --h 1.82 --happy
```

## Usage:

### String argument:

To get a string type argument you will use `strFlag`.

#### Example:

```rust
let name = SCLP::strFlag::new("name").setDefault("john".to_string()).setHelp("Your name.");
```

```shell
./app --name user
```

This will return a `strFlag` object, and you can use one of two parsing options:  
Calling `.parse()`:

```rust
let nameValue = name.parse();
```

This will return `String`.  
Calling `.tryParse()`:

```rust
let nameValue = name.tryParse();
```

This will return `Result<String,ArgumentError>`.

### Integer argument:

To get an int type argument you will use `intFlag`.

#### Example:

```rust
let age = SCLP::intFlag::new("age").setDefault(22).setHelp("Your age.");
```

```shell
./app --age 31
```

This will return a `intFlag` object, and you can use one of two parsing options:  
Calling `.parse()`:

```rust
let ageValue = age.parse();
```

This will return `i32`.  
Calling `.tryParse()`:

```rust
let ageValue = age.tryParse();
```

This will return `Result<i32,ArgumentError>`.

### Float argument:

To get a float type argument you will use `floatFlag`.

#### Example:

```rust
let height = SCLP::floatFlag::new("height").setDefault(1.82).setHelp("Your height in meters.");
```

```shell
./app --height 1.73
```

This will return a `floatFlag` object, and you can use one of two parsing options:  
Calling `.parse()`:

```rust
let heightValue = height.parse();
```

This will return `f32`.  
Calling `.tryParse()`:

```rust
let heightValue = age.heightParse();
```

This will return `Result<f32,ArgumentError>`.

### Boolean argument:

To get a boolean type argument you will use `boolFlag`.

#### Example:

```rust
let happy = SCLP::boolFlag::new("height").setHelp("Are you happy.");
```

```shell
./app --happy # isHappy is true
./app # isHappy is false
```

This will return a `boolFlag` object, and you can use one of two parsing options:  
Calling `.parse()`:

```rust
let isHappy = happy.parse();
```

This will return `f32`.  
Calling `.tryParse()`:

```rust
let isHappy = happy.tryParse();
```

This will return `Result<bool,ArgumentError>`.

### `.parse()` VS `.tryParse()`:

`.parse()` returns the primitive types + `String` and will exit in the case of marking the argument as required (by not
setting default value with `.setDefault()`) and the user not providing the argument.  
`.tryParse()` returns a `Result<T,ArgumentError>` and let you decide what will happen in the case of:

#### `ArgumentError`:

`ArgumentError` is an `enum` contain the following values:

- `ARG_REQUIRED` will be returned when the value is required and the user didn't provide it.
- `ARG_PROCESSING` will be returned when you are trying to parse a `i32`or`f32` type of argument but the user provided a
  `String`.

## Flag format:

Every flag should start with `--`.Any flag not starting with `--` is ignored.

### Example:

```rust
use SCLP::*;

fn main() {
    let name = SCLP::strFlag::new("name")
        .setDefault("john".to_string())
        .setHelp("Your name.");
    println!("name: {}", name.parse());
}
```

And in the terminal:

```shell
./app --name user1
```

For `bool` type of argument:

```shell
./app --boolArg
```

will be returned as `true` otherwise `false` or default value.

## Help message:

The library creates a default help message that will be shown when the user uses `--help` flag.  
Currently there are no way to change the help message.

## Notice:

This library is for learning purposes only I am planning to make future updates for it as I get better at rust but
nothing to promis.



