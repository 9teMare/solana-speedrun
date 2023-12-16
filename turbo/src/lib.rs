use solana_sdk::pubkey::Pubkey;
use turbo::solana;

// Define the game configuration using the turbo::cfg! macro

turbo::cfg! {r#"
    name = "Pancake Cat"
    version = "1.0.0"
    author = "Turbo"
    description = "Catch falling pancakes!"
    [settings]
    resolution = [256, 144]
    [solana]
    http-rpc-url = "http://127.0.0.1:8899"
    ws-rpc-url = "ws://127.0.0.1:8900"
"#}

// Define the game state initialization using the turbo::init! macro
turbo::init! {
    struct GameState {
        frame: u32,
        gamestart: bool,
        gamestage: u32,
        round: u32,
        cursor_x: i32,
        cursor_y: i32,
        picked_race: i32
    } = {
        Self {
            frame: 0,
            gamestart: false,
            gamestage: 0,
            round: 0,
            cursor_x: 120,
            cursor_y: 100,
            picked_race: -1
        }
    }
}

fn calculate_distance_and_update_state(
    state: &mut GameState,
    race_center: (i32, i32),
    race_id: i32,
) -> i32 {
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
    if range_x.0 <= cursor_center_x
        && cursor_center_x <= range_x.1
        && range_y.0 <= cursor_center_y
        && cursor_center_y <= range_y.1
    {
        return true;
    } else {
        return false;
    }
}

// Implement the game loop using the turbo::go! macro
turbo::go! {
    // Load the game state
    let mut state = GameState::load();

    // get the gamepad state for player 1
    let p1_gamepad = gamepad(0);
    let race_names = ["Royal", "Humanoid", "Undead"];
    // let counter_string = state.round.to_string();
    // text!(&format!("Round: {}", state.round), x = 10, y = 10, font = Font::M, color = 0xffffffff); // Render the score

    if p1_gamepad.start.pressed() {
        state.gamestart = true;
        state.gamestage = 1;
    }

    // Handle user input
    if gamepad(0).left.pressed() {
        if (state.cursor_x - 3) >= 0 {
            state.cursor_x -= 3;
        }
    }
    if gamepad(0).right.pressed() {
        if (state.cursor_x + 3) <= 256 - 12 {
            state.cursor_x += 3;
        }
    }
    if gamepad(0).up.pressed() {
        if (state.cursor_y - 3) >= 0 {
            state.cursor_y -= 3;
        }

    }
    if gamepad(0).down.pressed() {
        if (state.cursor_y + 3) <= 144 - 16 {
            state.cursor_y += 3;
        }
    }

    // init screen
    if (state.gamestart == false) && (state.gamestage == 0) {
        clear(0x000000FF);

        text(60, 57, Font::L, 0x003d7dff,"Tale");
        text(100, 57, Font::L, 0xffffff50,"of");
        text(125, 57, Font::L, 0xef7c01ff,"Kentridge");

        text(80, 130, Font::M, 0xffffffff, "Press Space to Start");

        sprite!("nus", x = 110, y = 78);


        let user_pubkey = solana::user_pubkey();
        text(10, 10, Font::M, 0xffffffff, &format!("Welcome {} !", user_pubkey));

        let program_id: Pubkey = user_pubkey;
        let (pda_pubkey, bump_seed) = Pubkey::find_program_address(
            &[b"seed"],
            &program_id,
        );
        text(10, 20, Font::M, 0xffffffff, &format!("PDA pubkey: {}", pda_pubkey));
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
            text(curr_x, 20, Font::M, 0xffffffff, &("Picked ".to_owned() + race_names[state.picked_race as usize]));

            // render confirm button
            rect!(w = 100, h = 20, x = 80, y = 120);
            sprite!("punch", 80, y = 110);
            text(130, 125, Font::M, 0x000000FF, "Confirm");

        } else {
            text(100, 20, Font::M, 0xffffffff, "Pick a Race");
        }

        if gamepad(0).select.pressed() {
            let royal_center = (20 + 16, 50 + 16);
            let humanoid_center = (110 + 16, 50 + 16);
            let undead_center = (210 + 16, 50 + 16);

            if state.picked_race != -1 {
                let result = check_within_confirm(&mut state);
                if result {
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

        for i in 0..3 {
            for j in 0..3 {
                sprite!("grass", x = 80 + 16 * i, y = 48 + 16*j);
            }
        }
        for i in 0..3 {
            for j in 0..3 {
                sprite!("road", x = 128 + 16 * i, y = 48 + 16*j);
            }
        }

        for i in 0..4 {
            sprite!("paper", x = 32 + 46 * i, y = 100);
        }

        // set ui
        sprite!("squarepaper", x = 104, y = 0);
        text!(&format!("Round: {}", state.round), x = 108, y = 10, font = Font::M, color = 0x000000FF);

        if gamepad(0).select.pressed() {
            sprite!("confirm", x = state.cursor_x, y = state.cursor_y);
        } else {
            sprite!("hand", x = state.cursor_x, y = state.cursor_y);
        }
    }

    // game end
    if (state.gamestart == true) && (state.gamestage == 3) {
        let x = 80;
        let y = 70;
        let font = Font::M; // try Font::S or Font::L too
        let color = 0xffffffff;
        let message = "Game Over";
        text(x, y, font, color, message);
    }

    state.frame += 1;
    state.save();
}
