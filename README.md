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

## Documentation

The type parameters are not included in [documentation](https://heaths.dev/typestate-rs), such that developers would see all
client builder methods documented together.

## License

Licensed under the [MIT](LICENSE.txt) license.
