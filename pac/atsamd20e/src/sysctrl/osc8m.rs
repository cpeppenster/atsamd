#[doc = "Register `OSC8M` reader"]
pub struct R(crate::R<OSC8M_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSC8M_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSC8M_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSC8M_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSC8M` writer"]
pub struct W(crate::W<OSC8M_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSC8M_SPEC>;
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
impl From<crate::W<OSC8M_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSC8M_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Enable"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, OSC8M_SPEC, O>;
#[doc = "Field `RUNSTDBY` reader - Run during Standby"]
pub type RUNSTDBY_R = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run during Standby"]
pub type RUNSTDBY_W<'a, const O: u8> = crate::BitWriter<'a, OSC8M_SPEC, O>;
#[doc = "Field `ONDEMAND` reader - Enable on Demand"]
pub type ONDEMAND_R = crate::BitReader;
#[doc = "Field `ONDEMAND` writer - Enable on Demand"]
pub type ONDEMAND_W<'a, const O: u8> = crate::BitWriter<'a, OSC8M_SPEC, O>;
#[doc = "Field `PRESC` reader - Prescaler Select"]
pub type PRESC_R = crate::FieldReader;
#[doc = "Field `PRESC` writer - Prescaler Select"]
pub type PRESC_W<'a, const O: u8> = crate::FieldWriter<'a, OSC8M_SPEC, 2, O>;
#[doc = "Field `CALIB` reader - Calibration Value"]
pub type CALIB_R = crate::FieldReader<u16>;
#[doc = "Field `CALIB` writer - Calibration Value"]
pub type CALIB_W<'a, const O: u8> = crate::FieldWriter<'a, OSC8M_SPEC, 12, O, u16>;
#[doc = "Field `FRANGE` reader - Frequency Range"]
pub type FRANGE_R = crate::FieldReader;
#[doc = "Field `FRANGE` writer - Frequency Range"]
pub type FRANGE_W<'a, const O: u8> = crate::FieldWriter<'a, OSC8M_SPEC, 2, O>;
impl R {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
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
    #[doc = "Bits 8:9 - Prescaler Select"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:27 - Calibration Value"]
    #[inline(always)]
    pub fn calib(&self) -> CALIB_R {
        CALIB_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 30:31 - Frequency Range"]
    #[inline(always)]
    pub fn frange(&self) -> FRANGE_R {
        FRANGE_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<1> {
        ENABLE_W::new(self)
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
    #[doc = "Bits 8:9 - Prescaler Select"]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<8> {
        PRESC_W::new(self)
    }
    #[doc = "Bits 16:27 - Calibration Value"]
    #[inline(always)]
    #[must_use]
    pub fn calib(&mut self) -> CALIB_W<16> {
        CALIB_W::new(self)
    }
    #[doc = "Bits 30:31 - Frequency Range"]
    #[inline(always)]
    #[must_use]
    pub fn frange(&mut self) -> FRANGE_W<30> {
        FRANGE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OSC8M Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc8m](index.html) module"]
pub struct OSC8M_SPEC;
impl crate::RegisterSpec for OSC8M_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [osc8m::R](R) reader structure"]
impl crate::Readable for OSC8M_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [osc8m::W](W) writer structure"]
impl crate::Writable for OSC8M_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSC8M to value 0x8707_0382"]
impl crate::Resettable for OSC8M_SPEC {
    const RESET_VALUE: Self::Ux = 0x8707_0382;
}
