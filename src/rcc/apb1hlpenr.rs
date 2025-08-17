#[doc = "Register `APB1HLPENR` reader"]
pub type R = crate::R<APB1HLPENR_SPEC>;
#[doc = "Register `APB1HLPENR` writer"]
pub type W = crate::W<APB1HLPENR_SPEC>;
#[doc = "DTS clock enable during sleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTSLPEN_A {
    #[doc = "0: DTS peripheral clock disabled during sleep mode"]
    B_0x0 = 0,
    #[doc = "1: DTS peripheral clock enabled during sleep mode (default after reset)"]
    B_0x1 = 1,
}
impl From<DTSLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: DTSLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTSLPEN` reader - DTS clock enable during sleep mode Set and reset by software."]
pub type DTSLPEN_R = crate::BitReader<DTSLPEN_A>;
impl DTSLPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DTSLPEN_A {
        match self.bits {
            false => DTSLPEN_A::B_0x0,
            true => DTSLPEN_A::B_0x1,
        }
    }
    #[doc = "DTS peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DTSLPEN_A::B_0x0
    }
    #[doc = "DTS peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DTSLPEN_A::B_0x1
    }
}
#[doc = "Field `DTSLPEN` writer - DTS clock enable during sleep mode Set and reset by software."]
pub type DTSLPEN_W<'a, REG> = crate::BitWriter<'a, REG, DTSLPEN_A>;
impl<'a, REG> DTSLPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DTS peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DTSLPEN_A::B_0x0)
    }
    #[doc = "DTS peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DTSLPEN_A::B_0x1)
    }
}
#[doc = "LPTIM2 clock enable during sleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPTIM2LPEN_A {
    #[doc = "0: LPTIM2 peripheral clock disabled during sleep mode"]
    B_0x0 = 0,
    #[doc = "1: LPTIM2 peripheral clock enabled during sleep mode (default after reset)"]
    B_0x1 = 1,
}
impl From<LPTIM2LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: LPTIM2LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPTIM2LPEN` reader - LPTIM2 clock enable during sleep mode Set and reset by software."]
pub type LPTIM2LPEN_R = crate::BitReader<LPTIM2LPEN_A>;
impl LPTIM2LPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPTIM2LPEN_A {
        match self.bits {
            false => LPTIM2LPEN_A::B_0x0,
            true => LPTIM2LPEN_A::B_0x1,
        }
    }
    #[doc = "LPTIM2 peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LPTIM2LPEN_A::B_0x0
    }
    #[doc = "LPTIM2 peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LPTIM2LPEN_A::B_0x1
    }
}
#[doc = "Field `LPTIM2LPEN` writer - LPTIM2 clock enable during sleep mode Set and reset by software."]
pub type LPTIM2LPEN_W<'a, REG> = crate::BitWriter<'a, REG, LPTIM2LPEN_A>;
impl<'a, REG> LPTIM2LPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LPTIM2 peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM2LPEN_A::B_0x0)
    }
    #[doc = "LPTIM2 peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM2LPEN_A::B_0x1)
    }
}
#[doc = "FDCAN1 peripheral clock enable during sleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FDCAN1LPEN_A {
    #[doc = "0: FDCAN1 peripheral clock disabled during sleep mode"]
    B_0x0 = 0,
    #[doc = "1: FDCAN1 peripheral clock enabled during sleep mode (default after reset)"]
    B_0x1 = 1,
}
impl From<FDCAN1LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: FDCAN1LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FDCAN1LPEN` reader - FDCAN1 peripheral clock enable during sleep mode Set and reset by software."]
pub type FDCAN1LPEN_R = crate::BitReader<FDCAN1LPEN_A>;
impl FDCAN1LPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FDCAN1LPEN_A {
        match self.bits {
            false => FDCAN1LPEN_A::B_0x0,
            true => FDCAN1LPEN_A::B_0x1,
        }
    }
    #[doc = "FDCAN1 peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FDCAN1LPEN_A::B_0x0
    }
    #[doc = "FDCAN1 peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FDCAN1LPEN_A::B_0x1
    }
}
#[doc = "Field `FDCAN1LPEN` writer - FDCAN1 peripheral clock enable during sleep mode Set and reset by software."]
pub type FDCAN1LPEN_W<'a, REG> = crate::BitWriter<'a, REG, FDCAN1LPEN_A>;
impl<'a, REG> FDCAN1LPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FDCAN1 peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FDCAN1LPEN_A::B_0x0)
    }
    #[doc = "FDCAN1 peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FDCAN1LPEN_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 3 - DTS clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn DTSLPEN(&self) -> DTSLPEN_R {
        DTSLPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - LPTIM2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn LPTIM2LPEN(&self) -> LPTIM2LPEN_R {
        LPTIM2LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - FDCAN1 peripheral clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn FDCAN1LPEN(&self) -> FDCAN1LPEN_R {
        FDCAN1LPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - DTS clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn DTSLPEN(&mut self) -> DTSLPEN_W<'_, APB1HLPENR_SPEC> {
        DTSLPEN_W::new(self, 3)
    }
    #[doc = "Bit 5 - LPTIM2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn LPTIM2LPEN(&mut self) -> LPTIM2LPEN_W<'_, APB1HLPENR_SPEC> {
        LPTIM2LPEN_W::new(self, 5)
    }
    #[doc = "Bit 9 - FDCAN1 peripheral clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn FDCAN1LPEN(&mut self) -> FDCAN1LPEN_W<'_, APB1HLPENR_SPEC> {
        FDCAN1LPEN_W::new(self, 9)
    }
}
#[doc = "RCC APB1 sleep clock register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1hlpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1hlpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1HLPENR_SPEC;
impl crate::RegisterSpec for APB1HLPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1hlpenr::R`](R) reader structure"]
impl crate::Readable for APB1HLPENR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb1hlpenr::W`](W) writer structure"]
impl crate::Writable for APB1HLPENR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets APB1HLPENR to value 0xffff_ffff"]
impl crate::Resettable for APB1HLPENR_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
