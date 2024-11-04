pub struct Race {
    pub id: i32,
    pub name: &'static str,
}

pub struct Card {
    pub id: i32,
    pub mana: i32,
    pub hp: i32,
    pub attack: i32,
    pub curr_hp: i32,
    pub name: &'static str,
    pub race: i32,
    pub rarity: i32,
}
