use drg_mission_gen_core::{gen_deep_dive_pair, EBiome, EMissionDNA, EObjective};                   // any export parameters you want to search for must be listed here

fn main() {
    let mut count = 0;
    for seed in 2048..0x20000 {                                                                    // number or hexadecimal values here will let you narrow the search's seed range (live server supports 2048 through 131072)
        let (normal, _hard) = gen_deep_dive_pair(seed);                                            // _hard indicates Elite dive, normal indicates regular dive

        if _hard.missions[0].biome != EBiome::BIOME_AzureWeald                                     // [0] indicates Stage 1 (use 1 or 2 for stages 2 and 3 respectively), EBiome filters biome
        {
            continue;
        }
        if _hard.missions[0].primary_objective.objective() != EObjective::OBJ_1st_Escort           // EObjective can be used to fetch primary or secondary objective (in this case, Escort primary)
        {                                                                                          // see src\data.rs for full list of objective names
            continue;
        }
        if _hard.missions[1].primary_objective.objective() != EObjective::OBJ_1st_Extraction {
            continue;
        }
        if _hard.missions[1].dna != EMissionDNA::DNA_2_01 {                                        // EMissionDNA allows you to specify objective by length/complexity,
            continue;                                                                              // and does not need to be preceded by an EObjective search for its stage
        }
        if _hard.missions[2].primary_objective.objective() != EObjective::OBJ_Excavation_C
        {
            continue;
        }
        if _hard                                                                                   // This section will allow you to exclude any dives with features you DON'T
            .missions                                                                              // want to see (in this case, Dreadnought and Black Box secondaries)
            .iter()
            .flat_map(|m| &m.secondary_objectives)
            .any(|s| {
                [
                    EObjective::OBJ_DD_Elimination_Eggs,
                    EObjective::OBJ_DD_Defense,
                ]
                .contains(&s.objective())
            })
        {
            continue;
        }

        count += 1;
        println!("DD: {seed} = {_hard:#?}");                                                       // The _hard or normal variable here must be swapped as well based on what dives you want
    }
    println!("found {count} matching seeds");
}