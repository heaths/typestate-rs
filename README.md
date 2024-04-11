# Typestate Builder Example

This is an example of the typestate builder pattern we could use for client construction in Azure SDKs for Rust.
You can open [examples/typestate_builder.rs](examples/typestate_builder.rs) in GitHub Codespaces
to familiarize yourself with the developer experience.

This pattern guides the developer into only valid variants. In this example, they must first specify either an endpoint or connection string,
then only if they specify an endpoint can they specify either a `TokenCredential` or SAS token. After either of those branches, they can
specify other options or call `build()` to construct the `BlobClient`.

This example uses a builder that consumes `self` such that `--release` code will either move memory or even elide the move.
Conversely, we could take a `&mut self` and mutate the builder, returning a copy only from `build()` to support constructing separate clients
from the same - or modified between calls to `build()` - builder.

The same pattern could be used for client methods as well, though a simple builder - one not using type parameters - should suffice.

## Example

The following example shows how to construct a client using a builder. After typing `Client::builder().`, you'll only see methods
from a few blanket implementations and connection options. Depending on the connection option you specified e.g., the endpoint,
you'll see authentication options. Only when all required options are populated will you see general options such as setting retry options.

```rust
use azure_core::{Result, RetryOptions};
use azure_identity::DefaultAzureCredential;
use azure_typestate_example::BlobClient;
use std::sync::Arc;

fn new_blob_client(endpoint: &str) -> Result<BlobClient> {
    Ok(BlobClient::builder()
        .with_endpoint(endpoint)?
        .with_credential(Arc::new(DefaultAzureCredential::default()))
        .with_retry(RetryOptions::none())
        .build())
}
```

## Documentation

The type parameters are not included in [documentation](https://heaths.dev/typestate-rs), such that developers would see all
client builder methods documented together.

## License

Licensed under the [MIT](LICENSE.txt) license.
