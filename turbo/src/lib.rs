// Define the game configuration using the turbo::cfg! macro
turbo::cfg! {r#"
    name = "Tale of kentridge"
    version = "1.0.0"
    author = "DDX510"
    description = "Tale of kentridge"
    [settings]
    resolution = [256, 144]
    [solana]
    http-rpc-url = "http://127.0.0.1:8899"
    ws-rpc-url = "ws://127.0.0.1:8900"
"#}

struct Race {
    id: i32,
    name: &'static str,
}

struct Card {
    id: i32,
    mana: i32,
    hp: i32,
    attack: i32,
    curr_hp: i32,
    name: &'static str,
    race: i32,
    rarity: i32,
}

const RACES: [Race; 3] = [
    Race { id: 0, name: "Royal"},
    Race { id: 1, name: "Humanoid"},
    Race { id: 2, name: "Undead"},
];

const ROYAL_DECK: [Card; 30] = [
    Card { id: 0, mana: 3, hp: 3, attack: 1, curr_hp: 0, name: "royal_knight", race: 0, rarity: 0},
    Card { id: 1, mana: 3, hp: 3, attack: 1, curr_hp: 0, name: "royal_knight", race: 0, rarity: 0},
    Card { id: 2, mana: 3, hp: 3, attack: 1, curr_hp: 0, name: "royal_knight", race: 0, rarity: 0},
    Card { id: 6, mana: 2, hp: 2, attack: 1, curr_hp: 0, name: "royal_priest", race: 0, rarity: 0},
    Card { id: 7, mana: 2, hp: 2, attack: 1, curr_hp: 0, name: "royal_priest", race: 0, rarity: 0},
    Card { id: 8, mana: 2, hp: 2, attack: 1, curr_hp: 0, name: "royal_priest", race: 0, rarity: 0},
    Card { id: 9, mana: 2, hp: 2, attack: 1, curr_hp: 0, name: "royal_soldier", race: 0, rarity: 0},
    Card { id: 10, mana: 2, hp: 2, attack: 1, curr_hp: 0, name: "royal_soldier", race: 0, rarity: 0},
    Card { id: 11, mana: 2, hp: 2, attack: 1, curr_hp: 0, name: "royal_soldier", race: 0, rarity: 0},
    Card { id: 12, mana: 3, hp: 5, attack: 0, curr_hp: 0, name: "royal_shield", race: 0, rarity: 0},
    Card { id: 13, mana: 3, hp: 5, attack: 0, curr_hp: 0, name: "royal_shield", race: 0, rarity: 0},
    Card { id: 14, mana: 3, hp: 5, attack: 0, curr_hp: 0, name: "royal_shield", race: 0, rarity: 0},
    Card { id: 18, mana: 3, hp: 3, attack: 1, curr_hp: 0, name: "royal_cleric", race: 0, rarity: 0},
    Card { id: 19, mana: 3, hp: 3, attack: 1, curr_hp: 0, name: "royal_cleric", race: 0, rarity: 0},
    Card { id: 20, mana: 3, hp: 3, attack: 1, curr_hp: 0, name: "royal_cleric", race: 0, rarity: 0},
    Card { id: 21, mana: 2, hp: 1, attack: 2, curr_hp: 0, name: "royal_swordman", race: 0, rarity: 0},
    Card { id: 22, mana: 2, hp: 1, attack: 2, curr_hp: 0, name: "royal_swordman", race: 0, rarity: 0},
    Card { id: 23, mana: 2, hp: 1, attack: 2, curr_hp: 0, name: "royal_swordman", race: 0, rarity: 0},
    Card { id: 3, mana: 4, hp: 2, attack: 3, curr_hp: 0, name: "royal_champion", race: 0, rarity: 0},
    Card { id: 4, mana: 4, hp: 2, attack: 3, curr_hp: 0, name: "royal_champion", race: 0, rarity: 0},
    Card { id: 5, mana: 4, hp: 2, attack: 3, curr_hp: 0, name: "royal_champion", race: 0, rarity: 0},
    Card { id: 15, mana: 5, hp: 4, attack: 3, curr_hp: 0, name: "royal_paladin", race: 0, rarity: 2},
    Card { id: 16, mana: 5, hp: 4, attack: 3, curr_hp: 0, name: "royal_paladin", race: 0, rarity: 2},
    Card { id: 17, mana: 5, hp: 4, attack: 3, curr_hp: 0, name: "royal_paladin", race: 0, rarity: 2},
    Card { id: 24, mana: 4, hp: 3, attack: 3, curr_hp: 0, name: "royal_angel", race: 0, rarity: 2},
    Card { id: 25, mana: 4, hp: 3, attack: 3, curr_hp: 0, name: "royal_angel", race: 0, rarity: 2},
    Card { id: 26, mana: 4, hp: 3, attack: 3, curr_hp: 0, name: "royal_angel", race: 0, rarity: 2},
    Card { id: 27, mana: 6, hp: 1, attack: 6, curr_hp: 0, name: "royal_crusade", race: 0, rarity: 0},
    Card { id: 28, mana: 6, hp: 1, attack: 6, curr_hp: 0, name: "royal_crusade", race: 0, rarity: 0},
    Card { id: 29, mana: 6, hp: 1, attack: 6, curr_hp: 0, name: "royal_crusade", race: 0, rarity: 0},
];

const HUMANOID_DECK: [Card; 30] = [
    Card { id: 0, mana: 3, hp: 3, attack: 1, curr_hp: 0, name: "humanoid_archer", race:1, rarity: 0},
    Card { id: 1, mana: 3, hp: 3, attack: 1, curr_hp: 0, name: "humanoid_archer", race:1, rarity: 0},
    Card { id: 2, mana: 3, hp: 3, attack: 1, curr_hp: 0, name: "humanoid_archer", race:1, rarity: 0},
    Card { id: 6, mana: 2, hp: 2, attack: 1, curr_hp: 0, name: "humanoid_assasin", race:1, rarity: 0},
    Card { id: 7, mana: 2, hp: 2, attack: 1, curr_hp: 0, name: "humanoid_assasin", race:1, rarity: 0},
    Card { id: 8, mana: 2, hp: 2, attack: 1, curr_hp: 0, name: "humanoid_assasin", race:1, rarity: 0},
    Card { id: 9, mana: 2, hp: 2, attack: 1, curr_hp: 0, name: "humanoid_fanatic", race:1, rarity: 0},
    Card { id: 10, mana: 2, hp: 2, attack: 1, curr_hp: 0, name: "humanoid_fanatic", race:1, rarity: 0},
    Card { id: 11, mana: 2, hp: 2, attack: 1, curr_hp: 0, name: "humanoid_fanatic", race:1, rarity: 0},
    Card { id: 12, mana: 3, hp: 5, attack: 0, curr_hp: 0, name: "humanoid_grunt", race:1, rarity: 0},
    Card { id: 13, mana: 3, hp: 5, attack: 0, curr_hp: 0, name: "humanoid_grunt", race:1, rarity: 0},
    Card { id: 14, mana: 3, hp: 5, attack: 0, curr_hp: 0, name: "humanoid_grunt", race:1, rarity: 0},
    Card { id: 18, mana: 3, hp: 3, attack: 1, curr_hp: 0, name: "humanoid_occultist", race:1, rarity: 0},
    Card { id: 19, mana: 3, hp: 3, attack: 1, curr_hp: 0, name: "humanoid_occultist", race:1, rarity: 0},
    Card { id: 20, mana: 3, hp: 3, attack: 1, curr_hp: 0, name: "humanoid_occultist", race:1, rarity: 0},
    Card { id: 21, mana: 2, hp: 1, attack: 2, curr_hp: 0, name: "humanoid_pikeman", race:1, rarity: 0},
    Card { id: 22, mana: 2, hp: 1, attack: 2, curr_hp: 0, name: "humanoid_pikeman", race:1, rarity: 0},
    Card { id: 23, mana: 2, hp: 1, attack: 2, curr_hp: 0, name: "humanoid_pikeman", race:1, rarity: 0},
    Card { id: 3, mana: 4, hp: 2, attack: 3, curr_hp: 0, name: "humanoid_tinker", race:1, rarity: 0},
    Card { id: 4, mana: 4, hp: 2, attack: 3, curr_hp: 0, name: "humanoid_tinker", race:1, rarity: 0},
    Card { id: 5, mana: 4, hp: 2, attack: 3, curr_hp: 0, name: "humanoid_tinker", race:1, rarity: 0},
    Card { id: 15, mana: 5, hp: 4, attack: 3, curr_hp: 0, name: "humanoid_wanderer", race:1, rarity: 2},
    Card { id: 16, mana: 5, hp: 4, attack: 3, curr_hp: 0, name: "humanoid_wanderer", race:1, rarity: 2},
    Card { id: 17, mana: 5, hp: 4, attack: 3, curr_hp: 0, name: "humanoid_wanderer", race:1, rarity: 2},
    Card { id: 24, mana: 4, hp: 3, attack: 3, curr_hp: 0, name: "humanoid_fox", race:1, rarity: 2},
    Card { id: 25, mana: 4, hp: 3, attack: 3, curr_hp: 0, name: "humanoid_fox", race:1, rarity: 2},
    Card { id: 26, mana: 4, hp: 3, attack: 3, curr_hp: 0, name: "humanoid_fox", race:1, rarity: 2},
    Card { id: 27, mana: 6, hp: 1, attack: 6, curr_hp: 0, name: "humanoid_wolfrider", race:1, rarity: 0},
    Card { id: 28, mana: 6, hp: 1, attack: 6, curr_hp: 0, name: "humanoid_wolfrider", race:1, rarity: 0},
    Card { id: 29, mana: 6, hp: 1, attack: 6, curr_hp: 0, name: "humanoid_wolfrider", race:1, rarity: 0},
];

const UNDEAD_DECK: [Card; 30] = [
    Card { id: 0, mana: 3, hp: 3, attack: 1, curr_hp: 0, name: "undead_archer", race:2, rarity: 0},
    Card { id: 1, mana: 3, hp: 3, attack: 1, curr_hp: 0, name: "undead_archer", race:2, rarity: 0},
    Card { id: 2, mana: 3, hp: 3, attack: 1, curr_hp: 0, name: "undead_archer", race:2, rarity: 0},
    Card { id: 6, mana: 2, hp: 2, attack: 1, curr_hp: 0, name: "undead_bones", race:2, rarity: 0},
    Card { id: 7, mana: 2, hp: 2, attack: 1, curr_hp: 0, name: "undead_bones", race:2, rarity: 0},
    Card { id: 8, mana: 2, hp: 2, attack: 1, curr_hp: 0, name: "undead_bones", race:2, rarity: 0},
    Card { id: 9, mana: 2, hp: 2, attack: 1, curr_hp: 0, name: "undead_crawler", race:2, rarity: 0},
    Card { id: 10, mana: 2, hp: 2, attack: 1, curr_hp: 0, name: "undead_crawler", race:2, rarity: 0},
    Card { id: 11, mana: 2, hp: 2, attack: 1, curr_hp: 0, name: "undead_crawler", race:2, rarity: 0},
    Card { id: 12, mana: 3, hp: 5, attack: 0, curr_hp: 0, name: "undead_eye", race:2, rarity: 0},
    Card { id: 13, mana: 3, hp: 5, attack: 0, curr_hp: 0, name: "undead_eye", race:2, rarity: 0},
    Card { id: 14, mana: 3, hp: 5, attack: 0, curr_hp: 0, name: "undead_eye", race:2, rarity: 0},
    Card { id: 18, mana: 3, hp: 3, attack: 1, curr_hp: 0, name: "undead_feader", race:2, rarity: 0},
    Card { id: 19, mana: 3, hp: 3, attack: 1, curr_hp: 0, name: "undead_feader", race:2, rarity: 0},
    Card { id: 20, mana: 3, hp: 3, attack: 1, curr_hp: 0, name: "undead_feader", race:2, rarity: 0},
    Card { id: 21, mana: 2, hp: 1, attack: 2, curr_hp: 0, name: "undead_revenant", race:2, rarity: 0},
    Card { id: 22, mana: 2, hp: 1, attack: 2, curr_hp: 0, name: "undead_revenant", race:2, rarity: 0},
    Card { id: 23, mana: 2, hp: 1, attack: 2, curr_hp: 0, name: "undead_revenant", race:2, rarity: 0},
    Card { id: 3, mana: 4, hp: 2, attack: 3, curr_hp: 0, name: "undead_scarab", race:2, rarity: 0},
    Card { id: 4, mana: 4, hp: 2, attack: 3, curr_hp: 0, name: "undead_scarab", race:2, rarity: 0},
    Card { id: 5, mana: 4, hp: 2, attack: 3, curr_hp: 0, name: "undead_scarab", race:2, rarity: 0},
    Card { id: 15, mana: 5, hp: 4, attack: 3, curr_hp: 0, name: "undead_stumbler", race:2, rarity: 2},
    Card { id: 16, mana: 5, hp: 4, attack: 3, curr_hp: 0, name: "undead_stumbler", race:2, rarity: 2},
    Card { id: 17, mana: 5, hp: 4, attack: 3, curr_hp: 0, name: "undead_stumbler", race:2, rarity: 2},
    Card { id: 24, mana: 4, hp: 3, attack: 3, curr_hp: 0, name: "undead_ghoul", race:2, rarity: 2},
    Card { id: 25, mana: 4, hp: 3, attack: 3, curr_hp: 0, name: "undead_ghoul", race:2, rarity: 2},
    Card { id: 26, mana: 4, hp: 3, attack: 3, curr_hp: 0, name: "undead_ghoul", race:2, rarity: 2},
    Card { id: 27, mana: 6, hp: 1, attack: 6, curr_hp: 0, name: "undead_hand", race:2, rarity: 0},
    Card { id: 28, mana: 6, hp: 1, attack: 6, curr_hp: 0, name: "undead_hand", race:2, rarity: 0},
    Card { id: 29, mana: 6, hp: 1, attack: 6, curr_hp: 0, name: "undead_hand", race:2, rarity: 0},
];
// Define the game state initialization using the turbo::init! macro
turbo::init! {
    struct GameState {
        frame: u32,
        gamestart: bool,
        gamestage: u32,
        cursor_x: i32,
        cursor_y: i32,
        picked_race: i32,
        reset_confirm: bool,
        // main game state
        round: i32,
        round_start: bool,
        round_running: bool,
        round_end: bool,
        curr_mana: i32,
        curr_hp: i32,
        curr_cards: Vec<i32>,

        used_deck_cards: Vec<i32>,
        deck_remain_cards_count: i32,
        curr_choice: i32,
        curr_choosing_mana: Vec<i32>,
        curr_chosen: Vec<i32>,
        curr_chosen_coordinate: Vec<(i32, i32)>,
        curr_cardboard_elements: Vec<Vec<i32>>,
        curr_cardboard_elements_hp: Vec<Vec<i32>>,
        curr_cardboard_elements_attack: Vec<Vec<i32>>,
        timer: i32,

        enemy_hp: i32,
        enemy_curr_mana: i32,
        enemy_used_deck_cards: Vec<i32>,
        enemy_curr_cards: Vec<i32>,
        // enemy_curr_choice: i32,
        // enemy_curr_choosing_mana: Vec<i32>,
        // enemy_curr_chosen: Vec<i32>,
        // enemy_curr_chosen_coordinate: Vec<(i32, i32)>,
        enemy_curr_cardboard_elements: Vec<Vec<i32>>,
        enemy_curr_cardboard_elements_hp: Vec<Vec<i32>>,
        enemy_curr_cardboard_elements_attack: Vec<Vec<i32>>,

    } = {
        Self {
            frame: 0,
            gamestart: false,
            gamestage: 0,
            cursor_x: 120,
            cursor_y: 100,
            picked_race: -1,
            reset_confirm: false,

            // main game state
            round: 0,
            round_start: true,
            round_running: false,
            round_end: false,
            curr_mana: 2,
            curr_choosing_mana: Vec::new(),
            curr_hp: 20,
            curr_cards: Vec::new(),
            used_deck_cards: Vec::new(),
            deck_remain_cards_count: 26,
            curr_choice: -1,
            curr_chosen: Vec::new(),
            curr_chosen_coordinate: Vec::new(),
            curr_cardboard_elements: Vec::new(),
            curr_cardboard_elements_hp: Vec::new(),
            curr_cardboard_elements_attack: Vec::new(),
            timer: 30,

            enemy_hp: 20,
            enemy_curr_mana: 2,
            enemy_used_deck_cards: Vec::new(),
            enemy_curr_cards: Vec::new(),
            // enemy_curr_choice: -1,
            // enemy_curr_choosing_mana: Vec::new(),
            // enemy_curr_chosen:  Vec::new(),
            // enemy_curr_chosen_coordinate: Vec::new(),
            enemy_curr_cardboard_elements: Vec::new(),
            enemy_curr_cardboard_elements_hp: Vec::new(),
            enemy_curr_cardboard_elements_attack: Vec::new(),

        }
    }
}

fn calculate_distance_and_update_state(state: &mut GameState, race_center: (i32, i32), race_id: i32) -> i32 {
    let cursor_center_x = state.cursor_x + 5;
    let cursor_center_y = state.cursor_y + 7;
    let dx = cursor_center_x - race_center.0;
    let dy = cursor_center_y - race_center.1;

    let distance_squared = dx.pow(2) + dy.pow(2);
    let radii_sum_squared = 22_i32.pow(2);

    if 0 <= distance_squared && distance_squared <= radii_sum_squared {
        state.picked_race = race_id;
    } else {
        state.picked_race = -1;
    }

    state.picked_race
}

fn check_within_confirm(state: &mut GameState) -> bool {
    let cursor_center_x = state.cursor_x + 5;
    let cursor_center_y = state.cursor_y + 7;
    let range_x = (80, 180);
    let range_y = (120, 140);
    if range_x.0 <= cursor_center_x && cursor_center_x <= range_x.1 && range_y.0 <= cursor_center_y && cursor_center_y <= range_y.1 {
        return true;
    } else {
        return false;
    }
}
 
fn determine_can_place(index: i32, state: &mut GameState) {
    if state.curr_chosen.contains(&index) {
        text!("Card is chosen", x = 0, y = 0, font = Font::M, color = 0xd92c23ff);
    } else {
        let mut mana = 0;
        if state.picked_race == 0 {
            mana = ROYAL_DECK[state.curr_cards[index as usize] as usize].mana;
        } else if state.picked_race == 1 {
            mana = HUMANOID_DECK[state.curr_cards[index as usize] as usize].mana;
        } else if state.picked_race == 2 {
            mana = UNDEAD_DECK[state.curr_cards[index as usize] as usize].mana;
        }
      
        let sum: i32 = state.curr_choosing_mana.iter().sum(); 
        if mana > (state.curr_mana - sum) {
            text!("Not enough mana", x = 0, y = 0, font = Font::M, color = 0xd92c23ff);
        } else {
            state.curr_choice = index;
        }    
    }
}

fn check_card_selection(state: &mut GameState) {
    let card1x_range = (0, 46);
    let card2x_range = (46, 92);
    let card3x_range = (92, 138);
    let card4x_range = (138, 184);

    let card1y_range = (100, 146);
    let card2y_range = (100, 146);
    let card3y_range = (100, 146);
    let card4y_range = (100, 146);

    if state.gamestage == 2 && (state.cursor_x >= 0 && state.cursor_x <= 34) && (state.cursor_y >= 68 && state.cursor_y <= 78){
        // pass clicked
        simulate_enemy(state);
        simulate_fight(state);
        state.curr_chosen.clear();
        state.curr_choosing_mana.clear();
        state.curr_chosen_coordinate.clear();
        state.round_running = false;
        state.round_end = true;
        state.cursor_x = 120;
        state.cursor_y = 80;
    } else if state.gamestage == 2 && (state.cursor_x >= 0 && state.cursor_x <= 34) && (state.cursor_y >= 80 && state.cursor_y <= 90) {
        if state.curr_chosen.len() > 0 {
            // cancel clicked
            state.curr_chosen.clear();
            state.curr_choosing_mana.clear();
            state.curr_chosen_coordinate.clear();
        }
    } else if state.gamestage == 2 && (state.cursor_x >= 36 && state.cursor_x <= 70) && (state.cursor_y >= 80 && state.cursor_y <= 90) {
        if state.curr_chosen.len() > 0 {
            // attack clicked
            for i in 0..state.curr_chosen.len() {
                let mut curr_card : &Card = &ROYAL_DECK[state.curr_cards[state.curr_chosen[i] as usize] as usize];
                if state.picked_race == 0 {
                    curr_card = &ROYAL_DECK[state.curr_cards[state.curr_chosen[i] as usize] as usize];
                } else if state.picked_race == 1 {
                    curr_card = &HUMANOID_DECK[state.curr_cards[state.curr_chosen[i] as usize] as usize];
                } else if state.picked_race == 2 {
                    curr_card = &UNDEAD_DECK[state.curr_cards[state.curr_chosen[i] as usize] as usize];
                }
               
                state.curr_cardboard_elements[state.curr_chosen_coordinate[i].0 as usize][state.curr_chosen_coordinate[i].1 as usize]= curr_card.id;
                state.curr_cardboard_elements_hp[state.curr_chosen_coordinate[i].0 as usize][state.curr_chosen_coordinate[i].1 as usize] = curr_card.hp;
                state.curr_cardboard_elements_attack[state.curr_chosen_coordinate[i].0 as usize][state.curr_chosen_coordinate[i].1 as usize] = curr_card.attack;
                text!(&format!("curr: {} {}", state.curr_chosen[i], state.curr_cards.len()), x = 0, y = 10 + 10*i as i32, font = Font::M, color = 0xffffffff); // Render the score
                // state.curr_cards.remove(state.curr_chosen[i] as usize);
                state.curr_cards[state.curr_chosen[i] as usize] = -1;
            }
            state.curr_cards.retain(|&card| card != -1);
            simulate_enemy(state);
            simulate_fight(state);
            state.curr_chosen.clear();
            state.curr_choosing_mana.clear();
            state.curr_chosen_coordinate.clear();
            state.round_running = false;
            state.round_end = true;
        }
    } else {
        if state.curr_cards.len() > 0 && state.gamestage == 2 && (card1x_range.0 <= state.cursor_x && state.cursor_x <= card1x_range.1) && (card1y_range.0 <= state.cursor_y && state.cursor_y <= card1y_range.1) {
            determine_can_place(0, state);
        } else if state.curr_cards.len() > 1 && state.gamestage == 2 && (card2x_range.0 <= state.cursor_x && state.cursor_x <= card2x_range.1) && (card2y_range.0 <= state.cursor_y && state.cursor_y <= card2y_range.1) {
            determine_can_place(1, state);
        } else if state.curr_cards.len() > 2 && state.gamestage == 2 && (card3x_range.0 <= state.cursor_x && state.cursor_x <= card3x_range.1) && (card3y_range.0 <= state.cursor_y && state.cursor_y <= card3y_range.1) {
            determine_can_place(2, state);
        } else if state.curr_cards.len() > 3 && state.gamestage == 2 &&(card4x_range.0 <= state.cursor_x && state.cursor_x <= card4x_range.1) && (card4y_range.0 <= state.cursor_y && state.cursor_y <= card4y_range.1) {
            determine_can_place(3, state);
        } else {
            if state.curr_choice != -1  {
                set_curr_chosen_coordinate(state);
            }
            state.curr_choice = -1;
        }
    }
}

fn convert_grid_to_coor(x: i32, y: i32) -> (i32, i32) {
    let pixel_x = 80 + 16 * x;
    let pixel_y = 48 + 16 * y;
    (pixel_x, pixel_y)
}

fn convert_coor_to_grid(pixel_x: i32, pixel_y: i32) -> (i32, i32) {
    let x = (pixel_x - 80) / 16;
    let y = (pixel_y - 48) / 16;
    (x, y)
}

fn set_curr_chosen_coordinate(state: &mut GameState) {
    let (grid_x, grid_y) = convert_coor_to_grid(state.cursor_x, state.cursor_y);
    if state.curr_choice == -1 {
        state.curr_chosen_coordinate.clear();
    } else {
        if (grid_x < 0) || (grid_x > 2) || (grid_y < 0) || (grid_y > 2) || state.curr_cardboard_elements[grid_x as usize][grid_y as usize] != -1 
            || state.curr_chosen_coordinate.contains(&(grid_x, grid_y))
        {
            state.curr_chosen.clear();
            state.curr_choosing_mana.clear();
            state.curr_chosen_coordinate.clear();
        } else {
            state.curr_chosen.push(state.curr_choice);
            state.curr_chosen_coordinate.push((grid_x, grid_y));
            let mut mana = 0;
            if state.picked_race == 0 {
                mana = ROYAL_DECK[state.curr_cards[state.curr_choice as usize] as usize].mana;
            } else if state.picked_race == 1 {
                mana = HUMANOID_DECK[state.curr_cards[state.curr_choice as usize] as usize].mana;
            } else if state.picked_race == 2 {
                mana = UNDEAD_DECK[state.curr_cards[state.curr_choice as usize] as usize].mana;
            }
           
            state.curr_choosing_mana.push(mana);
        }
    }
}

fn init_cardboard(state: &mut GameState) {
   state.curr_cardboard_elements_hp = vec![vec![0; 3]; 3];
   state.curr_cardboard_elements_attack = vec![vec![-1; 3]; 3];
   state.curr_cardboard_elements = vec![vec![-1; 3]; 3];

   state.enemy_curr_cardboard_elements_hp = vec![vec![0; 3]; 3];
   state.enemy_curr_cardboard_elements_attack = vec![vec![-1; 3]; 3];
   state.enemy_curr_cardboard_elements = vec![vec![-1; 3]; 3];
}

fn init_enemy_hand(state: &mut GameState) {
    while state.enemy_curr_cards.len() < 4 {
        let number = (rand() % 30) as i32;
        if !state.enemy_used_deck_cards.contains(&(number as i32)) {
            state.enemy_curr_cards.push(number);
            state.enemy_used_deck_cards.push(number);
        }
    }
}

fn calc_enemy_hand(state: &mut GameState ) {
    let curr_count = state.enemy_curr_cards.len();
    if curr_count < 4 {
        while state.enemy_curr_cards.len() < curr_count + 1 {
            let number = (rand() % 30) as i32;
            if !state.enemy_used_deck_cards.contains(&(number as i32)) {
                state.enemy_curr_cards.push(number);
                state.enemy_used_deck_cards.push(number);
            }
        }
    }
}

fn simulate_enemy(state: &mut GameState ) {
    for i in 0..state.enemy_curr_cards.len() {
        if UNDEAD_DECK[state.enemy_curr_cards[i] as usize].mana <= state.enemy_curr_mana {
            // chose and attack
            let mut random_x = rand() % 3;
            let mut random_y = rand() % 3;
            while state.enemy_curr_cardboard_elements[random_x as usize][random_y as usize] != -1 {
                random_x = rand() % 3;
                random_y = rand() % 3;
            }
            state.enemy_curr_cardboard_elements[random_x as usize][random_y as usize] = state.enemy_curr_cards[i];
            state.enemy_curr_cardboard_elements_hp[random_x as usize][random_y as usize] = UNDEAD_DECK[state.enemy_curr_cards[i] as usize].hp;
            state.enemy_curr_cardboard_elements_attack[random_x as usize][random_y as usize] = UNDEAD_DECK[state.enemy_curr_cards[i] as usize].attack;
            state.enemy_curr_cards.remove(i);
            break;
        }
    }
    // pass if nothing can be done
}

fn simulate_fight(state: &mut GameState ) {
    for i in 0..3 {
        // action
        let mut new_k = 0;
        for j in (0..3).rev() {
            let curr_attack = state.curr_cardboard_elements_attack[j][i];
            let curr_hp = state.curr_cardboard_elements_hp[j][i];
            if curr_hp <= 0 || curr_attack < 0 {
                continue;
            }
            for k in 0..3 {
                if state.enemy_curr_cardboard_elements_hp[k][i] > 0 {
                    state.enemy_curr_cardboard_elements_hp[k][i] -= curr_attack;
                    state.curr_cardboard_elements_hp[j][i] -= state.enemy_curr_cardboard_elements_attack[k][i];
                    if state.enemy_curr_cardboard_elements_hp[k][i] <= 0 {
                        state.enemy_curr_cardboard_elements[k][i] = -1;
                    }
                    if state.curr_cardboard_elements_hp[j][i] <= 0 {
                        state.curr_cardboard_elements[j][i] = -1;
                    }
                    new_k = k;
                    break;
                } else {
                    if k == 2 {
                        state.enemy_hp -= curr_attack;
                    }
                }
            }
        }

        for kk in new_k+1..3 {
            let enemy_attack = state.enemy_curr_cardboard_elements_attack[kk][i];
            let enemy_hp = state.enemy_curr_cardboard_elements_hp[kk][i];
            if enemy_hp <= 0 || enemy_attack < 0 {
                continue;
            }
            for jj in (0..3).rev () {
                if state.curr_cardboard_elements_hp[jj][i] > 0 {
                    state.curr_cardboard_elements_hp[jj][i] -= enemy_attack;
                    state.enemy_curr_cardboard_elements_hp[kk][i] -= state.curr_cardboard_elements_attack[jj][i];
                    if state.curr_cardboard_elements_hp[jj][i] <= 0 {
                        state.curr_cardboard_elements[jj][i] = -1;
                    }
                    if state.enemy_curr_cardboard_elements_hp[kk][i] <= 0 {
                        state.enemy_curr_cardboard_elements[kk][i] = -1;
                    }
                    break;
                } else {
                    if jj == 0 {
                        state.curr_hp -= enemy_attack;
                    }
                }
            }
        }
    }
}

// Implement the game loop using the turbo::go! macro
turbo::go! {
    // Load the game state
    let mut state = GameState::load();

    // get the gamepad state for player 1
    let p1_gamepad = gamepad(0);

    // let counter_string = state.round.to_string();
    // text!(&format!("Round: {}", state.round), x = 10, y = 10, font = Font::M, color = 0xffffffff); // Render the score

    if p1_gamepad.start.pressed() && (state.gamestart == false) && (state.gamestage == 0) {
        state.gamestart = true;
        state.gamestage = 1;
    }

    // Handle user input
    if gamepad(0).left.pressed() {
        if (state.cursor_x - 4) >= 0 {
            state.cursor_x -= 4;
        }
    }

    if gamepad(0).right.pressed() {
        if (state.cursor_x + 4) <= 256 - 12 {
            state.cursor_x += 4;
        }
    }
    
    if gamepad(0).up.pressed() {
        if (state.cursor_y -4) >= 0 {
            state.cursor_y -= 4;
        }
       
    }
    
    if gamepad(0).down.pressed() {
        if (state.cursor_y + 4) <= 144 - 16 {
            state.cursor_y += 4;
        }
    }

    // init screen
    if (state.gamestart == false) && (state.gamestage == 0) {
        clear(0x000000FF);
        let x = 75;
        let y = 70;
        let font = Font::M; // try Font::S or Font::L too
        let color = 0xffffffff;
        let message = "Press Space to Start";
        text(x, y, font, color, message);
    }

    // choose race
    if (state.gamestart == true) && (state.gamestage == 1) {
        // Set the background color
        clear(0x000000FF);

        sprite!("royal", x = 20, y = 50);
        sprite!("humanoid", x = 110, y = 50);
        sprite!("undead", x = 200, y = 50);

        if state.picked_race != -1 {
            let curr_x = match state.picked_race {
                0 => 98,
                1 => 90,
                2 => 95,
                _ => 95, // default case
            };
            // render picked race
            text(curr_x, 20, Font::M, 0xffffffff, &("Picked ".to_owned() + RACES[state.picked_race as usize].name));

            // render confirm button
            rect!(w = 100, h = 20, x = 80, y = 120);
            sprite!("punch", 80, y = 110);
            text(130, 125, Font::M, 0x000000FF, "Confirm");

        } else {
            text(100, 20, Font::M, 0xffffffff, "Pick a Race");
        }
        
        if gamepad(0).select.pressed() && state.gamestage == 1 { 
            let royal_center = (20 + 16, 50 + 16);
            let humanoid_center = (110 + 16, 50 + 16);
            let undead_center = (210 + 16, 50 + 16);

            if state.picked_race != -1 {
                let result = check_within_confirm(&mut state);
                if result {
                    if state.picked_race == 0 {
                        state.cursor_x = 36;
                        state.cursor_y = 66;
                    } else if state.picked_race == 1 {
                        state.cursor_x = 126;
                        state.cursor_y = 66;
                    } else if state.picked_race == 2 {
                        state.cursor_x = 226;
                        state.cursor_y = 66;
                    }
                    state.gamestage = 2;
                }
            } 

            if calculate_distance_and_update_state(&mut state, royal_center, 0) == -1 {
                // Continue with execution...
                if calculate_distance_and_update_state(&mut state, humanoid_center, 1) == -1 {
                    // Continue with execution...
                    calculate_distance_and_update_state(&mut state, undead_center, 2);
                }
            }   
            
            sprite!("confirm", x = state.cursor_x, y = state.cursor_y);
        } else {            
            sprite!("hand", x = state.cursor_x, y = state.cursor_y);
        }
    }

    // combat
    if (state.gamestart == true) && (state.gamestage == 2) {
        // Set the background color
        clear(0x000000FF);
        
        // set background
        for i in 0..8 {
            for j in 0..9 {
                sprite!("tree", x =  16 * i, y =  16*j);
            }
        }

        for i in 0..8{
            for j in 0..9 {
                sprite!("tree", x = 128 + 16 * i, y = 16*j);
            }
        }


        let max_mana = std::cmp::min(state.round + 2, 8);
        
        if state.round_start {
            // init game
            if state.round == 0 {
                init_cardboard(&mut state);
                state.deck_remain_cards_count = 26;
                while state.curr_cards.len() < 4 {
                    let number = (rand() % 30) as i32;
                    if !state.used_deck_cards.contains(&(number as i32)) {
                        state.curr_cards.push(number);
                        state.used_deck_cards.push(number);
                    }
                }
                init_enemy_hand(&mut state);
            }
            
            // after round 0 
            if state.round > 0 {
                let curr_count = state.curr_cards.len();
                if curr_count < 4 {
                    state.deck_remain_cards_count -= (4 - curr_count) as i32;
                    while state.curr_cards.len() < curr_count + 1 {
                        let number = (rand() % 30) as i32;
                        if !state.used_deck_cards.contains(&(number as i32)) {
                            state.curr_cards.push(number);
                            state.used_deck_cards.push(number);
                        }
                    }
                }
                calc_enemy_hand(&mut state);
            }

            state.curr_mana = max_mana;
            state.enemy_curr_mana = max_mana;
            state.round_start = false;
            state.round_running = true;
        } 

        // render existing characters
        for i in 0..3 {
            for j in 0..3 {
                sprite!("grass", x = 80 + 16 * i, y = 48 + 16*j);
                if state.curr_cardboard_elements[i as usize][j as usize] != -1 {
                    let mut card = None;
                    if state.picked_race == 0 {
                        card = ROYAL_DECK.iter().find(|&card| card.id == state.curr_cardboard_elements[i as usize][j as usize]);
                    } else if state.picked_race == 1 {
                        card = HUMANOID_DECK.iter().find(|&card| card.id == state.curr_cardboard_elements[i as usize][j as usize]);
                    } else if state.picked_race == 2 {
                        card = UNDEAD_DECK.iter().find(|&card| card.id == state.curr_cardboard_elements[i as usize][j as usize]);
                    }
               
                    match card {
                        Some(card) => {
                            sprite!(card.name, x = 80 + 16 * i, y = 48 + 16*j);
                        } ,
                        None => {
                            
                        }
                    }
                }
            }
        }
        
        // render selecting box
        for i in 0..3 {
            for j in 0..3 {
                if state.curr_choice != -1 {
                    if state.curr_chosen_coordinate.contains(&(i, j)) {
                        sprite!("selecting_yellow", x = 80 + 16 * i, y = 48 + 16*j);
                    } else if state.curr_cardboard_elements[i as usize][j as usize] != -1 {
                        sprite!("selecting_red", x = 80 + 16 * i, y = 48 + 16*j);
                    } else {
                        sprite!("selecting", x = 80 + 16 * i, y = 48 + 16*j);
                    }
                } else {
                    
                }
            }
        }

        // render opponents
        for i in 0..3 {
            for j in 0..3 {
                sprite!("road", x = 128 + 16 * i, y = 48 + 16*j);
                if state.enemy_curr_cardboard_elements[i as usize][j as usize] != -1 {
                    let enemy_card = &UNDEAD_DECK[state.enemy_curr_cardboard_elements[i as usize][j as usize] as usize];
                    sprite!(enemy_card.name, x = 128 + 16 * i, y = 48 + 16*j);
                    text!(&format!("{}", state.enemy_curr_cardboard_elements_attack[i as usize][j as usize]), x = (128 + 16 * i).try_into().unwrap(), y= (48 + 16*j).try_into().unwrap(), font = Font::M, color = 0xFF8C00ff);
                    text!(&format!("{}", state.enemy_curr_cardboard_elements_hp[i as usize][j as usize]), x = (138 + 16 * i).try_into().unwrap(), y= (48 + 16*j).try_into().unwrap(), font = Font::M, color = 0xd92c23ff);
                }
            }
        }

        // render attack/hp
        for i in 0..3 {
            for j in 0..3 {
                if state.curr_cardboard_elements[i][j] != -1 {
                    if state.curr_cardboard_elements_attack[i][j] > -1 {
                        text!(&format!("{}", state.curr_cardboard_elements_attack[i][j]), x = (80 + 16 * i).try_into().unwrap(), y= (48 + 16*j).try_into().unwrap(), font = Font::M, color = 0xFF8C00ff);
                    }
                    if state.curr_cardboard_elements_hp[i][j] > 0 {
                        text!(&format!("{}", state.curr_cardboard_elements_hp[i][j]), x = (90 + 16 * i).try_into().unwrap(), y= (48 + 16*j).try_into().unwrap(), font = Font::M, color = 0xd92c23ff);
                    } 
                }
            }
        }

        if state.round_end {
            if (state.curr_hp <= 0) || (state.enemy_hp <= 0) {
                state.gamestage = 3;
            } 
            state.round += 1;
            state.round_end = false;
            state.round_start = true;
        }

        // set card cards
        if state.curr_cards.len() > 0 {
            for i in 0..state.curr_cards.len() as i32 {
                if state.curr_choice == i || state.curr_chosen.contains(&i) {
                    sprite!("paper_selected", x = 0 + 46 * i, y = 100);
                } else {
                    sprite!("paper", x = 0 + 46 * i, y = 100);
                }
              
                sprite!("potion", x = 34 + 46 * i, y = 100);
                sprite!("card_attack", x = 0 + 46 * i, y = 132);
                sprite!("potion", x = 34 + 46 * i, y = 132);
                
                let mut curr_card: &Card = &ROYAL_DECK[state.curr_cards[i as usize] as usize];
                if state.picked_race == 0 {
                    curr_card = &ROYAL_DECK[state.curr_cards[i as usize] as usize];
                } else if state.picked_race == 1 {
                    curr_card = &HUMANOID_DECK[state.curr_cards[i as usize] as usize];
                } else if state.picked_race == 2 {
                    curr_card = &UNDEAD_DECK[state.curr_cards[i as usize] as usize];
                }
                
            
                sprite!(curr_card.name, x = 16+ 46 * i, y = 110, fps = fps::FAST);

                text!(&format!("{}", curr_card.mana), x = 37 + 46 * i, y = 106, font = Font::M, color = 0x0073adff); // mana
                text!(&format!("{}", curr_card.hp), x = 37 + 46 * i, y = 138, font = Font::M, color = 0xd92c23ff); // hp
                text!(&format!("{}", curr_card.attack), x = 15 + 46 * i, y = 138, font = Font::M, color = 0xffffffFF); // attack    
                
                if state.curr_choice == i  {
                    sprite!("card_attack", x = 18 + 46 * i, y = 90);
                }
            }
        }

        // set ui
        rect!(w = 256-184, h = 44, x = 184, y = 100, fill = 0xb1d354ff);

        // mana
        let sum: i32= state.curr_choosing_mana.iter().sum();

        if state.curr_mana - sum > 0 {
            sprite!("mana", x = 184, y = 100);
        } else {
            sprite!("mana_empty", x = 184, y = 100);
        }
       
        text!(&format!("{}", state.curr_mana - sum), x = 198, y = 130, font = Font::M, color = 0x000000FF);
         
        // deck count
        sprite!("deck", x = 224, y = 100);
        text!(&format!("{}", state.deck_remain_cards_count), x = 236, y = 130, font = Font::M, color = 0x000000FF);
        
        // round
        sprite!("squarepaper", x = 104, y = 0);
        text!(&format!("Round:{}", state.round), x = 110, y = 10, font = Font::M, color = 0x000000FF); 

        // hp
        text!(&format!("hp:{}", state.curr_hp), x = 110, y = 20, font = Font::M, color = 0x000000FF); 
        text!(&format!("hp:{}", state.enemy_hp), x = 110, y = 30, font = Font::M, color = 0x000000FF); 

        // cancel button
        if state.curr_chosen.len() > 0 {
            rect!(w = 34, h = 10, x = 0, y = 80);
            text!(&format!("Cancel"), x = 2, y = 82, font = Font::M, color = 0x000000FF);
        }

        // attack button 
        if state.curr_chosen.len() > 0 {
            rect!(w = 34, h = 10, x = 36, y = 80);
            text!(&format!("Attack"), x = 38, y = 82, font = Font::M, color = 0x000000FF);
        }

        rect!(w = 24, h = 10, x = 0, y = 68);
        text!(&format!("Pass"), x = 2, y = 70, font = Font::M, color = 0x000000FF);

        // log
        text!(&format!("Choice:{}", state.curr_choice), x = 0, y = 0, font = Font::M, color = 0x000000FF);
        // text!(&format!("cards:{} chosen {}", state.curr_cards.len(), state.curr_chosen.len()), x = 0, y = 10, font = Font::M, color = 0x000000FF);
        // text!(&format!("Chosen:{} {}", state.curr_chosen_coordinate.0, state.curr_chosen_coordinate.1), x = 0, y = 20, font = Font::M, color = 0x000000FF);
        //text!(&format!("Chosen Mana:{}", state.curr_choosing_mana), x = 0, y = 30, font = Font::M, color = 0x000000FF);

        // handle character
        if state.curr_chosen.len() > 0 && state.curr_chosen_coordinate.len() > 0 {

            for i in 0..state.curr_chosen.len() {
                let (x, y) = convert_grid_to_coor(state.curr_chosen_coordinate[i].0, state.curr_chosen_coordinate[i].1);
                let mut curr_card : &Card = &ROYAL_DECK[state.curr_cards[state.curr_chosen[i] as usize] as usize];
                if state.picked_race == 0 {
                    curr_card = &ROYAL_DECK[state.curr_cards[state.curr_chosen[i] as usize] as usize];
                } else if state.picked_race == 1 {
                    curr_card = &HUMANOID_DECK[state.curr_cards[state.curr_chosen[i] as usize] as usize];
                } else if state.picked_race == 2 {
                    curr_card = &UNDEAD_DECK[state.curr_cards[state.curr_chosen[i] as usize] as usize];
                }
                
                sprite!(curr_card.name, x = x, y = y, fps = fps::FAST);
            }
        }       

        // handle press
        if state.gamestage == 2 && gamepad(0).a.pressed() {
            state.cursor_x = 23;
            state.cursor_y = 123;
        } else if state.gamestage == 2 && gamepad(0).b.pressed() {
            state.cursor_x = 23 + 46;
            state.cursor_y = 123;
        } else if state.gamestage == 2 && gamepad(0).x.pressed() {
            state.cursor_x = 23 + 46 * 2;
            state.cursor_y = 123;
        } else if state.gamestage == 2 && gamepad(0).y.pressed() {
            state.cursor_x = 23 + 46 * 3;
            state.cursor_y = 123;
        }

        if (state.gamestage == 2) && gamepad(0).select.pressed(){ 
            sprite!("confirm", x = state.cursor_x, y = state.cursor_y);
            check_card_selection(&mut state);
        } else {         
            sprite!("hand", x = state.cursor_x, y = state.cursor_y);
        }
    }

    // game end
    if (state.gamestart == true) && (state.gamestage == 3) {
        let mut message = "Game Over";
        if state.curr_hp <= 0 && state.enemy_hp <= 0 {
            message = "Draw";
        } else if state.curr_hp <= 0 {
            message = "Defeat";
        } else if state.enemy_hp <= 0 {
            message = "Victory";
        }

        let x = 80;
        let y = 70;
        let font = Font::M; // try Font::S or Font::L too
        let color = 0xffffffff;
        
        text(x, y, font, color, message);
    }

    state.frame += 1;
    state.save();
}