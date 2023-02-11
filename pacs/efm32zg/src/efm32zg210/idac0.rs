#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Current Programming Register"]
    pub curprog: CURPROG,
    #[doc = "0x08 - Calibration Register"]
    pub cal: CAL,
    #[doc = "0x0c - Duty Cycle Configauration Register"]
    pub dutyconfig: DUTYCONFIG,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "CURPROG (rw) register accessor: an alias for `Reg<CURPROG_SPEC>`"]
pub type CURPROG = crate::Reg<curprog::CURPROG_SPEC>;
#[doc = "Current Programming Register"]
pub mod curprog;
#[doc = "CAL (rw) register accessor: an alias for `Reg<CAL_SPEC>`"]
pub type CAL = crate::Reg<cal::CAL_SPEC>;
#[doc = "Calibration Register"]
pub mod cal;
#[doc = "DUTYCONFIG (rw) register accessor: an alias for `Reg<DUTYCONFIG_SPEC>`"]
pub type DUTYCONFIG = crate::Reg<dutyconfig::DUTYCONFIG_SPEC>;
#[doc = "Duty Cycle Configauration Register"]
pub mod dutyconfig;
