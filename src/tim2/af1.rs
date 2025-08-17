#[doc = "Register `AF1` reader"]
pub type R = crate::R<AF1_SPEC>;
#[doc = "Register `AF1` writer"]
pub type W = crate::W<AF1_SPEC>;
#[doc = "etr_in source selection These bits select the etr_in input source. ... Refer to for product specific implementation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ETRSEL_A {
    #[doc = "0: tim_etr0: TIMx_ETR input"]
    B_0x0 = 0,
    #[doc = "1: tim_etr1"]
    B_0x1 = 1,
    #[doc = "15: tim_etr15"]
    B_0xF = 15,
}
impl From<ETRSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ETRSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ETRSEL_A {
    type Ux = u8;
}
impl crate::IsEnum for ETRSEL_A {}
#[doc = "Field `ETRSEL` reader - etr_in source selection These bits select the etr_in input source. ... Refer to for product specific implementation."]
pub type ETRSEL_R = crate::FieldReader<ETRSEL_A>;
impl ETRSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ETRSEL_A> {
        match self.bits {
            0 => Some(ETRSEL_A::B_0x0),
            1 => Some(ETRSEL_A::B_0x1),
            15 => Some(ETRSEL_A::B_0xF),
            _ => None,
        }
    }
    #[doc = "tim_etr0: TIMx_ETR input"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ETRSEL_A::B_0x0
    }
    #[doc = "tim_etr1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ETRSEL_A::B_0x1
    }
    #[doc = "tim_etr15"]
    #[inline(always)]
    pub fn is_B_0xF(&self) -> bool {
        *self == ETRSEL_A::B_0xF
    }
}
#[doc = "Field `ETRSEL` writer - etr_in source selection These bits select the etr_in input source. ... Refer to for product specific implementation."]
pub type ETRSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, ETRSEL_A>;
impl<'a, REG> ETRSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "tim_etr0: TIMx_ETR input"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ETRSEL_A::B_0x0)
    }
    #[doc = "tim_etr1"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ETRSEL_A::B_0x1)
    }
    #[doc = "tim_etr15"]
    #[inline(always)]
    pub fn B_0xF(self) -> &'a mut crate::W<REG> {
        self.variant(ETRSEL_A::B_0xF)
    }
}
impl R {
    #[doc = "Bits 14:17 - etr_in source selection These bits select the etr_in input source. ... Refer to for product specific implementation."]
    #[inline(always)]
    pub fn ETRSEL(&self) -> ETRSEL_R {
        ETRSEL_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 14:17 - etr_in source selection These bits select the etr_in input source. ... Refer to for product specific implementation."]
    #[inline(always)]
    pub fn ETRSEL(&mut self) -> ETRSEL_W<'_, AF1_SPEC> {
        ETRSEL_W::new(self, 14)
    }
}
#[doc = "TIM2 alternate function register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`af1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AF1_SPEC;
impl crate::RegisterSpec for AF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`af1::R`](R) reader structure"]
impl crate::Readable for AF1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`af1::W`](W) writer structure"]
impl crate::Writable for AF1_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets AF1 to value 0"]
impl crate::Resettable for AF1_SPEC {}
