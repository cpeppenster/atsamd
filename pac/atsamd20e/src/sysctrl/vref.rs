#[doc = "Register `VREF` reader"]
pub struct R(crate::R<VREF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VREF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VREF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VREF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VREF` writer"]
pub struct W(crate::W<VREF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VREF_SPEC>;
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
impl From<crate::W<VREF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VREF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSEN` reader - Temperature Sensor Output Enable"]
pub type TSEN_R = crate::BitReader;
#[doc = "Field `TSEN` writer - Temperature Sensor Output Enable"]
pub type TSEN_W<'a, const O: u8> = crate::BitWriter<'a, VREF_SPEC, O>;
#[doc = "Field `BGOUTEN` reader - Bandgap Output Enable"]
pub type BGOUTEN_R = crate::BitReader;
#[doc = "Field `BGOUTEN` writer - Bandgap Output Enable"]
pub type BGOUTEN_W<'a, const O: u8> = crate::BitWriter<'a, VREF_SPEC, O>;
#[doc = "Field `CALIB` reader - Voltage Reference Calibration Value"]
pub type CALIB_R = crate::FieldReader<u16>;
#[doc = "Field `CALIB` writer - Voltage Reference Calibration Value"]
pub type CALIB_W<'a, const O: u8> = crate::FieldWriter<'a, VREF_SPEC, 11, O, u16>;
impl R {
    #[doc = "Bit 1 - Temperature Sensor Output Enable"]
    #[inline(always)]
    pub fn tsen(&self) -> TSEN_R {
        TSEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bandgap Output Enable"]
    #[inline(always)]
    pub fn bgouten(&self) -> BGOUTEN_R {
        BGOUTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 16:26 - Voltage Reference Calibration Value"]
    #[inline(always)]
    pub fn calib(&self) -> CALIB_R {
        CALIB_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 1 - Temperature Sensor Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsen(&mut self) -> TSEN_W<1> {
        TSEN_W::new(self)
    }
    #[doc = "Bit 2 - Bandgap Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bgouten(&mut self) -> BGOUTEN_W<2> {
        BGOUTEN_W::new(self)
    }
    #[doc = "Bits 16:26 - Voltage Reference Calibration Value"]
    #[inline(always)]
    #[must_use]
    pub fn calib(&mut self) -> CALIB_W<16> {
        CALIB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VREF Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vref](index.html) module"]
pub struct VREF_SPEC;
impl crate::RegisterSpec for VREF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vref::R](R) reader structure"]
impl crate::Readable for VREF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vref::W](W) writer structure"]
impl crate::Writable for VREF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VREF to value 0"]
impl crate::Resettable for VREF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
