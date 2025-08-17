#[doc = "Register `CFGR1` reader"]
pub type R = crate::R<CFGR1_SPEC>;
#[doc = "Register `CFGR1` writer"]
pub type W = crate::W<CFGR1_SPEC>;
#[doc = "COMP Channel1 enable This bit is set and cleared by software (only if LOCK not set). It enables the COMP Channel1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_A {
    #[doc = "0: Disable"]
    B_0x0 = 0,
    #[doc = "1: Enable"]
    B_0x1 = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - COMP Channel1 enable This bit is set and cleared by software (only if LOCK not set). It enables the COMP Channel1."]
pub type EN_R = crate::BitReader<EN_A>;
impl EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::B_0x0,
            true => EN_A::B_0x1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EN_A::B_0x0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EN_A::B_0x1
    }
}
#[doc = "Field `EN` writer - COMP Channel1 enable This bit is set and cleared by software (only if LOCK not set). It enables the COMP Channel1."]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG, EN_A>;
impl<'a, REG> EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EN_A::B_0x0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EN_A::B_0x1)
    }
}
#[doc = "Scaler bridge enable This bit is set and cleared by software (only if LOCK not set). This bit enables the bridge of the scaler. If SCALEN is set and BRGEN is reset, all four scaler outputs provide the same level V REF_COMP (similar to V REFINT ). If SCALEN and BRGEN are set, the four scaler outputs provide V REF_COMP , 3/4 V REF_COMP , 1/2 V REF_COMP and 1/4 V REF_COMP levels, respectively.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRGEN_A {
    #[doc = "0: Scaler resistor bridge disabled"]
    B_0x0 = 0,
    #[doc = "1: Scaler resistor bridge enabled"]
    B_0x1 = 1,
}
impl From<BRGEN_A> for bool {
    #[inline(always)]
    fn from(variant: BRGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRGEN` reader - Scaler bridge enable This bit is set and cleared by software (only if LOCK not set). This bit enables the bridge of the scaler. If SCALEN is set and BRGEN is reset, all four scaler outputs provide the same level V REF_COMP (similar to V REFINT ). If SCALEN and BRGEN are set, the four scaler outputs provide V REF_COMP , 3/4 V REF_COMP , 1/2 V REF_COMP and 1/4 V REF_COMP levels, respectively."]
pub type BRGEN_R = crate::BitReader<BRGEN_A>;
impl BRGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BRGEN_A {
        match self.bits {
            false => BRGEN_A::B_0x0,
            true => BRGEN_A::B_0x1,
        }
    }
    #[doc = "Scaler resistor bridge disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BRGEN_A::B_0x0
    }
    #[doc = "Scaler resistor bridge enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BRGEN_A::B_0x1
    }
}
#[doc = "Field `BRGEN` writer - Scaler bridge enable This bit is set and cleared by software (only if LOCK not set). This bit enables the bridge of the scaler. If SCALEN is set and BRGEN is reset, all four scaler outputs provide the same level V REF_COMP (similar to V REFINT ). If SCALEN and BRGEN are set, the four scaler outputs provide V REF_COMP , 3/4 V REF_COMP , 1/2 V REF_COMP and 1/4 V REF_COMP levels, respectively."]
pub type BRGEN_W<'a, REG> = crate::BitWriter<'a, REG, BRGEN_A>;
impl<'a, REG> BRGEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Scaler resistor bridge disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BRGEN_A::B_0x0)
    }
    #[doc = "Scaler resistor bridge enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BRGEN_A::B_0x1)
    }
}
#[doc = "Voltage scaler enable This bit is set and cleared by software (only if LOCK not set). This bit enables the V REFINT scaler for the COMP channels.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCALEN_A {
    #[doc = "0: V REFINT scaler disabled"]
    B_0x0 = 0,
    #[doc = "1: V REFINT scaler enabled"]
    B_0x1 = 1,
}
impl From<SCALEN_A> for bool {
    #[inline(always)]
    fn from(variant: SCALEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCALEN` reader - Voltage scaler enable This bit is set and cleared by software (only if LOCK not set). This bit enables the V REFINT scaler for the COMP channels."]
pub type SCALEN_R = crate::BitReader<SCALEN_A>;
impl SCALEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SCALEN_A {
        match self.bits {
            false => SCALEN_A::B_0x0,
            true => SCALEN_A::B_0x1,
        }
    }
    #[doc = "V REFINT scaler disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SCALEN_A::B_0x0
    }
    #[doc = "V REFINT scaler enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SCALEN_A::B_0x1
    }
}
#[doc = "Field `SCALEN` writer - Voltage scaler enable This bit is set and cleared by software (only if LOCK not set). This bit enables the V REFINT scaler for the COMP channels."]
pub type SCALEN_W<'a, REG> = crate::BitWriter<'a, REG, SCALEN_A>;
impl<'a, REG> SCALEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "V REFINT scaler disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SCALEN_A::B_0x0)
    }
    #[doc = "V REFINT scaler enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SCALEN_A::B_0x1)
    }
}
#[doc = "COMP channel1 polarity selection This bit is set and cleared by software (only if LOCK not set). It inverts COMP channel1 polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POLARITY_A {
    #[doc = "0: COMP channel1 output is not inverted"]
    B_0x0 = 0,
    #[doc = "1: COMP Channel1 output is inverted"]
    B_0x1 = 1,
}
impl From<POLARITY_A> for bool {
    #[inline(always)]
    fn from(variant: POLARITY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POLARITY` reader - COMP channel1 polarity selection This bit is set and cleared by software (only if LOCK not set). It inverts COMP channel1 polarity."]
pub type POLARITY_R = crate::BitReader<POLARITY_A>;
impl POLARITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> POLARITY_A {
        match self.bits {
            false => POLARITY_A::B_0x0,
            true => POLARITY_A::B_0x1,
        }
    }
    #[doc = "COMP channel1 output is not inverted"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == POLARITY_A::B_0x0
    }
    #[doc = "COMP Channel1 output is inverted"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == POLARITY_A::B_0x1
    }
}
#[doc = "Field `POLARITY` writer - COMP channel1 polarity selection This bit is set and cleared by software (only if LOCK not set). It inverts COMP channel1 polarity."]
pub type POLARITY_W<'a, REG> = crate::BitWriter<'a, REG, POLARITY_A>;
impl<'a, REG> POLARITY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "COMP channel1 output is not inverted"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(POLARITY_A::B_0x0)
    }
    #[doc = "COMP Channel1 output is inverted"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(POLARITY_A::B_0x1)
    }
}
#[doc = "COMP channel1 interrupt enable This bit is set and cleared by software (only if LOCK not set). This bit enable the interrupt generation of the COMP channel1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITEN_A {
    #[doc = "0: Interrupt generation disabled for COMP channel1"]
    B_0x0 = 0,
    #[doc = "1: Interrupt generation enabled for COMP channel1"]
    B_0x1 = 1,
}
impl From<ITEN_A> for bool {
    #[inline(always)]
    fn from(variant: ITEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITEN` reader - COMP channel1 interrupt enable This bit is set and cleared by software (only if LOCK not set). This bit enable the interrupt generation of the COMP channel1."]
pub type ITEN_R = crate::BitReader<ITEN_A>;
impl ITEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITEN_A {
        match self.bits {
            false => ITEN_A::B_0x0,
            true => ITEN_A::B_0x1,
        }
    }
    #[doc = "Interrupt generation disabled for COMP channel1"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ITEN_A::B_0x0
    }
    #[doc = "Interrupt generation enabled for COMP channel1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ITEN_A::B_0x1
    }
}
#[doc = "Field `ITEN` writer - COMP channel1 interrupt enable This bit is set and cleared by software (only if LOCK not set). This bit enable the interrupt generation of the COMP channel1."]
pub type ITEN_W<'a, REG> = crate::BitWriter<'a, REG, ITEN_A>;
impl<'a, REG> ITEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt generation disabled for COMP channel1"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ITEN_A::B_0x0)
    }
    #[doc = "Interrupt generation enabled for COMP channel1"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ITEN_A::B_0x1)
    }
}
#[doc = "COMP channel1 hysteresis selection These bits are set and cleared by software (only if LOCK not set). They select the hysteresis voltage of the COMP channel1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HYST_A {
    #[doc = "0: No hysteresis"]
    B_0x0 = 0,
    #[doc = "1: Low hysteresis"]
    B_0x1 = 1,
    #[doc = "2: Medium hysteresis"]
    B_0x2 = 2,
    #[doc = "3: High hysteresis"]
    B_0x3 = 3,
}
impl From<HYST_A> for u8 {
    #[inline(always)]
    fn from(variant: HYST_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HYST_A {
    type Ux = u8;
}
impl crate::IsEnum for HYST_A {}
#[doc = "Field `HYST` reader - COMP channel1 hysteresis selection These bits are set and cleared by software (only if LOCK not set). They select the hysteresis voltage of the COMP channel1."]
pub type HYST_R = crate::FieldReader<HYST_A>;
impl HYST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HYST_A {
        match self.bits {
            0 => HYST_A::B_0x0,
            1 => HYST_A::B_0x1,
            2 => HYST_A::B_0x2,
            3 => HYST_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "No hysteresis"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HYST_A::B_0x0
    }
    #[doc = "Low hysteresis"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HYST_A::B_0x1
    }
    #[doc = "Medium hysteresis"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == HYST_A::B_0x2
    }
    #[doc = "High hysteresis"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == HYST_A::B_0x3
    }
}
#[doc = "Field `HYST` writer - COMP channel1 hysteresis selection These bits are set and cleared by software (only if LOCK not set). They select the hysteresis voltage of the COMP channel1."]
pub type HYST_W<'a, REG> = crate::FieldWriter<'a, REG, 2, HYST_A, crate::Safe>;
impl<'a, REG> HYST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No hysteresis"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HYST_A::B_0x0)
    }
    #[doc = "Low hysteresis"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HYST_A::B_0x1)
    }
    #[doc = "Medium hysteresis"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(HYST_A::B_0x2)
    }
    #[doc = "High hysteresis"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(HYST_A::B_0x3)
    }
}
#[doc = "Power mode of the COMP channel1 These bits are set and cleared by software (only if LOCK not set). They control the power/speed of the COMP channel1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWRMODE_A {
    #[doc = "0: High speed/high power"]
    B_0x0 = 0,
    #[doc = "1: Medium speed/medium power"]
    B_0x1 = 1,
    #[doc = "2: Medium speed/medium power"]
    B_0x2 = 2,
    #[doc = "3: Ultra low power/ultra-low-power"]
    B_0x3 = 3,
}
impl From<PWRMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PWRMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PWRMODE_A {
    type Ux = u8;
}
impl crate::IsEnum for PWRMODE_A {}
#[doc = "Field `PWRMODE` reader - Power mode of the COMP channel1 These bits are set and cleared by software (only if LOCK not set). They control the power/speed of the COMP channel1."]
pub type PWRMODE_R = crate::FieldReader<PWRMODE_A>;
impl PWRMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PWRMODE_A {
        match self.bits {
            0 => PWRMODE_A::B_0x0,
            1 => PWRMODE_A::B_0x1,
            2 => PWRMODE_A::B_0x2,
            3 => PWRMODE_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "High speed/high power"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PWRMODE_A::B_0x0
    }
    #[doc = "Medium speed/medium power"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PWRMODE_A::B_0x1
    }
    #[doc = "Medium speed/medium power"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == PWRMODE_A::B_0x2
    }
    #[doc = "Ultra low power/ultra-low-power"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == PWRMODE_A::B_0x3
    }
}
#[doc = "Field `PWRMODE` writer - Power mode of the COMP channel1 These bits are set and cleared by software (only if LOCK not set). They control the power/speed of the COMP channel1."]
pub type PWRMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PWRMODE_A, crate::Safe>;
impl<'a, REG> PWRMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "High speed/high power"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PWRMODE_A::B_0x0)
    }
    #[doc = "Medium speed/medium power"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PWRMODE_A::B_0x1)
    }
    #[doc = "Medium speed/medium power"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PWRMODE_A::B_0x2)
    }
    #[doc = "Ultra low power/ultra-low-power"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(PWRMODE_A::B_0x3)
    }
}
#[doc = "Field `INMSEL` reader - COMP channel1 inverting input selection These bits are set and cleared by software (only if LOCK not set). They select which input is connected to the input minus of the COMP channel. Note: See Table 146: COMP1 inverting input assignment for more details."]
pub type INMSEL_R = crate::FieldReader;
#[doc = "Field `INMSEL` writer - COMP channel1 inverting input selection These bits are set and cleared by software (only if LOCK not set). They select which input is connected to the input minus of the COMP channel. Note: See Table 146: COMP1 inverting input assignment for more details."]
pub type INMSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `INPSEL1` reader - COMP noninverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of COMP channel. Note: See Table 145: COMP1 noninverting input assignment for more details."]
pub type INPSEL1_R = crate::BitReader;
#[doc = "Field `INPSEL1` writer - COMP noninverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of COMP channel. Note: See Table 145: COMP1 noninverting input assignment for more details."]
pub type INPSEL1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INPSEL2` reader - COMP noninverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of the COMP channel. See Table 145: COMP1 noninverting input assignment for more details."]
pub type INPSEL2_R = crate::BitReader;
#[doc = "Field `INPSEL2` writer - COMP noninverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of the COMP channel. See Table 145: COMP1 noninverting input assignment for more details."]
pub type INPSEL2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "COMP Channel1 blanking source selection Bits of this field are set and cleared by software (only if LOCK not set). The field selects the input source for COMP Channel1 output blanking: All other values: reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BLANKING_A {
    #[doc = "0: No blanking"]
    B_0x0 = 0,
    #[doc = "1: comp_blk1"]
    B_0x1 = 1,
    #[doc = "2: comp_blk2"]
    B_0x2 = 2,
    #[doc = "3: comp_blk3"]
    B_0x3 = 3,
    #[doc = "4: comp_blk4"]
    B_0x4 = 4,
    #[doc = "5: comp_blk5"]
    B_0x5 = 5,
    #[doc = "6: comp_blk6"]
    B_0x6 = 6,
}
impl From<BLANKING_A> for u8 {
    #[inline(always)]
    fn from(variant: BLANKING_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BLANKING_A {
    type Ux = u8;
}
impl crate::IsEnum for BLANKING_A {}
#[doc = "Field `BLANKING` reader - COMP Channel1 blanking source selection Bits of this field are set and cleared by software (only if LOCK not set). The field selects the input source for COMP Channel1 output blanking: All other values: reserved"]
pub type BLANKING_R = crate::FieldReader<BLANKING_A>;
impl BLANKING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BLANKING_A> {
        match self.bits {
            0 => Some(BLANKING_A::B_0x0),
            1 => Some(BLANKING_A::B_0x1),
            2 => Some(BLANKING_A::B_0x2),
            3 => Some(BLANKING_A::B_0x3),
            4 => Some(BLANKING_A::B_0x4),
            5 => Some(BLANKING_A::B_0x5),
            6 => Some(BLANKING_A::B_0x6),
            _ => None,
        }
    }
    #[doc = "No blanking"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BLANKING_A::B_0x0
    }
    #[doc = "comp_blk1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BLANKING_A::B_0x1
    }
    #[doc = "comp_blk2"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == BLANKING_A::B_0x2
    }
    #[doc = "comp_blk3"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == BLANKING_A::B_0x3
    }
    #[doc = "comp_blk4"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == BLANKING_A::B_0x4
    }
    #[doc = "comp_blk5"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == BLANKING_A::B_0x5
    }
    #[doc = "comp_blk6"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == BLANKING_A::B_0x6
    }
}
#[doc = "Field `BLANKING` writer - COMP Channel1 blanking source selection Bits of this field are set and cleared by software (only if LOCK not set). The field selects the input source for COMP Channel1 output blanking: All other values: reserved"]
pub type BLANKING_W<'a, REG> = crate::FieldWriter<'a, REG, 4, BLANKING_A>;
impl<'a, REG> BLANKING_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No blanking"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BLANKING_A::B_0x0)
    }
    #[doc = "comp_blk1"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BLANKING_A::B_0x1)
    }
    #[doc = "comp_blk2"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(BLANKING_A::B_0x2)
    }
    #[doc = "comp_blk3"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(BLANKING_A::B_0x3)
    }
    #[doc = "comp_blk4"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(BLANKING_A::B_0x4)
    }
    #[doc = "comp_blk5"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(BLANKING_A::B_0x5)
    }
    #[doc = "comp_blk6"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(BLANKING_A::B_0x6)
    }
}
#[doc = "Lock This bit is set by software and cleared by a hardware system reset. It locks the whole content of the COMP Channel1 configuration register COMP_CFGR1\\[31:0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCK_A {
    #[doc = "0: COMP_CFGR1\\[31:0\\] register is read/write"]
    B_0x0 = 0,
    #[doc = "1: COMP_CFGR1\\[31:0\\] is read-only"]
    B_0x1 = 1,
}
impl From<LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK` reader - Lock This bit is set by software and cleared by a hardware system reset. It locks the whole content of the COMP Channel1 configuration register COMP_CFGR1\\[31:0\\]"]
pub type LOCK_R = crate::BitReader<LOCK_A>;
impl LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LOCK_A {
        match self.bits {
            false => LOCK_A::B_0x0,
            true => LOCK_A::B_0x1,
        }
    }
    #[doc = "COMP_CFGR1\\[31:0\\] register is read/write"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LOCK_A::B_0x0
    }
    #[doc = "COMP_CFGR1\\[31:0\\] is read-only"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LOCK_A::B_0x1
    }
}
#[doc = "Field `LOCK` writer - Lock This bit is set by software and cleared by a hardware system reset. It locks the whole content of the COMP Channel1 configuration register COMP_CFGR1\\[31:0\\]"]
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG, LOCK_A>;
impl<'a, REG> LOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "COMP_CFGR1\\[31:0\\] register is read/write"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK_A::B_0x0)
    }
    #[doc = "COMP_CFGR1\\[31:0\\] is read-only"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - COMP Channel1 enable This bit is set and cleared by software (only if LOCK not set). It enables the COMP Channel1."]
    #[inline(always)]
    pub fn EN(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Scaler bridge enable This bit is set and cleared by software (only if LOCK not set). This bit enables the bridge of the scaler. If SCALEN is set and BRGEN is reset, all four scaler outputs provide the same level V REF_COMP (similar to V REFINT ). If SCALEN and BRGEN are set, the four scaler outputs provide V REF_COMP , 3/4 V REF_COMP , 1/2 V REF_COMP and 1/4 V REF_COMP levels, respectively."]
    #[inline(always)]
    pub fn BRGEN(&self) -> BRGEN_R {
        BRGEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Voltage scaler enable This bit is set and cleared by software (only if LOCK not set). This bit enables the V REFINT scaler for the COMP channels."]
    #[inline(always)]
    pub fn SCALEN(&self) -> SCALEN_R {
        SCALEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - COMP channel1 polarity selection This bit is set and cleared by software (only if LOCK not set). It inverts COMP channel1 polarity."]
    #[inline(always)]
    pub fn POLARITY(&self) -> POLARITY_R {
        POLARITY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - COMP channel1 interrupt enable This bit is set and cleared by software (only if LOCK not set). This bit enable the interrupt generation of the COMP channel1."]
    #[inline(always)]
    pub fn ITEN(&self) -> ITEN_R {
        ITEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9 - COMP channel1 hysteresis selection These bits are set and cleared by software (only if LOCK not set). They select the hysteresis voltage of the COMP channel1."]
    #[inline(always)]
    pub fn HYST(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Power mode of the COMP channel1 These bits are set and cleared by software (only if LOCK not set). They control the power/speed of the COMP channel1."]
    #[inline(always)]
    pub fn PWRMODE(&self) -> PWRMODE_R {
        PWRMODE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:19 - COMP channel1 inverting input selection These bits are set and cleared by software (only if LOCK not set). They select which input is connected to the input minus of the COMP channel. Note: See Table 146: COMP1 inverting input assignment for more details."]
    #[inline(always)]
    pub fn INMSEL(&self) -> INMSEL_R {
        INMSEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - COMP noninverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of COMP channel. Note: See Table 145: COMP1 noninverting input assignment for more details."]
    #[inline(always)]
    pub fn INPSEL1(&self) -> INPSEL1_R {
        INPSEL1_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - COMP noninverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of the COMP channel. See Table 145: COMP1 noninverting input assignment for more details."]
    #[inline(always)]
    pub fn INPSEL2(&self) -> INPSEL2_R {
        INPSEL2_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:27 - COMP Channel1 blanking source selection Bits of this field are set and cleared by software (only if LOCK not set). The field selects the input source for COMP Channel1 output blanking: All other values: reserved"]
    #[inline(always)]
    pub fn BLANKING(&self) -> BLANKING_R {
        BLANKING_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Lock This bit is set by software and cleared by a hardware system reset. It locks the whole content of the COMP Channel1 configuration register COMP_CFGR1\\[31:0\\]"]
    #[inline(always)]
    pub fn LOCK(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - COMP Channel1 enable This bit is set and cleared by software (only if LOCK not set). It enables the COMP Channel1."]
    #[inline(always)]
    pub fn EN(&mut self) -> EN_W<'_, CFGR1_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Scaler bridge enable This bit is set and cleared by software (only if LOCK not set). This bit enables the bridge of the scaler. If SCALEN is set and BRGEN is reset, all four scaler outputs provide the same level V REF_COMP (similar to V REFINT ). If SCALEN and BRGEN are set, the four scaler outputs provide V REF_COMP , 3/4 V REF_COMP , 1/2 V REF_COMP and 1/4 V REF_COMP levels, respectively."]
    #[inline(always)]
    pub fn BRGEN(&mut self) -> BRGEN_W<'_, CFGR1_SPEC> {
        BRGEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Voltage scaler enable This bit is set and cleared by software (only if LOCK not set). This bit enables the V REFINT scaler for the COMP channels."]
    #[inline(always)]
    pub fn SCALEN(&mut self) -> SCALEN_W<'_, CFGR1_SPEC> {
        SCALEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - COMP channel1 polarity selection This bit is set and cleared by software (only if LOCK not set). It inverts COMP channel1 polarity."]
    #[inline(always)]
    pub fn POLARITY(&mut self) -> POLARITY_W<'_, CFGR1_SPEC> {
        POLARITY_W::new(self, 3)
    }
    #[doc = "Bit 6 - COMP channel1 interrupt enable This bit is set and cleared by software (only if LOCK not set). This bit enable the interrupt generation of the COMP channel1."]
    #[inline(always)]
    pub fn ITEN(&mut self) -> ITEN_W<'_, CFGR1_SPEC> {
        ITEN_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - COMP channel1 hysteresis selection These bits are set and cleared by software (only if LOCK not set). They select the hysteresis voltage of the COMP channel1."]
    #[inline(always)]
    pub fn HYST(&mut self) -> HYST_W<'_, CFGR1_SPEC> {
        HYST_W::new(self, 8)
    }
    #[doc = "Bits 12:13 - Power mode of the COMP channel1 These bits are set and cleared by software (only if LOCK not set). They control the power/speed of the COMP channel1."]
    #[inline(always)]
    pub fn PWRMODE(&mut self) -> PWRMODE_W<'_, CFGR1_SPEC> {
        PWRMODE_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - COMP channel1 inverting input selection These bits are set and cleared by software (only if LOCK not set). They select which input is connected to the input minus of the COMP channel. Note: See Table 146: COMP1 inverting input assignment for more details."]
    #[inline(always)]
    pub fn INMSEL(&mut self) -> INMSEL_W<'_, CFGR1_SPEC> {
        INMSEL_W::new(self, 16)
    }
    #[doc = "Bit 20 - COMP noninverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of COMP channel. Note: See Table 145: COMP1 noninverting input assignment for more details."]
    #[inline(always)]
    pub fn INPSEL1(&mut self) -> INPSEL1_W<'_, CFGR1_SPEC> {
        INPSEL1_W::new(self, 20)
    }
    #[doc = "Bit 22 - COMP noninverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of the COMP channel. See Table 145: COMP1 noninverting input assignment for more details."]
    #[inline(always)]
    pub fn INPSEL2(&mut self) -> INPSEL2_W<'_, CFGR1_SPEC> {
        INPSEL2_W::new(self, 22)
    }
    #[doc = "Bits 24:27 - COMP Channel1 blanking source selection Bits of this field are set and cleared by software (only if LOCK not set). The field selects the input source for COMP Channel1 output blanking: All other values: reserved"]
    #[inline(always)]
    pub fn BLANKING(&mut self) -> BLANKING_W<'_, CFGR1_SPEC> {
        BLANKING_W::new(self, 24)
    }
    #[doc = "Bit 31 - Lock This bit is set by software and cleared by a hardware system reset. It locks the whole content of the COMP Channel1 configuration register COMP_CFGR1\\[31:0\\]"]
    #[inline(always)]
    pub fn LOCK(&mut self) -> LOCK_W<'_, CFGR1_SPEC> {
        LOCK_W::new(self, 31)
    }
}
#[doc = "Comparator configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR1_SPEC;
impl crate::RegisterSpec for CFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr1::R`](R) reader structure"]
impl crate::Readable for CFGR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfgr1::W`](W) writer structure"]
impl crate::Writable for CFGR1_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CFGR1 to value 0"]
impl crate::Resettable for CFGR1_SPEC {}
