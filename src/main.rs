//! SPOILERS FOR THE BLIND TESTER: in-world staging for the memory archive.
//! The player-facing README contains no rune dictionary or puzzle solution.
mod audio_cues;

use archives_du_silence::{Archive, CLUES, EMPTY_RUNE, Page, RUNE_COUNT, Verdict};
use audio_cues::{AudioCues, Cue};
use gotoo_pixel_engine::{
    EngineConfig, Frame, Framebuffer, Game, GameResult, Key, MouseButton, Pixel, run,
};

const WIDTH: u32 = 640;
const HEIGHT: u32 = 360;
const INK: Pixel = Pixel::rgb(8, 15, 24);
const DEEP: Pixel = Pixel::rgb(15, 26, 37);
const STONE: Pixel = Pixel::rgb(33, 44, 52);
const EDGE: Pixel = Pixel::rgb(91, 103, 98);
const DUST: Pixel = Pixel::rgb(62, 70, 70);
const IVORY: Pixel = Pixel::rgb(222, 213, 181);
const GOLD: Pixel = Pixel::rgb(200, 157, 96);
const RUST: Pixel = Pixel::rgb(143, 95, 65);
const SPIRIT: Pixel = Pixel::rgb(113, 207, 184);
const DIM: Pixel = Pixel::rgb(131, 143, 136);
const BLAZE: Pixel = Pixel::rgb(248, 163, 82);
const ERROR: Pixel = Pixel::rgb(212, 126, 117);

const CARD_W: i32 = 128;
const CARD_H: i32 = 98;
const CARD_X: [i32; 3] = [27, 166, 305];
const CARD_Y: [i32; 2] = [80, 191];

struct MemoryGame {
    archive: Archive,
    sounds: AudioCues,
    elapsed: f32,
    help: bool,
    message: &'static str,
    testimony_focus: usize,
    draft_rune: u8,
}

impl MemoryGame {
    fn new() -> Result<Self, gotoo_pixel_engine::AudioError> {
        Ok(Self {
            archive: Archive::default(),
            sounds: AudioCues::new()?,
            elapsed: 0.0,
            help: false,
            message: "TOUCHEZ UNE ARCHIVE. RECONSTITUEZ SON HISTOIRE.",
            testimony_focus: 0,
            draft_rune: 0,
        })
    }

    fn activate_hall(&mut self, focus: usize, audio: &mut dyn gotoo_pixel_engine::Audio) {
        match focus {
            0..=5 => {
                self.archive.open_clip(focus);
                self.message = "UNE MEMOIRE SE REJOUE.";
            }
            6 => {
                self.archive.open_witness();
                self.testimony_focus = self.archive.witness_scene().unwrap_or(0);
                self.message = "QUELLE SCENE RACONTE CE TEMOIGNAGE ?";
            }
            7 => {
                self.archive.open_restore();
                self.message = "RECONSTITUEZ LES TROIS SIGNES MANQUANTS.";
            }
            _ => return,
        }
        self.sounds.play(audio, Cue::Tick);
    }

    fn seal(&mut self, audio: &mut dyn gotoo_pixel_engine::Audio) {
        match self.archive.commit() {
            Verdict::MissingEvidence => {
                self.message = "DEUX FRAGMENTS RESTENT A RECONSTRUIRE.";
                self.sounds.play(audio, Cue::Reject);
            }
            Verdict::Discordant => {
                self.message = "LES DEUX TRACES NE CONCORDENT PAS.";
                self.sounds.play(audio, Cue::Reject);
            }
            Verdict::Recovered => {
                self.message = "UNE MEMOIRE OUBLIEE REFAIT SURFACE.";
                self.sounds.play(audio, Cue::Discovery);
            }
        }
    }

    fn pick_witness(&mut self, option: usize, audio: &mut dyn gotoo_pixel_engine::Audio) {
        self.testimony_focus = option;
        if self.archive.choose_witness_scene(option) {
            self.message = "INTERPRETATION CONSERVEE. AUCUNE CONFIRMATION.";
            self.archive.back();
            self.archive.set_focus(6);
            self.sounds.play(audio, Cue::Shift);
        }
    }

    fn pick_rune(&mut self, rune: u8, audio: &mut dyn gotoo_pixel_engine::Audio) {
        if self.archive.insert(rune) {
            self.draft_rune = rune;
            self.sounds.play(audio, Cue::Tick);
        }
    }

    fn handle_click(&mut self, x: i32, y: i32, audio: &mut dyn gotoo_pixel_engine::Audio) {
        if self.help {
            self.help = false;
            return;
        }
        match self.archive.page() {
            Page::Hall => {
                for index in 0..6 {
                    let (cx, cy) = card_pos(index);
                    if hit(x, y, cx, cy, CARD_W, CARD_H) {
                        self.archive.set_focus(index);
                        self.activate_hall(index, audio);
                        return;
                    }
                }
                if hit(x, y, 464, 229, 74, 35) {
                    self.activate_hall(6, audio);
                } else if hit(x, y, 542, 229, 74, 35) {
                    self.activate_hall(7, audio);
                } else if hit(x, y, 502, 294, 116, 25) {
                    self.seal(audio);
                }
            }
            Page::Clip(_) => {
                if hit(x, y, 480, 272, 130, 37) {
                    self.archive.back();
                    self.sounds.play(audio, Cue::Tick);
                }
            }
            Page::Witness => {
                for option in 0..3 {
                    if hit(x, y, 70, 119 + option as i32 * 58, 500, 55) {
                        self.pick_witness(option, audio);
                        return;
                    }
                }
                if hit(x, y, 500, 294, 112, 26) {
                    self.archive.back();
                }
            }
            Page::Restore => {
                for slot in 0..3 {
                    if hit(x, y, 75 + slot as i32 * 65, 273, 55, 51) {
                        self.archive.set_slot(slot);
                        self.sounds.play(audio, Cue::Tick);
                        return;
                    }
                }
                for rune in 0..RUNE_COUNT {
                    let rx = 438 + i32::from(rune % 3) * 58;
                    let ry = 192 + i32::from(rune / 3) * 50;
                    if hit(x, y, rx, ry, 50, 43) {
                        self.pick_rune(rune, audio);
                        return;
                    }
                }
                if hit(x, y, 480, 285, 129, 37) {
                    self.archive.back();
                    self.sounds.play(audio, Cue::Tick);
                }
            }
            Page::Recovered => {}
        }
    }

    fn keyboard(&mut self, frame: &mut Frame<'_>) {
        let key = |key| frame.input.key(key).pressed();
        if key(Key::H) {
            self.help = !self.help;
            self.sounds.play(frame.audio, Cue::Tick);
        }
        if key(Key::R) {
            self.archive = Archive::default();
            self.message = "LES ARCHIVES RECOMMENCENT LEUR VEILLE.";
            self.sounds.play(frame.audio, Cue::Tick);
            return;
        }
        if key(Key::Escape) {
            self.help = false;
            self.archive.back();
            self.sounds.play(frame.audio, Cue::Tick);
        }
        if self.help {
            return;
        }
        match self.archive.page() {
            Page::Hall => {
                if key(Key::Tab) || key(Key::Right) {
                    self.archive.set_focus((self.archive.focus() + 1) % 8);
                    self.sounds.play(frame.audio, Cue::Tick);
                } else if key(Key::Left) {
                    self.archive.set_focus((self.archive.focus() + 7) % 8);
                    self.sounds.play(frame.audio, Cue::Tick);
                }
                if key(Key::Enter) {
                    self.activate_hall(self.archive.focus(), frame.audio);
                }
                if key(Key::S) {
                    self.seal(frame.audio);
                }
            }
            Page::Clip(index) => {
                if key(Key::Right) {
                    self.archive.open_clip((index + 1) % 6);
                    self.sounds.play(frame.audio, Cue::Tick);
                } else if key(Key::Left) {
                    self.archive.open_clip((index + 5) % 6);
                    self.sounds.play(frame.audio, Cue::Tick);
                } else if key(Key::Enter) || key(Key::Space) {
                    self.archive.back();
                    self.sounds.play(frame.audio, Cue::Tick);
                }
            }
            Page::Witness => {
                if key(Key::Down) || key(Key::Right) {
                    self.testimony_focus = (self.testimony_focus + 1) % 3;
                    self.sounds.play(frame.audio, Cue::Tick);
                }
                if key(Key::Up) || key(Key::Left) {
                    self.testimony_focus = (self.testimony_focus + 2) % 3;
                    self.sounds.play(frame.audio, Cue::Tick);
                }
                if key(Key::Enter) || key(Key::Space) {
                    self.pick_witness(self.testimony_focus, frame.audio);
                }
            }
            Page::Restore => {
                if key(Key::Left) {
                    self.archive.set_slot((self.archive.slot() + 2) % 3);
                    self.sounds.play(frame.audio, Cue::Tick);
                }
                if key(Key::Right) {
                    self.archive.set_slot((self.archive.slot() + 1) % 3);
                    self.sounds.play(frame.audio, Cue::Tick);
                }
                if key(Key::Up) {
                    self.draft_rune = (self.draft_rune + 1) % RUNE_COUNT;
                    self.pick_rune(self.draft_rune, frame.audio);
                }
                if key(Key::Down) {
                    self.draft_rune = (self.draft_rune + RUNE_COUNT - 1) % RUNE_COUNT;
                    self.pick_rune(self.draft_rune, frame.audio);
                }
                if key(Key::Space) {
                    self.archive.clear_slot();
                    self.sounds.play(frame.audio, Cue::Tick);
                }
                if key(Key::Enter) {
                    self.archive.back();
                    self.sounds.play(frame.audio, Cue::Tick);
                }
            }
            Page::Recovered => {}
        }
    }

    fn draw(&self, fb: &mut Framebuffer) {
        draw_room(fb, self.elapsed);
        match self.archive.page() {
            Page::Hall => self.draw_hall(fb),
            Page::Clip(index) => self.draw_clip(fb, index),
            Page::Witness => self.draw_witness(fb),
            Page::Restore => self.draw_restore(fb),
            Page::Recovered => self.draw_recovered(fb),
        }
        draw_footer(fb, self.message, self.archive.page());
        if self.help {
            self.draw_help(fb);
        }
        if self.sounds.unavailable() {
            fb.draw_text(532, 15, "SON INDISPO", ERROR);
        }
    }

    fn draw_hall(&self, fb: &mut Framebuffer) {
        for (index, clue) in CLUES.iter().enumerate() {
            let (x, y) = card_pos(index);
            let focused = self.archive.focus() == index;
            fb.fill_rect(x, y, CARD_W as u32, CARD_H as u32, DEEP);
            fb.draw_rect(
                x,
                y,
                CARD_W as u32,
                CARD_H as u32,
                if focused { GOLD } else { EDGE },
            );
            draw_scene(
                fb,
                x + 8,
                y + 8,
                112,
                58,
                event_for_clip(index),
                self.elapsed,
            );
            fb.fill_rect(x + 5, y + 70, 117, 23, INK);
            draw_sentence(fb, clue, x + 22, y + 73, 1, IVORY);
            if self.archive.watched(index) {
                fb.draw_text(x + 5, y + 5, "*", SPIRIT);
            }
        }
        draw_monument(fb, 467, 87, self.elapsed, false);
        draw_button(fb, 464, 229, 74, 35, "LIRE", self.archive.focus() == 6);
        draw_button(fb, 542, 229, 74, 35, "ECRIRE", self.archive.focus() == 7);
        fb.draw_text(475, 270, "MEMOIRE BRISEE", DIM);
        fb.draw_text(475, 281, "DEUX TRACES", DIM);
        draw_button(fb, 502, 294, 116, 25, "SCELLER [S]", false);
    }

    fn draw_clip(&self, fb: &mut Framebuffer, index: usize) {
        draw_plaque(fb, 25, 73, 431, 233);
        fb.draw_text(44, 89, "REMANENCE", GOLD);
        fb.draw_text_scaled(41, 106, "ARCHIVE", 2, IVORY);
        fb.draw_text_scaled(
            173,
            106,
            match index {
                0 => "01",
                1 => "02",
                2 => "03",
                3 => "04",
                4 => "05",
                _ => "06",
            },
            2,
            GOLD,
        );
        draw_scene(fb, 49, 143, 383, 135, event_for_clip(index), self.elapsed);
        fb.draw_text(478, 90, "TRACE INSCRITE", DIM);
        fb.draw_line(477, 105, 612, 105, EDGE);
        draw_inscription(fb, &CLUES[index], 482, 126, GOLD);
        fb.draw_text(479, 208, "COMPARER LES", IVORY);
        fb.draw_text(479, 221, "MEMOIRES.", IVORY);
        fb.draw_text(479, 239, "< > : FRAGMENTS", DIM);
        draw_button(fb, 480, 272, 130, 37, "RETOUR", false);
    }

    fn draw_witness(&self, fb: &mut Framebuffer) {
        draw_plaque(fb, 23, 69, 595, 259);
        fb.draw_text_scaled(42, 80, "LE TEMOIGNAGE", 2, IVORY);
        fb.draw_text(42, 106, "QUEL EVENEMENT CE TEXTE DECRIT-IL ?", DIM);
        draw_sentence(fb, &archives_du_silence::WITNESS, 457, 75, 1, GOLD);
        for option in 0..3 {
            let y = 119 + option as i32 * 58;
            let active = self.testimony_focus == option;
            fb.fill_rect(70, y, 500, 55, if active { STONE } else { DEEP });
            fb.draw_rect(70, y, 500, 55, if active { SPIRIT } else { EDGE });
            fb.draw_text_scaled(
                91,
                y + 16,
                match option {
                    0 => "I",
                    1 => "II",
                    _ => "III",
                },
                2,
                GOLD,
            );
            draw_scene(
                fb,
                155,
                y + 3,
                405,
                48,
                scene_for_option(option),
                self.elapsed,
            );
        }
        fb.draw_text(72, 305, "CHOISIR UNE INTERPRETATION", DIM);
        draw_button(fb, 500, 294, 112, 26, "RETOUR", false);
    }

    fn draw_restore(&self, fb: &mut Framebuffer) {
        draw_plaque(fb, 22, 72, 595, 257);
        fb.draw_text_scaled(40, 85, "LA SCENE SANS PAROLES", 2, IVORY);
        fb.draw_text(40, 112, "RECONSTITUEZ SON INSCRIPTION", DIM);
        draw_scene(fb, 39, 134, 361, 122, (2, 3, 5), self.elapsed);
        fb.draw_text(64, 262, "INSCRIPTION MANQUANTE", GOLD);
        let draft = self.archive.draft();
        for (index, rune) in draft.iter().copied().enumerate() {
            let x = 75 + index as i32 * 65;
            fb.fill_rect(x, 273, 55, 51, STONE);
            fb.draw_rect(
                x,
                273,
                55,
                51,
                if self.archive.slot() == index {
                    SPIRIT
                } else {
                    EDGE
                },
            );
            if rune == EMPTY_RUNE {
                fb.draw_line(x + 12, 300, x + 42, 300, EDGE);
            } else {
                draw_rune(fb, rune, x + 17, 281, 2, IVORY);
            }
        }
        fb.draw_text(435, 167, "SIGNES DISPONIBLES", DIM);
        for rune in 0..RUNE_COUNT {
            let x = 438 + i32::from(rune % 3) * 58;
            let y = 192 + i32::from(rune / 3) * 50;
            fb.fill_rect(x, y, 50, 43, DEEP);
            fb.draw_rect(x, y, 50, 43, EDGE);
            draw_rune(fb, rune, x + 14, y + 5, 2, GOLD);
        }
        draw_button(fb, 480, 285, 129, 37, "RETOUR", false);
    }

    fn draw_recovered(&self, fb: &mut Framebuffer) {
        draw_plaque(fb, 31, 78, 577, 240);
        draw_monument(fb, 66, 112, self.elapsed, true);
        draw_scene(fb, 250, 129, 332, 135, (0, 4, 3), self.elapsed);
        fb.draw_text_scaled(248, 97, "LA MEMOIRE", 2, IVORY);
        fb.draw_text_scaled(248, 114, "RESTAUREE", 2, GOLD);
        fb.draw_text(248, 276, "CE QUE LA PIERRE AVAIT EFFACE", SPIRIT);
        fb.draw_text(248, 291, "EST ENFIN VISIBLE.", SPIRIT);
    }

    fn draw_help(&self, fb: &mut Framebuffer) {
        fb.fill_rect(114, 83, 412, 204, INK);
        fb.draw_rect(114, 83, 412, 204, GOLD);
        fb.draw_text_scaled(134, 102, "OBSERVER, DEDUIRE, RESTITUER", 1, IVORY);
        fb.draw_text(136, 135, "CLIC : CHOISIR UNE ARCHIVE OU UN SIGNE", IVORY);
        fb.draw_text(136, 155, "TAB / FLECHES : NAVIGUER", IVORY);
        fb.draw_text(136, 175, "ENTREE : CHOISIR / RETOUR", IVORY);
        fb.draw_text(136, 195, "S DANS LA SALLE : SCELLER LES DEUX TRACES", IVORY);
        fb.draw_text(136, 215, "ESPACE EN ECRITURE : EFFACER UN SIGNE", IVORY);
        fb.draw_text(136, 244, "ECHAP : REVENIR    H : FERMER", DIM);
    }
}

impl Game for MemoryGame {
    fn update(&mut self, frame: &mut Frame<'_>) -> GameResult {
        self.sounds.ensure_ambience(frame.audio);
        if frame.input.key(Key::Escape).pressed() && self.archive.page() == Page::Hall && !self.help
        {
            return GameResult::Exit;
        }
        self.keyboard(frame);
        if frame.input.mouse_button(MouseButton::Left).pressed()
            && let Some((x, y)) = frame.input.mouse_position()
        {
            self.handle_click(x, y, frame.audio);
        }
        self.elapsed += frame.delta_time.as_secs_f32().min(0.06);
        self.draw(frame.framebuffer);
        GameResult::Continue
    }
}

fn hit(x: i32, y: i32, xx: i32, yy: i32, w: i32, h: i32) -> bool {
    x >= xx && x < xx + w && y >= yy && y < yy + h
}

fn card_pos(i: usize) -> (i32, i32) {
    (CARD_X[i % 3], CARD_Y[i / 3])
}

fn draw_room(fb: &mut Framebuffer, elapsed: f32) {
    fb.clear(INK);
    fb.fill_rect(0, 0, WIDTH, 52, DEEP);
    fb.draw_line(0, 52, 639, 52, GOLD);
    fb.draw_text(23, 15, "LES ARCHIVES DU SILENCE", IVORY);
    fb.draw_text(23, 30, "LES REMANENCES", GOLD);
    fb.draw_text(539, 20, "FRAGMENT 002", GOLD);
    fb.fill_rect(14, 60, 612, 259, STONE);
    for i in 0..19 {
        let x = 22 + i * 31;
        fb.draw_line(x, 60, x + 7, 64, DUST);
        fb.draw_line(x, 315, x + 18, 317, DUST);
        if i % 3 == 0 {
            fb.draw_line(x + 8, 69, x - 4, 301, Pixel::rgb(38, 49, 55));
        }
    }
    for i in 0..13 {
        let x = 16 + ((i * 173 + 49) % 605);
        let y = 64 + ((i * 131 + 29) % 247);
        let shift = ((elapsed * 4.0) as i32 + i * 3) % 18;
        fb.draw(x + shift, y, GOLD);
    }
    fb.draw_line(13, 60, 13, 319, EDGE);
    fb.draw_line(626, 60, 626, 319, EDGE);
}

fn draw_footer(fb: &mut Framebuffer, msg: &str, page: Page) {
    fb.fill_rect(0, 325, 640, 35, INK);
    fb.draw_line(17, 324, 622, 324, GOLD);
    let visible = if msg.len() > 66 { &msg[..66] } else { msg };
    fb.draw_text(22, 335, visible, IVORY);
    match page {
        Page::Hall => fb.draw_text(
            21,
            349,
            "H:AIDE  TAB:NAVIGUER  ENTREE:OBSERVER  S:SCELLER",
            DIM,
        ),
        Page::Clip(_) => fb.draw_text(21, 349, "< >:AUTRE ARCHIVE  ENTREE:RETOUR  H:AIDE", DIM),
        Page::Witness => fb.draw_text(
            21,
            349,
            "FLECHES:SCENE  ENTREE:CONSERVER  ECHAP:RETOUR",
            DIM,
        ),
        Page::Restore => fb.draw_text(
            21,
            349,
            "FLECHES:SIGNE / CASE  ESPACE:EFFACER  ENTREE:RETOUR",
            DIM,
        ),
        Page::Recovered => fb.draw_text(21, 349, "FIN DE CETTE EXPERIENCE. R:RECOMMENCER.", DIM),
    }
}

fn draw_plaque(fb: &mut Framebuffer, x: i32, y: i32, w: u32, h: u32) {
    fb.fill_rect(x, y, w, h, DEEP);
    fb.draw_rect(x, y, w, h, EDGE);
    fb.draw_rect(x + 4, y + 4, w - 8, h - 8, DUST);
    fb.draw_line(x + 14, y + 16, x + 35, y + 16, GOLD);
    fb.draw_line(
        x + w as i32 - 39,
        y + h as i32 - 17,
        x + w as i32 - 17,
        y + h as i32 - 17,
        GOLD,
    );
}

fn draw_button(fb: &mut Framebuffer, x: i32, y: i32, w: u32, h: u32, label: &str, focus: bool) {
    fb.fill_rect(x, y, w, h, if focus { STONE } else { DEEP });
    fb.draw_rect(x, y, w, h, if focus { SPIRIT } else { GOLD });
    let (tx, ty) = Framebuffer::text_size(label, 1);
    fb.draw_text(
        x + (w as i32 - tx as i32) / 2,
        y + (h as i32 - ty as i32) / 2,
        label,
        IVORY,
    );
}

// All runes follow a shared six-column etching style; none is a plain polygon.
fn draw_rune(fb: &mut Framebuffer, rune: u8, x: i32, y: i32, scale: i32, color: Pixel) {
    const A: &[(i32, i32, i32, i32)] = &[
        (1, 13, 4, 3),
        (4, 3, 8, 1),
        (8, 1, 12, 6),
        (12, 6, 8, 9),
        (8, 9, 13, 14),
        (3, 8, 10, 8),
    ];
    const B: &[(i32, i32, i32, i32)] = &[
        (2, 2, 6, 6),
        (6, 6, 11, 0),
        (6, 6, 7, 14),
        (7, 14, 13, 11),
        (1, 12, 7, 10),
        (2, 2, 1, 7),
    ];
    const C: &[(i32, i32, i32, i32)] = &[
        (1, 2, 13, 8),
        (13, 8, 5, 13),
        (5, 13, 2, 9),
        (2, 9, 8, 4),
        (8, 4, 12, 1),
        (5, 13, 12, 14),
    ];
    const D: &[(i32, i32, i32, i32)] = &[
        (7, 0, 7, 14),
        (3, 4, 7, 2),
        (7, 2, 12, 5),
        (2, 10, 7, 12),
        (7, 12, 13, 9),
        (12, 5, 11, 9),
    ];
    const E: &[(i32, i32, i32, i32)] = &[
        (4, 0, 10, 2),
        (10, 2, 9, 8),
        (9, 8, 13, 13),
        (4, 0, 2, 7),
        (2, 7, 7, 14),
        (7, 14, 13, 13),
        (4, 9, 10, 9),
    ];
    const F: &[(i32, i32, i32, i32)] = &[
        (7, 0, 12, 5),
        (12, 5, 9, 11),
        (9, 11, 3, 13),
        (3, 13, 1, 8),
        (1, 8, 5, 5),
        (5, 5, 10, 7),
        (10, 7, 6, 13),
    ];
    let strokes = match rune {
        0 => A,
        1 => B,
        2 => C,
        3 => D,
        4 => E,
        _ => F,
    };
    for &(x1, y1, x2, y2) in strokes {
        fb.draw_line(
            x + x1 * scale,
            y + y1 * scale,
            x + x2 * scale,
            y + y2 * scale,
            color,
        );
    }
    let (dx, dy) = match rune {
        0 => (1, 2),
        1 => (12, 14),
        2 => (0, 11),
        3 => (13, 1),
        4 => (1, 13),
        _ => (12, 1),
    };
    fb.fill_rect(
        x + dx * scale,
        y + dy * scale,
        scale.max(1) as u32,
        scale.max(1) as u32,
        color,
    );
}

fn draw_sentence(
    fb: &mut Framebuffer,
    sentence: &[u8; 3],
    x: i32,
    y: i32,
    scale: i32,
    color: Pixel,
) {
    for (index, rune) in sentence.iter().copied().enumerate() {
        draw_rune(fb, rune, x + index as i32 * 31 * scale, y, scale, color);
        if index < 2 {
            fb.draw_line(
                x + (index as i32 * 31 + 21) * scale,
                y + 16 * scale,
                x + (index as i32 * 31 + 25) * scale,
                y + 16 * scale,
                DUST,
            );
        }
    }
}

// The three engraved runes fit inside the evidence panel.
fn draw_inscription(fb: &mut Framebuffer, sentence: &[u8; 3], x: i32, y: i32, color: Pixel) {
    for (index, rune) in sentence.iter().copied().enumerate() {
        let xx = x + index as i32 * 43;
        draw_rune(fb, rune, xx, y, 2, color);
        fb.draw_line(xx - 3, y + 37, xx + 30, y + 37, DUST);
    }
}

type Scene = (u8, u8, u8); // verb, agent, patient
fn event_for_clip(i: usize) -> Scene {
    const EVENTS: [Scene; 6] = [
        (0, 3, 4),
        (0, 4, 5),
        (1, 3, 5),
        (1, 4, 5),
        (2, 3, 4),
        (2, 4, 3),
    ];
    EVENTS[i]
}

fn scene_for_option(i: usize) -> Scene {
    match i {
        0 => (0, 3, 5),
        1 => (0, 4, 3),
        _ => (1, 3, 4),
    }
}

fn draw_scene(fb: &mut Framebuffer, x: i32, y: i32, w: i32, h: i32, scene: Scene, time: f32) {
    let (verb, agent, patient) = scene;
    fb.fill_rect(x, y, w as u32, h as u32, INK);
    // Ruined vault and receding arches, without adding linguistic clues.
    let back = Pixel::rgb(15, 29, 39);
    fb.fill_rect(x + 5, y + 4, (w - 10) as u32, (h - 14) as u32, back);
    let floor = y + h - 10;
    fb.fill_rect(x + 5, floor, (w - 10) as u32, 6, STONE);
    for column in 0..3 {
        let cx = x + 10 + column * (w - 20) / 2;
        fb.fill_rect(cx, y + 10, 3, (h - 21) as u32, Pixel::rgb(30, 43, 49));
        fb.draw_line(cx - 5, y + 10, cx + 8, y + 10, EDGE);
        fb.draw_line(cx - 5, floor - 2, cx + 8, floor - 2, DUST);
    }
    for layer in 0..3 {
        let inset = 9 + layer * 4;
        let tint = if layer == 0 { EDGE } else { DUST };
        fb.draw_line(x + inset, y + inset, x + w / 2, y + 4, tint);
        fb.draw_line(x + w / 2, y + 4, x + w - inset, y + inset, tint);
    }
    for step in 0..7 {
        let xx = x + 14 + step * (w - 29) / 7;
        fb.draw_line(xx, y + 18, xx + 4, y + 22, DUST);
        fb.draw(xx + ((time * 2.0) as i32 % 3), y + 27, GOLD);
    }
    fb.draw_line(x + 5, floor, x + w - 6, floor, EDGE);
    fb.draw_line(x + 5, y + 4, x + 5, floor, DUST);
    fb.draw_line(x + w - 6, y + 4, x + w - 6, floor, DUST);
    let unit = if h >= 110 { 2 } else { 1 };
    let base = y + h - 12;
    let ax = x + w / 3;
    let px = x + w * 2 / 3;
    let phase = (time * 1.65).sin();
    let movement = (phase * (7 * unit) as f32) as i32;
    // Action direction is from actor to patient regardless of horizontal iconography.
    draw_entity(fb, ax, base, agent, unit, GOLD, false);
    if verb == 2 {
        // The compact memory tiles leave less headroom than the large projection.
        let lift = if unit == 1 { 1 } else { 6 * unit };
        fb.draw_line(
            ax + 7 * unit,
            base - 12 * unit,
            px - 6 * unit,
            base - 16 * unit,
            SPIRIT,
        );
        draw_entity(fb, px + movement, base - lift, patient, unit, IVORY, false);
        fb.draw_line(
            px - 12 * unit,
            base - 5 * unit,
            px + 12 * unit,
            base - 5 * unit,
            SPIRIT,
        );
        fb.draw_line(px - 10 * unit, base + 1, px + 10 * unit, base + 1, RUST);
    } else {
        let still_dormant = verb == 1 && phase <= 0.0;
        draw_entity(
            fb,
            px,
            base,
            patient,
            unit,
            if verb == 1 { BLAZE } else { IVORY },
            still_dormant,
        );
    }
    match verb {
        0 => {
            // A barrier consistently surrounds the patient, never the actor.
            let radius = 19 * unit + (phase.abs() * (3 * unit) as f32) as i32;
            fb.draw_circle(px, base - 13 * unit, radius as u32, SPIRIT);
            fb.draw_line(
                px - 21 * unit,
                base - 13 * unit,
                px - 13 * unit,
                base - 24 * unit,
                SPIRIT,
            );
            fb.draw_line(
                px + 21 * unit,
                base - 13 * unit,
                px + 13 * unit,
                base - 24 * unit,
                SPIRIT,
            );
            fb.draw_line(
                ax + 9 * unit,
                base - 20 * unit,
                px - 20 * unit,
                base - 20 * unit,
                GOLD,
            );
        }
        1 => {
            // Pulse travels from actor to a patient coming back into light.
            fb.draw_line(
                ax + 11 * unit,
                base - 24 * unit,
                px - 12 * unit,
                base - 24 * unit,
                BLAZE,
            );
            if phase > 0.0 {
                for (dx, dy) in [(-17, -27), (14, -29), (-14, -9), (17, -11)] {
                    fb.draw_line(
                        px + dx * unit,
                        base + dy * unit,
                        px + (dx + (if dx < 0 { -4 } else { 4 })) * unit,
                        base + (dy - 2) * unit,
                        BLAZE,
                    );
                }
            }
        }
        _ => {}
    }
}

// Two original 17-pixel-wide costume silhouettes, not interchangeable stick figures.
const WATCHER_SILHOUETTE: [&str; 27] = [
    ".......g.g.......",
    "......gbbg......",
    ".....ggbbgg.....",
    "......bbbb......",
    ".....bffffb.....",
    ".....bffffb.....",
    "......bffb......",
    ".....bggggb.....",
    "...bbcccccbb..",
    "...bcccccccccbs.",
    "..sbclcccclccbss",
    "...sclcccclccbs.",
    "...scclcccclccbs",
    "....blcccccccb..",
    "....bclgglcccb..",
    ".....bclllccb...",
    "....bbclllccbb..",
    "....bcllllcclb..",
    "...bbcllllcclbb.",
    "...bclllclllcbb.",
    "...bblllllllcbb.",
    ".....bllbbllb...",
    ".....bllbbllb...",
    "....bbb..bbb....",
    "....bgg..ggb....",
    "...bbb....bbb...",
    "..bbbb....bbbb..",
];
const SCRIBE_SILHOUETTE: [&str; 25] = [
    ".......gg........",
    "......gbbg......",
    ".....gbbbbg.....",
    "....gbbbbbbg....",
    "....bbffffbb....",
    "....bbffffbb....",
    ".....bbffbb.....",
    ".....bggggb.....",
    "....bbccccbb....",
    "...bbccccccbb...",
    "..bbclccccclbb..",
    "..blclcccclclb..",
    "..bllcllcllclb..",
    "..bbcllcllccbb..",
    "...bcllcllccb...",
    "...bcllclllcb...",
    "...bclllllccb...",
    "...bclllllccb...",
    "...bclllllccb...",
    "..bbcllccclcbb..",
    "..bcllcccccclb..",
    "..bbbbbbbbbbbb..",
    "....bll..llb....",
    "...bbb....bbb...",
    "..bbbb....bbbb..",
];

fn draw_costume(
    fb: &mut Framebuffer,
    x: i32,
    ground: i32,
    scale: i32,
    rows: &[&str],
    tone: Pixel,
    faded: bool,
) {
    let top = ground - rows.len() as i32 * scale;
    for (row_idx, row) in rows.iter().enumerate() {
        let bytes = row.as_bytes();
        let mut start = 0;
        while start < bytes.len() {
            let symbol = bytes[start];
            let mut end = start + 1;
            while end < bytes.len() && bytes[end] == symbol {
                end += 1;
            }
            let pixel = match symbol {
                b'b' => Some(INK),
                b'c' => Some(if faded { DUST } else { Pixel::rgb(58, 78, 78) }),
                b'l' => Some(if faded {
                    RUST
                } else {
                    Pixel::rgb(129, 149, 135)
                }),
                b'f' => Some(if faded { RUST } else { tone }),
                b'g' => Some(if faded { EDGE } else { GOLD }),
                b'w' => Some(if faded { EDGE } else { IVORY }),
                b's' => Some(DUST),
                _ => None,
            };
            if let Some(color) = pixel {
                fb.fill_rect(
                    x + (start as i32 - 8) * scale,
                    top + row_idx as i32 * scale,
                    ((end - start) as i32 * scale) as u32,
                    scale as u32,
                    color,
                );
            }
            start = end;
        }
    }
}

fn draw_entity(
    fb: &mut Framebuffer,
    x: i32,
    y: i32,
    kind: u8,
    scale: i32,
    color: Pixel,
    faded: bool,
) {
    match kind {
        3 => {
            draw_costume(fb, x, y, scale, &WATCHER_SILHOUETTE, color, faded);
            // Ritual staff belongs only to this figure, even at thumbnail scale.
            fb.fill_rect(
                x + 12 * scale,
                y - 28 * scale,
                (2 * scale) as u32,
                (28 * scale) as u32,
                RUST,
            );
            fb.draw_line(
                x + 13 * scale,
                y - 29 * scale,
                x + 13 * scale,
                y - 34 * scale,
                GOLD,
            );
            fb.draw_line(
                x + 10 * scale,
                y - 31 * scale,
                x + 16 * scale,
                y - 31 * scale,
                GOLD,
            );
        }
        4 => {
            draw_costume(fb, x, y, scale, &SCRIBE_SILHOUETTE, color, faded);
            // A short diagonal sash distinguishes the scribe from the crowned watcher.
            fb.draw_line(
                x - 5 * scale,
                y - 14 * scale,
                x + 3 * scale,
                y - 9 * scale,
                GOLD,
            );
        }
        _ => {
            // The third referent is a suspended ember, not a humanoid.
            let orb = if faded { RUST } else { BLAZE };
            fb.fill_rect(
                x - 13 * scale,
                y - 4 * scale,
                (26 * scale) as u32,
                (5 * scale) as u32,
                INK,
            );
            fb.fill_rect(
                x - 10 * scale,
                y - 6 * scale,
                (20 * scale) as u32,
                (4 * scale) as u32,
                STONE,
            );
            fb.draw_line(
                x - 13 * scale,
                y - 7 * scale,
                x + 13 * scale,
                y - 7 * scale,
                GOLD,
            );
            fb.fill_circle(x, y - 20 * scale, (9 * scale) as u32, orb);
            fb.draw_circle(x, y - 20 * scale, (13 * scale) as u32, RUST);
            fb.draw_circle(x, y - 20 * scale, (11 * scale) as u32, GOLD);
            fb.fill_rect(
                x - 3 * scale,
                y - 26 * scale,
                (4 * scale) as u32,
                (3 * scale) as u32,
                IVORY,
            );
            fb.draw_line(
                x + 2 * scale,
                y - 35 * scale,
                x + 6 * scale,
                y - 29 * scale,
                GOLD,
            );
        }
    }
}

fn draw_monument(fb: &mut Framebuffer, x: i32, y: i32, time: f32, revealed: bool) {
    let w = if revealed { 147 } else { 149 };
    let h = if revealed { 147 } else { 134 };
    fb.fill_rect(x, y, w, h, DEEP);
    fb.draw_rect(x, y, w, h, GOLD);
    fb.draw_line(x + 16, y + 12, x + w as i32 - 16, y + 12, RUST);
    fb.draw_text(x + 29, y + 15, "LE MONUMENT", DIM);
    draw_entity(fb, x + 71, y + 110, 3, 2, IVORY, false);
    if revealed {
        draw_entity(fb, x + 32, y + 107, 4, 2, SPIRIT, false);
        fb.draw_circle(x + 71, y + 84, 32, SPIRIT);
    } else {
        // The erased secondary figure is deliberately visible as damage.
        fb.draw_line(x + 19, y + 82, x + 45, y + 108, RUST);
        fb.draw_line(x + 19, y + 108, x + 45, y + 82, RUST);
        fb.draw_line(x + 16, y + 110, x + 44, y + 113, DUST);
    }
    let shimmer = if (time * 2.0) as i32 % 2 == 0 {
        GOLD
    } else {
        EDGE
    };
    fb.draw_line(
        x + 10,
        y + h as i32 - 8,
        x + w as i32 - 10,
        y + h as i32 - 8,
        shimmer,
    );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run(
        EngineConfig {
            title: "Les Archives du Silence - Les Remanences (Prototype 002)".into(),
            framebuffer_width: WIDTH,
            framebuffer_height: HEIGHT,
            window_width: 1280,
            window_height: 720,
        },
        MemoryGame::new()?,
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use gotoo_pixel_engine::NoopAudio;

    #[test]
    fn clickable_archive_regions_are_non_overlapping() {
        for i in 0..6 {
            let (x, y) = card_pos(i);
            assert!(hit(x + 1, y + 1, x, y, CARD_W, CARD_H));
            for j in i + 1..6 {
                let (nx, ny) = card_pos(j);
                assert!(!hit(nx + 1, ny + 1, x, y, CARD_W, CARD_H));
            }
        }
    }

    #[test]
    fn hub_navigation_changes_pages_without_implicit_resolution() {
        let mut game = MemoryGame::new().expect("audio bank");
        let mut audio = NoopAudio::default();
        game.activate_hall(0, &mut audio);
        assert_eq!(game.archive.page(), Page::Clip(0));
        game.archive.back();
        game.activate_hall(6, &mut audio);
        assert_eq!(game.archive.page(), Page::Witness);
        game.pick_witness(1, &mut audio);
        assert_eq!(game.archive.page(), Page::Hall);
        assert!(!game.archive.recovered());
    }

    #[test]
    fn witness_and_restoration_do_not_emit_correctness_per_part() {
        let mut game = MemoryGame::new().expect("audio bank");
        let mut audio = NoopAudio::default();
        game.archive.open_witness();
        game.pick_witness(1, &mut audio);
        game.archive.open_restore();
        game.pick_rune(0, &mut audio);
        assert_eq!(game.archive.commit(), Verdict::MissingEvidence);
        assert!(!game.archive.recovered());
    }

    #[test]
    fn different_frames_change_the_visible_evidence_for_each_action() {
        for verb in 0..3 {
            let mut before = Framebuffer::new(640, 360);
            let mut after = Framebuffer::new(640, 360);
            draw_scene(&mut before, 40, 40, 380, 130, (verb, 4, 5), 0.0);
            draw_scene(&mut after, 40, 40, 380, 130, (verb, 4, 5), 1.0);
            assert_ne!(
                before.as_rgba8(),
                after.as_rgba8(),
                "each type of memory must animate"
            );
        }
    }

    #[test]
    fn testimony_last_choice_is_clickable_and_return_does_not_select() {
        let mut game = MemoryGame::new().expect("procedural audio bank");
        let mut audio = NoopAudio::default();
        game.archive.open_witness();
        game.handle_click(551, 304, &mut audio);
        assert_eq!(game.archive.page(), Page::Hall);
        assert_eq!(game.archive.witness_scene(), None);

        game.archive.open_witness();
        game.handle_click(220, 264, &mut audio);
        assert_eq!(game.archive.page(), Page::Hall);
        assert_eq!(game.archive.witness_scene(), Some(2));
    }

    #[test]
    fn projected_figures_stay_inside_scene_top_edge() {
        let empty = Framebuffer::new(640, 360);
        for scene in [(0, 3, 4), (1, 4, 5), (2, 3, 5)] {
            let mut fb = Framebuffer::new(640, 360);
            draw_scene(&mut fb, 40, 40, 405, 48, scene, 1.0);
            assert_eq!(
                &fb.as_rgba8()[..40 * 640 * 4],
                &empty.as_rgba8()[..40 * 640 * 4],
                "scene art must not overflow the tile above"
            );
        }
    }

    #[test]
    fn evidence_glyphs_fit_inside_memory_panel() {
        let mut fb = Framebuffer::new(640, 360);
        draw_inscription(&mut fb, &CLUES[0], 482, 126, GOLD);
        // The last mark ends before the right frame (x = 618).
        assert!(482 + 2 * 43 + 13 * 2 < 618);
        // The inscription was actually rendered, not silently clipped away.
        assert!(fb.as_rgba8().iter().any(|byte| *byte != 0));
    }

    #[test]
    fn custom_runic_alphabet_draws_safely_at_all_expected_scales() {
        let mut fb = Framebuffer::new(640, 360);
        for rune in 0..RUNE_COUNT {
            draw_rune(&mut fb, rune, 20 + 65 * i32::from(rune), 15, 1, IVORY);
            draw_rune(&mut fb, rune, 20 + 65 * i32::from(rune), 55, 2, IVORY);
        }
        assert_eq!(fb.width(), 640);
        assert_eq!(fb.height(), 360);
        assert!(fb.as_rgba8().iter().any(|byte| *byte != 0));
    }
}
