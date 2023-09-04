use std::collections::HashMap;
use crate::data::*;

use serde_json::Value;

const BASE_URL: &str = "https://api.sleeper.app/v1";

pub struct Client {
    client: reqwest::Client
}

pub enum AllPlayers {
    NFL(HashMap<PlayerId, NflPlayer>),
    // TODO
    LCS(HashMap<PlayerId, Value>),
    // TODO
    NBA(HashMap<PlayerId, Value>),
}

impl Client {
    pub fn new() -> Self {
        Client {
            client: reqwest::Client::builder()
                .https_only(true)
                .build()
                .unwrap()
        }
    }

    pub async fn get_league(&self, id: &str) -> Result<League, SleeperError> {
        let url = format!("{}/league/{}", BASE_URL, &id);

        let res = match self.client.get(&url).send().await {
            Ok(res) => res,
            Err(e) => return Err(SleeperError::NetworkError(e.status()))
        };

        let league: League = match res.json::<League>().await {
            Ok(league) => league,
            Err(_) => return Err(SleeperError::DeserializationError(String::from("League")))
        };

        Ok(league)
    }

    pub async fn get_rosters(&self, league_id: &str) -> Result<Vec<Roster>, SleeperError> {
        let url = format!("{}/league/{}/rosters", BASE_URL, &league_id);

        let res = match self.client.get(&url).send().await {
            Ok(res) => res,
            Err(e) => return Err(SleeperError::NetworkError(e.status()))
        };

        let rosters: Vec<Roster> = match res.json().await {
            Ok(rost) => rost,
            Err(_) => return Err(SleeperError::DeserializationError(String::from("Roster")))
        };

        Ok(rosters)
    }

    pub async fn get_users_in_league(&self, league_id: &str) -> Result<Vec<SleeperUser>, SleeperError> {
        let url = format!("{}/league/{}/users", BASE_URL, &league_id);

        let res = match self.client.get(&url).send().await {
            Ok(res) => res,
            Err(e) => return Err(SleeperError::NetworkError(e.status()))
        };

        let users: Vec<SleeperUser> = match res.json().await {
            Ok(users) => users,
            Err(_) => return Err(SleeperError::DeserializationError(String::from("SleeperUser")))
        };

        Ok(users)
    }

    pub async fn get_matchups(&self, league_id: &str, week: u8) -> Result<Vec<Matchup>, SleeperError> {
        let url = format!("{}/league/{}/matchups/{}", BASE_URL, &league_id, week);

        let res = match self.client.get(&url).send().await {
            Ok(res) => res,
            Err(e) => return Result::Err(SleeperError::NetworkError(e.status()))
        };

        let matchups: Vec<Matchup> = match res.json().await {
            Ok(m) => m,
            Err(_) => return Err(SleeperError::DeserializationError(String::from("Matchup")))
        };

        Ok(matchups)
    }

    pub async fn get_sport_state(&self, sport: SleeperSport) -> Result<SportState, SleeperError> {
        let url = format!("{}/state/{}", BASE_URL, &sport.to_string());

        let res = match self.client.get(&url).send().await {
            Ok(res) => res,
            Err(e) => return Result::Err(SleeperError::NetworkError(e.status()))
        };

        match res.json::<SportState>().await {
            Ok(st) => Ok(st),
            Err(_) => Err(SleeperError::DeserializationError(String::from("SportState")))
        }
    }

    // Be careful, it's thicccc
    pub async fn get_all_players(&self, sport: SleeperSport) -> Result<AllPlayers, SleeperError> {

        fn to_hashmap_nfl(players_unparsed: &str) -> Result<HashMap<PlayerId, NflPlayer>, SleeperError> {
            let parsed = match serde_json::from_str::<HashMap<PlayerId, NflPlayer>>(players_unparsed) {
                Ok(p) => p,
                Err(e) => {
                    panic!("Data model for all players is bad! {}", e);
                }
            };

            Ok(parsed)
        }

        let url = format!("{}/players/{}", BASE_URL, &sport.to_string());
        
        let res = match self.client.get(&url).send().await {
            Ok(res) => res,
            Err(e) => return Err(SleeperError::NetworkError(e.status()))
        };

        // TODO - revisit type
        match res.text_with_charset("utf-8").await {
            Ok(players_raw) => {
                match sport {
                    SleeperSport::NFL => {
                        if let Ok(result) = to_hashmap_nfl(players_raw.as_ref()) {
                            Ok(AllPlayers::NFL(result))
                        } else {
                            Err(SleeperError::DeserializationError("Error trying to deserialize from JSON raw string to HashMap<PlayerId, NflPlayer>".to_string()))
                        }
                    }
                    SleeperSport::LCS => todo!(),
                    SleeperSport::NBA => todo!(),
                }
            }
            Err(_) => Err(SleeperError::DeserializationError(String::from("String"))) // TODO lol
        }
    }

    /// Use this to request the 'get players' endpoint with the provided sport,
    ///   without parsing the resp body.
    pub async fn get_all_players_unparsed(&self, sport: SleeperSport) -> Result<Value, SleeperError> {
        let url = format!("{}/players/{}", BASE_URL, &sport.to_string());
        
        let res = match self.client.get(&url).send().await {
            Ok(res) => res,
            Err(e) => return Err(SleeperError::NetworkError(e.status()))
        };

        // TODO - revisit type
        match res.text_with_charset("utf-8").await {
            Ok(p) => Ok(serde_json::json!(p)),
            Err(_) => Err(SleeperError::DeserializationError(String::from("String"))) // TODO lol
        }
    }
}

