#[doc = r"Register block"]
#[repr(C)]
pub struct I2CM {
    #[doc = "0x00 - I2CM Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x04 - I2CM Control B"]
    pub ctrlb: CTRLB,
    #[doc = "0x08 - I2CM Debug Control"]
    pub dbgctrl: DBGCTRL,
    _reserved3: [u8; 0x01],
    #[doc = "0x0a - I2CM Baud Rate"]
    pub baud: BAUD,
    #[doc = "0x0c - I2CM Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x0d - I2CM Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x0e - I2CM Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    _reserved7: [u8; 0x01],
    #[doc = "0x10 - I2CM Status"]
    pub status: STATUS,
    _reserved8: [u8; 0x02],
    #[doc = "0x14 - I2CM Address"]
    pub addr: ADDR,
    _reserved9: [u8; 0x03],
    #[doc = "0x18 - I2CM Data"]
    pub data: DATA,
}
#[doc = "CTRLA (rw) register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "I2CM Control A"]
pub mod ctrla;
#[doc = "CTRLB (rw) register accessor: an alias for `Reg<CTRLB_SPEC>`"]
pub type CTRLB = crate::Reg<ctrlb::CTRLB_SPEC>;
#[doc = "I2CM Control B"]
pub mod ctrlb;
#[doc = "DBGCTRL (rw) register accessor: an alias for `Reg<DBGCTRL_SPEC>`"]
pub type DBGCTRL = crate::Reg<dbgctrl::DBGCTRL_SPEC>;
#[doc = "I2CM Debug Control"]
pub mod dbgctrl;
#[doc = "BAUD (rw) register accessor: an alias for `Reg<BAUD_SPEC>`"]
pub type BAUD = crate::Reg<baud::BAUD_SPEC>;
#[doc = "I2CM Baud Rate"]
pub mod baud;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "I2CM Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "I2CM Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: an alias for `Reg<INTFLAG_SPEC>`"]
pub type INTFLAG = crate::Reg<intflag::INTFLAG_SPEC>;
#[doc = "I2CM Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "I2CM Status"]
pub mod status;
#[doc = "ADDR (rw) register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "I2CM Address"]
pub mod addr;
#[doc = "DATA (rw) register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "I2CM Data"]
pub mod data;
