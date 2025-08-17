#[doc = "Register `PLL2CFGR` reader"]
pub type R = crate::R<PLL2CFGR_SPEC>;
#[doc = "Register `PLL2CFGR` writer"]
pub type W = crate::W<PLL2CFGR_SPEC>;
#[doc = "DIVMx and PLLs clock source selection Set and reset by software to select the PLL clock source. These bits can be written only when all PLLs are disabled. In order to save power, when no PLL is used, the value of PLL2SRC must be set to '00'.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLL2SRC_A {
    #[doc = "0: no clock send to DIVMx divider and PLLs (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: HSI selected as PLL clock (hsi_ck)"]
    B_0x1 = 1,
    #[doc = "2: CSI selected as PLL clock (csi_ck)"]
    B_0x2 = 2,
    #[doc = "3: HSE selected as PLL clock (hse_ck)"]
    B_0x3 = 3,
}
impl From<PLL2SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: PLL2SRC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLL2SRC_A {
    type Ux = u8;
}
impl crate::IsEnum for PLL2SRC_A {}
#[doc = "Field `PLL2SRC` reader - DIVMx and PLLs clock source selection Set and reset by software to select the PLL clock source. These bits can be written only when all PLLs are disabled. In order to save power, when no PLL is used, the value of PLL2SRC must be set to '00'."]
pub type PLL2SRC_R = crate::FieldReader<PLL2SRC_A>;
impl PLL2SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLL2SRC_A {
        match self.bits {
            0 => PLL2SRC_A::B_0x0,
            1 => PLL2SRC_A::B_0x1,
            2 => PLL2SRC_A::B_0x2,
            3 => PLL2SRC_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "no clock send to DIVMx divider and PLLs (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PLL2SRC_A::B_0x0
    }
    #[doc = "HSI selected as PLL clock (hsi_ck)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PLL2SRC_A::B_0x1
    }
    #[doc = "CSI selected as PLL clock (csi_ck)"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == PLL2SRC_A::B_0x2
    }
    #[doc = "HSE selected as PLL clock (hse_ck)"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == PLL2SRC_A::B_0x3
    }
}
#[doc = "Field `PLL2SRC` writer - DIVMx and PLLs clock source selection Set and reset by software to select the PLL clock source. These bits can be written only when all PLLs are disabled. In order to save power, when no PLL is used, the value of PLL2SRC must be set to '00'."]
pub type PLL2SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PLL2SRC_A, crate::Safe>;
impl<'a, REG> PLL2SRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no clock send to DIVMx divider and PLLs (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2SRC_A::B_0x0)
    }
    #[doc = "HSI selected as PLL clock (hsi_ck)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2SRC_A::B_0x1)
    }
    #[doc = "CSI selected as PLL clock (csi_ck)"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2SRC_A::B_0x2)
    }
    #[doc = "HSE selected as PLL clock (hse_ck)"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2SRC_A::B_0x3)
    }
}
#[doc = "PLL2 input frequency range Set and reset by software to select the proper reference frequency range used for PLL2. These bits must be written before enabling the PLL2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLL2RGE_A {
    #[doc = "0: PLL2 input (ref2_ck) clock range frequency between 1 and 2 MHz (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: PLL2 input (ref2_ck) clock range frequency between 2 and 4 MHz"]
    B_0x1 = 1,
    #[doc = "2: PLL2 input (ref2_ck) clock range frequency between 4 and 8 MHz"]
    B_0x2 = 2,
    #[doc = "3: PLL2 input (ref2_ck) clock range frequency between 8 and 16 MHz"]
    B_0x3 = 3,
}
impl From<PLL2RGE_A> for u8 {
    #[inline(always)]
    fn from(variant: PLL2RGE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLL2RGE_A {
    type Ux = u8;
}
impl crate::IsEnum for PLL2RGE_A {}
#[doc = "Field `PLL2RGE` reader - PLL2 input frequency range Set and reset by software to select the proper reference frequency range used for PLL2. These bits must be written before enabling the PLL2."]
pub type PLL2RGE_R = crate::FieldReader<PLL2RGE_A>;
impl PLL2RGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLL2RGE_A {
        match self.bits {
            0 => PLL2RGE_A::B_0x0,
            1 => PLL2RGE_A::B_0x1,
            2 => PLL2RGE_A::B_0x2,
            3 => PLL2RGE_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "PLL2 input (ref2_ck) clock range frequency between 1 and 2 MHz (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PLL2RGE_A::B_0x0
    }
    #[doc = "PLL2 input (ref2_ck) clock range frequency between 2 and 4 MHz"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PLL2RGE_A::B_0x1
    }
    #[doc = "PLL2 input (ref2_ck) clock range frequency between 4 and 8 MHz"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == PLL2RGE_A::B_0x2
    }
    #[doc = "PLL2 input (ref2_ck) clock range frequency between 8 and 16 MHz"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == PLL2RGE_A::B_0x3
    }
}
#[doc = "Field `PLL2RGE` writer - PLL2 input frequency range Set and reset by software to select the proper reference frequency range used for PLL2. These bits must be written before enabling the PLL2."]
pub type PLL2RGE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PLL2RGE_A, crate::Safe>;
impl<'a, REG> PLL2RGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PLL2 input (ref2_ck) clock range frequency between 1 and 2 MHz (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2RGE_A::B_0x0)
    }
    #[doc = "PLL2 input (ref2_ck) clock range frequency between 2 and 4 MHz"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2RGE_A::B_0x1)
    }
    #[doc = "PLL2 input (ref2_ck) clock range frequency between 4 and 8 MHz"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2RGE_A::B_0x2)
    }
    #[doc = "PLL2 input (ref2_ck) clock range frequency between 8 and 16 MHz"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2RGE_A::B_0x3)
    }
}
#[doc = "PLL2 fractional latch enable Set and reset by software to enable the pll2_p_ck output of the PLL2. To save power, when the pll2_p_ck output of the PLL2 is not used, the pll2_p_ck must be disabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL2FRACEN_A {
    #[doc = "0: pll2_p_ck output disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: pll2_p_ck output enabled"]
    B_0x1 = 1,
}
impl From<PLL2FRACEN_A> for bool {
    #[inline(always)]
    fn from(variant: PLL2FRACEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLL2FRACEN` reader - PLL2 fractional latch enable Set and reset by software to enable the pll2_p_ck output of the PLL2. To save power, when the pll2_p_ck output of the PLL2 is not used, the pll2_p_ck must be disabled."]
pub type PLL2FRACEN_R = crate::BitReader<PLL2FRACEN_A>;
impl PLL2FRACEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLL2FRACEN_A {
        match self.bits {
            false => PLL2FRACEN_A::B_0x0,
            true => PLL2FRACEN_A::B_0x1,
        }
    }
    #[doc = "pll2_p_ck output disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PLL2FRACEN_A::B_0x0
    }
    #[doc = "pll2_p_ck output enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PLL2FRACEN_A::B_0x1
    }
}
#[doc = "Field `PLL2FRACEN` writer - PLL2 fractional latch enable Set and reset by software to enable the pll2_p_ck output of the PLL2. To save power, when the pll2_p_ck output of the PLL2 is not used, the pll2_p_ck must be disabled."]
pub type PLL2FRACEN_W<'a, REG> = crate::BitWriter<'a, REG, PLL2FRACEN_A>;
impl<'a, REG> PLL2FRACEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "pll2_p_ck output disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2FRACEN_A::B_0x0)
    }
    #[doc = "pll2_p_ck output enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2FRACEN_A::B_0x1)
    }
}
#[doc = "PLL2 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL2. This bit must be written before enabling the PLL2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL2VCOSEL_A {
    #[doc = "0: wide VCO range 192 to 836 MHz (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: medium VCO range 150 to 420 MHz"]
    B_0x1 = 1,
}
impl From<PLL2VCOSEL_A> for bool {
    #[inline(always)]
    fn from(variant: PLL2VCOSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLL2VCOSEL` reader - PLL2 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL2. This bit must be written before enabling the PLL2."]
pub type PLL2VCOSEL_R = crate::BitReader<PLL2VCOSEL_A>;
impl PLL2VCOSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLL2VCOSEL_A {
        match self.bits {
            false => PLL2VCOSEL_A::B_0x0,
            true => PLL2VCOSEL_A::B_0x1,
        }
    }
    #[doc = "wide VCO range 192 to 836 MHz (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PLL2VCOSEL_A::B_0x0
    }
    #[doc = "medium VCO range 150 to 420 MHz"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PLL2VCOSEL_A::B_0x1
    }
}
#[doc = "Field `PLL2VCOSEL` writer - PLL2 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL2. This bit must be written before enabling the PLL2."]
pub type PLL2VCOSEL_W<'a, REG> = crate::BitWriter<'a, REG, PLL2VCOSEL_A>;
impl<'a, REG> PLL2VCOSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "wide VCO range 192 to 836 MHz (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2VCOSEL_A::B_0x0)
    }
    #[doc = "medium VCO range 150 to 420 MHz"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2VCOSEL_A::B_0x1)
    }
}
#[doc = "prescaler for PLL2 Set and cleared by software to configure the prescaler of the PLL2. The hardware does not allow any modification of this prescaler when PLL2 is enabled (PLL2ON = 1 or PLL2RDY = 1). In order to save power when PLL2 is not used, the value of DIVM2 must be set to 0. ... ...\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLL2M_A {
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
impl From<PLL2M_A> for u8 {
    #[inline(always)]
    fn from(variant: PLL2M_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLL2M_A {
    type Ux = u8;
}
impl crate::IsEnum for PLL2M_A {}
#[doc = "Field `PLL2M` reader - prescaler for PLL2 Set and cleared by software to configure the prescaler of the PLL2. The hardware does not allow any modification of this prescaler when PLL2 is enabled (PLL2ON = 1 or PLL2RDY = 1). In order to save power when PLL2 is not used, the value of DIVM2 must be set to 0. ... ..."]
pub type PLL2M_R = crate::FieldReader<PLL2M_A>;
impl PLL2M_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PLL2M_A> {
        match self.bits {
            0 => Some(PLL2M_A::B_0x0),
            1 => Some(PLL2M_A::B_0x1),
            2 => Some(PLL2M_A::B_0x2),
            3 => Some(PLL2M_A::B_0x3),
            32 => Some(PLL2M_A::B_0x20),
            63 => Some(PLL2M_A::B_0x3F),
            _ => None,
        }
    }
    #[doc = "prescaler disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PLL2M_A::B_0x0
    }
    #[doc = "division by 1 (bypass)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PLL2M_A::B_0x1
    }
    #[doc = "division by 2"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == PLL2M_A::B_0x2
    }
    #[doc = "division by 3"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == PLL2M_A::B_0x3
    }
    #[doc = "division by 32"]
    #[inline(always)]
    pub fn is_B_0x20(&self) -> bool {
        *self == PLL2M_A::B_0x20
    }
    #[doc = "division by 63"]
    #[inline(always)]
    pub fn is_B_0x3F(&self) -> bool {
        *self == PLL2M_A::B_0x3F
    }
}
#[doc = "Field `PLL2M` writer - prescaler for PLL2 Set and cleared by software to configure the prescaler of the PLL2. The hardware does not allow any modification of this prescaler when PLL2 is enabled (PLL2ON = 1 or PLL2RDY = 1). In order to save power when PLL2 is not used, the value of DIVM2 must be set to 0. ... ..."]
pub type PLL2M_W<'a, REG> = crate::FieldWriter<'a, REG, 6, PLL2M_A>;
impl<'a, REG> PLL2M_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "prescaler disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2M_A::B_0x0)
    }
    #[doc = "division by 1 (bypass)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2M_A::B_0x1)
    }
    #[doc = "division by 2"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2M_A::B_0x2)
    }
    #[doc = "division by 3"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2M_A::B_0x3)
    }
    #[doc = "division by 32"]
    #[inline(always)]
    pub fn B_0x20(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2M_A::B_0x20)
    }
    #[doc = "division by 63"]
    #[inline(always)]
    pub fn B_0x3F(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2M_A::B_0x3F)
    }
}
#[doc = "PLL2 DIVP divider output enable Set and reset by software to enable the pll2_p_ck output of the PLL2. To save power, when the pll2_p_ck output of the PLL2 is not used, the pll2_p_ck must be disabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL2PEN_A {
    #[doc = "0: pll2_p_ck output disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: pll2_p_ck output enabled"]
    B_0x1 = 1,
}
impl From<PLL2PEN_A> for bool {
    #[inline(always)]
    fn from(variant: PLL2PEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLL2PEN` reader - PLL2 DIVP divider output enable Set and reset by software to enable the pll2_p_ck output of the PLL2. To save power, when the pll2_p_ck output of the PLL2 is not used, the pll2_p_ck must be disabled."]
pub type PLL2PEN_R = crate::BitReader<PLL2PEN_A>;
impl PLL2PEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLL2PEN_A {
        match self.bits {
            false => PLL2PEN_A::B_0x0,
            true => PLL2PEN_A::B_0x1,
        }
    }
    #[doc = "pll2_p_ck output disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PLL2PEN_A::B_0x0
    }
    #[doc = "pll2_p_ck output enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PLL2PEN_A::B_0x1
    }
}
#[doc = "Field `PLL2PEN` writer - PLL2 DIVP divider output enable Set and reset by software to enable the pll2_p_ck output of the PLL2. To save power, when the pll2_p_ck output of the PLL2 is not used, the pll2_p_ck must be disabled."]
pub type PLL2PEN_W<'a, REG> = crate::BitWriter<'a, REG, PLL2PEN_A>;
impl<'a, REG> PLL2PEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "pll2_p_ck output disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2PEN_A::B_0x0)
    }
    #[doc = "pll2_p_ck output enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2PEN_A::B_0x1)
    }
}
#[doc = "PLL2 DIVQ divider output enable Set and reset by software to enable the pll2_q_ck output of the PLL2. To save power, when the pll2_q_ck output of the PLL2 is not used, the pll2_q_ck must be disabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL2QEN_A {
    #[doc = "0: pll2_q_ck output disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: pll2_q_ck output enabled"]
    B_0x1 = 1,
}
impl From<PLL2QEN_A> for bool {
    #[inline(always)]
    fn from(variant: PLL2QEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLL2QEN` reader - PLL2 DIVQ divider output enable Set and reset by software to enable the pll2_q_ck output of the PLL2. To save power, when the pll2_q_ck output of the PLL2 is not used, the pll2_q_ck must be disabled."]
pub type PLL2QEN_R = crate::BitReader<PLL2QEN_A>;
impl PLL2QEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLL2QEN_A {
        match self.bits {
            false => PLL2QEN_A::B_0x0,
            true => PLL2QEN_A::B_0x1,
        }
    }
    #[doc = "pll2_q_ck output disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PLL2QEN_A::B_0x0
    }
    #[doc = "pll2_q_ck output enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PLL2QEN_A::B_0x1
    }
}
#[doc = "Field `PLL2QEN` writer - PLL2 DIVQ divider output enable Set and reset by software to enable the pll2_q_ck output of the PLL2. To save power, when the pll2_q_ck output of the PLL2 is not used, the pll2_q_ck must be disabled."]
pub type PLL2QEN_W<'a, REG> = crate::BitWriter<'a, REG, PLL2QEN_A>;
impl<'a, REG> PLL2QEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "pll2_q_ck output disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2QEN_A::B_0x0)
    }
    #[doc = "pll2_q_ck output enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2QEN_A::B_0x1)
    }
}
#[doc = "PLL2 DIVR divider output enable Set and reset by software to enable the pll2_r_ck output of the PLL2. To save power, DIVR2EN and DIVR2 bits must be set to 0 when the pll2_r_ck is not used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL2REN_A {
    #[doc = "0: pll2_r_ck output disabled (default after reset)"]
    B_0x0 = 0,
    #[doc = "1: pll2_r_ck output enabled"]
    B_0x1 = 1,
}
impl From<PLL2REN_A> for bool {
    #[inline(always)]
    fn from(variant: PLL2REN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLL2REN` reader - PLL2 DIVR divider output enable Set and reset by software to enable the pll2_r_ck output of the PLL2. To save power, DIVR2EN and DIVR2 bits must be set to 0 when the pll2_r_ck is not used."]
pub type PLL2REN_R = crate::BitReader<PLL2REN_A>;
impl PLL2REN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLL2REN_A {
        match self.bits {
            false => PLL2REN_A::B_0x0,
            true => PLL2REN_A::B_0x1,
        }
    }
    #[doc = "pll2_r_ck output disabled (default after reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PLL2REN_A::B_0x0
    }
    #[doc = "pll2_r_ck output enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PLL2REN_A::B_0x1
    }
}
#[doc = "Field `PLL2REN` writer - PLL2 DIVR divider output enable Set and reset by software to enable the pll2_r_ck output of the PLL2. To save power, DIVR2EN and DIVR2 bits must be set to 0 when the pll2_r_ck is not used."]
pub type PLL2REN_W<'a, REG> = crate::BitWriter<'a, REG, PLL2REN_A>;
impl<'a, REG> PLL2REN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "pll2_r_ck output disabled (default after reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2REN_A::B_0x0)
    }
    #[doc = "pll2_r_ck output enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PLL2REN_A::B_0x1)
    }
}
impl R {
    #[doc = "Bits 0:1 - DIVMx and PLLs clock source selection Set and reset by software to select the PLL clock source. These bits can be written only when all PLLs are disabled. In order to save power, when no PLL is used, the value of PLL2SRC must be set to '00'."]
    #[inline(always)]
    pub fn PLL2SRC(&self) -> PLL2SRC_R {
        PLL2SRC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PLL2 input frequency range Set and reset by software to select the proper reference frequency range used for PLL2. These bits must be written before enabling the PLL2."]
    #[inline(always)]
    pub fn PLL2RGE(&self) -> PLL2RGE_R {
        PLL2RGE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - PLL2 fractional latch enable Set and reset by software to enable the pll2_p_ck output of the PLL2. To save power, when the pll2_p_ck output of the PLL2 is not used, the pll2_p_ck must be disabled."]
    #[inline(always)]
    pub fn PLL2FRACEN(&self) -> PLL2FRACEN_R {
        PLL2FRACEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PLL2 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL2. This bit must be written before enabling the PLL2."]
    #[inline(always)]
    pub fn PLL2VCOSEL(&self) -> PLL2VCOSEL_R {
        PLL2VCOSEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:13 - prescaler for PLL2 Set and cleared by software to configure the prescaler of the PLL2. The hardware does not allow any modification of this prescaler when PLL2 is enabled (PLL2ON = 1 or PLL2RDY = 1). In order to save power when PLL2 is not used, the value of DIVM2 must be set to 0. ... ..."]
    #[inline(always)]
    pub fn PLL2M(&self) -> PLL2M_R {
        PLL2M_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - PLL2 DIVP divider output enable Set and reset by software to enable the pll2_p_ck output of the PLL2. To save power, when the pll2_p_ck output of the PLL2 is not used, the pll2_p_ck must be disabled."]
    #[inline(always)]
    pub fn PLL2PEN(&self) -> PLL2PEN_R {
        PLL2PEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PLL2 DIVQ divider output enable Set and reset by software to enable the pll2_q_ck output of the PLL2. To save power, when the pll2_q_ck output of the PLL2 is not used, the pll2_q_ck must be disabled."]
    #[inline(always)]
    pub fn PLL2QEN(&self) -> PLL2QEN_R {
        PLL2QEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PLL2 DIVR divider output enable Set and reset by software to enable the pll2_r_ck output of the PLL2. To save power, DIVR2EN and DIVR2 bits must be set to 0 when the pll2_r_ck is not used."]
    #[inline(always)]
    pub fn PLL2REN(&self) -> PLL2REN_R {
        PLL2REN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - DIVMx and PLLs clock source selection Set and reset by software to select the PLL clock source. These bits can be written only when all PLLs are disabled. In order to save power, when no PLL is used, the value of PLL2SRC must be set to '00'."]
    #[inline(always)]
    pub fn PLL2SRC(&mut self) -> PLL2SRC_W<'_, PLL2CFGR_SPEC> {
        PLL2SRC_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - PLL2 input frequency range Set and reset by software to select the proper reference frequency range used for PLL2. These bits must be written before enabling the PLL2."]
    #[inline(always)]
    pub fn PLL2RGE(&mut self) -> PLL2RGE_W<'_, PLL2CFGR_SPEC> {
        PLL2RGE_W::new(self, 2)
    }
    #[doc = "Bit 4 - PLL2 fractional latch enable Set and reset by software to enable the pll2_p_ck output of the PLL2. To save power, when the pll2_p_ck output of the PLL2 is not used, the pll2_p_ck must be disabled."]
    #[inline(always)]
    pub fn PLL2FRACEN(&mut self) -> PLL2FRACEN_W<'_, PLL2CFGR_SPEC> {
        PLL2FRACEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - PLL2 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL2. This bit must be written before enabling the PLL2."]
    #[inline(always)]
    pub fn PLL2VCOSEL(&mut self) -> PLL2VCOSEL_W<'_, PLL2CFGR_SPEC> {
        PLL2VCOSEL_W::new(self, 5)
    }
    #[doc = "Bits 8:13 - prescaler for PLL2 Set and cleared by software to configure the prescaler of the PLL2. The hardware does not allow any modification of this prescaler when PLL2 is enabled (PLL2ON = 1 or PLL2RDY = 1). In order to save power when PLL2 is not used, the value of DIVM2 must be set to 0. ... ..."]
    #[inline(always)]
    pub fn PLL2M(&mut self) -> PLL2M_W<'_, PLL2CFGR_SPEC> {
        PLL2M_W::new(self, 8)
    }
    #[doc = "Bit 16 - PLL2 DIVP divider output enable Set and reset by software to enable the pll2_p_ck output of the PLL2. To save power, when the pll2_p_ck output of the PLL2 is not used, the pll2_p_ck must be disabled."]
    #[inline(always)]
    pub fn PLL2PEN(&mut self) -> PLL2PEN_W<'_, PLL2CFGR_SPEC> {
        PLL2PEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - PLL2 DIVQ divider output enable Set and reset by software to enable the pll2_q_ck output of the PLL2. To save power, when the pll2_q_ck output of the PLL2 is not used, the pll2_q_ck must be disabled."]
    #[inline(always)]
    pub fn PLL2QEN(&mut self) -> PLL2QEN_W<'_, PLL2CFGR_SPEC> {
        PLL2QEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - PLL2 DIVR divider output enable Set and reset by software to enable the pll2_r_ck output of the PLL2. To save power, DIVR2EN and DIVR2 bits must be set to 0 when the pll2_r_ck is not used."]
    #[inline(always)]
    pub fn PLL2REN(&mut self) -> PLL2REN_W<'_, PLL2CFGR_SPEC> {
        PLL2REN_W::new(self, 18)
    }
}
#[doc = "RCC PLL clock source selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll2cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll2cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLL2CFGR_SPEC;
impl crate::RegisterSpec for PLL2CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll2cfgr::R`](R) reader structure"]
impl crate::Readable for PLL2CFGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pll2cfgr::W`](W) writer structure"]
impl crate::Writable for PLL2CFGR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets PLL2CFGR to value 0"]
impl crate::Resettable for PLL2CFGR_SPEC {}
