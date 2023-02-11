#[doc = "Register `IFS` writer"]
pub struct W(crate::W<IFS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFS_SPEC>;
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
impl From<crate::W<IFS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0DONE` writer - DMA Channel 0 Complete Interrupt Flag Set"]
pub type CH0DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
#[doc = "Field `CH1DONE` writer - DMA Channel 1 Complete Interrupt Flag Set"]
pub type CH1DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
#[doc = "Field `CH2DONE` writer - DMA Channel 2 Complete Interrupt Flag Set"]
pub type CH2DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
#[doc = "Field `CH3DONE` writer - DMA Channel 3 Complete Interrupt Flag Set"]
pub type CH3DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
#[doc = "Field `ERR` writer - DMA Error Interrupt Flag Set"]
pub type ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - DMA Channel 0 Complete Interrupt Flag Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch0done(&mut self) -> CH0DONE_W<0> {
        CH0DONE_W::new(self)
    }
    #[doc = "Bit 1 - DMA Channel 1 Complete Interrupt Flag Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch1done(&mut self) -> CH1DONE_W<1> {
        CH1DONE_W::new(self)
    }
    #[doc = "Bit 2 - DMA Channel 2 Complete Interrupt Flag Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch2done(&mut self) -> CH2DONE_W<2> {
        CH2DONE_W::new(self)
    }
    #[doc = "Bit 3 - DMA Channel 3 Complete Interrupt Flag Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch3done(&mut self) -> CH3DONE_W<3> {
        CH3DONE_W::new(self)
    }
    #[doc = "Bit 31 - DMA Error Interrupt Flag Set"]
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
#[doc = "Interrupt Flag Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifs](index.html) module"]
pub struct IFS_SPEC;
impl crate::RegisterSpec for IFS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ifs::W](W) writer structure"]
impl crate::Writable for IFS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IFS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
