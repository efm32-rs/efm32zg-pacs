#[doc = "Register `IEN` reader"]
pub struct R(crate::R<IEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IEN` writer"]
pub struct W(crate::W<IEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IEN_SPEC>;
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
impl From<crate::W<IEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0DONE` reader - DMA Channel 0 Complete Interrupt Enable"]
pub type CH0DONE_R = crate::BitReader<bool>;
#[doc = "Field `CH0DONE` writer - DMA Channel 0 Complete Interrupt Enable"]
pub type CH0DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `CH1DONE` reader - DMA Channel 1 Complete Interrupt Enable"]
pub type CH1DONE_R = crate::BitReader<bool>;
#[doc = "Field `CH1DONE` writer - DMA Channel 1 Complete Interrupt Enable"]
pub type CH1DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `CH2DONE` reader - DMA Channel 2 Complete Interrupt Enable"]
pub type CH2DONE_R = crate::BitReader<bool>;
#[doc = "Field `CH2DONE` writer - DMA Channel 2 Complete Interrupt Enable"]
pub type CH2DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `CH3DONE` reader - DMA Channel 3 Complete Interrupt Enable"]
pub type CH3DONE_R = crate::BitReader<bool>;
#[doc = "Field `CH3DONE` writer - DMA Channel 3 Complete Interrupt Enable"]
pub type CH3DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `ERR` reader - DMA Error Interrupt Flag Enable"]
pub type ERR_R = crate::BitReader<bool>;
#[doc = "Field `ERR` writer - DMA Error Interrupt Flag Enable"]
pub type ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DMA Channel 0 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch0done(&self) -> CH0DONE_R {
        CH0DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Channel 1 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch1done(&self) -> CH1DONE_R {
        CH1DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA Channel 2 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch2done(&self) -> CH2DONE_R {
        CH2DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Channel 3 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch3done(&self) -> CH3DONE_R {
        CH3DONE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 31 - DMA Error Interrupt Flag Enable"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Channel 0 Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0done(&mut self) -> CH0DONE_W<0> {
        CH0DONE_W::new(self)
    }
    #[doc = "Bit 1 - DMA Channel 1 Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1done(&mut self) -> CH1DONE_W<1> {
        CH1DONE_W::new(self)
    }
    #[doc = "Bit 2 - DMA Channel 2 Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2done(&mut self) -> CH2DONE_W<2> {
        CH2DONE_W::new(self)
    }
    #[doc = "Bit 3 - DMA Channel 3 Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch3done(&mut self) -> CH3DONE_W<3> {
        CH3DONE_W::new(self)
    }
    #[doc = "Bit 31 - DMA Error Interrupt Flag Enable"]
    #[inline(always)]
    #[must_use]
    pub fn err(&mut self) -> ERR_W<31> {
        ERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ien](index.html) module"]
pub struct IEN_SPEC;
impl crate::RegisterSpec for IEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ien::R](R) reader structure"]
impl crate::Readable for IEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ien::W](W) writer structure"]
impl crate::Writable for IEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
