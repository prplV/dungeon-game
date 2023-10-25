mod enemy;
use enemy::EnemyPattern;
use std::io;

extern crate rand;
use rand::Rng;

//use self::enemy::get_new_enemy;

struct Difficulty{
    hp: i32, 
    damage: i32,
    heal: i32,
}

enum BattleResult{
    PersonIsDead,
    EnemyisDead,
    Active,
}

pub struct Game{
    game_dif: Difficulty,
    person_name: String, 
    turn_id: i8,
}

// PLACE FOR GAMEMODE TRAITS //

// PLACE FOR GAMEMODE TRAITS //

impl Game{
    pub fn new() -> Game{
        let mut diff_console_ans = String::new();
        let mut name_console_ans = String::new();

        println!("What is your name, warrior?");
        io::stdin().read_line(&mut name_console_ans).expect("error with reading name");
        name_console_ans = name_console_ans.trim().to_string();
        
        println!("Which difficulty ypu prefer? (1..=3)");
        io::stdin().read_line(&mut diff_console_ans).expect("error with reading name");
        let mut diff_console_ans: usize = diff_console_ans.trim().parse().expect("error parsing diff");

        let diff = match diff_console_ans{
            1 => Difficulty{hp: 200, damage: 55, heal: 35},
            2 => Difficulty{hp: 100, damage: 45, heal: 30},
            3 => Difficulty{hp: 50, damage: 35, heal: 25},
            _ => panic!("error in diff choosing"),
        };
        return Game{
            game_dif: diff,
            person_name: name_console_ans,
            turn_id: 0,
        };
    }

    // generating vec with enemies and starting  
    pub fn start(&mut self) {
        let mut enemies: Vec<EnemyPattern> = Vec::new();
        enemies.push(EnemyPattern::get_new_enemy(1));
        enemies.push(EnemyPattern::get_new_enemy(2));
        enemies.push(EnemyPattern::get_new_enemy(3));
        println!("Your person's stats according to your difficulty chose:\nHP - {}\nDamage - {}\nHeal - {}", self.game_dif.hp, self.game_dif.damage, self.game_dif.heal);

        println!("\n\n*** You entered the Dungeon. What will it show you ?\n Oh, there is 3 opponents that would like you to be died.\n---    DON'T LET THEM STOP YOU!!!   ---\n");
        //

        'bigloop: for enemy in enemies{
            let mut new_enemy = enemy;
            println!("New battle!\nYour opponent is {}\nLVL - {}\nHP - {}\nDamage - {}\nLet's start a fight!\n", new_enemy.name, new_enemy.lvl, new_enemy.hp, new_enemy.damage);
            
            'inner_loop: loop{
                    match self.turn(&mut new_enemy){
                    BattleResult::PersonIsDead => {
                        println!("\nOh no! You're died!\n\nGAME OVER!\n");
                        break 'bigloop;
                    },
                    BattleResult::EnemyisDead => {
                        println!("Your enemy was defeted! Great job!\n");
                        break 'inner_loop;
                    },
                    BattleResult::Active => {
                        ()
                    },
                }
            }
        }

    }

    fn turn(&mut self, enemy: &mut EnemyPattern) -> BattleResult{
        let turn_num = self.turn_id + 1;
        self.turn_id = turn_num;
        println!("\n************** TURN {} **************\n", self.turn_id);
        let oracle_chose = rand::thread_rng().gen_range(1..=2);
        let mut res: u8;

        if oracle_chose == 1 {
            println!("Oracle let {} to be the first!", self.person_name);
            res = match self.person_turn(enemy){
                BattleResult::PersonIsDead => 1,
                BattleResult::EnemyisDead => 2,
                BattleResult::Active => 0,
            };

            if res == 2{
                println!("\n************* END TURN *************\n\n\n");
                self.turn_id = 0;
                return BattleResult::EnemyisDead;
            }
            else {
                res = match self.enemy_turn(enemy){
                    BattleResult::PersonIsDead => 1,
                    BattleResult::EnemyisDead => 2,
                    BattleResult::Active => 0,
                };
                if res == 1{
                    println!("\n************* END TURN *************\n\n\n");
                    self.turn_id = 0;
                    return BattleResult::PersonIsDead;
                }
                else{
                    println!("\n************* END TURN *************\n\n\n");
                    return BattleResult::Active;
                }
            }
        } else if oracle_chose == 2 {
            println!("Oracle let your enemy to be the first!");
            res = match self.enemy_turn(enemy){
                BattleResult::PersonIsDead => 1,
                BattleResult::EnemyisDead => 2,
                BattleResult::Active => 0,
            };

            if res == 1{
                println!("\n************* END TURN *************\n\n\n");
                self.turn_id = 0;
                return BattleResult::PersonIsDead;
            }
            else {
                res = match self.person_turn(enemy){
                    BattleResult::PersonIsDead => 1,
                    BattleResult::EnemyisDead => 2,
                    BattleResult::Active => 0,
                };
                if res == 2{
                    println!("\n************* END TURN *************\n\n\n");
                    self.turn_id = 0;
                    return BattleResult::EnemyisDead;
                }
                else{
                    println!("\n************* END TURN *************\n\n\n");
                    return BattleResult::Active;
                }
            }

        } else {
            panic!("error with generating Oracle's answer");

        }
        
    }
    fn person_turn(&mut self, enemy: &mut EnemyPattern) -> BattleResult {
        println!("********************\nYour person's stats:\nHP - {}\nDamage - {}\nHeal ability - {} hps\n********************", self.game_dif.hp, self.game_dif.damage, self.game_dif.heal);
        println!("********************\n{}'s stats:\nHP - {}\nDamage - {}\n********************", enemy.name, enemy.hp, enemy.damage);
        loop {
            println!("What would you like to do?\n1) Attack\n2) Heal");
            let mut ans = String::new();
            io::stdin().read_line(&mut ans).expect("error with reading turn ans");
            let ans: isize = ans.trim().parse().expect("error parsing turn ans");

            match ans{
                1 => {
                    let en_hp = enemy.hp;
                    enemy.hp = en_hp - self.game_dif.damage;
                    println!("$$$$$$$$$$$$$$$\nKick from the {}!\n$$$$$$$$$$$$$$$\n\n", self.person_name);
                    break;
                },
                2 => {
                    let self_hp = self.game_dif.hp;
                    self.game_dif.hp = self_hp + self.game_dif.heal;
                    println!("$$$$$$$$$$$$$$$\n{} healed himself!\n$$$$$$$$$$$$$$$\n\n", self.person_name);
                    break;
                },
                _ => {
                    println!("Unknown command - {}", ans);
                    continue;
                },
            }
        }
        if (enemy.hp <= 0)
        {
            return BattleResult::EnemyisDead;
        }
        return BattleResult::Active;
    }
    fn enemy_turn(&mut self, enemy: &mut EnemyPattern) -> BattleResult {
        if (enemy.hp <= self.game_dif.damage)
        {
            let en_hp = enemy.hp;
            enemy.hp = en_hp + enemy.damage;
            println!("$$$$$$$$$$$$$$$\n{} healed himself!\n$$$$$$$$$$$$$$$\n\n", enemy.name);
        }
        else{
            let self_hp = self.game_dif.hp;
            self.game_dif.hp = self_hp - enemy.damage;
            println!("$$$$$$$$$$$$$$$\nKick from the {}!\n$$$$$$$$$$$$$$$\n\n", enemy.name);
        }

        if (self.game_dif.hp <= 0)
        {
            return BattleResult::PersonIsDead;
        }
        return BattleResult::Active;
    }
}