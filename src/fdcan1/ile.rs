#[doc = "Register `ILE` reader"]
pub type R = crate::R<ILE_SPEC>;
#[doc = "Register `ILE` writer"]
pub type W = crate::W<ILE_SPEC>;
#[doc = "Enable interrupt line 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EINT0_A {
    #[doc = "0: Interrupt line fdcan_intr1_it disabled"]
    B_0x0 = 0,
    #[doc = "1: Interrupt line fdcan_intr1_it enabled"]
    B_0x1 = 1,
}
impl From<EINT0_A> for bool {
    #[inline(always)]
    fn from(variant: EINT0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EINT0` reader - Enable interrupt line 0"]
pub type EINT0_R = crate::BitReader<EINT0_A>;
impl EINT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EINT0_A {
        match self.bits {
            false => EINT0_A::B_0x0,
            true => EINT0_A::B_0x1,
        }
    }
    #[doc = "Interrupt line fdcan_intr1_it disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EINT0_A::B_0x0
    }
    #[doc = "Interrupt line fdcan_intr1_it enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EINT0_A::B_0x1
    }
}
#[doc = "Field `EINT0` writer - Enable interrupt line 0"]
pub type EINT0_W<'a, REG> = crate::BitWriter<'a, REG, EINT0_A>;
impl<'a, REG> EINT0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt line fdcan_intr1_it disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EINT0_A::B_0x0)
    }
    #[doc = "Interrupt line fdcan_intr1_it enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EINT0_A::B_0x1)
    }
}
#[doc = "Enable interrupt line 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EINT1_A {
    #[doc = "0: Interrupt line fdcan_intr0_it disabled"]
    B_0x0 = 0,
    #[doc = "1: Interrupt line fdcan_intr0_it enabled"]
    B_0x1 = 1,
}
impl From<EINT1_A> for bool {
    #[inline(always)]
    fn from(variant: EINT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EINT1` reader - Enable interrupt line 1"]
pub type EINT1_R = crate::BitReader<EINT1_A>;
impl EINT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EINT1_A {
        match self.bits {
            false => EINT1_A::B_0x0,
            true => EINT1_A::B_0x1,
        }
    }
    #[doc = "Interrupt line fdcan_intr0_it disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EINT1_A::B_0x0
    }
    #[doc = "Interrupt line fdcan_intr0_it enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EINT1_A::B_0x1
    }
}
#[doc = "Field `EINT1` writer - Enable interrupt line 1"]
pub type EINT1_W<'a, REG> = crate::BitWriter<'a, REG, EINT1_A>;
impl<'a, REG> EINT1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt line fdcan_intr0_it disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EINT1_A::B_0x0)
    }
    #[doc = "Interrupt line fdcan_intr0_it enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EINT1_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Enable interrupt line 0"]
    #[inline(always)]
    pub fn EINT0(&self) -> EINT0_R {
        EINT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable interrupt line 1"]
    #[inline(always)]
    pub fn EINT1(&self) -> EINT1_R {
        EINT1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable interrupt line 0"]
    #[inline(always)]
    pub fn EINT0(&mut self) -> EINT0_W<'_, ILE_SPEC> {
        EINT0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable interrupt line 1"]
    #[inline(always)]
    pub fn EINT1(&mut self) -> EINT1_W<'_, ILE_SPEC> {
        EINT1_W::new(self, 1)
    }
}
#[doc = "FDCAN interrupt line enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ile::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ile::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ILE_SPEC;
impl crate::RegisterSpec for ILE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ile::R`](R) reader structure"]
impl crate::Readable for ILE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ile::W`](W) writer structure"]
impl crate::Writable for ILE_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ILE to value 0"]
impl crate::Resettable for ILE_SPEC {}
