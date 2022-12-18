//! Unit tests.

use super::*;

/// Set on state.
#[test]
fn set_on() {
    let mut signal = TimedSignal::new();

    let state = signal.update(0);
    assert_eq!(state, false);

    signal.set(true);
    let state = signal.update(1);
    assert_eq!(state, true);
}

/// Set off state.
#[test]
fn set_off() {
    let mut signal = TimedSignal::new();

    let state = signal.update(0);
    assert_eq!(state, false);

    signal.on();
    signal.set(false);
    let state = signal.update(1);
    assert_eq!(state, false);
}

/// Set on.
#[test]
fn on() {
    let mut signal = TimedSignal::new();

    let state = signal.update(0);
    assert_eq!(state, false);

    signal.on();
    let state = signal.update(1);
    assert_eq!(state, true);
}

/// Set off.
#[test]
fn off() {
    let mut signal = TimedSignal::new();

    let state = signal.update(0);
    assert_eq!(state, false);

    signal.on();
    signal.off();
    let state = signal.update(1);
    assert_eq!(state, false);
}

/// Blink.
#[test]
fn blink() {
    let mut signal = TimedSignal::new();
    let expected_states = [
        true, true, true, false, false, false, true, true, true, false, false, false,
    ];

    signal.blink(6);

    for (tick, expected_state) in expected_states.into_iter().enumerate() {
        let state = signal.update(tick as u64);
        assert_eq!(state, expected_state);
    }
}

/// Set on state at specific tick when state is initially off.
#[test]
fn on_at_when_off() {
    let mut signal = TimedSignal::new();
    let expected_states = [false, false, true, true];

    signal.off();

    for (tick, expected_state) in expected_states.into_iter().enumerate() {
        if tick == 1 {
            signal.on_at(2);
        }
        let state = signal.update(tick as u64);
        assert_eq!(state, expected_state);
    }
}

/// Set on state at specific tick when state is initially on.
#[test]
fn on_at_when_on() {
    let mut signal = TimedSignal::new();
    let expected_states = [true, true, true, true];

    signal.on();

    for (tick, expected_state) in expected_states.into_iter().enumerate() {
        if tick == 1 {
            signal.on_at(2);
        }
        let state = signal.update(tick as u64);
        assert_eq!(state, expected_state);
    }
}

/// Set on state until a specific tick when state is initially off.
#[test]
fn on_until_when_off() {
    let mut signal = TimedSignal::new();
    let expected_states = [false, true, false, false];

    signal.off();

    for (tick, expected_state) in expected_states.into_iter().enumerate() {
        if tick == 1 {
            signal.on_until(2);
        }
        let state = signal.update(tick as u64);
        assert_eq!(state, expected_state);
    }
}

/// Set on state until a specific tick when state is initially on.
#[test]
fn on_until_when_on() {
    let mut signal = TimedSignal::new();
    let expected_states = [true, true, false, false];

    signal.on();

    for (tick, expected_state) in expected_states.into_iter().enumerate() {
        if tick == 1 {
            signal.on_until(2);
        }
        let state = signal.update(tick as u64);
        assert_eq!(state, expected_state);
    }
}

/// Set off state at specific tick when state is initially off.
#[test]
fn off_at_when_off() {
    let mut signal = TimedSignal::new();
    let expected_states = [false, false, false, false];

    signal.off();

    for (tick, expected_state) in expected_states.into_iter().enumerate() {
        if tick == 1 {
            signal.off_at(2);
        }
        let state = signal.update(tick as u64);
        assert_eq!(state, expected_state);
    }
}

/// Set off state at specific tick when state is initially on.
#[test]
fn off_at_when_on() {
    let mut signal = TimedSignal::new();
    let expected_states = [true, true, false, false];

    signal.on();

    for (tick, expected_state) in expected_states.into_iter().enumerate() {
        if tick == 1 {
            signal.off_at(2);
        }
        let state = signal.update(tick as u64);
        assert_eq!(state, expected_state);
    }
}

/// Set off state until a specific tick when state is initially off.
#[test]
fn off_until_when_off() {
    let mut signal = TimedSignal::new();
    let expected_states = [false, false, true, true];

    signal.off();

    for (tick, expected_state) in expected_states.into_iter().enumerate() {
        if tick == 1 {
            signal.off_until(2);
        }
        let state = signal.update(tick as u64);
        assert_eq!(state, expected_state);
    }
}

/// Set off state until a specific tick when state is initially on.
#[test]
fn off_until_when_on() {
    let mut signal = TimedSignal::new();
    let expected_states = [true, false, true, true];

    signal.on();

    for (tick, expected_state) in expected_states.into_iter().enumerate() {
        if tick == 1 {
            signal.off_until(2);
        }
        let state = signal.update(tick as u64);
        assert_eq!(state, expected_state);
    }
}

/// Set on state at specific tick when state is initially off.
#[test]
fn set_on_at_when_off() {
    let mut signal = TimedSignal::new();
    let expected_states = [false, false, true, true];

    signal.off();

    for (tick, expected_state) in expected_states.into_iter().enumerate() {
        if tick == 1 {
            signal.set_at(2, true);
        }
        let state = signal.update(tick as u64);
        assert_eq!(state, expected_state);
    }
}

/// Set on state at specific tick when state is initially on.
#[test]
fn set_on_at_when_on() {
    let mut signal = TimedSignal::new();
    let expected_states = [true, true, true, true];

    signal.on();

    for (tick, expected_state) in expected_states.into_iter().enumerate() {
        if tick == 1 {
            signal.set_at(2, true);
        }
        let state = signal.update(tick as u64);
        assert_eq!(state, expected_state);
    }
}

/// Set off state at specific tick when state is initially off.
#[test]
fn set_off_at_when_off() {
    let mut signal = TimedSignal::new();
    let expected_states = [false, false, false, false];

    signal.off();

    for (tick, expected_state) in expected_states.into_iter().enumerate() {
        if tick == 1 {
            signal.set_at(2, false);
        }
        let state = signal.update(tick as u64);
        assert_eq!(state, expected_state);
    }
}

/// Set off state at specific tick when state is initially on.
#[test]
fn set_off_at_when_on() {
    let mut signal = TimedSignal::new();
    let expected_states = [true, true, false, false];

    signal.on();

    for (tick, expected_state) in expected_states.into_iter().enumerate() {
        if tick == 1 {
            signal.set_at(2, false);
        }
        let state = signal.update(tick as u64);
        assert_eq!(state, expected_state);
    }
}

/// Set on state until a specific tick when state is initially off.
#[test]
fn set_on_until_when_off() {
    let mut signal = TimedSignal::new();
    let expected_states = [false, true, false, false];

    signal.off();

    for (tick, expected_state) in expected_states.into_iter().enumerate() {
        if tick == 1 {
            signal.set_until(2, true);
        }
        let state = signal.update(tick as u64);
        assert_eq!(state, expected_state);
    }
}

/// Set on state until a specific tick when state is initially on.
#[test]
fn set_on_until_when_on() {
    let mut signal = TimedSignal::new();
    let expected_states = [true, true, false, false];

    signal.on();

    for (tick, expected_state) in expected_states.into_iter().enumerate() {
        if tick == 1 {
            signal.set_until(2, true);
        }
        let state = signal.update(tick as u64);
        assert_eq!(state, expected_state);
    }
}

/// Set off state until a specific tick when state is initially off.
#[test]
fn set_off_until_when_off() {
    let mut signal = TimedSignal::new();
    let expected_states = [false, false, true, true];

    signal.off();

    for (tick, expected_state) in expected_states.into_iter().enumerate() {
        if tick == 1 {
            signal.set_until(2, false);
        }
        let state = signal.update(tick as u64);
        assert_eq!(state, expected_state);
    }
}

/// Set off state until a specific tick when state is initially on.
#[test]
fn set_off_until_when_on() {
    let mut signal = TimedSignal::new();
    let expected_states = [true, false, true, true];

    signal.on();

    for (tick, expected_state) in expected_states.into_iter().enumerate() {
        if tick == 1 {
            signal.set_until(2, false);
        }
        let state = signal.update(tick as u64);
        assert_eq!(state, expected_state);
    }
}

/// Set on state for a number of ticks. Single trigger.
#[test]
fn set_on_for() {
    let mut signal = TimedSignal::new();
    let expected_states = [false, true, true, false];

    signal.off();

    for (tick, expected_state) in expected_states.into_iter().enumerate() {
        if tick == 1 || tick == 2 {
            signal.set_for(2, true);
        }
        let state = signal.update(tick as u64);
        assert_eq!(state, expected_state);
    }
}

/// Set on state for a number of ticks. Retrigger.
#[test]
fn set_on_for_retrigger() {
    let mut signal = TimedSignal::new();
    let expected_states = [false, true, true, true, false];

    signal.off();

    for (tick, expected_state) in expected_states.into_iter().enumerate() {
        if tick == 1 || tick == 2 {
            signal.set_for_retrigger(2, true);
        }
        let state = signal.update(tick as u64);
        assert_eq!(state, expected_state);
    }
}

/// Toggle state at a specific tick when state is initially off.
#[test]
fn toggle_at_when_off() {
    let mut signal = TimedSignal::new();
    let expected_states = [false, false, true, true];

    signal.off();

    for (tick, expected_state) in expected_states.into_iter().enumerate() {
        if tick == 1 {
            signal.toggle_at(2);
        }
        let state = signal.update(tick as u64);
        assert_eq!(state, expected_state);
    }
}

/// Toggle state at a specific tick when state is initially on.
#[test]
fn toggle_at_when_on() {
    let mut signal = TimedSignal::new();
    let expected_states = [true, true, false, false];

    signal.on();

    for (tick, expected_state) in expected_states.into_iter().enumerate() {
        if tick == 1 {
            signal.toggle_at(2);
        }
        let state = signal.update(tick as u64);
        assert_eq!(state, expected_state);
    }
}

/// Toggle state until a specific tick when state is initially off.
#[test]
fn toggle_until_when_off() {
    let mut signal = TimedSignal::new();
    let expected_states = [false, true, false, false];

    signal.off();

    for (tick, expected_state) in expected_states.into_iter().enumerate() {
        if tick == 1 {
            signal.toggle_until(2);
        }
        let state = signal.update(tick as u64);
        assert_eq!(state, expected_state);
    }
}

/// Toggle state until a specific tick when state is initially on.
#[test]
fn toggle_until_when_on() {
    let mut signal = TimedSignal::new();
    let expected_states = [true, false, true, true];

    signal.on();

    for (tick, expected_state) in expected_states.into_iter().enumerate() {
        if tick == 1 {
            signal.toggle_until(2);
        }
        let state = signal.update(tick as u64);
        assert_eq!(state, expected_state);
    }
}

/// Toggle state for a number of ticks when state is initially off.
#[test]
fn toggle_for_when_off() {
    let mut signal = TimedSignal::new();
    let expected_states = [false, true, true, false];

    signal.off();

    for (tick, expected_state) in expected_states.into_iter().enumerate() {
        if tick == 1 || tick == 2 {
            signal.toggle_for(2);
        }
        let state = signal.update(tick as u64);
        assert_eq!(state, expected_state);
    }
}

/// Toggle state for a number of ticks when state is initially on.
#[test]
fn toggle_for_when_on() {
    let mut signal = TimedSignal::new();
    let expected_states = [true, false, false, true];

    signal.on();

    for (tick, expected_state) in expected_states.into_iter().enumerate() {
        if tick == 1 || tick == 2 {
            signal.toggle_for(2);
        }
        let state = signal.update(tick as u64);
        assert_eq!(state, expected_state);
    }
}

/// Toggle state for a number of ticks when state is initially off. Retrigger.
#[test]
fn toggle_for_when_off_retrigger() {
    let mut signal = TimedSignal::new();
    let expected_states = [false, true, true, true, false];

    signal.off();

    for (tick, expected_state) in expected_states.into_iter().enumerate() {
        if tick == 1 || tick == 2 {
            signal.toggle_for_retrigger(2);
        }
        let state = signal.update(tick as u64);
        assert_eq!(state, expected_state);
    }
}

/// Toggle state for a number of ticks when state is initially on. Retrigger.
#[test]
fn toggle_for_when_on_retrigger() {
    let mut signal = TimedSignal::new();
    let expected_states = [true, false, false, false, true];

    signal.on();

    for (tick, expected_state) in expected_states.into_iter().enumerate() {
        if tick == 1 || tick == 2 {
            signal.toggle_for_retrigger(2);
        }
        let state = signal.update(tick as u64);
        assert_eq!(state, expected_state);
    }
}
