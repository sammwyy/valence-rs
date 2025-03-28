use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::quote;
use serde::Deserialize;
use valence_build_utils::write_generated_file;

#[derive(Deserialize)]
struct Packet {
    name: String,
    side: String,
    state: String,
    id: i32,
}

pub fn main() -> anyhow::Result<()> {
    let packets: Vec<Packet> = serde_json::from_str(include_str!("extracted/packets.json"))?;

    write_packets(&packets)?;
    write_transformer(&packets)?;

    Ok(())
}

fn write_packets(packets: &Vec<Packet>) -> anyhow::Result<()> {
    let mut consts = TokenStream::new();

    let len = packets.len();

    let mut p: Vec<TokenStream> = Vec::new();

    for packet in packets {
        let name = packet.name.strip_suffix("Packet").unwrap_or(&packet.name);
        // lowercase the last character of name
        let name = {
            let mut chars = name.chars();
            let last_char = chars.next_back().unwrap();
            let last_char = last_char.to_lowercase().to_string();
            let mut name = chars.collect::<String>();
            name.push_str(&last_char);
            name
        };

        // if the packet is clientbound, but the name does not ends with S2c, add it
        let name = if packet.side == "clientbound" && !name.ends_with("S2c") {
            format!("{}S2c", name)
        } else {
            name
        };

        // same for serverbound
        let name = if packet.side == "serverbound" && !name.ends_with("C2s") {
            format!("{}C2s", name)
        } else {
            name
        };

        let id = packet.id;
        let side = match packet.side.as_str() {
            "clientbound" => quote! { valence_protocol::PacketSide::Clientbound },
            "serverbound" => quote! { valence_protocol::PacketSide::Serverbound },
            _ => unreachable!(),
        };

        let state = match packet.state.as_str() {
            "handshaking" => quote! { valence_protocol::PacketState::Handshaking },
            "status" => quote! { valence_protocol::PacketState::Status },
            "configuration" => quote! { valence_protocol::PacketState::Configuration },
            "login" => quote! { valence_protocol::PacketState::Login },
            "play" => quote! { valence_protocol::PacketState::Play },
            _ => unreachable!(),
        };

        // const STD_PACKETS =
        // [PacketSide::Client(PacketState::Handshaking(Packet{..})), ..];
        p.push(quote! {
            crate::packet_registry::Packet {
                id: #id,
                side: #side,
                state: #state,
                timestamp: None,
                name: #name,
                data: None,
            }
        });
    }

    consts.extend([quote! {
        pub const STD_PACKETS: [crate::packet_registry::Packet; #len] = [
            #(#p),*
        ];
    }]);

    write_generated_file(consts, "packets.rs")?;

    Ok(())
}

fn write_transformer(packets: &[Packet]) -> anyhow::Result<()> {
    // HashMap<side, HashMap<state, Vec<name>>>
    let grouped_packets = HashMap::<String, HashMap<String, Vec<String>>>::new();

    let mut grouped_packets = packets.iter().fold(grouped_packets, |mut acc, packet| {
        let side = match packet.side.as_str() {
            "serverbound" => "Serverbound".to_string(),
            "clientbound" => "Clientbound".to_string(),
            _ => panic!("Invalid side"),
        };

        let state = match packet.state.as_str() {
            "handshaking" => "Handshaking".to_string(),
            "status" => "Status".to_string(),
            "login" => "Login".to_string(),
            "configuration" => "Configuration".to_string(),
            "play" => "Play".to_string(),
            _ => panic!("Invalid state"),
        };

        let name = packet
            .name
            .strip_suffix("Packet")
            .unwrap_or(&packet.name)
            .to_string();

        // lowercase the last character of name
        let name = {
            let mut chars = name.chars();
            let last_char = chars.next_back().unwrap();
            let last_char = last_char.to_lowercase().to_string();
            let mut name = chars.collect::<String>();
            name.push_str(&last_char);
            name
        };

        // if the packet is clientbound, but the name does not ends with S2c, add it
        let name = if side == "Clientbound" && !name.ends_with("S2c") {
            format!("{}S2c", name)
        } else {
            name
        };

        // same for serverbound
        let name = if side == "Serverbound" && !name.ends_with("C2s") {
            format!("{}C2s", name)
        } else {
            name
        };

        let state_map = acc.entry(side).or_default();
        let id_map = state_map.entry(state).or_default();
        id_map.push(name);

        acc
    });

    let mut generated = TokenStream::new();

    for (side, state_map) in grouped_packets.iter_mut() {
        let mut side_arms = TokenStream::new();
        for (state, id_map) in state_map.iter_mut() {
            let mut match_arms = TokenStream::new();

            for name in id_map.iter_mut() {
                let name = syn::parse_str::<syn::Ident>(name).unwrap();

                match_arms.extend(quote! {
                    #name::ID => {
                        Ok(format!("{:#?}", #name::decode(&mut data)))
                    }
                });
            }

            let state = syn::parse_str::<syn::Ident>(state).unwrap();

            side_arms.extend(quote! {
                valence_protocol::PacketState::#state => match packet.id {
                    #match_arms
                    _ => Ok(NOT_AVAILABLE.to_string()),
                },
            });
        }

        if side == "Clientbound" {
            side_arms.extend(quote! {
                _ => Ok(NOT_AVAILABLE.to_string()),
            });
        }

        let side = syn::parse_str::<syn::Ident>(side).unwrap();

        generated.extend(quote! {
            valence_protocol::PacketSide::#side => match packet.state {
                #side_arms
            },
        });
    }

    // wrap generated in a function definition
    let generated = quote! {
        const NOT_AVAILABLE: &str = "Not yet implemented";

        pub fn packet_to_string(packet: &ProxyPacket) -> Result<String, Box<dyn std::error::Error>> {
            let bytes = packet.data.as_ref().unwrap();
            let mut data = &bytes.clone()[..];

            match packet.side {
                #generated
            }
        }
    };

    write_generated_file(generated, "packet_to_string.rs")?;

    Ok(())
}
