#[doc = "Register `APB2LPENR` reader"]
pub type R = crate::R<APB2LPENR_SPEC>;
#[doc = "Register `APB2LPENR` writer"]
pub type W = crate::W<APB2LPENR_SPEC>;
#[doc = "TIM1 clock enable during sleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM1LPEN_A {
    #[doc = "0: TIM1 peripheral clock disabled during sleep mode"]
    B_0x0 = 0,
    #[doc = "1: TIM1 peripheral clock enabled during sleep mode (default after reset)"]
    B_0x1 = 1,
}
impl From<TIM1LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM1LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM1LPEN` reader - TIM1 clock enable during sleep mode Set and reset by software."]
pub type TIM1LPEN_R = crate::BitReader<TIM1LPEN_A>;
impl TIM1LPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIM1LPEN_A {
        match self.bits {
            false => TIM1LPEN_A::B_0x0,
            true => TIM1LPEN_A::B_0x1,
        }
    }
    #[doc = "TIM1 peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TIM1LPEN_A::B_0x0
    }
    #[doc = "TIM1 peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TIM1LPEN_A::B_0x1
    }
}
#[doc = "Field `TIM1LPEN` writer - TIM1 clock enable during sleep mode Set and reset by software."]
pub type TIM1LPEN_W<'a, REG> = crate::BitWriter<'a, REG, TIM1LPEN_A>;
impl<'a, REG> TIM1LPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIM1 peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1LPEN_A::B_0x0)
    }
    #[doc = "TIM1 peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1LPEN_A::B_0x1)
    }
}
#[doc = "SPI1 clock enable during sleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI1LPEN_A {
    #[doc = "0: SPI1 peripheral clock disabled during sleep mode"]
    B_0x0 = 0,
    #[doc = "1: SPI1 peripheral clock enabled during sleep mode (default after reset)"]
    B_0x1 = 1,
}
impl From<SPI1LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: SPI1LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI1LPEN` reader - SPI1 clock enable during sleep mode Set and reset by software."]
pub type SPI1LPEN_R = crate::BitReader<SPI1LPEN_A>;
impl SPI1LPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPI1LPEN_A {
        match self.bits {
            false => SPI1LPEN_A::B_0x0,
            true => SPI1LPEN_A::B_0x1,
        }
    }
    #[doc = "SPI1 peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SPI1LPEN_A::B_0x0
    }
    #[doc = "SPI1 peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SPI1LPEN_A::B_0x1
    }
}
#[doc = "Field `SPI1LPEN` writer - SPI1 clock enable during sleep mode Set and reset by software."]
pub type SPI1LPEN_W<'a, REG> = crate::BitWriter<'a, REG, SPI1LPEN_A>;
impl<'a, REG> SPI1LPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI1 peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SPI1LPEN_A::B_0x0)
    }
    #[doc = "SPI1 peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SPI1LPEN_A::B_0x1)
    }
}
#[doc = "USART1 clock enable during sleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART1LPEN_A {
    #[doc = "0: USART1 peripheral clock disabled during sleep mode"]
    B_0x0 = 0,
    #[doc = "1: USART1 peripheral clock enabled during sleep mode (default after reset)"]
    B_0x1 = 1,
}
impl From<USART1LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: USART1LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USART1LPEN` reader - USART1 clock enable during sleep mode Set and reset by software."]
pub type USART1LPEN_R = crate::BitReader<USART1LPEN_A>;
impl USART1LPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USART1LPEN_A {
        match self.bits {
            false => USART1LPEN_A::B_0x0,
            true => USART1LPEN_A::B_0x1,
        }
    }
    #[doc = "USART1 peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == USART1LPEN_A::B_0x0
    }
    #[doc = "USART1 peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == USART1LPEN_A::B_0x1
    }
}
#[doc = "Field `USART1LPEN` writer - USART1 clock enable during sleep mode Set and reset by software."]
pub type USART1LPEN_W<'a, REG> = crate::BitWriter<'a, REG, USART1LPEN_A>;
impl<'a, REG> USART1LPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USART1 peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(USART1LPEN_A::B_0x0)
    }
    #[doc = "USART1 peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(USART1LPEN_A::B_0x1)
    }
}
#[doc = "USBFS clock enable during sleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBFSLPEN_A {
    #[doc = "0: USBFS peripheral clock disabled during sleep mode"]
    B_0x0 = 0,
    #[doc = "1: USBFS peripheral clock enabled during sleep mode (default after reset)"]
    B_0x1 = 1,
}
impl From<USBFSLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: USBFSLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBFSLPEN` reader - USBFS clock enable during sleep mode Set and reset by software."]
pub type USBFSLPEN_R = crate::BitReader<USBFSLPEN_A>;
impl USBFSLPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USBFSLPEN_A {
        match self.bits {
            false => USBFSLPEN_A::B_0x0,
            true => USBFSLPEN_A::B_0x1,
        }
    }
    #[doc = "USBFS peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == USBFSLPEN_A::B_0x0
    }
    #[doc = "USBFS peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == USBFSLPEN_A::B_0x1
    }
}
#[doc = "Field `USBFSLPEN` writer - USBFS clock enable during sleep mode Set and reset by software."]
pub type USBFSLPEN_W<'a, REG> = crate::BitWriter<'a, REG, USBFSLPEN_A>;
impl<'a, REG> USBFSLPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USBFS peripheral clock disabled during sleep mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(USBFSLPEN_A::B_0x0)
    }
    #[doc = "USBFS peripheral clock enabled during sleep mode (default after reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(USBFSLPEN_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 11 - TIM1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn TIM1LPEN(&self) -> TIM1LPEN_R {
        TIM1LPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn SPI1LPEN(&self) -> SPI1LPEN_R {
        SPI1LPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn USART1LPEN(&self) -> USART1LPEN_R {
        USART1LPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 24 - USBFS clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn USBFSLPEN(&self) -> USBFSLPEN_R {
        USBFSLPEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - TIM1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn TIM1LPEN(&mut self) -> TIM1LPEN_W<'_, APB2LPENR_SPEC> {
        TIM1LPEN_W::new(self, 11)
    }
    #[doc = "Bit 12 - SPI1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn SPI1LPEN(&mut self) -> SPI1LPEN_W<'_, APB2LPENR_SPEC> {
        SPI1LPEN_W::new(self, 12)
    }
    #[doc = "Bit 14 - USART1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn USART1LPEN(&mut self) -> USART1LPEN_W<'_, APB2LPENR_SPEC> {
        USART1LPEN_W::new(self, 14)
    }
    #[doc = "Bit 24 - USBFS clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn USBFSLPEN(&mut self) -> USBFSLPEN_W<'_, APB2LPENR_SPEC> {
        USBFSLPEN_W::new(self, 24)
    }
}
#[doc = "RCC APB2 sleep clock register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2LPENR_SPEC;
impl crate::RegisterSpec for APB2LPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2lpenr::R`](R) reader structure"]
impl crate::Readable for APB2LPENR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb2lpenr::W`](W) writer structure"]
impl crate::Writable for APB2LPENR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets APB2LPENR to value 0xffff_ffff"]
impl crate::Resettable for APB2LPENR_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
