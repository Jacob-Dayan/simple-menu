//! I tried to document this code so it would be easier to contribue and maintain the game, but I ran into a problem!
//! I barely manage to maintain it!
//! good luck I guess <crying emoji>
//! (no budget for actual emoji, deal with that)

use simple_menu::*;

const DEFAULT_DOTS_COUNT: usize = 20;
const GAME_AREA_SIZE: Vec2 = vec2(500.0, 300.0);
const BTN_SIZE: Vec2 = vec2(200.0, 60.0);
const COLOR_BTN_SIZE: Vec2 = vec2(200.0, 50.0);
const SCREEN_OUTLINE_THICKNESS: f32 = 10.0;
const GAME_OUTLINE_THICKNESS: f32 = 10.0;
const BTN_OUTLINE_THICKNESS: f32 = 4.0;
const FPS_UPDATE_INTERVAL: i32 = 10;

fn game_bounds() -> Rect {
    Rect::new(
        (screen_width() - GAME_AREA_SIZE.x) / 2.0,
        (screen_height() - GAME_AREA_SIZE.y) / 2.0,
        GAME_AREA_SIZE.x,
        GAME_AREA_SIZE.y,
    )
}

fn centered_rect(center_x: f32, y: f32, size: Vec2) -> Rect {
    Rect::new(center_x - size.x / 2.0, y, size.x, size.y)
}

// FORMERLY "WORK IN PROGRESS FUNCTION, DO NOT USE YET"
// The existential crisis has been resolved: it now returns TextDimensions with actual width
// and height fields instead of stuffing dimensions into a Vec2 like a barbarian.
fn text_dimensions(text: &str, font_size: f32) -> TextDimensions {
    measure_text(text, None, font_size as u16, 1.0)
}

fn centered_text_pos(text: &str, center_x: f32, center_y: f32, font_size: f32) -> Vec2 {
    let dims = text_dimensions(text, font_size);
    vec2(center_x - dims.width / 2.0, center_y - dims.height / 2.0)
}

fn game_msg(text: &str, position: Vec2, font_size: f32) {
    draw_text(text, position.x, position.y, font_size, WHITE);
}

//this function draws an outline around the screen, used for visual effect
fn screen_outline() {
    draw_rectangle_lines(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        SCREEN_OUTLINE_THICKNESS,
        BLACK,
    );
}

// simple button function, returns true if clicked yes i know it's basic but hey it works
fn button(rect: Rect, label: &str) -> bool {
    let (mx, my) = mouse_position();
    let hovered = rect.contains(vec2(mx, my));

    let (bg, font_size) = if hovered {
        (LIGHTGRAY, 31.0)
    } else {
        (GRAY, 30.0)
    };

    draw_rectangle(rect.x, rect.y, rect.w, rect.h, bg);
    draw_text(
        label,
        rect.x + 10.0,
        rect.y + rect.h * 0.6,
        font_size,
        BLACK,
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, BTN_OUTLINE_THICKNESS, BLACK);

    hovered && is_mouse_button_pressed(MouseButton::Left)
}

fn dev_mode_display(dev_mode: bool, mouse: (f32, f32), fps: i32, amount_of_dots: usize) {
    // displays mouse coordinates and a dev mode message when dev mode is active
    // the function is called every frame, but only draws when dev_mode is true
    // which is efficient enough for this simple use case
    // but i should probably add some throttling or optimization if this were to be used in a more complex application oh well

    if dev_mode {
        let entries = [
            format!("x: {} | y: {}", mouse.0, mouse.1),
            "developer mode active".to_string(),
            format!("FPS: {fps}"),
            format!("dots: {amount_of_dots}"),
        ];

        let font_size = 20.0;
        let base_x = screen_width() * 0.85;

        for (idx, line) in entries.iter().enumerate() {
            let dims = text_dimensions(line, font_size);
            // although the fps is drawn every frame, it only updates every 10 frames in the main loop to reduce performance impact and change frequency
            // probably a less complex way to do this but it works for now
            let y_pos = screen_height() * (0.05 * (idx + 1) as f32);
            draw_text(
                line,
                base_x - dims.width / 2.0,
                y_pos - dims.height / 2.0,
                font_size,
                BLACK,
            );
        }
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
    #[arg(long)]
    dots_count: Option<usize>,
}

#[macroquad::main("I click button, I happy")]
async fn main() {
    let cli = Cli::parse();

    // fps tracking variables
    let mut fps = 60;
    let mut i = 0;

    // lots of variables to keep track of the state of the game
    let mut dev_mode = cli.dev_mode;
    let mut state = CurrentState::MainMenu;
    let mut current_color = BgColor::Purple;

    let mut amount_of_dots = cli.dots_count.unwrap_or(DEFAULT_DOTS_COUNT);
    let mut dots: Vec<CuteDot> = Vec::with_capacity(amount_of_dots);
    let initial_bounds = game_bounds();

    for _ in 0..amount_of_dots {
        spawn_dot_in_box(&mut dots, initial_bounds);
    }

    loop {
        let dt = get_frame_time();
        clear_background(Color::from(current_color));

        // for fps display in dev mode
        i += 1;
        if i >= FPS_UPDATE_INTERVAL {
            i = 0;
            fps = get_fps();
        }

        // draw the outline around the screen
        screen_outline();

        // display dev mode info if active
        dev_mode_display(dev_mode, mouse_position(), fps, amount_of_dots);

        // this match statement handles the different states of the game, note how each state has its own UI and functionality
        match state {
            CurrentState::MainMenu => {
                let v_text = cli.version;
                let v_text_dims = text_dimensions(v_text, 20.0);
                let game_text_pos = vec2(10.0, v_text_dims.height * 3.0);

                game_msg(v_text, game_text_pos, 40.0);

                // center position: screen/2 - size/2
                // this is used to center the button on the screen
                let btn_rect = centered_rect(
                    screen_width() / 2.0,
                    (screen_height() - BTN_SIZE.y) / 2.0,
                    BTN_SIZE,
                );

                if button(btn_rect, "START") {
                    println!("game started!");
                    state = CurrentState::Game;
                }
            }
            CurrentState::Game => {
                // actually calling it a game is a bit of a stretch
                let game_text = "game prototype:";
                let game_bounds = game_bounds();

                draw_rectangle(
                    game_bounds.x,
                    game_bounds.y,
                    game_bounds.w,
                    game_bounds.h,
                    LIGHTGRAY,
                );
                let game_text_pos =
                    centered_text_pos(game_text, screen_width() / 2.0, screen_height() / 4.0, 40.0);
                game_msg(game_text, game_text_pos, 40.0);

                // update and draw each cute dot
                for dot in &mut dots {
                    dot.update(dt, game_bounds);
                    dot.draw();
                }

                // draw the game area outline after drawing the dots to ensure it's on top
                draw_rectangle_lines(
                    game_bounds.x,
                    game_bounds.y,
                    game_bounds.w,
                    game_bounds.h,
                    GAME_OUTLINE_THICKNESS,
                    BLACK,
                );

                if button(
                    Rect::new(
                        screen_width() / 4.0,
                        screen_height() - 100.0,
                        BTN_SIZE.x,
                        BTN_SIZE.y,
                    ),
                    "dots +",
                ) {
                    amount_of_dots += 1;
                    spawn_dot_in_box(&mut dots, game_bounds);
                    println!("dots increased to {amount_of_dots}");
                }

                if button(Rect::new(10.0, 10.0, BTN_SIZE.x, BTN_SIZE.y), "Settings") {
                    state = CurrentState::Settings;
                }
            }
            CurrentState::Settings => {
                // i might add more settings later but for now this is fine
                // also the settings text at the bottom is just a placeholder for now
                let settings_text = "change your settings as you desire! (SETTINGS SOON!!)";
                let settings_dims = text_dimensions(settings_text, 30.0);
                let settings_text_pos = vec2(
                    (screen_width() - settings_dims.width) / 2.0,
                    screen_height() - settings_dims.height / 2.0,
                );
                game_msg(settings_text, settings_text_pos, 30.0);

                if button(
                    centered_rect(screen_width() / 2.0, 125.0, COLOR_BTN_SIZE),
                    &format!("Dev: {}", if dev_mode { "activated" } else { "disabled" }),
                ) {
                    dev_mode = !dev_mode;
                }

                if button(
                    centered_rect(screen_width() / 2.0, 50.0, COLOR_BTN_SIZE),
                    "Choose color",
                ) {
                    current_color = current_color.next();
                }

                if button(
                    Rect::new(10.0, 10.0, BTN_SIZE.x, BTN_SIZE.y),
                    "Back to game",
                ) {
                    state = CurrentState::Game;
                }
            }
        }
        next_frame().await;
    }
}
