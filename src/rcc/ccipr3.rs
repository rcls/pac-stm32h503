#[doc = "Register `CCIPR3` reader"]
pub type R = crate::R<CCIPR3_SPEC>;
#[doc = "Register `CCIPR3` writer"]
pub type W = crate::W<CCIPR3_SPEC>;
#[doc = "SPI1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPI1SEL_A {
    #[doc = "0: pll1_q_ck selected as kernel clock (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: pll2_p_ck selected as kernel clock"]
    B_0x1 = 1,
    #[doc = "3: AUDIOCLK selected as kernel clock"]
    B_0x3 = 3,
    #[doc = "4: per_ck selected as kernel clock"]
    B_0x4 = 4,
}
impl From<SPI1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SPI1SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPI1SEL_A {
    type Ux = u8;
}
impl crate::IsEnum for SPI1SEL_A {}
#[doc = "Field `SPI1SEL` reader - SPI1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type SPI1SEL_R = crate::FieldReader<SPI1SEL_A>;
impl SPI1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SPI1SEL_A> {
        match self.bits {
            0 => Some(SPI1SEL_A::B_0x0),
            1 => Some(SPI1SEL_A::B_0x1),
            3 => Some(SPI1SEL_A::B_0x3),
            4 => Some(SPI1SEL_A::B_0x4),
            _ => None,
        }
    }
    #[doc = "pll1_q_ck selected as kernel clock (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SPI1SEL_A::B_0x0
    }
    #[doc = "pll2_p_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SPI1SEL_A::B_0x1
    }
    #[doc = "AUDIOCLK selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == SPI1SEL_A::B_0x3
    }
    #[doc = "per_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == SPI1SEL_A::B_0x4
    }
}
#[doc = "Field `SPI1SEL` writer - SPI1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type SPI1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SPI1SEL_A>;
impl<'a, REG> SPI1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "pll1_q_ck selected as kernel clock (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SPI1SEL_A::B_0x0)
    }
    #[doc = "pll2_p_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SPI1SEL_A::B_0x1)
    }
    #[doc = "AUDIOCLK selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(SPI1SEL_A::B_0x3)
    }
    #[doc = "per_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(SPI1SEL_A::B_0x4)
    }
}
#[doc = "SPI2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPI2SEL_A {
    #[doc = "0: pll1_q_ck selected as kernel clock (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: pll2_p_ck selected as kernel clock"]
    B_0x1 = 1,
    #[doc = "3: AUDIOCLK selected as kernel clock"]
    B_0x3 = 3,
    #[doc = "4: per_ck selected as kernel clock"]
    B_0x4 = 4,
}
impl From<SPI2SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SPI2SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPI2SEL_A {
    type Ux = u8;
}
impl crate::IsEnum for SPI2SEL_A {}
#[doc = "Field `SPI2SEL` reader - SPI2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type SPI2SEL_R = crate::FieldReader<SPI2SEL_A>;
impl SPI2SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SPI2SEL_A> {
        match self.bits {
            0 => Some(SPI2SEL_A::B_0x0),
            1 => Some(SPI2SEL_A::B_0x1),
            3 => Some(SPI2SEL_A::B_0x3),
            4 => Some(SPI2SEL_A::B_0x4),
            _ => None,
        }
    }
    #[doc = "pll1_q_ck selected as kernel clock (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SPI2SEL_A::B_0x0
    }
    #[doc = "pll2_p_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SPI2SEL_A::B_0x1
    }
    #[doc = "AUDIOCLK selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == SPI2SEL_A::B_0x3
    }
    #[doc = "per_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == SPI2SEL_A::B_0x4
    }
}
#[doc = "Field `SPI2SEL` writer - SPI2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type SPI2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SPI2SEL_A>;
impl<'a, REG> SPI2SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "pll1_q_ck selected as kernel clock (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SPI2SEL_A::B_0x0)
    }
    #[doc = "pll2_p_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SPI2SEL_A::B_0x1)
    }
    #[doc = "AUDIOCLK selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(SPI2SEL_A::B_0x3)
    }
    #[doc = "per_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(SPI2SEL_A::B_0x4)
    }
}
#[doc = "SPI3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPI3SEL_A {
    #[doc = "0: pll1_q_ck selected as kernel clock (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: pll2_p_ck selected as kernel clock"]
    B_0x1 = 1,
    #[doc = "3: AUDIOCLK selected as kernel clock"]
    B_0x3 = 3,
    #[doc = "4: per_ck selected as kernel clock"]
    B_0x4 = 4,
}
impl From<SPI3SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SPI3SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPI3SEL_A {
    type Ux = u8;
}
impl crate::IsEnum for SPI3SEL_A {}
#[doc = "Field `SPI3SEL` reader - SPI3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type SPI3SEL_R = crate::FieldReader<SPI3SEL_A>;
impl SPI3SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SPI3SEL_A> {
        match self.bits {
            0 => Some(SPI3SEL_A::B_0x0),
            1 => Some(SPI3SEL_A::B_0x1),
            3 => Some(SPI3SEL_A::B_0x3),
            4 => Some(SPI3SEL_A::B_0x4),
            _ => None,
        }
    }
    #[doc = "pll1_q_ck selected as kernel clock (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SPI3SEL_A::B_0x0
    }
    #[doc = "pll2_p_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SPI3SEL_A::B_0x1
    }
    #[doc = "AUDIOCLK selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == SPI3SEL_A::B_0x3
    }
    #[doc = "per_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == SPI3SEL_A::B_0x4
    }
}
#[doc = "Field `SPI3SEL` writer - SPI3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub type SPI3SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SPI3SEL_A>;
impl<'a, REG> SPI3SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "pll1_q_ck selected as kernel clock (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SPI3SEL_A::B_0x0)
    }
    #[doc = "pll2_p_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SPI3SEL_A::B_0x1)
    }
    #[doc = "AUDIOCLK selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(SPI3SEL_A::B_0x3)
    }
    #[doc = "per_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(SPI3SEL_A::B_0x4)
    }
}
#[doc = "LPUART1 kernel clock source selection others: reserved, the kernel clock is disabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPUART1SEL_A {
    #[doc = "0: rcc_pclk3 s elected as kernel clock (default after reset)"]
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
impl From<LPUART1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LPUART1SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LPUART1SEL_A {
    type Ux = u8;
}
impl crate::IsEnum for LPUART1SEL_A {}
#[doc = "Field `LPUART1SEL` reader - LPUART1 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type LPUART1SEL_R = crate::FieldReader<LPUART1SEL_A>;
impl LPUART1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LPUART1SEL_A> {
        match self.bits {
            0 => Some(LPUART1SEL_A::B_0x0),
            1 => Some(LPUART1SEL_A::B_0x1),
            3 => Some(LPUART1SEL_A::B_0x3),
            4 => Some(LPUART1SEL_A::B_0x4),
            5 => Some(LPUART1SEL_A::B_0x5),
            _ => None,
        }
    }
    #[doc = "rcc_pclk3 s elected as kernel clock (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LPUART1SEL_A::B_0x0
    }
    #[doc = "pll2_q_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LPUART1SEL_A::B_0x1
    }
    #[doc = "hsi_ker_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == LPUART1SEL_A::B_0x3
    }
    #[doc = "csi_ker_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == LPUART1SEL_A::B_0x4
    }
    #[doc = "lse_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == LPUART1SEL_A::B_0x5
    }
}
#[doc = "Field `LPUART1SEL` writer - LPUART1 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type LPUART1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, LPUART1SEL_A>;
impl<'a, REG> LPUART1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "rcc_pclk3 s elected as kernel clock (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1SEL_A::B_0x0)
    }
    #[doc = "pll2_q_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1SEL_A::B_0x1)
    }
    #[doc = "hsi_ker_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1SEL_A::B_0x3)
    }
    #[doc = "csi_ker_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1SEL_A::B_0x4)
    }
    #[doc = "lse_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1SEL_A::B_0x5)
    }
}
impl R {
    #[doc = "Bits 0:2 - SPI1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn SPI1SEL(&self) -> SPI1SEL_R {
        SPI1SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - SPI2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn SPI2SEL(&self) -> SPI2SEL_R {
        SPI2SEL_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - SPI3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn SPI3SEL(&self) -> SPI3SEL_R {
        SPI3SEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 24:26 - LPUART1 kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn LPUART1SEL(&self) -> LPUART1SEL_R {
        LPUART1SEL_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SPI1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn SPI1SEL(&mut self) -> SPI1SEL_W<'_, CCIPR3_SPEC> {
        SPI1SEL_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - SPI2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn SPI2SEL(&mut self) -> SPI2SEL_W<'_, CCIPR3_SPEC> {
        SPI2SEL_W::new(self, 3)
    }
    #[doc = "Bits 6:8 - SPI3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn SPI3SEL(&mut self) -> SPI3SEL_W<'_, CCIPR3_SPEC> {
        SPI3SEL_W::new(self, 6)
    }
    #[doc = "Bits 24:26 - LPUART1 kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn LPUART1SEL(&mut self) -> LPUART1SEL_W<'_, CCIPR3_SPEC> {
        LPUART1SEL_W::new(self, 24)
    }
}
#[doc = "RCC kernel clock configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccipr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCIPR3_SPEC;
impl crate::RegisterSpec for CCIPR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccipr3::R`](R) reader structure"]
impl crate::Readable for CCIPR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccipr3::W`](W) writer structure"]
impl crate::Writable for CCIPR3_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CCIPR3 to value 0"]
impl crate::Resettable for CCIPR3_SPEC {}
