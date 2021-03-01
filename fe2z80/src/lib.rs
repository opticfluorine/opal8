//! # fe2z80 - High Performance Z80 Emulator
//!
//! fe2z80 is a Z80 emulator implemented in Rust prioritizing
//! performance over emulation accuracy. It is WebAssembly
//! aware and suitable for integration with a browser-based
//! application.

use array_init;

/// Describes the current state of a bistate pin. The details
/// of "high" vs "low" are abstracted away here in favor of
/// "active" and "inactive", allowing the emulator and users
/// to avoid worrying about whether a given pin is active-high
/// or active-low.
pub enum PinBistate {
    Active,
    Inactive,
}

/// Describes the current state of a tristate pin. The details
/// of "high" vs "low" are abstracted away here in favor of
/// "active" and "inactive", allowing the emulator and users
/// to avoid worrying about whether a given pin is active-high
/// or active-low.
pub enum PinTristate {
    Active,
    Inactive,
    Other,
}

/// Data type that describes the current state of the
/// input signals. The clock pin is omitted since it is
/// replaced by the `Emulator::on_clock` method instead.
pub struct InputPins {
    pub busreq: PinBistate,
    pub int: PinBistate,
    pub nmi: PinBistate,
    pub reset: PinBistate,
    pub wait: PinBistate,
}

/// Data type that describes the current state of the
/// output pins.
pub struct OutputPins {
    pub address_bus: u16,
    pub busack: PinBistate,
    pub halt: PinBistate,
    pub iorq: PinTristate,
    pub m1: PinBistate,
    pub mreq: PinTristate,
    pub read: PinTristate,
    pub rfsh: PinBistate,
    pub wr: PinBistate,
}

/// Data type that describes the current state of the
/// input/output pins.
pub struct InputOutputPins {
    pub data_bus: u8,
}

/// Split representation of a 16-bit register.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Register16HiLo {
    pub hi: u8,
    pub lo: u8,
}

/// Union type modeling a register pair that can be accessed
/// as a single 16-bit register or two 8-bit registers.
#[repr(C)]
pub union Register16 {
    pub hilo: Register16HiLo,
    pub full: u16,
}

/// Data type that describes the current state of all 22
/// registers.
pub struct Registers {
    pub a0: u8,
    pub f0: u8,
    pub a1: u8,
    pub f1: u8,
    pub i: u8,
    pub r: u8,
    pub ix: u16,
    pub iy: u16,
    pub sp: u16,
    pub pc: u16,
    pub bc0: Register16,
    pub de0: Register16,
    pub hl0: Register16,
    pub bc1: Register16,
    pub de1: Register16,
    pub hl1: Register16,
}

mod mmu;

/// Each instance of Emulator provides a single instance of
/// the fe2z80 Z80 emulator.
pub struct Emulator<'a> {
    memory_pages: mmu::PageArray<'a>,
}

impl Emulator<'_> {
    /// Causes the emulator to evaluate its next clock cycle,
    /// blocking until the cycle is complete.
    ///
    /// # Arguments
    /// * `input_pins` - (In) Current input pin state.
    /// * `output_pins` - (Out) Current output pin state.
    /// * `input_output_pins` - (In/Out) Current in/out pin state.
    /// * `registers` - (In/Out) Current registers state.
    pub fn on_clock(
        input_pins: &InputPins,
        output_pins: &mut OutputPins,
        input_output_pins: &mut InputOutputPins,
        registers: &mut Registers,
    ) {
    }
}

/// Creates and returns a new emulator instance.
pub fn create_emulator<'a>() -> Emulator<'a> {
    Emulator {
        memory_pages: array_init::array_init(|_| None),
    }
}
