#[doc = "Register `APB2ENR` reader"]
pub type R = crate::R<APB2ENR_SPEC>;
#[doc = "Register `APB2ENR` writer"]
pub type W = crate::W<APB2ENR_SPEC>;
#[doc = "TIM1 clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM1EN_A {
    #[doc = "0: TIM1 peripheral clock disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: TIM1 peripheral clock enabled"]
    B_0x1 = 1,
}
impl From<TIM1EN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM1EN` reader - TIM1 clock enable Set and reset by software."]
pub type TIM1EN_R = crate::BitReader<TIM1EN_A>;
impl TIM1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIM1EN_A {
        match self.bits {
            false => TIM1EN_A::B_0x0,
            true => TIM1EN_A::B_0x1,
        }
    }
    #[doc = "TIM1 peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TIM1EN_A::B_0x0
    }
    #[doc = "TIM1 peripheral clock enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TIM1EN_A::B_0x1
    }
}
#[doc = "Field `TIM1EN` writer - TIM1 clock enable Set and reset by software."]
pub type TIM1EN_W<'a, REG> = crate::BitWriter<'a, REG, TIM1EN_A>;
impl<'a, REG> TIM1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIM1 peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1EN_A::B_0x0)
    }
    #[doc = "TIM1 peripheral clock enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1EN_A::B_0x1)
    }
}
#[doc = "SPI1 clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI1EN_A {
    #[doc = "0: SPI1 peripheral clock disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: SPI1 peripheral clock enabled"]
    B_0x1 = 1,
}
impl From<SPI1EN_A> for bool {
    #[inline(always)]
    fn from(variant: SPI1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI1EN` reader - SPI1 clock enable Set and reset by software."]
pub type SPI1EN_R = crate::BitReader<SPI1EN_A>;
impl SPI1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPI1EN_A {
        match self.bits {
            false => SPI1EN_A::B_0x0,
            true => SPI1EN_A::B_0x1,
        }
    }
    #[doc = "SPI1 peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SPI1EN_A::B_0x0
    }
    #[doc = "SPI1 peripheral clock enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SPI1EN_A::B_0x1
    }
}
#[doc = "Field `SPI1EN` writer - SPI1 clock enable Set and reset by software."]
pub type SPI1EN_W<'a, REG> = crate::BitWriter<'a, REG, SPI1EN_A>;
impl<'a, REG> SPI1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI1 peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SPI1EN_A::B_0x0)
    }
    #[doc = "SPI1 peripheral clock enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SPI1EN_A::B_0x1)
    }
}
#[doc = "USART1 clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART1EN_A {
    #[doc = "0: USART1 peripheral clock disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: USART1 peripheral clock enabled"]
    B_0x1 = 1,
}
impl From<USART1EN_A> for bool {
    #[inline(always)]
    fn from(variant: USART1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USART1EN` reader - USART1 clock enable Set and reset by software."]
pub type USART1EN_R = crate::BitReader<USART1EN_A>;
impl USART1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USART1EN_A {
        match self.bits {
            false => USART1EN_A::B_0x0,
            true => USART1EN_A::B_0x1,
        }
    }
    #[doc = "USART1 peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == USART1EN_A::B_0x0
    }
    #[doc = "USART1 peripheral clock enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == USART1EN_A::B_0x1
    }
}
#[doc = "Field `USART1EN` writer - USART1 clock enable Set and reset by software."]
pub type USART1EN_W<'a, REG> = crate::BitWriter<'a, REG, USART1EN_A>;
impl<'a, REG> USART1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USART1 peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(USART1EN_A::B_0x0)
    }
    #[doc = "USART1 peripheral clock enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(USART1EN_A::B_0x1)
    }
}
#[doc = "USBFS clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBFSEN_A {
    #[doc = "0: USBFS peripheral clock disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: USBFS peripheral clock enabled"]
    B_0x1 = 1,
}
impl From<USBFSEN_A> for bool {
    #[inline(always)]
    fn from(variant: USBFSEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBFSEN` reader - USBFS clock enable Set and reset by software."]
pub type USBFSEN_R = crate::BitReader<USBFSEN_A>;
impl USBFSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USBFSEN_A {
        match self.bits {
            false => USBFSEN_A::B_0x0,
            true => USBFSEN_A::B_0x1,
        }
    }
    #[doc = "USBFS peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == USBFSEN_A::B_0x0
    }
    #[doc = "USBFS peripheral clock enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == USBFSEN_A::B_0x1
    }
}
#[doc = "Field `USBFSEN` writer - USBFS clock enable Set and reset by software."]
pub type USBFSEN_W<'a, REG> = crate::BitWriter<'a, REG, USBFSEN_A>;
impl<'a, REG> USBFSEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USBFS peripheral clock disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(USBFSEN_A::B_0x0)
    }
    #[doc = "USBFS peripheral clock enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(USBFSEN_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 11 - TIM1 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn TIM1EN(&self) -> TIM1EN_R {
        TIM1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn SPI1EN(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn USART1EN(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 24 - USBFS clock enable Set and reset by software."]
    #[inline(always)]
    pub fn USBFSEN(&self) -> USBFSEN_R {
        USBFSEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - TIM1 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn TIM1EN(&mut self) -> TIM1EN_W<'_, APB2ENR_SPEC> {
        TIM1EN_W::new(self, 11)
    }
    #[doc = "Bit 12 - SPI1 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn SPI1EN(&mut self) -> SPI1EN_W<'_, APB2ENR_SPEC> {
        SPI1EN_W::new(self, 12)
    }
    #[doc = "Bit 14 - USART1 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn USART1EN(&mut self) -> USART1EN_W<'_, APB2ENR_SPEC> {
        USART1EN_W::new(self, 14)
    }
    #[doc = "Bit 24 - USBFS clock enable Set and reset by software."]
    #[inline(always)]
    pub fn USBFSEN(&mut self) -> USBFSEN_W<'_, APB2ENR_SPEC> {
        USBFSEN_W::new(self, 24)
    }
}
#[doc = "RCC APB2 peripheral clock register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2ENR_SPEC;
impl crate::RegisterSpec for APB2ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2enr::R`](R) reader structure"]
impl crate::Readable for APB2ENR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb2enr::W`](W) writer structure"]
impl crate::Writable for APB2ENR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets APB2ENR to value 0"]
impl crate::Resettable for APB2ENR_SPEC {}
