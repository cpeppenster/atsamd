#[doc = "Register `BOD33` reader"]
pub struct R(crate::R<BOD33_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BOD33_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BOD33_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BOD33_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BOD33` writer"]
pub struct W(crate::W<BOD33_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BOD33_SPEC>;
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
impl From<crate::W<BOD33_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BOD33_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Enable"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, BOD33_SPEC, O>;
#[doc = "Field `HYST` reader - Hysteresis Enable"]
pub type HYST_R = crate::BitReader;
#[doc = "Field `HYST` writer - Hysteresis Enable"]
pub type HYST_W<'a, const O: u8> = crate::BitWriter<'a, BOD33_SPEC, O>;
#[doc = "Field `ACTION` reader - Action when Threshold Crossed"]
pub type ACTION_R = crate::FieldReader;
#[doc = "Field `ACTION` writer - Action when Threshold Crossed"]
pub type ACTION_W<'a, const O: u8> = crate::FieldWriter<'a, BOD33_SPEC, 2, O>;
#[doc = "Field `RUNSTDBY` reader - Run during Standby"]
pub type RUNSTDBY_R = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run during Standby"]
pub type RUNSTDBY_W<'a, const O: u8> = crate::BitWriter<'a, BOD33_SPEC, O>;
#[doc = "Field `MODE` reader - Operation Modes"]
pub type MODE_R = crate::BitReader;
#[doc = "Field `MODE` writer - Operation Modes"]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, BOD33_SPEC, O>;
#[doc = "Field `CEN` reader - Clock Enable"]
pub type CEN_R = crate::BitReader;
#[doc = "Field `CEN` writer - Clock Enable"]
pub type CEN_W<'a, const O: u8> = crate::BitWriter<'a, BOD33_SPEC, O>;
#[doc = "Field `PSEL` reader - Prescaler Select"]
pub type PSEL_R = crate::FieldReader;
#[doc = "Field `PSEL` writer - Prescaler Select"]
pub type PSEL_W<'a, const O: u8> = crate::FieldWriter<'a, BOD33_SPEC, 4, O>;
#[doc = "Field `LEVEL` reader - Threshold Level"]
pub type LEVEL_R = crate::FieldReader;
#[doc = "Field `LEVEL` writer - Threshold Level"]
pub type LEVEL_W<'a, const O: u8> = crate::FieldWriter<'a, BOD33_SPEC, 6, O>;
impl R {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Hysteresis Enable"]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Action when Threshold Crossed"]
    #[inline(always)]
    pub fn action(&self) -> ACTION_R {
        ACTION_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 6 - Run during Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Operation Modes"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clock Enable"]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Prescaler Select"]
    #[inline(always)]
    pub fn psel(&self) -> PSEL_R {
        PSEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - Threshold Level"]
    #[inline(always)]
    pub fn level(&self) -> LEVEL_R {
        LEVEL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<1> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 2 - Hysteresis Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hyst(&mut self) -> HYST_W<2> {
        HYST_W::new(self)
    }
    #[doc = "Bits 3:4 - Action when Threshold Crossed"]
    #[inline(always)]
    #[must_use]
    pub fn action(&mut self) -> ACTION_W<3> {
        ACTION_W::new(self)
    }
    #[doc = "Bit 6 - Run during Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<6> {
        RUNSTDBY_W::new(self)
    }
    #[doc = "Bit 8 - Operation Modes"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<8> {
        MODE_W::new(self)
    }
    #[doc = "Bit 9 - Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cen(&mut self) -> CEN_W<9> {
        CEN_W::new(self)
    }
    #[doc = "Bits 12:15 - Prescaler Select"]
    #[inline(always)]
    #[must_use]
    pub fn psel(&mut self) -> PSEL_W<12> {
        PSEL_W::new(self)
    }
    #[doc = "Bits 16:21 - Threshold Level"]
    #[inline(always)]
    #[must_use]
    pub fn level(&mut self) -> LEVEL_W<16> {
        LEVEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "3.3V Brown-Out Detector (BOD33) Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bod33](index.html) module"]
pub struct BOD33_SPEC;
impl crate::RegisterSpec for BOD33_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bod33::R](R) reader structure"]
impl crate::Readable for BOD33_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bod33::W](W) writer structure"]
impl crate::Writable for BOD33_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BOD33 to value 0"]
impl crate::Resettable for BOD33_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
