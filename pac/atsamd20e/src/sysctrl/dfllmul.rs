#[doc = "Register `DFLLMUL` reader"]
pub struct R(crate::R<DFLLMUL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFLLMUL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFLLMUL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFLLMUL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFLLMUL` writer"]
pub struct W(crate::W<DFLLMUL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFLLMUL_SPEC>;
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
impl From<crate::W<DFLLMUL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFLLMUL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MUL` reader - Multiplication Value"]
pub type MUL_R = crate::FieldReader<u16>;
#[doc = "Field `MUL` writer - Multiplication Value"]
pub type MUL_W<'a, const O: u8> = crate::FieldWriter<'a, DFLLMUL_SPEC, 16, O, u16>;
#[doc = "Field `FSTEP` reader - Maximum Fine Step Size"]
pub type FSTEP_R = crate::FieldReader<u16>;
#[doc = "Field `FSTEP` writer - Maximum Fine Step Size"]
pub type FSTEP_W<'a, const O: u8> = crate::FieldWriter<'a, DFLLMUL_SPEC, 10, O, u16>;
#[doc = "Field `CSTEP` reader - Maximum Coarse Step Size"]
pub type CSTEP_R = crate::FieldReader;
#[doc = "Field `CSTEP` writer - Maximum Coarse Step Size"]
pub type CSTEP_W<'a, const O: u8> = crate::FieldWriter<'a, DFLLMUL_SPEC, 6, O>;
impl R {
    #[doc = "Bits 0:15 - Multiplication Value"]
    #[inline(always)]
    pub fn mul(&self) -> MUL_R {
        MUL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:25 - Maximum Fine Step Size"]
    #[inline(always)]
    pub fn fstep(&self) -> FSTEP_R {
        FSTEP_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 26:31 - Maximum Coarse Step Size"]
    #[inline(always)]
    pub fn cstep(&self) -> CSTEP_R {
        CSTEP_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Multiplication Value"]
    #[inline(always)]
    #[must_use]
    pub fn mul(&mut self) -> MUL_W<0> {
        MUL_W::new(self)
    }
    #[doc = "Bits 16:25 - Maximum Fine Step Size"]
    #[inline(always)]
    #[must_use]
    pub fn fstep(&mut self) -> FSTEP_W<16> {
        FSTEP_W::new(self)
    }
    #[doc = "Bits 26:31 - Maximum Coarse Step Size"]
    #[inline(always)]
    #[must_use]
    pub fn cstep(&mut self) -> CSTEP_W<26> {
        CSTEP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DFLL Multiplier\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfllmul](index.html) module"]
pub struct DFLLMUL_SPEC;
impl crate::RegisterSpec for DFLLMUL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfllmul::R](R) reader structure"]
impl crate::Readable for DFLLMUL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfllmul::W](W) writer structure"]
impl crate::Writable for DFLLMUL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DFLLMUL to value 0"]
impl crate::Resettable for DFLLMUL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
