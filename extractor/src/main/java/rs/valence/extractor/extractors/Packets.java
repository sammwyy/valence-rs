package rs.valence.extractor.extractors;

import java.util.Locale;
import java.util.TreeSet;

import com.google.gson.JsonArray;
import com.google.gson.JsonElement;
import com.google.gson.JsonObject;

import net.minecraft.network.NetworkSide;
import net.minecraft.network.NetworkState;
import rs.valence.extractor.Main;

public class Packets implements Main.Extractor {
    @Override
    public String fileName() {
        return "packets.json";
    }

    private static String capitalize(String str) {
        return str.substring(0, 1).toUpperCase(Locale.ROOT) + str.substring(1);
    }

    @Override
    public JsonElement extract() {
        var packetsJson = new JsonArray();

        for (var side : NetworkSide.values()) {
            for (var state : NetworkState.values()) {
                var map = state.getPacketIdToPacketMap(side);

                for (var id : new TreeSet<>(map.keySet())) {
                    var packetJson = new JsonObject();

                    String packetName = map.get(id.intValue()).getSimpleName();
                    String packetSide = side.name().toLowerCase(Locale.ROOT);
                    String packetState = state.name().toLowerCase(Locale.ROOT);
                    String fullPacketName = capitalize(packetState) + packetName;

                    packetJson.addProperty("name", fullPacketName);
                    packetJson.addProperty("side", packetSide);
                    packetJson.addProperty("state", packetState);
                    packetJson.addProperty("id", id);

                    packetsJson.add(packetJson);
                }
            }
        }

        return packetsJson;
    }
}
