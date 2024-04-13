use macroquad::prelude::*;

use crate::{
    controls::Action,
    engine::{controls::Controls, text::draw_text_ex2},
};

#[derive(Debug, PartialEq)]
pub enum PauseChoice {
    Resume,
    Restart,
    Exit,
}

#[derive(Debug)]
pub struct Pause {
    selected_choice: PauseChoice,
    currently_paused: bool,
}

impl Default for Pause {
    fn default() -> Self {
        Self {
            selected_choice: PauseChoice::Resume,
            currently_paused: false,
        }
    }
}

impl Pause {
    const INACTIVE: Color = GRAY;
    const ACTIVE: Color = WHITE;

    pub fn is_paused(&self) -> bool {
        !self.currently_paused
    }

    pub fn update(&mut self, controls: &Controls<Action>) {
        if controls.is_pressed(Action::Escape) {
            self.currently_paused = !self.currently_paused;
        }
    }

    pub fn draw(&self, font: &Font) {
        if !self.currently_paused {
            return;
        }

        draw_rectangle(0., 0., 1., 1., Color::new(0., 0., 0., 0.75));

        draw_text_ex2(
            "Game Paused",
            0.51,
            0.40,
            0.05,
            Self::ACTIVE,
            true,
            Some(font),
        );

        if self.selected_choice == PauseChoice::Resume {
            draw_text_ex2("Resume", 0.51, 0.45, 0.05, Self::ACTIVE, true, Some(font));
        } else {
            draw_text_ex2("Resume", 0.51, 0.45, 0.05, Self::INACTIVE, true, Some(font));
        }

        if self.selected_choice == PauseChoice::Restart {
            draw_text_ex2("Restart", 0.51, 0.50, 0.05, Self::ACTIVE, true, Some(font));
        } else {
            draw_text_ex2(
                "Restart",
                0.51,
                0.50,
                0.05,
                Self::INACTIVE,
                true,
                Some(font),
            );
        }

        if self.selected_choice == PauseChoice::Exit {
            draw_text_ex2("Exit", 0.50, 0.55, 0.05, Self::ACTIVE, true, Some(font));
        } else {
            draw_text_ex2("Exit", 0.50, 0.55, 0.05, Self::INACTIVE, true, Some(font));
        }
    }
}
