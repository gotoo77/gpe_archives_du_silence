//! Deterministic rules for the first archaeological prototype.
//! Spoiler notice: this module is not intended for a blind playtester.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Console {
    First,
    Second,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reaction {
    Changed,
    Unchanged,
    Invalid,
    Inaccessible,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct World {
    first_active: bool,
    second_active: bool,
}

impl Default for World {
    fn default() -> Self {
        Self {
            first_active: true,
            second_active: false,
        }
    }
}

impl World {
    pub fn first_active(&self) -> bool {
        self.first_active
    }

    pub fn second_active(&self) -> bool {
        self.second_active
    }

    pub fn accessible(&self, console: Console) -> bool {
        console == Console::First || !self.first_active
    }

    /// Every console addresses its local machine only.
    /// Other symbol combinations never modify any device.
    pub fn submit(&mut self, console: Console, symbols: [usize; 2]) -> Reaction {
        if !self.accessible(console) {
            return Reaction::Inaccessible;
        }

        let expected_object = match console {
            Console::First => 3,
            Console::Second => 4,
        };

        if symbols[0] > 1 || symbols[1] != expected_object {
            return Reaction::Invalid;
        }

        let desired_active = symbols[0] == 0;
        let current = match console {
            Console::First => &mut self.first_active,
            Console::Second => &mut self.second_active,
        };

        if *current == desired_active {
            Reaction::Unchanged
        } else {
            *current = desired_active;
            Reaction::Changed
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_scene_is_observable_but_second_console_is_unreachable() {
        let world = World::default();
        assert!(world.first_active());
        assert!(!world.second_active());
        assert!(world.accessible(Console::First));
        assert!(!world.accessible(Console::Second));
    }

    #[test]
    fn puzzle_requires_two_local_state_changes() {
        let mut world = World::default();
        assert_eq!(world.submit(Console::First, [1, 3]), Reaction::Changed);
        assert!(!world.first_active());
        assert!(world.accessible(Console::Second));
        assert_eq!(world.submit(Console::Second, [0, 4]), Reaction::Changed);
        assert!(world.second_active());
    }

    #[test]
    fn first_console_cannot_control_second_device() {
        let mut world = World::default();
        assert_eq!(world.submit(Console::First, [0, 4]), Reaction::Invalid);
        assert!(!world.second_active());
        assert!(world.first_active());
    }

    #[test]
    fn inaccessible_console_cannot_be_activated() {
        let mut world = World::default();
        assert_eq!(world.submit(Console::Second, [0, 4]), Reaction::Inaccessible);
        assert!(!world.second_active());
    }

    #[test]
    fn invalid_symbols_do_not_mutate_world() {
        let mut world = World::default();
        let before = world;
        for symbols in [[2, 3], [0, 1], [1, 4], [3, 3], [5, 3]] {
            assert_eq!(world.submit(Console::First, symbols), Reaction::Invalid);
            assert_eq!(world, before);
        }
    }

    #[test]
    fn requesting_existing_state_is_idempotent() {
        let mut world = World::default();
        assert_eq!(world.submit(Console::First, [0, 3]), Reaction::Unchanged);
        assert_eq!(world, World::default());
    }

    #[test]
    fn active_first_device_can_be_reactivated_after_shutdown() {
        let mut world = World::default();
        assert_eq!(world.submit(Console::First, [1, 3]), Reaction::Changed);
        assert_eq!(world.submit(Console::First, [0, 3]), Reaction::Changed);
        assert!(world.first_active());
        assert!(!world.accessible(Console::Second));
    }

    #[test]
    fn second_device_does_not_revert_when_first_reactivates() {
        let mut world = World::default();
        assert_eq!(world.submit(Console::First, [1, 3]), Reaction::Changed);
        assert_eq!(world.submit(Console::Second, [0, 4]), Reaction::Changed);
        assert_eq!(world.submit(Console::First, [0, 3]), Reaction::Changed);
        assert!(world.second_active());
    }
}
