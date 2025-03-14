// A list of scores (one per line) of a soccer match is given. Each line is of
// the form "<team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>"
// Example: "England,France,4,2" (England scored 4 goals, France 2).
//
// You have to build a scores table containing the name of the team, the total
// number of goals the team scored, and the total number of goals the team
// conceded.

use std::collections::HashMap;

// A structure to store the goal details of a team.
#[derive(Default)]
struct TeamScores {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: &str) -> HashMap<&str, TeamScores> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores = HashMap::new();

    for line in results.lines() {
        let mut split_iterator = line.split(',');
        // NOTE: We use `unwrap` because we didn't deal with error handling yet.
        let team_1_name = split_iterator.next().unwrap();
        let team_2_name = split_iterator.next().unwrap();
        let team_1_score: u8 = split_iterator.next().unwrap().parse().unwrap();
        let team_2_score: u8 = split_iterator.next().unwrap().parse().unwrap();

        // TODO: Populate the scores table with the extracted details.
        // Keep in mind that goals scored by team 1 will be the number of goals
        // conceded by team 2. Similarly, goals scored by team 2 will be the
        // number of goals conceded by team 1.
        let team_1_scores = scores.entry(team_1_name).or_insert(TeamScores::default());
        team_1_scores.goals_scored += team_1_score;
        team_1_scores.goals_conceded += team_2_score;
        let team_2_scores = scores.entry(team_2_name).or_insert(TeamScores::default());
        team_2_scores.goals_scored += team_2_score;
        team_2_scores.goals_conceded += team_1_score;
        /*
        SOLUTION:
        // Insert the default with zeros if a team doesn't exist yet.
        let team_1 = scores
            .entry(team_1_name)
            .or_insert_with(TeamScores::default);
        // Update the values.
        team_1.goals_scored += team_1_score;
        team_1.goals_conceded += team_2_score;

        // Similarly for the second team.
        let team_2 = scores
            .entry(team_2_name)
            .or_insert_with(TeamScores::default);
        team_2.goals_scored += team_2_score;
        team_2.goals_conceded += team_1_score;

        NOTE:
        - or_insert: This method takes a value directly and inserts it if the key does not exist. The value is always evaluated, even if it is not inserted.
        - or_insert_with: This method takes a closure that generates the value. The closure is only evaluated if the key does not exist, which can be more efficient if the value is expensive to create.
         */
    }

    scores
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    const RESULTS: &str = "England,France,4,2
France,Italy,3,1
Poland,Spain,2,0
Germany,England,2,1
England,Spain,1,0";

    #[test]
    fn build_scores() {
        let scores = build_scores_table(RESULTS);

        assert!(["England", "France", "Germany", "Italy", "Poland", "Spain"]
            .into_iter()
            .all(|team_name| scores.contains_key(team_name)));
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 6);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 3);
    }
}
