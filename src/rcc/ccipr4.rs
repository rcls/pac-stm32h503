#[doc = "Register `CCIPR4` reader"]
pub type R = crate::R<CCIPR4_SPEC>;
#[doc = "Register `CCIPR4` writer"]
pub type W = crate::W<CCIPR4_SPEC>;
#[doc = "SYSTICK clock source selection Note: rcc_hclk frequency must be four times higher than lsi_ker_ck/lse_ck (period (LSI/LSE) greater than or equal 4 * period (HCLK).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYSTICKSEL_A {
    #[doc = "0: rcc_hclk/8 selected as clock source (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: lsi_ker_ck\\[1\\] selected as clock source"]
    B_0x1 = 1,
    #[doc = "2: lse_ck\\[1\\] selected as clock source"]
    B_0x2 = 2,
    #[doc = "3: reserved, the kernel clock is disabled"]
    B_0x3 = 3,
}
impl From<SYSTICKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSTICKSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYSTICKSEL_A {
    type Ux = u8;
}
impl crate::IsEnum for SYSTICKSEL_A {}
#[doc = "Field `SYSTICKSEL` reader - SYSTICK clock source selection Note: rcc_hclk frequency must be four times higher than lsi_ker_ck/lse_ck (period (LSI/LSE) greater than or equal 4 * period (HCLK)."]
pub type SYSTICKSEL_R = crate::FieldReader<SYSTICKSEL_A>;
impl SYSTICKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SYSTICKSEL_A {
        match self.bits {
            0 => SYSTICKSEL_A::B_0x0,
            1 => SYSTICKSEL_A::B_0x1,
            2 => SYSTICKSEL_A::B_0x2,
            3 => SYSTICKSEL_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "rcc_hclk/8 selected as clock source (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SYSTICKSEL_A::B_0x0
    }
    #[doc = "lsi_ker_ck\\[1\\] selected as clock source"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SYSTICKSEL_A::B_0x1
    }
    #[doc = "lse_ck\\[1\\] selected as clock source"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == SYSTICKSEL_A::B_0x2
    }
    #[doc = "reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == SYSTICKSEL_A::B_0x3
    }
}
#[doc = "Field `SYSTICKSEL` writer - SYSTICK clock source selection Note: rcc_hclk frequency must be four times higher than lsi_ker_ck/lse_ck (period (LSI/LSE) greater than or equal 4 * period (HCLK)."]
pub type SYSTICKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SYSTICKSEL_A, crate::Safe>;
impl<'a, REG> SYSTICKSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "rcc_hclk/8 selected as clock source (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SYSTICKSEL_A::B_0x0)
    }
    #[doc = "lsi_ker_ck\\[1\\] selected as clock source"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SYSTICKSEL_A::B_0x1)
    }
    #[doc = "lse_ck\\[1\\] selected as clock source"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(SYSTICKSEL_A::B_0x2)
    }
    #[doc = "reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(SYSTICKSEL_A::B_0x3)
    }
}
#[doc = "USBFS kernel clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USBFSSEL_A {
    #[doc = "0: no clock is selected as kernel clock (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: pll1_q_ck selected as kernel clock"]
    B_0x1 = 1,
    #[doc = "2: pll2_q_ck selected as kernel clock"]
    B_0x2 = 2,
    #[doc = "3: hsi48_ker_ck selected as kernel clock"]
    B_0x3 = 3,
}
impl From<USBFSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: USBFSSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USBFSSEL_A {
    type Ux = u8;
}
impl crate::IsEnum for USBFSSEL_A {}
#[doc = "Field `USBFSSEL` reader - USBFS kernel clock source selection"]
pub type USBFSSEL_R = crate::FieldReader<USBFSSEL_A>;
impl USBFSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USBFSSEL_A {
        match self.bits {
            0 => USBFSSEL_A::B_0x0,
            1 => USBFSSEL_A::B_0x1,
            2 => USBFSSEL_A::B_0x2,
            3 => USBFSSEL_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "no clock is selected as kernel clock (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == USBFSSEL_A::B_0x0
    }
    #[doc = "pll1_q_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == USBFSSEL_A::B_0x1
    }
    #[doc = "pll2_q_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == USBFSSEL_A::B_0x2
    }
    #[doc = "hsi48_ker_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == USBFSSEL_A::B_0x3
    }
}
#[doc = "Field `USBFSSEL` writer - USBFS kernel clock source selection"]
pub type USBFSSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, USBFSSEL_A, crate::Safe>;
impl<'a, REG> USBFSSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no clock is selected as kernel clock (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(USBFSSEL_A::B_0x0)
    }
    #[doc = "pll1_q_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(USBFSSEL_A::B_0x1)
    }
    #[doc = "pll2_q_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(USBFSSEL_A::B_0x2)
    }
    #[doc = "hsi48_ker_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(USBFSSEL_A::B_0x3)
    }
}
#[doc = "I2C1 kernel clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2C1SEL_A {
    #[doc = "0: rcc_pclk1 selected as kernel clock (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: pll2_r_ck selected as kernel clock"]
    B_0x1 = 1,
    #[doc = "2: hsi_ker_ck selected as kernel clock"]
    B_0x2 = 2,
    #[doc = "3: csi_ker_ck selected as kernel clock"]
    B_0x3 = 3,
}
impl From<I2C1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: I2C1SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2C1SEL_A {
    type Ux = u8;
}
impl crate::IsEnum for I2C1SEL_A {}
#[doc = "Field `I2C1SEL` reader - I2C1 kernel clock source selection"]
pub type I2C1SEL_R = crate::FieldReader<I2C1SEL_A>;
impl I2C1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2C1SEL_A {
        match self.bits {
            0 => I2C1SEL_A::B_0x0,
            1 => I2C1SEL_A::B_0x1,
            2 => I2C1SEL_A::B_0x2,
            3 => I2C1SEL_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "rcc_pclk1 selected as kernel clock (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == I2C1SEL_A::B_0x0
    }
    #[doc = "pll2_r_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == I2C1SEL_A::B_0x1
    }
    #[doc = "hsi_ker_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == I2C1SEL_A::B_0x2
    }
    #[doc = "csi_ker_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == I2C1SEL_A::B_0x3
    }
}
#[doc = "Field `I2C1SEL` writer - I2C1 kernel clock source selection"]
pub type I2C1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, I2C1SEL_A, crate::Safe>;
impl<'a, REG> I2C1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "rcc_pclk1 selected as kernel clock (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1SEL_A::B_0x0)
    }
    #[doc = "pll2_r_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1SEL_A::B_0x1)
    }
    #[doc = "hsi_ker_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1SEL_A::B_0x2)
    }
    #[doc = "csi_ker_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1SEL_A::B_0x3)
    }
}
#[doc = "I2C2 kernel clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2C2SEL_A {
    #[doc = "0: rcc_pclk1 selected as kernel clock (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: pll2_r_ck selected as kernel clock"]
    B_0x1 = 1,
    #[doc = "2: hsi_ker_ck selected as kernel clock"]
    B_0x2 = 2,
    #[doc = "3: csi_ker_ck selected as kernel clock"]
    B_0x3 = 3,
}
impl From<I2C2SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: I2C2SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2C2SEL_A {
    type Ux = u8;
}
impl crate::IsEnum for I2C2SEL_A {}
#[doc = "Field `I2C2SEL` reader - I2C2 kernel clock source selection"]
pub type I2C2SEL_R = crate::FieldReader<I2C2SEL_A>;
impl I2C2SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2C2SEL_A {
        match self.bits {
            0 => I2C2SEL_A::B_0x0,
            1 => I2C2SEL_A::B_0x1,
            2 => I2C2SEL_A::B_0x2,
            3 => I2C2SEL_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "rcc_pclk1 selected as kernel clock (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == I2C2SEL_A::B_0x0
    }
    #[doc = "pll2_r_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == I2C2SEL_A::B_0x1
    }
    #[doc = "hsi_ker_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == I2C2SEL_A::B_0x2
    }
    #[doc = "csi_ker_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == I2C2SEL_A::B_0x3
    }
}
#[doc = "Field `I2C2SEL` writer - I2C2 kernel clock source selection"]
pub type I2C2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, I2C2SEL_A, crate::Safe>;
impl<'a, REG> I2C2SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "rcc_pclk1 selected as kernel clock (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2C2SEL_A::B_0x0)
    }
    #[doc = "pll2_r_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2C2SEL_A::B_0x1)
    }
    #[doc = "hsi_ker_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(I2C2SEL_A::B_0x2)
    }
    #[doc = "csi_ker_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(I2C2SEL_A::B_0x3)
    }
}
#[doc = "I3C1 kernel clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I3C1SEL_A {
    #[doc = "0: rcc_pclk3 selected as kernel clock (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: pll2_r_ck selected as kernel clock"]
    B_0x1 = 1,
    #[doc = "2: hsi_ker_ck selected as kernel clock"]
    B_0x2 = 2,
    #[doc = "3: no clock selected"]
    B_0x3 = 3,
}
impl From<I3C1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: I3C1SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I3C1SEL_A {
    type Ux = u8;
}
impl crate::IsEnum for I3C1SEL_A {}
#[doc = "Field `I3C1SEL` reader - I3C1 kernel clock source selection"]
pub type I3C1SEL_R = crate::FieldReader<I3C1SEL_A>;
impl I3C1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I3C1SEL_A {
        match self.bits {
            0 => I3C1SEL_A::B_0x0,
            1 => I3C1SEL_A::B_0x1,
            2 => I3C1SEL_A::B_0x2,
            3 => I3C1SEL_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "rcc_pclk3 selected as kernel clock (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == I3C1SEL_A::B_0x0
    }
    #[doc = "pll2_r_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == I3C1SEL_A::B_0x1
    }
    #[doc = "hsi_ker_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == I3C1SEL_A::B_0x2
    }
    #[doc = "no clock selected"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == I3C1SEL_A::B_0x3
    }
}
#[doc = "Field `I3C1SEL` writer - I3C1 kernel clock source selection"]
pub type I3C1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, I3C1SEL_A, crate::Safe>;
impl<'a, REG> I3C1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "rcc_pclk3 selected as kernel clock (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I3C1SEL_A::B_0x0)
    }
    #[doc = "pll2_r_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I3C1SEL_A::B_0x1)
    }
    #[doc = "hsi_ker_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(I3C1SEL_A::B_0x2)
    }
    #[doc = "no clock selected"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(I3C1SEL_A::B_0x3)
    }
}
#[doc = "I3C2 kernel clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I3C2SEL_A {
    #[doc = "0: rcc_pclk3 selected as kernel clock (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: pll2_r_ck selected as kernel clock"]
    B_0x1 = 1,
    #[doc = "2: hsi_ker_ck selected as kernel clock"]
    B_0x2 = 2,
    #[doc = "3: no clock selected"]
    B_0x3 = 3,
}
impl From<I3C2SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: I3C2SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I3C2SEL_A {
    type Ux = u8;
}
impl crate::IsEnum for I3C2SEL_A {}
#[doc = "Field `I3C2SEL` reader - I3C2 kernel clock source selection"]
pub type I3C2SEL_R = crate::FieldReader<I3C2SEL_A>;
impl I3C2SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I3C2SEL_A {
        match self.bits {
            0 => I3C2SEL_A::B_0x0,
            1 => I3C2SEL_A::B_0x1,
            2 => I3C2SEL_A::B_0x2,
            3 => I3C2SEL_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "rcc_pclk3 selected as kernel clock (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == I3C2SEL_A::B_0x0
    }
    #[doc = "pll2_r_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == I3C2SEL_A::B_0x1
    }
    #[doc = "hsi_ker_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == I3C2SEL_A::B_0x2
    }
    #[doc = "no clock selected"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == I3C2SEL_A::B_0x3
    }
}
#[doc = "Field `I3C2SEL` writer - I3C2 kernel clock source selection"]
pub type I3C2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, I3C2SEL_A, crate::Safe>;
impl<'a, REG> I3C2SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "rcc_pclk3 selected as kernel clock (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I3C2SEL_A::B_0x0)
    }
    #[doc = "pll2_r_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I3C2SEL_A::B_0x1)
    }
    #[doc = "hsi_ker_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(I3C2SEL_A::B_0x2)
    }
    #[doc = "no clock selected"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(I3C2SEL_A::B_0x3)
    }
}
impl R {
    #[doc = "Bits 2:3 - SYSTICK clock source selection Note: rcc_hclk frequency must be four times higher than lsi_ker_ck/lse_ck (period (LSI/LSE) greater than or equal 4 * period (HCLK)."]
    #[inline(always)]
    pub fn SYSTICKSEL(&self) -> SYSTICKSEL_R {
        SYSTICKSEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - USBFS kernel clock source selection"]
    #[inline(always)]
    pub fn USBFSSEL(&self) -> USBFSSEL_R {
        USBFSSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 16:17 - I2C1 kernel clock source selection"]
    #[inline(always)]
    pub fn I2C1SEL(&self) -> I2C1SEL_R {
        I2C1SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - I2C2 kernel clock source selection"]
    #[inline(always)]
    pub fn I2C2SEL(&self) -> I2C2SEL_R {
        I2C2SEL_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 24:25 - I3C1 kernel clock source selection"]
    #[inline(always)]
    pub fn I3C1SEL(&self) -> I3C1SEL_R {
        I3C1SEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - I3C2 kernel clock source selection"]
    #[inline(always)]
    pub fn I3C2SEL(&self) -> I3C2SEL_R {
        I3C2SEL_R::new(((self.bits >> 26) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 2:3 - SYSTICK clock source selection Note: rcc_hclk frequency must be four times higher than lsi_ker_ck/lse_ck (period (LSI/LSE) greater than or equal 4 * period (HCLK)."]
    #[inline(always)]
    pub fn SYSTICKSEL(&mut self) -> SYSTICKSEL_W<'_, CCIPR4_SPEC> {
        SYSTICKSEL_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - USBFS kernel clock source selection"]
    #[inline(always)]
    pub fn USBFSSEL(&mut self) -> USBFSSEL_W<'_, CCIPR4_SPEC> {
        USBFSSEL_W::new(self, 4)
    }
    #[doc = "Bits 16:17 - I2C1 kernel clock source selection"]
    #[inline(always)]
    pub fn I2C1SEL(&mut self) -> I2C1SEL_W<'_, CCIPR4_SPEC> {
        I2C1SEL_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - I2C2 kernel clock source selection"]
    #[inline(always)]
    pub fn I2C2SEL(&mut self) -> I2C2SEL_W<'_, CCIPR4_SPEC> {
        I2C2SEL_W::new(self, 18)
    }
    #[doc = "Bits 24:25 - I3C1 kernel clock source selection"]
    #[inline(always)]
    pub fn I3C1SEL(&mut self) -> I3C1SEL_W<'_, CCIPR4_SPEC> {
        I3C1SEL_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - I3C2 kernel clock source selection"]
    #[inline(always)]
    pub fn I3C2SEL(&mut self) -> I3C2SEL_W<'_, CCIPR4_SPEC> {
        I3C2SEL_W::new(self, 26)
    }
}
#[doc = "RCC kernel clock configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccipr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCIPR4_SPEC;
impl crate::RegisterSpec for CCIPR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccipr4::R`](R) reader structure"]
impl crate::Readable for CCIPR4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccipr4::W`](W) writer structure"]
impl crate::Writable for CCIPR4_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CCIPR4 to value 0"]
impl crate::Resettable for CCIPR4_SPEC {}
