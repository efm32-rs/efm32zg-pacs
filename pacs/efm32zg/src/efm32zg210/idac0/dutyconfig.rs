#[doc = "Register `DUTYCONFIG` reader"]
pub struct R(crate::R<DUTYCONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DUTYCONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DUTYCONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DUTYCONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DUTYCONFIG` writer"]
pub struct W(crate::W<DUTYCONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DUTYCONFIG_SPEC>;
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
impl From<crate::W<DUTYCONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DUTYCONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DUTYCYCLEEN` reader - Duty Cycle Enable."]
pub type DUTYCYCLEEN_R = crate::BitReader<bool>;
#[doc = "Field `DUTYCYCLEEN` writer - Duty Cycle Enable."]
pub type DUTYCYCLEEN_W<'a> = crate::BitWriter<'a, u32, DUTYCONFIG_SPEC, bool, 0>;
#[doc = "Field `EM2DUTYCYCLEDIS` reader - EM2/EM3 Duty Cycle Disable."]
pub type EM2DUTYCYCLEDIS_R = crate::BitReader<bool>;
#[doc = "Field `EM2DUTYCYCLEDIS` writer - EM2/EM3 Duty Cycle Disable."]
pub type EM2DUTYCYCLEDIS_W<'a> = crate::BitWriter<'a, u32, DUTYCONFIG_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 0 - Duty Cycle Enable."]
    #[inline(always)]
    pub fn dutycycleen(&self) -> DUTYCYCLEEN_R {
        DUTYCYCLEEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EM2/EM3 Duty Cycle Disable."]
    #[inline(always)]
    pub fn em2dutycycledis(&self) -> EM2DUTYCYCLEDIS_R {
        EM2DUTYCYCLEDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Duty Cycle Enable."]
    #[inline(always)]
    pub fn dutycycleen(&mut self) -> DUTYCYCLEEN_W {
        DUTYCYCLEEN_W::new(self)
    }
    #[doc = "Bit 1 - EM2/EM3 Duty Cycle Disable."]
    #[inline(always)]
    pub fn em2dutycycledis(&mut self) -> EM2DUTYCYCLEDIS_W {
        EM2DUTYCYCLEDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Duty Cycle Configauration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dutyconfig](index.html) module"]
pub struct DUTYCONFIG_SPEC;
impl crate::RegisterSpec for DUTYCONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dutyconfig::R](R) reader structure"]
impl crate::Readable for DUTYCONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dutyconfig::W](W) writer structure"]
impl crate::Writable for DUTYCONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DUTYCONFIG to value 0"]
impl crate::Resettable for DUTYCONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
