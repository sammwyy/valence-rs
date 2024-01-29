use std::borrow::Cow;
use std::collections::BTreeSet;

use valence_ident::Ident;

use crate::{packet_id, Decode, Encode, Packet};

#[derive(Clone, Debug, Encode, Decode, Packet)]
#[packet(id = packet_id::CONFIGURATION_FEATURES_S2C)]
pub struct FeaturesS2c<'a> {
    pub features: Cow<'a, BTreeSet<Ident<String>>>,
}
