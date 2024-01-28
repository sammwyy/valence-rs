use crate::{packet_id, Decode, Encode, Packet};

#[derive(Copy, Clone, Debug, Encode, Decode, Packet)]
#[packet(id = packet_id::PLAY_BUNDLE_SPLITTER)]
pub struct BundleSplitterS2c;
