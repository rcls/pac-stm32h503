#[doc = "Register `CCIPR2` reader"]
pub type R = crate::R<CCIPR2_SPEC>;
#[doc = "Register `CCIPR2` writer"]
pub type W = crate::W<CCIPR2_SPEC>;
#[doc = "LPTIM1 kernel clock source selection others: reserved, the kernel clock is disabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPTIM1SEL_A {
    #[doc = "0: rcc_pclk3 selected as kernel clock (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: pll2_p_ck selected as kernel clock"]
    B_0x1 = 1,
    #[doc = "3: lse_ker_ck selected as kernel clock"]
    B_0x3 = 3,
    #[doc = "4: lsi_ker_ck selected as kernel clock"]
    B_0x4 = 4,
    #[doc = "5: per_ck selected as kernel clock"]
    B_0x5 = 5,
}
impl From<LPTIM1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LPTIM1SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LPTIM1SEL_A {
    type Ux = u8;
}
impl crate::IsEnum for LPTIM1SEL_A {}
#[doc = "Field `LPTIM1SEL` reader - LPTIM1 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type LPTIM1SEL_R = crate::FieldReader<LPTIM1SEL_A>;
impl LPTIM1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LPTIM1SEL_A> {
        match self.bits {
            0 => Some(LPTIM1SEL_A::B_0x0),
            1 => Some(LPTIM1SEL_A::B_0x1),
            3 => Some(LPTIM1SEL_A::B_0x3),
            4 => Some(LPTIM1SEL_A::B_0x4),
            5 => Some(LPTIM1SEL_A::B_0x5),
            _ => None,
        }
    }
    #[doc = "rcc_pclk3 selected as kernel clock (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LPTIM1SEL_A::B_0x0
    }
    #[doc = "pll2_p_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LPTIM1SEL_A::B_0x1
    }
    #[doc = "lse_ker_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == LPTIM1SEL_A::B_0x3
    }
    #[doc = "lsi_ker_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == LPTIM1SEL_A::B_0x4
    }
    #[doc = "per_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == LPTIM1SEL_A::B_0x5
    }
}
#[doc = "Field `LPTIM1SEL` writer - LPTIM1 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type LPTIM1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, LPTIM1SEL_A>;
impl<'a, REG> LPTIM1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "rcc_pclk3 selected as kernel clock (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL_A::B_0x0)
    }
    #[doc = "pll2_p_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL_A::B_0x1)
    }
    #[doc = "lse_ker_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL_A::B_0x3)
    }
    #[doc = "lsi_ker_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL_A::B_0x4)
    }
    #[doc = "per_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM1SEL_A::B_0x5)
    }
}
#[doc = "LPTIM2 kernel clock source selection others: reserved, the kernel clock is disabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPTIM2SEL_A {
    #[doc = "0: rcc_pclk1 selected as kernel clock (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: pll2_p_ck selected as kernel clock"]
    B_0x1 = 1,
    #[doc = "3: lse_ker_ck selected as kernel clock"]
    B_0x3 = 3,
    #[doc = "4: lsi_ker_ck selected as kernel clock"]
    B_0x4 = 4,
    #[doc = "5: per_ck selected as kernel clock"]
    B_0x5 = 5,
}
impl From<LPTIM2SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LPTIM2SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LPTIM2SEL_A {
    type Ux = u8;
}
impl crate::IsEnum for LPTIM2SEL_A {}
#[doc = "Field `LPTIM2SEL` reader - LPTIM2 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type LPTIM2SEL_R = crate::FieldReader<LPTIM2SEL_A>;
impl LPTIM2SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LPTIM2SEL_A> {
        match self.bits {
            0 => Some(LPTIM2SEL_A::B_0x0),
            1 => Some(LPTIM2SEL_A::B_0x1),
            3 => Some(LPTIM2SEL_A::B_0x3),
            4 => Some(LPTIM2SEL_A::B_0x4),
            5 => Some(LPTIM2SEL_A::B_0x5),
            _ => None,
        }
    }
    #[doc = "rcc_pclk1 selected as kernel clock (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LPTIM2SEL_A::B_0x0
    }
    #[doc = "pll2_p_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LPTIM2SEL_A::B_0x1
    }
    #[doc = "lse_ker_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == LPTIM2SEL_A::B_0x3
    }
    #[doc = "lsi_ker_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == LPTIM2SEL_A::B_0x4
    }
    #[doc = "per_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == LPTIM2SEL_A::B_0x5
    }
}
#[doc = "Field `LPTIM2SEL` writer - LPTIM2 kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type LPTIM2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, LPTIM2SEL_A>;
impl<'a, REG> LPTIM2SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "rcc_pclk1 selected as kernel clock (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM2SEL_A::B_0x0)
    }
    #[doc = "pll2_p_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM2SEL_A::B_0x1)
    }
    #[doc = "lse_ker_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM2SEL_A::B_0x3)
    }
    #[doc = "lsi_ker_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM2SEL_A::B_0x4)
    }
    #[doc = "per_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(LPTIM2SEL_A::B_0x5)
    }
}
impl R {
    #[doc = "Bits 8:10 - LPTIM1 kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn LPTIM1SEL(&self) -> LPTIM1SEL_R {
        LPTIM1SEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - LPTIM2 kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn LPTIM2SEL(&self) -> LPTIM2SEL_R {
        LPTIM2SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 8:10 - LPTIM1 kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn LPTIM1SEL(&mut self) -> LPTIM1SEL_W<'_, CCIPR2_SPEC> {
        LPTIM1SEL_W::new(self, 8)
    }
    #[doc = "Bits 12:14 - LPTIM2 kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn LPTIM2SEL(&mut self) -> LPTIM2SEL_W<'_, CCIPR2_SPEC> {
        LPTIM2SEL_W::new(self, 12)
    }
}
#[doc = "RCC kernel clock configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccipr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCIPR2_SPEC;
impl crate::RegisterSpec for CCIPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccipr2::R`](R) reader structure"]
impl crate::Readable for CCIPR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccipr2::W`](W) writer structure"]
impl crate::Writable for CCIPR2_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CCIPR2 to value 0"]
impl crate::Resettable for CCIPR2_SPEC {}
