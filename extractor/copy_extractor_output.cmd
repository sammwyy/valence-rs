@echo off

:: Copies all files generated by the extractor to the necessary locations.
:: Run the extractor first (`./gradlew runServer`) before running this.

:: Copy to valence_entity
copy run\valennce_extractor_output\entities.json ..\crates\valence_entity\extracted\
copy run\valence_extractor_output\misc.json ..\crates\valence_entity\extracted\

:: Copy to valence_generated
copy run\valence_extractor_output\attributes.json ..\crates\valence_generated\extracted\
copy run\valence_extractor_output\blocks.json ..\crates\valence_generated\extracted\
copy run\valence_extractor_output\effects.json ..\crates\valence_generated\extracted\
copy run\valence_extractor_output\items.json ..\crates\valence_generated\extracted\
copy run\valence_extractor_output\packets.json ..\crates\valence_generated\extracted\
copy run\valence_extractor_output\sounds.json ..\crates\valence_generated\extracted\

:: Copy to valence_lang
copy run\valence_extractor_output\translation_keys.json ..\crates\valence_lang\extracted\

:: Copy to valence_registry
copy run\valence_extractor_output\registry_codec.dat ..\crates\valence_registry\extracted\
copy run\valence_extractor_output\tags.json ..\crates\valence_registry\extracted\

:: Copy to packet_inspector
copy run\valence_extractor_output\packets.json ..\tools\packet_inspector\extracted\