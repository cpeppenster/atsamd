#[doc = r"Register block"]
#[repr(C)]
pub struct USART {
    #[doc = "0x00 - USART Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x04 - USART Control B"]
    pub ctrlb: CTRLB,
    #[doc = "0x08 - USART Debug Control"]
    pub dbgctrl: DBGCTRL,
    _reserved3: [u8; 0x01],
    #[doc = "0x0a - USART Baud"]
    pub baud: BAUD,
    #[doc = "0x0c - USART Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x0d - USART Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x0e - USART Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    _reserved7: [u8; 0x01],
    #[doc = "0x10 - USART Status"]
    pub status: STATUS,
    _reserved8: [u8; 0x06],
    #[doc = "0x18 - USART Data"]
    pub data: DATA,
}
#[doc = "CTRLA (rw) register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "USART Control A"]
pub mod ctrla;
#[doc = "CTRLB (rw) register accessor: an alias for `Reg<CTRLB_SPEC>`"]
pub type CTRLB = crate::Reg<ctrlb::CTRLB_SPEC>;
#[doc = "USART Control B"]
pub mod ctrlb;
#[doc = "DBGCTRL (rw) register accessor: an alias for `Reg<DBGCTRL_SPEC>`"]
pub type DBGCTRL = crate::Reg<dbgctrl::DBGCTRL_SPEC>;
#[doc = "USART Debug Control"]
pub mod dbgctrl;
#[doc = "BAUD (rw) register accessor: an alias for `Reg<BAUD_SPEC>`"]
pub type BAUD = crate::Reg<baud::BAUD_SPEC>;
#[doc = "USART Baud"]
pub mod baud;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "USART Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "USART Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: an alias for `Reg<INTFLAG_SPEC>`"]
pub type INTFLAG = crate::Reg<intflag::INTFLAG_SPEC>;
#[doc = "USART Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "USART Status"]
pub mod status;
#[doc = "DATA (rw) register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "USART Data"]
pub mod data;
