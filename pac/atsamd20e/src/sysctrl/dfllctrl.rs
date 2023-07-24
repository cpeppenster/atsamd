#[doc = "Register `DFLLCTRL` reader"]
pub struct R(crate::R<DFLLCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFLLCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFLLCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFLLCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFLLCTRL` writer"]
pub struct W(crate::W<DFLLCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFLLCTRL_SPEC>;
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
impl From<crate::W<DFLLCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFLLCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Enable"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, DFLLCTRL_SPEC, O>;
#[doc = "Field `MODE` reader - Mode Selection"]
pub type MODE_R = crate::BitReader;
#[doc = "Field `MODE` writer - Mode Selection"]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, DFLLCTRL_SPEC, O>;
#[doc = "Field `STABLE` reader - Stable Frequency"]
pub type STABLE_R = crate::BitReader;
#[doc = "Field `STABLE` writer - Stable Frequency"]
pub type STABLE_W<'a, const O: u8> = crate::BitWriter<'a, DFLLCTRL_SPEC, O>;
#[doc = "Field `LLAW` reader - Lose Lock After Wake"]
pub type LLAW_R = crate::BitReader;
#[doc = "Field `LLAW` writer - Lose Lock After Wake"]
pub type LLAW_W<'a, const O: u8> = crate::BitWriter<'a, DFLLCTRL_SPEC, O>;
#[doc = "Field `RUNSTDBY` reader - Run during Standby"]
pub type RUNSTDBY_R = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run during Standby"]
pub type RUNSTDBY_W<'a, const O: u8> = crate::BitWriter<'a, DFLLCTRL_SPEC, O>;
#[doc = "Field `ONDEMAND` reader - Enable on Demand"]
pub type ONDEMAND_R = crate::BitReader;
#[doc = "Field `ONDEMAND` writer - Enable on Demand"]
pub type ONDEMAND_W<'a, const O: u8> = crate::BitWriter<'a, DFLLCTRL_SPEC, O>;
#[doc = "Field `CCDIS` reader - Chill Cycle Disable"]
pub type CCDIS_R = crate::BitReader;
#[doc = "Field `CCDIS` writer - Chill Cycle Disable"]
pub type CCDIS_W<'a, const O: u8> = crate::BitWriter<'a, DFLLCTRL_SPEC, O>;
#[doc = "Field `QLDIS` reader - Quick Lock Disable"]
pub type QLDIS_R = crate::BitReader;
#[doc = "Field `QLDIS` writer - Quick Lock Disable"]
pub type QLDIS_W<'a, const O: u8> = crate::BitWriter<'a, DFLLCTRL_SPEC, O>;
impl R {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mode Selection"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Stable Frequency"]
    #[inline(always)]
    pub fn stable(&self) -> STABLE_R {
        STABLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Lose Lock After Wake"]
    #[inline(always)]
    pub fn llaw(&self) -> LLAW_R {
        LLAW_R::new(((self.bits >> 4) & 1) != 0)
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
    #[doc = "Bit 8 - Chill Cycle Disable"]
    #[inline(always)]
    pub fn ccdis(&self) -> CCDIS_R {
        CCDIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Quick Lock Disable"]
    #[inline(always)]
    pub fn qldis(&self) -> QLDIS_R {
        QLDIS_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<1> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 2 - Mode Selection"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<2> {
        MODE_W::new(self)
    }
    #[doc = "Bit 3 - Stable Frequency"]
    #[inline(always)]
    #[must_use]
    pub fn stable(&mut self) -> STABLE_W<3> {
        STABLE_W::new(self)
    }
    #[doc = "Bit 4 - Lose Lock After Wake"]
    #[inline(always)]
    #[must_use]
    pub fn llaw(&mut self) -> LLAW_W<4> {
        LLAW_W::new(self)
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
    #[doc = "Bit 8 - Chill Cycle Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ccdis(&mut self) -> CCDIS_W<8> {
        CCDIS_W::new(self)
    }
    #[doc = "Bit 9 - Quick Lock Disable"]
    #[inline(always)]
    #[must_use]
    pub fn qldis(&mut self) -> QLDIS_W<9> {
        QLDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DFLL Config\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfllctrl](index.html) module"]
pub struct DFLLCTRL_SPEC;
impl crate::RegisterSpec for DFLLCTRL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [dfllctrl::R](R) reader structure"]
impl crate::Readable for DFLLCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfllctrl::W](W) writer structure"]
impl crate::Writable for DFLLCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DFLLCTRL to value 0x80"]
impl crate::Resettable for DFLLCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
