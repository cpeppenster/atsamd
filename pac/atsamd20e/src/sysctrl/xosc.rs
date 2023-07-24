#[doc = "Register `XOSC` reader"]
pub struct R(crate::R<XOSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XOSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XOSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XOSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XOSC` writer"]
pub struct W(crate::W<XOSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XOSC_SPEC>;
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
impl From<crate::W<XOSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XOSC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Enable"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, XOSC_SPEC, O>;
#[doc = "Field `XTALEN` reader - Crystal Oscillator Enable"]
pub type XTALEN_R = crate::BitReader;
#[doc = "Field `XTALEN` writer - Crystal Oscillator Enable"]
pub type XTALEN_W<'a, const O: u8> = crate::BitWriter<'a, XOSC_SPEC, O>;
#[doc = "Field `RUNSTDBY` reader - Run during Standby"]
pub type RUNSTDBY_R = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run during Standby"]
pub type RUNSTDBY_W<'a, const O: u8> = crate::BitWriter<'a, XOSC_SPEC, O>;
#[doc = "Field `ONDEMAND` reader - Enable on Demand"]
pub type ONDEMAND_R = crate::BitReader;
#[doc = "Field `ONDEMAND` writer - Enable on Demand"]
pub type ONDEMAND_W<'a, const O: u8> = crate::BitWriter<'a, XOSC_SPEC, O>;
#[doc = "Field `GAIN` reader - Gain Value"]
pub type GAIN_R = crate::FieldReader;
#[doc = "Field `GAIN` writer - Gain Value"]
pub type GAIN_W<'a, const O: u8> = crate::FieldWriter<'a, XOSC_SPEC, 3, O>;
#[doc = "Field `AMPGC` reader - Automatic Amplitude Gain Control"]
pub type AMPGC_R = crate::BitReader;
#[doc = "Field `AMPGC` writer - Automatic Amplitude Gain Control"]
pub type AMPGC_W<'a, const O: u8> = crate::BitWriter<'a, XOSC_SPEC, O>;
#[doc = "Field `STARTUP` reader - Start-Up Time"]
pub type STARTUP_R = crate::FieldReader;
#[doc = "Field `STARTUP` writer - Start-Up Time"]
pub type STARTUP_W<'a, const O: u8> = crate::FieldWriter<'a, XOSC_SPEC, 4, O>;
impl R {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Crystal Oscillator Enable"]
    #[inline(always)]
    pub fn xtalen(&self) -> XTALEN_R {
        XTALEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Run during Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable on Demand"]
    #[inline(always)]
    pub fn ondemand(&self) -> ONDEMAND_R {
        ONDEMAND_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Gain Value"]
    #[inline(always)]
    pub fn gain(&self) -> GAIN_R {
        GAIN_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Automatic Amplitude Gain Control"]
    #[inline(always)]
    pub fn ampgc(&self) -> AMPGC_R {
        AMPGC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Start-Up Time"]
    #[inline(always)]
    pub fn startup(&self) -> STARTUP_R {
        STARTUP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<1> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 2 - Crystal Oscillator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn xtalen(&mut self) -> XTALEN_W<2> {
        XTALEN_W::new(self)
    }
    #[doc = "Bit 6 - Run during Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<6> {
        RUNSTDBY_W::new(self)
    }
    #[doc = "Bit 7 - Enable on Demand"]
    #[inline(always)]
    #[must_use]
    pub fn ondemand(&mut self) -> ONDEMAND_W<7> {
        ONDEMAND_W::new(self)
    }
    #[doc = "Bits 8:10 - Gain Value"]
    #[inline(always)]
    #[must_use]
    pub fn gain(&mut self) -> GAIN_W<8> {
        GAIN_W::new(self)
    }
    #[doc = "Bit 11 - Automatic Amplitude Gain Control"]
    #[inline(always)]
    #[must_use]
    pub fn ampgc(&mut self) -> AMPGC_W<11> {
        AMPGC_W::new(self)
    }
    #[doc = "Bits 12:15 - Start-Up Time"]
    #[inline(always)]
    #[must_use]
    pub fn startup(&mut self) -> STARTUP_W<12> {
        STARTUP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "XOSC Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xosc](index.html) module"]
pub struct XOSC_SPEC;
impl crate::RegisterSpec for XOSC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [xosc::R](R) reader structure"]
impl crate::Readable for XOSC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xosc::W](W) writer structure"]
impl crate::Writable for XOSC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets XOSC to value 0x80"]
impl crate::Resettable for XOSC_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
