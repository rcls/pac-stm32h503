#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CFGR_SPEC>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CFGR_SPEC>;
#[doc = "Clock selector The CKSEL bit selects which clock source the LPTIM uses:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKSEL_A {
    #[doc = "0: LPTIM is clocked by internal clock source (APB clock or any of the embedded oscillators)"]
    B_0x0 = 0,
    #[doc = "1: LPTIM is clocked by an external clock source through the LPTIM external Input1"]
    B_0x1 = 1,
}
impl From<CKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CKSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKSEL` reader - Clock selector The CKSEL bit selects which clock source the LPTIM uses:"]
pub type CKSEL_R = crate::BitReader<CKSEL_A>;
impl CKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CKSEL_A {
        match self.bits {
            false => CKSEL_A::B_0x0,
            true => CKSEL_A::B_0x1,
        }
    }
    #[doc = "LPTIM is clocked by internal clock source (APB clock or any of the embedded oscillators)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CKSEL_A::B_0x0
    }
    #[doc = "LPTIM is clocked by an external clock source through the LPTIM external Input1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CKSEL_A::B_0x1
    }
}
#[doc = "Field `CKSEL` writer - Clock selector The CKSEL bit selects which clock source the LPTIM uses:"]
pub type CKSEL_W<'a, REG> = crate::BitWriter<'a, REG, CKSEL_A>;
impl<'a, REG> CKSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LPTIM is clocked by internal clock source (APB clock or any of the embedded oscillators)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CKSEL_A::B_0x0)
    }
    #[doc = "LPTIM is clocked by an external clock source through the LPTIM external Input1"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CKSEL_A::B_0x1)
    }
}
#[doc = "Clock Polarity When the LPTIM is clocked by an external clock source, CKPOL bits is used to configure the active edge or edges used by the counter: If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 1 is active. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 2 is active. Refer to for more details about Encoder mode sub-modes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKPOL_A {
    #[doc = "0: the rising edge is the active edge used for counting."]
    B_0x0 = 0,
    #[doc = "1: the falling edge is the active edge used for counting."]
    B_0x1 = 1,
    #[doc = "2: both edges are active edges. When both external clock signal edges are considered active ones, the LPTIM must also be clocked by an internal clock source with a frequency equal to at least four times the external clock frequency.If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 3 is active."]
    B_0x2 = 2,
    #[doc = "3: not allowed"]
    B_0x3 = 3,
}
impl From<CKPOL_A> for u8 {
    #[inline(always)]
    fn from(variant: CKPOL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CKPOL_A {
    type Ux = u8;
}
impl crate::IsEnum for CKPOL_A {}
#[doc = "Field `CKPOL` reader - Clock Polarity When the LPTIM is clocked by an external clock source, CKPOL bits is used to configure the active edge or edges used by the counter: If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 1 is active. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 2 is active. Refer to for more details about Encoder mode sub-modes."]
pub type CKPOL_R = crate::FieldReader<CKPOL_A>;
impl CKPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CKPOL_A {
        match self.bits {
            0 => CKPOL_A::B_0x0,
            1 => CKPOL_A::B_0x1,
            2 => CKPOL_A::B_0x2,
            3 => CKPOL_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "the rising edge is the active edge used for counting."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CKPOL_A::B_0x0
    }
    #[doc = "the falling edge is the active edge used for counting."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CKPOL_A::B_0x1
    }
    #[doc = "both edges are active edges. When both external clock signal edges are considered active ones, the LPTIM must also be clocked by an internal clock source with a frequency equal to at least four times the external clock frequency.If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 3 is active."]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == CKPOL_A::B_0x2
    }
    #[doc = "not allowed"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == CKPOL_A::B_0x3
    }
}
#[doc = "Field `CKPOL` writer - Clock Polarity When the LPTIM is clocked by an external clock source, CKPOL bits is used to configure the active edge or edges used by the counter: If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 1 is active. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 2 is active. Refer to for more details about Encoder mode sub-modes."]
pub type CKPOL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CKPOL_A, crate::Safe>;
impl<'a, REG> CKPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "the rising edge is the active edge used for counting."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CKPOL_A::B_0x0)
    }
    #[doc = "the falling edge is the active edge used for counting."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CKPOL_A::B_0x1)
    }
    #[doc = "both edges are active edges. When both external clock signal edges are considered active ones, the LPTIM must also be clocked by an internal clock source with a frequency equal to at least four times the external clock frequency.If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 3 is active."]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(CKPOL_A::B_0x2)
    }
    #[doc = "not allowed"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(CKPOL_A::B_0x3)
    }
}
#[doc = "Configurable digital filter for external clock The CKFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an external clock signal before it is considered as a valid level transition. An internal clock source must be present to use this feature\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKFLT_A {
    #[doc = "0: any external clock signal level change is considered as a valid transition"]
    B_0x0 = 0,
    #[doc = "1: external clock signal level change must be stable for at least 2 clock periods before it is considered as valid transition."]
    B_0x1 = 1,
    #[doc = "2: external clock signal level change must be stable for at least 4 clock periods before it is considered as valid transition."]
    B_0x2 = 2,
    #[doc = "3: external clock signal level change must be stable for at least 8 clock periods before it is considered as valid transition."]
    B_0x3 = 3,
}
impl From<CKFLT_A> for u8 {
    #[inline(always)]
    fn from(variant: CKFLT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CKFLT_A {
    type Ux = u8;
}
impl crate::IsEnum for CKFLT_A {}
#[doc = "Field `CKFLT` reader - Configurable digital filter for external clock The CKFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an external clock signal before it is considered as a valid level transition. An internal clock source must be present to use this feature"]
pub type CKFLT_R = crate::FieldReader<CKFLT_A>;
impl CKFLT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CKFLT_A {
        match self.bits {
            0 => CKFLT_A::B_0x0,
            1 => CKFLT_A::B_0x1,
            2 => CKFLT_A::B_0x2,
            3 => CKFLT_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "any external clock signal level change is considered as a valid transition"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CKFLT_A::B_0x0
    }
    #[doc = "external clock signal level change must be stable for at least 2 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CKFLT_A::B_0x1
    }
    #[doc = "external clock signal level change must be stable for at least 4 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == CKFLT_A::B_0x2
    }
    #[doc = "external clock signal level change must be stable for at least 8 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == CKFLT_A::B_0x3
    }
}
#[doc = "Field `CKFLT` writer - Configurable digital filter for external clock The CKFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an external clock signal before it is considered as a valid level transition. An internal clock source must be present to use this feature"]
pub type CKFLT_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CKFLT_A, crate::Safe>;
impl<'a, REG> CKFLT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "any external clock signal level change is considered as a valid transition"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CKFLT_A::B_0x0)
    }
    #[doc = "external clock signal level change must be stable for at least 2 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CKFLT_A::B_0x1)
    }
    #[doc = "external clock signal level change must be stable for at least 4 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(CKFLT_A::B_0x2)
    }
    #[doc = "external clock signal level change must be stable for at least 8 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(CKFLT_A::B_0x3)
    }
}
#[doc = "Configurable digital filter for trigger The TRGFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an internal trigger before it is considered as a valid level transition. An internal clock source must be present to use this feature\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRGFLT_A {
    #[doc = "0: any trigger active level change is considered as a valid trigger"]
    B_0x0 = 0,
    #[doc = "1: trigger active level change must be stable for at least 2 clock periods before it is considered as valid trigger."]
    B_0x1 = 1,
    #[doc = "2: trigger active level change must be stable for at least 4 clock periods before it is considered as valid trigger."]
    B_0x2 = 2,
    #[doc = "3: trigger active level change must be stable for at least 8 clock periods before it is considered as valid trigger."]
    B_0x3 = 3,
}
impl From<TRGFLT_A> for u8 {
    #[inline(always)]
    fn from(variant: TRGFLT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TRGFLT_A {
    type Ux = u8;
}
impl crate::IsEnum for TRGFLT_A {}
#[doc = "Field `TRGFLT` reader - Configurable digital filter for trigger The TRGFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an internal trigger before it is considered as a valid level transition. An internal clock source must be present to use this feature"]
pub type TRGFLT_R = crate::FieldReader<TRGFLT_A>;
impl TRGFLT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TRGFLT_A {
        match self.bits {
            0 => TRGFLT_A::B_0x0,
            1 => TRGFLT_A::B_0x1,
            2 => TRGFLT_A::B_0x2,
            3 => TRGFLT_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "any trigger active level change is considered as a valid trigger"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TRGFLT_A::B_0x0
    }
    #[doc = "trigger active level change must be stable for at least 2 clock periods before it is considered as valid trigger."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TRGFLT_A::B_0x1
    }
    #[doc = "trigger active level change must be stable for at least 4 clock periods before it is considered as valid trigger."]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == TRGFLT_A::B_0x2
    }
    #[doc = "trigger active level change must be stable for at least 8 clock periods before it is considered as valid trigger."]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == TRGFLT_A::B_0x3
    }
}
#[doc = "Field `TRGFLT` writer - Configurable digital filter for trigger The TRGFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an internal trigger before it is considered as a valid level transition. An internal clock source must be present to use this feature"]
pub type TRGFLT_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TRGFLT_A, crate::Safe>;
impl<'a, REG> TRGFLT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "any trigger active level change is considered as a valid trigger"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TRGFLT_A::B_0x0)
    }
    #[doc = "trigger active level change must be stable for at least 2 clock periods before it is considered as valid trigger."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TRGFLT_A::B_0x1)
    }
    #[doc = "trigger active level change must be stable for at least 4 clock periods before it is considered as valid trigger."]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(TRGFLT_A::B_0x2)
    }
    #[doc = "trigger active level change must be stable for at least 8 clock periods before it is considered as valid trigger."]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(TRGFLT_A::B_0x3)
    }
}
#[doc = "Clock prescaler The PRESC bits configure the prescaler division factor. It can be one among the following division factors:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESC_A {
    #[doc = "0: /1"]
    B_0x0 = 0,
    #[doc = "1: /2"]
    B_0x1 = 1,
    #[doc = "2: /4"]
    B_0x2 = 2,
    #[doc = "3: /8"]
    B_0x3 = 3,
    #[doc = "4: /16"]
    B_0x4 = 4,
    #[doc = "5: /32"]
    B_0x5 = 5,
    #[doc = "6: /64"]
    B_0x6 = 6,
    #[doc = "7: /128"]
    B_0x7 = 7,
}
impl From<PRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRESC_A {
    type Ux = u8;
}
impl crate::IsEnum for PRESC_A {}
#[doc = "Field `PRESC` reader - Clock prescaler The PRESC bits configure the prescaler division factor. It can be one among the following division factors:"]
pub type PRESC_R = crate::FieldReader<PRESC_A>;
impl PRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRESC_A {
        match self.bits {
            0 => PRESC_A::B_0x0,
            1 => PRESC_A::B_0x1,
            2 => PRESC_A::B_0x2,
            3 => PRESC_A::B_0x3,
            4 => PRESC_A::B_0x4,
            5 => PRESC_A::B_0x5,
            6 => PRESC_A::B_0x6,
            7 => PRESC_A::B_0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "/1"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRESC_A::B_0x0
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRESC_A::B_0x1
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == PRESC_A::B_0x2
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == PRESC_A::B_0x3
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == PRESC_A::B_0x4
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == PRESC_A::B_0x5
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == PRESC_A::B_0x6
    }
    #[doc = "/128"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == PRESC_A::B_0x7
    }
}
#[doc = "Field `PRESC` writer - Clock prescaler The PRESC bits configure the prescaler division factor. It can be one among the following division factors:"]
pub type PRESC_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PRESC_A, crate::Safe>;
impl<'a, REG> PRESC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "/1"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::B_0x0)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::B_0x1)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::B_0x2)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::B_0x3)
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::B_0x4)
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::B_0x5)
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::B_0x6)
    }
    #[doc = "/128"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC_A::B_0x7)
    }
}
#[doc = "Trigger selector The TRIGSEL bits select the trigger source that serves as a trigger event for the LPTIM among the below 8 available sources: See for details.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRIGSEL_A {
    #[doc = "0: lptim_ext_trig0"]
    B_0x0 = 0,
    #[doc = "1: lptim_ext_trig1"]
    B_0x1 = 1,
    #[doc = "2: lptim_ext_trig2"]
    B_0x2 = 2,
    #[doc = "3: lptim_ext_trig3"]
    B_0x3 = 3,
    #[doc = "4: lptim_ext_trig4"]
    B_0x4 = 4,
    #[doc = "5: lptim_ext_trig5"]
    B_0x5 = 5,
    #[doc = "6: lptim_ext_trig6"]
    B_0x6 = 6,
    #[doc = "7: lptim_ext_trig7"]
    B_0x7 = 7,
}
impl From<TRIGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIGSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TRIGSEL_A {
    type Ux = u8;
}
impl crate::IsEnum for TRIGSEL_A {}
#[doc = "Field `TRIGSEL` reader - Trigger selector The TRIGSEL bits select the trigger source that serves as a trigger event for the LPTIM among the below 8 available sources: See for details."]
pub type TRIGSEL_R = crate::FieldReader<TRIGSEL_A>;
impl TRIGSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TRIGSEL_A {
        match self.bits {
            0 => TRIGSEL_A::B_0x0,
            1 => TRIGSEL_A::B_0x1,
            2 => TRIGSEL_A::B_0x2,
            3 => TRIGSEL_A::B_0x3,
            4 => TRIGSEL_A::B_0x4,
            5 => TRIGSEL_A::B_0x5,
            6 => TRIGSEL_A::B_0x6,
            7 => TRIGSEL_A::B_0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "lptim_ext_trig0"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TRIGSEL_A::B_0x0
    }
    #[doc = "lptim_ext_trig1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TRIGSEL_A::B_0x1
    }
    #[doc = "lptim_ext_trig2"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == TRIGSEL_A::B_0x2
    }
    #[doc = "lptim_ext_trig3"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == TRIGSEL_A::B_0x3
    }
    #[doc = "lptim_ext_trig4"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == TRIGSEL_A::B_0x4
    }
    #[doc = "lptim_ext_trig5"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == TRIGSEL_A::B_0x5
    }
    #[doc = "lptim_ext_trig6"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == TRIGSEL_A::B_0x6
    }
    #[doc = "lptim_ext_trig7"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == TRIGSEL_A::B_0x7
    }
}
#[doc = "Field `TRIGSEL` writer - Trigger selector The TRIGSEL bits select the trigger source that serves as a trigger event for the LPTIM among the below 8 available sources: See for details."]
pub type TRIGSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, TRIGSEL_A, crate::Safe>;
impl<'a, REG> TRIGSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "lptim_ext_trig0"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL_A::B_0x0)
    }
    #[doc = "lptim_ext_trig1"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL_A::B_0x1)
    }
    #[doc = "lptim_ext_trig2"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL_A::B_0x2)
    }
    #[doc = "lptim_ext_trig3"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL_A::B_0x3)
    }
    #[doc = "lptim_ext_trig4"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL_A::B_0x4)
    }
    #[doc = "lptim_ext_trig5"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL_A::B_0x5)
    }
    #[doc = "lptim_ext_trig6"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL_A::B_0x6)
    }
    #[doc = "lptim_ext_trig7"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL_A::B_0x7)
    }
}
#[doc = "Trigger enable and polarity The TRIGEN bits controls whether the LPTIM counter is started by an external trigger or not. If the external trigger option is selected, three configurations are possible for the trigger active edge:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRIGEN_A {
    #[doc = "0: software trigger (counting start is initiated by software)"]
    B_0x0 = 0,
    #[doc = "1: rising edge is the active edge"]
    B_0x1 = 1,
    #[doc = "2: falling edge is the active edge"]
    B_0x2 = 2,
    #[doc = "3: both edges are active edges"]
    B_0x3 = 3,
}
impl From<TRIGEN_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIGEN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TRIGEN_A {
    type Ux = u8;
}
impl crate::IsEnum for TRIGEN_A {}
#[doc = "Field `TRIGEN` reader - Trigger enable and polarity The TRIGEN bits controls whether the LPTIM counter is started by an external trigger or not. If the external trigger option is selected, three configurations are possible for the trigger active edge:"]
pub type TRIGEN_R = crate::FieldReader<TRIGEN_A>;
impl TRIGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TRIGEN_A {
        match self.bits {
            0 => TRIGEN_A::B_0x0,
            1 => TRIGEN_A::B_0x1,
            2 => TRIGEN_A::B_0x2,
            3 => TRIGEN_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "software trigger (counting start is initiated by software)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TRIGEN_A::B_0x0
    }
    #[doc = "rising edge is the active edge"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TRIGEN_A::B_0x1
    }
    #[doc = "falling edge is the active edge"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == TRIGEN_A::B_0x2
    }
    #[doc = "both edges are active edges"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == TRIGEN_A::B_0x3
    }
}
#[doc = "Field `TRIGEN` writer - Trigger enable and polarity The TRIGEN bits controls whether the LPTIM counter is started by an external trigger or not. If the external trigger option is selected, three configurations are possible for the trigger active edge:"]
pub type TRIGEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TRIGEN_A, crate::Safe>;
impl<'a, REG> TRIGEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "software trigger (counting start is initiated by software)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGEN_A::B_0x0)
    }
    #[doc = "rising edge is the active edge"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGEN_A::B_0x1)
    }
    #[doc = "falling edge is the active edge"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGEN_A::B_0x2)
    }
    #[doc = "both edges are active edges"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGEN_A::B_0x3)
    }
}
#[doc = "Timeout enable The TIMOUT bit controls the Timeout feature\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMOUT_A {
    #[doc = "0: A trigger event arriving when the timer is already started is ignored"]
    B_0x0 = 0,
    #[doc = "1: A trigger event arriving when the timer is already started resets and restarts the LPTIM counter and the repetition counter"]
    B_0x1 = 1,
}
impl From<TIMOUT_A> for bool {
    #[inline(always)]
    fn from(variant: TIMOUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMOUT` reader - Timeout enable The TIMOUT bit controls the Timeout feature"]
pub type TIMOUT_R = crate::BitReader<TIMOUT_A>;
impl TIMOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIMOUT_A {
        match self.bits {
            false => TIMOUT_A::B_0x0,
            true => TIMOUT_A::B_0x1,
        }
    }
    #[doc = "A trigger event arriving when the timer is already started is ignored"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TIMOUT_A::B_0x0
    }
    #[doc = "A trigger event arriving when the timer is already started resets and restarts the LPTIM counter and the repetition counter"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TIMOUT_A::B_0x1
    }
}
#[doc = "Field `TIMOUT` writer - Timeout enable The TIMOUT bit controls the Timeout feature"]
pub type TIMOUT_W<'a, REG> = crate::BitWriter<'a, REG, TIMOUT_A>;
impl<'a, REG> TIMOUT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A trigger event arriving when the timer is already started is ignored"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TIMOUT_A::B_0x0)
    }
    #[doc = "A trigger event arriving when the timer is already started resets and restarts the LPTIM counter and the repetition counter"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TIMOUT_A::B_0x1)
    }
}
#[doc = "Waveform shape The WAVE bit controls the output shape\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAVE_A {
    #[doc = "0: Deactivate Set-once mode"]
    B_0x0 = 0,
    #[doc = "1: Activate the Set-once mode"]
    B_0x1 = 1,
}
impl From<WAVE_A> for bool {
    #[inline(always)]
    fn from(variant: WAVE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAVE` reader - Waveform shape The WAVE bit controls the output shape"]
pub type WAVE_R = crate::BitReader<WAVE_A>;
impl WAVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WAVE_A {
        match self.bits {
            false => WAVE_A::B_0x0,
            true => WAVE_A::B_0x1,
        }
    }
    #[doc = "Deactivate Set-once mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == WAVE_A::B_0x0
    }
    #[doc = "Activate the Set-once mode"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == WAVE_A::B_0x1
    }
}
#[doc = "Field `WAVE` writer - Waveform shape The WAVE bit controls the output shape"]
pub type WAVE_W<'a, REG> = crate::BitWriter<'a, REG, WAVE_A>;
impl<'a, REG> WAVE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Deactivate Set-once mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WAVE_A::B_0x0)
    }
    #[doc = "Activate the Set-once mode"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WAVE_A::B_0x1)
    }
}
#[doc = "Waveform shape polarity The WAVPOL bit controls the output polarity Note: If the LPTIM implements at least one capture/compare channel, this bit is reserved. Please refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAVPOL_A {
    #[doc = "0: The LPTIM output reflects the compare results between LPTIM_CNT and LPTIM_CCRx registers"]
    B_0x0 = 0,
    #[doc = "1: The LPTIM output reflects the inverse of the compare results between LPTIM_CNT and LPTIM_CCRx registers"]
    B_0x1 = 1,
}
impl From<WAVPOL_A> for bool {
    #[inline(always)]
    fn from(variant: WAVPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAVPOL` reader - Waveform shape polarity The WAVPOL bit controls the output polarity Note: If the LPTIM implements at least one capture/compare channel, this bit is reserved. Please refer to ."]
pub type WAVPOL_R = crate::BitReader<WAVPOL_A>;
impl WAVPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WAVPOL_A {
        match self.bits {
            false => WAVPOL_A::B_0x0,
            true => WAVPOL_A::B_0x1,
        }
    }
    #[doc = "The LPTIM output reflects the compare results between LPTIM_CNT and LPTIM_CCRx registers"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == WAVPOL_A::B_0x0
    }
    #[doc = "The LPTIM output reflects the inverse of the compare results between LPTIM_CNT and LPTIM_CCRx registers"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == WAVPOL_A::B_0x1
    }
}
#[doc = "Field `WAVPOL` writer - Waveform shape polarity The WAVPOL bit controls the output polarity Note: If the LPTIM implements at least one capture/compare channel, this bit is reserved. Please refer to ."]
pub type WAVPOL_W<'a, REG> = crate::BitWriter<'a, REG, WAVPOL_A>;
impl<'a, REG> WAVPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The LPTIM output reflects the compare results between LPTIM_CNT and LPTIM_CCRx registers"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WAVPOL_A::B_0x0)
    }
    #[doc = "The LPTIM output reflects the inverse of the compare results between LPTIM_CNT and LPTIM_CCRx registers"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WAVPOL_A::B_0x1)
    }
}
#[doc = "Registers update mode The PRELOAD bit controls the LPTIM_ARR, LPTIM_RCR and the LPTIM_CCRx registers update modality\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRELOAD_A {
    #[doc = "0: Registers are updated after each APB bus write access"]
    B_0x0 = 0,
    #[doc = "1: Registers are updated at the end of the current LPTIM period"]
    B_0x1 = 1,
}
impl From<PRELOAD_A> for bool {
    #[inline(always)]
    fn from(variant: PRELOAD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRELOAD` reader - Registers update mode The PRELOAD bit controls the LPTIM_ARR, LPTIM_RCR and the LPTIM_CCRx registers update modality"]
pub type PRELOAD_R = crate::BitReader<PRELOAD_A>;
impl PRELOAD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRELOAD_A {
        match self.bits {
            false => PRELOAD_A::B_0x0,
            true => PRELOAD_A::B_0x1,
        }
    }
    #[doc = "Registers are updated after each APB bus write access"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRELOAD_A::B_0x0
    }
    #[doc = "Registers are updated at the end of the current LPTIM period"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRELOAD_A::B_0x1
    }
}
#[doc = "Field `PRELOAD` writer - Registers update mode The PRELOAD bit controls the LPTIM_ARR, LPTIM_RCR and the LPTIM_CCRx registers update modality"]
pub type PRELOAD_W<'a, REG> = crate::BitWriter<'a, REG, PRELOAD_A>;
impl<'a, REG> PRELOAD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Registers are updated after each APB bus write access"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRELOAD_A::B_0x0)
    }
    #[doc = "Registers are updated at the end of the current LPTIM period"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRELOAD_A::B_0x1)
    }
}
#[doc = "counter mode enabled The COUNTMODE bit selects which clock source is used by the LPTIM to clock the counter:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COUNTMODE_A {
    #[doc = "0: the counter is incremented following each internal clock pulse"]
    B_0x0 = 0,
    #[doc = "1: the counter is incremented following each valid clock pulse on the LPTIM external Input1"]
    B_0x1 = 1,
}
impl From<COUNTMODE_A> for bool {
    #[inline(always)]
    fn from(variant: COUNTMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COUNTMODE` reader - counter mode enabled The COUNTMODE bit selects which clock source is used by the LPTIM to clock the counter:"]
pub type COUNTMODE_R = crate::BitReader<COUNTMODE_A>;
impl COUNTMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COUNTMODE_A {
        match self.bits {
            false => COUNTMODE_A::B_0x0,
            true => COUNTMODE_A::B_0x1,
        }
    }
    #[doc = "the counter is incremented following each internal clock pulse"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == COUNTMODE_A::B_0x0
    }
    #[doc = "the counter is incremented following each valid clock pulse on the LPTIM external Input1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == COUNTMODE_A::B_0x1
    }
}
#[doc = "Field `COUNTMODE` writer - counter mode enabled The COUNTMODE bit selects which clock source is used by the LPTIM to clock the counter:"]
pub type COUNTMODE_W<'a, REG> = crate::BitWriter<'a, REG, COUNTMODE_A>;
impl<'a, REG> COUNTMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the counter is incremented following each internal clock pulse"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(COUNTMODE_A::B_0x0)
    }
    #[doc = "the counter is incremented following each valid clock pulse on the LPTIM external Input1"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(COUNTMODE_A::B_0x1)
    }
}
#[doc = "Encoder mode enable The ENC bit controls the Encoder mode Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENC_A {
    #[doc = "0: Encoder mode disabled"]
    B_0x0 = 0,
    #[doc = "1: Encoder mode enabled"]
    B_0x1 = 1,
}
impl From<ENC_A> for bool {
    #[inline(always)]
    fn from(variant: ENC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENC` reader - Encoder mode enable The ENC bit controls the Encoder mode Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
pub type ENC_R = crate::BitReader<ENC_A>;
impl ENC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ENC_A {
        match self.bits {
            false => ENC_A::B_0x0,
            true => ENC_A::B_0x1,
        }
    }
    #[doc = "Encoder mode disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ENC_A::B_0x0
    }
    #[doc = "Encoder mode enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ENC_A::B_0x1
    }
}
#[doc = "Field `ENC` writer - Encoder mode enable The ENC bit controls the Encoder mode Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
pub type ENC_W<'a, REG> = crate::BitWriter<'a, REG, ENC_A>;
impl<'a, REG> ENC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Encoder mode disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ENC_A::B_0x0)
    }
    #[doc = "Encoder mode enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ENC_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Clock selector The CKSEL bit selects which clock source the LPTIM uses:"]
    #[inline(always)]
    pub fn CKSEL(&self) -> CKSEL_R {
        CKSEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Clock Polarity When the LPTIM is clocked by an external clock source, CKPOL bits is used to configure the active edge or edges used by the counter: If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 1 is active. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 2 is active. Refer to for more details about Encoder mode sub-modes."]
    #[inline(always)]
    pub fn CKPOL(&self) -> CKPOL_R {
        CKPOL_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - Configurable digital filter for external clock The CKFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an external clock signal before it is considered as a valid level transition. An internal clock source must be present to use this feature"]
    #[inline(always)]
    pub fn CKFLT(&self) -> CKFLT_R {
        CKFLT_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Configurable digital filter for trigger The TRGFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an internal trigger before it is considered as a valid level transition. An internal clock source must be present to use this feature"]
    #[inline(always)]
    pub fn TRGFLT(&self) -> TRGFLT_R {
        TRGFLT_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 9:11 - Clock prescaler The PRESC bits configure the prescaler division factor. It can be one among the following division factors:"]
    #[inline(always)]
    pub fn PRESC(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 13:15 - Trigger selector The TRIGSEL bits select the trigger source that serves as a trigger event for the LPTIM among the below 8 available sources: See for details."]
    #[inline(always)]
    pub fn TRIGSEL(&self) -> TRIGSEL_R {
        TRIGSEL_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 17:18 - Trigger enable and polarity The TRIGEN bits controls whether the LPTIM counter is started by an external trigger or not. If the external trigger option is selected, three configurations are possible for the trigger active edge:"]
    #[inline(always)]
    pub fn TRIGEN(&self) -> TRIGEN_R {
        TRIGEN_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - Timeout enable The TIMOUT bit controls the Timeout feature"]
    #[inline(always)]
    pub fn TIMOUT(&self) -> TIMOUT_R {
        TIMOUT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Waveform shape The WAVE bit controls the output shape"]
    #[inline(always)]
    pub fn WAVE(&self) -> WAVE_R {
        WAVE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Waveform shape polarity The WAVPOL bit controls the output polarity Note: If the LPTIM implements at least one capture/compare channel, this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn WAVPOL(&self) -> WAVPOL_R {
        WAVPOL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Registers update mode The PRELOAD bit controls the LPTIM_ARR, LPTIM_RCR and the LPTIM_CCRx registers update modality"]
    #[inline(always)]
    pub fn PRELOAD(&self) -> PRELOAD_R {
        PRELOAD_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - counter mode enabled The COUNTMODE bit selects which clock source is used by the LPTIM to clock the counter:"]
    #[inline(always)]
    pub fn COUNTMODE(&self) -> COUNTMODE_R {
        COUNTMODE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Encoder mode enable The ENC bit controls the Encoder mode Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn ENC(&self) -> ENC_R {
        ENC_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock selector The CKSEL bit selects which clock source the LPTIM uses:"]
    #[inline(always)]
    pub fn CKSEL(&mut self) -> CKSEL_W<'_, CFGR_SPEC> {
        CKSEL_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Clock Polarity When the LPTIM is clocked by an external clock source, CKPOL bits is used to configure the active edge or edges used by the counter: If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 1 is active. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 2 is active. Refer to for more details about Encoder mode sub-modes."]
    #[inline(always)]
    pub fn CKPOL(&mut self) -> CKPOL_W<'_, CFGR_SPEC> {
        CKPOL_W::new(self, 1)
    }
    #[doc = "Bits 3:4 - Configurable digital filter for external clock The CKFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an external clock signal before it is considered as a valid level transition. An internal clock source must be present to use this feature"]
    #[inline(always)]
    pub fn CKFLT(&mut self) -> CKFLT_W<'_, CFGR_SPEC> {
        CKFLT_W::new(self, 3)
    }
    #[doc = "Bits 6:7 - Configurable digital filter for trigger The TRGFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an internal trigger before it is considered as a valid level transition. An internal clock source must be present to use this feature"]
    #[inline(always)]
    pub fn TRGFLT(&mut self) -> TRGFLT_W<'_, CFGR_SPEC> {
        TRGFLT_W::new(self, 6)
    }
    #[doc = "Bits 9:11 - Clock prescaler The PRESC bits configure the prescaler division factor. It can be one among the following division factors:"]
    #[inline(always)]
    pub fn PRESC(&mut self) -> PRESC_W<'_, CFGR_SPEC> {
        PRESC_W::new(self, 9)
    }
    #[doc = "Bits 13:15 - Trigger selector The TRIGSEL bits select the trigger source that serves as a trigger event for the LPTIM among the below 8 available sources: See for details."]
    #[inline(always)]
    pub fn TRIGSEL(&mut self) -> TRIGSEL_W<'_, CFGR_SPEC> {
        TRIGSEL_W::new(self, 13)
    }
    #[doc = "Bits 17:18 - Trigger enable and polarity The TRIGEN bits controls whether the LPTIM counter is started by an external trigger or not. If the external trigger option is selected, three configurations are possible for the trigger active edge:"]
    #[inline(always)]
    pub fn TRIGEN(&mut self) -> TRIGEN_W<'_, CFGR_SPEC> {
        TRIGEN_W::new(self, 17)
    }
    #[doc = "Bit 19 - Timeout enable The TIMOUT bit controls the Timeout feature"]
    #[inline(always)]
    pub fn TIMOUT(&mut self) -> TIMOUT_W<'_, CFGR_SPEC> {
        TIMOUT_W::new(self, 19)
    }
    #[doc = "Bit 20 - Waveform shape The WAVE bit controls the output shape"]
    #[inline(always)]
    pub fn WAVE(&mut self) -> WAVE_W<'_, CFGR_SPEC> {
        WAVE_W::new(self, 20)
    }
    #[doc = "Bit 21 - Waveform shape polarity The WAVPOL bit controls the output polarity Note: If the LPTIM implements at least one capture/compare channel, this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn WAVPOL(&mut self) -> WAVPOL_W<'_, CFGR_SPEC> {
        WAVPOL_W::new(self, 21)
    }
    #[doc = "Bit 22 - Registers update mode The PRELOAD bit controls the LPTIM_ARR, LPTIM_RCR and the LPTIM_CCRx registers update modality"]
    #[inline(always)]
    pub fn PRELOAD(&mut self) -> PRELOAD_W<'_, CFGR_SPEC> {
        PRELOAD_W::new(self, 22)
    }
    #[doc = "Bit 23 - counter mode enabled The COUNTMODE bit selects which clock source is used by the LPTIM to clock the counter:"]
    #[inline(always)]
    pub fn COUNTMODE(&mut self) -> COUNTMODE_W<'_, CFGR_SPEC> {
        COUNTMODE_W::new(self, 23)
    }
    #[doc = "Bit 24 - Encoder mode enable The ENC bit controls the Encoder mode Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn ENC(&mut self) -> ENC_W<'_, CFGR_SPEC> {
        ENC_W::new(self, 24)
    }
}
#[doc = "LPTIM configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR_SPEC;
impl crate::RegisterSpec for CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CFGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CFGR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CFGR_SPEC {}
