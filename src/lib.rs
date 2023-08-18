#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), no_std)]

/// Signal state and scheduled tick action.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TimedSignal {
    /// Current state, false = off, true = on.
    state: bool,

    /// Timed action.
    tick_action: TickAction,
}

/// Action to perform when update() is called on a specific tick.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
enum TickAction {
    /// No action.
    #[default]
    None,

    /// Set a state when tick is reached.
    Set(u64, bool),

    /// Set a state for a number of ticks.
    /// This is a temporary state and will be changed to `Set` within `update()`.
    SetFor(u64, bool),

    /// Blink with given period.
    Blink(u32),
}

impl TimedSignal {
    /// Return a new instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Update internals and return state.
    /// When an action is scheduled and tick reaches the specific value for that
    /// action, the state is updated accordingly.
    pub fn update(&mut self, tick: u64) -> bool {
        match self.tick_action {
            TickAction::Set(trigger_tick, state) => {
                if tick >= trigger_tick {
                    self.state = state;
                    self.tick_action = TickAction::None;
                }
            }
            TickAction::SetFor(duration_ticks, state) => {
                self.state = state;
                self.tick_action = TickAction::Set(tick + duration_ticks, !state);
                return self.update(tick);
            }
            TickAction::Blink(blink_period) => {
                self.state = (tick as u32 % blink_period) < (blink_period >> 1)
            }
            _ => {}
        }

        self.state
    }

    /// Return current state.
    pub fn state(&self) -> bool {
        self.state
    }

    /// Set state according to a boolean value.
    pub fn set(&mut self, state: bool) {
        self.state = state;
        self.tick_action = TickAction::None;
    }

    /// Set on state.
    pub fn on(&mut self) {
        self.set(true);
    }

    /// Set off state.
    pub fn off(&mut self) {
        self.set(false);
    }

    /// Toggle state.
    pub fn toggle(&mut self) {
        self.set(!self.state);
    }

    /// Toggle state at a predefined rate with a 50% ratio.
    /// `period` is the number of ticks for one period. E.g. if set to 4, the state will be
    /// on for 2 ticks and off for another 2 ticks.
    pub fn blink(&mut self, period: u32) {
        self.tick_action = TickAction::Blink(period);
    }

    /// Set a state when a tick is reached.
    pub fn set_at(&mut self, trigger_tick: u64, state: bool) {
        self.tick_action = TickAction::Set(trigger_tick, state);
    }

    /// Set a state until a tick is reached, then return to previous state.
    pub fn set_until(&mut self, trigger_tick: u64, state: bool) {
        self.set(state);
        self.tick_action = TickAction::Set(trigger_tick, !state);
    }

    /// Set a state for a number of ticks, then invert it. Single trigger.
    pub fn set_for(&mut self, duration_ticks: u64, state: bool) {
        if self.tick_action == TickAction::None {
            self.tick_action = TickAction::SetFor(duration_ticks, state);
        }
    }

    /// Set a state for a number of ticks, then invert it. Retrigger.
    pub fn set_for_retrigger(&mut self, duration_ticks: u64, state: bool) {
        self.tick_action = TickAction::SetFor(duration_ticks, state);
    }

    /// Set on state when a tick is reached.
    pub fn on_at(&mut self, trigger_tick: u64) {
        self.set_at(trigger_tick, true);
    }

    /// Set on state until a tick is reached, then set it off.
    pub fn on_until(&mut self, trigger_tick: u64) {
        self.set_until(trigger_tick, true);
    }

    /// Set on state for a number of ticks, then set it off.
    pub fn on_for(&mut self, duration_ticks: u64) {
        self.set_for(duration_ticks, true);
    }

    /// Set off state when a tick is reached.
    pub fn off_at(&mut self, trigger_tick: u64) {
        self.set_at(trigger_tick, false);
    }

    /// Set off state until a tick is reached, then set it on.
    pub fn off_until(&mut self, trigger_tick: u64) {
        self.set_until(trigger_tick, false);
    }

    /// Set off state for a number of ticks, then set it on.
    pub fn off_for(&mut self, duration_ticks: u64) {
        self.set_for(duration_ticks, false);
    }

    /// Toggle state when a tick is reached.
    pub fn toggle_at(&mut self, trigger_tick: u64) {
        if self.tick_action == TickAction::None {
            self.tick_action = TickAction::Set(trigger_tick, !self.state);
        }
    }

    /// Toggle state until a tick is reached, then return to previous state.
    pub fn toggle_until(&mut self, trigger_tick: u64) {
        if self.tick_action == TickAction::None {
            self.toggle();
            self.tick_action = TickAction::Set(trigger_tick, !self.state);
        }
    }

    /// Toggle state for a number of ticks, then return to previous state. Single trigger.
    pub fn toggle_for(&mut self, duration_ticks: u64) {
        if self.tick_action == TickAction::None {
            self.tick_action = TickAction::SetFor(duration_ticks, !self.state);
        }
    }

    /// Toggle state for a number of ticks, then return to previous state. Retrigger.
    pub fn toggle_for_retrigger(&mut self, duration_ticks: u64) {
        match self.tick_action {
            TickAction::Set(_, state) => {
                self.tick_action = TickAction::SetFor(duration_ticks, !state);
            }
            _ => {
                self.tick_action = TickAction::SetFor(duration_ticks, !self.state);
            }
        }
    }
}

#[cfg(test)]
mod tests;
