#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Current DAC Enable"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - Current DAC Enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `CURSINK` reader - Current Sink Enable"]
pub type CURSINK_R = crate::BitReader<bool>;
#[doc = "Field `CURSINK` writer - Current Sink Enable"]
pub type CURSINK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `MINOUTTRANS` reader - Minimum Output Transition Enable"]
pub type MINOUTTRANS_R = crate::BitReader<bool>;
#[doc = "Field `MINOUTTRANS` writer - Minimum Output Transition Enable"]
pub type MINOUTTRANS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `OUTEN` reader - Output Enable"]
pub type OUTEN_R = crate::BitReader<bool>;
#[doc = "Field `OUTEN` writer - Output Enable"]
pub type OUTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `OUTMODE` reader - Output Modes"]
pub type OUTMODE_R = crate::BitReader<bool>;
#[doc = "Field `OUTMODE` writer - Output Modes"]
pub type OUTMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `OUTENPRS` reader - PRS Controlled Output Enable"]
pub type OUTENPRS_R = crate::BitReader<bool>;
#[doc = "Field `OUTENPRS` writer - PRS Controlled Output Enable"]
pub type OUTENPRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `PRSSEL` reader - IDAC Output PRS channnel Select"]
pub type PRSSEL_R = crate::FieldReader<u8, PRSSEL_A>;
#[doc = "IDAC Output PRS channnel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSSEL_A {
    #[doc = "0: PRS Channel 0 selected."]
    PRSCH0 = 0,
    #[doc = "1: PRS Channel 1 selected."]
    PRSCH1 = 1,
    #[doc = "2: PRS Channel 2 selected."]
    PRSCH2 = 2,
    #[doc = "3: PRS Channel 3 selected."]
    PRSCH3 = 3,
}
impl From<PRSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSSEL_A) -> Self {
        variant as _
    }
}
impl PRSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRSSEL_A {
        match self.bits {
            0 => PRSSEL_A::PRSCH0,
            1 => PRSSEL_A::PRSCH1,
            2 => PRSSEL_A::PRSCH2,
            3 => PRSSEL_A::PRSCH3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSEL_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSEL_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSEL_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSEL_A::PRSCH3
    }
}
#[doc = "Field `PRSSEL` writer - IDAC Output PRS channnel Select"]
pub type PRSSEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, PRSSEL_A, 2, O>;
impl<'a, const O: u8> PRSSEL_W<'a, O> {
    #[doc = "PRS Channel 0 selected."]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected."]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected."]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected."]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH3)
    }
}
impl R {
    #[doc = "Bit 0 - Current DAC Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Current Sink Enable"]
    #[inline(always)]
    pub fn cursink(&self) -> CURSINK_R {
        CURSINK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Minimum Output Transition Enable"]
    #[inline(always)]
    pub fn minouttrans(&self) -> MINOUTTRANS_R {
        MINOUTTRANS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output Enable"]
    #[inline(always)]
    pub fn outen(&self) -> OUTEN_R {
        OUTEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Output Modes"]
    #[inline(always)]
    pub fn outmode(&self) -> OUTMODE_R {
        OUTMODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 18 - PRS Controlled Output Enable"]
    #[inline(always)]
    pub fn outenprs(&self) -> OUTENPRS_R {
        OUTENPRS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:21 - IDAC Output PRS channnel Select"]
    #[inline(always)]
    pub fn prssel(&self) -> PRSSEL_R {
        PRSSEL_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Current DAC Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - Current Sink Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cursink(&mut self) -> CURSINK_W<1> {
        CURSINK_W::new(self)
    }
    #[doc = "Bit 2 - Minimum Output Transition Enable"]
    #[inline(always)]
    #[must_use]
    pub fn minouttrans(&mut self) -> MINOUTTRANS_W<2> {
        MINOUTTRANS_W::new(self)
    }
    #[doc = "Bit 3 - Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn outen(&mut self) -> OUTEN_W<3> {
        OUTEN_W::new(self)
    }
    #[doc = "Bit 4 - Output Modes"]
    #[inline(always)]
    #[must_use]
    pub fn outmode(&mut self) -> OUTMODE_W<4> {
        OUTMODE_W::new(self)
    }
    #[doc = "Bit 18 - PRS Controlled Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn outenprs(&mut self) -> OUTENPRS_W<18> {
        OUTENPRS_W::new(self)
    }
    #[doc = "Bits 20:21 - IDAC Output PRS channnel Select"]
    #[inline(always)]
    #[must_use]
    pub fn prssel(&mut self) -> PRSSEL_W<20> {
        PRSSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
