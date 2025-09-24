#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(transparent)]
pub struct Signal {
    pub current_value: bool,
}

/// A representation of a boolean signal
impl Signal {
    /// Returns true if the value is true, and the previous value is false, otherwise false
    pub fn rising_edge(&mut self, next_value: bool) -> bool {
        let res = next_value && (self.current_value ^ next_value);
        self.current_value = next_value;
        res
    }

    /// Returns true if the value is false and the previous value is true, otherwise it returns false
    pub fn falling_edge(&mut self, next_value: bool) -> bool {
        let res = !next_value && (self.current_value ^ next_value);
        self.current_value = next_value;
        res
    }

    pub fn set(&mut self, value: bool) {
        self.current_value = value;
    }

    pub fn get(&self) -> bool {
        self.current_value
    }
}

impl From<bool> for Signal {
    fn from(val: bool) -> Self {
        Signal { current_value: val }
    }
}

#[cfg(test)]
mod tests {
    use super::Signal;

    #[test]
    fn rising_edge_subsequent_true() {
        //Given a signal
        let mut re = Signal::default();
        //When the signal is true
        //Then the rising edge is true
        assert!(re.rising_edge(true));
        //If the signal remains true
        //Then the rising edge is false
        assert!(!re.rising_edge(true));
    }

    #[test]
    fn rising_edge_true_false_true_true() {
        //Given a signal
        let mut re = Signal::default();
        //If the signal is true
        //Then the rising edge is true
        assert!(re.rising_edge(true));
        //If the signal becomes false
        //Then the rising edge is false
        assert!(!re.rising_edge(false));
        //If the signal becomes true again
        //Then the rising edge becomes true again
        assert!(re.rising_edge(true));
        //If the signal remains true
        //Then the rising edge becomes false again
        assert!(!re.rising_edge(true));
    }

    #[test]
    fn rising_edge_reuse() {
        //Given a signal
        let mut re = Signal::default();
        //When the signal is false
        //Then the rising edge is false
        assert!(!re.rising_edge(false));
        //When the signal becomes true
        //Then the rising edge becomes true
        assert!(re.rising_edge(true));
        //If the signal remains true
        //Then the rising edge becomes false
        assert!(!re.rising_edge(true));
        //If the signal becomes false
        //Then the rising edge becomes false
        assert!(!re.rising_edge(false));
        //If the signal becomes true again
        //Then the rising edge becomes true
        assert!(re.rising_edge(true));
    }

    #[test]
    fn falling_edge_returns_false_on_subsequent_true() {
        //Given a signal
        let mut fe = Signal::default();
        //If the signal is true
        //Then the falling edge is false
        assert!(!fe.falling_edge(true));
        //If the signal remains true
        //Then the falling edge remains false
        assert!(!fe.falling_edge(true));
    }

    #[test]
    fn falling_edge_true_false_true_true() {
        //Given a signal
        let mut fe = Signal::default();
        //When the signal is set to true
        //Then the falling edge is false
        assert!(!fe.falling_edge(true));
        //When the signal is then set to false
        //the the falling edge becomes true
        assert!(fe.falling_edge(false));
        //When the signal remains false
        //the the falling is false again
        assert!(!fe.falling_edge(false));
        //When the signal is set to true again
        //then the falling edge remains false
        assert!(!fe.falling_edge(true));
        //When the signal is set to true again
        //Then the falling edge remains false
        assert!(!fe.falling_edge(true));
    }

    #[test]
    fn falling_edge_reuse() {
        //Given a signal
        let mut fe = Signal::default();
        //When the signal is set to true
        //Then the falling edge is false
        assert!(!fe.falling_edge(true));
        //When the signal is then set to false
        //the the falling edge becomes true
        assert!(fe.falling_edge(false));
        //When the signal is set to true again
        //then the falling edge becomes false
        assert!(!fe.falling_edge(true));
        //When the signal is set to true again
        //Then the falling edge remains false
        assert!(fe.falling_edge(false));
        //When the signal remains false
        //Then the falling edge remains false
        assert!(!fe.falling_edge(false));
    }
}
