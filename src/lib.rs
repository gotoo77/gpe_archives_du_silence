//! SPOILERS FOR THE BLIND TESTER.
//! Deterministic archive logic. Visual evidence is shown by the GPE presentation.
//! Correctness is checked only on a full reconstruction, never one token at a time.

pub const ARCHIVE_COUNT: usize = 6;
pub const RUNE_COUNT: u8 = 6;
pub const EMPTY_RUNE: u8 = u8::MAX;
pub type Sentence = [u8; 3];

// VSO roles are never shown as Latin labels inside the game.
pub const CLUES: [Sentence; ARCHIVE_COUNT] = [
    [0, 3, 4],
    [0, 4, 5],
    [1, 3, 5],
    [1, 4, 5],
    [2, 3, 4],
    [2, 4, 3],
];
// An unillustrated testimony reverses the official monument's story.
pub const WITNESS: Sentence = [0, 4, 3];
// The player also reconstructs a new sentence from a silent scene.
pub const RECONSTRUCTION: Sentence = [2, 3, 5];
const WITNESS_SCENE: usize = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Hall,
    Clip(usize),
    Witness,
    Restore,
    Recovered,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    MissingEvidence,
    Discordant,
    Recovered,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Archive {
    page: Page,
    focus: usize,
    watched: [bool; ARCHIVE_COUNT],
    witness_scene: Option<usize>,
    draft: Sentence,
    slot: usize,
    commitments: u32,
    recovered: bool,
}

impl Default for Archive {
    fn default() -> Self {
        Self {
            page: Page::Hall,
            focus: 0,
            watched: [false; ARCHIVE_COUNT],
            witness_scene: None,
            draft: [EMPTY_RUNE; 3],
            slot: 0,
            commitments: 0,
            recovered: false,
        }
    }
}

impl Archive {
    pub fn page(&self) -> Page {
        self.page
    }

    pub fn focus(&self) -> usize {
        self.focus
    }

    pub fn set_focus(&mut self, focus: usize) {
        self.focus = focus.min(7);
    }

    pub fn watched(&self, index: usize) -> bool {
        self.watched.get(index).copied().unwrap_or(false)
    }

    pub fn witness_scene(&self) -> Option<usize> {
        self.witness_scene
    }

    pub fn draft(&self) -> Sentence {
        self.draft
    }

    pub fn slot(&self) -> usize {
        self.slot
    }

    pub fn commitments(&self) -> u32 {
        self.commitments
    }

    pub fn recovered(&self) -> bool {
        self.recovered
    }

    /// Focus the memory projected by a physical archive cabinet.
    pub fn open_clip(&mut self, index: usize) -> bool {
        if self.recovered || index >= ARCHIVE_COUNT {
            return false;
        }
        self.watched[index] = true;
        self.page = Page::Clip(index);
        true
    }

    pub fn open_witness(&mut self) {
        if !self.recovered {
            self.page = Page::Witness;
        }
    }

    pub fn choose_witness_scene(&mut self, index: usize) -> bool {
        if self.page != Page::Witness || index >= 3 {
            return false;
        }
        self.witness_scene = Some(index);
        true
    }

    pub fn open_restore(&mut self) {
        if !self.recovered {
            self.page = Page::Restore;
        }
    }

    pub fn set_slot(&mut self, index: usize) {
        if index < 3 {
            self.slot = index;
        }
    }

    pub fn insert(&mut self, rune: u8) -> bool {
        if self.page != Page::Restore || rune >= RUNE_COUNT {
            return false;
        }
        self.draft[self.slot] = rune;
        true
    }

    pub fn clear_slot(&mut self) {
        if self.page == Page::Restore {
            self.draft[self.slot] = EMPTY_RUNE;
        }
    }

    pub fn back(&mut self) {
        if self.page != Page::Recovered {
            self.page = Page::Hall;
        }
    }

    /// No per-slot or per-question correctness signal is provided.
    pub fn commit(&mut self) -> Verdict {
        if self.recovered {
            return Verdict::Recovered;
        }
        if self.witness_scene.is_none() || self.draft.contains(&EMPTY_RUNE) {
            return Verdict::MissingEvidence;
        }
        self.commitments = self.commitments.saturating_add(1);
        if self.witness_scene == Some(WITNESS_SCENE) && self.draft == RECONSTRUCTION {
            self.recovered = true;
            self.page = Page::Recovered;
            Verdict::Recovered
        } else {
            Verdict::Discordant
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_clue_sentences_are_distinct_and_transfer_sentences_are_unseen() {
        for (i, clue) in CLUES.iter().enumerate() {
            assert!(clue.iter().all(|r| *r < RUNE_COUNT));
            assert!(!CLUES[i + 1..].contains(clue));
            assert_ne!(*clue, WITNESS);
            assert_ne!(*clue, RECONSTRUCTION);
        }
        assert_ne!(WITNESS, RECONSTRUCTION);
    }

    #[test]
    fn out_of_bounds_clip_and_glyph_are_rejected() {
        let mut archive = Archive::default();
        assert!(!archive.open_clip(ARCHIVE_COUNT));
        assert_eq!(archive.page(), Page::Hall);
        archive.open_restore();
        assert!(!archive.insert(RUNE_COUNT));
        assert_eq!(archive.draft(), [EMPTY_RUNE; 3]);
    }

    #[test]
    fn inspection_is_unrestricted_and_order_independent() {
        let mut a = Archive::default();
        assert!(a.open_clip(5));
        a.back();
        assert!(a.open_clip(0));
        assert!(a.watched(0) && a.watched(5));
        assert!(!a.watched(4));
        assert_eq!(a.page(), Page::Clip(0));
    }

    #[test]
    fn no_partial_score_leaks_from_witness_selection() {
        let mut a = Archive::default();
        a.open_witness();
        assert!(a.choose_witness_scene(2));
        a.back();
        assert_eq!(a.commit(), Verdict::MissingEvidence);
        assert_eq!(a.commitments(), 0);
        a.open_restore();
        for (i, r) in RECONSTRUCTION.iter().enumerate() {
            a.set_slot(i);
            assert!(a.insert(*r));
        }
        assert_eq!(a.commit(), Verdict::Discordant);
        assert!(!a.recovered());
        assert_eq!(a.commitments(), 1);
        a.open_witness();
        assert!(a.choose_witness_scene(WITNESS_SCENE));
        assert_eq!(a.commit(), Verdict::Recovered);
        assert!(a.recovered());
    }

    #[test]
    fn completely_wrong_reconstruction_is_never_accepted() {
        let mut a = Archive::default();
        a.open_witness();
        a.choose_witness_scene(WITNESS_SCENE);
        a.open_restore();
        for i in 0..3 {
            a.set_slot(i);
            assert!(a.insert(0));
        }
        assert_eq!(a.commit(), Verdict::Discordant);
        assert_eq!(a.page(), Page::Restore);
    }

    #[test]
    fn cannot_claim_restoration_without_both_independent_tasks() {
        let mut a = Archive::default();
        a.open_witness();
        a.choose_witness_scene(WITNESS_SCENE);
        assert_eq!(a.commit(), Verdict::MissingEvidence);
        a.open_restore();
        a.insert(RECONSTRUCTION[0]);
        assert_eq!(a.commit(), Verdict::MissingEvidence);
        assert!(!a.recovered());
    }

    #[test]
    fn slots_are_editable_without_checking_local_correctness() {
        let mut a = Archive::default();
        a.open_restore();
        a.set_slot(1);
        assert!(a.insert(3));
        assert!(a.insert(1));
        assert_eq!(a.draft(), [EMPTY_RUNE, 1, EMPTY_RUNE]);
        a.clear_slot();
        assert_eq!(a.draft(), [EMPTY_RUNE; 3]);
    }

    #[test]
    fn success_reveals_archive_once_and_prevents_editing() {
        let mut a = Archive::default();
        a.open_witness();
        a.choose_witness_scene(WITNESS_SCENE);
        a.open_restore();
        for (i, r) in RECONSTRUCTION.iter().enumerate() {
            a.set_slot(i);
            a.insert(*r);
        }
        assert_eq!(a.commit(), Verdict::Recovered);
        assert_eq!(a.commit(), Verdict::Recovered);
        assert_eq!(a.commitments(), 1);
        a.back();
        assert_eq!(a.page(), Page::Recovered);
        assert!(!a.open_clip(0));
    }
}
