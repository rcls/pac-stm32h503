#[doc = "Register `PLL1CFGR` reader"]
pub type R = crate::R<PLL1CFGR_SPEC>;
#[doc = "Register `PLL1CFGR` writer"]
pub type W = crate::W<PLL1CFGR_SPEC>;
#[doc = "DIVMx and PLLs clock source selection Set and reset by software to select the PLL clock source. These bits can be written only when all PLLs are disabled. In order to save power, when no PLL is used, the value of PLL1SRC must be set to '00'. 00: no clock send to DIVMx divider and PLLs (default after reset).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLL1SRC_A {
    #[doc = "1: HSI selected as PLL clock (hsi_ck)"]
    B_0x1 = 1,
    #[doc = "2: CSI selected as PLL clock (csi_ck)"]
    B_0x2 = 2,
    #[doc = "3: HSE selected as PLL clock (hse_ck)"]
    B_0x3 = 3,
}
impl From<PLL1SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: PLL1SRC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLL1SRC_A {
    type Ux = u8;
}
impl crate::IsEnum for PLL1SRC_A {}
#[doc = "Field `PLL1SRC` reader - DIVMx and PLLs clock source selection Set and reset by software to select the PLL clock source. These bits can be written only when all PLLs are disabled. In order to save power, when no PLL is used, the value of PLL1SRC must be set to '00'. 00: no clock send to DIVMx divider and PLLs (default after reset)."]
pub type PLL1SRC_R = crate::FieldReader<PLL1SRC_A>;
impl PLL1SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PLL1SRC_A> {
        match self.bits {
            1 => Some(PLL1SRC_A::B_0x1),
            2 => Some(PLL1SRC_A::B_0x2),
            3 => Some(PLL1SRC_A::B_0x3),
            _ => None,
        }
    }
    #[doc = "HSI selected as PLL clock (hsi_ck)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PLL1SRC_A::B_0x1
    }
    #[doc = "CSI selected as PLL clock (csi_ck)"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == PLL1SRC_A::B_0x2
    }
    #[doc = "HSE selected as PLL clock (hse_ck)"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == PLL1SRC_A::B_0x3
    }
}
#[doc = "Field `PLL1SRC` writer - DIVMx and PLLs clock source selection Set and reset by software to select the PLL clock source. These bits can be written only when all PLLs are disabled. In order to save power, when no PLL is used, the value of PLL1SRC must be set to '00'. 00: no clock send to DIVMx divider and PLLs (default after reset)."]
pub type PLL1SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PLL1SRC_A>;
impl<'a, REG> PLL1SRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HSI selected as PLL clock (hsi_ck)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1SRC_A::B_0x1)
    }
    #[doc = "CSI selected as PLL clock (csi_ck)"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1SRC_A::B_0x2)
    }
    #[doc = "HSE selected as PLL clock (hse_ck)"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1SRC_A::B_0x3)
    }
}
#[doc = "PLL1 input frequency range Set and reset by software to select the proper reference frequency range used for PLL1. This bit must be written before enabling the PLL1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLL1RGE_A {
    #[doc = "0: PLL1 input (ref1_ck) clock range frequency between 1 and 2 MHz (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: PLL1 input (ref1_ck) clock range frequency between 2 and 4 MHz"]
    B_0x1 = 1,
    #[doc = "2: PLL1 input (ref1_ck) clock range frequency between 4 and 8 MHz"]
    B_0x2 = 2,
    #[doc = "3: PLL1 input (ref1_ck) clock range frequency between 8 and 16 MHz"]
    B_0x3 = 3,
}
impl From<PLL1RGE_A> for u8 {
    #[inline(always)]
    fn from(variant: PLL1RGE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLL1RGE_A {
    type Ux = u8;
}
impl crate::IsEnum for PLL1RGE_A {}
#[doc = "Field `PLL1RGE` reader - PLL1 input frequency range Set and reset by software to select the proper reference frequency range used for PLL1. This bit must be written before enabling the PLL1."]
pub type PLL1RGE_R = crate::FieldReader<PLL1RGE_A>;
impl PLL1RGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLL1RGE_A {
        match self.bits {
            0 => PLL1RGE_A::B_0x0,
            1 => PLL1RGE_A::B_0x1,
            2 => PLL1RGE_A::B_0x2,
            3 => PLL1RGE_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "PLL1 input (ref1_ck) clock range frequency between 1 and 2 MHz (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PLL1RGE_A::B_0x0
    }
    #[doc = "PLL1 input (ref1_ck) clock range frequency between 2 and 4 MHz"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PLL1RGE_A::B_0x1
    }
    #[doc = "PLL1 input (ref1_ck) clock range frequency between 4 and 8 MHz"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == PLL1RGE_A::B_0x2
    }
    #[doc = "PLL1 input (ref1_ck) clock range frequency between 8 and 16 MHz"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == PLL1RGE_A::B_0x3
    }
}
#[doc = "Field `PLL1RGE` writer - PLL1 input frequency range Set and reset by software to select the proper reference frequency range used for PLL1. This bit must be written before enabling the PLL1."]
pub type PLL1RGE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PLL1RGE_A, crate::Safe>;
impl<'a, REG> PLL1RGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PLL1 input (ref1_ck) clock range frequency between 1 and 2 MHz (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1RGE_A::B_0x0)
    }
    #[doc = "PLL1 input (ref1_ck) clock range frequency between 2 and 4 MHz"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1RGE_A::B_0x1)
    }
    #[doc = "PLL1 input (ref1_ck) clock range frequency between 4 and 8 MHz"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1RGE_A::B_0x2)
    }
    #[doc = "PLL1 input (ref1_ck) clock range frequency between 8 and 16 MHz"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1RGE_A::B_0x3)
    }
}
#[doc = "Field `PLL1FRACEN` reader - PLL1 fractional latch enable Set and reset by software to latch the content of FRACN1 into the sigma-delta modulator. In order to latch the FRACN1 value into the sigma-delta modulator, PLL1FRACEN must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN1 into the modulator."]
pub type PLL1FRACEN_R = crate::BitReader;
#[doc = "Field `PLL1FRACEN` writer - PLL1 fractional latch enable Set and reset by software to latch the content of FRACN1 into the sigma-delta modulator. In order to latch the FRACN1 value into the sigma-delta modulator, PLL1FRACEN must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN1 into the modulator."]
pub type PLL1FRACEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "PLL1 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL1. This bit must be written before enabling the PLL1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL1VCOSEL_A {
    #[doc = "0: wide VCO range 192 to 836 MHz (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: medium VCO range 150 to 420 MHz"]
    B_0x1 = 1,
}
impl From<PLL1VCOSEL_A> for bool {
    #[inline(always)]
    fn from(variant: PLL1VCOSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLL1VCOSEL` reader - PLL1 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL1. This bit must be written before enabling the PLL1."]
pub type PLL1VCOSEL_R = crate::BitReader<PLL1VCOSEL_A>;
impl PLL1VCOSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLL1VCOSEL_A {
        match self.bits {
            false => PLL1VCOSEL_A::B_0x0,
            true => PLL1VCOSEL_A::B_0x1,
        }
    }
    #[doc = "wide VCO range 192 to 836 MHz (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PLL1VCOSEL_A::B_0x0
    }
    #[doc = "medium VCO range 150 to 420 MHz"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PLL1VCOSEL_A::B_0x1
    }
}
#[doc = "Field `PLL1VCOSEL` writer - PLL1 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL1. This bit must be written before enabling the PLL1."]
pub type PLL1VCOSEL_W<'a, REG> = crate::BitWriter<'a, REG, PLL1VCOSEL_A>;
impl<'a, REG> PLL1VCOSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wide VCO range 192 to 836 MHz (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1VCOSEL_A::B_0x0)
    }
    #[doc = "medium VCO range 150 to 420 MHz"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1VCOSEL_A::B_0x1)
    }
}
#[doc = "prescaler for PLL1 Set and cleared by software to configure the prescaler of the PLL1. The hardware does not allow any modification of this prescaler when PLL1 is enabled (PLL1ON = 1 or PLL1RDY = 1). In order to save power when PLL1 is not used, the value of DIVM1 must be set to 0. ... ...\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLL1M_A {
    #[doc = "0: prescaler disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: division by 1 (bypass)"]
    B_0x1 = 1,
    #[doc = "2: division by 2"]
    B_0x2 = 2,
    #[doc = "3: division by 3"]
    B_0x3 = 3,
    #[doc = "32: division by 32"]
    B_0x20 = 32,
    #[doc = "63: division by 63"]
    B_0x3F = 63,
}
impl From<PLL1M_A> for u8 {
    #[inline(always)]
    fn from(variant: PLL1M_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLL1M_A {
    type Ux = u8;
}
impl crate::IsEnum for PLL1M_A {}
#[doc = "Field `PLL1M` reader - prescaler for PLL1 Set and cleared by software to configure the prescaler of the PLL1. The hardware does not allow any modification of this prescaler when PLL1 is enabled (PLL1ON = 1 or PLL1RDY = 1). In order to save power when PLL1 is not used, the value of DIVM1 must be set to 0. ... ..."]
pub type PLL1M_R = crate::FieldReader<PLL1M_A>;
impl PLL1M_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PLL1M_A> {
        match self.bits {
            0 => Some(PLL1M_A::B_0x0),
            1 => Some(PLL1M_A::B_0x1),
            2 => Some(PLL1M_A::B_0x2),
            3 => Some(PLL1M_A::B_0x3),
            32 => Some(PLL1M_A::B_0x20),
            63 => Some(PLL1M_A::B_0x3F),
            _ => None,
        }
    }
    #[doc = "prescaler disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PLL1M_A::B_0x0
    }
    #[doc = "division by 1 (bypass)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PLL1M_A::B_0x1
    }
    #[doc = "division by 2"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == PLL1M_A::B_0x2
    }
    #[doc = "division by 3"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == PLL1M_A::B_0x3
    }
    #[doc = "division by 32"]
    #[inline(always)]
    pub fn is_B_0x20(&self) -> bool {
        *self == PLL1M_A::B_0x20
    }
    #[doc = "division by 63"]
    #[inline(always)]
    pub fn is_B_0x3F(&self) -> bool {
        *self == PLL1M_A::B_0x3F
    }
}
#[doc = "Field `PLL1M` writer - prescaler for PLL1 Set and cleared by software to configure the prescaler of the PLL1. The hardware does not allow any modification of this prescaler when PLL1 is enabled (PLL1ON = 1 or PLL1RDY = 1). In order to save power when PLL1 is not used, the value of DIVM1 must be set to 0. ... ..."]
pub type PLL1M_W<'a, REG> = crate::FieldWriter<'a, REG, 6, PLL1M_A>;
impl<'a, REG> PLL1M_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "prescaler disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1M_A::B_0x0)
    }
    #[doc = "division by 1 (bypass)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1M_A::B_0x1)
    }
    #[doc = "division by 2"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1M_A::B_0x2)
    }
    #[doc = "division by 3"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1M_A::B_0x3)
    }
    #[doc = "division by 32"]
    #[inline(always)]
    pub fn B_0x20(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1M_A::B_0x20)
    }
    #[doc = "division by 63"]
    #[inline(always)]
    pub fn B_0x3F(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1M_A::B_0x3F)
    }
}
#[doc = "PLL1 DIVP divider output enable Set and reset by software to enable the pll1_p_ck output of the PLL1. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). In order to save power, when the pll1_p_ck output of the PLL1 is not used, the pll1_p_ck must be disabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL1PEN_A {
    #[doc = "0: pll1_p_ck output disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: pll1_p_ck output enabled"]
    B_0x1 = 1,
}
impl From<PLL1PEN_A> for bool {
    #[inline(always)]
    fn from(variant: PLL1PEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLL1PEN` reader - PLL1 DIVP divider output enable Set and reset by software to enable the pll1_p_ck output of the PLL1. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). In order to save power, when the pll1_p_ck output of the PLL1 is not used, the pll1_p_ck must be disabled."]
pub type PLL1PEN_R = crate::BitReader<PLL1PEN_A>;
impl PLL1PEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLL1PEN_A {
        match self.bits {
            false => PLL1PEN_A::B_0x0,
            true => PLL1PEN_A::B_0x1,
        }
    }
    #[doc = "pll1_p_ck output disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PLL1PEN_A::B_0x0
    }
    #[doc = "pll1_p_ck output enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PLL1PEN_A::B_0x1
    }
}
#[doc = "Field `PLL1PEN` writer - PLL1 DIVP divider output enable Set and reset by software to enable the pll1_p_ck output of the PLL1. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). In order to save power, when the pll1_p_ck output of the PLL1 is not used, the pll1_p_ck must be disabled."]
pub type PLL1PEN_W<'a, REG> = crate::BitWriter<'a, REG, PLL1PEN_A>;
impl<'a, REG> PLL1PEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "pll1_p_ck output disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1PEN_A::B_0x0)
    }
    #[doc = "pll1_p_ck output enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1PEN_A::B_0x1)
    }
}
#[doc = "PLL1 DIVQ divider output enable Set and reset by software to enable the pll1_q_ck output of the PLL1. In order to save power, when the pll1_q_ck output of the PLL1 is not used, the pll1_q_ck must be disabled. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL1QEN_A {
    #[doc = "0: pll1_q_ck output disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: pll1_q_ck output enabled"]
    B_0x1 = 1,
}
impl From<PLL1QEN_A> for bool {
    #[inline(always)]
    fn from(variant: PLL1QEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLL1QEN` reader - PLL1 DIVQ divider output enable Set and reset by software to enable the pll1_q_ck output of the PLL1. In order to save power, when the pll1_q_ck output of the PLL1 is not used, the pll1_q_ck must be disabled. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0)."]
pub type PLL1QEN_R = crate::BitReader<PLL1QEN_A>;
impl PLL1QEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLL1QEN_A {
        match self.bits {
            false => PLL1QEN_A::B_0x0,
            true => PLL1QEN_A::B_0x1,
        }
    }
    #[doc = "pll1_q_ck output disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PLL1QEN_A::B_0x0
    }
    #[doc = "pll1_q_ck output enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PLL1QEN_A::B_0x1
    }
}
#[doc = "Field `PLL1QEN` writer - PLL1 DIVQ divider output enable Set and reset by software to enable the pll1_q_ck output of the PLL1. In order to save power, when the pll1_q_ck output of the PLL1 is not used, the pll1_q_ck must be disabled. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0)."]
pub type PLL1QEN_W<'a, REG> = crate::BitWriter<'a, REG, PLL1QEN_A>;
impl<'a, REG> PLL1QEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "pll1_q_ck output disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1QEN_A::B_0x0)
    }
    #[doc = "pll1_q_ck output enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1QEN_A::B_0x1)
    }
}
#[doc = "PLL1 DIVR divider output enable Set and reset by software to enable the pll1_r_ck output of the PLL1. To save power, DIVR1EN and DIVR1 bits must be set to 0 when the pll1_r_ck is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL1REN_A {
    #[doc = "0: pll1_r_ck output disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: pll1_r_ck output enabled"]
    B_0x1 = 1,
}
impl From<PLL1REN_A> for bool {
    #[inline(always)]
    fn from(variant: PLL1REN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLL1REN` reader - PLL1 DIVR divider output enable Set and reset by software to enable the pll1_r_ck output of the PLL1. To save power, DIVR1EN and DIVR1 bits must be set to 0 when the pll1_r_ck is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0)."]
pub type PLL1REN_R = crate::BitReader<PLL1REN_A>;
impl PLL1REN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLL1REN_A {
        match self.bits {
            false => PLL1REN_A::B_0x0,
            true => PLL1REN_A::B_0x1,
        }
    }
    #[doc = "pll1_r_ck output disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PLL1REN_A::B_0x0
    }
    #[doc = "pll1_r_ck output enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PLL1REN_A::B_0x1
    }
}
#[doc = "Field `PLL1REN` writer - PLL1 DIVR divider output enable Set and reset by software to enable the pll1_r_ck output of the PLL1. To save power, DIVR1EN and DIVR1 bits must be set to 0 when the pll1_r_ck is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0)."]
pub type PLL1REN_W<'a, REG> = crate::BitWriter<'a, REG, PLL1REN_A>;
impl<'a, REG> PLL1REN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "pll1_r_ck output disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1REN_A::B_0x0)
    }
    #[doc = "pll1_r_ck output enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PLL1REN_A::B_0x1)
    }
}
impl R {
    #[doc = "Bits 0:1 - DIVMx and PLLs clock source selection Set and reset by software to select the PLL clock source. These bits can be written only when all PLLs are disabled. In order to save power, when no PLL is used, the value of PLL1SRC must be set to '00'. 00: no clock send to DIVMx divider and PLLs (default after reset)."]
    #[inline(always)]
    pub fn PLL1SRC(&self) -> PLL1SRC_R {
        PLL1SRC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PLL1 input frequency range Set and reset by software to select the proper reference frequency range used for PLL1. This bit must be written before enabling the PLL1."]
    #[inline(always)]
    pub fn PLL1RGE(&self) -> PLL1RGE_R {
        PLL1RGE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - PLL1 fractional latch enable Set and reset by software to latch the content of FRACN1 into the sigma-delta modulator. In order to latch the FRACN1 value into the sigma-delta modulator, PLL1FRACEN must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN1 into the modulator."]
    #[inline(always)]
    pub fn PLL1FRACEN(&self) -> PLL1FRACEN_R {
        PLL1FRACEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PLL1 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL1. This bit must be written before enabling the PLL1."]
    #[inline(always)]
    pub fn PLL1VCOSEL(&self) -> PLL1VCOSEL_R {
        PLL1VCOSEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:13 - prescaler for PLL1 Set and cleared by software to configure the prescaler of the PLL1. The hardware does not allow any modification of this prescaler when PLL1 is enabled (PLL1ON = 1 or PLL1RDY = 1). In order to save power when PLL1 is not used, the value of DIVM1 must be set to 0. ... ..."]
    #[inline(always)]
    pub fn PLL1M(&self) -> PLL1M_R {
        PLL1M_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - PLL1 DIVP divider output enable Set and reset by software to enable the pll1_p_ck output of the PLL1. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). In order to save power, when the pll1_p_ck output of the PLL1 is not used, the pll1_p_ck must be disabled."]
    #[inline(always)]
    pub fn PLL1PEN(&self) -> PLL1PEN_R {
        PLL1PEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PLL1 DIVQ divider output enable Set and reset by software to enable the pll1_q_ck output of the PLL1. In order to save power, when the pll1_q_ck output of the PLL1 is not used, the pll1_q_ck must be disabled. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0)."]
    #[inline(always)]
    pub fn PLL1QEN(&self) -> PLL1QEN_R {
        PLL1QEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PLL1 DIVR divider output enable Set and reset by software to enable the pll1_r_ck output of the PLL1. To save power, DIVR1EN and DIVR1 bits must be set to 0 when the pll1_r_ck is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0)."]
    #[inline(always)]
    pub fn PLL1REN(&self) -> PLL1REN_R {
        PLL1REN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - DIVMx and PLLs clock source selection Set and reset by software to select the PLL clock source. These bits can be written only when all PLLs are disabled. In order to save power, when no PLL is used, the value of PLL1SRC must be set to '00'. 00: no clock send to DIVMx divider and PLLs (default after reset)."]
    #[inline(always)]
    pub fn PLL1SRC(&mut self) -> PLL1SRC_W<'_, PLL1CFGR_SPEC> {
        PLL1SRC_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - PLL1 input frequency range Set and reset by software to select the proper reference frequency range used for PLL1. This bit must be written before enabling the PLL1."]
    #[inline(always)]
    pub fn PLL1RGE(&mut self) -> PLL1RGE_W<'_, PLL1CFGR_SPEC> {
        PLL1RGE_W::new(self, 2)
    }
    #[doc = "Bit 4 - PLL1 fractional latch enable Set and reset by software to latch the content of FRACN1 into the sigma-delta modulator. In order to latch the FRACN1 value into the sigma-delta modulator, PLL1FRACEN must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN1 into the modulator."]
    #[inline(always)]
    pub fn PLL1FRACEN(&mut self) -> PLL1FRACEN_W<'_, PLL1CFGR_SPEC> {
        PLL1FRACEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - PLL1 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL1. This bit must be written before enabling the PLL1."]
    #[inline(always)]
    pub fn PLL1VCOSEL(&mut self) -> PLL1VCOSEL_W<'_, PLL1CFGR_SPEC> {
        PLL1VCOSEL_W::new(self, 5)
    }
    #[doc = "Bits 8:13 - prescaler for PLL1 Set and cleared by software to configure the prescaler of the PLL1. The hardware does not allow any modification of this prescaler when PLL1 is enabled (PLL1ON = 1 or PLL1RDY = 1). In order to save power when PLL1 is not used, the value of DIVM1 must be set to 0. ... ..."]
    #[inline(always)]
    pub fn PLL1M(&mut self) -> PLL1M_W<'_, PLL1CFGR_SPEC> {
        PLL1M_W::new(self, 8)
    }
    #[doc = "Bit 16 - PLL1 DIVP divider output enable Set and reset by software to enable the pll1_p_ck output of the PLL1. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). In order to save power, when the pll1_p_ck output of the PLL1 is not used, the pll1_p_ck must be disabled."]
    #[inline(always)]
    pub fn PLL1PEN(&mut self) -> PLL1PEN_W<'_, PLL1CFGR_SPEC> {
        PLL1PEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - PLL1 DIVQ divider output enable Set and reset by software to enable the pll1_q_ck output of the PLL1. In order to save power, when the pll1_q_ck output of the PLL1 is not used, the pll1_q_ck must be disabled. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0)."]
    #[inline(always)]
    pub fn PLL1QEN(&mut self) -> PLL1QEN_W<'_, PLL1CFGR_SPEC> {
        PLL1QEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - PLL1 DIVR divider output enable Set and reset by software to enable the pll1_r_ck output of the PLL1. To save power, DIVR1EN and DIVR1 bits must be set to 0 when the pll1_r_ck is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0)."]
    #[inline(always)]
    pub fn PLL1REN(&mut self) -> PLL1REN_W<'_, PLL1CFGR_SPEC> {
        PLL1REN_W::new(self, 18)
    }
}
#[doc = "RCC PLL clock source selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll1cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLL1CFGR_SPEC;
impl crate::RegisterSpec for PLL1CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll1cfgr::R`](R) reader structure"]
impl crate::Readable for PLL1CFGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pll1cfgr::W`](W) writer structure"]
impl crate::Writable for PLL1CFGR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets PLL1CFGR to value 0"]
impl crate::Resettable for PLL1CFGR_SPEC {}
