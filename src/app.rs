use crate::{
    assets::AssetsManager,
    controls::Controls,
    engine::stage::StageManager,
    pause::Pause,
    score::ScoreData,
    stage::stage1::stage_1,
    system::{update_draw, update_system},
    ui::{draw_entity_number, draw_fps, draw_version, fill_outside_game_window, StageUI},
    window::Window,
};

use hecs::World;
use macroquad::prelude::*;

pub struct App<'a> {
    window: Window,
    controls: Controls,
    world: World,
    score_data: ScoreData,
    assets_manager: AssetsManager,
    debugger: crate::engine::debug::Debugger,
    stage_manager: StageManager<'a>,
    pause: Pause,
}

impl App<'_> {
    /// Initialize Game state
    #[must_use]
    pub async fn new(window: Window, controls: Controls) -> Self {
        let world = World::new();
        let mut assets_manager = AssetsManager::default();
        let score_data = ScoreData::default();
        let mut stage_manager = StageManager::new(vec![stage_1()]);

        // TODO : Put this into Engine part
        let debugger = crate::engine::debug::Debugger::new();

        stage_manager.preload(&mut assets_manager, &window).await;
        Self {
            window,
            controls,
            world,
            score_data,
            assets_manager,
            debugger,
            stage_manager,
            pause: Pause::new(),
        }
    }

    /// This is where the update happen
    pub fn update(&mut self) {
        let time = get_time();
        self.pause.update(time, &self.controls);
        self.debugger.update(&self.window);
        self.window.update();
        if !self.pause.is_paused() {
            self.stage_manager
                .update(time, &mut self.world, &self.assets_manager);

            update_system(
                &mut self.world,
                &self.controls,
                &self.window,
                get_frame_time(),
                time,
                &mut self.score_data,
                &self.assets_manager,
                &self.pause,
            );
        }
    }

    /// This is where the draw happen
    pub async fn draw(&mut self) {
        clear_background(BLACK);

        let time = get_time();
        let delta = get_frame_time();
        self.stage_manager
            .draw(time, &self.window, &self.assets_manager);
        update_draw(
            &self.world,
            &self.controls,
            &self.window,
            time,
            delta,
            &self.assets_manager,
        );
        StageUI::draw(&self.window, &self.score_data, &self.assets_manager).await;

        draw_entity_number(&self.window, self.world.len());
        draw_fps(&self.window, 32.0, WHITE);
        draw_version(&self.window);

        self.pause.draw(time, delta, &self.window);

        self.debugger.draw(&self.window);
        fill_outside_game_window(&self.window);

        // draw_rectangle(
        //     self.window.playable_window().get_start().x,
        //     self.window.playable_window().get_start().y,
        //     self.window.playable_window().size().x,
        //     self.window.playable_window().size().y,
        //     Color::new(255f32, 0f32, 0f32, 0.5),
        // );
    }
}
