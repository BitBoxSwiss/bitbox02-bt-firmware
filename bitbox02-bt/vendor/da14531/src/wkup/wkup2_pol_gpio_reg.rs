#[doc = "Register `WKUP2_POL_GPIO_REG` reader"]
pub struct R(crate::R<WKUP2_POL_GPIO_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WKUP2_POL_GPIO_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WKUP2_POL_GPIO_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WKUP2_POL_GPIO_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WKUP2_POL_GPIO_REG` writer"]
pub struct W(crate::W<WKUP2_POL_GPIO_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WKUP2_POL_GPIO_REG_SPEC>;
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
impl From<crate::W<WKUP2_POL_GPIO_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WKUP2_POL_GPIO_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WKUP2_POL_GPIO` reader - 0 = the enabled input P0x increments the event2 counter if that input goes high 1 = the enabled input P0x increments the event2 counter if that input goes low"]
pub struct WKUP2_POL_GPIO_R(crate::FieldReader<u16, u16>);
impl WKUP2_POL_GPIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        WKUP2_POL_GPIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUP2_POL_GPIO_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUP2_POL_GPIO` writer - 0 = the enabled input P0x increments the event2 counter if that input goes high 1 = the enabled input P0x increments the event2 counter if that input goes low"]
pub struct WKUP2_POL_GPIO_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUP2_POL_GPIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u16 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - 0 = the enabled input P0x increments the event2 counter if that input goes high 1 = the enabled input P0x increments the event2 counter if that input goes low"]
    #[inline(always)]
    pub fn wkup2_pol_gpio(&self) -> WKUP2_POL_GPIO_R {
        WKUP2_POL_GPIO_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - 0 = the enabled input P0x increments the event2 counter if that input goes high 1 = the enabled input P0x increments the event2 counter if that input goes low"]
    #[inline(always)]
    pub fn wkup2_pol_gpio(&mut self) -> WKUP2_POL_GPIO_W {
        WKUP2_POL_GPIO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Select the sensitivity polarity for each P1 input\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wkup2_pol_gpio_reg](index.html) module"]
pub struct WKUP2_POL_GPIO_REG_SPEC;
impl crate::RegisterSpec for WKUP2_POL_GPIO_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [wkup2_pol_gpio_reg::R](R) reader structure"]
impl crate::Readable for WKUP2_POL_GPIO_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wkup2_pol_gpio_reg::W](W) writer structure"]
impl crate::Writable for WKUP2_POL_GPIO_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WKUP2_POL_GPIO_REG to value 0"]
impl crate::Resettable for WKUP2_POL_GPIO_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
