#[doc = "Register `CHSREQSTATUS` reader"]
pub struct R(crate::R<CHSREQSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHSREQSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHSREQSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHSREQSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CH0SREQSTATUS` reader - Channel 0 Single Request Status"]
pub type CH0SREQSTATUS_R = crate::BitReader<bool>;
#[doc = "Field `CH1SREQSTATUS` reader - Channel 1 Single Request Status"]
pub type CH1SREQSTATUS_R = crate::BitReader<bool>;
#[doc = "Field `CH2SREQSTATUS` reader - Channel 2 Single Request Status"]
pub type CH2SREQSTATUS_R = crate::BitReader<bool>;
#[doc = "Field `CH3SREQSTATUS` reader - Channel 3 Single Request Status"]
pub type CH3SREQSTATUS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Channel 0 Single Request Status"]
    #[inline(always)]
    pub fn ch0sreqstatus(&self) -> CH0SREQSTATUS_R {
        CH0SREQSTATUS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Single Request Status"]
    #[inline(always)]
    pub fn ch1sreqstatus(&self) -> CH1SREQSTATUS_R {
        CH1SREQSTATUS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Single Request Status"]
    #[inline(always)]
    pub fn ch2sreqstatus(&self) -> CH2SREQSTATUS_R {
        CH2SREQSTATUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Single Request Status"]
    #[inline(always)]
    pub fn ch3sreqstatus(&self) -> CH3SREQSTATUS_R {
        CH3SREQSTATUS_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Channel Single Request Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chsreqstatus](index.html) module"]
pub struct CHSREQSTATUS_SPEC;
impl crate::RegisterSpec for CHSREQSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chsreqstatus::R](R) reader structure"]
impl crate::Readable for CHSREQSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CHSREQSTATUS to value 0"]
impl crate::Resettable for CHSREQSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
