#[doc = "Register `IFC` writer"]
pub struct W(crate::W<IFC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFC_SPEC>;
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
impl From<crate::W<IFC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0DONE` writer - DMA Channel 0 Complete Interrupt Flag Clear"]
pub type CH0DONE_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 0>;
#[doc = "Field `CH1DONE` writer - DMA Channel 1 Complete Interrupt Flag Clear"]
pub type CH1DONE_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 1>;
#[doc = "Field `CH2DONE` writer - DMA Channel 2 Complete Interrupt Flag Clear"]
pub type CH2DONE_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 2>;
#[doc = "Field `CH3DONE` writer - DMA Channel 3 Complete Interrupt Flag Clear"]
pub type CH3DONE_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 3>;
#[doc = "Field `ERR` writer - DMA Error Interrupt Flag Clear"]
pub type ERR_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 31>;
impl W {
    #[doc = "Bit 0 - DMA Channel 0 Complete Interrupt Flag Clear"]
    #[inline(always)]
    pub fn ch0done(&mut self) -> CH0DONE_W {
        CH0DONE_W::new(self)
    }
    #[doc = "Bit 1 - DMA Channel 1 Complete Interrupt Flag Clear"]
    #[inline(always)]
    pub fn ch1done(&mut self) -> CH1DONE_W {
        CH1DONE_W::new(self)
    }
    #[doc = "Bit 2 - DMA Channel 2 Complete Interrupt Flag Clear"]
    #[inline(always)]
    pub fn ch2done(&mut self) -> CH2DONE_W {
        CH2DONE_W::new(self)
    }
    #[doc = "Bit 3 - DMA Channel 3 Complete Interrupt Flag Clear"]
    #[inline(always)]
    pub fn ch3done(&mut self) -> CH3DONE_W {
        CH3DONE_W::new(self)
    }
    #[doc = "Bit 31 - DMA Error Interrupt Flag Clear"]
    #[inline(always)]
    pub fn err(&mut self) -> ERR_W {
        ERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flag Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifc](index.html) module"]
pub struct IFC_SPEC;
impl crate::RegisterSpec for IFC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ifc::W](W) writer structure"]
impl crate::Writable for IFC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IFC to value 0"]
impl crate::Resettable for IFC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
