use std::collections::HashMap;

use crate::{
    data_processor::{Me, Position, RawData, Side}, 
    interface::{match_data::MatchData, timeline::{ParticipantFrames, Timeline}}
};

impl RawData {
    pub fn new(game_data: &MatchData, game_tl: &Timeline) -> RawData {
        RawData { 
            pids: Self::filter_pids(&game_data),
            me: Me::default(),
            g15: Self::filter_g15(&game_tl),
            csm: Self::filter_csm(&game_tl),
            dpm: Self::filter_dpm(&game_tl),
            kp: Self::filter_kp(&game_data),
            champs: Self::filter_champs(&game_data),
            win_loss: Self::find_wl(&game_data),
            game_end: game_data.info.game_end_timestamp }
    }

    pub fn find_me(&mut self, pid: &String) {
        for i in 0..5 {
            if &self.pids[i].0 == pid {
                self.me.side = Side::BLUE;
                self.me.pos = match i {
                    0 => Position::TOP,
                    1 => Position::JUNGLE,
                    2 => Position::MIDDLE,
                    3 => Position::BOTTOM,
                    4 => Position::SUPPORT,
                    _ => Position::default(),
                };
                self.me.champ = self.champs[i].0.clone();
            } else if &self.pids[i].1 == pid{
                self.me.side = Side::RED;
                self.me.pos = match i {
                    0 => Position::TOP,
                    1 => Position::JUNGLE,
                    2 => Position::MIDDLE,
                    3 => Position::BOTTOM,
                    4 => Position::SUPPORT,
                    _ => Position::default(),
                };
                self.me.champ = self.champs[i].1.clone();
            }
        }
    }

    fn filter_pids(game: &MatchData) -> [(String, String); 5] {
        let mut out: [(String, String); 5] = Default::default();
        for i in 0..5 {
            out[i] = (game.info.participants.get(i).unwrap().puuid.clone(), game.info.participants.get(i+5).unwrap().puuid.clone());
        }
        return out;
    }

    fn filter_champs(game: &MatchData) -> [(String, String); 5] {
        let mut out: [(String, String); 5] = Default::default();
        for i in 0..5 {
            out[i] = (game.info.participants.get(i).unwrap().champion_name.clone(), game.info.participants.get(i+5).unwrap().champion_name.clone());
        }
        return out;
    }

    fn filter_g15(game_tl: &Timeline) -> [(i32, i32); 5] {
        let mut out: [(i32, i32); 5] = [(0, 0); 5];
        for i in 0..5 {
            let (p1_gold, p2_gold) = Self::find_gold_in_frame(&game_tl.info.frames[15].participant_frames, i);
            out[i as usize] = (p1_gold, p2_gold);
        }
        return out;
    }

    fn filter_csm(game_tl: &Timeline) -> [(f32, f32); 5] {
        let mut out: [(f32, f32); 5] = [(0.0, 0.0); 5];
        let frame_count = game_tl.info.frames.len() - 2;
        for i in 0..5 {
            let(p1_cs, p2_cs) = Self::find_cs_in_frame(&game_tl.info.frames.get(frame_count).unwrap().participant_frames, i);
            out[i as usize] = (p1_cs / (frame_count as f32), p2_cs / (frame_count as f32));
        }
        
        return out;
    }

    fn filter_dpm(game_tl: &Timeline) -> [(f32, f32); 5] {
        let mut out: [(f32, f32); 5] = [(0.0, 0.0); 5];
        let frame_count = game_tl.info.frames.len() - 2;
        for i in 0..5 {
            let(p1_cs, p2_cs) = Self::find_damage_in_frame(&game_tl.info.frames.get(frame_count).unwrap().participant_frames, i);
            out[i as usize] = (p1_cs / (frame_count as f32), p2_cs / (frame_count as f32));
        }
        return out;
    }

    fn filter_kp(game: &MatchData) -> [(f32, f32); 5] {
        let mut out: [(f32, f32); 5] = [(0.0, 0.0); 5];
        let mut b_team_kills = 0;
        let mut r_team_kills = 0;
        let participants = &game.info.participants;
        
        for i in 0..5 {
            let j = i + 5;
            b_team_kills += participants.get(i).unwrap().kills;
            r_team_kills += participants.get(j).unwrap().kills;
        }

        for i in 0..5 {
            let p1_kp = ((participants.get(i).unwrap().kills + participants.get(i).unwrap().assists) as f32) / (b_team_kills as f32) * 100.0;
            let p2_kp = ((participants.get(i + 5).unwrap().kills + participants.get(i + 5).unwrap().assists) as f32) / (r_team_kills as f32) * 100.0;
            out[i] = (p1_kp, p2_kp);
        }
        return out;
    }

    fn find_wl(game: &MatchData) -> (bool, bool) {
        (game.info.teams.get(0).unwrap().win, game.info.teams.get(1).unwrap().win)
    }

    fn find_gold_in_frame(frame: &ParticipantFrames, p_index: i8) -> (i32, i32) {
        let mut frame_map = HashMap::new();
        let p_index2 = p_index + 5;

        frame_map.insert(0, frame.n1.total_gold);
        frame_map.insert(1, frame.n2.total_gold);
        frame_map.insert(2, frame.n3.total_gold);
        frame_map.insert(3, frame.n4.total_gold);
        frame_map.insert(4, frame.n5.total_gold);
        frame_map.insert(5, frame.n6.total_gold);
        frame_map.insert(6, frame.n7.total_gold);
        frame_map.insert(7, frame.n8.total_gold);
        frame_map.insert(8, frame.n9.total_gold);
        frame_map.insert(9, frame.n10.total_gold);

        let g1 = match frame_map.get(&p_index) {
            Some(&num) => num,
            _ => 0
        };
    
        let g2 = match frame_map.get(&p_index2) {
            Some(&num) => num,
            _ => 0
        };

        (g1 as i32, g2 as i32)
    }

    fn find_cs_in_frame(frame: &ParticipantFrames, p_index: i8) -> (f32, f32) {
        let mut frame_map: HashMap<i8, i64> = HashMap::new();
        let p_index2 = p_index + 5;

        frame_map.insert(0, frame.n1.minions_killed + frame.n1.jungle_minions_killed);
        frame_map.insert(1, frame.n2.minions_killed + frame.n2.jungle_minions_killed);
        frame_map.insert(2, frame.n3.minions_killed + frame.n3.jungle_minions_killed);
        frame_map.insert(3, frame.n4.minions_killed + frame.n4.jungle_minions_killed);
        frame_map.insert(4, frame.n5.minions_killed + frame.n5.jungle_minions_killed);
        frame_map.insert(5, frame.n6.minions_killed + frame.n6.jungle_minions_killed);
        frame_map.insert(6, frame.n7.minions_killed + frame.n7.jungle_minions_killed);
        frame_map.insert(7, frame.n8.minions_killed + frame.n8.jungle_minions_killed);
        frame_map.insert(8, frame.n9.minions_killed + frame.n9.jungle_minions_killed);
        frame_map.insert(9, frame.n10.minions_killed + frame.n10.jungle_minions_killed);

        let cs1 = match frame_map.get(&p_index) {
            Some(&num) => num,
            _ => 0
        };
    
        let cs2 = match frame_map.get(&p_index2) {
            Some(&num) => num,
            _ => 0
        };

        (cs1 as f32, cs2 as f32)
    }

    fn find_damage_in_frame(frame: &ParticipantFrames, p_index: i8) -> (f32, f32) {
        let mut frame_map: HashMap<i8, i64> = HashMap::new();
        let p_index2 = p_index + 5;

        frame_map.insert(0, frame.n1.damage_stats.total_damage_done_to_champions);
        frame_map.insert(1, frame.n2.damage_stats.total_damage_done_to_champions);
        frame_map.insert(2, frame.n3.damage_stats.total_damage_done_to_champions);
        frame_map.insert(3, frame.n4.damage_stats.total_damage_done_to_champions);
        frame_map.insert(4, frame.n5.damage_stats.total_damage_done_to_champions);
        frame_map.insert(5, frame.n6.damage_stats.total_damage_done_to_champions);
        frame_map.insert(6, frame.n7.damage_stats.total_damage_done_to_champions);
        frame_map.insert(7, frame.n8.damage_stats.total_damage_done_to_champions);
        frame_map.insert(8, frame.n9.damage_stats.total_damage_done_to_champions);
        frame_map.insert(9, frame.n10.damage_stats.total_damage_done_to_champions);

        let d1 = match frame_map.get(&p_index) {
            Some(&num) => num,
            _ => 0
        };
    
        let d2 = match frame_map.get(&p_index2) {
            Some(&num) => num,
            _ => 0
        };

        (d1 as f32, d2 as f32)
    }
}