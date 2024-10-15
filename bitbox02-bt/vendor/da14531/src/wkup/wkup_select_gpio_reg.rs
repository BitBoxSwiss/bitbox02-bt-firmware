#[doc = "Register `WKUP_SELECT_GPIO_REG` reader"]
pub struct R(crate::R<WKUP_SELECT_GPIO_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WKUP_SELECT_GPIO_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WKUP_SELECT_GPIO_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WKUP_SELECT_GPIO_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WKUP_SELECT_GPIO_REG` writer"]
pub struct W(crate::W<WKUP_SELECT_GPIO_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WKUP_SELECT_GPIO_REG_SPEC>;
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
impl From<crate::W<WKUP_SELECT_GPIO_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WKUP_SELECT_GPIO_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WKUP_SELECT_GPIO` reader - 0 = input P0x is not enabled for wakeup event counter 1 = input P0x is enabled for wakeup event counter"]
pub struct WKUP_SELECT_GPIO_R(crate::FieldReader<u16, u16>);
impl WKUP_SELECT_GPIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        WKUP_SELECT_GPIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUP_SELECT_GPIO_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUP_SELECT_GPIO` writer - 0 = input P0x is not enabled for wakeup event counter 1 = input P0x is enabled for wakeup event counter"]
pub struct WKUP_SELECT_GPIO_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUP_SELECT_GPIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u16 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - 0 = input P0x is not enabled for wakeup event counter 1 = input P0x is enabled for wakeup event counter"]
    #[inline(always)]
    pub fn wkup_select_gpio(&self) -> WKUP_SELECT_GPIO_R {
        WKUP_SELECT_GPIO_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - 0 = input P0x is not enabled for wakeup event counter 1 = input P0x is enabled for wakeup event counter"]
    #[inline(always)]
    pub fn wkup_select_gpio(&mut self) -> WKUP_SELECT_GPIO_W {
        WKUP_SELECT_GPIO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Select which inputs from P0 port can trigger wkup counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wkup_select_gpio_reg](index.html) module"]
pub struct WKUP_SELECT_GPIO_REG_SPEC;
impl crate::RegisterSpec for WKUP_SELECT_GPIO_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [wkup_select_gpio_reg::R](R) reader structure"]
impl crate::Readable for WKUP_SELECT_GPIO_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wkup_select_gpio_reg::W](W) writer structure"]
impl crate::Writable for WKUP_SELECT_GPIO_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WKUP_SELECT_GPIO_REG to value 0"]
impl crate::Resettable for WKUP_SELECT_GPIO_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
