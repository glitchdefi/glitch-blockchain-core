// This file is part of Substrate.

// Copyright (C) 2019-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Network event types. These are are not the part of the protocol, but rather
//! events that happen on the network like DHT get/put results received.

use bytes::Bytes;
use libp2p::core::PeerId;
use libp2p::kad::record::Key;
use std::borrow::Cow;

/// Events generated by DHT as a response to get_value and put_value requests.
#[derive(Debug, Clone)]
#[must_use]
pub enum DhtEvent {
	/// The value was found.
	ValueFound(Vec<(Key, Vec<u8>)>),

	/// The requested record has not been found in the DHT.
	ValueNotFound(Key),

	/// The record has been successfully inserted into the DHT.
	ValuePut(Key),

	/// An error has occurred while putting a record into the DHT.
	ValuePutFailed(Key),
}

/// Type for events generated by networking layer.
#[derive(Debug, Clone)]
#[must_use]
pub enum Event {
	/// Event generated by a DHT.
	Dht(DhtEvent),

	/// Now connected to a new peer for syncing purposes.
	SyncConnected {
		/// Node we are now syncing from.
		remote: PeerId,
	},

	/// Now disconnected from a peer for syncing purposes.
	SyncDisconnected {
		/// Node we are no longer syncing from.
		remote: PeerId,
	},

	/// Opened a substream with the given node with the given notifications protocol.
	///
	/// The protocol is always one of the notification protocols that have been registered.
	NotificationStreamOpened {
		/// Node we opened the substream with.
		remote: PeerId,
		/// The concerned protocol. Each protocol uses a different substream.
		protocol: Cow<'static, str>,
		/// Role of the remote.
		role: ObservedRole,
	},

	/// Closed a substream with the given node. Always matches a corresponding previous
	/// `NotificationStreamOpened` message.
	NotificationStreamClosed {
		/// Node we closed the substream with.
		remote: PeerId,
		/// The concerned protocol. Each protocol uses a different substream.
		protocol: Cow<'static, str>,
	},

	/// Received one or more messages from the given node using the given protocol.
	NotificationsReceived {
		/// Node we received the message from.
		remote: PeerId,
		/// Concerned protocol and associated message.
		messages: Vec<(Cow<'static, str>, Bytes)>,
	},
}

/// Role that the peer sent to us during the handshake, with the addition of what our local node
/// knows about that peer.
#[derive(Debug, Clone)]
pub enum ObservedRole {
	/// Full node.
	Full,
	/// Light node.
	Light,
	/// When we are a validator node, this is a sentry that protects us.
	OurSentry,
	/// When we are a sentry node, this is the authority we are protecting.
	OurGuardedAuthority,
	/// Third-party authority.
	Authority,
}

impl ObservedRole {
	/// Returns `true` for `ObservedRole::Light`.
	pub fn is_light(&self) -> bool {
		matches!(self, ObservedRole::Light)
	}
}
