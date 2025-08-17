#[doc = "Register `APB1HENR` reader"]
pub type R = crate::R<APB1HENR_SPEC>;
#[doc = "Register `APB1HENR` writer"]
pub type W = crate::W<APB1HENR_SPEC>;
#[doc = "DTS clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTSEN_A {
    #[doc = "0: DTS peripheral clock disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: DTS peripheral clock enabled"]
    B_0x1 = 1,
}
impl From<DTSEN_A> for bool {
    #[inline(always)]
    fn from(variant: DTSEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTSEN` reader - DTS clock enable Set and reset by software."]
pub type DTSEN_R = crate::BitReader<DTSEN_A>;
impl DTSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DTSEN_A {
        match self.bits {
            false => DTSEN_A::B_0x0,
            true => DTSEN_A::B_0x1,
        }
    }
    #[doc = "DTS peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DTSEN_A::B_0x0
    }
    #[doc = "DTS peripheral clock enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DTSEN_A::B_0x1
    }
}
#[doc = "Field `DTSEN` writer - DTS clock enable Set and reset by software."]
pub type DTSEN_W<'a, REG> = crate::BitWriter<'a, REG, DTSEN_A>;
impl<'a, REG> DTSEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DTS peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DTSEN_A::B_0x0)
    }
    #[doc = "DTS peripheral clock enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DTSEN_A::B_0x1)
    }
}
#[doc = "LPTIM2 clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPTIM2EN_A {
    #[doc = "0: LPTIM2 peripheral clock disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: LPTIM2 peripheral clock enabled"]
    B_0x1 = 1,
}
impl From<LPTIM2EN_A> for bool {
    #[inline(always)]
    fn from(variant: LPTIM2EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPTIM2EN` reader - LPTIM2 clock enable Set and reset by software."]
pub type LPTIM2EN_R = crate::BitReader<LPTIM2EN_A>;
impl LPTIM2EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPTIM2EN_A {
        match self.bits {
            false => LPTIM2EN_A::B_0x0,
            true => LPTIM2EN_A::B_0x1,
        }
    }
    #[doc = "LPTIM2 peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LPTIM2EN_A::B_0x0
    }
    #[doc = "LPTIM2 peripheral clock enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LPTIM2EN_A::B_0x1
    }
}
#[doc = "Field `LPTIM2EN` writer - LPTIM2 clock enable Set and reset by software."]
pub type LPTIM2EN_W<'a, REG> = crate::BitWriter<'a, REG, LPTIM2EN_A>;
impl<'a, REG> LPTIM2EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LPTIM2 peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM2EN_A::B_0x0)
    }
    #[doc = "LPTIM2 peripheral clock enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM2EN_A::B_0x1)
    }
}
#[doc = "FDCAN1 peripheral clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FDCAN1EN_A {
    #[doc = "0: FDCAN1 peripheral clock disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: FDCAN1 peripheral clock enabled"]
    B_0x1 = 1,
}
impl From<FDCAN1EN_A> for bool {
    #[inline(always)]
    fn from(variant: FDCAN1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FDCAN1EN` reader - FDCAN1 peripheral clock enable Set and reset by software."]
pub type FDCAN1EN_R = crate::BitReader<FDCAN1EN_A>;
impl FDCAN1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FDCAN1EN_A {
        match self.bits {
            false => FDCAN1EN_A::B_0x0,
            true => FDCAN1EN_A::B_0x1,
        }
    }
    #[doc = "FDCAN1 peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FDCAN1EN_A::B_0x0
    }
    #[doc = "FDCAN1 peripheral clock enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FDCAN1EN_A::B_0x1
    }
}
#[doc = "Field `FDCAN1EN` writer - FDCAN1 peripheral clock enable Set and reset by software."]
pub type FDCAN1EN_W<'a, REG> = crate::BitWriter<'a, REG, FDCAN1EN_A>;
impl<'a, REG> FDCAN1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FDCAN1 peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FDCAN1EN_A::B_0x0)
    }
    #[doc = "FDCAN1 peripheral clock enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FDCAN1EN_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 3 - DTS clock enable Set and reset by software."]
    #[inline(always)]
    pub fn DTSEN(&self) -> DTSEN_R {
        DTSEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - LPTIM2 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn LPTIM2EN(&self) -> LPTIM2EN_R {
        LPTIM2EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - FDCAN1 peripheral clock enable Set and reset by software."]
    #[inline(always)]
    pub fn FDCAN1EN(&self) -> FDCAN1EN_R {
        FDCAN1EN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - DTS clock enable Set and reset by software."]
    #[inline(always)]
    pub fn DTSEN(&mut self) -> DTSEN_W<'_, APB1HENR_SPEC> {
        DTSEN_W::new(self, 3)
    }
    #[doc = "Bit 5 - LPTIM2 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn LPTIM2EN(&mut self) -> LPTIM2EN_W<'_, APB1HENR_SPEC> {
        LPTIM2EN_W::new(self, 5)
    }
    #[doc = "Bit 9 - FDCAN1 peripheral clock enable Set and reset by software."]
    #[inline(always)]
    pub fn FDCAN1EN(&mut self) -> FDCAN1EN_W<'_, APB1HENR_SPEC> {
        FDCAN1EN_W::new(self, 9)
    }
}
#[doc = "RCC APB1 peripheral clock register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1henr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1henr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1HENR_SPEC;
impl crate::RegisterSpec for APB1HENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1henr::R`](R) reader structure"]
impl crate::Readable for APB1HENR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb1henr::W`](W) writer structure"]
impl crate::Writable for APB1HENR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets APB1HENR to value 0"]
impl crate::Resettable for APB1HENR_SPEC {}
