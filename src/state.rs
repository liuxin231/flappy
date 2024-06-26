use crate::obstacle::Obstacle;
use crate::player::Player;
use crate::{GameMode, FRAME_DURATION, SCREEN_HEIGHT, SCREEN_WIDTH};
use bracket_lib::color::NAVY;
use bracket_lib::prelude::{BTerm, GameState, VirtualKeyCode};

#[derive(Debug)]
pub struct State {
    score: i32,
    obstacle: Obstacle,
    mode: GameMode,
    frame_time: f32,
    player: Player,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx),
        }
    }
}

impl State {
    pub fn new() -> Self {
        Self {
            score: 0,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            mode: GameMode::Menu,
            frame_time: 0.,
            player: Player::new(5, 25),
        }
    }

    pub fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.;
            self.player.gravity_and_move();
        }

        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }

        self.player.render(ctx);
        ctx.print(0, 0, "Press SPACE to flap.");
        ctx.print(0, 1, format!("Score: {}", self.score));
        self.obstacle.render(ctx, self.player.x);

        if self.player.x > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
        }

        if self.player.y > SCREEN_HEIGHT || self.obstacle.hit_obstacle(&self.player) {
            self.mode = GameMode::End;
        }
    }

    pub fn restart(&mut self) {
        self.mode = GameMode::Playing;
        self.player = Player::new(5, 25);
        self.frame_time = 0.;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
        self.score = 0;
    }

    pub fn dead(&mut self, ctx: &mut BTerm) {
        let start_y = 20;
        ctx.cls();
        ctx.print_centered(start_y, "You are dead!");
        ctx.print_centered(start_y + 2, format!("You earned {} points", self.score));
        ctx.print_centered(start_y + 5, "(P) Play Again");
        ctx.print_centered(start_y + 8, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    pub fn main_menu(&mut self, ctx: &mut BTerm) {
        let start_y = 20;
        ctx.cls();
        ctx.print_centered(start_y, "Welcome to Flappy Dragon");
        ctx.print_centered(start_y + 5, "(P) Play Game");
        ctx.print_centered(start_y + 8, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
}
