#[doc = "Register `AF2` reader"]
pub type R = crate::R<AF2_SPEC>;
#[doc = "Register `AF2` writer"]
pub type W = crate::W<AF2_SPEC>;
#[doc = "ocref_clr source selection These bits select the ocref_clr input source. ... Refer to for product specific implementation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OCRSEL_A {
    #[doc = "0: tim_ocref_clr0"]
    B_0x0 = 0,
    #[doc = "1: tim_ocref_clr1"]
    B_0x1 = 1,
    #[doc = "7: tim_ocref_clr7"]
    B_0x7 = 7,
}
impl From<OCRSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: OCRSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OCRSEL_A {
    type Ux = u8;
}
impl crate::IsEnum for OCRSEL_A {}
#[doc = "Field `OCRSEL` reader - ocref_clr source selection These bits select the ocref_clr input source. ... Refer to for product specific implementation."]
pub type OCRSEL_R = crate::FieldReader<OCRSEL_A>;
impl OCRSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<OCRSEL_A> {
        match self.bits {
            0 => Some(OCRSEL_A::B_0x0),
            1 => Some(OCRSEL_A::B_0x1),
            7 => Some(OCRSEL_A::B_0x7),
            _ => None,
        }
    }
    #[doc = "tim_ocref_clr0"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OCRSEL_A::B_0x0
    }
    #[doc = "tim_ocref_clr1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OCRSEL_A::B_0x1
    }
    #[doc = "tim_ocref_clr7"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == OCRSEL_A::B_0x7
    }
}
#[doc = "Field `OCRSEL` writer - ocref_clr source selection These bits select the ocref_clr input source. ... Refer to for product specific implementation."]
pub type OCRSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, OCRSEL_A>;
impl<'a, REG> OCRSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "tim_ocref_clr0"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OCRSEL_A::B_0x0)
    }
    #[doc = "tim_ocref_clr1"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OCRSEL_A::B_0x1)
    }
    #[doc = "tim_ocref_clr7"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(OCRSEL_A::B_0x7)
    }
}
impl R {
    #[doc = "Bits 16:18 - ocref_clr source selection These bits select the ocref_clr input source. ... Refer to for product specific implementation."]
    #[inline(always)]
    pub fn OCRSEL(&self) -> OCRSEL_R {
        OCRSEL_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 16:18 - ocref_clr source selection These bits select the ocref_clr input source. ... Refer to for product specific implementation."]
    #[inline(always)]
    pub fn OCRSEL(&mut self) -> OCRSEL_W<'_, AF2_SPEC> {
        OCRSEL_W::new(self, 16)
    }
}
#[doc = "TIM2 alternate function register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`af2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AF2_SPEC;
impl crate::RegisterSpec for AF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`af2::R`](R) reader structure"]
impl crate::Readable for AF2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`af2::W`](W) writer structure"]
impl crate::Writable for AF2_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets AF2 to value 0"]
impl crate::Resettable for AF2_SPEC {}
