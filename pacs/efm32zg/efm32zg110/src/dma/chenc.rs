#[doc = "Register `CHENC` writer"]
pub struct W(crate::W<CHENC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHENC_SPEC>;
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
impl From<crate::W<CHENC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHENC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0ENC` writer - Channel 0 Enable Clear"]
pub type CH0ENC_W<'a> = crate::BitWriter<'a, u32, CHENC_SPEC, bool, 0>;
#[doc = "Field `CH1ENC` writer - Channel 1 Enable Clear"]
pub type CH1ENC_W<'a> = crate::BitWriter<'a, u32, CHENC_SPEC, bool, 1>;
#[doc = "Field `CH2ENC` writer - Channel 2 Enable Clear"]
pub type CH2ENC_W<'a> = crate::BitWriter<'a, u32, CHENC_SPEC, bool, 2>;
#[doc = "Field `CH3ENC` writer - Channel 3 Enable Clear"]
pub type CH3ENC_W<'a> = crate::BitWriter<'a, u32, CHENC_SPEC, bool, 3>;
impl W {
    #[doc = "Bit 0 - Channel 0 Enable Clear"]
    #[inline(always)]
    pub fn ch0enc(&mut self) -> CH0ENC_W {
        CH0ENC_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Enable Clear"]
    #[inline(always)]
    pub fn ch1enc(&mut self) -> CH1ENC_W {
        CH1ENC_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 Enable Clear"]
    #[inline(always)]
    pub fn ch2enc(&mut self) -> CH2ENC_W {
        CH2ENC_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 Enable Clear"]
    #[inline(always)]
    pub fn ch3enc(&mut self) -> CH3ENC_W {
        CH3ENC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Enable Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chenc](index.html) module"]
pub struct CHENC_SPEC;
impl crate::RegisterSpec for CHENC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [chenc::W](W) writer structure"]
impl crate::Writable for CHENC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHENC to value 0"]
impl crate::Resettable for CHENC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
