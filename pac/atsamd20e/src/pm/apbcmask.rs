#[doc = "Register `APBCMASK` reader"]
pub struct R(crate::R<APBCMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APBCMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APBCMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APBCMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APBCMASK` writer"]
pub struct W(crate::W<APBCMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APBCMASK_SPEC>;
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
impl From<crate::W<APBCMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APBCMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAC2_` reader - PAC2 APB Clock Enable"]
pub type PAC2__R = crate::BitReader;
#[doc = "Field `PAC2_` writer - PAC2 APB Clock Enable"]
pub type PAC2__W<'a, const O: u8> = crate::BitWriter<'a, APBCMASK_SPEC, O>;
#[doc = "Field `EVSYS_` reader - EVSYS APB Clock Enable"]
pub type EVSYS__R = crate::BitReader;
#[doc = "Field `EVSYS_` writer - EVSYS APB Clock Enable"]
pub type EVSYS__W<'a, const O: u8> = crate::BitWriter<'a, APBCMASK_SPEC, O>;
#[doc = "Field `SERCOM0_` reader - SERCOM0 APB Clock Enable"]
pub type SERCOM0__R = crate::BitReader;
#[doc = "Field `SERCOM0_` writer - SERCOM0 APB Clock Enable"]
pub type SERCOM0__W<'a, const O: u8> = crate::BitWriter<'a, APBCMASK_SPEC, O>;
#[doc = "Field `SERCOM1_` reader - SERCOM1 APB Clock Enable"]
pub type SERCOM1__R = crate::BitReader;
#[doc = "Field `SERCOM1_` writer - SERCOM1 APB Clock Enable"]
pub type SERCOM1__W<'a, const O: u8> = crate::BitWriter<'a, APBCMASK_SPEC, O>;
#[doc = "Field `SERCOM2_` reader - SERCOM2 APB Clock Enable"]
pub type SERCOM2__R = crate::BitReader;
#[doc = "Field `SERCOM2_` writer - SERCOM2 APB Clock Enable"]
pub type SERCOM2__W<'a, const O: u8> = crate::BitWriter<'a, APBCMASK_SPEC, O>;
#[doc = "Field `SERCOM3_` reader - SERCOM3 APB Clock Enable"]
pub type SERCOM3__R = crate::BitReader;
#[doc = "Field `SERCOM3_` writer - SERCOM3 APB Clock Enable"]
pub type SERCOM3__W<'a, const O: u8> = crate::BitWriter<'a, APBCMASK_SPEC, O>;
#[doc = "Field `TC0_` reader - TC0 APB Clock Enable"]
pub type TC0__R = crate::BitReader;
#[doc = "Field `TC0_` writer - TC0 APB Clock Enable"]
pub type TC0__W<'a, const O: u8> = crate::BitWriter<'a, APBCMASK_SPEC, O>;
#[doc = "Field `TC1_` reader - TC1 APB Clock Enable"]
pub type TC1__R = crate::BitReader;
#[doc = "Field `TC1_` writer - TC1 APB Clock Enable"]
pub type TC1__W<'a, const O: u8> = crate::BitWriter<'a, APBCMASK_SPEC, O>;
#[doc = "Field `TC2_` reader - TC2 APB Clock Enable"]
pub type TC2__R = crate::BitReader;
#[doc = "Field `TC2_` writer - TC2 APB Clock Enable"]
pub type TC2__W<'a, const O: u8> = crate::BitWriter<'a, APBCMASK_SPEC, O>;
#[doc = "Field `TC3_` reader - TC3 APB Clock Enable"]
pub type TC3__R = crate::BitReader;
#[doc = "Field `TC3_` writer - TC3 APB Clock Enable"]
pub type TC3__W<'a, const O: u8> = crate::BitWriter<'a, APBCMASK_SPEC, O>;
#[doc = "Field `TC4_` reader - TC4 APB Clock Enable"]
pub type TC4__R = crate::BitReader;
#[doc = "Field `TC4_` writer - TC4 APB Clock Enable"]
pub type TC4__W<'a, const O: u8> = crate::BitWriter<'a, APBCMASK_SPEC, O>;
#[doc = "Field `TC5_` reader - TC5 APB Clock Enable"]
pub type TC5__R = crate::BitReader;
#[doc = "Field `TC5_` writer - TC5 APB Clock Enable"]
pub type TC5__W<'a, const O: u8> = crate::BitWriter<'a, APBCMASK_SPEC, O>;
#[doc = "Field `ADC_` reader - ADC APB Clock Enable"]
pub type ADC__R = crate::BitReader;
#[doc = "Field `ADC_` writer - ADC APB Clock Enable"]
pub type ADC__W<'a, const O: u8> = crate::BitWriter<'a, APBCMASK_SPEC, O>;
#[doc = "Field `AC_` reader - AC APB Clock Enable"]
pub type AC__R = crate::BitReader;
#[doc = "Field `AC_` writer - AC APB Clock Enable"]
pub type AC__W<'a, const O: u8> = crate::BitWriter<'a, APBCMASK_SPEC, O>;
#[doc = "Field `DAC_` reader - DAC APB Clock Enable"]
pub type DAC__R = crate::BitReader;
#[doc = "Field `DAC_` writer - DAC APB Clock Enable"]
pub type DAC__W<'a, const O: u8> = crate::BitWriter<'a, APBCMASK_SPEC, O>;
#[doc = "Field `PTC_` reader - PTC APB Clock Enable"]
pub type PTC__R = crate::BitReader;
#[doc = "Field `PTC_` writer - PTC APB Clock Enable"]
pub type PTC__W<'a, const O: u8> = crate::BitWriter<'a, APBCMASK_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PAC2 APB Clock Enable"]
    #[inline(always)]
    pub fn pac2_(&self) -> PAC2__R {
        PAC2__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EVSYS APB Clock Enable"]
    #[inline(always)]
    pub fn evsys_(&self) -> EVSYS__R {
        EVSYS__R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SERCOM0 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom0_(&self) -> SERCOM0__R {
        SERCOM0__R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SERCOM1 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom1_(&self) -> SERCOM1__R {
        SERCOM1__R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SERCOM2 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom2_(&self) -> SERCOM2__R {
        SERCOM2__R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SERCOM3 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom3_(&self) -> SERCOM3__R {
        SERCOM3__R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - TC0 APB Clock Enable"]
    #[inline(always)]
    pub fn tc0_(&self) -> TC0__R {
        TC0__R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TC1 APB Clock Enable"]
    #[inline(always)]
    pub fn tc1_(&self) -> TC1__R {
        TC1__R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TC2 APB Clock Enable"]
    #[inline(always)]
    pub fn tc2_(&self) -> TC2__R {
        TC2__R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TC3 APB Clock Enable"]
    #[inline(always)]
    pub fn tc3_(&self) -> TC3__R {
        TC3__R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TC4 APB Clock Enable"]
    #[inline(always)]
    pub fn tc4_(&self) -> TC4__R {
        TC4__R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TC5 APB Clock Enable"]
    #[inline(always)]
    pub fn tc5_(&self) -> TC5__R {
        TC5__R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - ADC APB Clock Enable"]
    #[inline(always)]
    pub fn adc_(&self) -> ADC__R {
        ADC__R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - AC APB Clock Enable"]
    #[inline(always)]
    pub fn ac_(&self) -> AC__R {
        AC__R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DAC APB Clock Enable"]
    #[inline(always)]
    pub fn dac_(&self) -> DAC__R {
        DAC__R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PTC APB Clock Enable"]
    #[inline(always)]
    pub fn ptc_(&self) -> PTC__R {
        PTC__R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PAC2 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pac2_(&mut self) -> PAC2__W<0> {
        PAC2__W::new(self)
    }
    #[doc = "Bit 1 - EVSYS APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn evsys_(&mut self) -> EVSYS__W<1> {
        EVSYS__W::new(self)
    }
    #[doc = "Bit 2 - SERCOM0 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sercom0_(&mut self) -> SERCOM0__W<2> {
        SERCOM0__W::new(self)
    }
    #[doc = "Bit 3 - SERCOM1 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sercom1_(&mut self) -> SERCOM1__W<3> {
        SERCOM1__W::new(self)
    }
    #[doc = "Bit 4 - SERCOM2 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sercom2_(&mut self) -> SERCOM2__W<4> {
        SERCOM2__W::new(self)
    }
    #[doc = "Bit 5 - SERCOM3 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sercom3_(&mut self) -> SERCOM3__W<5> {
        SERCOM3__W::new(self)
    }
    #[doc = "Bit 8 - TC0 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tc0_(&mut self) -> TC0__W<8> {
        TC0__W::new(self)
    }
    #[doc = "Bit 9 - TC1 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tc1_(&mut self) -> TC1__W<9> {
        TC1__W::new(self)
    }
    #[doc = "Bit 10 - TC2 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tc2_(&mut self) -> TC2__W<10> {
        TC2__W::new(self)
    }
    #[doc = "Bit 11 - TC3 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tc3_(&mut self) -> TC3__W<11> {
        TC3__W::new(self)
    }
    #[doc = "Bit 12 - TC4 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tc4_(&mut self) -> TC4__W<12> {
        TC4__W::new(self)
    }
    #[doc = "Bit 13 - TC5 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tc5_(&mut self) -> TC5__W<13> {
        TC5__W::new(self)
    }
    #[doc = "Bit 16 - ADC APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc_(&mut self) -> ADC__W<16> {
        ADC__W::new(self)
    }
    #[doc = "Bit 17 - AC APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ac_(&mut self) -> AC__W<17> {
        AC__W::new(self)
    }
    #[doc = "Bit 18 - DAC APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dac_(&mut self) -> DAC__W<18> {
        DAC__W::new(self)
    }
    #[doc = "Bit 19 - PTC APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ptc_(&mut self) -> PTC__W<19> {
        PTC__W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APBC Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbcmask](index.html) module"]
pub struct APBCMASK_SPEC;
impl crate::RegisterSpec for APBCMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apbcmask::R](R) reader structure"]
impl crate::Readable for APBCMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apbcmask::W](W) writer structure"]
impl crate::Writable for APBCMASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APBCMASK to value 0x0001_0000"]
impl crate::Resettable for APBCMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0000;
}
