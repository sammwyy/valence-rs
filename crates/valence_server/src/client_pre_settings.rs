use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use valence_entity::player::{self, PlayerModelParts};
use valence_protocol::packets::configuration::client_options_c2s::ChatMode;
use valence_protocol::packets::configuration::ClientOptionsC2s;

use crate::client::ViewDistance;
use crate::event_loop::{EventLoopPreUpdate, PacketConfigEvent, PacketEvent};

pub struct ClientPreSettingsPlugin;

impl Plugin for ClientPreSettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(EventLoopPreUpdate, handle_client_settings);
    }
}

/// Component containing client-controlled settings about a client.
#[derive(Component, Default, Debug)]
pub struct ClientSettings {
    pub locale: Box<str>,
    pub chat_mode: ChatMode,
    pub chat_colors: bool,
    pub enable_text_filtering: bool,
    pub allow_server_listings: bool,
}

fn handle_client_settings(
    mut packets: EventReader<PacketConfigEvent>,
    mut clients: Query<(
        &mut ViewDistance,
        &mut ClientSettings,
        &mut PlayerModelParts,
        &mut player::MainArm,
    )>,
) {
    for packet in packets.read() {
        if let Some(pkt) = packet.decode::<ClientOptionsC2s>() {
            println!("{}", pkt.locale);
            if let Ok((mut view_dist, mut settings, mut model_parts, mut main_arm)) =
                clients.get_mut(packet.client)
            {
                view_dist.set_if_neq(ViewDistance::new(pkt.view_distance));

                settings.locale = pkt.locale.into();
                settings.chat_mode = pkt.chat_mode;
                settings.chat_colors = pkt.chat_colors;
                settings.enable_text_filtering = pkt.enable_text_filtering;
                settings.allow_server_listings = pkt.allow_server_listings;

                model_parts.set_if_neq(PlayerModelParts(u8::from(pkt.displayed_skin_parts) as i8));
                main_arm.set_if_neq(player::MainArm(pkt.main_arm as i8));
            }
        }
    }
}
