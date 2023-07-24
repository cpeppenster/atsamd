#[doc = "Register `CTRLB` reader"]
pub struct R(crate::R<CTRLB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLB` writer"]
pub struct W(crate::W<CTRLB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLB_SPEC>;
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
impl From<crate::W<CTRLB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHSIZE` reader - Character Size"]
pub type CHSIZE_R = crate::FieldReader;
#[doc = "Field `CHSIZE` writer - Character Size"]
pub type CHSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, CTRLB_SPEC, 3, O>;
#[doc = "Field `PLOADEN` reader - Slave Data Preload Enable"]
pub type PLOADEN_R = crate::BitReader;
#[doc = "Field `PLOADEN` writer - Slave Data Preload Enable"]
pub type PLOADEN_W<'a, const O: u8> = crate::BitWriter<'a, CTRLB_SPEC, O>;
#[doc = "Field `AMODE` reader - Address Mode"]
pub type AMODE_R = crate::FieldReader<AMODESELECT_A>;
#[doc = "Address Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AMODESELECT_A {
    #[doc = "0: ADDRMASK is used as a mask to the ADDR register."]
    MASK = 0,
    #[doc = "1: The slave responds to the 2 unique addresses in ADDR and ADDRMASK."]
    _2ADDR = 1,
    #[doc = "2: The slave responds to the range of addresses between and including ADDR and ADDRMASK. ADDR is the upper limit."]
    RANGE = 2,
}
impl From<AMODESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: AMODESELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AMODESELECT_A {
    type Ux = u8;
}
impl AMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AMODESELECT_A> {
        match self.bits {
            0 => Some(AMODESELECT_A::MASK),
            1 => Some(AMODESELECT_A::_2ADDR),
            2 => Some(AMODESELECT_A::RANGE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == AMODESELECT_A::MASK
    }
    #[doc = "Checks if the value of the field is `_2ADDR`"]
    #[inline(always)]
    pub fn is_2addr(&self) -> bool {
        *self == AMODESELECT_A::_2ADDR
    }
    #[doc = "Checks if the value of the field is `RANGE`"]
    #[inline(always)]
    pub fn is_range(&self) -> bool {
        *self == AMODESELECT_A::RANGE
    }
}
#[doc = "Field `AMODE` writer - Address Mode"]
pub type AMODE_W<'a, const O: u8> = crate::FieldWriter<'a, CTRLB_SPEC, 2, O, AMODESELECT_A>;
impl<'a, const O: u8> AMODE_W<'a, O> {
    #[doc = "ADDRMASK is used as a mask to the ADDR register."]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(AMODESELECT_A::MASK)
    }
    #[doc = "The slave responds to the 2 unique addresses in ADDR and ADDRMASK."]
    #[inline(always)]
    pub fn _2addr(self) -> &'a mut W {
        self.variant(AMODESELECT_A::_2ADDR)
    }
    #[doc = "The slave responds to the range of addresses between and including ADDR and ADDRMASK. ADDR is the upper limit."]
    #[inline(always)]
    pub fn range(self) -> &'a mut W {
        self.variant(AMODESELECT_A::RANGE)
    }
}
#[doc = "Field `RXEN` reader - Receiver Enable"]
pub type RXEN_R = crate::BitReader;
#[doc = "Field `RXEN` writer - Receiver Enable"]
pub type RXEN_W<'a, const O: u8> = crate::BitWriter<'a, CTRLB_SPEC, O>;
impl R {
    #[doc = "Bits 0:2 - Character Size"]
    #[inline(always)]
    pub fn chsize(&self) -> CHSIZE_R {
        CHSIZE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 6 - Slave Data Preload Enable"]
    #[inline(always)]
    pub fn ploaden(&self) -> PLOADEN_R {
        PLOADEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Address Mode"]
    #[inline(always)]
    pub fn amode(&self) -> AMODE_R {
        AMODE_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 17 - Receiver Enable"]
    #[inline(always)]
    pub fn rxen(&self) -> RXEN_R {
        RXEN_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Character Size"]
    #[inline(always)]
    #[must_use]
    pub fn chsize(&mut self) -> CHSIZE_W<0> {
        CHSIZE_W::new(self)
    }
    #[doc = "Bit 6 - Slave Data Preload Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ploaden(&mut self) -> PLOADEN_W<6> {
        PLOADEN_W::new(self)
    }
    #[doc = "Bits 14:15 - Address Mode"]
    #[inline(always)]
    #[must_use]
    pub fn amode(&mut self) -> AMODE_W<14> {
        AMODE_W::new(self)
    }
    #[doc = "Bit 17 - Receiver Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxen(&mut self) -> RXEN_W<17> {
        RXEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlb](index.html) module"]
pub struct CTRLB_SPEC;
impl crate::RegisterSpec for CTRLB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrlb::R](R) reader structure"]
impl crate::Readable for CTRLB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrlb::W](W) writer structure"]
impl crate::Writable for CTRLB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
