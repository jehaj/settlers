use rand::Rng;

pub trait Game {
    /// When you are done, give the turn to the next player.
    /// Ending your turn makes the player's, who is now in turn,
    /// development cards become active. You must have rolled
    /// the dice before you can end your turn.
    fn end_turn(&mut self);
    /// Roll the two dice. Output is from 2 to 14. This is
    /// done when it becomes your turn. You have to do it
    /// before ending your turn and it can only be done once.
    fn roll_dice_and_distribute(&mut self);
    /// Add city to the map. The rules for placing this
    /// are the same as for the settlement.
    fn add_city(&mut self);
    /// Add settlement to the map. It is not allowed if
    /// there is no road up to it or it does not respect
    /// the housing rules (two or more roads between other
    /// settlements/cities)
    fn add_settlement(&mut self);
    /// Add a road to the map. You cannot place a road
    /// if there are no roads/settlement/city up to it.
    fn add_road(&mut self);
    /// Get the player in turn.
    fn player_in_turn(&self) -> usize;
    /// Move the robber to a hex.
    fn move_robber(&mut self);
    /// Get one of the 6 adjacent hexes
    /// Imagine an analog clock: 0 is the hex at 1, 1 is hex at 3 and so on.
    fn get_adjacent_hex(&self, pos: usize);
}

pub trait MutableGame: Game {

}

pub struct GameImpl {
    turn_counter: usize,
    number_of_players: usize
}

impl GameImpl {
    fn new() -> GameImpl {
        GameImpl { turn_counter: 0, number_of_players: 3 }
    }

    fn roll_dice(&self) -> usize {
        rand::thread_rng().gen_range(2..=14)
    }
}

impl Game for GameImpl {
    fn end_turn(&mut self) {
        self.turn_counter += 1;
    }

    fn roll_dice_and_distribute(&mut self) {
        todo!()
    }

    fn add_city(&mut self) {
        todo!()
    }

    fn add_settlement(&mut self) {
        todo!()
    }

    fn add_road(&mut self) {
        todo!()
    }

    fn player_in_turn(&self) -> usize {
        self.turn_counter % self.number_of_players
    }

    fn move_robber(&mut self) {
        todo!()
    }

    fn get_adjacent_hex(&self, pos: usize) {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn should_be_player_0_turn_at_start() {
        let game = GameImpl::new();
        let player_in_turn = game.player_in_turn();
        assert_eq!(player_in_turn, 0);
    }

    #[test]
    fn should_be_player_1_turn_after_ending_turn() {
        let mut game = GameImpl::new();
        game.end_turn();
        let player_in_turn = game.player_in_turn();
        assert_eq!(player_in_turn, 1);
    }

    #[test]
    fn should_be_between_2_and_14() {
        let mut game = GameImpl::new();
        let dice_eyes = game.roll_dice();
        let between = 2 <= dice_eyes && dice_eyes <= 14;
        assert_eq!(between, true);
    }
}
