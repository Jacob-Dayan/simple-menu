//! I tried to document this code so it would be easier to contribue and maintain the game, but I ran into a problem!
//! I barely manage to maintain it!
//! good luck I guess <crying emoji>
//! (no budget for actual emoji, deal with that)

use simple_menu::*;

fn spawn_dot_in_box(dots: &mut Vec<CuteDot>, game_pos: Vec2, game_dim: Vec2) {
    let radius = 5.0;
    let x = macroquad::rand::gen_range(game_pos.x + radius, game_pos.x + game_dim.x - radius);
    let y = macroquad::rand::gen_range(game_pos.y + radius, game_pos.y + game_dim.y - radius);

    let mut dot = CuteDot::new(vec2(x, y));
    dot.radius = radius;
    dots.push(dot);
}

fn text_dimensions(text: &str, font_size: f32) -> Vec2 {
    let dims = measure_text(text, None, font_size as u16, 1.0);
    vec2(dims.width, dims.height)
}

// WORK IN PROGRESS FUNCTION, DO NOT USE YET
// actually never mind it's done now
// i dont really like how it returns a vec2 when it could return a struct with width and height fields but oh well
//  i might change that later
// please ignore the mess that is this function

fn game_msg(text: &str, position: Vec2, font_size: f32) {
    draw_text(text, position.x, position.y, font_size, WHITE);
}
//this function draws an outline around the screen, used for visual effect
fn screen_outline(x: f32, y: f32, w: f32, h: f32) {
    draw_rectangle_lines(x, y, w, h, 10.0, BLACK);
}
// simple button function, returns true if clicked yes i know it's basic but hey it works
fn button(x: f32, y: f32, w: f32, h: f32, label: &str) -> bool {
    let (mx, my) = mouse_position();
    let inside_x = mx >= x && mx <= x + w;
    let inside_y = my >= y && my <= y + h;
    let hovered = inside_x && inside_y;

    if hovered {
        draw_rectangle(x, y, w, h, LIGHTGRAY);
        draw_text(label, x + 10.0, y + h * 0.6, 31.0, BLACK);
    } else {
        draw_rectangle(x, y, w, h, GRAY);
        draw_text(label, x + 10.0, y + h * 0.6, 30.0, BLACK);
    }
    draw_rectangle_lines(x, y, w, h, 4.0, BLACK);

    hovered && is_mouse_button_pressed(MouseButton::Left)
}
fn dev_mode_display(dev_mode: bool, mouse: (f32, f32), fps: i32, amount_of_dots: i32) {
    // displays mouse coordinates and a dev mode message when dev mode is active
    // the function is called every frame, but only draws when dev_mode is true
    // which is efficient enough for this simple use case
    // but i should probably add some throttling or optimization if this were to be used in a more complex application oh well

    if dev_mode {
        let dev_txt_dim = text_dimensions("developer mode active", 20.0);
        let cords_txt_dim = text_dimensions(&format!("x: {} | y: {}", mouse.0, mouse.1), 20.0);
        let fps_txt_dim = text_dimensions(&format!("FPS: {}", fps), 20.0);
        let dots_txt_dim = text_dimensions(&format!("dots: {amount_of_dots}"), 20.0);

        let dev_dim = vec2(dev_txt_dim.x, dev_txt_dim.y);
        let fps_dim = vec2(fps_txt_dim.x, fps_txt_dim.y);
        let cords_dim = vec2(cords_txt_dim.x, cords_txt_dim.y);
        let dots_dim = vec2(dots_txt_dim.x, dots_txt_dim.y);

        draw_text(
            &format!("x: {} | y: {}", mouse.0, mouse.1),
            screen_width() * 0.85 - cords_dim.x / 2.0,
            screen_height() * 0.05 - cords_dim.y / 2.0,
            20.0,
            BLACK,
        );

        draw_text(
            "developer mode active",
            screen_width() * 0.85 - dev_dim.x / 2.0,
            screen_height() * 0.1 - dev_dim.y / 2.0,
            20.0,
            BLACK,
        );
        // although the fps is drawn every frame, it only updates every 10 frames in the main loop to reduce performance impact and change frequency
        // probably a less complex way to do this but it works for now
        draw_text(
            &format!("FPS: {fps}"),
            screen_width() * 0.85 - fps_dim.x / 2.0,
            screen_height() * 0.15 - fps_dim.y / 2.0,
            20.0,
            BLACK,
        );

        draw_text(
            &format!("dots: {amount_of_dots}"),
            screen_width() * 0.85 - dots_dim.x / 2.0,
            screen_height() * 0.2 - dots_dim.y / 2.0,
            20.0,
            BLACK,
        );
    }
}

use clap::Parser;
#[derive(Parser)]
#[command(version)]
struct Cli {
    #[arg(skip = env!("CARGO_PKG_VERSION"))]
    version: &'static str,
    #[arg(long)]
    dev_mode: bool,
}

#[macroquad::main("I click button, I happy")]
async fn main() {
    let cli = Cli::parse();

    // fps tracking variables
    let mut fps = 60;
    let mut i = 0;
    let max = 10;

    // lots of variables to keep track of the state of the game
    let mut dev_mode = cli.dev_mode;
    let mut state = CurrentState::MainMenu;
    let mut current_color = BgColor::Purple;

    let mut dots: Vec<CuteDot> = Vec::new();
    let mut amount_of_dots = 20;

    let rec_dim = vec2(500.0, 300.0);
    let rec_pos = vec2(
        screen_width() / 2.0 - rec_dim.x / 2.0,
        screen_height() / 2.0 - rec_dim.y / 2.0,
    );

    for _ in 0..amount_of_dots {
        let radius = 5.0;
        let x = macroquad::rand::gen_range(rec_pos.x + radius, rec_pos.x + rec_dim.x - radius);
        let y = macroquad::rand::gen_range(rec_pos.y + radius, rec_pos.y + rec_dim.y - radius);
        let mut dot = CuteDot::new(vec2(x, y));
        dot.radius = radius;
        dots.push(dot);
    }

    loop {
        let dt = get_frame_time();
        clear_background(Color::from(current_color));

        // for fps display in dev mode
        i += 1;
        if i >= max {
            i = 0;
            fps = get_fps();
        }
        // get screen dimensions and button dimensions
        let screen = vec2(screen_width(), screen_height());
        let btn_size = vec2(200.0, 60.0);
        let outline_size = vec2(screen_width(), screen_height());
        let mouse: (f32, f32) = mouse_position();
        // center position: screen/2 - size/2
        // this is used to center the button on the screen
        let btn_pos = vec2(
            screen.x / 2.0 - btn_size.x / 2.0,
            screen.y / 2.0 - btn_size.y / 2.0,
        );
        let outline_pos = vec2(
            screen.x / 2.0 - outline_size.x / 2.0,
            screen.y / 2.0 - outline_size.y / 2.0,
        );
        // draw the outline around the screen
        screen_outline(outline_pos.x, outline_pos.y, screen.x, screen.y);
        // display dev mode info if active
        dev_mode_display(dev_mode, mouse, fps, amount_of_dots);
        // this match statement handles the different states of the game, note how each state has its own UI and functionality
        match state {
            CurrentState::MainMenu => {
                let v_text = cli.version;
                let v_text_dims = measure_text(v_text, None, 20, 1.0);
                let game_text_pos = vec2(10.0, v_text_dims.height * 3.0);

                game_msg(v_text, game_text_pos, 40.0);

                if button(btn_pos.x, btn_pos.y, btn_size.x, btn_size.y, "START") {
                    println!("game started!");
                    state = CurrentState::Game;
                }
            }
            CurrentState::Game => {
                // actually calling it a game is a bit of a stretch
                let game_text = "game prototype:";
                let game_text_dims = measure_text(game_text, None, 40, 1.0);
                let game_text_pos = vec2(
                    screen_width() / 2.0 - game_text_dims.width / 2.0,
                    screen_height() / 4.0 - game_text_dims.height / 2.0,
                );
                let game_dim = vec2(500.0, 300.0);
                let game_pos = vec2(
                    screen_width() / 2.0 - game_dim.x / 2.0,
                    screen_height() / 2.0 - game_dim.y / 2.0,
                );
                let outline_size = 10.0;

                draw_rectangle(game_pos.x, game_pos.y, game_dim.x, game_dim.y, LIGHTGRAY);
                game_msg(game_text, game_text_pos, 40.0);
                // update and draw each cute dot
                let inside_game_pos = game_pos - vec2(outline_size / 2.0, outline_size / 2.0);

                for dot in &mut dots {
                    update_cute_dot(dot, dt, inside_game_pos, game_dim);
                }
                for dot in &dots {
                    dot.draw();
                }
                // draw the game area outline after drawing the dots to ensure it's on top
                draw_rectangle_lines(
                    game_pos.x,
                    game_pos.y,
                    game_dim.x,
                    game_dim.y,
                    outline_size,
                    BLACK,
                );
                if button(
                    screen_width() / 4.0,
                    screen_height() - 100.0,
                    btn_size.x,
                    btn_size.y,
                    "dots +",
                ) {
                    amount_of_dots += 1;
                    spawn_dot_in_box(&mut dots, inside_game_pos, game_dim);
                    println!("dots increased to {}", amount_of_dots);
                }
                if button(10.0, 10.0, btn_size.x, btn_size.y, "Settings") {
                    state = CurrentState::Settings;
                }
            }
            CurrentState::Settings => {
                // i might add more settings later but for now this is fine
                // also the settings text at the bottom is just a placeholder for now
                let settings_text = "change your settings as you desire! (SETTINGS SOON!!)";
                let settings_text_dims = measure_text(settings_text, None, 30, 1.0);
                let settings_text_pos: Vec2 = vec2(
                    screen_width() / 2.0 - settings_text_dims.width / 2.0,
                    screen_height() - settings_text_dims.height / 2.0,
                );
                game_msg(settings_text, settings_text_pos, 30.0);
                let color_btn_size = vec2(200.0, 50.0);
                if button(
                    screen_width() / 2.0 - btn_size.x / 2.0,
                    125.0,
                    color_btn_size.x,
                    color_btn_size.y,
                    &format!("Dev: {}", dev_mode),
                ) {
                    dev_mode = !dev_mode;
                }
                if button(
                    screen_width() / 2.0 - btn_size.x / 2.0,
                    50.0,
                    color_btn_size.x,
                    color_btn_size.y,
                    "Choose color",
                ) {
                    current_color = current_color.next();
                }

                if button(10.0, 10.0, btn_size.x, btn_size.y, "Back to game") {
                    state = CurrentState::Game;
                }
            }
        }
        next_frame().await;
    }
}

fn update_cute_dot(dot: &mut CuteDot, dt: f32, area_pos: Vec2, area_dim: Vec2) {
    dot.pos += dot.vel * dt;
    // each of these variables represents the boundaries of the area the dot can move in
    let left = area_pos.x + dot.radius;
    let right = area_pos.x + area_dim.x - dot.radius;
    let top = area_pos.y + dot.radius;
    let bottom = area_pos.y + area_dim.y - dot.radius;

    // below is simple collision detection with the area boundaries
    if dot.pos.x < left {
        dot.pos.x = left;
        dot.vel.x *= -1.0;
    } else if dot.pos.x > right {
        dot.pos.x = right;
        dot.vel.x *= -1.0;
    }

    if dot.pos.y < top {
        dot.pos.y = top;
        dot.vel.y *= -1.0;
    } else if dot.pos.y > bottom {
        dot.pos.y = bottom;
        dot.vel.y *= -1.0;
    }
}
