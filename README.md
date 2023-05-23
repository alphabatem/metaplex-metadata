# Metaplex Token Metadata (Token2022 Fork)


_This is a fork of the Metaplex Program Library - Focused purely on the metadata contract. This contract provides support for new Token2022 SPL tokens and their associated 
Metadata requirements._



[![Program Tests](https://github.com/metaplex-foundation/metaplex-program-library/actions/workflows/program.yml/badge.svg)](https://github.com/metaplex-foundation/metaplex-program-library/actions/workflows/program.yml)
[![Integration Tests](https://github.com/metaplex-foundation/metaplex-program-library/actions/workflows/integration.yml/badge.svg)](https://github.com/metaplex-foundation/metaplex-program-library/actions/workflows/integration.yml)
[![SDK Tests](https://github.com/metaplex-foundation/metaplex-program-library/actions/workflows/sdk.yml/badge.svg)](https://github.com/metaplex-foundation/metaplex-program-library/actions/workflows/sdk.yml)

## Metaplex Contracts

| Name                                       | Rust Crate                                                                | npm Package                                                            |
|:-------------------------------------------|:--------------------------------------------------------------------------|------------------------------------------------------------------------|
| [Bubblegum](./bubblegum)                   | [![Crate][mpl-bubblegum-img-long]][mpl-bubblegum-crate]                   | [![NPM][mpl-bubblegum-nimg-long]][mpl-bubblegum-npm]                   |
| [Gumdrop](./gumdrop)                       | [![Crate][mpl-gumdrop-img-long]][mpl-gumdrop-crate]                       | [![NPM][mpl-gumdrop-nimg-long]][mpl-gumdrop-npm]                       |
| [Token Metadata](./token-metadata)         | [![Crate][mpl-token-metadata-img-long]][mpl-token-metadata-crate]         | [![NPM][mpl-token-metadata-nimg-long]][mpl-token-metadata-npm]         |

## Development

### Setting up Rust Tests

Run the `build.sh` script with the name of the program to build the shared object and put it in a directory
called `test-programs` in the root of the project.

E.g.:

```bash
./build.sh auction-house
```

Running with `all` builds all programs.

### Versioning and Publishing Packages

Smart contract SDK packages are versioned independently since a contract isn't necessarily coupled
to other contracts.

We use the following `(pre|post)(version|publish)` npm scripts to manage related checks, tagging,
committing and pushing the version bump.

- `preversion`: ensures that the package builds and its tests pass
- `postversion`: adds and commits the version bump and adds a tag indicating package name and new
  version, i.e. `@metaplex-foundation/mp-core@v0.0.1`
- `prepublishOnly`: ensures that the package builds and its tests pass again (just to be _really_ sure)
- `postpublish`: pushes the committed change and new tag to GitHub

In order to version and then publish a package just run the following commands from the folder of
the package you want to update:

- `npm version <patch|minor|major>`
- `npm publish`

As you note if version + publish succeeds the scripts end up pushing those updates to the master
branch. Therefore, please ensure to be on and up to date `master` branch before running them. Please
**don't ever publish from another branch** but only from the main one with only PR approved changes
merged.

### Rust Crates

| Package            | Link                                                   | Version                                                              |
|:-------------------|:-------------------------------------------------------|:---------------------------------------------------------------------|
| Bubblegum          | [mpl-bubblegum][mpl-bubblegum-crate]                   | [![Crate][mpl-bubblegum-img]][mpl-bubblegum-crate]                   |
| Testing Utils      | [mpl-testing-utils][mpl-testing-utils-crate]           | [![Crate][mpl-testing-utils-img]][mpl-testing-utils-crate]           |
| Utils              | [mpl-utils][mpl-utils-crate]                           | [![Crate][mpl-utils-img]][mpl-utils-crate]                           |
| Gumdrop            | [mpl-gumdrop][mpl-gumdrop-crate]                       | [![Crate][mpl-gumdrop-img]][mpl-gumdrop-crate]                       |
| Token Metadata     | [mpl-token-metadata][mpl-token-metadata-crate]         | [![Crate][mpl-token-metadata-img]][mpl-token-metadata-crate]         |

### npm Packages

| Package            | Link                                                 | Version                                                           |
|:-------------------|:-----------------------------------------------------|:------------------------------------------------------------------|
| Bubblegum          | [mpl-bubblegum][mpl-bubblegum-npm]                   | [![NPM][mpl-bubblegum-nimg]][mpl-bubblegum-npm]                   |
| Gumdrop            | [mpl-gumdrop][mpl-gumdrop-npm]                       | [![NPM][mpl-gumdrop-nimg]][mpl-gumdrop-npm]                       |
| Token Metadata     | [mpl-token-metadata][mpl-token-metadata-npm]         | [![NPM][mpl-token-metadata-nimg]][mpl-token-metadata-npm]         |

## Reporting security issues

To report a security issue, please follow the guidance on the [SECURITY](.github/SECURITY.md) page.

## License

The Rust/Cargo programs are licensed under the
“Apache-style” [Metaplex(TM) NFT Open Source License][metaplex-nft-license] and the JS/TS client libraries are licensed
under either the [MIT][mit-license] or the [Apache][apache-license] licenses.


<!-- ===================================== -->
<!-- Links for badges and such shown above -->
<!-- Please add any links you add to the   -->
<!-- readme here instead of inlining them  -->
<!-- ===================================== -->

<!-- Workflow Status Badges -->

[integration-tests-yml]:https://github.com/metaplex-foundation/metaplex-program-library/actions/workflows/integration.yml
[integration-tests-svg]:https://github.com/metaplex-foundation/metaplex-program-library/actions/workflows/integration.yml/badge.svg
[program-tests-yml]:https://github.com/metaplex-foundation/metaplex-program-library/actions/workflows/program.yml
[program-tests-svg]:https://github.com/metaplex-foundation/metaplex-program-library/actions/workflows/program.yml/badge.svg
[sdk-tests-yml]:https://github.com/metaplex-foundation/metaplex-program-library/actions/workflows/sdk.yml
[sdk-tests-svg]:https://github.com/metaplex-foundation/metaplex-program-library/actions/workflows/sdk.yml/badge.svg

<!-- Crates -->

[mpl-bubblegum-crate]:https://crates.io/crates/mpl-bubblegum
[mpl-gumdrop-crate]:https://crates.io/crates/mpl-gumdrop
[mpl-token-metadata-crate]:https://crates.io/crates/mpl-token-metadata

[mpl-bubblegum-img-long]:https://img.shields.io/crates/v/mpl-bubblegum?label=crates.io%20%7C%20mpl-bubblegum&logo=rust
[mpl-bubblegum-img]:https://img.shields.io/crates/v/mpl-bubblegum?logo=rust

[mpl-gumdrop-img-long]:https://img.shields.io/crates/v/mpl-gumdrop?label=crates.io%20%7C%20mpl-gumdrop&logo=rust
[mpl-gumdrop-img]:https://img.shields.io/crates/v/mpl-gumdrop?logo=rust

[mpl-token-metadata-img-long]:https://img.shields.io/crates/v/mpl-token-metadata?label=crates.io%20%7C%20mpl-token-metadata&logo=rust
[mpl-token-metadata-img]:https://img.shields.io/crates/v/mpl-token-metadata?logo=rust

<!-- NPM Packages -->

[mpl-bubblegum-npm]:https://www.npmjs.com/package/@metaplex-foundation/mpl-bubblegum
[mpl-core-npm]:https://www.npmjs.com/package/@metaplex-foundation/mpl-core
[mpl-gumdrop-npm]:https://www.npmjs.com/package/@metaplex-foundation/mpl-gumdrop
[mpl-token-metadata-npm]:https://www.npmjs.com/package/@metaplex-foundation/mpl-token-metadata

[mpl-bubblegum-nimg-long]:https://img.shields.io/npm/v/@metaplex-foundation/mpl-bubblegum?label=npm%20%7C%20mpl-bubblegum&logo=typescript
[mpl-bubblegum-nimg]:https://img.shields.io/npm/v/@metaplex-foundation/mpl-bubblegum?logo=typescript

[mpl-gumdrop-nimg-long]:https://img.shields.io/npm/v/@metaplex-foundation/mpl-gumdrop?label=npm%20%7C%20mpl-gumdrop&logo=typescript
[mpl-gumdrop-nimg]:https://img.shields.io/npm/v/@metaplex-foundation/mpl-gumdrop?logo=typescript

[mpl-token-metadata-nimg-long]:https://img.shields.io/npm/v/@metaplex-foundation/mpl-token-metadata?label=npm%20%7C%20mpl-token-metadata&logo=typescript
[mpl-token-metadata-nimg]:https://img.shields.io/npm/v/@metaplex-foundation/mpl-token-metadata?logo=typescript

<!-- Licenses -->

[metaplex-nft-license]:  https://github.com/metaplex-foundation/metaplex-program-library/blob/master/LICENSE

[apache-license]: https://www.apache.org/licenses/LICENSE-2.0.txt

[mit-license]: https://www.mit.edu/~amini/LICENSE.md
