use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Player {
    id: u32,
    resource_id: Option<u32>,
    name: String,
    age: u32,
    resource_base_id: Option<u32>,
    fut_bin_id: Option<u32>,
    fut_wiz_id: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    common_name: String,
    height: u32,
    weight: u32,
    birth_date: String,
    league: u32,
    nation: u32,
    club: u32,
    rarity: u32,
    traits: Vec<Trait>,
    specialities: Vec<Specialitie>,
    position: String,
    skill_moves: u32,
    weak_foot: u32,
    foot: String,
    attack_work_rate: String,
    defense_work_rate: String,
    total_stats: Option<u32>,
    total_stats_in_game: Option<u32>,
    rating: u32,
    rating_average: Option<u32>,
    pace: u32,
    shooting: u32,
    passing: u32,
    dribbling: u32,
    defending: u32,
    physicality: u32,
    pace_attributes: Option<PlayerPace>,
    shooting_attributes: Option<PlayerShooting>,
    passing_attributes: Option<PlayerPassing>,
    dribbling_attributes: Option<PlayreDribbling>,
    defending_attributes: Option<PlayerDefending>,
    physicality_attributes: Option<PlayerPhysicality>,
    goalkeeper_attributes: Option<PlayerGoalkeeper>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Players {
    pub items: Vec<Player>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Trait {
    id: u32,
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Specialitie {
    id: u32,
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct PlayerPace {
    acceleration: u32,
    sprint_speed: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct PlayerShooting {
    positioning: u32,
    finishing: u32,
    shot_power: u32,
    long_shots: u32,
    volleys: u32,
    penalties: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct PlayerPassing {
    vision: u32,
    crossing: u32,
    free_kick_accuracy: u32,
    short_passing: u32,
    long_passing: u32,
    curve: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct PlayreDribbling {
    agility: u32,
    balance: u32,
    reactions: u32,
    ball_control: u32,
    dribbling: u32,
    composure: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct PlayerDefending {
    interceptions: u32,
    heading_accuracy: u32,
    standing_tackle: u32,
    sliding_tackle: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct PlayerPhysicality {
    jumping: u32,
    stamina: u32,
    strength: u32,
    aggression: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct 	PlayerGoalkeeper {
    diving: u32,
    handling: u32,
    kicking: u32,
    positioning: u32,
    reflexes: u32,
}