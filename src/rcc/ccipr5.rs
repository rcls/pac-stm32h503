#[doc = "Register `CCIPR5` reader"]
pub type R = crate::R<CCIPR5_SPEC>;
#[doc = "Register `CCIPR5` writer"]
pub type W = crate::W<CCIPR5_SPEC>;
#[doc = "ADC and DAC kernel clock source selection others: reserved, the kernel clock is disabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADCDACSEL_A {
    #[doc = "0: rcc_hclk selected as kernel clock (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: sys_ck selected as kernel clock"]
    B_0x1 = 1,
    #[doc = "2: pll2_r_ck selected as kernel clock"]
    B_0x2 = 2,
    #[doc = "3: hse_ck selected as kernel clock"]
    B_0x3 = 3,
    #[doc = "4: hsi_ker_ck selected as kernel clock"]
    B_0x4 = 4,
    #[doc = "5: csi_ker_ck selected as kernel clock"]
    B_0x5 = 5,
}
impl From<ADCDACSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCDACSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADCDACSEL_A {
    type Ux = u8;
}
impl crate::IsEnum for ADCDACSEL_A {}
#[doc = "Field `ADCDACSEL` reader - ADC and DAC kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type ADCDACSEL_R = crate::FieldReader<ADCDACSEL_A>;
impl ADCDACSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ADCDACSEL_A> {
        match self.bits {
            0 => Some(ADCDACSEL_A::B_0x0),
            1 => Some(ADCDACSEL_A::B_0x1),
            2 => Some(ADCDACSEL_A::B_0x2),
            3 => Some(ADCDACSEL_A::B_0x3),
            4 => Some(ADCDACSEL_A::B_0x4),
            5 => Some(ADCDACSEL_A::B_0x5),
            _ => None,
        }
    }
    #[doc = "rcc_hclk selected as kernel clock (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ADCDACSEL_A::B_0x0
    }
    #[doc = "sys_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ADCDACSEL_A::B_0x1
    }
    #[doc = "pll2_r_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == ADCDACSEL_A::B_0x2
    }
    #[doc = "hse_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == ADCDACSEL_A::B_0x3
    }
    #[doc = "hsi_ker_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == ADCDACSEL_A::B_0x4
    }
    #[doc = "csi_ker_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == ADCDACSEL_A::B_0x5
    }
}
#[doc = "Field `ADCDACSEL` writer - ADC and DAC kernel clock source selection others: reserved, the kernel clock is disabled"]
pub type ADCDACSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, ADCDACSEL_A>;
impl<'a, REG> ADCDACSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "rcc_hclk selected as kernel clock (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ADCDACSEL_A::B_0x0)
    }
    #[doc = "sys_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ADCDACSEL_A::B_0x1)
    }
    #[doc = "pll2_r_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(ADCDACSEL_A::B_0x2)
    }
    #[doc = "hse_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(ADCDACSEL_A::B_0x3)
    }
    #[doc = "hsi_ker_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(ADCDACSEL_A::B_0x4)
    }
    #[doc = "csi_ker_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(ADCDACSEL_A::B_0x5)
    }
}
#[doc = "DAC1 sample and hold clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAC1SEL_A {
    #[doc = "0: dac_hold_ck selected as kernel clock (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: dac_hold_ck selected as kernel clock"]
    B_0x1 = 1,
}
impl From<DAC1SEL_A> for bool {
    #[inline(always)]
    fn from(variant: DAC1SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAC1SEL` reader - DAC1 sample and hold clock source selection"]
pub type DAC1SEL_R = crate::BitReader<DAC1SEL_A>;
impl DAC1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DAC1SEL_A {
        match self.bits {
            false => DAC1SEL_A::B_0x0,
            true => DAC1SEL_A::B_0x1,
        }
    }
    #[doc = "dac_hold_ck selected as kernel clock (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DAC1SEL_A::B_0x0
    }
    #[doc = "dac_hold_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DAC1SEL_A::B_0x1
    }
}
#[doc = "Field `DAC1SEL` writer - DAC1 sample and hold clock source selection"]
pub type DAC1SEL_W<'a, REG> = crate::BitWriter<'a, REG, DAC1SEL_A>;
impl<'a, REG> DAC1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "dac_hold_ck selected as kernel clock (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DAC1SEL_A::B_0x0)
    }
    #[doc = "dac_hold_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DAC1SEL_A::B_0x1)
    }
}
#[doc = "RNG kernel clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RNGSEL_A {
    #[doc = "0: hsi48_ker_ck selected as kernel clock (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: pll1_q_ck selected as kernel clock"]
    B_0x1 = 1,
    #[doc = "2: lse_ck selected as kernel clock"]
    B_0x2 = 2,
    #[doc = "3: lsi_ker_ck selected as kernel clock"]
    B_0x3 = 3,
}
impl From<RNGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RNGSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RNGSEL_A {
    type Ux = u8;
}
impl crate::IsEnum for RNGSEL_A {}
#[doc = "Field `RNGSEL` reader - RNG kernel clock source selection"]
pub type RNGSEL_R = crate::FieldReader<RNGSEL_A>;
impl RNGSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RNGSEL_A {
        match self.bits {
            0 => RNGSEL_A::B_0x0,
            1 => RNGSEL_A::B_0x1,
            2 => RNGSEL_A::B_0x2,
            3 => RNGSEL_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "hsi48_ker_ck selected as kernel clock (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RNGSEL_A::B_0x0
    }
    #[doc = "pll1_q_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RNGSEL_A::B_0x1
    }
    #[doc = "lse_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == RNGSEL_A::B_0x2
    }
    #[doc = "lsi_ker_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == RNGSEL_A::B_0x3
    }
}
#[doc = "Field `RNGSEL` writer - RNG kernel clock source selection"]
pub type RNGSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, RNGSEL_A, crate::Safe>;
impl<'a, REG> RNGSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "hsi48_ker_ck selected as kernel clock (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RNGSEL_A::B_0x0)
    }
    #[doc = "pll1_q_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RNGSEL_A::B_0x1)
    }
    #[doc = "lse_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(RNGSEL_A::B_0x2)
    }
    #[doc = "lsi_ker_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(RNGSEL_A::B_0x3)
    }
}
#[doc = "FDCAN1 kernel clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FDCAN1SEL_A {
    #[doc = "0: hse_ck selected as kernel clock (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: pll1_q_ck selected as kernel clock"]
    B_0x1 = 1,
    #[doc = "2: pll2_q_ck selected as kernel clock"]
    B_0x2 = 2,
    #[doc = "3: reserved, the kernel clock is disabled"]
    B_0x3 = 3,
}
impl From<FDCAN1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FDCAN1SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FDCAN1SEL_A {
    type Ux = u8;
}
impl crate::IsEnum for FDCAN1SEL_A {}
#[doc = "Field `FDCAN1SEL` reader - FDCAN1 kernel clock source selection"]
pub type FDCAN1SEL_R = crate::FieldReader<FDCAN1SEL_A>;
impl FDCAN1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FDCAN1SEL_A {
        match self.bits {
            0 => FDCAN1SEL_A::B_0x0,
            1 => FDCAN1SEL_A::B_0x1,
            2 => FDCAN1SEL_A::B_0x2,
            3 => FDCAN1SEL_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "hse_ck selected as kernel clock (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FDCAN1SEL_A::B_0x0
    }
    #[doc = "pll1_q_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FDCAN1SEL_A::B_0x1
    }
    #[doc = "pll2_q_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == FDCAN1SEL_A::B_0x2
    }
    #[doc = "reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == FDCAN1SEL_A::B_0x3
    }
}
#[doc = "Field `FDCAN1SEL` writer - FDCAN1 kernel clock source selection"]
pub type FDCAN1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, FDCAN1SEL_A, crate::Safe>;
impl<'a, REG> FDCAN1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "hse_ck selected as kernel clock (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FDCAN1SEL_A::B_0x0)
    }
    #[doc = "pll1_q_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FDCAN1SEL_A::B_0x1)
    }
    #[doc = "pll2_q_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(FDCAN1SEL_A::B_0x2)
    }
    #[doc = "reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(FDCAN1SEL_A::B_0x3)
    }
}
#[doc = "per_ck clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKPERSEL_A {
    #[doc = "0: hsi_ker_ck selected as kernel clock (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: csi_ker_ck selected as kernel clock"]
    B_0x1 = 1,
    #[doc = "2: hse_ck selected as kernel clock"]
    B_0x2 = 2,
    #[doc = "3: reserved, the per_ck clock is disabled"]
    B_0x3 = 3,
}
impl From<CKPERSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CKPERSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CKPERSEL_A {
    type Ux = u8;
}
impl crate::IsEnum for CKPERSEL_A {}
#[doc = "Field `CKPERSEL` reader - per_ck clock source selection"]
pub type CKPERSEL_R = crate::FieldReader<CKPERSEL_A>;
impl CKPERSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CKPERSEL_A {
        match self.bits {
            0 => CKPERSEL_A::B_0x0,
            1 => CKPERSEL_A::B_0x1,
            2 => CKPERSEL_A::B_0x2,
            3 => CKPERSEL_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "hsi_ker_ck selected as kernel clock (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CKPERSEL_A::B_0x0
    }
    #[doc = "csi_ker_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CKPERSEL_A::B_0x1
    }
    #[doc = "hse_ck selected as kernel clock"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == CKPERSEL_A::B_0x2
    }
    #[doc = "reserved, the per_ck clock is disabled"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == CKPERSEL_A::B_0x3
    }
}
#[doc = "Field `CKPERSEL` writer - per_ck clock source selection"]
pub type CKPERSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CKPERSEL_A, crate::Safe>;
impl<'a, REG> CKPERSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "hsi_ker_ck selected as kernel clock (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CKPERSEL_A::B_0x0)
    }
    #[doc = "csi_ker_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CKPERSEL_A::B_0x1)
    }
    #[doc = "hse_ck selected as kernel clock"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(CKPERSEL_A::B_0x2)
    }
    #[doc = "reserved, the per_ck clock is disabled"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(CKPERSEL_A::B_0x3)
    }
}
impl R {
    #[doc = "Bits 0:2 - ADC and DAC kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn ADCDACSEL(&self) -> ADCDACSEL_R {
        ADCDACSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - DAC1 sample and hold clock source selection"]
    #[inline(always)]
    pub fn DAC1SEL(&self) -> DAC1SEL_R {
        DAC1SEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - RNG kernel clock source selection"]
    #[inline(always)]
    pub fn RNGSEL(&self) -> RNGSEL_R {
        RNGSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - FDCAN1 kernel clock source selection"]
    #[inline(always)]
    pub fn FDCAN1SEL(&self) -> FDCAN1SEL_R {
        FDCAN1SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 30:31 - per_ck clock source selection"]
    #[inline(always)]
    pub fn CKPERSEL(&self) -> CKPERSEL_R {
        CKPERSEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - ADC and DAC kernel clock source selection others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn ADCDACSEL(&mut self) -> ADCDACSEL_W<'_, CCIPR5_SPEC> {
        ADCDACSEL_W::new(self, 0)
    }
    #[doc = "Bit 3 - DAC1 sample and hold clock source selection"]
    #[inline(always)]
    pub fn DAC1SEL(&mut self) -> DAC1SEL_W<'_, CCIPR5_SPEC> {
        DAC1SEL_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - RNG kernel clock source selection"]
    #[inline(always)]
    pub fn RNGSEL(&mut self) -> RNGSEL_W<'_, CCIPR5_SPEC> {
        RNGSEL_W::new(self, 4)
    }
    #[doc = "Bits 8:9 - FDCAN1 kernel clock source selection"]
    #[inline(always)]
    pub fn FDCAN1SEL(&mut self) -> FDCAN1SEL_W<'_, CCIPR5_SPEC> {
        FDCAN1SEL_W::new(self, 8)
    }
    #[doc = "Bits 30:31 - per_ck clock source selection"]
    #[inline(always)]
    pub fn CKPERSEL(&mut self) -> CKPERSEL_W<'_, CCIPR5_SPEC> {
        CKPERSEL_W::new(self, 30)
    }
}
#[doc = "RCC kernel clock configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccipr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCIPR5_SPEC;
impl crate::RegisterSpec for CCIPR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccipr5::R`](R) reader structure"]
impl crate::Readable for CCIPR5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccipr5::W`](W) writer structure"]
impl crate::Writable for CCIPR5_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CCIPR5 to value 0"]
impl crate::Resettable for CCIPR5_SPEC {}
