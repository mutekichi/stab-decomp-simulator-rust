pub mod cx;
pub mod cz;
pub mod h;
pub mod pauli_application;
pub mod s;
pub mod sqrt_x;
pub mod swap;
pub mod x;
pub mod y;
pub mod z;

pub use cx::CXGate;
pub use cz::CZGate;
pub use h::HGate;
pub use s::SGate;
pub use sqrt_x::SqrtXGate;
pub use swap::SwapGate;
pub use x::XGate;
pub use y::YGate;
pub use z::ZGate;

use crate::{
    StabilizerCHForm,
    api::representation::{CliffordCircuit, CliffordGate},
};

impl StabilizerCHForm {
    pub fn apply_gate(&mut self, gate: &CliffordGate) {
        match gate {
            CliffordGate::H(qarg) => self.apply_h(*qarg),
            CliffordGate::X(qarg) => self.apply_x(*qarg),
            CliffordGate::Y(qarg) => self.apply_y(*qarg),
            CliffordGate::Z(qarg) => self.apply_z(*qarg),
            CliffordGate::S(qarg) => self.apply_s(*qarg),
            CliffordGate::Sdg(qarg) => self.apply_sdg(*qarg),
            CliffordGate::SqrtX(qarg) => self.apply_sqrt_x(*qarg),
            CliffordGate::SqrtXdg(qarg) => self.apply_sqrt_xdg(*qarg),
            CliffordGate::CX(control, target) => self.apply_cx(*control, *target),
            CliffordGate::CZ(control, target) => self.apply_cz(*control, *target),
            CliffordGate::Swap(q1, q2) => self.apply_swap(*q1, *q2),
        }
    }

    pub fn apply_circuit(&mut self, circuit: &CliffordCircuit) {
        for gate in &circuit.gates {
            self.apply_gate(gate);
        }
    }
}
