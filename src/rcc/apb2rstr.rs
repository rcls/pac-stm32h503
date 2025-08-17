#[doc = "Register `APB2RSTR` reader"]
pub type R = crate::R<APB2RSTR_SPEC>;
#[doc = "Register `APB2RSTR` writer"]
pub type W = crate::W<APB2RSTR_SPEC>;
#[doc = "TIM1 block reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM1RST_A {
    #[doc = "0: does not reset the TIM1 block (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: resets the TIM1 block"]
    B_0x1 = 1,
}
impl From<TIM1RST_A> for bool {
    #[inline(always)]
    fn from(variant: TIM1RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM1RST` reader - TIM1 block reset Set and reset by software."]
pub type TIM1RST_R = crate::BitReader<TIM1RST_A>;
impl TIM1RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIM1RST_A {
        match self.bits {
            false => TIM1RST_A::B_0x0,
            true => TIM1RST_A::B_0x1,
        }
    }
    #[doc = "does not reset the TIM1 block (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TIM1RST_A::B_0x0
    }
    #[doc = "resets the TIM1 block"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TIM1RST_A::B_0x1
    }
}
#[doc = "Field `TIM1RST` writer - TIM1 block reset Set and reset by software."]
pub type TIM1RST_W<'a, REG> = crate::BitWriter<'a, REG, TIM1RST_A>;
impl<'a, REG> TIM1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "does not reset the TIM1 block (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1RST_A::B_0x0)
    }
    #[doc = "resets the TIM1 block"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1RST_A::B_0x1)
    }
}
#[doc = "SPI1 block reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI1RST_A {
    #[doc = "0: does not reset the SPI1 block (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: resets the SPI1 block"]
    B_0x1 = 1,
}
impl From<SPI1RST_A> for bool {
    #[inline(always)]
    fn from(variant: SPI1RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI1RST` reader - SPI1 block reset Set and reset by software."]
pub type SPI1RST_R = crate::BitReader<SPI1RST_A>;
impl SPI1RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPI1RST_A {
        match self.bits {
            false => SPI1RST_A::B_0x0,
            true => SPI1RST_A::B_0x1,
        }
    }
    #[doc = "does not reset the SPI1 block (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SPI1RST_A::B_0x0
    }
    #[doc = "resets the SPI1 block"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SPI1RST_A::B_0x1
    }
}
#[doc = "Field `SPI1RST` writer - SPI1 block reset Set and reset by software."]
pub type SPI1RST_W<'a, REG> = crate::BitWriter<'a, REG, SPI1RST_A>;
impl<'a, REG> SPI1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "does not reset the SPI1 block (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SPI1RST_A::B_0x0)
    }
    #[doc = "resets the SPI1 block"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SPI1RST_A::B_0x1)
    }
}
#[doc = "USART1 block reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART1RST_A {
    #[doc = "0: does not reset the USART1 block (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: resets the USART1 block"]
    B_0x1 = 1,
}
impl From<USART1RST_A> for bool {
    #[inline(always)]
    fn from(variant: USART1RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USART1RST` reader - USART1 block reset Set and reset by software."]
pub type USART1RST_R = crate::BitReader<USART1RST_A>;
impl USART1RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USART1RST_A {
        match self.bits {
            false => USART1RST_A::B_0x0,
            true => USART1RST_A::B_0x1,
        }
    }
    #[doc = "does not reset the USART1 block (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == USART1RST_A::B_0x0
    }
    #[doc = "resets the USART1 block"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == USART1RST_A::B_0x1
    }
}
#[doc = "Field `USART1RST` writer - USART1 block reset Set and reset by software."]
pub type USART1RST_W<'a, REG> = crate::BitWriter<'a, REG, USART1RST_A>;
impl<'a, REG> USART1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "does not reset the USART1 block (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(USART1RST_A::B_0x0)
    }
    #[doc = "resets the USART1 block"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(USART1RST_A::B_0x1)
    }
}
#[doc = "USBFS block reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBFSRST_A {
    #[doc = "0: does not reset the USBFS block (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: resets the USBFS block"]
    B_0x1 = 1,
}
impl From<USBFSRST_A> for bool {
    #[inline(always)]
    fn from(variant: USBFSRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBFSRST` reader - USBFS block reset Set and reset by software."]
pub type USBFSRST_R = crate::BitReader<USBFSRST_A>;
impl USBFSRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USBFSRST_A {
        match self.bits {
            false => USBFSRST_A::B_0x0,
            true => USBFSRST_A::B_0x1,
        }
    }
    #[doc = "does not reset the USBFS block (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == USBFSRST_A::B_0x0
    }
    #[doc = "resets the USBFS block"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == USBFSRST_A::B_0x1
    }
}
#[doc = "Field `USBFSRST` writer - USBFS block reset Set and reset by software."]
pub type USBFSRST_W<'a, REG> = crate::BitWriter<'a, REG, USBFSRST_A>;
impl<'a, REG> USBFSRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "does not reset the USBFS block (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(USBFSRST_A::B_0x0)
    }
    #[doc = "resets the USBFS block"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(USBFSRST_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 11 - TIM1 block reset Set and reset by software."]
    #[inline(always)]
    pub fn TIM1RST(&self) -> TIM1RST_R {
        TIM1RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 block reset Set and reset by software."]
    #[inline(always)]
    pub fn SPI1RST(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 block reset Set and reset by software."]
    #[inline(always)]
    pub fn USART1RST(&self) -> USART1RST_R {
        USART1RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 24 - USBFS block reset Set and reset by software."]
    #[inline(always)]
    pub fn USBFSRST(&self) -> USBFSRST_R {
        USBFSRST_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - TIM1 block reset Set and reset by software."]
    #[inline(always)]
    pub fn TIM1RST(&mut self) -> TIM1RST_W<'_, APB2RSTR_SPEC> {
        TIM1RST_W::new(self, 11)
    }
    #[doc = "Bit 12 - SPI1 block reset Set and reset by software."]
    #[inline(always)]
    pub fn SPI1RST(&mut self) -> SPI1RST_W<'_, APB2RSTR_SPEC> {
        SPI1RST_W::new(self, 12)
    }
    #[doc = "Bit 14 - USART1 block reset Set and reset by software."]
    #[inline(always)]
    pub fn USART1RST(&mut self) -> USART1RST_W<'_, APB2RSTR_SPEC> {
        USART1RST_W::new(self, 14)
    }
    #[doc = "Bit 24 - USBFS block reset Set and reset by software."]
    #[inline(always)]
    pub fn USBFSRST(&mut self) -> USBFSRST_W<'_, APB2RSTR_SPEC> {
        USBFSRST_W::new(self, 24)
    }
}
#[doc = "RCC APB2 peripheral reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2RSTR_SPEC;
impl crate::RegisterSpec for APB2RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2rstr::R`](R) reader structure"]
impl crate::Readable for APB2RSTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb2rstr::W`](W) writer structure"]
impl crate::Writable for APB2RSTR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets APB2RSTR to value 0"]
impl crate::Resettable for APB2RSTR_SPEC {}
