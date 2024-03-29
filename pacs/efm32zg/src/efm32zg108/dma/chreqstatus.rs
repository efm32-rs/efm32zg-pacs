#[doc = "Register `CHREQSTATUS` reader"]
pub struct R(crate::R<CHREQSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHREQSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHREQSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHREQSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CH0REQSTATUS` reader - Channel 0 Request Status"]
pub type CH0REQSTATUS_R = crate::BitReader<bool>;
#[doc = "Field `CH1REQSTATUS` reader - Channel 1 Request Status"]
pub type CH1REQSTATUS_R = crate::BitReader<bool>;
#[doc = "Field `CH2REQSTATUS` reader - Channel 2 Request Status"]
pub type CH2REQSTATUS_R = crate::BitReader<bool>;
#[doc = "Field `CH3REQSTATUS` reader - Channel 3 Request Status"]
pub type CH3REQSTATUS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Channel 0 Request Status"]
    #[inline(always)]
    pub fn ch0reqstatus(&self) -> CH0REQSTATUS_R {
        CH0REQSTATUS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Request Status"]
    #[inline(always)]
    pub fn ch1reqstatus(&self) -> CH1REQSTATUS_R {
        CH1REQSTATUS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Request Status"]
    #[inline(always)]
    pub fn ch2reqstatus(&self) -> CH2REQSTATUS_R {
        CH2REQSTATUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Request Status"]
    #[inline(always)]
    pub fn ch3reqstatus(&self) -> CH3REQSTATUS_R {
        CH3REQSTATUS_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Channel Request Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chreqstatus](index.html) module"]
pub struct CHREQSTATUS_SPEC;
impl crate::RegisterSpec for CHREQSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chreqstatus::R](R) reader structure"]
impl crate::Readable for CHREQSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CHREQSTATUS to value 0"]
impl crate::Resettable for CHREQSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
