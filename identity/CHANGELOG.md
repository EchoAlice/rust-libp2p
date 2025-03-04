## 0.2.0 - unreleased

- Raise MSRV to 1.65.
  See [PR 3715].
- Add support for exporting and importing ECDSA keys via the libp2p [protobuf format].
  See [PR 3863].

[PR 3715]: https://github.com/libp2p/rust-libp2p/pull/3715
[PR 3863]: https://github.com/libp2p/rust-libp2p/pull/3863
[protobuf format]: https://github.com/libp2p/specs/blob/master/peer-ids/peer-ids.md#keys

## 0.1.2

- Add `impl From<ed25519::PublicKey> for PublicKey` so that `PublicKey::from(ed25519::PublicKey)` works.
  See [PR 3805].

[PR 3805]: https://github.com/libp2p/rust-libp2p/pull/3805

- Follow Rust naming conventions for conversion methods.
  See [PR 3775].

[PR 3775]: https://github.com/libp2p/rust-libp2p/pull/3775

## 0.1.1

- Add `From` impl for specific keypairs.
  See [PR 3626].

[PR 3626]: https://github.com/libp2p/rust-libp2p/pull/3626
