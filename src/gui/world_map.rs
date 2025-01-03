use crate::texture_manager::TextureManager;
use crate::{manager::bot_manager::BotManager, types::config::BotConfig, utils};
use eframe::egui::{self, Color32, Pos2, Rect, Ui};
use egui::{Painter, TextBuffer};
use gtworld_r::TileType;
use paris::info;
use std::sync::{Arc, RwLock};
use std::thread;

#[derive(Default)]
pub struct WorldMap {
    pub selected_bot: String,
    pub warp_name: String,
    pub bots: Vec<BotConfig>,
    camera_pos: Pos2,
    zoom: f32,
}

impl WorldMap {
    pub fn render(
        &mut self,
        ui: &mut Ui,
        manager: &Arc<RwLock<BotManager>>,
        texture_manager: &Arc<RwLock<TextureManager>>,
    ) {
        self.bots = utils::config::get_bots();
        self.selected_bot = utils::config::get_selected_bot();

        if !self.selected_bot.is_empty() {
            let bot = {
                let manager = manager.read().unwrap();

                match manager.get_bot(&self.selected_bot) {
                    Some(bot) => Some(bot.clone()),
                    None => None,
                }
            };
            if let Some(bot) = bot {
                let size = ui.available_size();
                let (rect, response) = ui.allocate_exact_size(size, egui::Sense::hover());
                let draw_list = ui.painter_at(rect);

                draw_list.rect_filled(rect, 0.0, Color32::from_rgb(96, 215, 255));
                match texture_manager.read().unwrap().get_texture("hills3.rttex") {
                    Some(texture) => {
                        let uv_start = Pos2::new(0.0, 0.0);
                        let uv_end = Pos2::new(1.0, 1.0);
                        draw_list.image(texture.id(), rect, Rect::from_min_max(uv_start, uv_end), Color32::WHITE);
                    }
                    None => (),
                }
                match texture_manager.read().unwrap().get_texture("hills2.rttex") {
                    Some(texture) => {
                        let uv_start = Pos2::new(0.0, 0.0);
                        let uv_end = Pos2::new(1.0, 1.0);
                        draw_list.image(texture.id(), rect, Rect::from_min_max(uv_start, uv_end), Color32::WHITE);
                    }
                    None => (),
                }
                match texture_manager.read().unwrap().get_texture("hills1.rttex") {
                    Some(texture) => {
                        let uv_start = Pos2::new(0.0, 0.0);
                        let uv_end = Pos2::new(1.0, 1.0);
                        draw_list.image(texture.id(), rect, Rect::from_min_max(uv_start, uv_end), Color32::WHITE);
                    }
                    None => (),
                }
                match texture_manager.read().unwrap().get_texture("sun.rttex") {
                    Some(texture) => {
                        let uv_start = Pos2::new(0.0, 0.0);
                        let uv_end = Pos2::new(1.0, 1.0);
                        let offset_rect = Rect::from_min_max(
                            Pos2::new(rect.min.x - 20.0, rect.min.y - 20.0),
                            Pos2::new(rect.min.x - 20.0 + 150.0, rect.min.y - 20.0 + 150.0),
                        );
                        draw_list.image(texture.id(), offset_rect, Rect::from_min_max(uv_start, uv_end), Color32::WHITE);
                    }
                    None => (),
                }

                if self.camera_pos == Pos2::default() {
                    let bot_position = bot.position.lock().unwrap();
                    self.camera_pos = Pos2::new(bot_position.x, bot_position.y);
                    self.zoom = 0.5;
                }

                {
                    let bot_position = bot.position.lock().unwrap();
                    let target_pos = Pos2::new(bot_position.x, bot_position.y);
                    let smoothing_factor = 0.1;
                    self.camera_pos.x += (target_pos.x - self.camera_pos.x) * smoothing_factor;
                    self.camera_pos.y += (target_pos.y - self.camera_pos.y) * smoothing_factor;
                }

                let cell_size = 32.0 * self.zoom;
                let camera_tile_x = (self.camera_pos.x / 32.0).floor() as i32;
                let camera_tile_y = (self.camera_pos.y / 32.0).floor() as i32;
                let offset_x = (self.camera_pos.x % 32.0) * self.zoom;
                let offset_y = (self.camera_pos.y % 32.0) * self.zoom;

                let tiles_in_view_x = (size.x / cell_size).ceil() as i32 + 1;
                let tiles_in_view_y = (size.y / cell_size).ceil() as i32 + 1;

                let world = bot.world.read().unwrap();
                for y in 0..tiles_in_view_y {
                    for x in 0..tiles_in_view_x {
                        let world_x = camera_tile_x + x - tiles_in_view_x / 2;
                        let world_y = camera_tile_y + y - tiles_in_view_y / 2;

                        let cell_min = Pos2::new(
                            rect.min.x + x as f32 * cell_size - offset_x,
                            rect.min.y + y as f32 * cell_size - offset_y,
                        );
                        let cell_max = Pos2::new(cell_min.x + cell_size, cell_min.y + cell_size);

                        if world_x < 0
                            || world_y < 0
                            || world_x >= world.width as i32
                            || world_y >= world.height as i32
                        {
                            continue;
                        }

                        if (world_y * world.width as i32 + world_x) >= world.tile_count as i32 {
                            draw_list.rect_filled(
                                Rect::from_min_max(cell_min, cell_max),
                                0.0,
                                Color32::from_rgb(255, 215, 0),
                            );
                            continue;
                        }
                        let tile = world.get_tile(world_x as u32, world_y as u32).unwrap();
                        let item = {
                            let item_database = bot.item_database.read().unwrap();
                            item_database
                                .get_item(&(tile.foreground_item_id as u32))
                                .unwrap()
                        };

                        if tile.background_item_id != 0 {
                            let item_database = bot.item_database.read().unwrap();
                            let background_item = item_database
                                .get_item(&((tile.background_item_id + 1) as u32))
                                .unwrap();

                            self.draw_texture(
                                &draw_list,
                                texture_manager,
                                background_item.texture_x,
                                background_item.texture_y,
                                background_item.texture_file_name.clone(),
                                cell_min,
                                cell_max,
                                tile.flags.flipped_x,
                                Color32::WHITE,
                            );
                        }

                        if item.id != 0 {
                            let mut texture_x = item.texture_x;
                            let mut texture_y = item.texture_y;
                            let texture_name = item.texture_file_name.clone();

                            let left_tile = if world_x > 0 {
                                world.get_tile(world_x as u32 - 1, world_y as u32)
                            } else {
                                None
                            };
                            let right_tile = if world_x < world.width as i32 - 1 {
                                world.get_tile(world_x as u32 + 1, world_y as u32)
                            } else {
                                None
                            };
                            let top_tile = if world_y > 0 {
                                world.get_tile(world_x as u32, world_y as u32 - 1)
                            } else {
                                None
                            };
                            let bottom_tile = if world_y < world.height as i32 - 1 {
                                world.get_tile(world_x as u32, world_y as u32 + 1)
                            } else {
                                None
                            };

                            if item.render_type == 2 {
                                if let (
                                    Some(left_tile),
                                    Some(right_tile),
                                    Some(top_tile),
                                    Some(bottom_tile),
                                ) = (left_tile, right_tile, top_tile, bottom_tile)
                                {
                                    let left_match = left_tile.foreground_item_id == item.id as u16;
                                    let right_match =
                                        right_tile.foreground_item_id == item.id as u16;
                                    let top_match = top_tile.foreground_item_id == item.id as u16;
                                    let bottom_match =
                                        bottom_tile.foreground_item_id == item.id as u16;

                                    match (left_match, right_match, top_match, bottom_match) {
                                        (true, true, true, true) => (),
                                        (true, true, true, false) => texture_x += 2,
                                        (true, true, false, true) => texture_x += 1,
                                        (true, false, true, true) => texture_x += 4,
                                        (false, true, true, true) => texture_x += 3,
                                        (true, true, false, false) => texture_x += 1,
                                        (true, false, false, true) => texture_x += 6,
                                        (false, true, true, false) => texture_x += 7,
                                        (false, true, false, true) => texture_x += 5,
                                        (true, false, false, false) => texture_x += 6,
                                        (false, false, false, true) => {
                                            texture_x += 2;
                                            texture_y += 1;
                                        }
                                        (false, true, false, false) => texture_x += 5,
                                        _ => (),
                                    }
                                }

                                if let (None, Some(right_tile), Some(top_tile), Some(bottom_tile)) =
                                    (left_tile, right_tile, top_tile, bottom_tile)
                                {
                                    let right_match =
                                        right_tile.foreground_item_id == item.id as u16;
                                    let bottom_match =
                                        bottom_tile.foreground_item_id == item.id as u16;
                                    let top_match = top_tile.foreground_item_id != item.id as u16;

                                    if right_match && bottom_match && top_match {
                                        texture_x += 1;
                                    }
                                }

                                if let (Some(left_tile), None, Some(top_tile), Some(bottom_tile)) =
                                    (left_tile, right_tile, top_tile, bottom_tile)
                                {
                                    let left_match = left_tile.foreground_item_id == item.id as u16;
                                    let bottom_match =
                                        bottom_tile.foreground_item_id == item.id as u16;
                                    let top_match = top_tile.foreground_item_id != item.id as u16;

                                    if left_match && bottom_match && top_match {
                                        texture_x += 1;
                                    }
                                }
                            }

                            if item.render_type == 7 {
                                if let (Some(top_tile), Some(bottom_tile)) = (top_tile, bottom_tile)
                                {
                                    if top_tile.foreground_item_id != item.id as u16
                                        && bottom_tile.foreground_item_id == item.id as u16
                                    {
                                        texture_x += 2;
                                    }
                                    if top_tile.foreground_item_id == item.id as u16
                                        && bottom_tile.foreground_item_id == item.id as u16
                                    {
                                        texture_x += 1;
                                    }
                                    if top_tile.foreground_item_id != item.id as u16
                                        && bottom_tile.foreground_item_id != item.id as u16
                                    {
                                        texture_x += 3;
                                    }
                                }
                            }

                            if item.render_type == 3 {
                                if let (Some(left_tile), Some(right_tile), Some(top_tile)) =
                                    (left_tile, right_tile, top_tile)
                                {
                                    if left_tile.foreground_item_id == item.id as u16
                                        && right_tile.foreground_item_id == item.id as u16
                                    {
                                        texture_x += 1;
                                    }
                                    if left_tile.foreground_item_id != item.id as u16
                                        || right_tile.foreground_item_id != item.id as u16
                                    {
                                        if top_tile.foreground_item_id == 8986 {
                                            texture_x += 4;
                                        }
                                    }
                                }
                            }

                            if item.render_type == 5 {
                                if let (
                                    Some(left_tile),
                                    Some(right_tile),
                                    Some(top_tile),
                                    Some(bottom_tile),
                                ) = (left_tile, right_tile, top_tile, bottom_tile)
                                {
                                    let left_match = left_tile.foreground_item_id == item.id as u16;
                                    let right_match =
                                        right_tile.foreground_item_id == item.id as u16;
                                    let top_match = top_tile.foreground_item_id == item.id as u16;
                                    let bottom_match =
                                        bottom_tile.foreground_item_id == item.id as u16;

                                    if (left_match && !right_match && !top_match && !bottom_match)
                                        || (!left_match
                                            && right_match
                                            && !top_match
                                            && !bottom_match)
                                    {
                                        texture_x += 7;
                                    }
                                }
                            }

                            if item.id % 2 != 0 {
                                let (b, g, r, a) = utils::color::extract_bgra(item.overlay_color);
                                let (spread_x, spread_y) = match item.render_type {
                                    2 | 5 => (4.0, 1.0),
                                    4 => (4.0, 0.0),
                                    3 | 7 | 8 | 9 | 10 => (3.0, 0.0),
                                    _ => (0.0, 0.0),
                                };

                                self.draw_texture(
                                    &draw_list,
                                    texture_manager,
                                    item.tree_base_sprite,
                                    19,
                                    "tiles_page1.rttex".to_string(),
                                    cell_min,
                                    cell_max,
                                    tile.flags.flipped_x,
                                    Color32::WHITE,
                                );
                                self.draw_texture(
                                    &draw_list,
                                    texture_manager,
                                    item.tree_overlay_sprite,
                                    18,
                                    "tiles_page1.rttex".to_string(),
                                    cell_min,
                                    cell_max,
                                    tile.flags.flipped_x,
                                    Color32::from_rgba_unmultiplied(r, g, b, a),
                                );
                                let new_cell_min = Pos2::new(
                                    cell_min.x + cell_size * 0.375,
                                    cell_min.y + cell_size * 0.375,
                                );
                                let new_cell_max = Pos2::new(
                                    cell_max.x - cell_size * 0.375,
                                    cell_max.y - cell_size * 0.375,
                                );

                                self.draw_texture(
                                    &draw_list,
                                    texture_manager,
                                    texture_x + spread_x as u8,
                                    texture_y + spread_y as u8,
                                    texture_name,
                                    new_cell_min,
                                    new_cell_max,
                                    tile.flags.flipped_x,
                                    Color32::WHITE,
                                );

                                draw_list.rect_stroke(
                                    Rect::from_min_max(new_cell_min, new_cell_max),
                                    0.2,
                                    (2.0, Color32::WHITE),
                                );
                            } else {
                                self.draw_texture(
                                    &draw_list,
                                    texture_manager,
                                    texture_x,
                                    texture_y,
                                    texture_name,
                                    cell_min,
                                    cell_max,
                                    tile.flags.flipped_x,
                                    Color32::WHITE,
                                );
                            }
                        }

                        for player in bot.players.lock().unwrap().clone() {
                            if (player.position.x / 32.0).floor() == (world_x as f32)
                                && (player.position.y / 32.0).floor() == (world_y as f32)
                            {
                                let center_min = Pos2::new(
                                    cell_min.x + cell_size * 0.25,
                                    cell_min.y + cell_size * 0.25,
                                );
                                let center_max = Pos2::new(
                                    cell_max.x - cell_size * 0.25,
                                    cell_max.y - cell_size * 0.25,
                                );
                                self.draw_texture(
                                    &draw_list,
                                    texture_manager,
                                    17,
                                    5,
                                    "player_cosmetics1.rttex".to_string(),
                                    center_min,
                                    center_max,
                                    tile.flags.flipped_x,
                                    Color32::WHITE,
                                );
                            }
                        }

                        let bot_position = bot.position.lock().unwrap();
                        if (bot_position.x / 32.0).floor() == (world_x as f32)
                            && (bot_position.y / 32.0).floor() == (world_y as f32)
                        {
                            self.draw_texture(
                                &draw_list,
                                texture_manager,
                                3,
                                4,
                                "tiles_page1.rttex".to_string(),
                                cell_min,
                                cell_max,
                                tile.flags.flipped_x,
                                Color32::WHITE,
                            );
                        }

                        if response.hover_pos().map_or(false, |pos| {
                            Rect::from_min_max(cell_min, cell_max).contains(pos)
                        }) {
                            let data;
                            if let TileType::Seed {
                                ready_to_harvest,
                                timer,
                                ..
                            } = &tile.tile_type
                            {
                                let elapsed = timer.elapsed().as_secs();
                                let ready_to_harvest = if *ready_to_harvest {
                                    "Yes"
                                } else {
                                    if world.is_tile_harvestable(tile) {
                                        "Yes"
                                    } else {
                                        "No"
                                    }
                                };
                                data = format!(
                                    "Position: {}|{}\nItem name: {}\nCollision type: {}\nReady to harvest: {}\nTime passed: {}\nRender type: {}",
                                    world_x, world_y, item.name, item.collision_type, ready_to_harvest, elapsed, item.render_type
                                )
                            } else {
                                data = format!(
                                    "Position: {}|{}\nItem name: {}\nCollision type: {}\nRender type: {}",
                                    world_x, world_y, item.name, item.collision_type, item.render_type
                                )
                            }

                            egui::show_tooltip(
                                ui.ctx(),
                                ui.layer_id(),
                                egui::Id::new("tile_info"),
                                |ui| {
                                    ui.label(egui::RichText::new(data).monospace());
                                },
                            );

                            if ui.input(|i| i.pointer.any_click()) {
                                info!("Clicked on tile: {}|{}", world_x, world_y);
                                let bot_clone = bot.clone();
                                thread::spawn(move || {
                                    bot_clone.find_path(world_x as u32, world_y as u32);
                                });
                            }
                        }
                    }
                }

                egui::Window::new("Movement")
                    .anchor(egui::Align2::RIGHT_BOTTOM, [0.0, 0.0])
                    .default_open(false)
                    .show(ui.ctx(), |ui| {
                        ui.horizontal(|ui| {
                            if ui.button("Up").clicked() {
                                let bot_clone = bot.clone();
                                thread::spawn(move || {
                                    bot_clone.walk(0, -1, false);
                                });
                            }
                            if ui.button("Down").clicked() {
                                let bot_clone = bot.clone();
                                thread::spawn(move || {
                                    bot_clone.walk(0, 1, false);
                                });
                            }
                            if ui.button("Left").clicked() {
                                let bot_clone = bot.clone();
                                thread::spawn(move || {
                                    bot_clone.walk(-1, 0, false);
                                });
                            }
                            if ui.button("Right").clicked() {
                                let bot_clone = bot.clone();
                                thread::spawn(move || {
                                    bot_clone.walk(1, 0, false);
                                });
                            }
                            ui.add(egui::Slider::new(&mut self.zoom, 0.1..=2.0).text("Zoom"));
                        });
                    });

                egui::Window::new("FTUE")
                    .anchor(egui::Align2::LEFT_BOTTOM, [0.0, 0.0])
                    .default_open(false)
                    .show(ui.ctx(), |ui| {
                        ui.vertical(|ui| {
                            let ftue = {
                                let ftue = bot.ftue.lock().unwrap();
                                ftue.clone()
                            };

                            ui.label(format!("FTUE: {}", ftue.info));
                            ui.label(format!(
                                "Current progress: {}/{}",
                                ftue.current_progress, ftue.total_progress
                            ));
                        });
                    });
            }
        }
    }

    fn draw_texture(
        &self,
        draw_list: &Painter,
        texture_manager: &Arc<RwLock<TextureManager>>,
        texture_x: u8,
        texture_y: u8,
        texture_name: String,
        cell_min: Pos2,
        cell_max: Pos2,
        flipped: bool,
        color: Color32,
    ) {
        match texture_manager.read().unwrap().get_texture(&texture_name) {
            Some(texture) => {
                let [width, height] = texture.size();
                let uv_x_start = (texture_x as f32 * 32.0) / width as f32;
                let uv_y_start = (texture_y as f32 * 32.0) / height as f32;
                let uv_x_end = ((texture_x as f32 * 32.0) + 32.0) / width as f32;
                let uv_y_end = ((texture_y as f32 * 32.0) + 32.0) / height as f32;

                let (uv_start, uv_end) = if flipped {
                    (
                        egui::Pos2::new(uv_x_end, uv_y_start),
                        egui::Pos2::new(uv_x_start, uv_y_end),
                    )
                } else {
                    (
                        egui::Pos2::new(uv_x_start, uv_y_start),
                        egui::Pos2::new(uv_x_end, uv_y_end),
                    )
                };

                let cell_min = Pos2::new(cell_min.x.round(), cell_min.y.round());
                let cell_max = Pos2::new(cell_max.x.round(), cell_max.y.round());

                draw_list.image(
                    texture.id(),
                    Rect::from_min_max(
                        Pos2::new(cell_min.x, cell_min.y),
                        Pos2::new(cell_max.x, cell_max.y),
                    ),
                    egui::Rect::from_min_max(uv_start, uv_end),
                    color,
                );
            }
            None => (),
        }
    }
}
