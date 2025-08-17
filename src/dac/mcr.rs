#[doc = "Register `MCR` reader"]
pub type R = crate::R<MCR_SPEC>;
#[doc = "Register `MCR` writer"]
pub type W = crate::W<MCR_SPEC>;
#[doc = "DAC channel1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN1 = 0 and bit CEN1 = 0 in the DAC_CR register). If EN1 = 1 or CEN1 = 1 the write operation is ignored. They can be set and cleared by software to select the DAC channel1 mode: DAC channel1 in Normal mode DAC channel1 in sample & hold mode Note: This register can be modified only when EN1 = 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE1_A {
    #[doc = "0: DAC channel1 is connected to external pin with Buffer enabled"]
    B_0x0 = 0,
    #[doc = "1: DAC channel1 is connected to external pin and to on chip peripherals with Buffer enabled"]
    B_0x1 = 1,
    #[doc = "2: DAC channel1 is connected to external pin with Buffer disabled"]
    B_0x2 = 2,
    #[doc = "3: DAC channel1 is connected to on chip peripherals with Buffer disabled"]
    B_0x3 = 3,
    #[doc = "4: DAC channel1 is connected to external pin with Buffer enabled"]
    B_0x4 = 4,
    #[doc = "5: DAC channel1 is connected to external pin and to on chip peripherals with Buffer enabled"]
    B_0x5 = 5,
    #[doc = "6: DAC channel1 is connected to external pin and to on chip peripherals with Buffer disabled"]
    B_0x6 = 6,
    #[doc = "7: DAC channel1 is connected to on chip peripherals with Buffer disabled"]
    B_0x7 = 7,
}
impl From<MODE1_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE1_A {
    type Ux = u8;
}
impl crate::IsEnum for MODE1_A {}
#[doc = "Field `MODE1` reader - DAC channel1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN1 = 0 and bit CEN1 = 0 in the DAC_CR register). If EN1 = 1 or CEN1 = 1 the write operation is ignored. They can be set and cleared by software to select the DAC channel1 mode: DAC channel1 in Normal mode DAC channel1 in sample & hold mode Note: This register can be modified only when EN1 = 0."]
pub type MODE1_R = crate::FieldReader<MODE1_A>;
impl MODE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE1_A {
        match self.bits {
            0 => MODE1_A::B_0x0,
            1 => MODE1_A::B_0x1,
            2 => MODE1_A::B_0x2,
            3 => MODE1_A::B_0x3,
            4 => MODE1_A::B_0x4,
            5 => MODE1_A::B_0x5,
            6 => MODE1_A::B_0x6,
            7 => MODE1_A::B_0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "DAC channel1 is connected to external pin with Buffer enabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MODE1_A::B_0x0
    }
    #[doc = "DAC channel1 is connected to external pin and to on chip peripherals with Buffer enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MODE1_A::B_0x1
    }
    #[doc = "DAC channel1 is connected to external pin with Buffer disabled"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == MODE1_A::B_0x2
    }
    #[doc = "DAC channel1 is connected to on chip peripherals with Buffer disabled"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == MODE1_A::B_0x3
    }
    #[doc = "DAC channel1 is connected to external pin with Buffer enabled"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == MODE1_A::B_0x4
    }
    #[doc = "DAC channel1 is connected to external pin and to on chip peripherals with Buffer enabled"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == MODE1_A::B_0x5
    }
    #[doc = "DAC channel1 is connected to external pin and to on chip peripherals with Buffer disabled"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == MODE1_A::B_0x6
    }
    #[doc = "DAC channel1 is connected to on chip peripherals with Buffer disabled"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == MODE1_A::B_0x7
    }
}
#[doc = "Field `MODE1` writer - DAC channel1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN1 = 0 and bit CEN1 = 0 in the DAC_CR register). If EN1 = 1 or CEN1 = 1 the write operation is ignored. They can be set and cleared by software to select the DAC channel1 mode: DAC channel1 in Normal mode DAC channel1 in sample & hold mode Note: This register can be modified only when EN1 = 0."]
pub type MODE1_W<'a, REG> = crate::FieldWriter<'a, REG, 3, MODE1_A, crate::Safe>;
impl<'a, REG> MODE1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DAC channel1 is connected to external pin with Buffer enabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1_A::B_0x0)
    }
    #[doc = "DAC channel1 is connected to external pin and to on chip peripherals with Buffer enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1_A::B_0x1)
    }
    #[doc = "DAC channel1 is connected to external pin with Buffer disabled"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1_A::B_0x2)
    }
    #[doc = "DAC channel1 is connected to on chip peripherals with Buffer disabled"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1_A::B_0x3)
    }
    #[doc = "DAC channel1 is connected to external pin with Buffer enabled"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1_A::B_0x4)
    }
    #[doc = "DAC channel1 is connected to external pin and to on chip peripherals with Buffer enabled"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1_A::B_0x5)
    }
    #[doc = "DAC channel1 is connected to external pin and to on chip peripherals with Buffer disabled"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1_A::B_0x6)
    }
    #[doc = "DAC channel1 is connected to on chip peripherals with Buffer disabled"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1_A::B_0x7)
    }
}
#[doc = "DAC channel1 DMA double data mode This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMADOUBLE1_A {
    #[doc = "0: DMA Normal mode selected"]
    B_0x0 = 0,
    #[doc = "1: DMA Double data mode selected"]
    B_0x1 = 1,
}
impl From<DMADOUBLE1_A> for bool {
    #[inline(always)]
    fn from(variant: DMADOUBLE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMADOUBLE1` reader - DAC channel1 DMA double data mode This bit is set and cleared by software."]
pub type DMADOUBLE1_R = crate::BitReader<DMADOUBLE1_A>;
impl DMADOUBLE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMADOUBLE1_A {
        match self.bits {
            false => DMADOUBLE1_A::B_0x0,
            true => DMADOUBLE1_A::B_0x1,
        }
    }
    #[doc = "DMA Normal mode selected"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DMADOUBLE1_A::B_0x0
    }
    #[doc = "DMA Double data mode selected"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DMADOUBLE1_A::B_0x1
    }
}
#[doc = "Field `DMADOUBLE1` writer - DAC channel1 DMA double data mode This bit is set and cleared by software."]
pub type DMADOUBLE1_W<'a, REG> = crate::BitWriter<'a, REG, DMADOUBLE1_A>;
impl<'a, REG> DMADOUBLE1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA Normal mode selected"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DMADOUBLE1_A::B_0x0)
    }
    #[doc = "DMA Double data mode selected"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DMADOUBLE1_A::B_0x1)
    }
}
#[doc = "Enable signed format for DAC channel1 This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SINFORMAT1_A {
    #[doc = "0: Input data is in unsigned format"]
    B_0x0 = 0,
    #[doc = "1: Input data is in signed format (2's complement). The MSB bit represents the sign."]
    B_0x1 = 1,
}
impl From<SINFORMAT1_A> for bool {
    #[inline(always)]
    fn from(variant: SINFORMAT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SINFORMAT1` reader - Enable signed format for DAC channel1 This bit is set and cleared by software."]
pub type SINFORMAT1_R = crate::BitReader<SINFORMAT1_A>;
impl SINFORMAT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SINFORMAT1_A {
        match self.bits {
            false => SINFORMAT1_A::B_0x0,
            true => SINFORMAT1_A::B_0x1,
        }
    }
    #[doc = "Input data is in unsigned format"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SINFORMAT1_A::B_0x0
    }
    #[doc = "Input data is in signed format (2's complement). The MSB bit represents the sign."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SINFORMAT1_A::B_0x1
    }
}
#[doc = "Field `SINFORMAT1` writer - Enable signed format for DAC channel1 This bit is set and cleared by software."]
pub type SINFORMAT1_W<'a, REG> = crate::BitWriter<'a, REG, SINFORMAT1_A>;
impl<'a, REG> SINFORMAT1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input data is in unsigned format"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SINFORMAT1_A::B_0x0)
    }
    #[doc = "Input data is in signed format (2's complement). The MSB bit represents the sign."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SINFORMAT1_A::B_0x1)
    }
}
#[doc = "High frequency interface mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HFSEL0_A {
    #[doc = "0: High frequency interface mode disabled"]
    B_0x0 = 0,
    #[doc = "1: High frequency interface mode compatible to AHB80 MHz enabled"]
    B_0x1 = 1,
}
impl From<HFSEL0_A> for bool {
    #[inline(always)]
    fn from(variant: HFSEL0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFSEL0` reader - High frequency interface mode selection"]
pub type HFSEL0_R = crate::BitReader<HFSEL0_A>;
impl HFSEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HFSEL0_A {
        match self.bits {
            false => HFSEL0_A::B_0x0,
            true => HFSEL0_A::B_0x1,
        }
    }
    #[doc = "High frequency interface mode disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HFSEL0_A::B_0x0
    }
    #[doc = "High frequency interface mode compatible to AHB80 MHz enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HFSEL0_A::B_0x1
    }
}
#[doc = "Field `HFSEL0` writer - High frequency interface mode selection"]
pub type HFSEL0_W<'a, REG> = crate::BitWriter<'a, REG, HFSEL0_A>;
impl<'a, REG> HFSEL0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "High frequency interface mode disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HFSEL0_A::B_0x0)
    }
    #[doc = "High frequency interface mode compatible to AHB80 MHz enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HFSEL0_A::B_0x1)
    }
}
#[doc = "High frequency interface mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HFSEL1_A {
    #[doc = "0: High frequency interface mode disabled"]
    B_0x0 = 0,
    #[doc = "1: High frequency interface mode compatible to AHB80 MHz enabled"]
    B_0x1 = 1,
}
impl From<HFSEL1_A> for bool {
    #[inline(always)]
    fn from(variant: HFSEL1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFSEL1` reader - High frequency interface mode selection"]
pub type HFSEL1_R = crate::BitReader<HFSEL1_A>;
impl HFSEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HFSEL1_A {
        match self.bits {
            false => HFSEL1_A::B_0x0,
            true => HFSEL1_A::B_0x1,
        }
    }
    #[doc = "High frequency interface mode disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HFSEL1_A::B_0x0
    }
    #[doc = "High frequency interface mode compatible to AHB80 MHz enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HFSEL1_A::B_0x1
    }
}
#[doc = "Field `HFSEL1` writer - High frequency interface mode selection"]
pub type HFSEL1_W<'a, REG> = crate::BitWriter<'a, REG, HFSEL1_A>;
impl<'a, REG> HFSEL1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "High frequency interface mode disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HFSEL1_A::B_0x0)
    }
    #[doc = "High frequency interface mode compatible to AHB80 MHz enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HFSEL1_A::B_0x1)
    }
}
#[doc = "DAC channel2 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN2 = 0 and bit CEN2 = 0 in the DAC_CR register). If EN2 = 1 or CEN2 = 1 the write operation is ignored. They can be set and cleared by software to select the DAC channel2 mode: DAC channel2 in Normal mode DAC channel2 in Sample and hold mode Note: This register can be modified only when EN2 = 0. Refer to for the availability of DAC channel2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE2_A {
    #[doc = "0: DAC channel2 is connected to external pin with Buffer enabled"]
    B_0x0 = 0,
    #[doc = "2: DAC channel2 is connected to external pin with buffer disabled"]
    B_0x2 = 2,
    #[doc = "4: DAC channel2 is connected to external pin with Buffer enabled"]
    B_0x4 = 4,
    #[doc = "6: DAC channel2 is connected to external pin with Buffer disabled"]
    B_0x6 = 6,
}
impl From<MODE2_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE2_A {
    type Ux = u8;
}
impl crate::IsEnum for MODE2_A {}
#[doc = "Field `MODE2` reader - DAC channel2 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN2 = 0 and bit CEN2 = 0 in the DAC_CR register). If EN2 = 1 or CEN2 = 1 the write operation is ignored. They can be set and cleared by software to select the DAC channel2 mode: DAC channel2 in Normal mode DAC channel2 in Sample and hold mode Note: This register can be modified only when EN2 = 0. Refer to for the availability of DAC channel2."]
pub type MODE2_R = crate::FieldReader<MODE2_A>;
impl MODE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MODE2_A> {
        match self.bits {
            0 => Some(MODE2_A::B_0x0),
            2 => Some(MODE2_A::B_0x2),
            4 => Some(MODE2_A::B_0x4),
            6 => Some(MODE2_A::B_0x6),
            _ => None,
        }
    }
    #[doc = "DAC channel2 is connected to external pin with Buffer enabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MODE2_A::B_0x0
    }
    #[doc = "DAC channel2 is connected to external pin with buffer disabled"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == MODE2_A::B_0x2
    }
    #[doc = "DAC channel2 is connected to external pin with Buffer enabled"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == MODE2_A::B_0x4
    }
    #[doc = "DAC channel2 is connected to external pin with Buffer disabled"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == MODE2_A::B_0x6
    }
}
#[doc = "Field `MODE2` writer - DAC channel2 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN2 = 0 and bit CEN2 = 0 in the DAC_CR register). If EN2 = 1 or CEN2 = 1 the write operation is ignored. They can be set and cleared by software to select the DAC channel2 mode: DAC channel2 in Normal mode DAC channel2 in Sample and hold mode Note: This register can be modified only when EN2 = 0. Refer to for the availability of DAC channel2."]
pub type MODE2_W<'a, REG> = crate::FieldWriter<'a, REG, 3, MODE2_A>;
impl<'a, REG> MODE2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DAC channel2 is connected to external pin with Buffer enabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MODE2_A::B_0x0)
    }
    #[doc = "DAC channel2 is connected to external pin with buffer disabled"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MODE2_A::B_0x2)
    }
    #[doc = "DAC channel2 is connected to external pin with Buffer enabled"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(MODE2_A::B_0x4)
    }
    #[doc = "DAC channel2 is connected to external pin with Buffer disabled"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(MODE2_A::B_0x6)
    }
}
#[doc = "DAC channel2 DMA double data mode This bit is set and cleared by software. Note: This bit is available only on dual-channel DACs. Refer to implementation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMADOUBLE2_A {
    #[doc = "0: DMA Normal mode selected"]
    B_0x0 = 0,
    #[doc = "1: DMA Double data mode selected"]
    B_0x1 = 1,
}
impl From<DMADOUBLE2_A> for bool {
    #[inline(always)]
    fn from(variant: DMADOUBLE2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMADOUBLE2` reader - DAC channel2 DMA double data mode This bit is set and cleared by software. Note: This bit is available only on dual-channel DACs. Refer to implementation."]
pub type DMADOUBLE2_R = crate::BitReader<DMADOUBLE2_A>;
impl DMADOUBLE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMADOUBLE2_A {
        match self.bits {
            false => DMADOUBLE2_A::B_0x0,
            true => DMADOUBLE2_A::B_0x1,
        }
    }
    #[doc = "DMA Normal mode selected"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DMADOUBLE2_A::B_0x0
    }
    #[doc = "DMA Double data mode selected"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DMADOUBLE2_A::B_0x1
    }
}
#[doc = "Field `DMADOUBLE2` writer - DAC channel2 DMA double data mode This bit is set and cleared by software. Note: This bit is available only on dual-channel DACs. Refer to implementation."]
pub type DMADOUBLE2_W<'a, REG> = crate::BitWriter<'a, REG, DMADOUBLE2_A>;
impl<'a, REG> DMADOUBLE2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA Normal mode selected"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DMADOUBLE2_A::B_0x0)
    }
    #[doc = "DMA Double data mode selected"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DMADOUBLE2_A::B_0x1)
    }
}
#[doc = "Enable signed format for DAC channel2 This bit is set and cleared by software. Note: This bit is available only on dual-channel DACs. Refer to implementation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SINFORMAT2_A {
    #[doc = "0: Input data is in unsigned format"]
    B_0x0 = 0,
    #[doc = "1: Input data is in signed format (2's complement). The MSB bit represents the sign."]
    B_0x1 = 1,
}
impl From<SINFORMAT2_A> for bool {
    #[inline(always)]
    fn from(variant: SINFORMAT2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SINFORMAT2` reader - Enable signed format for DAC channel2 This bit is set and cleared by software. Note: This bit is available only on dual-channel DACs. Refer to implementation."]
pub type SINFORMAT2_R = crate::BitReader<SINFORMAT2_A>;
impl SINFORMAT2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SINFORMAT2_A {
        match self.bits {
            false => SINFORMAT2_A::B_0x0,
            true => SINFORMAT2_A::B_0x1,
        }
    }
    #[doc = "Input data is in unsigned format"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SINFORMAT2_A::B_0x0
    }
    #[doc = "Input data is in signed format (2's complement). The MSB bit represents the sign."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SINFORMAT2_A::B_0x1
    }
}
#[doc = "Field `SINFORMAT2` writer - Enable signed format for DAC channel2 This bit is set and cleared by software. Note: This bit is available only on dual-channel DACs. Refer to implementation."]
pub type SINFORMAT2_W<'a, REG> = crate::BitWriter<'a, REG, SINFORMAT2_A>;
impl<'a, REG> SINFORMAT2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input data is in unsigned format"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SINFORMAT2_A::B_0x0)
    }
    #[doc = "Input data is in signed format (2's complement). The MSB bit represents the sign."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SINFORMAT2_A::B_0x1)
    }
}
impl R {
    #[doc = "Bits 0:2 - DAC channel1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN1 = 0 and bit CEN1 = 0 in the DAC_CR register). If EN1 = 1 or CEN1 = 1 the write operation is ignored. They can be set and cleared by software to select the DAC channel1 mode: DAC channel1 in Normal mode DAC channel1 in sample & hold mode Note: This register can be modified only when EN1 = 0."]
    #[inline(always)]
    pub fn MODE1(&self) -> MODE1_R {
        MODE1_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - DAC channel1 DMA double data mode This bit is set and cleared by software."]
    #[inline(always)]
    pub fn DMADOUBLE1(&self) -> DMADOUBLE1_R {
        DMADOUBLE1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable signed format for DAC channel1 This bit is set and cleared by software."]
    #[inline(always)]
    pub fn SINFORMAT1(&self) -> SINFORMAT1_R {
        SINFORMAT1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - High frequency interface mode selection"]
    #[inline(always)]
    pub fn HFSEL0(&self) -> HFSEL0_R {
        HFSEL0_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - High frequency interface mode selection"]
    #[inline(always)]
    pub fn HFSEL1(&self) -> HFSEL1_R {
        HFSEL1_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - DAC channel2 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN2 = 0 and bit CEN2 = 0 in the DAC_CR register). If EN2 = 1 or CEN2 = 1 the write operation is ignored. They can be set and cleared by software to select the DAC channel2 mode: DAC channel2 in Normal mode DAC channel2 in Sample and hold mode Note: This register can be modified only when EN2 = 0. Refer to for the availability of DAC channel2."]
    #[inline(always)]
    pub fn MODE2(&self) -> MODE2_R {
        MODE2_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 24 - DAC channel2 DMA double data mode This bit is set and cleared by software. Note: This bit is available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn DMADOUBLE2(&self) -> DMADOUBLE2_R {
        DMADOUBLE2_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable signed format for DAC channel2 This bit is set and cleared by software. Note: This bit is available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn SINFORMAT2(&self) -> SINFORMAT2_R {
        SINFORMAT2_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - DAC channel1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN1 = 0 and bit CEN1 = 0 in the DAC_CR register). If EN1 = 1 or CEN1 = 1 the write operation is ignored. They can be set and cleared by software to select the DAC channel1 mode: DAC channel1 in Normal mode DAC channel1 in sample & hold mode Note: This register can be modified only when EN1 = 0."]
    #[inline(always)]
    pub fn MODE1(&mut self) -> MODE1_W<'_, MCR_SPEC> {
        MODE1_W::new(self, 0)
    }
    #[doc = "Bit 8 - DAC channel1 DMA double data mode This bit is set and cleared by software."]
    #[inline(always)]
    pub fn DMADOUBLE1(&mut self) -> DMADOUBLE1_W<'_, MCR_SPEC> {
        DMADOUBLE1_W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable signed format for DAC channel1 This bit is set and cleared by software."]
    #[inline(always)]
    pub fn SINFORMAT1(&mut self) -> SINFORMAT1_W<'_, MCR_SPEC> {
        SINFORMAT1_W::new(self, 9)
    }
    #[doc = "Bit 14 - High frequency interface mode selection"]
    #[inline(always)]
    pub fn HFSEL0(&mut self) -> HFSEL0_W<'_, MCR_SPEC> {
        HFSEL0_W::new(self, 14)
    }
    #[doc = "Bit 15 - High frequency interface mode selection"]
    #[inline(always)]
    pub fn HFSEL1(&mut self) -> HFSEL1_W<'_, MCR_SPEC> {
        HFSEL1_W::new(self, 15)
    }
    #[doc = "Bits 16:18 - DAC channel2 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN2 = 0 and bit CEN2 = 0 in the DAC_CR register). If EN2 = 1 or CEN2 = 1 the write operation is ignored. They can be set and cleared by software to select the DAC channel2 mode: DAC channel2 in Normal mode DAC channel2 in Sample and hold mode Note: This register can be modified only when EN2 = 0. Refer to for the availability of DAC channel2."]
    #[inline(always)]
    pub fn MODE2(&mut self) -> MODE2_W<'_, MCR_SPEC> {
        MODE2_W::new(self, 16)
    }
    #[doc = "Bit 24 - DAC channel2 DMA double data mode This bit is set and cleared by software. Note: This bit is available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn DMADOUBLE2(&mut self) -> DMADOUBLE2_W<'_, MCR_SPEC> {
        DMADOUBLE2_W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable signed format for DAC channel2 This bit is set and cleared by software. Note: This bit is available only on dual-channel DACs. Refer to implementation."]
    #[inline(always)]
    pub fn SINFORMAT2(&mut self) -> SINFORMAT2_W<'_, MCR_SPEC> {
        SINFORMAT2_W::new(self, 25)
    }
}
#[doc = "DAC mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCR_SPEC;
impl crate::RegisterSpec for MCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcr::R`](R) reader structure"]
impl crate::Readable for MCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mcr::W`](W) writer structure"]
impl crate::Writable for MCR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets MCR to value 0"]
impl crate::Resettable for MCR_SPEC {}
