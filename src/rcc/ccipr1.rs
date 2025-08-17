#[doc = "Register `CCIPR1` reader"]
pub type R = crate::R<CCIPR1_SPEC>;
#[doc = "Register `CCIPR1` writer"]
pub type W = crate::W<CCIPR1_SPEC>;
#[doc = "USART1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART1SEL_A {
    #[doc = "0: rcc_pclk2 selected as kernel clock (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: pll2_q_ck selected as kernel clock"]
    B_0x1 = 1,
    #[doc = "3: hsi_ker_ck selected as kernel clock"]
    B_0x3 = 3,
    #[doc = "4: csi_ker_ck selected as kernel clock"]
    B_0x4 = 4,
    #[doc = "5: lse_ck selected as kernel clock"]
    B_0x5 = 5,
}
impl From<USART1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: USART1SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USART1SEL_A {
    type Ux = u8;
}
impl crate::IsEnum for USART1SEL_A {}
#[doc = "Field `USART1SEL` reader - USART1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type USART1SEL_R = crate::FieldReader<USART1SEL_A>;
impl USART1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<USART1SEL_A> {
        match self.bits {
            0 => Some(USART1SEL_A::B_0x0),
            1 => Some(USART1SEL_A::B_0x1),
            3 => Some(USART1SEL_A::B_0x3),
            4 => Some(USART1SEL_A::B_0x4),
            5 => Some(USART1SEL_A::B_0x5),
            _ => None,
        }
    }
    #[doc = "rcc_pclk2 selected as kernel clock (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == USART1SEL_A::B_0x0
    }
    #[doc = "pll2_q_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == USART1SEL_A::B_0x1
    }
    #[doc = "hsi_ker_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == USART1SEL_A::B_0x3
    }
    #[doc = "csi_ker_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == USART1SEL_A::B_0x4
    }
    #[doc = "lse_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == USART1SEL_A::B_0x5
    }
}
#[doc = "Field `USART1SEL` writer - USART1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type USART1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, USART1SEL_A>;
impl<'a, REG> USART1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "rcc_pclk2 selected as kernel clock (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL_A::B_0x0)
    }
    #[doc = "pll2_q_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL_A::B_0x1)
    }
    #[doc = "hsi_ker_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL_A::B_0x3)
    }
    #[doc = "csi_ker_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL_A::B_0x4)
    }
    #[doc = "lse_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(USART1SEL_A::B_0x5)
    }
}
#[doc = "USART2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART2SEL_A {
    #[doc = "0: rcc_pclk1 selected as kernel clock (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: pll2_q_ck selected as kernel clock"]
    B_0x1 = 1,
    #[doc = "3: hsi_ker_ck selected as kernel clock"]
    B_0x3 = 3,
    #[doc = "4: csi_ker_ck selected as kernel clock"]
    B_0x4 = 4,
    #[doc = "5: lse_ck selected as kernel clock"]
    B_0x5 = 5,
}
impl From<USART2SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: USART2SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USART2SEL_A {
    type Ux = u8;
}
impl crate::IsEnum for USART2SEL_A {}
#[doc = "Field `USART2SEL` reader - USART2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type USART2SEL_R = crate::FieldReader<USART2SEL_A>;
impl USART2SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<USART2SEL_A> {
        match self.bits {
            0 => Some(USART2SEL_A::B_0x0),
            1 => Some(USART2SEL_A::B_0x1),
            3 => Some(USART2SEL_A::B_0x3),
            4 => Some(USART2SEL_A::B_0x4),
            5 => Some(USART2SEL_A::B_0x5),
            _ => None,
        }
    }
    #[doc = "rcc_pclk1 selected as kernel clock (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == USART2SEL_A::B_0x0
    }
    #[doc = "pll2_q_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == USART2SEL_A::B_0x1
    }
    #[doc = "hsi_ker_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == USART2SEL_A::B_0x3
    }
    #[doc = "csi_ker_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == USART2SEL_A::B_0x4
    }
    #[doc = "lse_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == USART2SEL_A::B_0x5
    }
}
#[doc = "Field `USART2SEL` writer - USART2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type USART2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, USART2SEL_A>;
impl<'a, REG> USART2SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "rcc_pclk1 selected as kernel clock (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(USART2SEL_A::B_0x0)
    }
    #[doc = "pll2_q_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(USART2SEL_A::B_0x1)
    }
    #[doc = "hsi_ker_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(USART2SEL_A::B_0x3)
    }
    #[doc = "csi_ker_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(USART2SEL_A::B_0x4)
    }
    #[doc = "lse_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(USART2SEL_A::B_0x5)
    }
}
#[doc = "USART3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART3SEL_A {
    #[doc = "0: rcc_pclk1 selected as kernel clock (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: pll2_q_ck selected as kernel clock"]
    B_0x1 = 1,
    #[doc = "3: hsi_ker_ck selected as kernel clock"]
    B_0x3 = 3,
    #[doc = "4: csi_ker_ck selected as kernel clock"]
    B_0x4 = 4,
    #[doc = "5: lse_ck selected as kernel clock"]
    B_0x5 = 5,
}
impl From<USART3SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: USART3SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USART3SEL_A {
    type Ux = u8;
}
impl crate::IsEnum for USART3SEL_A {}
#[doc = "Field `USART3SEL` reader - USART3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type USART3SEL_R = crate::FieldReader<USART3SEL_A>;
impl USART3SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<USART3SEL_A> {
        match self.bits {
            0 => Some(USART3SEL_A::B_0x0),
            1 => Some(USART3SEL_A::B_0x1),
            3 => Some(USART3SEL_A::B_0x3),
            4 => Some(USART3SEL_A::B_0x4),
            5 => Some(USART3SEL_A::B_0x5),
            _ => None,
        }
    }
    #[doc = "rcc_pclk1 selected as kernel clock (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == USART3SEL_A::B_0x0
    }
    #[doc = "pll2_q_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == USART3SEL_A::B_0x1
    }
    #[doc = "hsi_ker_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == USART3SEL_A::B_0x3
    }
    #[doc = "csi_ker_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == USART3SEL_A::B_0x4
    }
    #[doc = "lse_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == USART3SEL_A::B_0x5
    }
}
#[doc = "Field `USART3SEL` writer - USART3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type USART3SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, USART3SEL_A>;
impl<'a, REG> USART3SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "rcc_pclk1 selected as kernel clock (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(USART3SEL_A::B_0x0)
    }
    #[doc = "pll2_q_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(USART3SEL_A::B_0x1)
    }
    #[doc = "hsi_ker_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(USART3SEL_A::B_0x3)
    }
    #[doc = "csi_ker_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(USART3SEL_A::B_0x4)
    }
    #[doc = "lse_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(USART3SEL_A::B_0x5)
    }
}
#[doc = "TIM2, TIM3 and LPTIM2 input capture source selection Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMICSEL_A {
    #[doc = "0: No internal clock available for timers input capture (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: hsi_ker_ck/1024, hsi_ker_ck/8 and csi_ker_ck/128 selected for timers input capture"]
    B_0x1 = 1,
}
impl From<TIMICSEL_A> for bool {
    #[inline(always)]
    fn from(variant: TIMICSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMICSEL` reader - TIM2, TIM3 and LPTIM2 input capture source selection Set and reset by software."]
pub type TIMICSEL_R = crate::BitReader<TIMICSEL_A>;
impl TIMICSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIMICSEL_A {
        match self.bits {
            false => TIMICSEL_A::B_0x0,
            true => TIMICSEL_A::B_0x1,
        }
    }
    #[doc = "No internal clock available for timers input capture (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TIMICSEL_A::B_0x0
    }
    #[doc = "hsi_ker_ck/1024, hsi_ker_ck/8 and csi_ker_ck/128 selected for timers input capture"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TIMICSEL_A::B_0x1
    }
}
#[doc = "Field `TIMICSEL` writer - TIM2, TIM3 and LPTIM2 input capture source selection Set and reset by software."]
pub type TIMICSEL_W<'a, REG> = crate::BitWriter<'a, REG, TIMICSEL_A>;
impl<'a, REG> TIMICSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No internal clock available for timers input capture (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TIMICSEL_A::B_0x0)
    }
    #[doc = "hsi_ker_ck/1024, hsi_ker_ck/8 and csi_ker_ck/128 selected for timers input capture"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TIMICSEL_A::B_0x1)
    }
}
impl R {
    #[doc = "Bits 0:2 - USART1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn USART1SEL(&self) -> USART1SEL_R {
        USART1SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - USART2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn USART2SEL(&self) -> USART2SEL_R {
        USART2SEL_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - USART3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn USART3SEL(&self) -> USART3SEL_R {
        USART3SEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bit 31 - TIM2, TIM3 and LPTIM2 input capture source selection Set and reset by software."]
    #[inline(always)]
    pub fn TIMICSEL(&self) -> TIMICSEL_R {
        TIMICSEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - USART1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn USART1SEL(&mut self) -> USART1SEL_W<'_, CCIPR1_SPEC> {
        USART1SEL_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - USART2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn USART2SEL(&mut self) -> USART2SEL_W<'_, CCIPR1_SPEC> {
        USART2SEL_W::new(self, 3)
    }
    #[doc = "Bits 6:8 - USART3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn USART3SEL(&mut self) -> USART3SEL_W<'_, CCIPR1_SPEC> {
        USART3SEL_W::new(self, 6)
    }
    #[doc = "Bit 31 - TIM2, TIM3 and LPTIM2 input capture source selection Set and reset by software."]
    #[inline(always)]
    pub fn TIMICSEL(&mut self) -> TIMICSEL_W<'_, CCIPR1_SPEC> {
        TIMICSEL_W::new(self, 31)
    }
}
#[doc = "RCC kernel clock configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccipr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCIPR1_SPEC;
impl crate::RegisterSpec for CCIPR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccipr1::R`](R) reader structure"]
impl crate::Readable for CCIPR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccipr1::W`](W) writer structure"]
impl crate::Writable for CCIPR1_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CCIPR1 to value 0"]
impl crate::Resettable for CCIPR1_SPEC {}
