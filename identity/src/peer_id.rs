// Copyright 2018 Parity Technologies (UK) Ltd.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

use multiaddr::{Multiaddr, Protocol};
use multihash::{Code, Error, MultihashGeneric};
use rand::Rng;
use sha2::Digest as _;
use std::{convert::TryFrom, fmt, str::FromStr};
use thiserror::Error;

/// Local type-alias for multihash.
///
/// Must be big enough to accommodate for `MAX_INLINE_KEY_LENGTH`.
/// 64 satisfies that and can hold 512 bit hashes which is what the ecosystem typically uses.
/// Given that this appears in our type-signature, using a "common" number here makes us more compatible.
type Multihash = MultihashGeneric<64>;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Public keys with byte-lengths smaller than `MAX_INLINE_KEY_LENGTH` will be
/// automatically used as the peer id using an identity multihash.
const MAX_INLINE_KEY_LENGTH: usize = 42;

const MULTIHASH_IDENTITY_CODE: u64 = 0;
const MULTIHASH_SHA256_CODE: u64 = 0x12;

/// Identifier of a peer of the network.
///
/// The data is a CIDv0 compatible multihash of the protobuf encoded public key of the peer
/// as specified in [specs/peer-ids](https://github.com/libp2p/specs/blob/master/peer-ids/peer-ids.md).
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PeerId {
    multihash: Multihash,
}

impl fmt::Debug for PeerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("PeerId").field(&self.to_base58()).finish()
    }
}

impl fmt::Display for PeerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_base58().fmt(f)
    }
}

impl PeerId {
    /// Builds a `PeerId` from a public key.
    pub fn from_public_key(key: &crate::keypair::PublicKey) -> PeerId {
        let key_enc = key.encode_protobuf();

        let multihash = if key_enc.len() <= MAX_INLINE_KEY_LENGTH {
            Multihash::wrap(MULTIHASH_IDENTITY_CODE, &key_enc)
                .expect("64 byte multihash provides sufficient space")
        } else {
            Multihash::wrap(MULTIHASH_SHA256_CODE, &sha2::Sha256::digest(key_enc))
                .expect("64 byte multihash provides sufficient space")
        };

        PeerId { multihash }
    }

    /// Parses a `PeerId` from bytes.
    pub fn from_bytes(data: &[u8]) -> Result<PeerId, Error> {
        PeerId::from_multihash(Multihash::from_bytes(data)?)
            .map_err(|mh| Error::UnsupportedCode(mh.code()))
    }

    /// Tries to turn a `Multihash` into a `PeerId`.
    ///
    /// If the multihash does not use a valid hashing algorithm for peer IDs,
    /// or the hash value does not satisfy the constraints for a hashed
    /// peer ID, it is returned as an `Err`.
    pub fn from_multihash(multihash: Multihash) -> Result<PeerId, Multihash> {
        match multihash.code() {
            MULTIHASH_SHA256_CODE => Ok(PeerId { multihash }),
            MULTIHASH_IDENTITY_CODE if multihash.digest().len() <= MAX_INLINE_KEY_LENGTH => {
                Ok(PeerId { multihash })
            }
            _ => Err(multihash),
        }
    }

    /// Tries to extract a [`PeerId`] from the given [`Multiaddr`].
    ///
    /// In case the given [`Multiaddr`] ends with `/p2p/<peer-id>`, this function
    /// will return the encapsulated [`PeerId`], otherwise it will return `None`.
    pub fn try_from_multiaddr(address: &Multiaddr) -> Option<PeerId> {
        address.iter().last().and_then(|p| match p {
            Protocol::P2p(hash) => PeerId::from_multihash(hash).ok(),
            _ => None,
        })
    }

    /// Generates a random peer ID from a cryptographically secure PRNG.
    ///
    /// This is useful for randomly walking on a DHT, or for testing purposes.
    pub fn random() -> PeerId {
        let peer_id = rand::thread_rng().gen::<[u8; 32]>();
        PeerId {
            multihash: Multihash::wrap(Code::Identity.into(), &peer_id)
                .expect("The digest size is never too large"),
        }
    }

    /// Returns a raw bytes representation of this `PeerId`.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.multihash.to_bytes()
    }

    /// Returns a base-58 encoded string of this `PeerId`.
    pub fn to_base58(&self) -> String {
        bs58::encode(self.to_bytes()).into_string()
    }

    /// Checks whether the public key passed as parameter matches the public key of this `PeerId`.
    ///
    /// Returns `None` if this `PeerId`s hash algorithm is not supported when encoding the
    /// given public key, otherwise `Some` boolean as the result of an equality check.
    pub fn is_public_key(&self, public_key: &crate::PublicKey) -> Option<bool> {
        use multihash::MultihashDigest as _;

        let alg = Code::try_from(self.multihash.code())
            .expect("Internal multihash is always a valid `Code`");
        let enc = public_key.encode_protobuf();
        Some(alg.digest(&enc) == self.multihash)
    }
}

impl From<crate::PublicKey> for PeerId {
    fn from(key: crate::PublicKey) -> PeerId {
        PeerId::from_public_key(&key)
    }
}

impl From<&crate::PublicKey> for PeerId {
    fn from(key: &crate::PublicKey) -> PeerId {
        PeerId::from_public_key(key)
    }
}

impl TryFrom<Vec<u8>> for PeerId {
    type Error = Vec<u8>;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        PeerId::from_bytes(&value).map_err(|_| value)
    }
}

impl TryFrom<Multihash> for PeerId {
    type Error = Multihash;

    fn try_from(value: Multihash) -> Result<Self, Self::Error> {
        PeerId::from_multihash(value)
    }
}

impl AsRef<Multihash> for PeerId {
    fn as_ref(&self) -> &Multihash {
        &self.multihash
    }
}

impl From<PeerId> for Multihash {
    fn from(peer_id: PeerId) -> Self {
        peer_id.multihash
    }
}

impl From<PeerId> for Vec<u8> {
    fn from(peer_id: PeerId) -> Self {
        peer_id.to_bytes()
    }
}

#[cfg(feature = "serde")]
impl Serialize for PeerId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if serializer.is_human_readable() {
            serializer.serialize_str(&self.to_base58())
        } else {
            serializer.serialize_bytes(&self.to_bytes()[..])
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for PeerId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::*;

        struct PeerIdVisitor;

        impl<'de> Visitor<'de> for PeerIdVisitor {
            type Value = PeerId;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "valid peer id")
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: Error,
            {
                PeerId::from_bytes(v).map_err(|_| Error::invalid_value(Unexpected::Bytes(v), &self))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                PeerId::from_str(v).map_err(|_| Error::invalid_value(Unexpected::Str(v), &self))
            }
        }

        if deserializer.is_human_readable() {
            deserializer.deserialize_str(PeerIdVisitor)
        } else {
            deserializer.deserialize_bytes(PeerIdVisitor)
        }
    }
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("base-58 decode error: {0}")]
    B58(#[from] bs58::decode::Error),
    #[error("decoding multihash failed")]
    MultiHash,
}

impl FromStr for PeerId {
    type Err = ParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = bs58::decode(s).into_vec()?;
        PeerId::from_bytes(&bytes).map_err(|_| ParseError::MultiHash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "ed25519")]
    fn peer_id_is_public_key() {
        let key = crate::Keypair::generate_ed25519().public();
        let peer_id = key.to_peer_id();
        assert_eq!(peer_id.is_public_key(&key), Some(true));
    }

    #[test]
    #[cfg(feature = "ed25519")]
    fn peer_id_into_bytes_then_from_bytes() {
        let peer_id = crate::Keypair::generate_ed25519().public().to_peer_id();
        let second = PeerId::from_bytes(&peer_id.to_bytes()).unwrap();
        assert_eq!(peer_id, second);
    }

    #[test]
    #[cfg(feature = "ed25519")]
    fn peer_id_to_base58_then_back() {
        let peer_id = crate::Keypair::generate_ed25519().public().to_peer_id();
        let second: PeerId = peer_id.to_base58().parse().unwrap();
        assert_eq!(peer_id, second);
    }

    #[test]
    fn random_peer_id_is_valid() {
        for _ in 0..5000 {
            let peer_id = PeerId::random();
            assert_eq!(peer_id, PeerId::from_bytes(&peer_id.to_bytes()).unwrap());
        }
    }

    #[test]
    fn extract_peer_id_from_multi_address() {
        let address = "/memory/1234/p2p/12D3KooWGQmdpzHXCqLno4mMxWXKNFQHASBeF99gTm2JR8Vu5Bdc"
            .to_string()
            .parse()
            .unwrap();

        #[allow(deprecated)]
        let peer_id = PeerId::try_from_multiaddr(&address).unwrap();

        assert_eq!(
            peer_id,
            "12D3KooWGQmdpzHXCqLno4mMxWXKNFQHASBeF99gTm2JR8Vu5Bdc"
                .parse()
                .unwrap()
        );
    }

    #[test]
    fn no_panic_on_extract_peer_id_from_multi_address_if_not_present() {
        let address = "/memory/1234".to_string().parse().unwrap();

        #[allow(deprecated)]
        let maybe_empty = PeerId::try_from_multiaddr(&address);

        assert!(maybe_empty.is_none());
    }
}
