//! Blind-test presentation. Do not read src/lib.rs or design documents before playing.
use archives_du_silence::{Console, Reaction, World};
use gotoo_pixel_engine::{
    EngineConfig, EngineError, Frame, Framebuffer, Game, GameResult, Key, Pixel, run,
};

const WIDTH: u32 = 320;
const HEIGHT: u32 = 180;
const BG: Pixel = Pixel::rgb(8, 12, 20);
const WALL: Pixel = Pixel::rgb(18, 27, 40);
const STONE: Pixel = Pixel::rgb(37, 52, 65);
const METAL: Pixel = Pixel::rgb(72, 101, 110);
const TEXT: Pixel = Pixel::rgb(213, 228, 221);
const MUTED: Pixel = Pixel::rgb(123, 150, 158);
const LIGHT: Pixel = Pixel::rgb(252, 211, 125);
const ACCENT: Pixel = Pixel::rgb(104, 226, 213);
const DARK: Pixel = Pixel::rgb(5, 18, 26);

struct FirstRoom {
    world: World,
    console: Console,
    symbols: [usize; 2],
    slot: usize,
    message: &'static str,
    elapsed: f32,
}

impl Default for FirstRoom {
    fn default() -> Self {
        Self {
            world: World::default(),
            console: Console::First,
            symbols: [0, 0],
            slot: 0,
            message: "OBSERVEZ LES MACHINES.",
            elapsed: 0.0,
        }
    }
}

impl Game for FirstRoom {
    fn update(&mut self, frame: &mut Frame<'_>) -> GameResult {
        if frame.input.key(Key::Escape).pressed() {
            return GameResult::Exit;
        }
        if frame.input.key(Key::R).pressed() {
            *self = Self::default();
        } else if !self.world.second_active() {
            if frame.input.key(Key::Tab).pressed() {
                if self.world.accessible(Console::Second) {
                    self.console = match self.console {
                        Console::First => Console::Second,
                        Console::Second => Console::First,
                    };
                    self.message = "UN AUTRE PUPITRE.";
                } else {
                    self.message = "LE PASSAGE EST OBSTRUE.";
                }
            }
            if frame.input.key(Key::Left).pressed() {
                self.slot = 0;
            }
            if frame.input.key(Key::Right).pressed() {
                self.slot = 1;
            }
            if frame.input.key(Key::Up).pressed() {
                self.symbols[self.slot] = (self.symbols[self.slot] + 1) % 5;
            }
            if frame.input.key(Key::Down).pressed() {
                self.symbols[self.slot] = (self.symbols[self.slot] + 4) % 5;
            }
            if frame.input.key(Key::Enter).pressed() {
                self.message = match self.world.submit(self.console, self.symbols) {
                    Reaction::Changed => "LE MONDE A REAGI.",
                    Reaction::Unchanged => "AUCUN CHANGEMENT VISIBLE.",
                    Reaction::Invalid => "AUCUNE REACTION.",
                    Reaction::Inaccessible => "INACCESSIBLE.",
                };
            }
        }
        self.elapsed += frame.delta_time.as_secs_f32().min(0.10);
        self.draw(frame.framebuffer);
        GameResult::Continue
    }
}

impl FirstRoom {
    fn draw(&self, fb: &mut Framebuffer) {
        fb.clear(BG);
        fb.draw_text(8, 7, "LES ARCHIVES DU SILENCE", TEXT);
        fb.draw_text(267, 7, "01", MUTED);
        fb.draw_line(8, 20, 311, 20, STONE);

        fb.fill_rect(8, 25, 304, 96, WALL);
        fb.draw_rect(8, 25, 304, 96, METAL);
        for i in 0..12 {
            let x = 14 + i * 25;
            fb.draw_line(x, 25, x + 12, 32, STONE);
            fb.draw_line(x, 113, x + 10, 121, STONE);
        }
        fb.fill_rect(9, 116, 302, 5, STONE);
        fb.draw_line(9, 112, 310, 112, METAL);

        draw_lamp(fb, 46, true, [0, 2]);
        draw_lamp(fb, 94, false, [1, 2]);
        draw_machine(fb, self.world.first_active());
        draw_door(fb, self.world.second_active());

        if self.world.first_active() {
            // Obstruction is drawn in the foreground between the two consoles.
            let drift = (self.elapsed * 4.0).floor() as i32 % 4;
            for i in 0..5 {
                let y = 43 + i * 13 + drift;
                fb.fill_rect(201 + (i % 2) * 7, y, 47, 6, Pixel::rgb(89, 112, 116));
                fb.fill_rect(211 + (i % 3) * 5, y + 6, 39, 4, Pixel::rgb(61, 83, 90));
            }
        } else {
            fb.draw_line(189, 94, 236, 103, ACCENT);
            fb.draw_line(235, 102, 242, 102, ACCENT);
        }

        if self.world.second_active() {
            fb.fill_rect(11, 32, 298, 84, DARK);
            fb.draw_rect(36, 52, 248, 56, ACCENT);
            fb.draw_text_scaled(59, 65, "LE PASSAGE EST OUVERT", 1, TEXT);
            fb.draw_text(66, 87, "VOUS AVEZ DECOUVERT LE SIGNAL.", ACCENT);
        }

        self.draw_controls(fb);
    }

    fn draw_controls(&self, fb: &mut Framebuffer) {
        fb.fill_rect(7, 126, 306, 49, DARK);
        fb.draw_rect(7, 126, 306, 49, METAL);
        if self.world.second_active() {
            fb.draw_text(15, 136, "FIN DU PROTOTYPE 001", LIGHT);
            fb.draw_text(15, 154, "R: RECOMMENCER    ESC: QUITTER", MUTED);
            return;
        }
        let label = match self.console {
            Console::First => "PUPITRE 1",
            Console::Second => "PUPITRE 2",
        };
        fb.draw_text(14, 132, label, TEXT);
        for index in 0..2 {
            let x = 108 + 35 * index as i32;
            fb.fill_rect(x, 130, 27, 22, WALL);
            fb.draw_rect(
                x,
                130,
                27,
                22,
                if self.slot == index { ACCENT } else { METAL },
            );
            draw_glyph(fb, self.symbols[index], x + 7, 134, TEXT);
        }
        fb.draw_text(193, 136, "ENTREE: ESSAI", TEXT);
        fb.draw_text(
            14,
            155,
            self.message,
            if self.message.contains("REAGI") {
                ACCENT
            } else {
                MUTED
            },
        );
        fb.draw_text(14, 166, "FLECHES: CHOIX   TAB: PUPITRE   R: RESET", MUTED);
    }
}

fn glyph_band(fb: &mut Framebuffer, x: i32, y: i32, symbols: [usize; 2]) {
    fb.fill_rect(x, y, 36, 17, DARK);
    fb.draw_rect(x, y, 36, 17, METAL);
    draw_glyph(fb, symbols[0], x + 3, y + 2, ACCENT);
    draw_glyph(fb, symbols[1], x + 20, y + 2, ACCENT);
}

fn draw_glyph(fb: &mut Framebuffer, glyph: usize, x: i32, y: i32, color: Pixel) {
    match glyph {
        0 => {
            fb.draw_line(x + 6, y, x, y + 12, color);
            fb.draw_line(x, y + 12, x + 12, y + 12, color);
            fb.draw_line(x + 12, y + 12, x + 6, y, color);
        }
        1 => fb.draw_circle(x + 6, y + 6, 6, color),
        2 => {
            fb.draw_line(x + 6, y, x + 12, y + 6, color);
            fb.draw_line(x + 12, y + 6, x + 6, y + 12, color);
            fb.draw_line(x + 6, y + 12, x, y + 6, color);
            fb.draw_line(x, y + 6, x + 6, y, color);
        }
        3 => {
            fb.draw_rect(x, y, 12, 12, color);
            fb.fill_rect(x + 5, y + 2, 3, 9, color);
        }
        _ => {
            fb.draw_line(x + 3, y, x + 9, y, color);
            fb.draw_line(x + 9, y, x + 12, y + 6, color);
            fb.draw_line(x + 12, y + 6, x + 9, y + 12, color);
            fb.draw_line(x + 9, y + 12, x + 3, y + 12, color);
            fb.draw_line(x + 3, y + 12, x, y + 6, color);
            fb.draw_line(x, y + 6, x + 3, y, color);
        }
    }
}

fn draw_lamp(fb: &mut Framebuffer, x: i32, lit: bool, symbols: [usize; 2]) {
    fb.draw_line(x, 72, x, 88, METAL);
    fb.fill_rect(x - 8, 71, 16, 6, STONE);
    fb.draw_circle(x, 58, 9, if lit { LIGHT } else { METAL });
    if lit {
        fb.fill_circle(x, 58, 5, LIGHT);
        fb.draw_line(x - 13, 58, x - 18, 58, LIGHT);
        fb.draw_line(x + 13, 58, x + 18, 58, LIGHT);
    } else {
        fb.fill_circle(x, 58, 5, DARK);
    }
    glyph_band(fb, x - 18, 92, symbols);
}

fn draw_machine(fb: &mut Framebuffer, active: bool) {
    fb.fill_rect(145, 65, 51, 35, STONE);
    fb.draw_rect(145, 65, 51, 35, METAL);
    fb.draw_circle(169, 81, 12, METAL);
    fb.fill_circle(169, 81, 7, if active { LIGHT } else { DARK });
    fb.fill_rect(179, 70, 14, 8, if active { ACCENT } else { MUTED });
    fb.fill_rect(152, 102, 39, 10, STONE);
    glyph_band(fb, 150, 44, if active { [0, 3] } else { [1, 3] });
    fb.draw_line(167, 100, 167, 112, METAL);
}

fn draw_door(fb: &mut Framebuffer, open: bool) {
    fb.fill_rect(259, 45, 41, 69, STONE);
    fb.draw_rect(259, 45, 41, 69, METAL);
    if open {
        fb.fill_rect(266, 53, 27, 59, DARK);
        fb.draw_line(266, 53, 294, 53, ACCENT);
    } else {
        fb.fill_rect(267, 52, 25, 60, DARK);
        fb.draw_line(279, 52, 279, 112, METAL);
        fb.fill_rect(269, 82, 21, 3, LIGHT);
    }
    glyph_band(fb, 261, 29, if open { [0, 4] } else { [1, 4] });
}

fn main() -> Result<(), EngineError> {
    run(
        EngineConfig {
            title: "Les Archives du Silence - Prototype 001".into(),
            framebuffer_width: WIDTH,
            framebuffer_height: HEIGHT,
            window_width: WIDTH * 3,
            window_height: HEIGHT * 3,
        },
        FirstRoom::default(),
    )
}
