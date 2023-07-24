#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x04 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x08 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x0c - Power and Clocks Status"]
    pub pclksr: PCLKSR,
    #[doc = "0x10 - XOSC Control"]
    pub xosc: XOSC,
    _reserved5: [u8; 0x02],
    #[doc = "0x14 - XOSC32K Control"]
    pub xosc32k: XOSC32K,
    _reserved6: [u8; 0x02],
    #[doc = "0x18 - OSC32K Control"]
    pub osc32k: OSC32K,
    #[doc = "0x1c - OSCULP32K Control"]
    pub osculp32k: OSCULP32K,
    _reserved8: [u8; 0x03],
    #[doc = "0x20 - OSC8M Control A"]
    pub osc8m: OSC8M,
    #[doc = "0x24 - DFLL Config"]
    pub dfllctrl: DFLLCTRL,
    _reserved10: [u8; 0x02],
    #[doc = "0x28 - DFLL Calibration Value"]
    pub dfllval: DFLLVAL,
    #[doc = "0x2c - DFLL Multiplier"]
    pub dfllmul: DFLLMUL,
    #[doc = "0x30 - DFLL Synchronization"]
    pub dfllsync: DFLLSYNC,
    _reserved13: [u8; 0x03],
    #[doc = "0x34 - 3.3V Brown-Out Detector (BOD33) Control"]
    pub bod33: BOD33,
    _reserved14: [u8; 0x04],
    #[doc = "0x3c - VREG Control"]
    pub vreg: VREG,
    _reserved15: [u8; 0x02],
    #[doc = "0x40 - VREF Control A"]
    pub vref: VREF,
}
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: an alias for `Reg<INTFLAG_SPEC>`"]
pub type INTFLAG = crate::Reg<intflag::INTFLAG_SPEC>;
#[doc = "Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "PCLKSR (r) register accessor: an alias for `Reg<PCLKSR_SPEC>`"]
pub type PCLKSR = crate::Reg<pclksr::PCLKSR_SPEC>;
#[doc = "Power and Clocks Status"]
pub mod pclksr;
#[doc = "XOSC (rw) register accessor: an alias for `Reg<XOSC_SPEC>`"]
pub type XOSC = crate::Reg<xosc::XOSC_SPEC>;
#[doc = "XOSC Control"]
pub mod xosc;
#[doc = "XOSC32K (rw) register accessor: an alias for `Reg<XOSC32K_SPEC>`"]
pub type XOSC32K = crate::Reg<xosc32k::XOSC32K_SPEC>;
#[doc = "XOSC32K Control"]
pub mod xosc32k;
#[doc = "OSC32K (rw) register accessor: an alias for `Reg<OSC32K_SPEC>`"]
pub type OSC32K = crate::Reg<osc32k::OSC32K_SPEC>;
#[doc = "OSC32K Control"]
pub mod osc32k;
#[doc = "OSCULP32K (rw) register accessor: an alias for `Reg<OSCULP32K_SPEC>`"]
pub type OSCULP32K = crate::Reg<osculp32k::OSCULP32K_SPEC>;
#[doc = "OSCULP32K Control"]
pub mod osculp32k;
#[doc = "OSC8M (rw) register accessor: an alias for `Reg<OSC8M_SPEC>`"]
pub type OSC8M = crate::Reg<osc8m::OSC8M_SPEC>;
#[doc = "OSC8M Control A"]
pub mod osc8m;
#[doc = "DFLLCTRL (rw) register accessor: an alias for `Reg<DFLLCTRL_SPEC>`"]
pub type DFLLCTRL = crate::Reg<dfllctrl::DFLLCTRL_SPEC>;
#[doc = "DFLL Config"]
pub mod dfllctrl;
#[doc = "DFLLVAL (rw) register accessor: an alias for `Reg<DFLLVAL_SPEC>`"]
pub type DFLLVAL = crate::Reg<dfllval::DFLLVAL_SPEC>;
#[doc = "DFLL Calibration Value"]
pub mod dfllval;
#[doc = "DFLLMUL (rw) register accessor: an alias for `Reg<DFLLMUL_SPEC>`"]
pub type DFLLMUL = crate::Reg<dfllmul::DFLLMUL_SPEC>;
#[doc = "DFLL Multiplier"]
pub mod dfllmul;
#[doc = "DFLLSYNC (rw) register accessor: an alias for `Reg<DFLLSYNC_SPEC>`"]
pub type DFLLSYNC = crate::Reg<dfllsync::DFLLSYNC_SPEC>;
#[doc = "DFLL Synchronization"]
pub mod dfllsync;
#[doc = "BOD33 (rw) register accessor: an alias for `Reg<BOD33_SPEC>`"]
pub type BOD33 = crate::Reg<bod33::BOD33_SPEC>;
#[doc = "3.3V Brown-Out Detector (BOD33) Control"]
pub mod bod33;
#[doc = "VREG (rw) register accessor: an alias for `Reg<VREG_SPEC>`"]
pub type VREG = crate::Reg<vreg::VREG_SPEC>;
#[doc = "VREG Control"]
pub mod vreg;
#[doc = "VREF (rw) register accessor: an alias for `Reg<VREF_SPEC>`"]
pub type VREF = crate::Reg<vref::VREF_SPEC>;
#[doc = "VREF Control A"]
pub mod vref;
