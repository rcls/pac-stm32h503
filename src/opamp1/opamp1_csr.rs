#[doc = "Register `OPAMP1_CSR` reader"]
pub type R = crate::R<OPAMP1_CSR_SPEC>;
#[doc = "Register `OPAMP1_CSR` writer"]
pub type W = crate::W<OPAMP1_CSR_SPEC>;
#[doc = "Operational amplifier Enable Note: If OPAMP1 is unconnected in a specific package, it must remain disabled (keep OPAMP1_CSR register default value).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPAEN_A {
    #[doc = "0: operational amplifier disabled"]
    B_0x0 = 0,
    #[doc = "1: operational amplifier enabled"]
    B_0x1 = 1,
}
impl From<OPAEN_A> for bool {
    #[inline(always)]
    fn from(variant: OPAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPAEN` reader - Operational amplifier Enable Note: If OPAMP1 is unconnected in a specific package, it must remain disabled (keep OPAMP1_CSR register default value)."]
pub type OPAEN_R = crate::BitReader<OPAEN_A>;
impl OPAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OPAEN_A {
        match self.bits {
            false => OPAEN_A::B_0x0,
            true => OPAEN_A::B_0x1,
        }
    }
    #[doc = "operational amplifier disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OPAEN_A::B_0x0
    }
    #[doc = "operational amplifier enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OPAEN_A::B_0x1
    }
}
#[doc = "Field `OPAEN` writer - Operational amplifier Enable Note: If OPAMP1 is unconnected in a specific package, it must remain disabled (keep OPAMP1_CSR register default value)."]
pub type OPAEN_W<'a, REG> = crate::BitWriter<'a, REG, OPAEN_A>;
impl<'a, REG> OPAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "operational amplifier disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OPAEN_A::B_0x0)
    }
    #[doc = "operational amplifier enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OPAEN_A::B_0x1)
    }
}
#[doc = "Force internal reference on VP (reserved for test)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FORCE_VP_A {
    #[doc = "0: Normal operating mode. Non-inverting input connected to inputs."]
    B_0x0 = 0,
    #[doc = "1: Calibration verification mode: Non-inverting input connected to calibration reference voltage."]
    B_0x1 = 1,
}
impl From<FORCE_VP_A> for bool {
    #[inline(always)]
    fn from(variant: FORCE_VP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORCE_VP` reader - Force internal reference on VP (reserved for test)"]
pub type FORCE_VP_R = crate::BitReader<FORCE_VP_A>;
impl FORCE_VP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FORCE_VP_A {
        match self.bits {
            false => FORCE_VP_A::B_0x0,
            true => FORCE_VP_A::B_0x1,
        }
    }
    #[doc = "Normal operating mode. Non-inverting input connected to inputs."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == FORCE_VP_A::B_0x0
    }
    #[doc = "Calibration verification mode: Non-inverting input connected to calibration reference voltage."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == FORCE_VP_A::B_0x1
    }
}
#[doc = "Field `FORCE_VP` writer - Force internal reference on VP (reserved for test)"]
pub type FORCE_VP_W<'a, REG> = crate::BitWriter<'a, REG, FORCE_VP_A>;
impl<'a, REG> FORCE_VP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operating mode. Non-inverting input connected to inputs."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FORCE_VP_A::B_0x0)
    }
    #[doc = "Calibration verification mode: Non-inverting input connected to calibration reference voltage."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FORCE_VP_A::B_0x1)
    }
}
#[doc = "Non inverted input selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VP_SEL_A {
    #[doc = "0: GPIO INP0 connected to OPAMP_VINP"]
    B_0x0 = 0,
    #[doc = "1: dac_out1 connected to OPAMP_VINP"]
    B_0x1 = 1,
    #[doc = "2: GPIO INP2 is connected to OPAMP_VINP"]
    B_0x2 = 2,
}
impl From<VP_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: VP_SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VP_SEL_A {
    type Ux = u8;
}
impl crate::IsEnum for VP_SEL_A {}
#[doc = "Field `VP_SEL` reader - Non inverted input selection"]
pub type VP_SEL_R = crate::FieldReader<VP_SEL_A>;
impl VP_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<VP_SEL_A> {
        match self.bits {
            0 => Some(VP_SEL_A::B_0x0),
            1 => Some(VP_SEL_A::B_0x1),
            2 => Some(VP_SEL_A::B_0x2),
            _ => None,
        }
    }
    #[doc = "GPIO INP0 connected to OPAMP_VINP"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == VP_SEL_A::B_0x0
    }
    #[doc = "dac_out1 connected to OPAMP_VINP"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == VP_SEL_A::B_0x1
    }
    #[doc = "GPIO INP2 is connected to OPAMP_VINP"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == VP_SEL_A::B_0x2
    }
}
#[doc = "Field `VP_SEL` writer - Non inverted input selection"]
pub type VP_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, VP_SEL_A>;
impl<'a, REG> VP_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO INP0 connected to OPAMP_VINP"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(VP_SEL_A::B_0x0)
    }
    #[doc = "dac_out1 connected to OPAMP_VINP"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(VP_SEL_A::B_0x1)
    }
    #[doc = "GPIO INP2 is connected to OPAMP_VINP"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(VP_SEL_A::B_0x2)
    }
}
#[doc = "Inverting input selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VM_SEL_A {
    #[doc = "0: INM0 connected to OPAMP_VINM input"]
    B_0x0 = 0,
    #[doc = "1: INM1 connected to OPAMP_VINM input"]
    B_0x1 = 1,
    #[doc = "2: Feedback resistor is connected to OPAMP_VINM input (PGA mode), Inverting input selection is depends on the PGA_GAIN setting"]
    B_0x2 = 2,
    #[doc = "3: opamp_out connected to OPAMP_VINM input (Follower mode)"]
    B_0x3 = 3,
}
impl From<VM_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: VM_SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VM_SEL_A {
    type Ux = u8;
}
impl crate::IsEnum for VM_SEL_A {}
#[doc = "Field `VM_SEL` reader - Inverting input selection"]
pub type VM_SEL_R = crate::FieldReader<VM_SEL_A>;
impl VM_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VM_SEL_A {
        match self.bits {
            0 => VM_SEL_A::B_0x0,
            1 => VM_SEL_A::B_0x1,
            2 => VM_SEL_A::B_0x2,
            3 => VM_SEL_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "INM0 connected to OPAMP_VINM input"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == VM_SEL_A::B_0x0
    }
    #[doc = "INM1 connected to OPAMP_VINM input"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == VM_SEL_A::B_0x1
    }
    #[doc = "Feedback resistor is connected to OPAMP_VINM input (PGA mode), Inverting input selection is depends on the PGA_GAIN setting"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == VM_SEL_A::B_0x2
    }
    #[doc = "opamp_out connected to OPAMP_VINM input (Follower mode)"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == VM_SEL_A::B_0x3
    }
}
#[doc = "Field `VM_SEL` writer - Inverting input selection"]
pub type VM_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, VM_SEL_A, crate::Safe>;
impl<'a, REG> VM_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "INM0 connected to OPAMP_VINM input"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(VM_SEL_A::B_0x0)
    }
    #[doc = "INM1 connected to OPAMP_VINM input"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(VM_SEL_A::B_0x1)
    }
    #[doc = "Feedback resistor is connected to OPAMP_VINM input (PGA mode), Inverting input selection is depends on the PGA_GAIN setting"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(VM_SEL_A::B_0x2)
    }
    #[doc = "opamp_out connected to OPAMP_VINM input (Follower mode)"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(VM_SEL_A::B_0x3)
    }
}
#[doc = "Operational amplifier high-speed mode The operational amplifier must be disable to change this configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPAHSM_A {
    #[doc = "0: operational amplifier in normal mode"]
    B_0x0 = 0,
    #[doc = "1: operational amplifier in high-speed mode"]
    B_0x1 = 1,
}
impl From<OPAHSM_A> for bool {
    #[inline(always)]
    fn from(variant: OPAHSM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPAHSM` reader - Operational amplifier high-speed mode The operational amplifier must be disable to change this configuration."]
pub type OPAHSM_R = crate::BitReader<OPAHSM_A>;
impl OPAHSM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OPAHSM_A {
        match self.bits {
            false => OPAHSM_A::B_0x0,
            true => OPAHSM_A::B_0x1,
        }
    }
    #[doc = "operational amplifier in normal mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OPAHSM_A::B_0x0
    }
    #[doc = "operational amplifier in high-speed mode"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OPAHSM_A::B_0x1
    }
}
#[doc = "Field `OPAHSM` writer - Operational amplifier high-speed mode The operational amplifier must be disable to change this configuration."]
pub type OPAHSM_W<'a, REG> = crate::BitWriter<'a, REG, OPAHSM_A>;
impl<'a, REG> OPAHSM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "operational amplifier in normal mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OPAHSM_A::B_0x0)
    }
    #[doc = "operational amplifier in high-speed mode"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OPAHSM_A::B_0x1)
    }
}
#[doc = "Calibration mode enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALON_A {
    #[doc = "0: Normal mode"]
    B_0x0 = 0,
    #[doc = "1: Calibration mode (all switches opened by HW)"]
    B_0x1 = 1,
}
impl From<CALON_A> for bool {
    #[inline(always)]
    fn from(variant: CALON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALON` reader - Calibration mode enabled"]
pub type CALON_R = crate::BitReader<CALON_A>;
impl CALON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CALON_A {
        match self.bits {
            false => CALON_A::B_0x0,
            true => CALON_A::B_0x1,
        }
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CALON_A::B_0x0
    }
    #[doc = "Calibration mode (all switches opened by HW)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CALON_A::B_0x1
    }
}
#[doc = "Field `CALON` writer - Calibration mode enabled"]
pub type CALON_W<'a, REG> = crate::BitWriter<'a, REG, CALON_A>;
impl<'a, REG> CALON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CALON_A::B_0x0)
    }
    #[doc = "Calibration mode (all switches opened by HW)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CALON_A::B_0x1)
    }
}
#[doc = "Calibration selection It is used to select the offset calibration bus used to generate the internal reference voltage when CALON = 1 or FORCE_VP= 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CALSEL_A {
    #[doc = "0: 0.033*VDDA applied on OPAMP inputs"]
    B_0x0 = 0,
    #[doc = "1: 0.1*VDDA applied on OPAMP inputs (for PMOS calibration)"]
    B_0x1 = 1,
    #[doc = "2: 0.5*VDDA applied on OPAMP inputs"]
    B_0x2 = 2,
    #[doc = "3: 0.9*VDDA applied on OPAMP inputs (for NMOS calibration)"]
    B_0x3 = 3,
}
impl From<CALSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CALSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CALSEL_A {
    type Ux = u8;
}
impl crate::IsEnum for CALSEL_A {}
#[doc = "Field `CALSEL` reader - Calibration selection It is used to select the offset calibration bus used to generate the internal reference voltage when CALON = 1 or FORCE_VP= 1."]
pub type CALSEL_R = crate::FieldReader<CALSEL_A>;
impl CALSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CALSEL_A {
        match self.bits {
            0 => CALSEL_A::B_0x0,
            1 => CALSEL_A::B_0x1,
            2 => CALSEL_A::B_0x2,
            3 => CALSEL_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "0.033*VDDA applied on OPAMP inputs"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CALSEL_A::B_0x0
    }
    #[doc = "0.1*VDDA applied on OPAMP inputs (for PMOS calibration)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CALSEL_A::B_0x1
    }
    #[doc = "0.5*VDDA applied on OPAMP inputs"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == CALSEL_A::B_0x2
    }
    #[doc = "0.9*VDDA applied on OPAMP inputs (for NMOS calibration)"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == CALSEL_A::B_0x3
    }
}
#[doc = "Field `CALSEL` writer - Calibration selection It is used to select the offset calibration bus used to generate the internal reference voltage when CALON = 1 or FORCE_VP= 1."]
pub type CALSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CALSEL_A, crate::Safe>;
impl<'a, REG> CALSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0.033*VDDA applied on OPAMP inputs"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CALSEL_A::B_0x0)
    }
    #[doc = "0.1*VDDA applied on OPAMP inputs (for PMOS calibration)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CALSEL_A::B_0x1)
    }
    #[doc = "0.5*VDDA applied on OPAMP inputs"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(CALSEL_A::B_0x2)
    }
    #[doc = "0.9*VDDA applied on OPAMP inputs (for NMOS calibration)"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(CALSEL_A::B_0x3)
    }
}
#[doc = "Operational amplifier Programmable amplifier gain value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PGA_GAIN_A {
    #[doc = "0: Non-inverting internal Gain 2, VREF- referenced"]
    B_0x0 = 0,
    #[doc = "1: Non-inverting internal Gain 4, VREF- referenced"]
    B_0x1 = 1,
    #[doc = "2: Non-inverting internal Gain 8, VREF- referenced"]
    B_0x2 = 2,
    #[doc = "3: Non-inverting internal Gain 16, VREF- referenced"]
    B_0x3 = 3,
    #[doc = "4: Non-inverting internal Gain 2 with filtering on INM0, VREF- referenced"]
    B_0x4 = 4,
    #[doc = "5: Non-inverting internal Gain 4 with filtering on INM0, VREF- referenced"]
    B_0x5 = 5,
    #[doc = "6: Non-inverting internal Gain 8 with filtering on INM0, VREF- referenced"]
    B_0x6 = 6,
    #[doc = "7: Non-inverting internal Gain 16 with filtering on INM0, VREF- referenced"]
    B_0x7 = 7,
    #[doc = "8: Inverting gain=-1/ Non-inverting gain =2 with INM0 node for input or bias"]
    B_0x8 = 8,
    #[doc = "9: Inverting gain=-3/ Non-inverting gain =4 with INM0 node for input or bias"]
    B_0x9 = 9,
    #[doc = "10: Inverting gain=-7/ Non-inverting gain =8 with INM0 node for input or bias"]
    B_0xA = 10,
    #[doc = "11: Inverting gain=-15/ Non-inverting gain =16 with INM0 node for input or bias"]
    B_0xB = 11,
    #[doc = "12: Inverting gain=-1/ Non-inverting gain =2 with INM0 node for input or bias, INM1 node for filtering"]
    B_0xC = 12,
    #[doc = "13: Inverting gain=-3/ Non-inverting gain =4 with INM0 node for input or bias, INM1 node for filtering"]
    B_0xD = 13,
    #[doc = "14: Inverting gain=-7/ Non-inverting gain =8 with INM0 node for input or bias, INM1 node for filtering"]
    B_0xE = 14,
    #[doc = "15: Inverting gain=-15/ Non-inverting gain =16 with INM0 node for input or bias, INM1 node for filtering"]
    B_0xF = 15,
}
impl From<PGA_GAIN_A> for u8 {
    #[inline(always)]
    fn from(variant: PGA_GAIN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PGA_GAIN_A {
    type Ux = u8;
}
impl crate::IsEnum for PGA_GAIN_A {}
#[doc = "Field `PGA_GAIN` reader - Operational amplifier Programmable amplifier gain value"]
pub type PGA_GAIN_R = crate::FieldReader<PGA_GAIN_A>;
impl PGA_GAIN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PGA_GAIN_A {
        match self.bits {
            0 => PGA_GAIN_A::B_0x0,
            1 => PGA_GAIN_A::B_0x1,
            2 => PGA_GAIN_A::B_0x2,
            3 => PGA_GAIN_A::B_0x3,
            4 => PGA_GAIN_A::B_0x4,
            5 => PGA_GAIN_A::B_0x5,
            6 => PGA_GAIN_A::B_0x6,
            7 => PGA_GAIN_A::B_0x7,
            8 => PGA_GAIN_A::B_0x8,
            9 => PGA_GAIN_A::B_0x9,
            10 => PGA_GAIN_A::B_0xA,
            11 => PGA_GAIN_A::B_0xB,
            12 => PGA_GAIN_A::B_0xC,
            13 => PGA_GAIN_A::B_0xD,
            14 => PGA_GAIN_A::B_0xE,
            15 => PGA_GAIN_A::B_0xF,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-inverting internal Gain 2, VREF- referenced"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PGA_GAIN_A::B_0x0
    }
    #[doc = "Non-inverting internal Gain 4, VREF- referenced"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PGA_GAIN_A::B_0x1
    }
    #[doc = "Non-inverting internal Gain 8, VREF- referenced"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == PGA_GAIN_A::B_0x2
    }
    #[doc = "Non-inverting internal Gain 16, VREF- referenced"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == PGA_GAIN_A::B_0x3
    }
    #[doc = "Non-inverting internal Gain 2 with filtering on INM0, VREF- referenced"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == PGA_GAIN_A::B_0x4
    }
    #[doc = "Non-inverting internal Gain 4 with filtering on INM0, VREF- referenced"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == PGA_GAIN_A::B_0x5
    }
    #[doc = "Non-inverting internal Gain 8 with filtering on INM0, VREF- referenced"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == PGA_GAIN_A::B_0x6
    }
    #[doc = "Non-inverting internal Gain 16 with filtering on INM0, VREF- referenced"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == PGA_GAIN_A::B_0x7
    }
    #[doc = "Inverting gain=-1/ Non-inverting gain =2 with INM0 node for input or bias"]
    #[inline(always)]
    pub fn is_B_0x8(&self) -> bool {
        *self == PGA_GAIN_A::B_0x8
    }
    #[doc = "Inverting gain=-3/ Non-inverting gain =4 with INM0 node for input or bias"]
    #[inline(always)]
    pub fn is_B_0x9(&self) -> bool {
        *self == PGA_GAIN_A::B_0x9
    }
    #[doc = "Inverting gain=-7/ Non-inverting gain =8 with INM0 node for input or bias"]
    #[inline(always)]
    pub fn is_B_0xA(&self) -> bool {
        *self == PGA_GAIN_A::B_0xA
    }
    #[doc = "Inverting gain=-15/ Non-inverting gain =16 with INM0 node for input or bias"]
    #[inline(always)]
    pub fn is_B_0xB(&self) -> bool {
        *self == PGA_GAIN_A::B_0xB
    }
    #[doc = "Inverting gain=-1/ Non-inverting gain =2 with INM0 node for input or bias, INM1 node for filtering"]
    #[inline(always)]
    pub fn is_B_0xC(&self) -> bool {
        *self == PGA_GAIN_A::B_0xC
    }
    #[doc = "Inverting gain=-3/ Non-inverting gain =4 with INM0 node for input or bias, INM1 node for filtering"]
    #[inline(always)]
    pub fn is_B_0xD(&self) -> bool {
        *self == PGA_GAIN_A::B_0xD
    }
    #[doc = "Inverting gain=-7/ Non-inverting gain =8 with INM0 node for input or bias, INM1 node for filtering"]
    #[inline(always)]
    pub fn is_B_0xE(&self) -> bool {
        *self == PGA_GAIN_A::B_0xE
    }
    #[doc = "Inverting gain=-15/ Non-inverting gain =16 with INM0 node for input or bias, INM1 node for filtering"]
    #[inline(always)]
    pub fn is_B_0xF(&self) -> bool {
        *self == PGA_GAIN_A::B_0xF
    }
}
#[doc = "Field `PGA_GAIN` writer - Operational amplifier Programmable amplifier gain value"]
pub type PGA_GAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PGA_GAIN_A, crate::Safe>;
impl<'a, REG> PGA_GAIN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-inverting internal Gain 2, VREF- referenced"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN_A::B_0x0)
    }
    #[doc = "Non-inverting internal Gain 4, VREF- referenced"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN_A::B_0x1)
    }
    #[doc = "Non-inverting internal Gain 8, VREF- referenced"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN_A::B_0x2)
    }
    #[doc = "Non-inverting internal Gain 16, VREF- referenced"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN_A::B_0x3)
    }
    #[doc = "Non-inverting internal Gain 2 with filtering on INM0, VREF- referenced"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN_A::B_0x4)
    }
    #[doc = "Non-inverting internal Gain 4 with filtering on INM0, VREF- referenced"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN_A::B_0x5)
    }
    #[doc = "Non-inverting internal Gain 8 with filtering on INM0, VREF- referenced"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN_A::B_0x6)
    }
    #[doc = "Non-inverting internal Gain 16 with filtering on INM0, VREF- referenced"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN_A::B_0x7)
    }
    #[doc = "Inverting gain=-1/ Non-inverting gain =2 with INM0 node for input or bias"]
    #[inline(always)]
    pub fn B_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN_A::B_0x8)
    }
    #[doc = "Inverting gain=-3/ Non-inverting gain =4 with INM0 node for input or bias"]
    #[inline(always)]
    pub fn B_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN_A::B_0x9)
    }
    #[doc = "Inverting gain=-7/ Non-inverting gain =8 with INM0 node for input or bias"]
    #[inline(always)]
    pub fn B_0xA(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN_A::B_0xA)
    }
    #[doc = "Inverting gain=-15/ Non-inverting gain =16 with INM0 node for input or bias"]
    #[inline(always)]
    pub fn B_0xB(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN_A::B_0xB)
    }
    #[doc = "Inverting gain=-1/ Non-inverting gain =2 with INM0 node for input or bias, INM1 node for filtering"]
    #[inline(always)]
    pub fn B_0xC(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN_A::B_0xC)
    }
    #[doc = "Inverting gain=-3/ Non-inverting gain =4 with INM0 node for input or bias, INM1 node for filtering"]
    #[inline(always)]
    pub fn B_0xD(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN_A::B_0xD)
    }
    #[doc = "Inverting gain=-7/ Non-inverting gain =8 with INM0 node for input or bias, INM1 node for filtering"]
    #[inline(always)]
    pub fn B_0xE(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN_A::B_0xE)
    }
    #[doc = "Inverting gain=-15/ Non-inverting gain =16 with INM0 node for input or bias, INM1 node for filtering"]
    #[inline(always)]
    pub fn B_0xF(self) -> &'a mut crate::W<REG> {
        self.variant(PGA_GAIN_A::B_0xF)
    }
}
#[doc = "User trimming enable This bit allows to switch from 'factory' AOP offset trimmed values to 'user' AOP offset trimmed values This bit is active for both mode normal and high-power.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USERTRIM_A {
    #[doc = "0: 'factory' trim code used"]
    B_0x0 = 0,
    #[doc = "1: 'user' trim code used"]
    B_0x1 = 1,
}
impl From<USERTRIM_A> for bool {
    #[inline(always)]
    fn from(variant: USERTRIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USERTRIM` reader - User trimming enable This bit allows to switch from 'factory' AOP offset trimmed values to 'user' AOP offset trimmed values This bit is active for both mode normal and high-power."]
pub type USERTRIM_R = crate::BitReader<USERTRIM_A>;
impl USERTRIM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USERTRIM_A {
        match self.bits {
            false => USERTRIM_A::B_0x0,
            true => USERTRIM_A::B_0x1,
        }
    }
    #[doc = "'factory' trim code used"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == USERTRIM_A::B_0x0
    }
    #[doc = "'user' trim code used"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == USERTRIM_A::B_0x1
    }
}
#[doc = "Field `USERTRIM` writer - User trimming enable This bit allows to switch from 'factory' AOP offset trimmed values to 'user' AOP offset trimmed values This bit is active for both mode normal and high-power."]
pub type USERTRIM_W<'a, REG> = crate::BitWriter<'a, REG, USERTRIM_A>;
impl<'a, REG> USERTRIM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "'factory' trim code used"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(USERTRIM_A::B_0x0)
    }
    #[doc = "'user' trim code used"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(USERTRIM_A::B_0x1)
    }
}
#[doc = "OPAMP calibration reference voltage output control (reserved for test)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSTREF_A {
    #[doc = "0: INTVREF of OPAMP is not output"]
    B_0x0 = 0,
    #[doc = "1: INTVREF of OPAMP is output"]
    B_0x1 = 1,
}
impl From<TSTREF_A> for bool {
    #[inline(always)]
    fn from(variant: TSTREF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTREF` reader - OPAMP calibration reference voltage output control (reserved for test)"]
pub type TSTREF_R = crate::BitReader<TSTREF_A>;
impl TSTREF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSTREF_A {
        match self.bits {
            false => TSTREF_A::B_0x0,
            true => TSTREF_A::B_0x1,
        }
    }
    #[doc = "INTVREF of OPAMP is not output"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TSTREF_A::B_0x0
    }
    #[doc = "INTVREF of OPAMP is output"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TSTREF_A::B_0x1
    }
}
#[doc = "Field `TSTREF` writer - OPAMP calibration reference voltage output control (reserved for test)"]
pub type TSTREF_W<'a, REG> = crate::BitWriter<'a, REG, TSTREF_A>;
impl<'a, REG> TSTREF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "INTVREF of OPAMP is not output"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TSTREF_A::B_0x0)
    }
    #[doc = "INTVREF of OPAMP is output"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TSTREF_A::B_0x1)
    }
}
#[doc = "Operational amplifier calibration output OPAMP output status flag. During the calibration mode, OPAMP is used as comparator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALOUT_A {
    #[doc = "0: Non-inverting inverting"]
    B_0x0 = 0,
    #[doc = "1: Non-inverting inverting"]
    B_0x1 = 1,
}
impl From<CALOUT_A> for bool {
    #[inline(always)]
    fn from(variant: CALOUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALOUT` reader - Operational amplifier calibration output OPAMP output status flag. During the calibration mode, OPAMP is used as comparator."]
pub type CALOUT_R = crate::BitReader<CALOUT_A>;
impl CALOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CALOUT_A {
        match self.bits {
            false => CALOUT_A::B_0x0,
            true => CALOUT_A::B_0x1,
        }
    }
    #[doc = "Non-inverting inverting"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CALOUT_A::B_0x0
    }
    #[doc = "Non-inverting inverting"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CALOUT_A::B_0x1
    }
}
impl R {
    #[doc = "Bit 0 - Operational amplifier Enable Note: If OPAMP1 is unconnected in a specific package, it must remain disabled (keep OPAMP1_CSR register default value)."]
    #[inline(always)]
    pub fn OPAEN(&self) -> OPAEN_R {
        OPAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Force internal reference on VP (reserved for test)"]
    #[inline(always)]
    pub fn FORCE_VP(&self) -> FORCE_VP_R {
        FORCE_VP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Non inverted input selection"]
    #[inline(always)]
    pub fn VP_SEL(&self) -> VP_SEL_R {
        VP_SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 5:6 - Inverting input selection"]
    #[inline(always)]
    pub fn VM_SEL(&self) -> VM_SEL_R {
        VM_SEL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 8 - Operational amplifier high-speed mode The operational amplifier must be disable to change this configuration."]
    #[inline(always)]
    pub fn OPAHSM(&self) -> OPAHSM_R {
        OPAHSM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Calibration mode enabled"]
    #[inline(always)]
    pub fn CALON(&self) -> CALON_R {
        CALON_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Calibration selection It is used to select the offset calibration bus used to generate the internal reference voltage when CALON = 1 or FORCE_VP= 1."]
    #[inline(always)]
    pub fn CALSEL(&self) -> CALSEL_R {
        CALSEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:17 - Operational amplifier Programmable amplifier gain value"]
    #[inline(always)]
    pub fn PGA_GAIN(&self) -> PGA_GAIN_R {
        PGA_GAIN_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bit 18 - User trimming enable This bit allows to switch from 'factory' AOP offset trimmed values to 'user' AOP offset trimmed values This bit is active for both mode normal and high-power."]
    #[inline(always)]
    pub fn USERTRIM(&self) -> USERTRIM_R {
        USERTRIM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 29 - OPAMP calibration reference voltage output control (reserved for test)"]
    #[inline(always)]
    pub fn TSTREF(&self) -> TSTREF_R {
        TSTREF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Operational amplifier calibration output OPAMP output status flag. During the calibration mode, OPAMP is used as comparator."]
    #[inline(always)]
    pub fn CALOUT(&self) -> CALOUT_R {
        CALOUT_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Operational amplifier Enable Note: If OPAMP1 is unconnected in a specific package, it must remain disabled (keep OPAMP1_CSR register default value)."]
    #[inline(always)]
    pub fn OPAEN(&mut self) -> OPAEN_W<'_, OPAMP1_CSR_SPEC> {
        OPAEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Force internal reference on VP (reserved for test)"]
    #[inline(always)]
    pub fn FORCE_VP(&mut self) -> FORCE_VP_W<'_, OPAMP1_CSR_SPEC> {
        FORCE_VP_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - Non inverted input selection"]
    #[inline(always)]
    pub fn VP_SEL(&mut self) -> VP_SEL_W<'_, OPAMP1_CSR_SPEC> {
        VP_SEL_W::new(self, 2)
    }
    #[doc = "Bits 5:6 - Inverting input selection"]
    #[inline(always)]
    pub fn VM_SEL(&mut self) -> VM_SEL_W<'_, OPAMP1_CSR_SPEC> {
        VM_SEL_W::new(self, 5)
    }
    #[doc = "Bit 8 - Operational amplifier high-speed mode The operational amplifier must be disable to change this configuration."]
    #[inline(always)]
    pub fn OPAHSM(&mut self) -> OPAHSM_W<'_, OPAMP1_CSR_SPEC> {
        OPAHSM_W::new(self, 8)
    }
    #[doc = "Bit 11 - Calibration mode enabled"]
    #[inline(always)]
    pub fn CALON(&mut self) -> CALON_W<'_, OPAMP1_CSR_SPEC> {
        CALON_W::new(self, 11)
    }
    #[doc = "Bits 12:13 - Calibration selection It is used to select the offset calibration bus used to generate the internal reference voltage when CALON = 1 or FORCE_VP= 1."]
    #[inline(always)]
    pub fn CALSEL(&mut self) -> CALSEL_W<'_, OPAMP1_CSR_SPEC> {
        CALSEL_W::new(self, 12)
    }
    #[doc = "Bits 14:17 - Operational amplifier Programmable amplifier gain value"]
    #[inline(always)]
    pub fn PGA_GAIN(&mut self) -> PGA_GAIN_W<'_, OPAMP1_CSR_SPEC> {
        PGA_GAIN_W::new(self, 14)
    }
    #[doc = "Bit 18 - User trimming enable This bit allows to switch from 'factory' AOP offset trimmed values to 'user' AOP offset trimmed values This bit is active for both mode normal and high-power."]
    #[inline(always)]
    pub fn USERTRIM(&mut self) -> USERTRIM_W<'_, OPAMP1_CSR_SPEC> {
        USERTRIM_W::new(self, 18)
    }
    #[doc = "Bit 29 - OPAMP calibration reference voltage output control (reserved for test)"]
    #[inline(always)]
    pub fn TSTREF(&mut self) -> TSTREF_W<'_, OPAMP1_CSR_SPEC> {
        TSTREF_W::new(self, 29)
    }
}
#[doc = "OPAMP1 control/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`opamp1_csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp1_csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPAMP1_CSR_SPEC;
impl crate::RegisterSpec for OPAMP1_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opamp1_csr::R`](R) reader structure"]
impl crate::Readable for OPAMP1_CSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`opamp1_csr::W`](W) writer structure"]
impl crate::Writable for OPAMP1_CSR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets OPAMP1_CSR to value 0"]
impl crate::Resettable for OPAMP1_CSR_SPEC {}
