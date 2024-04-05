# Tinystamp

A tiny, zero-dependencies crate to format a timestamp (or now) as an ISO-8601 string (e.g. `2024-04-05T10:01:31Z`).

The main usage is in simple code where you need to log a timestamp, but don't need an entire date library, like chrono or time.

## Installation

Add this to your `Cargo.toml`:

```toml
tinystamp = "0.1.0"
```

## Usage 

The usage is very simple, use the `Datetime` struct to use a timestamp, or even easier use the current time. The you can format the timestamp as an ISO-8601 string using `format_iso8601`:

```rust
use tinystamp::Datetime;

fn main() {
    let datetime = Datetime::now();
    // or
    let datetime = Datetime::new(1712311291);
    let iso_string = datetime.format_iso8601(); // "2024-04-05T10:01:31Z"
}
```

The `Datetime` struct implements display, so it is even easier if you just want to print an ISO-8601 timestamp:


```rust
use tinystamp::Datetime;

fn main() {
    println!("{}: An event happened!", Datetime::now());
}
```

## Limitations

To keep this library minimal, there are a few limitations:

1. It is focused on the current date, so only usable between 2001-2099.
1. Only ISO-8601 without subseconds supported.
1. No timezone support, everything is in UTC.
1. No parsing as of now (maybe there will be ISO-8601 parsing).
