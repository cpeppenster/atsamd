#[doc = "Register `TSH` reader"]
pub struct R(crate::R<TSH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSH` writer"]
pub struct W(crate::W<TSH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSH_SPEC>;
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
impl From<crate::W<TSH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCS` reader - Timer Count in Seconds"]
pub struct TCS_R(crate::FieldReader<u16, u16>);
impl TCS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCS` writer - Timer Count in Seconds"]
pub struct TCS_W<'a> {
    w: &'a mut W,
}
impl<'a> TCS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Timer Count in Seconds"]
    #[inline(always)]
    pub fn tcs(&self) -> TCS_R {
        TCS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer Count in Seconds"]
    #[inline(always)]
    pub fn tcs(&mut self) -> TCS_W {
        TCS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "1588 Timer Seconds High \\[15:0\\]
Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsh](index.html) module"]
pub struct TSH_SPEC;
impl crate::RegisterSpec for TSH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsh::R](R) reader structure"]
impl crate::Readable for TSH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsh::W](W) writer structure"]
impl crate::Writable for TSH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSH to value 0"]
impl crate::Resettable for TSH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
