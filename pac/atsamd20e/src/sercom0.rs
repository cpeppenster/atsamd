#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_spi: [u8; 0x1a],
}
impl RegisterBlock {
    #[doc = "0x00..0x1a - USART Mode"]
    #[inline(always)]
    pub fn usart(&self) -> &USART {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const USART) }
    }
    #[doc = "0x00..0x1a - SPI Mode"]
    #[inline(always)]
    pub fn spi(&self) -> &SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const SPI) }
    }
    #[doc = "0x00..0x19 - I2C Slave Mode"]
    #[inline(always)]
    pub fn i2cs(&self) -> &I2CS {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const I2CS) }
    }
    #[doc = "0x00..0x19 - I2C Master Mode"]
    #[inline(always)]
    pub fn i2cm(&self) -> &I2CM {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const I2CM) }
    }
}
#[doc = r"Register block"]
#[repr(C)]
pub struct I2CM {
    #[doc = "0x00 - I2CM Control A"]
    pub ctrla: crate::Reg<self::i2cm::ctrla::CTRLA_SPEC>,
    #[doc = "0x04 - I2CM Control B"]
    pub ctrlb: crate::Reg<self::i2cm::ctrlb::CTRLB_SPEC>,
    #[doc = "0x08 - I2CM Debug Control"]
    pub dbgctrl: crate::Reg<self::i2cm::dbgctrl::DBGCTRL_SPEC>,
    _reserved3: [u8; 0x01],
    #[doc = "0x0a - I2CM Baud Rate"]
    pub baud: crate::Reg<self::i2cm::baud::BAUD_SPEC>,
    #[doc = "0x0c - I2CM Interrupt Enable Clear"]
    pub intenclr: crate::Reg<self::i2cm::intenclr::INTENCLR_SPEC>,
    #[doc = "0x0d - I2CM Interrupt Enable Set"]
    pub intenset: crate::Reg<self::i2cm::intenset::INTENSET_SPEC>,
    #[doc = "0x0e - I2CM Interrupt Flag Status and Clear"]
    pub intflag: crate::Reg<self::i2cm::intflag::INTFLAG_SPEC>,
    _reserved7: [u8; 0x01],
    #[doc = "0x10 - I2CM Status"]
    pub status: crate::Reg<self::i2cm::status::STATUS_SPEC>,
    _reserved8: [u8; 0x02],
    #[doc = "0x14 - I2CM Address"]
    pub addr: crate::Reg<self::i2cm::addr::ADDR_SPEC>,
    _reserved9: [u8; 0x03],
    #[doc = "0x18 - I2CM Data"]
    pub data: crate::Reg<self::i2cm::data::DATA_SPEC>,
}
#[doc = r"Register block"]
#[doc = "I2C Master Mode"]
pub mod i2cm;
#[doc = r"Register block"]
#[repr(C)]
pub struct I2CS {
    #[doc = "0x00 - I2CS Control A"]
    pub ctrla: crate::Reg<self::i2cs::ctrla::CTRLA_SPEC>,
    #[doc = "0x04 - I2CS Control B"]
    pub ctrlb: crate::Reg<self::i2cs::ctrlb::CTRLB_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - I2CS Interrupt Enable Clear"]
    pub intenclr: crate::Reg<self::i2cs::intenclr::INTENCLR_SPEC>,
    #[doc = "0x0d - I2CS Interrupt Enable Set"]
    pub intenset: crate::Reg<self::i2cs::intenset::INTENSET_SPEC>,
    #[doc = "0x0e - I2CS Interrupt Flag Status and Clear"]
    pub intflag: crate::Reg<self::i2cs::intflag::INTFLAG_SPEC>,
    _reserved5: [u8; 0x01],
    #[doc = "0x10 - I2CS Status"]
    pub status: crate::Reg<self::i2cs::status::STATUS_SPEC>,
    _reserved6: [u8; 0x02],
    #[doc = "0x14 - I2CS Address"]
    pub addr: crate::Reg<self::i2cs::addr::ADDR_SPEC>,
    #[doc = "0x18 - I2CS Data"]
    pub data: crate::Reg<self::i2cs::data::DATA_SPEC>,
}
#[doc = r"Register block"]
#[doc = "I2C Slave Mode"]
pub mod i2cs;
#[doc = r"Register block"]
#[repr(C)]
pub struct SPI {
    #[doc = "0x00 - SPI Control A"]
    pub ctrla: crate::Reg<self::spi::ctrla::CTRLA_SPEC>,
    #[doc = "0x04 - SPI Control B"]
    pub ctrlb: crate::Reg<self::spi::ctrlb::CTRLB_SPEC>,
    #[doc = "0x08 - SPI Debug Control"]
    pub dbgctrl: crate::Reg<self::spi::dbgctrl::DBGCTRL_SPEC>,
    _reserved3: [u8; 0x01],
    #[doc = "0x0a - SPI Baud Rate"]
    pub baud: crate::Reg<self::spi::baud::BAUD_SPEC>,
    _reserved4: [u8; 0x01],
    #[doc = "0x0c - SPI Interrupt Enable Clear"]
    pub intenclr: crate::Reg<self::spi::intenclr::INTENCLR_SPEC>,
    #[doc = "0x0d - SPI Interrupt Enable Set"]
    pub intenset: crate::Reg<self::spi::intenset::INTENSET_SPEC>,
    #[doc = "0x0e - SPI Interrupt Flag Status and Clear"]
    pub intflag: crate::Reg<self::spi::intflag::INTFLAG_SPEC>,
    _reserved7: [u8; 0x01],
    #[doc = "0x10 - SPI Status"]
    pub status: crate::Reg<self::spi::status::STATUS_SPEC>,
    _reserved8: [u8; 0x02],
    #[doc = "0x14 - SPI Address"]
    pub addr: crate::Reg<self::spi::addr::ADDR_SPEC>,
    #[doc = "0x18 - SPI Data"]
    pub data: crate::Reg<self::spi::data::DATA_SPEC>,
}
#[doc = r"Register block"]
#[doc = "SPI Mode"]
pub mod spi;
#[doc = r"Register block"]
#[repr(C)]
pub struct USART {
    #[doc = "0x00 - USART Control A"]
    pub ctrla: crate::Reg<self::usart::ctrla::CTRLA_SPEC>,
    #[doc = "0x04 - USART Control B"]
    pub ctrlb: crate::Reg<self::usart::ctrlb::CTRLB_SPEC>,
    #[doc = "0x08 - USART Debug Control"]
    pub dbgctrl: crate::Reg<self::usart::dbgctrl::DBGCTRL_SPEC>,
    _reserved3: [u8; 0x01],
    #[doc = "0x0a - USART Baud"]
    pub baud: crate::Reg<self::usart::baud::BAUD_SPEC>,
    #[doc = "0x0c - USART Interrupt Enable Clear"]
    pub intenclr: crate::Reg<self::usart::intenclr::INTENCLR_SPEC>,
    #[doc = "0x0d - USART Interrupt Enable Set"]
    pub intenset: crate::Reg<self::usart::intenset::INTENSET_SPEC>,
    #[doc = "0x0e - USART Interrupt Flag Status and Clear"]
    pub intflag: crate::Reg<self::usart::intflag::INTFLAG_SPEC>,
    _reserved7: [u8; 0x01],
    #[doc = "0x10 - USART Status"]
    pub status: crate::Reg<self::usart::status::STATUS_SPEC>,
    _reserved8: [u8; 0x06],
    #[doc = "0x18 - USART Data"]
    pub data: crate::Reg<self::usart::data::DATA_SPEC>,
}
#[doc = r"Register block"]
#[doc = "USART Mode"]
pub mod usart;
