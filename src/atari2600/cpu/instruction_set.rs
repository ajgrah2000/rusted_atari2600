use super::super::clocks;
use super::pc_state;

//TODO: Do actual instructions
pub fn noop(clock: &mut clocks::Clock, pc_state: &mut pc_state::PcState) {
    clock.increment(4);
}

