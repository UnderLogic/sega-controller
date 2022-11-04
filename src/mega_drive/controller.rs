use crate::ControllerResult;
use crate::Error::NotPresent;
use core::cell::RefCell;
use core::convert::Infallible;
use embedded_hal::digital::v2::{InputPin, OutputPin, PinState};

pub enum MegaDriveButton {
    Up,
    Down,
    Left,
    Right,
    A,
    B,
    C,
    Start,
    X,
    Y,
    Z,
    Mode,
}

impl MegaDriveButton {
    fn as_flag(&self) -> u16 {
        match self {
            MegaDriveButton::Up => 0x1,
            MegaDriveButton::Down => 0x2,
            MegaDriveButton::Left => 0x4,
            MegaDriveButton::Right => 0x8,
            MegaDriveButton::B => 0x10,
            MegaDriveButton::C => 0x20,
            MegaDriveButton::A => 0x40,
            MegaDriveButton::Start => 0x80,
            MegaDriveButton::Z => 0x100,
            MegaDriveButton::Y => 0x200,
            MegaDriveButton::X => 0x400,
            MegaDriveButton::Mode => 0x800,
        }
    }
}

pub struct MegaDriveController<S, D0, D1, D2, D3, D4, D5>
where
    S: OutputPin<Error = Infallible>,
    D0: InputPin<Error = Infallible>,
    D1: InputPin<Error = Infallible>,
    D2: InputPin<Error = Infallible>,
    D3: InputPin<Error = Infallible>,
    D4: InputPin<Error = Infallible>,
    D5: InputPin<Error = Infallible>,
{
    select_pin: RefCell<S>,
    data_pin_0: D0,
    data_pin_1: D1,
    data_pin_2: D2,
    data_pin_3: D3,
    data_pin_4: D4,
    data_pin_5: D5,
}

impl<S, D0, D1, D2, D3, D4, D5> MegaDriveController<S, D0, D1, D2, D3, D4, D5>
where
    S: OutputPin<Error = Infallible>,
    D0: InputPin<Error = Infallible>,
    D1: InputPin<Error = Infallible>,
    D2: InputPin<Error = Infallible>,
    D3: InputPin<Error = Infallible>,
    D4: InputPin<Error = Infallible>,
    D5: InputPin<Error = Infallible>,
{
    pub fn from_pins(
        select_pin: S,
        pin0: D0,
        pin1: D1,
        pin2: D2,
        pin3: D3,
        pin4: D4,
        pin5: D5,
    ) -> Self {
        MegaDriveController {
            select_pin: RefCell::new(select_pin),
            data_pin_0: pin0,
            data_pin_1: pin1,
            data_pin_2: pin2,
            data_pin_3: pin3,
            data_pin_4: pin4,
            data_pin_5: pin5,
        }
    }

    pub fn read_state(&self) -> ControllerResult<MegaDriveControllerState, Infallible> {
        let mut bits: u16 = 0x0000;

        // First read cycle (select = high)
        // Will contain states for UP, DOWN, LEFT, RIGHT, B, and C buttons
        self.set_select_pin(PinState::High)?;
        if self.data_pin_0.is_low()? {
            let flag = MegaDriveButton::Up.as_flag();
            bits = bits | flag;
        }
        if self.data_pin_1.is_low()? {
            bits |= MegaDriveButton::Down.as_flag();
        }
        if self.data_pin_2.is_low()? {
            bits |= MegaDriveButton::Left.as_flag();
        }
        if self.data_pin_3.is_low()? {
            bits |= MegaDriveButton::Right.as_flag();
        }
        if self.data_pin_4.is_low()? {
            bits |= MegaDriveButton::B.as_flag();
        }
        if self.data_pin_5.is_low()? {
            bits |= MegaDriveButton::C.as_flag();
        }

        // Second read cycle (select = low)
        // Will contain states for A and START buttons, and detect if controller present
        self.set_select_pin(PinState::Low)?;

        // Bits 2 and 3 are held low when the controller is present
        if self.data_pin_2.is_high()? || self.data_pin_3.is_high()? {
            return Err(NotPresent);
        }

        if self.data_pin_4.is_low()? {
            bits |= MegaDriveButton::A.as_flag();
        }
        if self.data_pin_5.is_low()? {
            bits |= MegaDriveButton::Up.as_flag();
        }

        // Skip third, fourth, and fifth cycles
        self.set_select_pin(PinState::High)?;
        self.set_select_pin(PinState::Low)?;

        // Sixth read cycle check if six-button controller is available
        let is_six_button = self.data_pin_0.is_low()?
            && self.data_pin_1.is_low()?
            && self.data_pin_2.is_low()?
            && self.data_pin_3.is_low()?;

        // If six button is present, set select to high and read Z, Y, X, and MODE button states
        if is_six_button {
            self.set_select_pin(PinState::High)?;

            if self.data_pin_0.is_low()? {
                bits |= MegaDriveButton::Z.as_flag();
            }
            if self.data_pin_1.is_low()? {
                bits |= MegaDriveButton::Y.as_flag();
            }
            if self.data_pin_2.is_low()? {
                bits |= MegaDriveButton::X.as_flag();
            }
            if self.data_pin_3.is_low()? {
                bits |= MegaDriveButton::Mode.as_flag();
            }
        }

        let state = MegaDriveControllerState::from_bits(bits, is_six_button);
        Ok(state)
    }

    fn set_select_pin(&self, state: PinState) -> Result<(), Infallible> {
        self.select_pin.borrow_mut().set_state(state)
    }
}

pub struct MegaDriveControllerState {
    pub is_six_button: bool,
    bits: u16,
}

impl MegaDriveControllerState {
    fn from_bits(bits: u16, is_six_button: bool) -> Self {
        MegaDriveControllerState {
            is_six_button,
            bits,
        }
    }

    pub fn is_pressed(&self, button: MegaDriveButton) -> bool {
        &self.bits & button.as_flag() == button.as_flag()
    }
}
