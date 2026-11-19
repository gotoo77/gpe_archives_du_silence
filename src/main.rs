//! Spoiler-safe native presentation for the first playable room.
mod audio_cues;

use archives_du_silence::{Console, Reaction, World};
use audio_cues::{AudioCues, Cue};
use gotoo_pixel_engine::{
    EngineConfig, Frame, Framebuffer, Game, GameResult, Key, MouseButton, Pixel, run,
};

const WIDTH: u32 = 640;
const HEIGHT: u32 = 360;
const BG: Pixel = Pixel::rgb(7, 13, 22);
const WALL: Pixel = Pixel::rgb(15, 26, 40);
const STONE: Pixel = Pixel::rgb(33, 49, 65);
const METAL: Pixel = Pixel::rgb(76, 111, 126);
const TEXT: Pixel = Pixel::rgb(218, 236, 230);
const MUTED: Pixel = Pixel::rgb(131, 160, 166);
const LIGHT: Pixel = Pixel::rgb(253, 213, 121);
const ACCENT: Pixel = Pixel::rgb(100, 225, 210);
const DARK: Pixel = Pixel::rgb(6, 17, 29);
const STEAM: Pixel = Pixel::rgb(93, 123, 129);
const CONSOLE_ONE: (i32, i32) = (377, 239);
const CONSOLE_TWO: (i32, i32) = (489, 239);
const SLOT_X: [i32; 2] = [240, 290];
const SLOT_Y: i32 = 316;
const SEND_X: i32 = 510;
const SEND_Y: i32 = 319;

struct FirstRoom {
    world: World,
    console: Console,
    symbols: [[usize; 2]; 2],
    slot: usize,
    message: &'static str,
    elapsed: f32,
    help: bool,
    sounds: AudioCues,
}

impl FirstRoom {
    fn new() -> Result<Self, gotoo_pixel_engine::AudioError> {
        Ok(Self {
            world: World::default(),
            console: Console::First,
            symbols: [[0, 0], [0, 0]],
            slot: 0,
            message: "OBSERVEZ LES LIEUX.",
            elapsed: 0.0,
            help: false,
            sounds: AudioCues::new()?,
        })
    }

    fn reset(&mut self) {
        self.world = World::default();
        self.console = Console::First;
        self.symbols = [[0, 0], [0, 0]];
        self.slot = 0;
        self.message = "OBSERVEZ LES LIEUX.";
        self.elapsed = 0.0;
        self.help = false;
    }
}

impl Game for FirstRoom {
    fn update(&mut self, frame: &mut Frame<'_>) -> GameResult {
        self.sounds.ensure_ambience(frame.audio);
        if frame.input.key(Key::Escape).pressed() {
            return GameResult::Exit;
        }
        if frame.input.key(Key::H).pressed() {
            self.help = !self.help;
            self.sounds.play(frame.audio, Cue::Tick);
        }
        if frame.input.key(Key::R).pressed() {
            self.reset();
            self.sounds.play(frame.audio, Cue::Tick);
        } else if !self.world.second_active() {
            if frame.input.key(Key::Tab).pressed() {
                self.select_console(other_console(self.console), frame.audio);
            }
            if frame.input.key(Key::Left).pressed() {
                self.slot = 0;
                self.sounds.play(frame.audio, Cue::Tick);
            }
            if frame.input.key(Key::Right).pressed() {
                self.slot = 1;
                self.sounds.play(frame.audio, Cue::Tick);
            }
            if frame.input.key(Key::Up).pressed() {
                adjust_draft(&mut self.symbols, self.console, self.slot, true);
                self.sounds.play(frame.audio, Cue::Tick);
            }
            if frame.input.key(Key::Down).pressed() {
                adjust_draft(&mut self.symbols, self.console, self.slot, false);
                self.sounds.play(frame.audio, Cue::Tick);
            }
            if frame.input.key(Key::Enter).pressed() {
                self.submit(frame.audio);
            }
            if frame.input.mouse_button(MouseButton::Left).pressed()
                && let Some((x, y)) = frame.input.mouse_position()
            {
                self.handle_click(x, y, frame.audio);
            }
        }
        self.elapsed += frame.delta_time.as_secs_f32().min(0.10);
        self.draw(frame.framebuffer);
        GameResult::Continue
    }
}

fn console_index(console: Console) -> usize {
    match console {
        Console::First => 0,
        Console::Second => 1,
    }
}

fn other_console(console: Console) -> Console {
    match console {
        Console::First => Console::Second,
        Console::Second => Console::First,
    }
}

fn adjust_draft(drafts: &mut [[usize; 2]; 2], console: Console, slot: usize, increase: bool) {
    let value = &mut drafts[console_index(console)][slot];
    *value = (*value + if increase { 1 } else { 4 }) % 5;
}

fn inside(x: i32, y: i32, area: (i32, i32, i32, i32)) -> bool {
    x >= area.0 && y >= area.1 && x < area.0 + area.2 && y < area.1 + area.3
}

impl FirstRoom {
    fn select_console(&mut self, desired: Console, audio: &mut dyn gotoo_pixel_engine::Audio) {
        if !self.world.accessible(desired) {
            self.message = "PUPITRE INACCESSIBLE.";
            self.sounds.play(audio, Cue::Reject);
            return;
        }
        self.console = desired;
        self.slot = 0;
        self.message = "LIAISON AVEC LE PUPITRE.";
        self.sounds.play(audio, Cue::Tick);
    }

    fn submit(&mut self, audio: &mut dyn gotoo_pixel_engine::Audio) {
        match self
            .world
            .submit(self.console, self.symbols[console_index(self.console)])
        {
            Reaction::Changed => {
                self.message = "QUELQUE CHOSE A CHANGE.";
                let cue = if self.world.second_active() {
                    Cue::Discovery
                } else {
                    Cue::Shift
                };
                self.sounds.play(audio, cue);
            }
            Reaction::Unchanged => {
                self.message = "AUCUN CHANGEMENT VISIBLE.";
                self.sounds.play(audio, Cue::Reject);
            }
            Reaction::Invalid => {
                self.message = "AUCUNE REACTION.";
                self.sounds.play(audio, Cue::Reject);
            }
            Reaction::Inaccessible => {
                self.message = "INACCESSIBLE.";
                self.sounds.play(audio, Cue::Reject);
            }
        }
    }

    fn handle_click(&mut self, x: i32, y: i32, audio: &mut dyn gotoo_pixel_engine::Audio) {
        if inside(x, y, (CONSOLE_ONE.0, CONSOLE_ONE.1, 43, 26)) {
            self.select_console(Console::First, audio);
        } else if inside(x, y, (CONSOLE_TWO.0, CONSOLE_TWO.1, 43, 26)) {
            self.select_console(Console::Second, audio);
        } else if let Some(index) = SLOT_X
            .iter()
            .position(|x0| inside(x, y, (*x0, SLOT_Y, 42, 36)))
        {
            if self.slot == index {
                adjust_draft(&mut self.symbols, self.console, index, true);
            } else {
                self.slot = index;
            }
            self.sounds.play(audio, Cue::Tick);
        } else if inside(x, y, (SEND_X, SEND_Y, 95, 32)) {
            self.submit(audio);
        }
    }
}

impl FirstRoom {
    fn draw(&self, fb: &mut Framebuffer) {
        fb.clear(BG);
        fb.draw_text_scaled(22, 12, "LES ARCHIVES DU SILENCE", 2, TEXT);
        fb.draw_text(578, 14, "001", MUTED);
        fb.draw_text(546, 30, "H: AIDE", MUTED);
        fb.draw_line(20, 42, 619, 42, STONE);

        self.draw_room(fb);
        self.draw_compact_hud(fb);
        if self.help {
            self.draw_help(fb);
        }
    }

    fn draw_room(&self, fb: &mut Framebuffer) {
        fb.fill_rect(20, 51, 600, 254, WALL);
        fb.draw_rect(20, 51, 600, 254, METAL);

        for i in 0..17 {
            let x = 32 + i * 34;
            fb.draw_line(x, 51, x + 20, 65, STONE);
        }
        for i in 0..9 {
            let x = 38 + i * 65;
            fb.draw_line(x, 54, x, 266, Pixel::rgb(22, 35, 49));
        }
        fb.fill_rect(21, 267, 598, 20, STONE);
        fb.draw_line(21, 266, 617, 266, METAL);
        for i in 0..18 {
            let x = 32 + i * 33;
            fb.draw_line(x, 269, x + 14, 286, WALL);
        }

        draw_lamp(fb, 124, true, [0, 2]);
        draw_lamp(fb, 240, false, [1, 2]);
        draw_machine(fb, self.world.first_active(), self.elapsed);
        draw_door(fb, self.world.second_active());

        // Wires make the local target of each console unambiguous.
        let first_wire = if self.console == Console::First {
            ACCENT
        } else {
            METAL
        };
        let second_wire = if self.console == Console::Second {
            ACCENT
        } else {
            METAL
        };
        fb.draw_line(355, 235, 355, 250, first_wire);
        fb.draw_line(355, 250, 377, 250, first_wire);
        fb.draw_line(533, 242, 533, 251, second_wire);
        fb.draw_line(533, 251, 530, 251, second_wire);

        // The vapor blocks the physical way to the remote console.
        if self.world.first_active() {
            let drift = (self.elapsed * 14.0).floor() as i32 % 17;
            for i in 0..9 {
                let y = 100 + i * 16 + drift;
                let x = 424 + (i % 3) * 7;
                fb.fill_rect(x, y, 83, 10, STEAM);
                fb.fill_rect(x + 10, y + 10, 70, 5, METAL);
            }
        } else {
            fb.draw_line(410, 227, 484, 244, ACCENT);
            fb.draw_line(485, 244, 513, 244, METAL);
        }

        draw_console_marker(
            fb,
            CONSOLE_ONE,
            1,
            self.console == Console::First,
            true,
            self.elapsed,
        );
        draw_console_marker(
            fb,
            CONSOLE_TWO,
            2,
            self.console == Console::Second,
            self.world.accessible(Console::Second),
            self.elapsed,
        );
        let (x, y, width, height) = match self.console {
            Console::First => (296, 142, 123, 99),
            Console::Second => (520, 114, 92, 151),
        };
        draw_focus_corners(fb, x, y, width, height, ACCENT);

        if self.world.second_active() {
            fb.fill_rect(105, 104, 429, 100, DARK);
            fb.draw_rect(105, 104, 429, 100, ACCENT);
            fb.draw_text_scaled(140, 125, "LE PASSAGE EST OUVERT", 2, LIGHT);
            fb.draw_text_scaled(173, 169, "LE SIGNAL A REPONDU.", 1, ACCENT);
        }
    }

    fn draw_compact_hud(&self, fb: &mut Framebuffer) {
        fb.fill_rect(21, 309, 598, 48, DARK);
        fb.draw_line(21, 309, 618, 309, METAL);
        if self.world.second_active() {
            fb.draw_text_scaled(40, 321, "FIN DU PROTOTYPE", 2, TEXT);
            fb.draw_text(443, 331, "R: REJOUER", MUTED);
            return;
        }
        fb.draw_text(34, 315, "PUPITRE ACTIF", MUTED);
        fb.draw_text_scaled(
            35,
            329,
            if self.console == Console::First {
                "01"
            } else {
                "02"
            },
            2,
            ACCENT,
        );
        fb.draw_text(90, 336, "TAB: CHANGER", MUTED);
        let draft = self.symbols[console_index(self.console)];
        for (index, x) in SLOT_X.iter().enumerate() {
            fb.fill_rect(*x, SLOT_Y, 42, 36, STONE);
            fb.draw_rect(
                *x,
                SLOT_Y,
                42,
                36,
                if self.slot == index { ACCENT } else { METAL },
            );
            draw_glyph(fb, draft[index], *x + 9, SLOT_Y + 5, 2, TEXT);
            fb.draw_text(*x + 1, 310, if index == 0 { "1" } else { "2" }, MUTED);
        }
        fb.draw_text(344, 317, "2 SIGNES", MUTED);
        fb.draw_text(344, 335, self.message, TEXT);
        fb.fill_rect(SEND_X, SEND_Y, 95, 32, STONE);
        fb.draw_rect(SEND_X, SEND_Y, 95, 32, ACCENT);
        fb.draw_text(528, 332, "ENVOYER", TEXT);
        if self.sounds.unavailable() {
            fb.draw_text(505, 351, "AUDIO INDISPO", LIGHT);
        }
    }

    fn draw_help(&self, fb: &mut Framebuffer) {
        fb.fill_rect(129, 66, 391, 151, DARK);
        fb.draw_rect(129, 66, 391, 151, ACCENT);
        fb.draw_text_scaled(153, 86, "COMMANDES", 2, LIGHT);
        fb.draw_text_scaled(153, 117, "GAUCHE / DROITE : CASE", 1, TEXT);
        fb.draw_text_scaled(153, 134, "HAUT / BAS : SYMBOLE", 1, TEXT);
        fb.draw_text_scaled(153, 151, "ENTREE : ESSAYER", 1, TEXT);
        fb.draw_text_scaled(153, 168, "TAB : AUTRE PUPITRE", 1, TEXT);
        fb.draw_text_scaled(153, 185, "CLIC : PUPITRE / GLYPHE / ENVOYER", 1, MUTED);
        fb.draw_text(153, 202, "R: RESET   H: FERMER   ESC: QUITTER", MUTED);
    }
}

fn draw_console_marker(
    fb: &mut Framebuffer,
    (x, y): (i32, i32),
    number: u8,
    selected: bool,
    accessible: bool,
    elapsed: f32,
) {
    let outline = if selected {
        ACCENT
    } else if accessible {
        METAL
    } else {
        MUTED
    };
    fb.fill_rect(x, y, 43, 26, DARK);
    fb.draw_rect(x, y, 43, 26, outline);
    fb.draw_text_scaled(
        x + 8,
        y + 5,
        if number == 1 { "1" } else { "2" },
        2,
        outline,
    );
    fb.draw_line(x + 26, y + 9, x + 35, y + 9, outline);
    fb.draw_line(x + 26, y + 15, x + 35, y + 15, outline);
    if selected {
        let pulse = if (elapsed * 3.0) as i32 % 2 == 0 {
            ACCENT
        } else {
            TEXT
        };
        fb.draw_rect(x - 3, y - 3, 49, 32, pulse);
    }
}

fn draw_focus_corners(fb: &mut Framebuffer, x: i32, y: i32, width: u32, height: u32, color: Pixel) {
    let width = width as i32;
    let height = height as i32;
    for &(cx, cy, sx, sy) in &[
        (x, y, 1, 1),
        (x + width, y, -1, 1),
        (x, y + height, 1, -1),
        (x + width, y + height, -1, -1),
    ] {
        fb.draw_line(cx, cy, cx + sx * 11, cy, color);
        fb.draw_line(cx, cy, cx, cy + sy * 11, color);
    }
}

fn glyph_band(fb: &mut Framebuffer, x: i32, y: i32, symbols: [usize; 2]) {
    fb.fill_rect(x, y, 73, 29, DARK);
    fb.draw_rect(x, y, 73, 29, METAL);
    draw_glyph(fb, symbols[0], x + 7, y + 5, 2, ACCENT);
    draw_glyph(fb, symbols[1], x + 39, y + 5, 2, ACCENT);
}

fn draw_glyph(fb: &mut Framebuffer, glyph: usize, x: i32, y: i32, scale: i32, color: Pixel) {
    let p = |dx: i32, dy: i32| (x + dx * scale, y + dy * scale);
    let mut line = |a: (i32, i32), b: (i32, i32)| {
        let (x0, y0) = p(a.0, a.1);
        let (x1, y1) = p(b.0, b.1);
        fb.draw_line(x0, y0, x1, y1, color);
    };
    match glyph {
        0 => {
            line((6, 0), (0, 12));
            line((0, 12), (12, 12));
            line((12, 12), (6, 0));
        }
        1 => {
            // Diamond-like octagon for clarity at variable logical resolutions.
            for (a, b) in [
                ((4, 0), (9, 0)),
                ((9, 0), (12, 4)),
                ((12, 4), (12, 9)),
                ((12, 9), (9, 12)),
                ((9, 12), (4, 12)),
                ((4, 12), (0, 9)),
                ((0, 9), (0, 4)),
                ((0, 4), (4, 0)),
            ] {
                line(a, b);
            }
        }
        2 => {
            line((6, 0), (12, 6));
            line((12, 6), (6, 12));
            line((6, 12), (0, 6));
            line((0, 6), (6, 0));
        }
        3 => {
            line((0, 0), (12, 0));
            line((12, 0), (12, 12));
            line((12, 12), (0, 12));
            line((0, 12), (0, 0));
            line((4, 2), (4, 10));
        }
        _ => {
            line((3, 0), (9, 0));
            line((9, 0), (12, 6));
            line((12, 6), (9, 12));
            line((9, 12), (3, 12));
            line((3, 12), (0, 6));
            line((0, 6), (3, 0));
        }
    }
}

fn draw_lamp(fb: &mut Framebuffer, x: i32, lit: bool, symbols: [usize; 2]) {
    fb.fill_rect(x - 11, 186, 22, 12, STONE);
    fb.draw_line(x, 168, x, 219, METAL);
    fb.draw_circle(x, 141, 22, if lit { LIGHT } else { METAL });
    fb.fill_circle(x, 141, 13, if lit { LIGHT } else { DARK });
    if lit {
        for (dx, dy) in [(-32, 0), (32, 0), (0, -32)] {
            fb.draw_line(x + dx / 2, 141 + dy / 2, x + dx, 141 + dy, LIGHT);
        }
    }
    glyph_band(fb, x - 37, 223, symbols);
}

fn draw_machine(fb: &mut Framebuffer, active: bool, elapsed: f32) {
    glyph_band(fb, 317, 77, if active { [0, 3] } else { [1, 3] });
    fb.fill_rect(299, 145, 117, 92, STONE);
    fb.draw_rect(299, 145, 117, 92, METAL);
    fb.draw_circle(355, 188, 30, METAL);
    fb.fill_circle(355, 188, 18, if active { LIGHT } else { DARK });
    if active {
        let pulse = if (elapsed * 3.0) as i32 % 2 == 0 {
            ACCENT
        } else {
            LIGHT
        };
        fb.fill_rect(392, 158, 16, 18, pulse);
    } else {
        fb.draw_rect(392, 158, 16, 18, MUTED);
    }
    fb.fill_rect(320, 240, 83, 25, STONE);
    fb.draw_line(350, 237, 350, 265, METAL);
}

fn draw_door(fb: &mut Framebuffer, open: bool) {
    glyph_band(fb, 528, 75, if open { [0, 4] } else { [1, 4] });
    fb.fill_rect(524, 118, 84, 147, STONE);
    fb.draw_rect(524, 118, 84, 147, METAL);
    fb.fill_rect(534, 129, 64, 134, DARK);
    if open {
        fb.draw_line(536, 130, 593, 130, ACCENT);
        fb.draw_line(536, 131, 536, 260, ACCENT);
    } else {
        fb.draw_line(565, 130, 565, 260, METAL);
        fb.fill_rect(548, 207, 34, 6, LIGHT);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run(
        EngineConfig {
            title: "Les Archives du Silence - Prototype 001".into(),
            framebuffer_width: WIDTH,
            framebuffer_height: HEIGHT,
            window_width: 1280,
            window_height: 720,
        },
        FirstRoom::new()?,
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn console_selection_has_stable_distinct_indices() {
        assert_eq!(console_index(Console::First), 0);
        assert_eq!(console_index(Console::Second), 1);
        assert_eq!(other_console(other_console(Console::First)), Console::First);
    }

    #[test]
    fn glyph_drafts_are_independent_between_consoles() {
        let mut drafts = [[0, 0], [0, 0]];
        adjust_draft(&mut drafts, Console::First, 0, true);
        adjust_draft(&mut drafts, Console::Second, 1, false);
        assert_eq!(drafts, [[1, 0], [0, 4]]);
        adjust_draft(&mut drafts, Console::First, 0, false);
        assert_eq!(drafts[0], [0, 0]);
        assert_eq!(drafts[1], [0, 4]);
    }

    #[test]
    fn focus_regions_correspond_to_distinct_visible_controls() {
        assert!(inside(390, 250, (CONSOLE_ONE.0, CONSOLE_ONE.1, 43, 26)));
        assert!(!inside(390, 250, (CONSOLE_TWO.0, CONSOLE_TWO.1, 43, 26)));
        assert!(inside(504, 250, (CONSOLE_TWO.0, CONSOLE_TWO.1, 43, 26)));
        assert!(inside(527, 333, (SEND_X, SEND_Y, 95, 32)));
        assert!(!inside(470, 333, (SEND_X, SEND_Y, 95, 32)));
    }

    #[test]
    fn switching_console_is_rejected_until_accessible() {
        use gotoo_pixel_engine::NoopAudio;

        let mut room = FirstRoom::new().expect("procedural audio should generate");
        let mut audio = NoopAudio::default();
        room.select_console(Console::Second, &mut audio);
        assert_eq!(room.console, Console::First);

        room.world.submit(Console::First, [1, 3]);
        room.select_console(Console::Second, &mut audio);
        assert_eq!(room.console, Console::Second);
    }

    #[test]
    fn world_rejects_second_console_before_it_is_reachable() {
        let world = World::default();
        assert!(!world.accessible(Console::Second));
        assert!(world.accessible(Console::First));
    }
}
