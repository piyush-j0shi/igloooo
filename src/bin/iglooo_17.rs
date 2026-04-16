use ::std::collections::HashMap;

#[derive(Debug)]
enum TournamentFormat {
    SingleElemination,
    DoubleElemination,
}

#[derive(Debug)]
enum TournamentState {
    NotStarted,
    InProgress,
    Completed,
}

#[derive(Debug, PartialEq)]
enum MatchState {
    Scheduled,
    ongoing,
    Completed,
}

#[derive(Debug)]
struct Player {
    id: u32,
    name: String,
}

#[derive(Debug)]
struct Match {
    match_id: u32,
    players: (u32, u32),
    scores: (u32, u32),
    state: MatchState,
}

#[derive(Debug)]
struct Round {
    round_number: u32,
    matches: Vec<Match>,
}

#[derive(Debug)]
struct PlayerStats {
    wins: u32,
    losses: u32,
    points_scored: u32,
}

#[derive(Debug)]
struct Tournament {
    id: u32,
    name: String,
    format: TournamentFormat,
    state: TournamentState,
    rounds: Vec<Round>,
    players: Vec<Player>,
    stats: HashMap<u32, PlayerStats>,
}

impl Tournament {
    fn new(name: &str, tournament_format: TournamentFormat, total_players: Vec<Player>) -> Self {
        let mut stats_map = HashMap::new();

        for players in &total_players {
            let stat_player1 = PlayerStats {
                wins: 0,
                losses: 0,
                points_scored: 0,
            };

            stats_map.insert(players.id, stat_player1);
        }

        // stats_map.insert(total_players[0].id, stat_player1);
        // stats_map.insert(total_players[1].id, stat_player2);
        // stats_map.insert(total_players[2].id, stat_player3);
        // stats_map.insert(total_players[3].id, stat_player4);

        Self {
            id: 1,
            name: name.to_string(),
            format: tournament_format,
            state: TournamentState::NotStarted,
            rounds: vec![],
            players: total_players,
            stats: stats_map,
        }
    }

    fn start(&mut self) {
        match self.state {
            TournamentState::NotStarted => self.state = TournamentState::InProgress,
            TournamentState::Completed => println!("tournament already completed"),
            TournamentState::InProgress => println!("tournament already in progress"),
        }

        let mut matches: Vec<Match> = vec![];
        let mut match_id = 1;

        for i in (0..self.players.len()).step_by(2) {
            if i + 1 < self.players.len() {
                let p1_id = self.players[i].id;
                let p2_id = self.players[i + 1].id;

                matches.push(Match {
                    match_id,
                    players: (p1_id, p2_id),
                    scores: (0, 0),
                    state: MatchState::Scheduled,
                });
                match_id += 1;
            } else {
                println!("player {} gets bye", self.players[i].id);
            }
        }

        let round_1 = Round {
            round_number: 1,
            matches: matches,
        };

        self.rounds = vec![round_1];
    }

    fn match_result(&mut self, match_id: u32, scores1: u32, score2: u32) {
        for rounds in &mut self.rounds {
            if let Some(matches) = rounds.matches.iter_mut().find(|x| x.match_id == match_id) {
                matches.scores.0 = scores1;
                matches.scores.1 = score2;
                matches.state = MatchState::Completed;

                println!("match is : {:#?}", matches);

                let player1 = matches.players.0;
                let player2 = matches.players.1;

                if scores1 > score2 {
                    if let Some(playerstate) = self.stats.get_mut(&player1) {
                        playerstate.wins += 1;
                        playerstate.losses += 0;
                        playerstate.points_scored += scores1;
                    }

                    if let Some(playerstate2) = self.stats.get_mut(&player2) {
                        playerstate2.wins += 0;
                        playerstate2.losses += 1;
                        playerstate2.points_scored += score2;
                    }
                } else {
                    if let Some(playerstate) = self.stats.get_mut(&player2) {
                        playerstate.wins += 1;
                        playerstate.losses += 0;
                        playerstate.points_scored += score2;
                    }

                    if let Some(playerstate2) = self.stats.get_mut(&player1) {
                        playerstate2.wins += 0;
                        playerstate2.losses += 1;
                        playerstate2.points_scored += scores1;
                    }
                }
            }
        }
    }

    fn next_round(&mut self) {
        if let Some(last_round) = self.rounds.last() {
            if !last_round
                .matches
                .iter()
                .all(|m| m.state == MatchState::Completed)
            {
                println!("Not all matches are completed yet!");
                return;
            }
        } else {
            println!("No rounds yet, start the tournament first!");
            return;
        }

        let last_round = self.rounds.last().unwrap();

        let mut winners: Vec<u32> = vec![];
        for m in &last_round.matches {
            let (p1, p2) = m.players;
            let (s1, s2) = m.scores;

            if s1 > s2 {
                winners.push(p1);
            } else {
                winners.push(p2);
            }
        }

        if winners.len() == 1 {
            println!("Tournament Winner is player {}", winners[0]);
            self.state = TournamentState::Completed;
            return;
        }

        let mut matches: Vec<Match> = vec![];
        let mut match_id = self.rounds.iter().map(|r| r.matches.len()).sum::<usize>() as u32 + 1;

        let mut i = 0;
        while i < winners.len() {
            if i + 1 < winners.len() {
                matches.push(Match {
                    match_id,
                    players: (winners[i], winners[i + 1]),
                    scores: (0, 0),
                    state: MatchState::Scheduled,
                });
                match_id += 1;
                i += 2;
            } else {
                println!("Player {} gets a bye and advances!", winners[i]);
                matches.push(Match {
                    match_id,
                    players: (winners[i], winners[i]),
                    scores: (1, 0),
                    state: MatchState::Completed,
                });
                match_id += 1;
                i += 1;
            }
        }

        let round_num = (self.rounds.len() + 1) as u32;
        let new_round = Round {
            round_number: round_num,
            matches,
        };

        self.rounds.push(new_round);
    }
}

fn main() {
    let players = vec![
        Player {
            id: 1,
            name: "player1".to_string(),
        },
        Player {
            id: 2,
            name: "player2".to_string(),
        },
        Player {
            id: 3,
            name: "player3".to_string(),
        },
        Player {
            id: 4,
            name: "player4".to_string(),
        },
    ];

    let mut new_tournament = Tournament::new(
        "first tournament",
        TournamentFormat::SingleElemination,
        players,
    );

    println!("tournament information : {:#?}", new_tournament);
    new_tournament.start();
    println!("tournament information : {:#?}", new_tournament);

    new_tournament.match_result(1, 10, 20);
    new_tournament.match_result(2, 10, 20);
    println!("after result match : {:#?}", new_tournament);

    new_tournament.next_round();
}
