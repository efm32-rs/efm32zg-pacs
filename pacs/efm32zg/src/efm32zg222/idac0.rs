#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - Current Programming Register"]
    pub curprog: crate::Reg<curprog::CURPROG_SPEC>,
    #[doc = "0x08 - Calibration Register"]
    pub cal: crate::Reg<cal::CAL_SPEC>,
    #[doc = "0x0c - Duty Cycle Configauration Register"]
    pub dutyconfig: crate::Reg<dutyconfig::DUTYCONFIG_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "CURPROG register accessor: an alias for `Reg<CURPROG_SPEC>`"]
pub type CURPROG = crate::Reg<curprog::CURPROG_SPEC>;
#[doc = "Current Programming Register"]
pub mod curprog;
#[doc = "CAL register accessor: an alias for `Reg<CAL_SPEC>`"]
pub type CAL = crate::Reg<cal::CAL_SPEC>;
#[doc = "Calibration Register"]
pub mod cal;
#[doc = "DUTYCONFIG register accessor: an alias for `Reg<DUTYCONFIG_SPEC>`"]
pub type DUTYCONFIG = crate::Reg<dutyconfig::DUTYCONFIG_SPEC>;
#[doc = "Duty Cycle Configauration Register"]
pub mod dutyconfig;
