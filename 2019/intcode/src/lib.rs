#[derive(Clone)]
pub struct Computer {
    memory: Vec<i64>,
    instruction_pointer: usize,
    relative_base: i64,
    parameter_modes: [ParameterMode; 3],
    state: State,
}

impl Computer {
    pub fn new(input: &str) -> Option<Computer> {
        let memory: Option<Vec<i64>> = input.split(',').map(|n| n.parse().ok()).collect();
        Some(Computer {
            memory: memory?,
            instruction_pointer: 0,
            relative_base: 0,
            parameter_modes: [ParameterMode::Position; 3],
            state: State::Running,
        })
    }

    pub fn run(&mut self) {
        while self.state == State::Running {
            if let Some(modes) =
                ParameterMode::modes(self.get_direct(self.instruction_pointer) / 100)
            {
                self.parameter_modes = modes;
                if self.run_instruction().is_none() {
                    self.state = State::Error;
                }
            } else {
                self.state = State::Error;
            }
        }
    }

    fn run_instruction(&mut self) -> Option<()> {
        match self.get_direct(self.instruction_pointer) % 100 {
            1 => {
                self.set(3, self.get(1)? + self.get(2)?)?;
                self.instruction_pointer += 4;
            }
            2 => {
                self.set(3, self.get(1)? * self.get(2)?)?;
                self.instruction_pointer += 4;
            }
            3 => self.state = State::Input,
            4 => self.state = State::Output,
            5 => {
                if self.get(1)? != 0 {
                    let new = self.get(2)?;
                    if new >= 0 {
                        self.instruction_pointer = new as usize;
                    } else {
                        return None;
                    }
                } else {
                    self.instruction_pointer += 3;
                }
            }
            6 => {
                if self.get(1)? == 0 {
                    let new = self.get(2)?;
                    if new >= 0 {
                        self.instruction_pointer = new as usize;
                    } else {
                        return None;
                    }
                } else {
                    self.instruction_pointer += 3;
                }
            }
            7 => {
                if self.get(1)? < self.get(2)? {
                    self.set(3, 1)?;
                } else {
                    self.set(3, 0)?;
                }
                self.instruction_pointer += 4;
            }
            8 => {
                if self.get(1)? == self.get(2)? {
                    self.set(3, 1)?;
                } else {
                    self.set(3, 0)?;
                }
                self.instruction_pointer += 4;
            }
            9 => {
                self.relative_base += self.get(1)?;
                self.instruction_pointer += 2;
            }
            99 => self.state = State::Halted,
            _ => self.state = State::Error,
        };
        Some(())
    }

    pub fn input(&mut self, value: i64) {
        if self.state == State::Input {
            if self.set(1, value).is_some() {
                self.instruction_pointer += 2;
                self.state = State::Running;
                self.run();
            } else {
                self.state = State::Error;
            }
        }
    }

    pub fn output(&mut self) -> Option<i64> {
        if self.state == State::Output {
            if let Some(result) = self.get(1) {
                self.instruction_pointer += 2;
                self.state = State::Running;
                self.run();
                return Some(result);
            } else {
                self.state = State::Error;
            }
        }
        None
    }

    fn get(&self, parameter: usize) -> Option<i64> {
        let value = self.get_direct(self.instruction_pointer + parameter);
        Some(match self.parameter_modes[parameter - 1] {
            ParameterMode::Position => {
                if value >= 0 {
                    self.get_direct(value as usize)
                } else {
                    return None;
                }
            }
            ParameterMode::Immediate => value,
            ParameterMode::Relative => {
                if value + self.relative_base >= 0 {
                    self.get_direct((value + self.relative_base) as usize)
                } else {
                    return None;
                }
            }
        })
    }

    fn set(&mut self, parameter: usize, value: i64) -> Option<()> {
        let address = self.get_direct(self.instruction_pointer + parameter)
            + match self.parameter_modes[parameter - 1] {
                ParameterMode::Position => 0,
                ParameterMode::Immediate => {
                    return None;
                }
                ParameterMode::Relative => self.relative_base,
            };
        if address < 0 {
            return None;
        }
        self.set_direct(address as usize, value);
        Some(())
    }

    pub fn get_direct(&self, address: usize) -> i64 {
        if address < self.memory.len() {
            self.memory[address]
        } else {
            0
        }
    }

    pub fn set_direct(&mut self, address: usize, value: i64) {
        while self.memory.len() <= address {
            self.memory.push(0);
        }
        self.memory[address] = value;
    }

    pub fn state(&self) -> State {
        self.state
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum State {
    Running,
    Input,
    Output,
    Halted,
    Error,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

impl ParameterMode {
    fn modes(values: i64) -> Option<[ParameterMode; 3]> {
        Some([
            ParameterMode::mode(values % 10)?,
            ParameterMode::mode(values / 10 % 10)?,
            ParameterMode::mode(values / 100 % 10)?,
        ])
    }

    fn mode(value: i64) -> Option<ParameterMode> {
        match value {
            0 => Some(ParameterMode::Position),
            1 => Some(ParameterMode::Immediate),
            2 => Some(ParameterMode::Relative),
            _ => None,
        }
    }
}
