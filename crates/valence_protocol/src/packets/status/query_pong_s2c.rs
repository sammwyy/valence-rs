use crate::{packet_id, Decode, Encode, Packet, PacketState};

#[derive(Copy, Clone, Debug, Encode, Decode, Packet)]
#[packet(state = PacketState::Status, id = packet_id::STATUS_PING_RESULT_S2C)]
pub struct QueryPongS2c {
    pub payload: u64,
}
