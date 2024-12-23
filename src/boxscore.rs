use serde::{Serialize, Deserialize};

/// # BoxScore struct
///
/// A `BoxScore` represents the result of a football game at the highest level.
/// It simply describes the date of the football game, the names of the home
/// and away teams involved in the game, and the scores of each respective
/// team.
#[derive(Serialize, Deserialize)]
pub struct BoxScore {
    date: String,
    away_team: String,
    away_score: i32,
    home_team: String,
    home_score: i32
}

impl BoxScore {
    /// Constructor for the BoxScore struct in which the struct properties are
    /// given as function arguments, and the initialized struct is returned
    ///
    /// ### Example
    /// ```
    /// use fbdb_webscrape::BoxScore;
    ///
    /// let my_date: String = String::from("12/1/2023");
    /// let my_away_team: String = String::from("Team A");
    /// let my_away_score: i32 = 28_i32;
    /// let my_home_team: String = String::from("Team B");
    /// let my_home_score: i32 = 31_i32;
    /// let my_box_score: BoxScore = BoxScore::from_properties(
    ///     my_date,
    ///     my_away_team,
    ///     my_away_score,
    ///     my_home_team,
    ///     my_home_score
    /// );
    /// ```
    pub fn from_properties(date: String, away_team: String, away_score: i32, home_team: String, home_score: i32) -> BoxScore {
        BoxScore{
            date: date,
            away_team: away_team,
            away_score: away_score,
            home_team: home_team,
            home_score: home_score
        }
    }
}

impl std::fmt::Display for BoxScore {
    /// Format a `BoxScore` as a string.
    ///
    /// ### Example
    ///
    /// ```
    /// println!("{}", my_box_score);
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let score_str = format!(
            "[{}] {} {} - {} {}",
            self.date,
            self.home_team,
            self.home_score,
            self.away_team,
            self.away_score
        );
        f.write_str(&score_str)
    }
}
