use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Timeline {
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
    pub frame_interval: i64,
    pub frames: Vec<Frame>,
    pub game_id: i64,
    pub participants: Vec<Participant>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Frame {
    pub events: Vec<Event>,
    pub participant_frames: ParticipantFrames,
    pub timestamp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub item_id: Option<i64>,
    pub participant_id: Option<i64>,
    pub timestamp: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub level_up_type: Option<String>,
    pub skill_slot: Option<i64>,
    pub creator_id: Option<i64>,
    pub ward_type: Option<String>,
    pub real_timestamp: Option<i64>,
    pub level: Option<i64>,
    #[serde(default)]
    pub assisting_participant_ids: Vec<i64>,
    pub bounty: Option<i64>,
    pub kill_streak_length: Option<i64>,
    pub killer_id: Option<i64>,
    pub position: Option<Position>,
    pub shutdown_bounty: Option<i64>,
    #[serde(default)]
    pub victim_damage_dealt: Vec<VictimDamageDealt>,
    #[serde(default)]
    pub victim_damage_received: Vec<VictimDamageReceived>,
    pub victim_id: Option<i64>,
    pub kill_type: Option<String>,
    pub feat_type: Option<i64>,
    pub feat_value: Option<i64>,
    pub team_id: Option<i64>,
    pub after_id: Option<i64>,
    pub before_id: Option<i64>,
    pub gold_gain: Option<i64>,
    pub killer_team_id: Option<i64>,
    pub monster_sub_type: Option<String>,
    pub monster_type: Option<String>,
    pub lane_type: Option<String>,
    pub name: Option<String>,
    pub building_type: Option<String>,
    pub tower_type: Option<String>,
    pub actual_start_time: Option<i64>,
    pub multi_kill_length: Option<i64>,
    pub game_id: Option<i64>,
    pub winning_team: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VictimDamageDealt {
    pub basic: bool,
    pub magic_damage: i64,
    pub name: String,
    pub participant_id: i64,
    pub physical_damage: i64,
    pub spell_name: String,
    pub spell_slot: i64,
    pub true_damage: i64,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VictimDamageReceived {
    pub basic: bool,
    pub magic_damage: i64,
    pub name: String,
    pub participant_id: i64,
    pub physical_damage: i64,
    pub spell_name: String,
    pub spell_slot: i64,
    pub true_damage: i64,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantFrames {
    #[serde(rename = "1")]
    pub n1: n1,
    #[serde(rename = "2")]
    pub n2: n2,
    #[serde(rename = "3")]
    pub n3: n3,
    #[serde(rename = "4")]
    pub n4: n4,
    #[serde(rename = "5")]
    pub n5: n5,
    #[serde(rename = "6")]
    pub n6: n6,
    #[serde(rename = "7")]
    pub n7: n7,
    #[serde(rename = "8")]
    pub n8: n8,
    #[serde(rename = "9")]
    pub n9: n9,
    #[serde(rename = "10")]
    pub n10: n10,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n1 {
    pub champion_stats: ChampionStats,
    pub current_gold: i64,
    pub damage_stats: DamageStats,
    pub gold_per_second: i64,
    pub jungle_minions_killed: i64,
    pub level: i64,
    pub minions_killed: i64,
    pub participant_id: i64,
    pub position: Position2,
    pub time_enemy_spent_controlled: i64,
    pub total_gold: i64,
    pub xp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionStats {
    pub ability_haste: i64,
    pub ability_power: i64,
    pub armor: i64,
    pub armor_pen: i64,
    pub armor_pen_percent: i64,
    pub attack_damage: i64,
    pub attack_speed: i64,
    pub bonus_armor_pen_percent: i64,
    pub bonus_magic_pen_percent: i64,
    pub cc_reduction: i64,
    pub cooldown_reduction: i64,
    pub health: i64,
    pub health_max: i64,
    pub health_regen: i64,
    pub lifesteal: i64,
    pub magic_pen: i64,
    pub magic_pen_percent: i64,
    pub magic_resist: i64,
    pub movement_speed: i64,
    pub omnivamp: i64,
    pub physical_vamp: i64,
    pub power: i64,
    pub power_max: i64,
    pub power_regen: i64,
    pub spell_vamp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DamageStats {
    pub magic_damage_done: i64,
    pub magic_damage_done_to_champions: i64,
    pub magic_damage_taken: i64,
    pub physical_damage_done: i64,
    pub physical_damage_done_to_champions: i64,
    pub physical_damage_taken: i64,
    pub total_damage_done: i64,
    pub total_damage_done_to_champions: i64,
    pub total_damage_taken: i64,
    pub true_damage_done: i64,
    pub true_damage_done_to_champions: i64,
    pub true_damage_taken: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position2 {
    pub x: i64,
    pub y: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n2 {
    pub champion_stats: ChampionStats2,
    pub current_gold: i64,
    pub damage_stats: DamageStats2,
    pub gold_per_second: i64,
    pub jungle_minions_killed: i64,
    pub level: i64,
    pub minions_killed: i64,
    pub participant_id: i64,
    pub position: Position3,
    pub time_enemy_spent_controlled: i64,
    pub total_gold: i64,
    pub xp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionStats2 {
    pub ability_haste: i64,
    pub ability_power: i64,
    pub armor: i64,
    pub armor_pen: i64,
    pub armor_pen_percent: i64,
    pub attack_damage: i64,
    pub attack_speed: i64,
    pub bonus_armor_pen_percent: i64,
    pub bonus_magic_pen_percent: i64,
    pub cc_reduction: i64,
    pub cooldown_reduction: i64,
    pub health: i64,
    pub health_max: i64,
    pub health_regen: i64,
    pub lifesteal: i64,
    pub magic_pen: i64,
    pub magic_pen_percent: i64,
    pub magic_resist: i64,
    pub movement_speed: i64,
    pub omnivamp: i64,
    pub physical_vamp: i64,
    pub power: i64,
    pub power_max: i64,
    pub power_regen: i64,
    pub spell_vamp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DamageStats2 {
    pub magic_damage_done: i64,
    pub magic_damage_done_to_champions: i64,
    pub magic_damage_taken: i64,
    pub physical_damage_done: i64,
    pub physical_damage_done_to_champions: i64,
    pub physical_damage_taken: i64,
    pub total_damage_done: i64,
    pub total_damage_done_to_champions: i64,
    pub total_damage_taken: i64,
    pub true_damage_done: i64,
    pub true_damage_done_to_champions: i64,
    pub true_damage_taken: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position3 {
    pub x: i64,
    pub y: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n3 {
    pub champion_stats: ChampionStats3,
    pub current_gold: i64,
    pub damage_stats: DamageStats3,
    pub gold_per_second: i64,
    pub jungle_minions_killed: i64,
    pub level: i64,
    pub minions_killed: i64,
    pub participant_id: i64,
    pub position: Position4,
    pub time_enemy_spent_controlled: i64,
    pub total_gold: i64,
    pub xp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionStats3 {
    pub ability_haste: i64,
    pub ability_power: i64,
    pub armor: i64,
    pub armor_pen: i64,
    pub armor_pen_percent: i64,
    pub attack_damage: i64,
    pub attack_speed: i64,
    pub bonus_armor_pen_percent: i64,
    pub bonus_magic_pen_percent: i64,
    pub cc_reduction: i64,
    pub cooldown_reduction: i64,
    pub health: i64,
    pub health_max: i64,
    pub health_regen: i64,
    pub lifesteal: i64,
    pub magic_pen: i64,
    pub magic_pen_percent: i64,
    pub magic_resist: i64,
    pub movement_speed: i64,
    pub omnivamp: i64,
    pub physical_vamp: i64,
    pub power: i64,
    pub power_max: i64,
    pub power_regen: i64,
    pub spell_vamp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DamageStats3 {
    pub magic_damage_done: i64,
    pub magic_damage_done_to_champions: i64,
    pub magic_damage_taken: i64,
    pub physical_damage_done: i64,
    pub physical_damage_done_to_champions: i64,
    pub physical_damage_taken: i64,
    pub total_damage_done: i64,
    pub total_damage_done_to_champions: i64,
    pub total_damage_taken: i64,
    pub true_damage_done: i64,
    pub true_damage_done_to_champions: i64,
    pub true_damage_taken: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position4 {
    pub x: i64,
    pub y: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n4 {
    pub champion_stats: ChampionStats4,
    pub current_gold: i64,
    pub damage_stats: DamageStats4,
    pub gold_per_second: i64,
    pub jungle_minions_killed: i64,
    pub level: i64,
    pub minions_killed: i64,
    pub participant_id: i64,
    pub position: Position5,
    pub time_enemy_spent_controlled: i64,
    pub total_gold: i64,
    pub xp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionStats4 {
    pub ability_haste: i64,
    pub ability_power: i64,
    pub armor: i64,
    pub armor_pen: i64,
    pub armor_pen_percent: i64,
    pub attack_damage: i64,
    pub attack_speed: i64,
    pub bonus_armor_pen_percent: i64,
    pub bonus_magic_pen_percent: i64,
    pub cc_reduction: i64,
    pub cooldown_reduction: i64,
    pub health: i64,
    pub health_max: i64,
    pub health_regen: i64,
    pub lifesteal: i64,
    pub magic_pen: i64,
    pub magic_pen_percent: i64,
    pub magic_resist: i64,
    pub movement_speed: i64,
    pub omnivamp: i64,
    pub physical_vamp: i64,
    pub power: i64,
    pub power_max: i64,
    pub power_regen: i64,
    pub spell_vamp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DamageStats4 {
    pub magic_damage_done: i64,
    pub magic_damage_done_to_champions: i64,
    pub magic_damage_taken: i64,
    pub physical_damage_done: i64,
    pub physical_damage_done_to_champions: i64,
    pub physical_damage_taken: i64,
    pub total_damage_done: i64,
    pub total_damage_done_to_champions: i64,
    pub total_damage_taken: i64,
    pub true_damage_done: i64,
    pub true_damage_done_to_champions: i64,
    pub true_damage_taken: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position5 {
    pub x: i64,
    pub y: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n5 {
    pub champion_stats: ChampionStats5,
    pub current_gold: i64,
    pub damage_stats: DamageStats5,
    pub gold_per_second: i64,
    pub jungle_minions_killed: i64,
    pub level: i64,
    pub minions_killed: i64,
    pub participant_id: i64,
    pub position: Position6,
    pub time_enemy_spent_controlled: i64,
    pub total_gold: i64,
    pub xp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionStats5 {
    pub ability_haste: i64,
    pub ability_power: i64,
    pub armor: i64,
    pub armor_pen: i64,
    pub armor_pen_percent: i64,
    pub attack_damage: i64,
    pub attack_speed: i64,
    pub bonus_armor_pen_percent: i64,
    pub bonus_magic_pen_percent: i64,
    pub cc_reduction: i64,
    pub cooldown_reduction: i64,
    pub health: i64,
    pub health_max: i64,
    pub health_regen: i64,
    pub lifesteal: i64,
    pub magic_pen: i64,
    pub magic_pen_percent: i64,
    pub magic_resist: i64,
    pub movement_speed: i64,
    pub omnivamp: i64,
    pub physical_vamp: i64,
    pub power: i64,
    pub power_max: i64,
    pub power_regen: i64,
    pub spell_vamp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DamageStats5 {
    pub magic_damage_done: i64,
    pub magic_damage_done_to_champions: i64,
    pub magic_damage_taken: i64,
    pub physical_damage_done: i64,
    pub physical_damage_done_to_champions: i64,
    pub physical_damage_taken: i64,
    pub total_damage_done: i64,
    pub total_damage_done_to_champions: i64,
    pub total_damage_taken: i64,
    pub true_damage_done: i64,
    pub true_damage_done_to_champions: i64,
    pub true_damage_taken: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position6 {
    pub x: i64,
    pub y: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n6 {
    pub champion_stats: ChampionStats6,
    pub current_gold: i64,
    pub damage_stats: DamageStats6,
    pub gold_per_second: i64,
    pub jungle_minions_killed: i64,
    pub level: i64,
    pub minions_killed: i64,
    pub participant_id: i64,
    pub position: Position7,
    pub time_enemy_spent_controlled: i64,
    pub total_gold: i64,
    pub xp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionStats6 {
    pub ability_haste: i64,
    pub ability_power: i64,
    pub armor: i64,
    pub armor_pen: i64,
    pub armor_pen_percent: i64,
    pub attack_damage: i64,
    pub attack_speed: i64,
    pub bonus_armor_pen_percent: i64,
    pub bonus_magic_pen_percent: i64,
    pub cc_reduction: i64,
    pub cooldown_reduction: i64,
    pub health: i64,
    pub health_max: i64,
    pub health_regen: i64,
    pub lifesteal: i64,
    pub magic_pen: i64,
    pub magic_pen_percent: i64,
    pub magic_resist: i64,
    pub movement_speed: i64,
    pub omnivamp: i64,
    pub physical_vamp: i64,
    pub power: i64,
    pub power_max: i64,
    pub power_regen: i64,
    pub spell_vamp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DamageStats6 {
    pub magic_damage_done: i64,
    pub magic_damage_done_to_champions: i64,
    pub magic_damage_taken: i64,
    pub physical_damage_done: i64,
    pub physical_damage_done_to_champions: i64,
    pub physical_damage_taken: i64,
    pub total_damage_done: i64,
    pub total_damage_done_to_champions: i64,
    pub total_damage_taken: i64,
    pub true_damage_done: i64,
    pub true_damage_done_to_champions: i64,
    pub true_damage_taken: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position7 {
    pub x: i64,
    pub y: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n7 {
    pub champion_stats: ChampionStats7,
    pub current_gold: i64,
    pub damage_stats: DamageStats7,
    pub gold_per_second: i64,
    pub jungle_minions_killed: i64,
    pub level: i64,
    pub minions_killed: i64,
    pub participant_id: i64,
    pub position: Position8,
    pub time_enemy_spent_controlled: i64,
    pub total_gold: i64,
    pub xp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionStats7 {
    pub ability_haste: i64,
    pub ability_power: i64,
    pub armor: i64,
    pub armor_pen: i64,
    pub armor_pen_percent: i64,
    pub attack_damage: i64,
    pub attack_speed: i64,
    pub bonus_armor_pen_percent: i64,
    pub bonus_magic_pen_percent: i64,
    pub cc_reduction: i64,
    pub cooldown_reduction: i64,
    pub health: i64,
    pub health_max: i64,
    pub health_regen: i64,
    pub lifesteal: i64,
    pub magic_pen: i64,
    pub magic_pen_percent: i64,
    pub magic_resist: i64,
    pub movement_speed: i64,
    pub omnivamp: i64,
    pub physical_vamp: i64,
    pub power: i64,
    pub power_max: i64,
    pub power_regen: i64,
    pub spell_vamp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DamageStats7 {
    pub magic_damage_done: i64,
    pub magic_damage_done_to_champions: i64,
    pub magic_damage_taken: i64,
    pub physical_damage_done: i64,
    pub physical_damage_done_to_champions: i64,
    pub physical_damage_taken: i64,
    pub total_damage_done: i64,
    pub total_damage_done_to_champions: i64,
    pub total_damage_taken: i64,
    pub true_damage_done: i64,
    pub true_damage_done_to_champions: i64,
    pub true_damage_taken: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position8 {
    pub x: i64,
    pub y: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n8 {
    pub champion_stats: ChampionStats8,
    pub current_gold: i64,
    pub damage_stats: DamageStats8,
    pub gold_per_second: i64,
    pub jungle_minions_killed: i64,
    pub level: i64,
    pub minions_killed: i64,
    pub participant_id: i64,
    pub position: Position9,
    pub time_enemy_spent_controlled: i64,
    pub total_gold: i64,
    pub xp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionStats8 {
    pub ability_haste: i64,
    pub ability_power: i64,
    pub armor: i64,
    pub armor_pen: i64,
    pub armor_pen_percent: i64,
    pub attack_damage: i64,
    pub attack_speed: i64,
    pub bonus_armor_pen_percent: i64,
    pub bonus_magic_pen_percent: i64,
    pub cc_reduction: i64,
    pub cooldown_reduction: i64,
    pub health: i64,
    pub health_max: i64,
    pub health_regen: i64,
    pub lifesteal: i64,
    pub magic_pen: i64,
    pub magic_pen_percent: i64,
    pub magic_resist: i64,
    pub movement_speed: i64,
    pub omnivamp: i64,
    pub physical_vamp: i64,
    pub power: i64,
    pub power_max: i64,
    pub power_regen: i64,
    pub spell_vamp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DamageStats8 {
    pub magic_damage_done: i64,
    pub magic_damage_done_to_champions: i64,
    pub magic_damage_taken: i64,
    pub physical_damage_done: i64,
    pub physical_damage_done_to_champions: i64,
    pub physical_damage_taken: i64,
    pub total_damage_done: i64,
    pub total_damage_done_to_champions: i64,
    pub total_damage_taken: i64,
    pub true_damage_done: i64,
    pub true_damage_done_to_champions: i64,
    pub true_damage_taken: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position9 {
    pub x: i64,
    pub y: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n9 {
    pub champion_stats: ChampionStats9,
    pub current_gold: i64,
    pub damage_stats: DamageStats9,
    pub gold_per_second: i64,
    pub jungle_minions_killed: i64,
    pub level: i64,
    pub minions_killed: i64,
    pub participant_id: i64,
    pub position: Position10,
    pub time_enemy_spent_controlled: i64,
    pub total_gold: i64,
    pub xp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionStats9 {
    pub ability_haste: i64,
    pub ability_power: i64,
    pub armor: i64,
    pub armor_pen: i64,
    pub armor_pen_percent: i64,
    pub attack_damage: i64,
    pub attack_speed: i64,
    pub bonus_armor_pen_percent: i64,
    pub bonus_magic_pen_percent: i64,
    pub cc_reduction: i64,
    pub cooldown_reduction: i64,
    pub health: i64,
    pub health_max: i64,
    pub health_regen: i64,
    pub lifesteal: i64,
    pub magic_pen: i64,
    pub magic_pen_percent: i64,
    pub magic_resist: i64,
    pub movement_speed: i64,
    pub omnivamp: i64,
    pub physical_vamp: i64,
    pub power: i64,
    pub power_max: i64,
    pub power_regen: i64,
    pub spell_vamp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DamageStats9 {
    pub magic_damage_done: i64,
    pub magic_damage_done_to_champions: i64,
    pub magic_damage_taken: i64,
    pub physical_damage_done: i64,
    pub physical_damage_done_to_champions: i64,
    pub physical_damage_taken: i64,
    pub total_damage_done: i64,
    pub total_damage_done_to_champions: i64,
    pub total_damage_taken: i64,
    pub true_damage_done: i64,
    pub true_damage_done_to_champions: i64,
    pub true_damage_taken: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position10 {
    pub x: i64,
    pub y: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n10 {
    pub champion_stats: ChampionStats10,
    pub current_gold: i64,
    pub damage_stats: DamageStats10,
    pub gold_per_second: i64,
    pub jungle_minions_killed: i64,
    pub level: i64,
    pub minions_killed: i64,
    pub participant_id: i64,
    pub position: Position11,
    pub time_enemy_spent_controlled: i64,
    pub total_gold: i64,
    pub xp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionStats10 {
    pub ability_haste: i64,
    pub ability_power: i64,
    pub armor: i64,
    pub armor_pen: i64,
    pub armor_pen_percent: i64,
    pub attack_damage: i64,
    pub attack_speed: i64,
    pub bonus_armor_pen_percent: i64,
    pub bonus_magic_pen_percent: i64,
    pub cc_reduction: i64,
    pub cooldown_reduction: i64,
    pub health: i64,
    pub health_max: i64,
    pub health_regen: i64,
    pub lifesteal: i64,
    pub magic_pen: i64,
    pub magic_pen_percent: i64,
    pub magic_resist: i64,
    pub movement_speed: i64,
    pub omnivamp: i64,
    pub physical_vamp: i64,
    pub power: i64,
    pub power_max: i64,
    pub power_regen: i64,
    pub spell_vamp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DamageStats10 {
    pub magic_damage_done: i64,
    pub magic_damage_done_to_champions: i64,
    pub magic_damage_taken: i64,
    pub physical_damage_done: i64,
    pub physical_damage_done_to_champions: i64,
    pub physical_damage_taken: i64,
    pub total_damage_done: i64,
    pub total_damage_done_to_champions: i64,
    pub total_damage_taken: i64,
    pub true_damage_done: i64,
    pub true_damage_done_to_champions: i64,
    pub true_damage_taken: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position11 {
    pub x: i64,
    pub y: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Participant {
    pub participant_id: i64,
    pub puuid: String,
}
