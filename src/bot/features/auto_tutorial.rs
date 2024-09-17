/*
1.  `oLock the World``|Open inventory and place a `2My First World Lock``.|1|interface/tutorial/tut_npc.rttex|Open inventory and place a `2My First World Lock``.|1
2.  `oBreak Dirt Blocks``|Select the `2Fist`` and break some `2Dirt``!|2|interface/tutorial/tut_npc.rttex|Select the `2Fist`` and break some `2Dirt``!|1
3.  `oCollect Dirt Seeds``|Break the `2Dirt`` to collect `2Dirt Seeds``.|3|interface/tutorial/tut_npc.rttex|Break the `2Dirt`` to collect `2Dirt Seeds``.|1
4.  `oPlant Dirt Seeds``|Plant `2Dirt Seeds`` on the ground to grow a `2Dirt Tree``.|4|interface/tutorial/tut_npc.rttex|Plant `2Dirt Seeds`` on the ground to grow a `2Dirt Tree``.|1
5.  `oHarvest Dirt Trees``|Harvest the `2Dirt Tree`` that you planted!|5|interface/tutorial/tut_npc.rttex|Harvest the `2Dirt Tree`` that you planted!|1
6.  `oBreak Rock Blocks``|Select the `2Fist`` and break some `2Rock``!|19|interface/tutorial/tut_npc.rttex|Select the `2Fist`` and break some `2Rock``!|1
7.  `oCollect Rock Seeds``|Break the `2Rock`` to collect `2Rock Seeds``.|6|interface/tutorial/tut_npc.rttex|Break the `2Rock`` to collect `2Rock Seeds``.|1
8.  `oBreak Cave Backgrounds``|Select the `2Fist`` and break some `2Cave Background``!|20|interface/tutorial/tut_npc.rttex|Select the `2Fist`` and break some `2Cave Background``!|1
9.  `oCollect Cave Background Seeds``|Break the `2Cave Background`` to collect `2Cave Background Seeds``.|14|interface/tutorial/tut_npc.rttex|Break the `2Cave Background`` to collect `2Cave Background Seeds``.|1
10.
11.
12.
 */
use std::sync::Arc;
use std::thread;
use gtworld_r::TileType;
use crate::bot;
use crate::bot::Bot;
use crate::types::epacket_type::EPacketType;

static DIRT: u16 = 2;
static ROCK: u16 = 10;
static CAVE_BACKGROUND: u16 = 14;
static DIRT_SEEDS: u16 = 3;
static ROCK_SEED: u16 = 11;

pub fn lock_the_world(bot: &Arc<Bot>) {
    if !is_current_task(bot, "`oLock the World`") {
        return;
    }

    let bot_clone = bot.clone();
    thread::spawn(move || {
        bot::send_packet(&bot_clone, EPacketType::NetMessageGenericText, "ftue_start_popup_close`".to_string());
        thread::sleep(std::time::Duration::from_millis(1000));
        bot::place(&bot_clone, 0, -1, 9640);
        thread::sleep(std::time::Duration::from_millis(250));
    });
}

pub fn break_dirt_block(bot: &Arc<Bot>) {
    let bot_clone = bot.clone();

    thread::spawn(move || {
        while is_current_task(&bot_clone, "`oBreak Dirt Blocks`") {
            let tiles = {
                let world = bot_clone.world.read().unwrap();
                world.tiles.clone()
            };

            for tile in tiles.iter() {
                if tile.foreground_item_id == DIRT {
                    if !is_current_task(&bot_clone, "`oBreak Dirt Blocks`") {
                        return;
                    }

                    while {
                        let world = bot_clone.world.read().unwrap();
                        world.get_tile(tile.x, tile.y).unwrap().foreground_item_id == DIRT
                    } {
                        bot::find_path(&bot_clone, tile.x, tile.y - 1);
                        thread::sleep(std::time::Duration::from_millis(100));
                        bot::punch(&bot_clone, 0, 1);
                        thread::sleep(std::time::Duration::from_millis(250));
                    }
                }
            }
        }
    });
}

pub fn plant_dirt_seed(bot: &Arc<Bot>) {
    let bot_clone = bot.clone();

    thread::spawn(move || {
        while is_current_task(&bot_clone, "`oPlant Dirt Seeds`") {
            let tiles = {
                let world = bot_clone.world.read().unwrap();
                world.tiles.clone()
            };

            for tile in tiles.iter() {
                if tile.foreground_item_id == DIRT {
                    if !is_current_task(&bot_clone, "`oPlant Dirt Seeds`") {
                        return;
                    }

                    while {
                        let world = bot_clone.world.read().unwrap();
                        world.get_tile(tile.x, tile.y - 1).unwrap().foreground_item_id == 0
                    } {
                        bot::find_path(&bot_clone, tile.x, tile.y - 1);
                        thread::sleep(std::time::Duration::from_millis(100));
                        bot::place(&bot_clone, 0, 0, DIRT_SEEDS as u32);
                        thread::sleep(std::time::Duration::from_millis(250));
                    }
                }
            }
        }
    });
}

pub fn harvest_dirt_tree(bot: &Arc<Bot>) {
    let bot_clone = bot.clone();

    thread::spawn(move || {
        while is_current_task(&bot_clone, "`oHarvest Dirt Trees`") {
            let tiles = {
                let world = bot_clone.world.read().unwrap();
                world.tiles.clone().into_iter().filter(|tile| tile.foreground_item_id == DIRT_SEEDS).collect::<Vec<_>>()
            };

            for tile in tiles.iter() {
                if !is_current_task(&bot_clone, "`oHarvest Dirt Trees`") {
                    return;
                }

                while {
                    let world = bot_clone.world.read().unwrap();
                    world.is_tile_harvestable(&tile)
                } {
                    bot::find_path(&bot_clone, tile.x, tile.y);
                    thread::sleep(std::time::Duration::from_millis(100));
                    bot::punch(&bot_clone, 0, 0);
                    thread::sleep(std::time::Duration::from_millis(250));
                }
            }
        }
    });
}

pub fn break_rock_block(bot: &Arc<Bot>) {
    let bot_clone = bot.clone();

    thread::spawn(move || {
        while is_current_task(&bot_clone, "`oBreak Rock Blocks`") {
            let rock_tree_tiles = {
                let world = bot_clone.world.read().unwrap();
                world.tiles.clone().into_iter().filter(|tile| tile.foreground_item_id == ROCK_SEED).collect::<Vec<_>>()
            };

            for tile in rock_tree_tiles.iter() {
                let rock_amount = {
                    let inventory = bot_clone.inventory.read().unwrap();
                    inventory.items.get(&ROCK).map_or(0, |item| item.amount)
                };

                if rock_amount >= 5 {
                    return;
                }

                bot::find_path(&bot_clone, tile.x, tile.y);
                thread::sleep(std::time::Duration::from_millis(100));
                bot::punch(&bot_clone, 0, 0);
                thread::sleep(std::time::Duration::from_millis(250));
            }

            bot::find_path(&bot_clone, 0, 0);
            thread::sleep(std::time::Duration::from_millis(100));
            while is_current_task(&bot_clone, "`oBreak Rock Blocks`") {
                bot::place(&bot_clone, 1, 0, ROCK as u32);
                thread::sleep(std::time::Duration::from_millis(100));

                while {
                    let world = &bot_clone.world.read().unwrap();
                    world.get_tile(1, 0).map_or(false, |tile| tile.foreground_item_id == ROCK)
                } {
                    bot::punch(&bot_clone, 1, 0);
                    thread::sleep(std::time::Duration::from_millis(250));
                }
            }
        }
    });
}

pub fn collect_rock_seed(bot: &Arc<Bot>) {
    let bot_clone = bot.clone();

    thread::spawn(move || {
        while is_current_task(&bot_clone, "`oCollect Rock Seeds`") {
            let rock_tree_tiles = {
                let world = bot_clone.world.read().unwrap();
                world.tiles.clone().into_iter().filter(|tile| tile.foreground_item_id == ROCK_SEED).collect::<Vec<_>>()
            };

            for tile in rock_tree_tiles.iter() {
                if !is_current_task(&bot_clone, "`oCollect Rock Seeds`") {
                    return;
                }

                bot::find_path(&bot_clone, tile.x, tile.y);
                thread::sleep(std::time::Duration::from_millis(100));
                bot::punch(&bot_clone, 0, 0);
                thread::sleep(std::time::Duration::from_millis(250));
            }

            bot::find_path(&bot_clone, 0, 0);
            thread::sleep(std::time::Duration::from_millis(100));

            while is_current_task(&bot_clone, "`oCollect Rock Seeds`") {
                bot::place(&bot_clone, 1, 0, ROCK as u32);
                thread::sleep(std::time::Duration::from_millis(100));

                while {
                    let world = bot_clone.world.read().unwrap();
                    world.get_tile(1, 0).map_or(false, |tile| tile.foreground_item_id == ROCK)
                } {
                    bot::punch(&bot_clone, 1, 0);
                    thread::sleep(std::time::Duration::from_millis(250));
                }
            }
        }
    });
}

pub fn break_cave_background(bot: &Arc<Bot>) {
    let bot_clone = bot.clone();

    thread::spawn(move || {
        while is_current_task(&bot_clone, "`oBreak Cave Backgrounds`") {
            let dirt_tiles = {
                let world = bot_clone.world.read().unwrap();
                world.tiles.clone().into_iter().filter(|tile| tile.foreground_item_id == DIRT).collect::<Vec<_>>()
            };

            for tile in dirt_tiles.iter() {
                if !is_current_task(&bot_clone, "`oBreak Cave Backgrounds`") {
                    return;
                }

                while {
                    let world = bot_clone.world.read().unwrap();
                    let tile = world.get_tile(tile.x, tile.y).unwrap();
                    tile.background_item_id != 0 || tile.foreground_item_id != 0
                } {
                    bot::find_path(&bot_clone, tile.x, tile.y - 1);
                    thread::sleep(std::time::Duration::from_millis(100));
                    bot::punch(&bot_clone, 0, 1);
                    thread::sleep(std::time::Duration::from_millis(250));
                }
            }
        }
    });
}

fn is_current_task(bot: &Arc<Bot>, task: &str) -> bool {
    let ftue = bot.ftue.read().unwrap();
    ftue.info.contains(task)
}