#[doc = "Register `BAUD` reader"]
pub struct R(crate::R<BAUD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BAUD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BAUD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BAUD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BAUD` writer"]
pub struct W(crate::W<BAUD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BAUD_SPEC>;
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
impl From<crate::W<BAUD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BAUD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BAUD` reader - Master Baud Rate"]
pub type BAUD_R = crate::FieldReader;
#[doc = "Field `BAUD` writer - Master Baud Rate"]
pub type BAUD_W<'a, const O: u8> = crate::FieldWriter<'a, BAUD_SPEC, 8, O>;
#[doc = "Field `BAUDLOW` reader - Master Baud Rate Low"]
pub type BAUDLOW_R = crate::FieldReader;
#[doc = "Field `BAUDLOW` writer - Master Baud Rate Low"]
pub type BAUDLOW_W<'a, const O: u8> = crate::FieldWriter<'a, BAUD_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Master Baud Rate"]
    #[inline(always)]
    pub fn baud(&self) -> BAUD_R {
        BAUD_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Master Baud Rate Low"]
    #[inline(always)]
    pub fn baudlow(&self) -> BAUDLOW_R {
        BAUDLOW_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Master Baud Rate"]
    #[inline(always)]
    #[must_use]
    pub fn baud(&mut self) -> BAUD_W<0> {
        BAUD_W::new(self)
    }
    #[doc = "Bits 8:15 - Master Baud Rate Low"]
    #[inline(always)]
    #[must_use]
    pub fn baudlow(&mut self) -> BAUDLOW_W<8> {
        BAUDLOW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2CM Baud Rate\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [baud](index.html) module"]
pub struct BAUD_SPEC;
impl crate::RegisterSpec for BAUD_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [baud::R](R) reader structure"]
impl crate::Readable for BAUD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [baud::W](W) writer structure"]
impl crate::Writable for BAUD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BAUD to value 0"]
impl crate::Resettable for BAUD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
