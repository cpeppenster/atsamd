#[doc = "Register `GENDIV` reader"]
pub struct R(crate::R<GENDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GENDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GENDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GENDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GENDIV` writer"]
pub struct W(crate::W<GENDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GENDIV_SPEC>;
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
impl From<crate::W<GENDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GENDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ID` reader - Generic Clock Generator Selection"]
pub type ID_R = crate::FieldReader<IDSELECT_A>;
#[doc = "Generic Clock Generator Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IDSELECT_A {
    #[doc = "0: Generic clock generator 0"]
    GCLK0 = 0,
    #[doc = "1: Generic clock generator 1"]
    GCLK1 = 1,
    #[doc = "2: Generic clock generator 2"]
    GCLK2 = 2,
    #[doc = "3: Generic clock generator 3"]
    GCLK3 = 3,
    #[doc = "4: Generic clock generator 4"]
    GCLK4 = 4,
    #[doc = "5: Generic clock generator 5"]
    GCLK5 = 5,
    #[doc = "6: Generic clock generator 6"]
    GCLK6 = 6,
    #[doc = "7: Generic clock generator 7"]
    GCLK7 = 7,
}
impl From<IDSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: IDSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IDSELECT_A {
    type Ux = u8;
}
impl ID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IDSELECT_A> {
        match self.bits {
            0 => Some(IDSELECT_A::GCLK0),
            1 => Some(IDSELECT_A::GCLK1),
            2 => Some(IDSELECT_A::GCLK2),
            3 => Some(IDSELECT_A::GCLK3),
            4 => Some(IDSELECT_A::GCLK4),
            5 => Some(IDSELECT_A::GCLK5),
            6 => Some(IDSELECT_A::GCLK6),
            7 => Some(IDSELECT_A::GCLK7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GCLK0`"]
    #[inline(always)]
    pub fn is_gclk0(&self) -> bool {
        *self == IDSELECT_A::GCLK0
    }
    #[doc = "Checks if the value of the field is `GCLK1`"]
    #[inline(always)]
    pub fn is_gclk1(&self) -> bool {
        *self == IDSELECT_A::GCLK1
    }
    #[doc = "Checks if the value of the field is `GCLK2`"]
    #[inline(always)]
    pub fn is_gclk2(&self) -> bool {
        *self == IDSELECT_A::GCLK2
    }
    #[doc = "Checks if the value of the field is `GCLK3`"]
    #[inline(always)]
    pub fn is_gclk3(&self) -> bool {
        *self == IDSELECT_A::GCLK3
    }
    #[doc = "Checks if the value of the field is `GCLK4`"]
    #[inline(always)]
    pub fn is_gclk4(&self) -> bool {
        *self == IDSELECT_A::GCLK4
    }
    #[doc = "Checks if the value of the field is `GCLK5`"]
    #[inline(always)]
    pub fn is_gclk5(&self) -> bool {
        *self == IDSELECT_A::GCLK5
    }
    #[doc = "Checks if the value of the field is `GCLK6`"]
    #[inline(always)]
    pub fn is_gclk6(&self) -> bool {
        *self == IDSELECT_A::GCLK6
    }
    #[doc = "Checks if the value of the field is `GCLK7`"]
    #[inline(always)]
    pub fn is_gclk7(&self) -> bool {
        *self == IDSELECT_A::GCLK7
    }
}
#[doc = "Field `ID` writer - Generic Clock Generator Selection"]
pub type ID_W<'a, const O: u8> = crate::FieldWriter<'a, GENDIV_SPEC, 4, O, IDSELECT_A>;
impl<'a, const O: u8> ID_W<'a, O> {
    #[doc = "Generic clock generator 0"]
    #[inline(always)]
    pub fn gclk0(self) -> &'a mut W {
        self.variant(IDSELECT_A::GCLK0)
    }
    #[doc = "Generic clock generator 1"]
    #[inline(always)]
    pub fn gclk1(self) -> &'a mut W {
        self.variant(IDSELECT_A::GCLK1)
    }
    #[doc = "Generic clock generator 2"]
    #[inline(always)]
    pub fn gclk2(self) -> &'a mut W {
        self.variant(IDSELECT_A::GCLK2)
    }
    #[doc = "Generic clock generator 3"]
    #[inline(always)]
    pub fn gclk3(self) -> &'a mut W {
        self.variant(IDSELECT_A::GCLK3)
    }
    #[doc = "Generic clock generator 4"]
    #[inline(always)]
    pub fn gclk4(self) -> &'a mut W {
        self.variant(IDSELECT_A::GCLK4)
    }
    #[doc = "Generic clock generator 5"]
    #[inline(always)]
    pub fn gclk5(self) -> &'a mut W {
        self.variant(IDSELECT_A::GCLK5)
    }
    #[doc = "Generic clock generator 6"]
    #[inline(always)]
    pub fn gclk6(self) -> &'a mut W {
        self.variant(IDSELECT_A::GCLK6)
    }
    #[doc = "Generic clock generator 7"]
    #[inline(always)]
    pub fn gclk7(self) -> &'a mut W {
        self.variant(IDSELECT_A::GCLK7)
    }
}
#[doc = "Field `DIV` reader - Division Factor"]
pub type DIV_R = crate::FieldReader<u16>;
#[doc = "Field `DIV` writer - Division Factor"]
pub type DIV_W<'a, const O: u8> = crate::FieldWriter<'a, GENDIV_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:3 - Generic Clock Generator Selection"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:23 - Division Factor"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Generic Clock Generator Selection"]
    #[inline(always)]
    #[must_use]
    pub fn id(&mut self) -> ID_W<0> {
        ID_W::new(self)
    }
    #[doc = "Bits 8:23 - Division Factor"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<8> {
        DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Generic Clock Generator Division\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gendiv](index.html) module"]
pub struct GENDIV_SPEC;
impl crate::RegisterSpec for GENDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gendiv::R](R) reader structure"]
impl crate::Readable for GENDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gendiv::W](W) writer structure"]
impl crate::Writable for GENDIV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GENDIV to value 0"]
impl crate::Resettable for GENDIV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
