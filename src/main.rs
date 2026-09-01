use std::{collections::HashSet, time::Instant};
use uuidv4::uuid;
use macroquad::{prelude::*};
use ::rand::random_range;

const JUMP_VEL : f32 = -110.0;
const GRAVITY : f32 = 250.0;
const BIRD_SIZE : f32 = 10.0;
const BIRD_COLOR : Color = YELLOW;
const X_OFFSET : f32 = 35.0;
const PIPE_WIDTH : f32 = 15.0;
const PIPE_COLOR : Color = LIME;
const JUMP_DELAY : f32 = 0.3;
const PIPE_SPAWN_DELAY : f32 = 3.6;
const IS_DEBUGGING : bool = true;
const DEBUG_LOG_DELAY : f32 = 0.1;
const PIPE_SPEED : f32 = -35.0;
const MAX_GAP : f32 = 35.0;
const MIN_GAP : f32 = 15.0;
const MIN_PIPE_HEIGHT : f32 = 20.0;

struct Pipe {
    position : Vec2,
    pipe_height : f32,
    pair_id : String
}

impl Pipe {
    pub fn intersects_bird(&self, bird : &Bird) -> bool {
        Rect::new(self.position.x, self.position.y, PIPE_WIDTH, self.pipe_height).overlaps(&Rect::new(bird.position.x, bird.position.y, BIRD_SIZE, BIRD_SIZE))
    }

    pub fn update(&mut self, dt : f32) {
        self.position.x += PIPE_SPEED * dt;
    }

    pub fn get_pair_id(&self) -> String {
        self.pair_id.clone()
    }

    pub fn draw(&self) {
        draw_rectangle(self.position.x, self.position.y, PIPE_WIDTH, self.pipe_height, PIPE_COLOR);
    }
}

struct Bird {
    position : Vec2,
    y_velocity : f32,
    jump_timer : Instant,
    has_made_first_jump : bool,
    is_falling : bool
}

impl Bird {
    pub fn new() -> Self {
        Self { position: Vec2::new(X_OFFSET - BIRD_SIZE / 2.0, screen_height() / 2.0 - BIRD_SIZE / 2.0 ), y_velocity: 0.0, jump_timer: Instant::now(), has_made_first_jump: false, is_falling: false }
    }

    pub fn can_jump(&self) -> bool {
        if self.is_falling {
            return false;
        }
        
        if !is_key_pressed(KeyCode::W) && !is_key_pressed(KeyCode::Space) {
            return false;
        }

        if !self.has_made_first_jump {
            return true;
        }

        return elapsed_time(self.jump_timer) > JUMP_DELAY;
    }

    pub fn update(&mut self, dt : f32) {
        self.y_velocity += GRAVITY * dt;

        if self.can_jump() {
            self.jump();
        }

        self.position.y += self.y_velocity * dt;
    }

    pub fn fall(&mut self) {
        self.is_falling = true;
    }

    pub fn jump(&mut self) {
        self.has_made_first_jump = true;
        self.y_velocity = JUMP_VEL;

        reset_timer(&mut self.jump_timer);
    }

    pub fn has_passed_pipe(&self, pipe : &Pipe) -> bool {
        return self.position.x > pipe.position.x + PIPE_WIDTH;
    }

    pub fn draw(&self) {
        draw_rectangle(self.position.x, self.position.y, BIRD_SIZE, BIRD_SIZE, BIRD_COLOR);
    }
}

fn generate_pair(pipes : &mut Vec<Pipe>) {
    let gap = random_range(MIN_GAP..=MAX_GAP) + BIRD_SIZE;
    let center_pos = random_range(MIN_PIPE_HEIGHT + gap..=screen_height() - MIN_PIPE_HEIGHT - gap);
    let h1 = center_pos - gap;
    let h2 = screen_height() - center_pos - gap;
    let x = screen_width() + PIPE_WIDTH;
    let y_1 = 0.0;
    let y_2 = center_pos + gap;
    let position_1 = Vec2 { x, y : y_1 };
    let position_2 = Vec2 { x, y: y_2 };
    let id = uuid::v4();

    pipes.push(Pipe { position: position_1, pipe_height: h1, pair_id: id.clone() });
    pipes.push(Pipe { position: position_2, pipe_height: h2, pair_id: id.clone() });
}

fn window_config() -> Conf {
    Conf {
        window_title: "Flappy Bird".to_owned(),
        window_width: 325,
        window_height: 275,
        window_resizable: false,
        ..Default::default()
    }
}

fn elapsed_time(timer : Instant) -> f32 {
    return timer.elapsed().as_secs_f32();
}

fn reset_timer(timer : &mut Instant) {
    *timer = Instant::now();
}

#[macroquad::main(window_config)]
async fn main() {
    let mut bird = Bird::new();
    let mut pipes : Vec<Pipe> = Vec::new();
    let mut game_over = false;
    let mut spawn_pipe_timer = Instant::now();
    let mut passed_pipes : HashSet<String> = HashSet::new();
    let mut debug_timer = Instant::now();

    generate_pair(&mut pipes);
    
    let mut time = Instant::now();

    loop {
        next_frame().await;

        if elapsed_time(debug_timer) > DEBUG_LOG_DELAY && IS_DEBUGGING {
            reset_timer(&mut debug_timer);
            println!("Score: {}", passed_pipes.len());
            println!("Game state: {}", if game_over { "Game Over" } else { "Game Playing" });
            println!("Number of pipes: {}", pipes.len());
        }

        if elapsed_time(spawn_pipe_timer) > PIPE_SPAWN_DELAY {
            generate_pair(&mut pipes);
            reset_timer(&mut spawn_pipe_timer);
        }

        let dt = elapsed_time(time);
        reset_timer(&mut time);

        clear_background(BLACK);

        bird.update(dt);

        bird.draw();

        if game_over {
            bird.fall();
            continue;
        }

        for pipe in &mut pipes {

            if pipe.intersects_bird(&bird) {
                game_over = true;
                break;
            }

            if bird.has_passed_pipe(pipe) {
                passed_pipes.insert(pipe.get_pair_id());
            }

            pipe.update(dt);

            pipe.draw();
        }

        if bird.position.y >= screen_height() || bird.position.y < 0.0 {
            game_over = true;
        }

        pipes.retain(|pipe| pipe.position.x > -PIPE_WIDTH);

    }

}