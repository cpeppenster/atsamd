#[doc = "Register `ADDR` reader"]
pub struct R(crate::R<ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDR` writer"]
pub struct W(crate::W<ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDR_SPEC>;
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
impl From<crate::W<ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GENCEN` reader - General Call Address Enable"]
pub type GENCEN_R = crate::BitReader;
#[doc = "Field `GENCEN` writer - General Call Address Enable"]
pub type GENCEN_W<'a, const O: u8> = crate::BitWriter<'a, ADDR_SPEC, O>;
#[doc = "Field `ADDR` reader - Address"]
pub type ADDR_R = crate::FieldReader;
#[doc = "Field `ADDR` writer - Address"]
pub type ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, ADDR_SPEC, 7, O>;
#[doc = "Field `ADDRMASK` reader - Address Mask"]
pub type ADDRMASK_R = crate::FieldReader;
#[doc = "Field `ADDRMASK` writer - Address Mask"]
pub type ADDRMASK_W<'a, const O: u8> = crate::FieldWriter<'a, ADDR_SPEC, 7, O>;
impl R {
    #[doc = "Bit 0 - General Call Address Enable"]
    #[inline(always)]
    pub fn gencen(&self) -> GENCEN_R {
        GENCEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 17:23 - Address Mask"]
    #[inline(always)]
    pub fn addrmask(&self) -> ADDRMASK_R {
        ADDRMASK_R::new(((self.bits >> 17) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - General Call Address Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gencen(&mut self) -> GENCEN_W<0> {
        GENCEN_W::new(self)
    }
    #[doc = "Bits 1:7 - Address"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<1> {
        ADDR_W::new(self)
    }
    #[doc = "Bits 17:23 - Address Mask"]
    #[inline(always)]
    #[must_use]
    pub fn addrmask(&mut self) -> ADDRMASK_W<17> {
        ADDRMASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2CS Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr](index.html) module"]
pub struct ADDR_SPEC;
impl crate::RegisterSpec for ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addr::R](R) reader structure"]
impl crate::Readable for ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addr::W](W) writer structure"]
impl crate::Writable for ADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDR to value 0"]
impl crate::Resettable for ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
