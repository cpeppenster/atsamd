#[doc = "Register `APBBMASK` reader"]
pub struct R(crate::R<APBBMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APBBMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APBBMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APBBMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APBBMASK` writer"]
pub struct W(crate::W<APBBMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APBBMASK_SPEC>;
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
impl From<crate::W<APBBMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APBBMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAC1_` reader - PAC1 APB Clock Enable"]
pub type PAC1__R = crate::BitReader;
#[doc = "Field `PAC1_` writer - PAC1 APB Clock Enable"]
pub type PAC1__W<'a, const O: u8> = crate::BitWriter<'a, APBBMASK_SPEC, O>;
#[doc = "Field `DSU_` reader - DSU APB Clock Enable"]
pub type DSU__R = crate::BitReader;
#[doc = "Field `DSU_` writer - DSU APB Clock Enable"]
pub type DSU__W<'a, const O: u8> = crate::BitWriter<'a, APBBMASK_SPEC, O>;
#[doc = "Field `NVMCTRL_` reader - NVMCTRL APB Clock Enable"]
pub type NVMCTRL__R = crate::BitReader;
#[doc = "Field `NVMCTRL_` writer - NVMCTRL APB Clock Enable"]
pub type NVMCTRL__W<'a, const O: u8> = crate::BitWriter<'a, APBBMASK_SPEC, O>;
#[doc = "Field `PORT_` reader - PORT APB Clock Enable"]
pub type PORT__R = crate::BitReader;
#[doc = "Field `PORT_` writer - PORT APB Clock Enable"]
pub type PORT__W<'a, const O: u8> = crate::BitWriter<'a, APBBMASK_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PAC1 APB Clock Enable"]
    #[inline(always)]
    pub fn pac1_(&self) -> PAC1__R {
        PAC1__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DSU APB Clock Enable"]
    #[inline(always)]
    pub fn dsu_(&self) -> DSU__R {
        DSU__R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NVMCTRL APB Clock Enable"]
    #[inline(always)]
    pub fn nvmctrl_(&self) -> NVMCTRL__R {
        NVMCTRL__R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PORT APB Clock Enable"]
    #[inline(always)]
    pub fn port_(&self) -> PORT__R {
        PORT__R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PAC1 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pac1_(&mut self) -> PAC1__W<0> {
        PAC1__W::new(self)
    }
    #[doc = "Bit 1 - DSU APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dsu_(&mut self) -> DSU__W<1> {
        DSU__W::new(self)
    }
    #[doc = "Bit 2 - NVMCTRL APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nvmctrl_(&mut self) -> NVMCTRL__W<2> {
        NVMCTRL__W::new(self)
    }
    #[doc = "Bit 3 - PORT APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn port_(&mut self) -> PORT__W<3> {
        PORT__W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APBB Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbbmask](index.html) module"]
pub struct APBBMASK_SPEC;
impl crate::RegisterSpec for APBBMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apbbmask::R](R) reader structure"]
impl crate::Readable for APBBMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apbbmask::W](W) writer structure"]
impl crate::Writable for APBBMASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APBBMASK to value 0x1f"]
impl crate::Resettable for APBBMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0x1f;
}
