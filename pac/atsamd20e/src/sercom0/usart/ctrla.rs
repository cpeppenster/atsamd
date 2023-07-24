#[doc = "Register `CTRLA` reader"]
pub struct R(crate::R<CTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLA` writer"]
pub struct W(crate::W<CTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWRST` reader - Software Reset"]
pub type SWRST_R = crate::BitReader;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SWRST_W<'a, const O: u8> = crate::BitWriter<'a, CTRLA_SPEC, O>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, CTRLA_SPEC, O>;
#[doc = "Field `MODE` reader - Operating Mode"]
pub type MODE_R = crate::FieldReader<MODESELECT_A>;
#[doc = "Operating Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODESELECT_A {
    #[doc = "0: USART mode with external clock"]
    USART_EXT_CLK = 0,
    #[doc = "1: USART mode with internal clock"]
    USART_INT_CLK = 1,
    #[doc = "2: SPI mode with external clock"]
    SPI_SLAVE = 2,
    #[doc = "3: SPI mode with internal clock"]
    SPI_MASTER = 3,
    #[doc = "4: I2C mode with external clock"]
    I2C_SLAVE = 4,
    #[doc = "5: I2C mode with internal clock"]
    I2C_MASTER = 5,
}
impl From<MODESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: MODESELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODESELECT_A {
    type Ux = u8;
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODESELECT_A> {
        match self.bits {
            0 => Some(MODESELECT_A::USART_EXT_CLK),
            1 => Some(MODESELECT_A::USART_INT_CLK),
            2 => Some(MODESELECT_A::SPI_SLAVE),
            3 => Some(MODESELECT_A::SPI_MASTER),
            4 => Some(MODESELECT_A::I2C_SLAVE),
            5 => Some(MODESELECT_A::I2C_MASTER),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `USART_EXT_CLK`"]
    #[inline(always)]
    pub fn is_usart_ext_clk(&self) -> bool {
        *self == MODESELECT_A::USART_EXT_CLK
    }
    #[doc = "Checks if the value of the field is `USART_INT_CLK`"]
    #[inline(always)]
    pub fn is_usart_int_clk(&self) -> bool {
        *self == MODESELECT_A::USART_INT_CLK
    }
    #[doc = "Checks if the value of the field is `SPI_SLAVE`"]
    #[inline(always)]
    pub fn is_spi_slave(&self) -> bool {
        *self == MODESELECT_A::SPI_SLAVE
    }
    #[doc = "Checks if the value of the field is `SPI_MASTER`"]
    #[inline(always)]
    pub fn is_spi_master(&self) -> bool {
        *self == MODESELECT_A::SPI_MASTER
    }
    #[doc = "Checks if the value of the field is `I2C_SLAVE`"]
    #[inline(always)]
    pub fn is_i2c_slave(&self) -> bool {
        *self == MODESELECT_A::I2C_SLAVE
    }
    #[doc = "Checks if the value of the field is `I2C_MASTER`"]
    #[inline(always)]
    pub fn is_i2c_master(&self) -> bool {
        *self == MODESELECT_A::I2C_MASTER
    }
}
#[doc = "Field `MODE` writer - Operating Mode"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, CTRLA_SPEC, 3, O, MODESELECT_A>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "USART mode with external clock"]
    #[inline(always)]
    pub fn usart_ext_clk(self) -> &'a mut W {
        self.variant(MODESELECT_A::USART_EXT_CLK)
    }
    #[doc = "USART mode with internal clock"]
    #[inline(always)]
    pub fn usart_int_clk(self) -> &'a mut W {
        self.variant(MODESELECT_A::USART_INT_CLK)
    }
    #[doc = "SPI mode with external clock"]
    #[inline(always)]
    pub fn spi_slave(self) -> &'a mut W {
        self.variant(MODESELECT_A::SPI_SLAVE)
    }
    #[doc = "SPI mode with internal clock"]
    #[inline(always)]
    pub fn spi_master(self) -> &'a mut W {
        self.variant(MODESELECT_A::SPI_MASTER)
    }
    #[doc = "I2C mode with external clock"]
    #[inline(always)]
    pub fn i2c_slave(self) -> &'a mut W {
        self.variant(MODESELECT_A::I2C_SLAVE)
    }
    #[doc = "I2C mode with internal clock"]
    #[inline(always)]
    pub fn i2c_master(self) -> &'a mut W {
        self.variant(MODESELECT_A::I2C_MASTER)
    }
}
#[doc = "Field `RUNSTDBY` reader - Run In Standby"]
pub type RUNSTDBY_R = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run In Standby"]
pub type RUNSTDBY_W<'a, const O: u8> = crate::BitWriter<'a, CTRLA_SPEC, O>;
#[doc = "Field `IBON` reader - Immediate Buffer Overflow Notification"]
pub type IBON_R = crate::BitReader;
#[doc = "Field `IBON` writer - Immediate Buffer Overflow Notification"]
pub type IBON_W<'a, const O: u8> = crate::BitWriter<'a, CTRLA_SPEC, O>;
#[doc = "Field `TXPO` reader - Transmit Data Pinout"]
pub type TXPO_R = crate::BitReader<TXPOSELECT_A>;
#[doc = "Transmit Data Pinout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXPOSELECT_A {
    #[doc = "0: TXD at PAD0, XCK at PAD1"]
    PAD0 = 0,
    #[doc = "1: TXD at PAD2, XCK at PAD3"]
    PAD2 = 1,
}
impl From<TXPOSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: TXPOSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl TXPO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXPOSELECT_A {
        match self.bits {
            false => TXPOSELECT_A::PAD0,
            true => TXPOSELECT_A::PAD2,
        }
    }
    #[doc = "Checks if the value of the field is `PAD0`"]
    #[inline(always)]
    pub fn is_pad0(&self) -> bool {
        *self == TXPOSELECT_A::PAD0
    }
    #[doc = "Checks if the value of the field is `PAD2`"]
    #[inline(always)]
    pub fn is_pad2(&self) -> bool {
        *self == TXPOSELECT_A::PAD2
    }
}
#[doc = "Field `TXPO` writer - Transmit Data Pinout"]
pub type TXPO_W<'a, const O: u8> = crate::BitWriter<'a, CTRLA_SPEC, O, TXPOSELECT_A>;
impl<'a, const O: u8> TXPO_W<'a, O> {
    #[doc = "TXD at PAD0, XCK at PAD1"]
    #[inline(always)]
    pub fn pad0(self) -> &'a mut W {
        self.variant(TXPOSELECT_A::PAD0)
    }
    #[doc = "TXD at PAD2, XCK at PAD3"]
    #[inline(always)]
    pub fn pad2(self) -> &'a mut W {
        self.variant(TXPOSELECT_A::PAD2)
    }
}
#[doc = "Field `RXPO` reader - Receive Data Pinout"]
pub type RXPO_R = crate::FieldReader<RXPOSELECT_A>;
#[doc = "Receive Data Pinout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXPOSELECT_A {
    #[doc = "0: SERCOM_PAD0"]
    PAD0 = 0,
    #[doc = "1: SERCOM_PAD1"]
    PAD1 = 1,
    #[doc = "2: SERCOM_PAD2"]
    PAD2 = 2,
    #[doc = "3: SERCOM_PAD3"]
    PAD3 = 3,
}
impl From<RXPOSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: RXPOSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RXPOSELECT_A {
    type Ux = u8;
}
impl RXPO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXPOSELECT_A {
        match self.bits {
            0 => RXPOSELECT_A::PAD0,
            1 => RXPOSELECT_A::PAD1,
            2 => RXPOSELECT_A::PAD2,
            3 => RXPOSELECT_A::PAD3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PAD0`"]
    #[inline(always)]
    pub fn is_pad0(&self) -> bool {
        *self == RXPOSELECT_A::PAD0
    }
    #[doc = "Checks if the value of the field is `PAD1`"]
    #[inline(always)]
    pub fn is_pad1(&self) -> bool {
        *self == RXPOSELECT_A::PAD1
    }
    #[doc = "Checks if the value of the field is `PAD2`"]
    #[inline(always)]
    pub fn is_pad2(&self) -> bool {
        *self == RXPOSELECT_A::PAD2
    }
    #[doc = "Checks if the value of the field is `PAD3`"]
    #[inline(always)]
    pub fn is_pad3(&self) -> bool {
        *self == RXPOSELECT_A::PAD3
    }
}
#[doc = "Field `RXPO` writer - Receive Data Pinout"]
pub type RXPO_W<'a, const O: u8> = crate::FieldWriterSafe<'a, CTRLA_SPEC, 2, O, RXPOSELECT_A>;
impl<'a, const O: u8> RXPO_W<'a, O> {
    #[doc = "SERCOM_PAD0"]
    #[inline(always)]
    pub fn pad0(self) -> &'a mut W {
        self.variant(RXPOSELECT_A::PAD0)
    }
    #[doc = "SERCOM_PAD1"]
    #[inline(always)]
    pub fn pad1(self) -> &'a mut W {
        self.variant(RXPOSELECT_A::PAD1)
    }
    #[doc = "SERCOM_PAD2"]
    #[inline(always)]
    pub fn pad2(self) -> &'a mut W {
        self.variant(RXPOSELECT_A::PAD2)
    }
    #[doc = "SERCOM_PAD3"]
    #[inline(always)]
    pub fn pad3(self) -> &'a mut W {
        self.variant(RXPOSELECT_A::PAD3)
    }
}
#[doc = "Field `FORM` reader - Frame Format"]
pub type FORM_R = crate::FieldReader<FORMSELECT_A>;
#[doc = "Frame Format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FORMSELECT_A {
    #[doc = "0: USART frame"]
    _0 = 0,
    #[doc = "1: USART frame with parity"]
    _1 = 1,
}
impl From<FORMSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: FORMSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FORMSELECT_A {
    type Ux = u8;
}
impl FORM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FORMSELECT_A> {
        match self.bits {
            0 => Some(FORMSELECT_A::_0),
            1 => Some(FORMSELECT_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FORMSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FORMSELECT_A::_1
    }
}
#[doc = "Field `FORM` writer - Frame Format"]
pub type FORM_W<'a, const O: u8> = crate::FieldWriter<'a, CTRLA_SPEC, 4, O, FORMSELECT_A>;
impl<'a, const O: u8> FORM_W<'a, O> {
    #[doc = "USART frame"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FORMSELECT_A::_0)
    }
    #[doc = "USART frame with parity"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FORMSELECT_A::_1)
    }
}
#[doc = "Field `CMODE` reader - Communication Mode"]
pub type CMODE_R = crate::BitReader;
#[doc = "Field `CMODE` writer - Communication Mode"]
pub type CMODE_W<'a, const O: u8> = crate::BitWriter<'a, CTRLA_SPEC, O>;
#[doc = "Field `CPOL` reader - Clock Polarity"]
pub type CPOL_R = crate::BitReader;
#[doc = "Field `CPOL` writer - Clock Polarity"]
pub type CPOL_W<'a, const O: u8> = crate::BitWriter<'a, CTRLA_SPEC, O>;
#[doc = "Field `DORD` reader - Data Order"]
pub type DORD_R = crate::BitReader;
#[doc = "Field `DORD` writer - Data Order"]
pub type DORD_W<'a, const O: u8> = crate::BitWriter<'a, CTRLA_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Operating Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 7 - Run In Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Immediate Buffer Overflow Notification"]
    #[inline(always)]
    pub fn ibon(&self) -> IBON_R {
        IBON_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Transmit Data Pinout"]
    #[inline(always)]
    pub fn txpo(&self) -> TXPO_R {
        TXPO_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Receive Data Pinout"]
    #[inline(always)]
    pub fn rxpo(&self) -> RXPO_R {
        RXPO_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:27 - Frame Format"]
    #[inline(always)]
    pub fn form(&self) -> FORM_R {
        FORM_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Communication Mode"]
    #[inline(always)]
    pub fn cmode(&self) -> CMODE_R {
        CMODE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Data Order"]
    #[inline(always)]
    pub fn dord(&self) -> DORD_R {
        DORD_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SWRST_W<0> {
        SWRST_W::new(self)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<1> {
        ENABLE_W::new(self)
    }
    #[doc = "Bits 2:4 - Operating Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<2> {
        MODE_W::new(self)
    }
    #[doc = "Bit 7 - Run In Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<7> {
        RUNSTDBY_W::new(self)
    }
    #[doc = "Bit 8 - Immediate Buffer Overflow Notification"]
    #[inline(always)]
    #[must_use]
    pub fn ibon(&mut self) -> IBON_W<8> {
        IBON_W::new(self)
    }
    #[doc = "Bit 16 - Transmit Data Pinout"]
    #[inline(always)]
    #[must_use]
    pub fn txpo(&mut self) -> TXPO_W<16> {
        TXPO_W::new(self)
    }
    #[doc = "Bits 20:21 - Receive Data Pinout"]
    #[inline(always)]
    #[must_use]
    pub fn rxpo(&mut self) -> RXPO_W<20> {
        RXPO_W::new(self)
    }
    #[doc = "Bits 24:27 - Frame Format"]
    #[inline(always)]
    #[must_use]
    pub fn form(&mut self) -> FORM_W<24> {
        FORM_W::new(self)
    }
    #[doc = "Bit 28 - Communication Mode"]
    #[inline(always)]
    #[must_use]
    pub fn cmode(&mut self) -> CMODE_W<28> {
        CMODE_W::new(self)
    }
    #[doc = "Bit 29 - Clock Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CPOL_W<29> {
        CPOL_W::new(self)
    }
    #[doc = "Bit 30 - Data Order"]
    #[inline(always)]
    #[must_use]
    pub fn dord(&mut self) -> DORD_W<30> {
        DORD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrla](index.html) module"]
pub struct CTRLA_SPEC;
impl crate::RegisterSpec for CTRLA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrla::R](R) reader structure"]
impl crate::Readable for CTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrla::W](W) writer structure"]
impl crate::Writable for CTRLA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
