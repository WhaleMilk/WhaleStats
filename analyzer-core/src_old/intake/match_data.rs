use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchData {
    pub metadata: Metadata,
    pub info: Info,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub data_version: String,
    pub match_id: String,
    pub participants: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub end_of_game_result: String,
    pub game_creation: i64,
    pub game_duration: i64,
    pub game_end_timestamp: i64,
    pub game_id: i64,
    pub game_mode: String,
    pub game_name: String,
    pub game_start_timestamp: i64,
    pub game_type: String,
    pub game_version: String,
    pub map_id: i64,
    pub participants: Vec<Participant>,
    pub platform_id: String,
    pub queue_id: i64,
    pub teams: Vec<Team>,
    pub tournament_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Participant {
    #[serde(rename = "PlayerScore0")]
    pub player_score0: i64,
    #[serde(rename = "PlayerScore1")]
    pub player_score1: i64,
    #[serde(rename = "PlayerScore10")]
    pub player_score10: i64,
    #[serde(rename = "PlayerScore11")]
    pub player_score11: i64,
    #[serde(rename = "PlayerScore2")]
    pub player_score2: i64,
    #[serde(rename = "PlayerScore3")]
    pub player_score3: i64,
    #[serde(rename = "PlayerScore4")]
    pub player_score4: i64,
    #[serde(rename = "PlayerScore5")]
    pub player_score5: i64,
    #[serde(rename = "PlayerScore6")]
    pub player_score6: i64,
    #[serde(rename = "PlayerScore7")]
    pub player_score7: i64,
    #[serde(rename = "PlayerScore8")]
    pub player_score8: i64,
    #[serde(rename = "PlayerScore9")]
    pub player_score9: i64,
    pub all_in_pings: i64,
    pub assist_me_pings: i64,
    pub assists: i64,
    pub baron_kills: i64,
    pub basic_pings: i64,
    pub challenges: Challenges,
    pub champ_experience: i64,
    pub champ_level: i64,
    pub champion_id: i64,
    pub champion_name: String,
    pub champion_transform: i64,
    pub command_pings: i64,
    pub consumables_purchased: i64,
    pub damage_dealt_to_buildings: i64,
    pub damage_dealt_to_objectives: i64,
    pub damage_dealt_to_turrets: i64,
    pub damage_self_mitigated: i64,
    pub danger_pings: i64,
    pub deaths: i64,
    pub detector_wards_placed: i64,
    pub double_kills: i64,
    pub dragon_kills: i64,
    pub eligible_for_progression: bool,
    pub enemy_missing_pings: i64,
    pub enemy_vision_pings: i64,
    pub first_blood_assist: bool,
    pub first_blood_kill: bool,
    pub first_tower_assist: bool,
    pub first_tower_kill: bool,
    pub game_ended_in_early_surrender: bool,
    pub game_ended_in_surrender: bool,
    pub get_back_pings: i64,
    pub gold_earned: i64,
    pub gold_spent: i64,
    pub hold_pings: i64,
    pub individual_position: String,
    pub inhibitor_kills: i64,
    pub inhibitor_takedowns: i64,
    pub inhibitors_lost: i64,
    pub item0: i64,
    pub item1: i64,
    pub item2: i64,
    pub item3: i64,
    pub item4: i64,
    pub item5: i64,
    pub item6: i64,
    pub items_purchased: i64,
    pub killing_sprees: i64,
    pub kills: i64,
    pub lane: String,
    pub largest_critical_strike: i64,
    pub largest_killing_spree: i64,
    pub largest_multi_kill: i64,
    pub longest_time_spent_living: i64,
    pub magic_damage_dealt: i64,
    pub magic_damage_dealt_to_champions: i64,
    pub magic_damage_taken: i64,
    pub missions: Missions,
    pub need_vision_pings: i64,
    pub neutral_minions_killed: i64,
    pub nexus_kills: i64,
    pub nexus_lost: i64,
    pub nexus_takedowns: i64,
    pub objectives_stolen: i64,
    pub objectives_stolen_assists: i64,
    pub on_my_way_pings: i64,
    pub participant_id: i64,
    pub penta_kills: i64,
    pub perks: Perks,
    pub physical_damage_dealt: i64,
    pub physical_damage_dealt_to_champions: i64,
    pub physical_damage_taken: i64,
    pub placement: i64,
    pub player_augment1: i64,
    pub player_augment2: i64,
    pub player_augment3: i64,
    pub player_augment4: i64,
    pub player_augment5: i64,
    pub player_augment6: i64,
    pub player_subteam_id: i64,
    pub profile_icon: i64,
    pub push_pings: i64,
    pub puuid: String,
    pub quadra_kills: i64,
    pub retreat_pings: i64,
    pub riot_id_game_name: String,
    pub riot_id_tagline: String,
    pub role: String,
    pub sight_wards_bought_in_game: i64,
    #[serde(rename = "spell1Casts")]
    pub spell1casts: i64,
    #[serde(rename = "spell2Casts")]
    pub spell2casts: i64,
    #[serde(rename = "spell3Casts")]
    pub spell3casts: i64,
    #[serde(rename = "spell4Casts")]
    pub spell4casts: i64,
    pub subteam_placement: i64,
    #[serde(rename = "summoner1Casts")]
    pub summoner1casts: i64,
    #[serde(rename = "summoner1Id")]
    pub summoner1id: i64,
    #[serde(rename = "summoner2Casts")]
    pub summoner2casts: i64,
    #[serde(rename = "summoner2Id")]
    pub summoner2id: i64,
    pub summoner_id: String,
    pub summoner_level: i64,
    pub summoner_name: String,
    pub team_early_surrendered: bool,
    pub team_id: i64,
    pub team_position: String,
    #[serde(rename = "timeCCingOthers")]
    pub time_ccing_others: i64,
    pub time_played: i64,
    pub total_ally_jungle_minions_killed: i64,
    pub total_damage_dealt: i64,
    pub total_damage_dealt_to_champions: i64,
    pub total_damage_shielded_on_teammates: i64,
    pub total_damage_taken: i64,
    pub total_enemy_jungle_minions_killed: i64,
    pub total_heal: i64,
    pub total_heals_on_teammates: i64,
    pub total_minions_killed: i64,
    #[serde(rename = "totalTimeCCDealt")]
    pub total_time_ccdealt: i64,
    pub total_time_spent_dead: i64,
    pub total_units_healed: i64,
    pub triple_kills: i64,
    pub true_damage_dealt: i64,
    pub true_damage_dealt_to_champions: i64,
    pub true_damage_taken: i64,
    pub turret_kills: i64,
    pub turret_takedowns: i64,
    pub turrets_lost: i64,
    pub unreal_kills: i64,
    pub vision_cleared_pings: i64,
    pub vision_score: i64,
    pub vision_wards_bought_in_game: i64,
    pub wards_killed: i64,
    pub wards_placed: i64,
    pub win: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Challenges {
    #[serde(rename = "12AssistStreakCount")]
    pub n12assist_streak_count: i64,
    #[serde(rename = "HealFromMapSources")]
    pub heal_from_map_sources: f64,
    #[serde(rename = "InfernalScalePickup")]
    pub infernal_scale_pickup: i64,
    #[serde(rename = "SWARM_DefeatAatrox")]
    pub swarm_defeat_aatrox: i64,
    #[serde(rename = "SWARM_DefeatBriar")]
    pub swarm_defeat_briar: i64,
    #[serde(rename = "SWARM_DefeatMiniBosses")]
    pub swarm_defeat_mini_bosses: i64,
    #[serde(rename = "SWARM_EvolveWeapon")]
    pub swarm_evolve_weapon: i64,
    #[serde(rename = "SWARM_Have3Passives")]
    pub swarm_have3passives: i64,
    #[serde(rename = "SWARM_KillEnemy")]
    pub swarm_kill_enemy: i64,
    #[serde(rename = "SWARM_PickupGold")]
    pub swarm_pickup_gold: i64,
    #[serde(rename = "SWARM_ReachLevel50")]
    pub swarm_reach_level50: i64,
    #[serde(rename = "SWARM_Survive15Min")]
    pub swarm_survive15min: i64,
    #[serde(rename = "SWARM_WinWith5EvolvedWeapons")]
    pub swarm_win_with5evolved_weapons: i64,
    pub ability_uses: i64,
    #[serde(rename = "acesBefore15Minutes")]
    pub aces_before15minutes: i64,
    pub allied_jungle_monster_kills: i64,
    pub baron_takedowns: i64,
    pub blast_cone_opposite_opponent_count: i64,
    pub bounty_gold: f64,
    pub buffs_stolen: i64,
    pub complete_support_quest_in_time: i64,
    pub control_wards_placed: i64,
    pub damage_per_minute: f64,
    pub damage_taken_on_team_percentage: f64,
    pub danced_with_rift_herald: i64,
    pub deaths_by_enemy_champs: i64,
    pub dodge_skill_shots_small_window: i64,
    pub double_aces: i64,
    pub dragon_takedowns: i64,
    pub early_laning_phase_gold_exp_advantage: i64,
    pub effective_heal_and_shielding: f64,
    pub elder_dragon_kills_with_opposing_soul: i64,
    pub elder_dragon_multikills: i64,
    pub enemy_champion_immobilizations: i64,
    pub enemy_jungle_monster_kills: i64,
    pub epic_monster_kills_near_enemy_jungler: i64,
    #[serde(rename = "epicMonsterKillsWithin30SecondsOfSpawn")]
    pub epic_monster_kills_within30seconds_of_spawn: i64,
    pub epic_monster_steals: i64,
    pub epic_monster_stolen_without_smite: i64,
    pub first_turret_killed: i64,
    pub fist_bump_participation: i64,
    pub flawless_aces: i64,
    pub full_team_takedown: i64,
    pub game_length: f64,
    pub get_takedowns_in_all_lanes_early_jungle_as_laner: Option<i64>,
    pub gold_per_minute: f64,
    pub had_open_nexus: i64,
    pub immobilize_and_kill_with_ally: i64,
    pub initial_buff_count: i64,
    pub initial_crab_count: i64,
    #[serde(rename = "jungleCsBefore10Minutes")]
    pub jungle_cs_before10minutes: f64,
    pub jungler_takedowns_near_damaged_epic_monster: i64,
    pub k_turrets_destroyed_before_plates_fall: i64,
    pub kda: f64,
    pub kill_after_hidden_with_ally: i64,
    pub kill_participation: f64,
    pub killed_champ_took_full_team_damage_survived: i64,
    pub killing_sprees: i64,
    pub kills_near_enemy_turret: i64,
    pub kills_on_other_lanes_early_jungle_as_laner: Option<i64>,
    pub kills_on_recently_healed_by_aram_pack: i64,
    pub kills_under_own_turret: i64,
    pub kills_with_help_from_epic_monster: i64,
    pub knock_enemy_into_team_and_kill: i64,
    pub land_skill_shots_early_game: i64,
    #[serde(rename = "laneMinionsFirst10Minutes")]
    pub lane_minions_first10minutes: i64,
    pub laning_phase_gold_exp_advantage: i64,
    pub legendary_count: i64,
    pub legendary_item_used: Vec<i64>,
    pub lost_an_inhibitor: i64,
    pub max_cs_advantage_on_lane_opponent: f64,
    pub max_kill_deficit: i64,
    pub max_level_lead_lane_opponent: i64,
    pub mejais_full_stack_in_time: i64,
    pub more_enemy_jungle_than_opponent: f64,
    pub multi_kill_one_spell: i64,
    pub multi_turret_rift_herald_count: i64,
    pub multikills: i64,
    pub multikills_after_aggressive_flash: i64,
    #[serde(rename = "outerTurretExecutesBefore10Minutes")]
    pub outer_turret_executes_before10minutes: i64,
    pub outnumbered_kills: i64,
    pub outnumbered_nexus_kill: i64,
    pub perfect_dragon_souls_taken: i64,
    pub perfect_game: i64,
    pub pick_kill_with_ally: i64,
    #[serde(skip)]
    pub played_champ_select_position: i64,
    pub poro_explosions: i64,
    pub quick_cleanse: i64,
    pub quick_first_turret: i64,
    pub quick_solo_kills: i64,
    pub rift_herald_takedowns: i64,
    pub save_ally_from_death: i64,
    pub scuttle_crab_kills: i64,
    pub skillshots_dodged: i64,
    pub skillshots_hit: i64,
    pub snowballs_hit: i64,
    pub solo_baron_kills: i64,
    pub solo_kills: i64,
    pub stealth_wards_placed: i64,
    pub survived_single_digit_hp_count: i64,
    pub survived_three_immobilizes_in_fight: i64,
    pub takedown_on_first_turret: i64,
    pub takedowns: i64,
    pub takedowns_after_gaining_level_advantage: i64,
    pub takedowns_before_jungle_minion_spawn: i64,
    #[serde(rename = "takedownsFirstXMinutes")]
    pub takedowns_first_xminutes: i64,
    pub takedowns_in_alcove: i64,
    pub takedowns_in_enemy_fountain: i64,
    pub team_baron_kills: i64,
    pub team_damage_percentage: f64,
    pub team_elder_dragon_kills: i64,
    pub team_rift_herald_kills: i64,
    pub took_large_damage_survived: i64,
    pub turret_plates_taken: i64,
    pub turret_takedowns: i64,
    pub turrets_taken_with_rift_herald: i64,
    #[serde(rename = "twentyMinionsIn3SecondsCount")]
    pub twenty_minions_in3seconds_count: i64,
    pub two_wards_one_sweeper_count: i64,
    pub unseen_recalls: i64,
    pub vision_score_advantage_lane_opponent: f64,
    pub vision_score_per_minute: f64,
    pub void_monster_kill: i64,
    pub ward_takedowns: i64,
    #[serde(rename = "wardTakedownsBefore20M")]
    pub ward_takedowns_before20m: i64,
    pub wards_guarded: i64,
    pub jungler_kills_early_jungle: Option<i64>,
    pub kills_on_laners_early_jungle_as_jungler: Option<i64>,
    pub control_ward_time_coverage_in_river_or_enemy_half: Option<f64>,
    pub highest_ward_kills: Option<i64>,
    pub first_turret_killed_time: Option<f64>,
    pub solo_turrets_lategame: Option<i64>,
    pub earliest_dragon_takedown: Option<f64>,
    pub highest_champion_damage: Option<i64>,
    pub faster_support_quest_completion: Option<i64>,
    pub highest_crowd_control_score: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Missions {
    pub player_score0: i64,
    pub player_score1: i64,
    pub player_score2: i64,
    pub player_score3: i64,
    pub player_score4: i64,
    pub player_score5: i64,
    pub player_score6: i64,
    pub player_score7: i64,
    pub player_score8: i64,
    pub player_score9: i64,
    pub player_score10: i64,
    pub player_score11: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Perks {
    pub stat_perks: StatPerks,
    pub styles: Vec<Style>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatPerks {
    pub defense: i64,
    pub flex: i64,
    pub offense: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Style {
    pub description: String,
    pub selections: Vec<Selection>,
    pub style: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Selection {
    pub perk: i64,
    pub var1: i64,
    pub var2: i64,
    pub var3: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub bans: Vec<Ban>,
    pub feats: Feats,
    pub objectives: Objectives,
    pub team_id: i64,
    pub win: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ban {
    pub champion_id: i64,
    pub pick_turn: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Feats {
    #[serde(rename = "EPIC_MONSTER_KILL")]
    pub epic_monster_kill: EpicMonsterKill,
    #[serde(rename = "FIRST_BLOOD")]
    pub first_blood: FirstBlood,
    #[serde(rename = "FIRST_TURRET")]
    pub first_turret: FirstTurret,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EpicMonsterKill {
    pub feat_state: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirstBlood {
    pub feat_state: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirstTurret {
    pub feat_state: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Objectives {
    pub atakhan: Atakhan,
    pub baron: Baron,
    pub champion: Champion,
    pub dragon: Dragon,
    pub horde: Horde,
    pub inhibitor: Inhibitor,
    pub rift_herald: RiftHerald,
    pub tower: Tower,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Atakhan {
    pub first: bool,
    pub kills: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Baron {
    pub first: bool,
    pub kills: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Champion {
    pub first: bool,
    pub kills: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dragon {
    pub first: bool,
    pub kills: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Horde {
    pub first: bool,
    pub kills: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Inhibitor {
    pub first: bool,
    pub kills: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiftHerald {
    pub first: bool,
    pub kills: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tower {
    pub first: bool,
    pub kills: i64,
}
