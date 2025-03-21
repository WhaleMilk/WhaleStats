use crate::player::Player;
use crate::intake::data_filter::{FilteredData, Position, Side};

#[derive(Debug)]
pub struct GameStatistics{
    puuid: String,
    pub game_start: i64,
    pub position: Position,
    pub champion: String,
    pub gd15: i32,
    pub csm: f32,
    pub dpm: f32,
    pub kp: f32,
    pub wl: bool
}

impl Player {
    pub fn process(&self, game: &FilteredData) -> GameStatistics {
        let pos = game.me.pos.clone();
        let champ = game.me.champ.clone();
        let side = game.me.side.clone();
        return GameStatistics {
            puuid: self.start.puuid.clone(),
            game_start: game.game_start,
            gd15: self.calc_gd(game, &pos, &side),
            csm: self.find_csm(game, &pos, &side),
            dpm: self.find_dpm(game, &pos, &side),
            kp: self.find_kp(game, &pos, &side),
            wl: self.find_wl(game, &side),
            position: pos, //NOTE: Support is referred to as "UTILITY"
            champion: champ,
        };
    }

    fn calc_gd(&self, game: &FilteredData, pos: &Position, side: &Side) -> i32 {
        let i = Self::get_index_from_pos(pos);

        //println!("@Analysis/calc_gd g1, g2 = {}, {}", game.g15[i].0, game.g15[i].1);

        let side = match side {
            Side::BLUE => game.g15[i].0 - game.g15[i].1,
            Side::RED => game.g15[i].1 - game.g15[i].0,
        };

        return side
    }

    fn find_csm(&self, game: &FilteredData, pos: &Position, side: &Side) -> f32 {
        let i = Self::get_index_from_pos(pos);

        let side = match side {
            Side::BLUE => game.csm[i].0,
            Side::RED => game.csm[i].1,
        };

        return side;
    }

    fn find_dpm(&self, game: &FilteredData, pos: &Position, side: &Side) -> f32 {
        let i = Self::get_index_from_pos(pos);

        let side = match side {
            Side::BLUE => game.dpm[i].0,
            Side::RED => game.dpm[i].1,
        };

        return side;
    }

    fn find_kp(&self, game: &FilteredData, pos: &Position, side: &Side) -> f32 {
        let i = Self::get_index_from_pos(pos);

        let side = match side {
            Side::BLUE => game.kp[i].0,
            Side::RED => game.kp[i].1,
        };

        return side;
    }

    fn find_wl(&self, game: &FilteredData, side: &Side) -> bool {
        let side = match side {
            Side::BLUE => game.win_loss.0,
            Side::RED => game.win_loss.1,
        };

        return side;
    }

    fn get_index_from_pos(pos: &Position) -> usize {
        return match pos {
            Position::TOP => 0,
            Position::JUNGLE => 1,
            Position::MIDDLE => 2,
            Position::BOTTOM => 3,
            Position::SUPPORT => 4
        };
    }

    pub fn pos_to_str(pos: &Position) -> String {
        return match pos {
            Position::TOP => String::from("Top"),
            Position::JUNGLE => String::from("Jungle"),
            Position::MIDDLE => String::from("Middle"),
            Position::BOTTOM => String::from("Bottom"),
            Position::SUPPORT => String::from("Support")
        };
    }
}