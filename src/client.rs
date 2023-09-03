use std::collections::HashMap;
use crate::data::*;

use serde_json::Value;

const BASE_URL: &str = "https://api.sleeper.app/v1";

pub struct Client {
    client: reqwest::Client
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
    pub async fn get_all_players(&self, sport: SleeperSport) -> Result<HashMap<PlayerId, Value>, SleeperError> {

        fn parse_into_player_map(players_unparsed: &str) -> Result<HashMap<PlayerId, Value>, SleeperError> {
            let mut result: HashMap<String, Value> = HashMap::new();
            let parsed_node: serde_json::Value = serde_json::from_str(players_unparsed).unwrap();

            if let Value::Object(map) = parsed_node {
                for (key, value) in map {
                    // match value { } // TODO - could drill down on parsing a little deeper here
                    result.insert(key, value);
                }
            };

            Ok(result)
        }

        let url = format!("{}/players/{}", BASE_URL, &sport.to_string());
        
        let res = match self.client.get(&url).send().await {
            Ok(res) => res,
            Err(e) => return Err(SleeperError::NetworkError(e.status()))
        };

        // TODO - revisit type
        match res.text_with_charset("utf-8").await {
            Ok(p) => parse_into_player_map(&p),
            Err(_) => Err(SleeperError::DeserializationError(String::from("String"))) // TODO lol
        }
    }
}

