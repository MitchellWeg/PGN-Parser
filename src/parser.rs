use std::io::BufReader;
use std::fs::File;
use serde::Serialize;
use std::io::prelude::*;

#[derive(Serialize, Clone)]
pub struct PGN {
    pub event: String,
    pub site: String,
    pub date: String,
    pub white: String,
    pub black: String,
    pub game_result: String,
    pub white_elo: String,
    pub black_elo: String,
    pub time_control: String,
    pub termination: String,
    pub moves: String
}

impl Default for PGN {
    fn default() -> PGN {
        PGN {
            event: "".to_string(),
            site: "".to_string(),
            date: "".to_string(),
            white: "".to_string(),
            black: "".to_string(),
            game_result: "".to_string(),
            white_elo: "".to_string(),
            black_elo: "".to_string(),
            time_control: "".to_string(),
            termination: "".to_string(),
            moves: "".to_string(),
        }
    }
}

/*
 * Parse the file.
 * Returns: a vector of PGN's.
 * Note: This should probably be refactored.
 * This now holds all PGN's in memory, which is not very efficient,
 * since it effectively doubles in memory.
 * In the future this should probably just return single PGN's,
 * which immediately should be written to the file.
 */
pub fn parse_file(file: File) -> Vec<PGN> {
    let reader = BufReader::new(file);
    let lines: Vec<_> = reader.lines().into_iter().collect();

    parse_lines(lines)
}

fn parse_lines(lines: Vec<Result<String, std::io::Error>>) -> Vec<PGN> {
    let line_count = lines.len();
    let mut out: Vec<PGN> = Vec::new();
    let mut pgn = PGN::default();
    let mut whitespace_found: bool = false;
    let mut moves_written: bool = false;

    for (i, line) in lines.into_iter().enumerate() {
        let _line = line.unwrap();
        let stripped = strip_line(_line.clone());

        if moves_written {
            assert!(_line.clone().is_empty());
            whitespace_found = false;
            moves_written = false;
            out.push(pgn.clone());
            pgn = PGN::default();
            continue;
        }

        if whitespace_found {
            pgn.moves = _line.clone();
            moves_written = true;

            if i >= line_count {
                out.push(pgn.clone());
            }

            continue;
        }

        if _line.clone().is_empty() {
           whitespace_found = true;
           continue;
        }

        if !whitespace_found { 
            let split = stripped.split(' ').collect::<Vec<&str>>();

            let target: String = split[0].chars().filter(|c| !c.is_whitespace()).collect();

            match target.as_str() {
                "Event"=> pgn.event = get_value(split),
                "Site"=> pgn.site = get_value(split),
                "UTCDate"=> pgn.date = get_value(split),
                "White"=> pgn.white = get_value(split),
                "Black"=> pgn.black = get_value(split),
                "Result"=> pgn.game_result = get_value(split),
                "WhiteElo"=> pgn.white_elo = get_value(split),
                "BlackElo"=> pgn.black_elo = get_value(split),
                "TimeControl"=> pgn.time_control = get_value(split),
                "Termination"=> pgn.termination = get_value(split) ,

                _ => ()
            }
        }
    }

    out
}

fn get_value(split: Vec<&str>) -> String {
    split[1..].into_iter().map(|x| x.to_string() + " ").collect::<String>()
}

fn strip_line(line: String) -> String {
    let s = line.replace(&['[', ']', '"'][..], "");
    return s.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn it_strips_correctly() {
        let target = strip_line("[White \"Fischer, Robert J.\"]".to_string());
        let right_output = "White Fischer, Robert J.";
        assert_eq!(target, right_output);
    }

    #[test]
    fn it_gets_the_value_correctly() {
        let target = strip_line("[White \"Fischer, Robert J.\"]".to_string());
        let right_output = "White Fischer, Robert J.";
        assert_eq!(target, right_output);
    }

    #[test]
    fn it_parse_a_pgn_correctly() {
        let test_file = format!("{}/test/pgn.pgn", std::env::current_dir().unwrap().to_str().unwrap());

        let handle = match File::open(test_file) {
            Ok(s) => s,
            Err(e) => {panic!("{}", e)}
        };

        let pgns = parse_file(handle);

        assert_eq!(pgns.len(), 1);
        assert_eq!(pgns[0].white, "Fischer, Robert J. ".to_string());
        assert_eq!(pgns[0].black, "Spassky, Boris V. ".to_string());
        assert_eq!(pgns[0].moves, "1.e4 e5 2.Nf3 Nc6 3.Bb5 {Deze opening wordt Spaans genoemd.} a6 4.Ba4 Nf6 5.O-O Be7 6.Re1 b5 7.Bb3 d6 8.c3 O-O 9. h3 Nb8 10.d4 Nbd7 11.c4 c6 12.cxb5 axb5 13.Nc3 Bb7 14.Bg5 b4 15.Nb1 h6 16.Bh4 c5 17.dxe5 Nxe4 18.Bxe7 Qxe7 19.exd6 Qf6 20.Nbd2 Nxd6 21.Nc4 Nxc4 22.Bxc4 Nb6 23.Ne5 Rae8 24.Bxf7+ Rxf7 25.Nxf7 Rxe1+ 26.Qxe1 Kxf7 27.Qe3 Qg5 28.Qxg5 hxg5 29.b3 Ke6 30.a3 Kd6 31.axb4 cxb4 32.Ra5 Nd5 33. f3 Bc8 34.Kf2 Bf5 35.Ra7 g6 36.Ra6+ Kc5 37.Ke1 Nf4 38.g3 Nxh3 39.Kd2 Kb5 40.Rd6 Kc5 41.Ra6 Nf2 42.g4 Bd3 43.Re6 1/2-1/2".to_string());
    }

    #[test]
    fn it_parses_multiple_pgns_correctly() {
        let test_file = format!("{}/test/pgns.pgn", std::env::current_dir().unwrap().to_str().unwrap());

        let handle = match File::open(test_file) {
            Ok(s) => s,
            Err(e) => {panic!("{}", e)}
        };

        let pgns = parse_file(handle);

        assert_eq!(pgns.len(), 2);
        assert_eq!(pgns[0].white, "Robert James Fischer ".to_string());
        assert_eq!(pgns[0].black, "Pal Benko ".to_string());
        assert_eq!(pgns[0].moves, "1. e4 c5 2. Nf3 Nc6 3. d4 cxd4 4. Nxd4 Nf6 5. Nc3 d6 6. Bc4 Qb6 7. Nde2 e6 8. O-O Be7 9. Bb3 O-O 10. Kh1 Na5 11. Bg5 Qc5 12. f4 b5 13. Ng3 b4 14. e5 dxe5 15. Bxf6 gxf6 16. Nce4 Qd4 17. Qh5 Nxb3 18. Qh6 exf4 19. Nh5 f5 20. Rad1 Qe5 21. Nef6+ Bxf6 22. Nxf6+ Qxf6 23. Qxf6 Nc5 24. Qg5+ Kh8 25. Qe7 Ba6 26. Qxc5 Bxf1 27. Rxf1 1-0".to_string());
    }

}
