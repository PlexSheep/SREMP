use std::sync::LazyLock;

use snow::{TransportState, params::NoiseParams};
use tokio::{io::AsyncWriteExt, net};

use crate::{
    current_function,
    error::{CoreError, CoreResult},
    identity::{Identity, UserIdentity},
};

mod frame;
use frame::*;

pub static NOISE_PARAMS: LazyLock<NoiseParams> = LazyLock::new(|| {
    "Noise_XX_25519_ChaChaPoly_BLAKE2s"
        .parse()
        .expect("noise parameter string is malformed")
});

#[derive(Debug)]
#[must_use]
pub enum Connection {
    P2P(P2PConnection),
}

macro_rules! delegate {
    ($self:tt, $($do:tt)+) => {
        match $self {
            Self::P2P(c) => c.$($do)+,
        }
    };
}

#[derive(Debug)]
#[must_use]
pub struct P2PConnection {
    stream: net::TcpStream,
    peer_identity: Identity,
    transport: snow::TransportState,
}

impl Connection {
    pub(crate) async fn connect_to(
        remote: std::net::SocketAddr,
        user: &UserIdentity,
    ) -> CoreResult<Self> {
        log::trace!("{}", current_function!());
        Ok(Self::P2P(P2PConnection::connect_to(remote, user).await?))
    }

    pub(crate) async fn connect_from(
        stream: net::TcpStream,
        remote: std::net::SocketAddr,
        user: &UserIdentity,
    ) -> CoreResult<Self> {
        Ok(Self::P2P(
            P2PConnection::connect_from(stream, remote, user).await?,
        ))
    }

    pub(crate) async fn disconnect(self) -> CoreResult<()> {
        delegate!(self, disconnect().await)
    }

    pub(crate) async fn peer_identity(&self) -> &Identity {
        delegate!(self, peer_identity().await)
    }
}

impl P2PConnection {
    async fn connect_to(remote: std::net::SocketAddr, user: &UserIdentity) -> CoreResult<Self> {
        log::trace!("{}", current_function!());
        let mut tcp_stream = net::TcpStream::connect(remote).await?;
        log::debug!("Tcp Connection Established");
        let (peer_identity, transport) = Self::dead_switch(&mut tcp_stream, async |tcp_stream| {
            let mut noise = Self::noise_initiator(user)?;
            let mut buf = [0u8; MAX_FRAME_SIZE];
            let mut len;

            log::debug!("Beginning noise handshake as initiator");

            log::debug!("Sending Noise: `XX: --> e`");
            len = noise.write_message(&[], &mut buf)?;
            Frame::from_payload(&buf[..len])?.send(tcp_stream).await?;

            log::debug!("Receiving: `XX: <-- e, ee, s, es`");
            let frame = Frame::recv(tcp_stream).await?;
            _ = noise.read_message(frame.data(), &mut buf)?;

            log::debug!("Sending Noise: `XX: --> s, se`");
            len = noise.write_message(&[], &mut buf)?;
            Frame::from_payload(&buf[..len])?.send(tcp_stream).await?;

            log::debug!("Finished noise handshake");

            Self::post_handshake(&mut buf, tcp_stream, user, noise, remote).await
        })
        .await?;

        Ok(Self {
            stream: tcp_stream,
            peer_identity,
            transport,
        })
    }

    async fn connect_from(
        mut tcp_stream: net::TcpStream,
        remote: std::net::SocketAddr,
        user: &UserIdentity,
    ) -> CoreResult<Self> {
        log::trace!("{}", current_function!());
        let (peer_identity, transport) = Self::dead_switch(&mut tcp_stream, async |tcp_stream| {
            let mut noise = Self::noise_responder(user)?;
            let mut buf = [0u8; MAX_FRAME_SIZE];
            let mut frame;

            log::debug!("Beginning noise handshake as responder");

            log::debug!("Receiving: `XX: --> e`");
            frame = Frame::recv(tcp_stream).await?;
            _ = noise.read_message(frame.data(), &mut buf)?;

            log::debug!("Sending Noise: `XX: <-- e, ee, s, es`");
            let len = noise.write_message(&[], &mut buf)?;
            Frame::from_payload(&buf[..len])?.send(tcp_stream).await?;

            log::debug!("Receiving: `XX: --> s, se`");
            frame = Frame::recv(tcp_stream).await?;
            _ = noise.read_message(frame.data(), &mut buf)?;

            log::debug!("Finished noise handshake");

            Self::post_handshake(&mut buf, tcp_stream, user, noise, remote).await
        })
        .await?;

        Ok(Self {
            stream: tcp_stream,
            peer_identity,
            transport,
        })
    }

    async fn post_handshake(
        buf: &mut [u8; MAX_FRAME_SIZE],
        stream: &mut net::TcpStream,
        user: &UserIdentity,
        noise: snow::HandshakeState,
        remote: std::net::SocketAddr,
    ) -> CoreResult<(Identity, TransportState)> {
        // SREMP uses the identity keys as the noise static key.
        let remote_static_key = noise
            .get_remote_static()
            .ok_or(CoreError::NoisePeerHasNoPublicKey(remote))?;

        let peer_key_bytes: &[u8; ed25519_dalek::PUBLIC_KEY_LENGTH] = remote_static_key
            .try_into()
            .map_err(|_| CoreError::PeerKeyIsMalformed(remote))?;

        let peer_public_key = ed25519_dalek::VerifyingKey::from_bytes(peer_key_bytes)
            .map_err(|e| CoreError::PeerKeyIsInvalid { remote, source: e })?;

        let mut transport = noise.into_transport_mode()?;

        // NOTE: both send before receiving, then listen for the incoming identity response
        // That way, the identity exchange is simultaneous and we dont need to program an order of
        // who sends first

        log::debug!("Sending identity to peer");
        let mut len = transport.write_message(&rmp_serde::to_vec(&user.identity)?, buf)?;
        Frame::from_payload(&buf[..len])?.send(stream).await?;

        log::debug!("Receiving identity from peer");
        let frame = Frame::recv(stream).await?;
        len = transport.read_message(frame.data(), buf)?;
        let peer_identity: Identity = rmp_serde::from_slice(&buf[..len])?;
        log::debug!("Received (unverified) Identity: {peer_identity:#?}");

        // FIXME: username might be a super long string, we should add some validator for the
        // username.

        if peer_identity.public_key != peer_public_key {
            log::error!("identity key does not match noise static public key:");
            return Err(CoreError::PeerKeyIsInvalid {
                remote,
                source: ed25519_dalek::SignatureError::new(),
            });
        }

        log::debug!("Noise Handshake and identity exchange with peer {remote} successful");

        Ok((peer_identity, transport))
    }

    async fn disconnect(mut self) -> CoreResult<()> {
        self.stream.shutdown().await?;
        Ok(())
    }

    async fn peer_identity(&self) -> &Identity {
        &self.peer_identity
    }

    /// Closes the [`net::TcpStream`] on error
    async fn dead_switch<T, F>(stream: &mut net::TcpStream, f: F) -> CoreResult<T>
    where
        F: AsyncFnOnce(&mut net::TcpStream) -> CoreResult<T>,
    {
        match f(stream).await {
            Ok(t) => Ok(t),
            Err(e) => {
                log::warn!("Error while handling a Connection, cutting the TcpStream: {e}");
                stream
                    .shutdown()
                    .await
                    .expect("could not shutdown the stream on error in dead switch");
                Err(e)
            }
        }
    }

    fn noise_builder<'a>(user: &'a UserIdentity) -> CoreResult<snow::Builder<'a>> {
        Ok(snow::Builder::new(NOISE_PARAMS.clone())
            .local_private_key(user.private_key().as_bytes())?)
    }

    fn noise_initiator(user: &UserIdentity) -> CoreResult<snow::HandshakeState> {
        Ok(Self::noise_builder(user)?.build_initiator()?)
    }

    fn noise_responder(user: &UserIdentity) -> CoreResult<snow::HandshakeState> {
        Ok(Self::noise_builder(user)?.build_responder()?)
    }
}
