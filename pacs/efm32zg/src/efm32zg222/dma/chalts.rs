#[doc = "Register `CHALTS` writer"]
pub struct W(crate::W<CHALTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHALTS_SPEC>;
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
impl From<crate::W<CHALTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHALTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0ALTS` writer - Channel 0 Alternate Structure Set"]
pub type CH0ALTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHALTS_SPEC, bool, O>;
#[doc = "Field `CH1ALTS` writer - Channel 1 Alternate Structure Set"]
pub type CH1ALTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHALTS_SPEC, bool, O>;
#[doc = "Field `CH2ALTS` writer - Channel 2 Alternate Structure Set"]
pub type CH2ALTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHALTS_SPEC, bool, O>;
#[doc = "Field `CH3ALTS` writer - Channel 3 Alternate Structure Set"]
pub type CH3ALTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHALTS_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Channel 0 Alternate Structure Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch0alts(&mut self) -> CH0ALTS_W<0> {
        CH0ALTS_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Alternate Structure Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch1alts(&mut self) -> CH1ALTS_W<1> {
        CH1ALTS_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 Alternate Structure Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch2alts(&mut self) -> CH2ALTS_W<2> {
        CH2ALTS_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 Alternate Structure Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch3alts(&mut self) -> CH3ALTS_W<3> {
        CH3ALTS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Alternate Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chalts](index.html) module"]
pub struct CHALTS_SPEC;
impl crate::RegisterSpec for CHALTS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [chalts::W](W) writer structure"]
impl crate::Writable for CHALTS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHALTS to value 0"]
impl crate::Resettable for CHALTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
