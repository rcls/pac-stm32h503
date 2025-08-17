#[doc = "Register `SMPR1` reader"]
pub type R = crate::R<SMPR1_SPEC>;
#[doc = "Register `SMPR1` writer"]
pub type W = crate::W<SMPR1_SPEC>;
#[doc = "Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMP0_A {
    #[doc = "0: 2.5 ADC clock cycles"]
    B_0x0 = 0,
    #[doc = "1: 6.5 ADC clock cycles"]
    B_0x1 = 1,
    #[doc = "2: 12.5 ADC clock cycles"]
    B_0x2 = 2,
    #[doc = "3: 24.5 ADC clock cycles"]
    B_0x3 = 3,
    #[doc = "4: 47.5 ADC clock cycles"]
    B_0x4 = 4,
    #[doc = "5: 92.5 ADC clock cycles"]
    B_0x5 = 5,
    #[doc = "6: 247.5 ADC clock cycles"]
    B_0x6 = 6,
    #[doc = "7: 640.5 ADC clock cycles"]
    B_0x7 = 7,
}
impl From<SMP0_A> for u8 {
    #[inline(always)]
    fn from(variant: SMP0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SMP0_A {
    type Ux = u8;
}
impl crate::IsEnum for SMP0_A {}
#[doc = "Field `SMP0` reader - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
pub type SMP0_R = crate::FieldReader<SMP0_A>;
impl SMP0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMP0_A {
        match self.bits {
            0 => SMP0_A::B_0x0,
            1 => SMP0_A::B_0x1,
            2 => SMP0_A::B_0x2,
            3 => SMP0_A::B_0x3,
            4 => SMP0_A::B_0x4,
            5 => SMP0_A::B_0x5,
            6 => SMP0_A::B_0x6,
            7 => SMP0_A::B_0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SMP0_A::B_0x0
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SMP0_A::B_0x1
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == SMP0_A::B_0x2
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == SMP0_A::B_0x3
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == SMP0_A::B_0x4
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == SMP0_A::B_0x5
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == SMP0_A::B_0x6
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == SMP0_A::B_0x7
    }
}
#[doc = "Field `SMP0` writer - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
pub type SMP0_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SMP0_A, crate::Safe>;
impl<'a, REG> SMP0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMP0_A::B_0x0)
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMP0_A::B_0x1)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(SMP0_A::B_0x2)
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(SMP0_A::B_0x3)
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(SMP0_A::B_0x4)
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP0_A::B_0x5)
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(SMP0_A::B_0x6)
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(SMP0_A::B_0x7)
    }
}
#[doc = "Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMP1_A {
    #[doc = "0: 2.5 ADC clock cycles"]
    B_0x0 = 0,
    #[doc = "1: 6.5 ADC clock cycles"]
    B_0x1 = 1,
    #[doc = "2: 12.5 ADC clock cycles"]
    B_0x2 = 2,
    #[doc = "3: 24.5 ADC clock cycles"]
    B_0x3 = 3,
    #[doc = "4: 47.5 ADC clock cycles"]
    B_0x4 = 4,
    #[doc = "5: 92.5 ADC clock cycles"]
    B_0x5 = 5,
    #[doc = "6: 247.5 ADC clock cycles"]
    B_0x6 = 6,
    #[doc = "7: 640.5 ADC clock cycles"]
    B_0x7 = 7,
}
impl From<SMP1_A> for u8 {
    #[inline(always)]
    fn from(variant: SMP1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SMP1_A {
    type Ux = u8;
}
impl crate::IsEnum for SMP1_A {}
#[doc = "Field `SMP1` reader - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
pub type SMP1_R = crate::FieldReader<SMP1_A>;
impl SMP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMP1_A {
        match self.bits {
            0 => SMP1_A::B_0x0,
            1 => SMP1_A::B_0x1,
            2 => SMP1_A::B_0x2,
            3 => SMP1_A::B_0x3,
            4 => SMP1_A::B_0x4,
            5 => SMP1_A::B_0x5,
            6 => SMP1_A::B_0x6,
            7 => SMP1_A::B_0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SMP1_A::B_0x0
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SMP1_A::B_0x1
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == SMP1_A::B_0x2
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == SMP1_A::B_0x3
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == SMP1_A::B_0x4
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == SMP1_A::B_0x5
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == SMP1_A::B_0x6
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == SMP1_A::B_0x7
    }
}
#[doc = "Field `SMP1` writer - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
pub type SMP1_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SMP1_A, crate::Safe>;
impl<'a, REG> SMP1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1_A::B_0x0)
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1_A::B_0x1)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1_A::B_0x2)
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1_A::B_0x3)
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1_A::B_0x4)
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1_A::B_0x5)
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1_A::B_0x6)
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(SMP1_A::B_0x7)
    }
}
#[doc = "Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMP2_A {
    #[doc = "0: 2.5 ADC clock cycles"]
    B_0x0 = 0,
    #[doc = "1: 6.5 ADC clock cycles"]
    B_0x1 = 1,
    #[doc = "2: 12.5 ADC clock cycles"]
    B_0x2 = 2,
    #[doc = "3: 24.5 ADC clock cycles"]
    B_0x3 = 3,
    #[doc = "4: 47.5 ADC clock cycles"]
    B_0x4 = 4,
    #[doc = "5: 92.5 ADC clock cycles"]
    B_0x5 = 5,
    #[doc = "6: 247.5 ADC clock cycles"]
    B_0x6 = 6,
    #[doc = "7: 640.5 ADC clock cycles"]
    B_0x7 = 7,
}
impl From<SMP2_A> for u8 {
    #[inline(always)]
    fn from(variant: SMP2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SMP2_A {
    type Ux = u8;
}
impl crate::IsEnum for SMP2_A {}
#[doc = "Field `SMP2` reader - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
pub type SMP2_R = crate::FieldReader<SMP2_A>;
impl SMP2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMP2_A {
        match self.bits {
            0 => SMP2_A::B_0x0,
            1 => SMP2_A::B_0x1,
            2 => SMP2_A::B_0x2,
            3 => SMP2_A::B_0x3,
            4 => SMP2_A::B_0x4,
            5 => SMP2_A::B_0x5,
            6 => SMP2_A::B_0x6,
            7 => SMP2_A::B_0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SMP2_A::B_0x0
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SMP2_A::B_0x1
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == SMP2_A::B_0x2
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == SMP2_A::B_0x3
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == SMP2_A::B_0x4
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == SMP2_A::B_0x5
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == SMP2_A::B_0x6
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == SMP2_A::B_0x7
    }
}
#[doc = "Field `SMP2` writer - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
pub type SMP2_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SMP2_A, crate::Safe>;
impl<'a, REG> SMP2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMP2_A::B_0x0)
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMP2_A::B_0x1)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(SMP2_A::B_0x2)
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(SMP2_A::B_0x3)
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(SMP2_A::B_0x4)
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP2_A::B_0x5)
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(SMP2_A::B_0x6)
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(SMP2_A::B_0x7)
    }
}
#[doc = "Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMP3_A {
    #[doc = "0: 2.5 ADC clock cycles"]
    B_0x0 = 0,
    #[doc = "1: 6.5 ADC clock cycles"]
    B_0x1 = 1,
    #[doc = "2: 12.5 ADC clock cycles"]
    B_0x2 = 2,
    #[doc = "3: 24.5 ADC clock cycles"]
    B_0x3 = 3,
    #[doc = "4: 47.5 ADC clock cycles"]
    B_0x4 = 4,
    #[doc = "5: 92.5 ADC clock cycles"]
    B_0x5 = 5,
    #[doc = "6: 247.5 ADC clock cycles"]
    B_0x6 = 6,
    #[doc = "7: 640.5 ADC clock cycles"]
    B_0x7 = 7,
}
impl From<SMP3_A> for u8 {
    #[inline(always)]
    fn from(variant: SMP3_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SMP3_A {
    type Ux = u8;
}
impl crate::IsEnum for SMP3_A {}
#[doc = "Field `SMP3` reader - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
pub type SMP3_R = crate::FieldReader<SMP3_A>;
impl SMP3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMP3_A {
        match self.bits {
            0 => SMP3_A::B_0x0,
            1 => SMP3_A::B_0x1,
            2 => SMP3_A::B_0x2,
            3 => SMP3_A::B_0x3,
            4 => SMP3_A::B_0x4,
            5 => SMP3_A::B_0x5,
            6 => SMP3_A::B_0x6,
            7 => SMP3_A::B_0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SMP3_A::B_0x0
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SMP3_A::B_0x1
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == SMP3_A::B_0x2
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == SMP3_A::B_0x3
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == SMP3_A::B_0x4
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == SMP3_A::B_0x5
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == SMP3_A::B_0x6
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == SMP3_A::B_0x7
    }
}
#[doc = "Field `SMP3` writer - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
pub type SMP3_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SMP3_A, crate::Safe>;
impl<'a, REG> SMP3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMP3_A::B_0x0)
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMP3_A::B_0x1)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(SMP3_A::B_0x2)
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(SMP3_A::B_0x3)
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(SMP3_A::B_0x4)
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP3_A::B_0x5)
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(SMP3_A::B_0x6)
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(SMP3_A::B_0x7)
    }
}
#[doc = "Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMP4_A {
    #[doc = "0: 2.5 ADC clock cycles"]
    B_0x0 = 0,
    #[doc = "1: 6.5 ADC clock cycles"]
    B_0x1 = 1,
    #[doc = "2: 12.5 ADC clock cycles"]
    B_0x2 = 2,
    #[doc = "3: 24.5 ADC clock cycles"]
    B_0x3 = 3,
    #[doc = "4: 47.5 ADC clock cycles"]
    B_0x4 = 4,
    #[doc = "5: 92.5 ADC clock cycles"]
    B_0x5 = 5,
    #[doc = "6: 247.5 ADC clock cycles"]
    B_0x6 = 6,
    #[doc = "7: 640.5 ADC clock cycles"]
    B_0x7 = 7,
}
impl From<SMP4_A> for u8 {
    #[inline(always)]
    fn from(variant: SMP4_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SMP4_A {
    type Ux = u8;
}
impl crate::IsEnum for SMP4_A {}
#[doc = "Field `SMP4` reader - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
pub type SMP4_R = crate::FieldReader<SMP4_A>;
impl SMP4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMP4_A {
        match self.bits {
            0 => SMP4_A::B_0x0,
            1 => SMP4_A::B_0x1,
            2 => SMP4_A::B_0x2,
            3 => SMP4_A::B_0x3,
            4 => SMP4_A::B_0x4,
            5 => SMP4_A::B_0x5,
            6 => SMP4_A::B_0x6,
            7 => SMP4_A::B_0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SMP4_A::B_0x0
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SMP4_A::B_0x1
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == SMP4_A::B_0x2
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == SMP4_A::B_0x3
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == SMP4_A::B_0x4
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == SMP4_A::B_0x5
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == SMP4_A::B_0x6
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == SMP4_A::B_0x7
    }
}
#[doc = "Field `SMP4` writer - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
pub type SMP4_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SMP4_A, crate::Safe>;
impl<'a, REG> SMP4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMP4_A::B_0x0)
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMP4_A::B_0x1)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(SMP4_A::B_0x2)
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(SMP4_A::B_0x3)
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(SMP4_A::B_0x4)
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP4_A::B_0x5)
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(SMP4_A::B_0x6)
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(SMP4_A::B_0x7)
    }
}
#[doc = "Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMP5_A {
    #[doc = "0: 2.5 ADC clock cycles"]
    B_0x0 = 0,
    #[doc = "1: 6.5 ADC clock cycles"]
    B_0x1 = 1,
    #[doc = "2: 12.5 ADC clock cycles"]
    B_0x2 = 2,
    #[doc = "3: 24.5 ADC clock cycles"]
    B_0x3 = 3,
    #[doc = "4: 47.5 ADC clock cycles"]
    B_0x4 = 4,
    #[doc = "5: 92.5 ADC clock cycles"]
    B_0x5 = 5,
    #[doc = "6: 247.5 ADC clock cycles"]
    B_0x6 = 6,
    #[doc = "7: 640.5 ADC clock cycles"]
    B_0x7 = 7,
}
impl From<SMP5_A> for u8 {
    #[inline(always)]
    fn from(variant: SMP5_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SMP5_A {
    type Ux = u8;
}
impl crate::IsEnum for SMP5_A {}
#[doc = "Field `SMP5` reader - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
pub type SMP5_R = crate::FieldReader<SMP5_A>;
impl SMP5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMP5_A {
        match self.bits {
            0 => SMP5_A::B_0x0,
            1 => SMP5_A::B_0x1,
            2 => SMP5_A::B_0x2,
            3 => SMP5_A::B_0x3,
            4 => SMP5_A::B_0x4,
            5 => SMP5_A::B_0x5,
            6 => SMP5_A::B_0x6,
            7 => SMP5_A::B_0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SMP5_A::B_0x0
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SMP5_A::B_0x1
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == SMP5_A::B_0x2
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == SMP5_A::B_0x3
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == SMP5_A::B_0x4
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == SMP5_A::B_0x5
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == SMP5_A::B_0x6
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == SMP5_A::B_0x7
    }
}
#[doc = "Field `SMP5` writer - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
pub type SMP5_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SMP5_A, crate::Safe>;
impl<'a, REG> SMP5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMP5_A::B_0x0)
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMP5_A::B_0x1)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(SMP5_A::B_0x2)
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(SMP5_A::B_0x3)
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(SMP5_A::B_0x4)
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP5_A::B_0x5)
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(SMP5_A::B_0x6)
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(SMP5_A::B_0x7)
    }
}
#[doc = "Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMP6_A {
    #[doc = "0: 2.5 ADC clock cycles"]
    B_0x0 = 0,
    #[doc = "1: 6.5 ADC clock cycles"]
    B_0x1 = 1,
    #[doc = "2: 12.5 ADC clock cycles"]
    B_0x2 = 2,
    #[doc = "3: 24.5 ADC clock cycles"]
    B_0x3 = 3,
    #[doc = "4: 47.5 ADC clock cycles"]
    B_0x4 = 4,
    #[doc = "5: 92.5 ADC clock cycles"]
    B_0x5 = 5,
    #[doc = "6: 247.5 ADC clock cycles"]
    B_0x6 = 6,
    #[doc = "7: 640.5 ADC clock cycles"]
    B_0x7 = 7,
}
impl From<SMP6_A> for u8 {
    #[inline(always)]
    fn from(variant: SMP6_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SMP6_A {
    type Ux = u8;
}
impl crate::IsEnum for SMP6_A {}
#[doc = "Field `SMP6` reader - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
pub type SMP6_R = crate::FieldReader<SMP6_A>;
impl SMP6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMP6_A {
        match self.bits {
            0 => SMP6_A::B_0x0,
            1 => SMP6_A::B_0x1,
            2 => SMP6_A::B_0x2,
            3 => SMP6_A::B_0x3,
            4 => SMP6_A::B_0x4,
            5 => SMP6_A::B_0x5,
            6 => SMP6_A::B_0x6,
            7 => SMP6_A::B_0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SMP6_A::B_0x0
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SMP6_A::B_0x1
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == SMP6_A::B_0x2
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == SMP6_A::B_0x3
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == SMP6_A::B_0x4
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == SMP6_A::B_0x5
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == SMP6_A::B_0x6
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == SMP6_A::B_0x7
    }
}
#[doc = "Field `SMP6` writer - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
pub type SMP6_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SMP6_A, crate::Safe>;
impl<'a, REG> SMP6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMP6_A::B_0x0)
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMP6_A::B_0x1)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(SMP6_A::B_0x2)
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(SMP6_A::B_0x3)
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(SMP6_A::B_0x4)
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP6_A::B_0x5)
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(SMP6_A::B_0x6)
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(SMP6_A::B_0x7)
    }
}
#[doc = "Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMP7_A {
    #[doc = "0: 2.5 ADC clock cycles"]
    B_0x0 = 0,
    #[doc = "1: 6.5 ADC clock cycles"]
    B_0x1 = 1,
    #[doc = "2: 12.5 ADC clock cycles"]
    B_0x2 = 2,
    #[doc = "3: 24.5 ADC clock cycles"]
    B_0x3 = 3,
    #[doc = "4: 47.5 ADC clock cycles"]
    B_0x4 = 4,
    #[doc = "5: 92.5 ADC clock cycles"]
    B_0x5 = 5,
    #[doc = "6: 247.5 ADC clock cycles"]
    B_0x6 = 6,
    #[doc = "7: 640.5 ADC clock cycles"]
    B_0x7 = 7,
}
impl From<SMP7_A> for u8 {
    #[inline(always)]
    fn from(variant: SMP7_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SMP7_A {
    type Ux = u8;
}
impl crate::IsEnum for SMP7_A {}
#[doc = "Field `SMP7` reader - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
pub type SMP7_R = crate::FieldReader<SMP7_A>;
impl SMP7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMP7_A {
        match self.bits {
            0 => SMP7_A::B_0x0,
            1 => SMP7_A::B_0x1,
            2 => SMP7_A::B_0x2,
            3 => SMP7_A::B_0x3,
            4 => SMP7_A::B_0x4,
            5 => SMP7_A::B_0x5,
            6 => SMP7_A::B_0x6,
            7 => SMP7_A::B_0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SMP7_A::B_0x0
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SMP7_A::B_0x1
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == SMP7_A::B_0x2
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == SMP7_A::B_0x3
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == SMP7_A::B_0x4
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == SMP7_A::B_0x5
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == SMP7_A::B_0x6
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == SMP7_A::B_0x7
    }
}
#[doc = "Field `SMP7` writer - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
pub type SMP7_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SMP7_A, crate::Safe>;
impl<'a, REG> SMP7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMP7_A::B_0x0)
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMP7_A::B_0x1)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(SMP7_A::B_0x2)
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(SMP7_A::B_0x3)
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(SMP7_A::B_0x4)
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP7_A::B_0x5)
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(SMP7_A::B_0x6)
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(SMP7_A::B_0x7)
    }
}
#[doc = "Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMP8_A {
    #[doc = "0: 2.5 ADC clock cycles"]
    B_0x0 = 0,
    #[doc = "1: 6.5 ADC clock cycles"]
    B_0x1 = 1,
    #[doc = "2: 12.5 ADC clock cycles"]
    B_0x2 = 2,
    #[doc = "3: 24.5 ADC clock cycles"]
    B_0x3 = 3,
    #[doc = "4: 47.5 ADC clock cycles"]
    B_0x4 = 4,
    #[doc = "5: 92.5 ADC clock cycles"]
    B_0x5 = 5,
    #[doc = "6: 247.5 ADC clock cycles"]
    B_0x6 = 6,
    #[doc = "7: 640.5 ADC clock cycles"]
    B_0x7 = 7,
}
impl From<SMP8_A> for u8 {
    #[inline(always)]
    fn from(variant: SMP8_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SMP8_A {
    type Ux = u8;
}
impl crate::IsEnum for SMP8_A {}
#[doc = "Field `SMP8` reader - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
pub type SMP8_R = crate::FieldReader<SMP8_A>;
impl SMP8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMP8_A {
        match self.bits {
            0 => SMP8_A::B_0x0,
            1 => SMP8_A::B_0x1,
            2 => SMP8_A::B_0x2,
            3 => SMP8_A::B_0x3,
            4 => SMP8_A::B_0x4,
            5 => SMP8_A::B_0x5,
            6 => SMP8_A::B_0x6,
            7 => SMP8_A::B_0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SMP8_A::B_0x0
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SMP8_A::B_0x1
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == SMP8_A::B_0x2
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == SMP8_A::B_0x3
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == SMP8_A::B_0x4
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == SMP8_A::B_0x5
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == SMP8_A::B_0x6
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == SMP8_A::B_0x7
    }
}
#[doc = "Field `SMP8` writer - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
pub type SMP8_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SMP8_A, crate::Safe>;
impl<'a, REG> SMP8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMP8_A::B_0x0)
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMP8_A::B_0x1)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(SMP8_A::B_0x2)
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(SMP8_A::B_0x3)
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(SMP8_A::B_0x4)
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP8_A::B_0x5)
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(SMP8_A::B_0x6)
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(SMP8_A::B_0x7)
    }
}
#[doc = "Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMP9_A {
    #[doc = "0: 2.5 ADC clock cycles"]
    B_0x0 = 0,
    #[doc = "1: 6.5 ADC clock cycles"]
    B_0x1 = 1,
    #[doc = "2: 12.5 ADC clock cycles"]
    B_0x2 = 2,
    #[doc = "3: 24.5 ADC clock cycles"]
    B_0x3 = 3,
    #[doc = "4: 47.5 ADC clock cycles"]
    B_0x4 = 4,
    #[doc = "5: 92.5 ADC clock cycles"]
    B_0x5 = 5,
    #[doc = "6: 247.5 ADC clock cycles"]
    B_0x6 = 6,
    #[doc = "7: 640.5 ADC clock cycles"]
    B_0x7 = 7,
}
impl From<SMP9_A> for u8 {
    #[inline(always)]
    fn from(variant: SMP9_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SMP9_A {
    type Ux = u8;
}
impl crate::IsEnum for SMP9_A {}
#[doc = "Field `SMP9` reader - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
pub type SMP9_R = crate::FieldReader<SMP9_A>;
impl SMP9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMP9_A {
        match self.bits {
            0 => SMP9_A::B_0x0,
            1 => SMP9_A::B_0x1,
            2 => SMP9_A::B_0x2,
            3 => SMP9_A::B_0x3,
            4 => SMP9_A::B_0x4,
            5 => SMP9_A::B_0x5,
            6 => SMP9_A::B_0x6,
            7 => SMP9_A::B_0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SMP9_A::B_0x0
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SMP9_A::B_0x1
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == SMP9_A::B_0x2
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == SMP9_A::B_0x3
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == SMP9_A::B_0x4
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == SMP9_A::B_0x5
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == SMP9_A::B_0x6
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == SMP9_A::B_0x7
    }
}
#[doc = "Field `SMP9` writer - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
pub type SMP9_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SMP9_A, crate::Safe>;
impl<'a, REG> SMP9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMP9_A::B_0x0)
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMP9_A::B_0x1)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(SMP9_A::B_0x2)
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(SMP9_A::B_0x3)
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(SMP9_A::B_0x4)
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP9_A::B_0x5)
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(SMP9_A::B_0x6)
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(SMP9_A::B_0x7)
    }
}
#[doc = "Addition of one clock cycle to the sampling time. To make sure no conversion is ongoing, the software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPPLUS_A {
    #[doc = "1: 2.5 ADC clock cycle sampling time becomes 3.5 ADC clock cycles for the ADC_SMPR1 and ADC_SMPR2 registers."]
    B_0x1 = 1,
    #[doc = "0: The sampling time remains set to 2.5 ADC clock cycles remains"]
    B_0x0 = 0,
}
impl From<SMPPLUS_A> for bool {
    #[inline(always)]
    fn from(variant: SMPPLUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMPPLUS` reader - Addition of one clock cycle to the sampling time. To make sure no conversion is ongoing, the software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0."]
pub type SMPPLUS_R = crate::BitReader<SMPPLUS_A>;
impl SMPPLUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMPPLUS_A {
        match self.bits {
            true => SMPPLUS_A::B_0x1,
            false => SMPPLUS_A::B_0x0,
        }
    }
    #[doc = "2.5 ADC clock cycle sampling time becomes 3.5 ADC clock cycles for the ADC_SMPR1 and ADC_SMPR2 registers."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SMPPLUS_A::B_0x1
    }
    #[doc = "The sampling time remains set to 2.5 ADC clock cycles remains"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SMPPLUS_A::B_0x0
    }
}
#[doc = "Field `SMPPLUS` writer - Addition of one clock cycle to the sampling time. To make sure no conversion is ongoing, the software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0."]
pub type SMPPLUS_W<'a, REG> = crate::BitWriter<'a, REG, SMPPLUS_A>;
impl<'a, REG> SMPPLUS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "2.5 ADC clock cycle sampling time becomes 3.5 ADC clock cycles for the ADC_SMPR1 and ADC_SMPR2 registers."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMPPLUS_A::B_0x1)
    }
    #[doc = "The sampling time remains set to 2.5 ADC clock cycles remains"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMPPLUS_A::B_0x0)
    }
}
impl R {
    #[doc = "Bits 0:2 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
    #[inline(always)]
    pub fn SMP0(&self) -> SMP0_R {
        SMP0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
    #[inline(always)]
    pub fn SMP1(&self) -> SMP1_R {
        SMP1_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
    #[inline(always)]
    pub fn SMP2(&self) -> SMP2_R {
        SMP2_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
    #[inline(always)]
    pub fn SMP3(&self) -> SMP3_R {
        SMP3_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
    #[inline(always)]
    pub fn SMP4(&self) -> SMP4_R {
        SMP4_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
    #[inline(always)]
    pub fn SMP5(&self) -> SMP5_R {
        SMP5_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
    #[inline(always)]
    pub fn SMP6(&self) -> SMP6_R {
        SMP6_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
    #[inline(always)]
    pub fn SMP7(&self) -> SMP7_R {
        SMP7_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
    #[inline(always)]
    pub fn SMP8(&self) -> SMP8_R {
        SMP8_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
    #[inline(always)]
    pub fn SMP9(&self) -> SMP9_R {
        SMP9_R::new(((self.bits >> 27) & 7) as u8)
    }
    #[doc = "Bit 31 - Addition of one clock cycle to the sampling time. To make sure no conversion is ongoing, the software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0."]
    #[inline(always)]
    pub fn SMPPLUS(&self) -> SMPPLUS_R {
        SMPPLUS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
    #[inline(always)]
    pub fn SMP0(&mut self) -> SMP0_W<'_, SMPR1_SPEC> {
        SMP0_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
    #[inline(always)]
    pub fn SMP1(&mut self) -> SMP1_W<'_, SMPR1_SPEC> {
        SMP1_W::new(self, 3)
    }
    #[doc = "Bits 6:8 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
    #[inline(always)]
    pub fn SMP2(&mut self) -> SMP2_W<'_, SMPR1_SPEC> {
        SMP2_W::new(self, 6)
    }
    #[doc = "Bits 9:11 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
    #[inline(always)]
    pub fn SMP3(&mut self) -> SMP3_W<'_, SMPR1_SPEC> {
        SMP3_W::new(self, 9)
    }
    #[doc = "Bits 12:14 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
    #[inline(always)]
    pub fn SMP4(&mut self) -> SMP4_W<'_, SMPR1_SPEC> {
        SMP4_W::new(self, 12)
    }
    #[doc = "Bits 15:17 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
    #[inline(always)]
    pub fn SMP5(&mut self) -> SMP5_W<'_, SMPR1_SPEC> {
        SMP5_W::new(self, 15)
    }
    #[doc = "Bits 18:20 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
    #[inline(always)]
    pub fn SMP6(&mut self) -> SMP6_W<'_, SMPR1_SPEC> {
        SMP6_W::new(self, 18)
    }
    #[doc = "Bits 21:23 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
    #[inline(always)]
    pub fn SMP7(&mut self) -> SMP7_W<'_, SMPR1_SPEC> {
        SMP7_W::new(self, 21)
    }
    #[doc = "Bits 24:26 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
    #[inline(always)]
    pub fn SMP8(&mut self) -> SMP8_W<'_, SMPR1_SPEC> {
        SMP8_W::new(self, 24)
    }
    #[doc = "Bits 27:29 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sample cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
    #[inline(always)]
    pub fn SMP9(&mut self) -> SMP9_W<'_, SMPR1_SPEC> {
        SMP9_W::new(self, 27)
    }
    #[doc = "Bit 31 - Addition of one clock cycle to the sampling time. To make sure no conversion is ongoing, the software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0."]
    #[inline(always)]
    pub fn SMPPLUS(&mut self) -> SMPPLUS_W<'_, SMPR1_SPEC> {
        SMPPLUS_W::new(self, 31)
    }
}
#[doc = "ADC sample time register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`smpr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMPR1_SPEC;
impl crate::RegisterSpec for SMPR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smpr1::R`](R) reader structure"]
impl crate::Readable for SMPR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smpr1::W`](W) writer structure"]
impl crate::Writable for SMPR1_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SMPR1 to value 0"]
impl crate::Resettable for SMPR1_SPEC {}
