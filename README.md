## Falocon

A OpenAPIv3 Rust HTTP Client generator.    
This generator **wont** give you a client, but a trait that you can implement to create your own client.

### Target Code Example

```bash 
├── src
│   ├── methods
│   │   ├── mod.rs
│   │   ├── method1.rs
│   │   ├── method2.rs
│   │   └── ...
│   ├── client.rs
│   ├── error.rs
│   ├── lib.rs
│   ├── request.rs
│   ├── route.rs
│   └── types.rs
├── rustfmt.toml
└── Cargo.toml
```

<details>
  <summary>method1.rs (example)</summary>

```rust
use std::borrow::Cow;

use reqwest::Method;

pub trait Request: Sized + Send + 'static {
    type Response: serde::de::DeserializeOwned + Send + 'static;

    type Body: serde::ser::Serialize + Send + 'static;

    type Params: serde::ser::Serialize + Send + 'static;

    const METHOD: Method;

    const PATH: &'static str;

    fn body(&self) -> Option<&Self::Body> {

        None
    }

    fn params(&self) -> Option<&Self::Params> {

        None
    }

    fn path(&self) -> Cow<'static, str> {

        Cow::Borrowed(Self::PATH)
    }
}
```
</details>

<details>
  <summary>request.rs</summary>

```rust
use std::borrow::Cow;

use reqwest::Method;

pub trait Request: Sized + Send + 'static {
    type Response: serde::de::DeserializeOwned + Send + 'static;

    type Body: serde::ser::Serialize + Send + 'static;

    type Params: serde::ser::Serialize + Send + 'static;

    const METHOD: Method;

    const PATH: &'static str;

    fn body(&self) -> Option<&Self::Body> {

        None
    }

    fn params(&self) -> Option<&Self::Params> {

        None
    }

    fn path(&self) -> Cow<'static, str> {

        Cow::Borrowed(Self::PATH)
    }
}
```
</details>

<details>
  <summary>route.rs</summary>

```rust
use futures::{FutureExt, future::BoxFuture};

use super::*;

pub struct Route<C, K> {
    client: C,
    kind: K,
}

impl<C, K> Route<C, K>
where
    C: ClientPrelude,
{
    pub fn new(client: &C, kind: K) -> Self {

        Self {
            client: client.clone(),
            kind,
        }
    }
}

impl<C, Re: Request> std::fmt::Display for Route<C, Re> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        fmt.write_str(&self.kind.path())
    }
}

impl<C, Re> IntoFuture for Route<C, Re>
where
    C: ClientPrelude,
    Re: Request,
{
    type Output = Result<Re::Response>;

    type IntoFuture = BoxFuture<'static, Self::Output>;

    fn into_future(self) -> Self::IntoFuture {

        async move {

            let mut request = self
                .client
                .client()
                .request(Re::METHOD, format!("{}{}", C::BASE_URI, self.kind.path()));

            if let Some(headers) = self.client.headers() {

                request = request.headers(headers)
            }

            if let Some(body) = self.kind.body() {

                request = request.json(&body);
            }

            if let Some(params) = self.kind.params() {

                request = request.query(&params);
            }

            let response = request.send().await.map_err(Error::HttpError)?;

            let data = response
                .json::<Re::Response>()
                .await
                .map_err(Error::HttpError)?;

            Ok(data)
        }
        .boxed()
    }
}

```
</details>

### Usage
```bash
Usage: falocon <JSON_PATH> [OUTPUT_DIR]

Arguments:
  <JSON_PATH>   
  [OUTPUT_DIR]  

Options:
  -h, --help     Print help
  -V, --version  Print version
```
