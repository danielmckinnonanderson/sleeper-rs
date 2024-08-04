use core::fmt;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

pub type LeagueId = String;
pub type PlayerId = String;
pub type OwnerId = String;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct League {
    pub total_rosters: u8,
    pub status: String,
    pub sport: String,
    pub shard: u32,
    pub settings: LeagueSettings,
    pub season_type: String,
    pub season: String,
    pub scoring_settings: ScoringSettings,
    pub roster_positions: Vec<RosterPosition>,
    pub previous_league_id: String,
    pub name: String,
    pub metadata: Option<HashMap<String, Option<String>>>,
    pub loser_bracket_id: Option<u64>,
    pub league_id: LeagueId,
    pub last_transation_id: Option<String>,
    pub last_read_id: Option<String>,
    pub last_pinned_message_id: Option<String>,
    pub last_message_time: u64,
    pub last_message_text_mape: Option<String>,
    pub last_message_id: Option<String>,
    pub last_message_attachment: Option<String>,
    pub last_author_is_bot: Option<bool>,
    pub last_author_id: Option<String>,
    pub last_author_display_name: Option<String>,
    pub last_author_avatar: Option<String>,
    pub group_id: Option<String>,
    pub draft_id: Option<String>,
    pub company_id: Option<String>,
    pub bracket_id: Option<u64>,
    pub avatar: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LeagueSettings {
    pub waiver_type: u8,
    pub waiver_day_of_week: u8,
    pub waiver_clear_days: u8,
    pub waiver_budget: u8,
    pub veto_votes_needed: u8,
    pub veto_show_votes: u8,
    pub veto_auto_poll: u8,
    pub trade_review_days: u8,
    pub trade_deadline: u8,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub _type: u8,
    pub taxi_years: u8,
    pub taxi_slots: u8,
    pub taxi_deadline: u8,
    pub taxi_allow_vets: u8,
    pub squads: u8,
    pub start_week: u8,
    pub reserve_slots: u8,
    pub reserve_allow_sus: u8,
    pub reserve_allow_out: u8,
    pub reserve_allow_na: u8,
    pub reserve_allow_doubtful: u8,
    pub reserve_allow_dnr: u8,
    pub reserve_allow_cov: u8,
    pub playoff_week_start: u8,
    pub playoff_type: u8,
    pub playoff_seed_type: u8,
    pub playoff_round_type: u8,
    pub playoff_teams: u8,
    pub pick_trading: u8,
    pub offseason_adds: u8,
    pub num_teams: u8,
    pub max_keepers: u8,
    pub leg: u8,
    pub league_average_match: u8,
    pub last_report: u8,
    pub disable_trades: u8,
    pub disable_adds: u8,
    pub draft_rounds: u8,
    pub daily_waivers_last_ran: u16,
    pub daily_waivers_hour: u32,
    pub daily_waivers_days: u16,
    pub daily_waivers: u8,
    pub commissioner_direct_invite: u8,
    pub capacity_override: u8,
    pub best_ball: u8,
    pub bench_lock: u8,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScoringSettings {
    pub st_ff: f64,
    pub pts_allow_7_13: f64,
    pub def_st_ff: f64,
    pub rec_yd: f64,
    pub fum_rec_td: f64,
    pub pts_allow_35p: f64,
    pub pts_allow_28_34: f64,
    pub fum: f64,
    pub rush_yd: f64,
    pub pass_td: f64,
    pub blk_kick: f64,
    pub pass_yd: f64,
    pub safe: f64,
    pub def_td: f64,
    pub fgm_50p: f64,
    pub def_st_td: f64,
    pub fum_rec: f64,
    pub rush_2pt: f64,
    pub xpm: f64,
    pub pts_allow_21_27: f64,
    pub fgm_20_29: f64,
    pub pts_allow_1_6: f64,
    pub fum_lost: f64,
    pub def_st_fum_rec: f64,
    pub int: f64,
    pub def_kr_td: f64,
    pub fgm_0_19: f64,
    pub pts_allow_14_20: f64,
    pub rec: f64,
    pub ff: f64,
    pub fgmiss: f64,
    pub st_fum_rec: f64,
    pub rec_2pt: f64,
    pub def_pr_td: f64,
    pub rush_td: f64,
    pub xpmiss: f64,
    pub fgm_30_39: f64,
    pub rec_td: f64,
    pub st_td: f64,
    pub pass_2pt: f64,
    pub pts_allow_0: f64,
    pub pass_int: f64,
    pub fgm_40_49: f64,
    pub sack: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum RosterPosition {
    QB,
    RB,
    WR,
    TE,
    FLEX,
    K,
    DEF,
    BN,
    IDP,
}

impl fmt::Display for RosterPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RosterPosition::QB => write!(f, "QB"),
            RosterPosition::RB => write!(f, "RB"),
            RosterPosition::WR => write!(f, "WR"),
            RosterPosition::TE => write!(f, "TE"),
            RosterPosition::FLEX => write!(f, "FLEX"),
            RosterPosition::K => write!(f, "K"),
            RosterPosition::DEF => write!(f, "DEF"),
            RosterPosition::BN => write!(f, "BN"),
            RosterPosition::IDP => write!(f, "IDP"),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Roster {
    pub taxi: Value,
    pub starters: Vec<String>,
    pub settings: Value,
    pub roster_id: u8,
    pub reserve: Value,
    pub players: Vec<String>,
    pub player_map: Value,
    pub owner_id: OwnerId,
    pub metadata: Value,
    pub league_id: LeagueId,
    pub keepers: Value,
    pub co_owners: Value,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RosterSettings {
    pub wins: u8,
    pub waiver_position: u16,
    pub waiver_budget_used: u32,
    pub total_moves: u32,
    pub ties: u16,
    pub losses: u16,
    pub fpts: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SleeperUser {
    pub user_id: String,
    pub username: Option<String>,
    pub settings: Option<HashMap<String, String>>,
    // TODO - metadata has one important field "team_name" that should be communicated
    pub metadata: HashMap<String, Option<String>>,
    pub is_owner: bool,
    pub is_bot: bool,
    pub league_id: LeagueId,
    pub display_name: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Matchup {
    starters_points: Vec<f64>,
    starters: Vec<PlayerId>,
    roster_id: u8,
    points: f64,
    players_points: Option<HashMap<PlayerId, f64>>,
    players: Option<Vec<PlayerId>>,
    matchup_id: u8,
    custom_points: Option<HashMap<String, String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum SleeperSport {
    NFL,
    NBA,
    LCS,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum InjuryStatus {
    Healthy,
    InjuredReserve,
    Covid,
    NotActive,
    Out,
    PUP,
    Questionable,
    Suspended,
}

impl InjuryStatus {
    pub fn from_str(s: &str) -> Result<InjuryStatus, SleeperError> {
        let lower = s.to_lowercase();
        match lower.as_ref() {
            "questionable" => Ok(InjuryStatus::Questionable),
            "cov" => Ok(InjuryStatus::Covid),
            "ir" => Ok(InjuryStatus::InjuredReserve),
            "na" => Ok(InjuryStatus::NotActive),
            "pup" => Ok(InjuryStatus::PUP),
            "sus" => Ok(InjuryStatus::Suspended),
            unmatched => Err(SleeperError::InvalidInjuryStatus(unmatched.to_string())),
        }
    }

    pub fn from_json(node: &Value) -> Result<InjuryStatus, SleeperError> {
        match node {
            Value::String(s) => InjuryStatus::from_str(s),
            Value::Null => Ok(InjuryStatus::Healthy),
            _ => Err(SleeperError::InvalidInjuryStatus(
                "{ an object }".to_string(),
            )),
        }
    }

    pub fn from_opt_string(s: Option<String>) -> Result<Self, SleeperError> {
        match s {
            Some(s) => InjuryStatus::from_str(s.as_ref()),
            None => Ok(InjuryStatus::Healthy),
        }
    }
}

impl fmt::Display for InjuryStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InjuryStatus::Healthy => write!(f, "Healthy"),
            InjuryStatus::InjuredReserve => write!(f, "Injured Reserve"),
            InjuryStatus::Covid => write!(f, "COVID-19"),
            InjuryStatus::NotActive => write!(f, "Inactive"),
            InjuryStatus::Out => write!(f, "Out"),
            InjuryStatus::PUP => write!(f, "PUP"),
            InjuryStatus::Questionable => write!(f, "Questionable"),
            InjuryStatus::Suspended => write!(f, "Suspended"),
        }
    }
}

impl SleeperSport {
    pub fn from_str(str: &str) -> Result<SleeperSport, SleeperError> {
        match str.to_lowercase().as_str() {
            "nfl" => Ok(SleeperSport::NFL),
            "nba" => Ok(SleeperSport::NBA),
            "lcs" => Ok(SleeperSport::LCS),
            unmatched => Err(SleeperError::InvalidSport(unmatched.to_string())),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            SleeperSport::NFL => String::from("nfl"),
            SleeperSport::NBA => String::from("nba"),
            SleeperSport::LCS => String::from("lcs"),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SportState {
    pub week: u8,
    pub season_type: String,
    pub season_start_date: String, // TODO this is a date in fmt YYYY-MM-DD
    pub season: String,            // TODO this is year in fmt YYYY
    pub previous_season: String,   // TODO this is year in fmt YYYY
    pub leg: u8,
    pub league_season: String,
    pub league_create_season: String,
    pub display_week: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NflPlayer {
    pub birth_city: Option<String>,
    pub status: Option<String>,
    pub sportradar_id: Option<String>, // UUID
    pub search_first_name: Option<String>,
    pub practice_description: Option<String>,
    pub injury_body_part: Option<String>,
    pub injury_status: Option<String>,
    pub team: Option<String>,
    pub position: Option<String>,
    pub metadata: Option<HashMap<String, Value>>,
    pub first_name: String,
    pub injury_start_date: Option<String>,
    pub high_school: Option<String>,
    pub injury_notes: Option<String>,
    pub yahoo_id: Option<i32>,
    pub birth_date: Option<String>,
    pub age: Option<u8>,
    pub pandascore_id: Option<String>,
    pub sport: String,
    pub practice_participation: Option<String>,
    pub weight: Option<String>,
    pub swish_id: Option<u32>,
    pub depth_chart_order: Option<u8>,
    pub years_exp: Option<i8>,
    pub college: Option<String>,
    pub news_updated: Option<u64>,
    pub rotoworld_id: Option<u64>,
    pub last_name: String,
    pub birth_state: Option<String>,
    pub fantasy_positions: Option<Vec<String>>,
    pub birth_country: Option<String>,
    pub search_full_name: Option<String>,
    pub stats_id: Option<u32>,
    pub gsis_id: Option<String>,
    pub depth_chart_position: Option<String>,
    pub full_name: Option<String>,
    pub rotowire_id: Option<u32>,
    pub espn_id: Option<u64>,
    pub fantasy_data_id: Option<u64>,
    pub active: bool,
    pub number: Option<u8>,
    pub player_id: PlayerId,
    pub hashtag: Option<String>,
    pub search_last_name: Option<String>,
    pub search_rank: Option<u64>,
    pub height: Option<String>,
}

pub enum AllPlayers {
    NFL(HashMap<PlayerId, NflPlayer>),
    // TODO
    LCS(HashMap<PlayerId, Value>),
    // TODO
    NBA(HashMap<PlayerId, Value>),
}

#[derive(Error, Debug)]
pub enum SleeperError {
    #[error("could not deserialize Sleeper response into provided type")]
    DeserializationError(String),

    #[error("request to Sleeper failed")]
    NetworkError(Option<http::StatusCode>),

    #[error("could parse String into SleeperSport: \"{0}\" was not a valid sport")]
    InvalidSport(String),

    #[error("could parse String into PlayerStatus: \"{0}\" was not a valid injury designation")]
    InvalidInjuryStatus(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_league_from_json_succeeds() {
        let json_str = r#"
        {
            "name": "League name",
            "status": "complete",
            "metadata": {
                "auto_continue": "on",
                "continued": "yes",
                "keeper_deadline": "0",
                "latest_league_winner_roster_id": "2"
            },
            "settings": {
                "best_ball": 0,
                "last_report": 14,
                "waiver_budget": 100,
                "disable_adds": 0,
                "capacity_override": 0,
                "taxi_deadline": 0,
                "draft_rounds": 3,
                "reserve_allow_na": 0,
                "start_week": 1,
                "playoff_seed_type": 0,
                "playoff_teams": 6,
                "veto_votes_needed": 5,
                "squads": 1,
                "num_teams": 10,
                "daily_waivers_hour": 0,
                "playoff_type": 0,
                "taxi_slots": 0,
                "last_scored_leg": 17,
                "daily_waivers_days": 1093,
                "playoff_week_start": 15,
                "waiver_clear_days": 2,
                "reserve_allow_doubtful": 0,
                "commissioner_direct_invite": 0,
                "veto_auto_poll": 0,
                "reserve_allow_dnr": 0,
                "taxi_allow_vets": 0,
                "waiver_day_of_week": 2,
                "playoff_round_type": 0,
                "reserve_allow_out": 1,
                "reserve_allow_sus": 0,
                "veto_show_votes": 0,
                "trade_deadline": 99,
                "taxi_years": 0,
                "daily_waivers": 0,
                "disable_trades": 0,
                "pick_trading": 1,
                "type": 1,
                "max_keepers": 3,
                "waiver_type": 0,
                "league_average_match": 0,
                "trade_review_days": 3,
                "bench_lock": 1,
                "offseason_adds": 1,
                "leg": 17,
                "reserve_slots": 1,
                "reserve_allow_cov": 1,
                "daily_waivers_last_ran": 27
            },
              "avatar": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
              "company_id": null,
              "shard": 434,
              "season": "2023",
              "season_type": "regular",
              "sport": "nfl",
              "scoring_settings": {
                "sack": 1,
                "fgm_40_49": 4,
                "pass_int": -1,
                "pts_allow_0": 10,
                "pass_2pt": 2,
                "st_td": 6,
                "rec_td": 6,
                "fgm_30_39": 3,
                "xpmiss": 0,
                "rush_td": 6,
                "def_pr_td": 0,
                "rec_2pt": 2,
                "st_fum_rec": 2,
                "fgmiss": 0,
                "ff": 0,
                "rec": 0,
                "pts_allow_14_20": 1,
                "fgm_0_19": 3,
                "def_kr_td": 0,
                "int": 2,
                "def_st_fum_rec": 2,
                "fum_lost": -2,
                "pts_allow_1_6": 7,
                "fgm_20_29": 3,
                "pts_allow_21_27": 0,
                "xpm": 1,
                "rush_2pt": 2,
                "fum_rec": 2,
                "def_st_td": 6,
                "fgm_50p": 5,
                "def_td": 6,
                "safe": 2,
                "pass_yd": 0.03999999910593033,
                "blk_kick": 2,
                "pass_td": 4,
                "rush_yd": 0.10000000149011612,
                "fum": 0,
                "pts_allow_28_34": -1,
                "pts_allow_35p": -4,
                "fum_rec_td": 6,
                "rec_yd": 0.10000000149011612,
                "def_st_ff": 0,
                "pts_allow_7_13": 4,
                "st_ff": 0
              },
              "last_message_id": "1111111111111111111",
              "last_author_avatar": null,
              "last_author_display_name": "sys",
              "last_author_id": "666666666666666666",
              "last_author_is_bot": true,
              "last_message_attachment": null,
              "last_message_text_map": null,
              "last_message_time": 1704267066920,
              "last_pinned_message_id": "555555555555555555",
              "last_read_id": null,
              "draft_id": "888888888888888888",
              "league_id": "999999999999999999",
              "previous_league_id": "777777777777777777",
              "bracket_id": 1111111111111111111,
              "group_id": null,
              "roster_positions": [
                "QB",
                "RB",
                "RB",
                "WR",
                "WR",
                "WR",
                "TE",
                "FLEX",
                "FLEX",
                "K",
                "DEF",
                "BN",
                "BN",
                "BN",
                "BN",
                "BN",
                "BN",
                "BN"
              ],
              "loser_bracket_id": 1111111111111111111,
              "total_rosters": 10
        }"#;

        let result: Result<League, _> = serde_json::from_str(json_str);
        assert_eq!(result.is_ok(), true);
        let league = result.unwrap();
        assert_eq!(league.name, "League name");
        assert_eq!(league.status, "complete");
        assert_eq!(league.settings.best_ball, 0);
        assert_eq!(league.settings.last_report, 14);
        assert_eq!(league.settings.waiver_budget, 100);
        assert_eq!(league.settings.disable_adds, 0);
        assert_eq!(league.settings.capacity_override, 0);
        assert_eq!(league.settings.taxi_deadline, 0);
        assert_eq!(league.settings.draft_rounds, 3);
        assert_eq!(league.settings.reserve_allow_na, 0);
        assert_eq!(league.settings.start_week, 1);
        assert_eq!(league.settings.playoff_seed_type, 0);
        assert_eq!(league.settings.playoff_teams, 6);
        assert_eq!(league.settings.veto_votes_needed, 5);
        assert_eq!(league.settings.squads, 1);
        assert_eq!(league.settings.num_teams, 10);
        assert_eq!(league.settings.daily_waivers_hour, 0);
        assert_eq!(league.settings.playoff_type, 0);
        assert_eq!(league.avatar, "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
        assert_eq!(league.company_id, None);
        assert_eq!(league.shard, 434);
        assert_eq!(league.season, "2023");
        assert_eq!(league.season_type, "regular");
        assert_eq!(league.sport, "nfl");
        assert_eq!(league.scoring_settings.sack, 1.0);
        assert_eq!(league.scoring_settings.fgm_40_49, 4.0);
        assert_eq!(league.scoring_settings.pass_int, -1.0);
        assert_eq!(league.scoring_settings.pts_allow_0, 10.0);
        assert_eq!(league.scoring_settings.pass_2pt, 2.0);
        assert_eq!(league.scoring_settings.st_td, 6.0);
        assert_eq!(league.scoring_settings.rec_td, 6.0);
        assert_eq!(league.scoring_settings.fgm_30_39, 3.0);
        assert_eq!(league.scoring_settings.xpmiss, 0.0);
        assert_eq!(league.scoring_settings.rush_td, 6.0);
        assert_eq!(league.scoring_settings.def_pr_td, 0.0);
        assert_eq!(league.scoring_settings.rec_2pt, 2.0);
    }

    #[test]
    fn test_deserialize_league_from_json_deserialization_error() {
        let json_str = r#"{ "invalid_field": null }"#;

        let result: Result<League, _> = serde_json::from_str(json_str);
        assert_eq!(result.is_err(), true);
    }
}
