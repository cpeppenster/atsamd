#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS` writer"]
pub struct W(crate::W<STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS_SPEC>;
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
impl From<crate::W<STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUSERR` reader - Bus Error"]
pub type BUSERR_R = crate::BitReader;
#[doc = "Field `BUSERR` writer - Bus Error"]
pub type BUSERR_W<'a, const O: u8> = crate::BitWriter<'a, STATUS_SPEC, O>;
#[doc = "Field `COLL` reader - Transmit Collision"]
pub type COLL_R = crate::BitReader;
#[doc = "Field `COLL` writer - Transmit Collision"]
pub type COLL_W<'a, const O: u8> = crate::BitWriter<'a, STATUS_SPEC, O>;
#[doc = "Field `RXNACK` reader - Received Not Acknowledge"]
pub type RXNACK_R = crate::BitReader;
#[doc = "Field `DIR` reader - Read / Write Direction"]
pub type DIR_R = crate::BitReader;
#[doc = "Field `SR` reader - Repeated Start"]
pub type SR_R = crate::BitReader;
#[doc = "Field `LOWTOUT` reader - SCL Low Time-out"]
pub type LOWTOUT_R = crate::BitReader;
#[doc = "Field `LOWTOUT` writer - SCL Low Time-out"]
pub type LOWTOUT_W<'a, const O: u8> = crate::BitWriter<'a, STATUS_SPEC, O>;
#[doc = "Field `CLKHOLD` reader - Clock Hold"]
pub type CLKHOLD_R = crate::BitReader;
#[doc = "Field `SYNCBUSY` reader - Synchronization Busy"]
pub type SYNCBUSY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Bus Error"]
    #[inline(always)]
    pub fn buserr(&self) -> BUSERR_R {
        BUSERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Collision"]
    #[inline(always)]
    pub fn coll(&self) -> COLL_R {
        COLL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Received Not Acknowledge"]
    #[inline(always)]
    pub fn rxnack(&self) -> RXNACK_R {
        RXNACK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Read / Write Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Repeated Start"]
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - SCL Low Time-out"]
    #[inline(always)]
    pub fn lowtout(&self) -> LOWTOUT_R {
        LOWTOUT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Clock Hold"]
    #[inline(always)]
    pub fn clkhold(&self) -> CLKHOLD_R {
        CLKHOLD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - Synchronization Busy"]
    #[inline(always)]
    pub fn syncbusy(&self) -> SYNCBUSY_R {
        SYNCBUSY_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bus Error"]
    #[inline(always)]
    #[must_use]
    pub fn buserr(&mut self) -> BUSERR_W<0> {
        BUSERR_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Collision"]
    #[inline(always)]
    #[must_use]
    pub fn coll(&mut self) -> COLL_W<1> {
        COLL_W::new(self)
    }
    #[doc = "Bit 6 - SCL Low Time-out"]
    #[inline(always)]
    #[must_use]
    pub fn lowtout(&mut self) -> LOWTOUT_W<6> {
        LOWTOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2CS Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status::W](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
