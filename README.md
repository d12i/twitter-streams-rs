# Twitter Stream

[![Build Status](https://github.com/tesaguri/twitter-stream-rs/workflows/CI/badge.svg)](https://github.com/tesaguri/twitter-stream-rs/actions)
[![Current Version](https://img.shields.io/crates/v/twitter-stream.svg)](https://crates.io/crates/twitter-stream)
[![Documentation](https://docs.rs/twitter-stream/badge.svg)](https://docs.rs/twitter-stream/)

A Rust library for listening on Twitter Streaming API.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
futures = "0.3"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
twitter-stream = "0.13"
```

Here is a basic example that prints public mentions to @Twitter in JSON format:

```rust
use futures::prelude::*;
use twitter_stream::{Token, TwitterStream};

#[tokio::main]
async fn main() {
    let token = Token::from_parts("consumer_key", "consumer_secret", "access_key", "access_secret");

    TwitterStream::track("@Twitter", &token)
        .try_flatten_stream()
        .try_for_each(|json| {
            println!("{}", json);
            future::ok(())
        })
        .await
        .unwrap();
}
```

## Alternatives

[`egg-mode`], a Twitter API client crate, implements a Streaming API client as well. The following table shows key differences between `twitter-stream` and `egg-mode`.

[`egg-mode`]: https://crates.io/crates/egg-mode

|                          | `twitter-stream`                                 | `egg-mode`                             |
| ------------------------ | ------------------------------------------------ | -------------------------------------- |
| Streaming message type   | `string::String<bytes::Bytes>` (raw JSON string) | `StreamMessage` (deserialized message) |
| REST API integration     | No                                               | Yes                                    |
| Customizable HTTP client | Yes                                              | No                                     |

If your application don't require explicit control over the raw JSON strings or underlying HTTP client, `egg-mode` may be a better choice.

## License

This project is licensed under the MIT license ([LICENSE](LICENSE) or https://opensource.org/licenses/MIT) unless explicitly stated otherwise.
