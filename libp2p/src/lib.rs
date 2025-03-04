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

//! libp2p is a modular peer-to-peer networking framework.
//!
//! To learn more about the general libp2p multi-language framework visit
//! [libp2p.io](https://libp2p.io/).
//!
//! To get started with this libp2p implementation in Rust, please take a look
//! at the [`tutorials`](crate::tutorials). Further examples can be found in the
//! [examples] directory.
//!
//! [examples]: https://github.com/libp2p/rust-libp2p/tree/master/examples

#![doc(html_logo_url = "https://libp2p.io/img/logo_small.png")]
#![doc(html_favicon_url = "https://libp2p.io/img/favicon.png")]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

pub use bytes;
pub use futures;
#[doc(inline)]
pub use libp2p_core::multihash;
#[doc(inline)]
pub use multiaddr;

#[doc(inline)]
pub use libp2p_allow_block_list as allow_block_list;
#[cfg(feature = "autonat")]
#[doc(inline)]
pub use libp2p_autonat as autonat;
#[doc(inline)]
pub use libp2p_connection_limits as connection_limits;
#[doc(inline)]
pub use libp2p_core as core;
#[cfg(feature = "dcutr")]
#[doc(inline)]
pub use libp2p_dcutr as dcutr;
#[cfg(feature = "deflate")]
#[cfg(not(target_arch = "wasm32"))]
#[doc(inline)]
pub use libp2p_deflate as deflate;
#[cfg(feature = "dns")]
#[cfg_attr(docsrs, doc(cfg(feature = "dns")))]
#[cfg(not(target_arch = "wasm32"))]
#[doc(inline)]
pub use libp2p_dns as dns;
#[cfg(feature = "floodsub")]
#[doc(inline)]
pub use libp2p_floodsub as floodsub;
#[cfg(feature = "gossipsub")]
#[cfg(not(target_os = "unknown"))]
#[doc(inline)]
pub use libp2p_gossipsub as gossipsub;
#[cfg(feature = "identify")]
#[doc(inline)]
pub use libp2p_identify as identify;
#[cfg(feature = "kad")]
#[doc(inline)]
pub use libp2p_kad as kad;
#[cfg(feature = "mdns")]
#[cfg(not(target_arch = "wasm32"))]
#[cfg_attr(docsrs, doc(cfg(feature = "mdns")))]
#[doc(inline)]
pub use libp2p_mdns as mdns;
#[cfg(feature = "metrics")]
#[doc(inline)]
pub use libp2p_metrics as metrics;
#[cfg(feature = "mplex")]
#[deprecated(
    note = "`mplex` is not recommended anymore. Please use `yamux` instead or depend on `libp2p-mplex` directly if you need it for legacy use cases."
)]
pub mod mplex {
    pub use libp2p_mplex::*;
}
#[cfg(feature = "noise")]
#[doc(inline)]
pub use libp2p_noise as noise;
#[cfg(feature = "perf")]
#[cfg(not(target_arch = "wasm32"))]
#[cfg_attr(docsrs, doc(cfg(feature = "perf")))]
#[doc(inline)]
pub use libp2p_perf as perf;
#[cfg(feature = "ping")]
#[doc(inline)]
pub use libp2p_ping as ping;
#[cfg(feature = "plaintext")]
#[doc(inline)]
pub use libp2p_plaintext as plaintext;
#[cfg(feature = "pnet")]
#[doc(inline)]
pub use libp2p_pnet as pnet;
#[cfg(feature = "quic")]
#[cfg(not(target_arch = "wasm32"))]
#[deprecated(
    note = "`quic` is only in alpha status. Please depend on `libp2p-quic` directly and don't ues the `quic` feature of `libp2p`."
)]
pub mod quic {
    pub use libp2p_quic::*;
}
#[cfg(feature = "relay")]
#[doc(inline)]
pub use libp2p_relay as relay;
#[cfg(feature = "rendezvous")]
#[doc(inline)]
pub use libp2p_rendezvous as rendezvous;
#[cfg(feature = "request-response")]
#[doc(inline)]
pub use libp2p_request_response as request_response;
#[doc(inline)]
pub use libp2p_swarm as swarm;
#[cfg(feature = "tcp")]
#[cfg(not(target_arch = "wasm32"))]
#[cfg_attr(docsrs, doc(cfg(feature = "tcp")))]
#[doc(inline)]
pub use libp2p_tcp as tcp;
#[cfg(feature = "tls")]
#[cfg_attr(docsrs, doc(cfg(feature = "tls")))]
#[cfg(not(target_arch = "wasm32"))]
#[doc(inline)]
pub use libp2p_tls as tls;
#[cfg(feature = "uds")]
#[cfg_attr(docsrs, doc(cfg(feature = "uds")))]
#[cfg(not(target_arch = "wasm32"))]
#[doc(inline)]
pub use libp2p_uds as uds;
#[cfg(feature = "wasm-ext")]
#[doc(inline)]
pub use libp2p_wasm_ext as wasm_ext;
#[cfg(feature = "webrtc")]
#[cfg_attr(docsrs, doc(cfg(feature = "webrtc")))]
#[cfg(not(target_arch = "wasm32"))]
#[deprecated(
    note = "`webrtc` is only in alpha status. Please depend on `libp2p-webrtc` directly and don't ues the `webrtc` feature of `libp2p`."
)]
pub mod webrtc {
    pub use libp2p_webrtc::*;
}
#[cfg(feature = "websocket")]
#[cfg(not(target_arch = "wasm32"))]
#[doc(inline)]
pub use libp2p_websocket as websocket;
#[cfg(feature = "yamux")]
#[doc(inline)]
pub use libp2p_yamux as yamux;

mod transport_ext;

pub mod bandwidth;

#[cfg(doc)]
pub mod tutorials;

pub use self::core::{
    transport::TransportError,
    upgrade::{InboundUpgrade, OutboundUpgrade},
    Transport,
};
pub use self::multiaddr::{multiaddr as build_multiaddr, Multiaddr};
pub use self::swarm::Swarm;
pub use self::transport_ext::TransportExt;
pub use libp2p_identity as identity;
pub use libp2p_identity::PeerId;
pub use libp2p_swarm::StreamProtocol;

/// Builds a `Transport` based on TCP/IP that supports the most commonly-used features of libp2p:
///
///  * DNS resolution.
///  * Noise protocol encryption.
///  * Websockets.
///  * Both Yamux and Mplex for substream multiplexing.
///
/// All async I/O of the transport is based on `async-std`.
///
/// > **Note**: This `Transport` is not suitable for production usage, as its implementation
/// >           reserves the right to support additional protocols or remove deprecated protocols.
#[cfg(all(
    not(target_arch = "wasm32"),
    any(
        all(feature = "tcp-async-io", feature = "dns-async-std"),
        all(feature = "tcp", feature = "dns", feature = "async-std")
    ),
    feature = "websocket",
    feature = "noise",
    feature = "mplex",
    feature = "yamux"
))]
#[cfg_attr(
    all(
        any(feature = "tcp-async-io", feature = "dns-async-std"),
        not(feature = "async-std")
    ),
    deprecated(
        since = "0.49.0",
        note = "The `tcp-async-io` and `dns-async-std` features are deprecated. Use the new `tcp` and `dns` features together with the `async-std` feature."
    )
)]
pub async fn development_transport(
    keypair: identity::Keypair,
) -> std::io::Result<core::transport::Boxed<(PeerId, core::muxing::StreamMuxerBox)>> {
    let transport = {
        let dns_tcp = dns::DnsConfig::system(tcp::async_io::Transport::new(
            tcp::Config::new().nodelay(true),
        ))
        .await?;
        let ws_dns_tcp = websocket::WsConfig::new(
            dns::DnsConfig::system(tcp::async_io::Transport::new(
                tcp::Config::new().nodelay(true),
            ))
            .await?,
        );
        dns_tcp.or_transport(ws_dns_tcp)
    };

    Ok(transport
        .upgrade(core::upgrade::Version::V1)
        .authenticate(noise::Config::new(&keypair).unwrap())
        .multiplex(core::upgrade::SelectUpgrade::new(
            yamux::Config::default(),
            #[allow(deprecated)]
            mplex::MplexConfig::default(),
        ))
        .timeout(std::time::Duration::from_secs(20))
        .boxed())
}

/// Builds a `Transport` based on TCP/IP that supports the most commonly-used features of libp2p:
///
///  * DNS resolution.
///  * Noise protocol encryption.
///  * Websockets.
///  * Both Yamux and Mplex for substream multiplexing.
///
/// All async I/O of the transport is based on `tokio`.
///
/// > **Note**: This `Transport` is not suitable for production usage, as its implementation
/// >           reserves the right to support additional protocols or remove deprecated protocols.
#[cfg(all(
    not(target_arch = "wasm32"),
    any(
        all(feature = "tcp-tokio", feature = "dns-tokio"),
        all(feature = "tcp", feature = "dns", feature = "tokio")
    ),
    feature = "websocket",
    feature = "noise",
    feature = "mplex",
    feature = "yamux"
))]
#[cfg_attr(
    all(
        any(feature = "tcp-tokio", feature = "dns-tokio"),
        not(feature = "tokio")
    ),
    deprecated(
        since = "0.49.0",
        note = "The `tcp-tokio` and `dns-tokio` features are deprecated. Use the new `tcp` and `dns` feature together with the `tokio` feature."
    )
)]
pub fn tokio_development_transport(
    keypair: identity::Keypair,
) -> std::io::Result<core::transport::Boxed<(PeerId, core::muxing::StreamMuxerBox)>> {
    let transport = {
        let dns_tcp = dns::TokioDnsConfig::system(tcp::tokio::Transport::new(
            tcp::Config::new().nodelay(true),
        ))?;
        let ws_dns_tcp = websocket::WsConfig::new(dns::TokioDnsConfig::system(
            tcp::tokio::Transport::new(tcp::Config::new().nodelay(true)),
        )?);
        dns_tcp.or_transport(ws_dns_tcp)
    };

    Ok(transport
        .upgrade(core::upgrade::Version::V1)
        .authenticate(noise::Config::new(&keypair).unwrap())
        .multiplex(core::upgrade::SelectUpgrade::new(
            yamux::Config::default(),
            #[allow(deprecated)]
            mplex::MplexConfig::default(),
        ))
        .timeout(std::time::Duration::from_secs(20))
        .boxed())
}
