#[doc = "Register `TXBCIE` reader"]
pub type R = crate::R<TXBCIE_SPEC>;
#[doc = "Register `TXBCIE` writer"]
pub type W = crate::W<TXBCIE_SPEC>;
#[doc = "Cancellation finished interrupt enable. Each Tx buffer has its own CFIE bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CFIE_A {
    #[doc = "0: Cancellation finished interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: Cancellation finished interrupt enabled"]
    B_0x1 = 1,
}
impl From<CFIE_A> for u8 {
    #[inline(always)]
    fn from(variant: CFIE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CFIE_A {
    type Ux = u8;
}
impl crate::IsEnum for CFIE_A {}
#[doc = "Field `CFIE` reader - Cancellation finished interrupt enable. Each Tx buffer has its own CFIE bit."]
pub type CFIE_R = crate::FieldReader<CFIE_A>;
impl CFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CFIE_A> {
        match self.bits {
            0 => Some(CFIE_A::B_0x0),
            1 => Some(CFIE_A::B_0x1),
            _ => None,
        }
    }
    #[doc = "Cancellation finished interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CFIE_A::B_0x0
    }
    #[doc = "Cancellation finished interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CFIE_A::B_0x1
    }
}
#[doc = "Field `CFIE` writer - Cancellation finished interrupt enable. Each Tx buffer has its own CFIE bit."]
pub type CFIE_W<'a, REG> = crate::FieldWriter<'a, REG, 3, CFIE_A>;
impl<'a, REG> CFIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Cancellation finished interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CFIE_A::B_0x0)
    }
    #[doc = "Cancellation finished interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CFIE_A::B_0x1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Cancellation finished interrupt enable. Each Tx buffer has its own CFIE bit."]
    #[inline(always)]
    pub fn CFIE(&self) -> CFIE_R {
        CFIE_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Cancellation finished interrupt enable. Each Tx buffer has its own CFIE bit."]
    #[inline(always)]
    pub fn CFIE(&mut self) -> CFIE_W<'_, TXBCIE_SPEC> {
        CFIE_W::new(self, 0)
    }
}
#[doc = "FDCAN Tx buffer cancellation finished interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`txbcie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbcie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXBCIE_SPEC;
impl crate::RegisterSpec for TXBCIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbcie::R`](R) reader structure"]
impl crate::Readable for TXBCIE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txbcie::W`](W) writer structure"]
impl crate::Writable for TXBCIE_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TXBCIE to value 0"]
impl crate::Resettable for TXBCIE_SPEC {}
