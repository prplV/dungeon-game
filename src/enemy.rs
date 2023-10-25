pub enum Enemy{
    Goblin,
    Ogre,
    DungeonKeeper,
}
pub struct EnemyPattern{
    pub name:  String, 
    pub lvl: u32,
    pub hp: i32,
    pub damage: i32, 
}

impl EnemyPattern{
    pub fn get_new_enemy(en: u8) -> EnemyPattern{
        let mut enemy_struct = match en{
            1 => EnemyPattern{ name: String::from("Goblin"), lvl: 1, hp: 60, damage: 15},
            2 => EnemyPattern{ name: String::from("Ogre"), lvl: 2, hp: 100, damage: 35},
            3 => EnemyPattern{ name: String::from("Dungeon Keeper"), lvl: 3, hp: 200, damage: 55},
            _ => panic!("This is not a enemy lvl stat!"),
        };
        return enemy_struct;
    }
}