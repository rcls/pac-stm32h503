#[doc = "Register `SMPR2` reader"]
pub type R = crate::R<SMPR2_SPEC>;
#[doc = "Register `SMPR2` writer"]
pub type W = crate::W<SMPR2_SPEC>;
#[doc = "Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMP10_A {
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
impl From<SMP10_A> for u8 {
    #[inline(always)]
    fn from(variant: SMP10_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SMP10_A {
    type Ux = u8;
}
impl crate::IsEnum for SMP10_A {}
#[doc = "Field `SMP10` reader - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
pub type SMP10_R = crate::FieldReader<SMP10_A>;
impl SMP10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMP10_A {
        match self.bits {
            0 => SMP10_A::B_0x0,
            1 => SMP10_A::B_0x1,
            2 => SMP10_A::B_0x2,
            3 => SMP10_A::B_0x3,
            4 => SMP10_A::B_0x4,
            5 => SMP10_A::B_0x5,
            6 => SMP10_A::B_0x6,
            7 => SMP10_A::B_0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SMP10_A::B_0x0
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SMP10_A::B_0x1
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == SMP10_A::B_0x2
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == SMP10_A::B_0x3
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == SMP10_A::B_0x4
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == SMP10_A::B_0x5
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == SMP10_A::B_0x6
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == SMP10_A::B_0x7
    }
}
#[doc = "Field `SMP10` writer - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
pub type SMP10_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SMP10_A, crate::Safe>;
impl<'a, REG> SMP10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMP10_A::B_0x0)
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMP10_A::B_0x1)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(SMP10_A::B_0x2)
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(SMP10_A::B_0x3)
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(SMP10_A::B_0x4)
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP10_A::B_0x5)
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(SMP10_A::B_0x6)
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(SMP10_A::B_0x7)
    }
}
#[doc = "Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMP11_A {
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
impl From<SMP11_A> for u8 {
    #[inline(always)]
    fn from(variant: SMP11_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SMP11_A {
    type Ux = u8;
}
impl crate::IsEnum for SMP11_A {}
#[doc = "Field `SMP11` reader - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
pub type SMP11_R = crate::FieldReader<SMP11_A>;
impl SMP11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMP11_A {
        match self.bits {
            0 => SMP11_A::B_0x0,
            1 => SMP11_A::B_0x1,
            2 => SMP11_A::B_0x2,
            3 => SMP11_A::B_0x3,
            4 => SMP11_A::B_0x4,
            5 => SMP11_A::B_0x5,
            6 => SMP11_A::B_0x6,
            7 => SMP11_A::B_0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SMP11_A::B_0x0
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SMP11_A::B_0x1
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == SMP11_A::B_0x2
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == SMP11_A::B_0x3
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == SMP11_A::B_0x4
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == SMP11_A::B_0x5
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == SMP11_A::B_0x6
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == SMP11_A::B_0x7
    }
}
#[doc = "Field `SMP11` writer - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
pub type SMP11_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SMP11_A, crate::Safe>;
impl<'a, REG> SMP11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMP11_A::B_0x0)
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMP11_A::B_0x1)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(SMP11_A::B_0x2)
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(SMP11_A::B_0x3)
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(SMP11_A::B_0x4)
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP11_A::B_0x5)
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(SMP11_A::B_0x6)
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(SMP11_A::B_0x7)
    }
}
#[doc = "Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMP12_A {
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
impl From<SMP12_A> for u8 {
    #[inline(always)]
    fn from(variant: SMP12_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SMP12_A {
    type Ux = u8;
}
impl crate::IsEnum for SMP12_A {}
#[doc = "Field `SMP12` reader - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
pub type SMP12_R = crate::FieldReader<SMP12_A>;
impl SMP12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMP12_A {
        match self.bits {
            0 => SMP12_A::B_0x0,
            1 => SMP12_A::B_0x1,
            2 => SMP12_A::B_0x2,
            3 => SMP12_A::B_0x3,
            4 => SMP12_A::B_0x4,
            5 => SMP12_A::B_0x5,
            6 => SMP12_A::B_0x6,
            7 => SMP12_A::B_0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SMP12_A::B_0x0
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SMP12_A::B_0x1
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == SMP12_A::B_0x2
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == SMP12_A::B_0x3
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == SMP12_A::B_0x4
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == SMP12_A::B_0x5
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == SMP12_A::B_0x6
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == SMP12_A::B_0x7
    }
}
#[doc = "Field `SMP12` writer - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
pub type SMP12_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SMP12_A, crate::Safe>;
impl<'a, REG> SMP12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMP12_A::B_0x0)
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMP12_A::B_0x1)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(SMP12_A::B_0x2)
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(SMP12_A::B_0x3)
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(SMP12_A::B_0x4)
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP12_A::B_0x5)
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(SMP12_A::B_0x6)
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(SMP12_A::B_0x7)
    }
}
#[doc = "Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMP13_A {
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
impl From<SMP13_A> for u8 {
    #[inline(always)]
    fn from(variant: SMP13_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SMP13_A {
    type Ux = u8;
}
impl crate::IsEnum for SMP13_A {}
#[doc = "Field `SMP13` reader - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
pub type SMP13_R = crate::FieldReader<SMP13_A>;
impl SMP13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMP13_A {
        match self.bits {
            0 => SMP13_A::B_0x0,
            1 => SMP13_A::B_0x1,
            2 => SMP13_A::B_0x2,
            3 => SMP13_A::B_0x3,
            4 => SMP13_A::B_0x4,
            5 => SMP13_A::B_0x5,
            6 => SMP13_A::B_0x6,
            7 => SMP13_A::B_0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SMP13_A::B_0x0
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SMP13_A::B_0x1
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == SMP13_A::B_0x2
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == SMP13_A::B_0x3
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == SMP13_A::B_0x4
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == SMP13_A::B_0x5
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == SMP13_A::B_0x6
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == SMP13_A::B_0x7
    }
}
#[doc = "Field `SMP13` writer - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
pub type SMP13_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SMP13_A, crate::Safe>;
impl<'a, REG> SMP13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMP13_A::B_0x0)
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMP13_A::B_0x1)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(SMP13_A::B_0x2)
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(SMP13_A::B_0x3)
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(SMP13_A::B_0x4)
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP13_A::B_0x5)
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(SMP13_A::B_0x6)
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(SMP13_A::B_0x7)
    }
}
#[doc = "Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMP14_A {
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
impl From<SMP14_A> for u8 {
    #[inline(always)]
    fn from(variant: SMP14_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SMP14_A {
    type Ux = u8;
}
impl crate::IsEnum for SMP14_A {}
#[doc = "Field `SMP14` reader - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
pub type SMP14_R = crate::FieldReader<SMP14_A>;
impl SMP14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMP14_A {
        match self.bits {
            0 => SMP14_A::B_0x0,
            1 => SMP14_A::B_0x1,
            2 => SMP14_A::B_0x2,
            3 => SMP14_A::B_0x3,
            4 => SMP14_A::B_0x4,
            5 => SMP14_A::B_0x5,
            6 => SMP14_A::B_0x6,
            7 => SMP14_A::B_0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SMP14_A::B_0x0
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SMP14_A::B_0x1
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == SMP14_A::B_0x2
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == SMP14_A::B_0x3
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == SMP14_A::B_0x4
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == SMP14_A::B_0x5
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == SMP14_A::B_0x6
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == SMP14_A::B_0x7
    }
}
#[doc = "Field `SMP14` writer - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
pub type SMP14_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SMP14_A, crate::Safe>;
impl<'a, REG> SMP14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMP14_A::B_0x0)
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMP14_A::B_0x1)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(SMP14_A::B_0x2)
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(SMP14_A::B_0x3)
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(SMP14_A::B_0x4)
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP14_A::B_0x5)
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(SMP14_A::B_0x6)
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(SMP14_A::B_0x7)
    }
}
#[doc = "Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMP15_A {
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
impl From<SMP15_A> for u8 {
    #[inline(always)]
    fn from(variant: SMP15_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SMP15_A {
    type Ux = u8;
}
impl crate::IsEnum for SMP15_A {}
#[doc = "Field `SMP15` reader - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
pub type SMP15_R = crate::FieldReader<SMP15_A>;
impl SMP15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMP15_A {
        match self.bits {
            0 => SMP15_A::B_0x0,
            1 => SMP15_A::B_0x1,
            2 => SMP15_A::B_0x2,
            3 => SMP15_A::B_0x3,
            4 => SMP15_A::B_0x4,
            5 => SMP15_A::B_0x5,
            6 => SMP15_A::B_0x6,
            7 => SMP15_A::B_0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SMP15_A::B_0x0
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SMP15_A::B_0x1
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == SMP15_A::B_0x2
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == SMP15_A::B_0x3
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == SMP15_A::B_0x4
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == SMP15_A::B_0x5
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == SMP15_A::B_0x6
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == SMP15_A::B_0x7
    }
}
#[doc = "Field `SMP15` writer - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
pub type SMP15_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SMP15_A, crate::Safe>;
impl<'a, REG> SMP15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMP15_A::B_0x0)
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMP15_A::B_0x1)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(SMP15_A::B_0x2)
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(SMP15_A::B_0x3)
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(SMP15_A::B_0x4)
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP15_A::B_0x5)
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(SMP15_A::B_0x6)
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(SMP15_A::B_0x7)
    }
}
#[doc = "Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMP16_A {
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
impl From<SMP16_A> for u8 {
    #[inline(always)]
    fn from(variant: SMP16_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SMP16_A {
    type Ux = u8;
}
impl crate::IsEnum for SMP16_A {}
#[doc = "Field `SMP16` reader - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
pub type SMP16_R = crate::FieldReader<SMP16_A>;
impl SMP16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMP16_A {
        match self.bits {
            0 => SMP16_A::B_0x0,
            1 => SMP16_A::B_0x1,
            2 => SMP16_A::B_0x2,
            3 => SMP16_A::B_0x3,
            4 => SMP16_A::B_0x4,
            5 => SMP16_A::B_0x5,
            6 => SMP16_A::B_0x6,
            7 => SMP16_A::B_0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SMP16_A::B_0x0
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SMP16_A::B_0x1
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == SMP16_A::B_0x2
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == SMP16_A::B_0x3
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == SMP16_A::B_0x4
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == SMP16_A::B_0x5
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == SMP16_A::B_0x6
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == SMP16_A::B_0x7
    }
}
#[doc = "Field `SMP16` writer - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
pub type SMP16_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SMP16_A, crate::Safe>;
impl<'a, REG> SMP16_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMP16_A::B_0x0)
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMP16_A::B_0x1)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(SMP16_A::B_0x2)
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(SMP16_A::B_0x3)
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(SMP16_A::B_0x4)
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP16_A::B_0x5)
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(SMP16_A::B_0x6)
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(SMP16_A::B_0x7)
    }
}
#[doc = "Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMP17_A {
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
impl From<SMP17_A> for u8 {
    #[inline(always)]
    fn from(variant: SMP17_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SMP17_A {
    type Ux = u8;
}
impl crate::IsEnum for SMP17_A {}
#[doc = "Field `SMP17` reader - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
pub type SMP17_R = crate::FieldReader<SMP17_A>;
impl SMP17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMP17_A {
        match self.bits {
            0 => SMP17_A::B_0x0,
            1 => SMP17_A::B_0x1,
            2 => SMP17_A::B_0x2,
            3 => SMP17_A::B_0x3,
            4 => SMP17_A::B_0x4,
            5 => SMP17_A::B_0x5,
            6 => SMP17_A::B_0x6,
            7 => SMP17_A::B_0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SMP17_A::B_0x0
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SMP17_A::B_0x1
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == SMP17_A::B_0x2
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == SMP17_A::B_0x3
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == SMP17_A::B_0x4
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == SMP17_A::B_0x5
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == SMP17_A::B_0x6
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == SMP17_A::B_0x7
    }
}
#[doc = "Field `SMP17` writer - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
pub type SMP17_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SMP17_A, crate::Safe>;
impl<'a, REG> SMP17_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMP17_A::B_0x0)
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMP17_A::B_0x1)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(SMP17_A::B_0x2)
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(SMP17_A::B_0x3)
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(SMP17_A::B_0x4)
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP17_A::B_0x5)
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(SMP17_A::B_0x6)
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(SMP17_A::B_0x7)
    }
}
#[doc = "Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMP18_A {
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
impl From<SMP18_A> for u8 {
    #[inline(always)]
    fn from(variant: SMP18_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SMP18_A {
    type Ux = u8;
}
impl crate::IsEnum for SMP18_A {}
#[doc = "Field `SMP18` reader - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
pub type SMP18_R = crate::FieldReader<SMP18_A>;
impl SMP18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMP18_A {
        match self.bits {
            0 => SMP18_A::B_0x0,
            1 => SMP18_A::B_0x1,
            2 => SMP18_A::B_0x2,
            3 => SMP18_A::B_0x3,
            4 => SMP18_A::B_0x4,
            5 => SMP18_A::B_0x5,
            6 => SMP18_A::B_0x6,
            7 => SMP18_A::B_0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SMP18_A::B_0x0
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SMP18_A::B_0x1
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == SMP18_A::B_0x2
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == SMP18_A::B_0x3
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == SMP18_A::B_0x4
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == SMP18_A::B_0x5
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == SMP18_A::B_0x6
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == SMP18_A::B_0x7
    }
}
#[doc = "Field `SMP18` writer - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
pub type SMP18_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SMP18_A, crate::Safe>;
impl<'a, REG> SMP18_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMP18_A::B_0x0)
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMP18_A::B_0x1)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(SMP18_A::B_0x2)
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(SMP18_A::B_0x3)
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(SMP18_A::B_0x4)
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP18_A::B_0x5)
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(SMP18_A::B_0x6)
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(SMP18_A::B_0x7)
    }
}
#[doc = "Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMP19_A {
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
impl From<SMP19_A> for u8 {
    #[inline(always)]
    fn from(variant: SMP19_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SMP19_A {
    type Ux = u8;
}
impl crate::IsEnum for SMP19_A {}
#[doc = "Field `SMP19` reader - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
pub type SMP19_R = crate::FieldReader<SMP19_A>;
impl SMP19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMP19_A {
        match self.bits {
            0 => SMP19_A::B_0x0,
            1 => SMP19_A::B_0x1,
            2 => SMP19_A::B_0x2,
            3 => SMP19_A::B_0x3,
            4 => SMP19_A::B_0x4,
            5 => SMP19_A::B_0x5,
            6 => SMP19_A::B_0x6,
            7 => SMP19_A::B_0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SMP19_A::B_0x0
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SMP19_A::B_0x1
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == SMP19_A::B_0x2
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == SMP19_A::B_0x3
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == SMP19_A::B_0x4
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == SMP19_A::B_0x5
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == SMP19_A::B_0x6
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == SMP19_A::B_0x7
    }
}
#[doc = "Field `SMP19` writer - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
pub type SMP19_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SMP19_A, crate::Safe>;
impl<'a, REG> SMP19_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMP19_A::B_0x0)
    }
    #[doc = "6.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMP19_A::B_0x1)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(SMP19_A::B_0x2)
    }
    #[doc = "24.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(SMP19_A::B_0x3)
    }
    #[doc = "47.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(SMP19_A::B_0x4)
    }
    #[doc = "92.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(SMP19_A::B_0x5)
    }
    #[doc = "247.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(SMP19_A::B_0x6)
    }
    #[doc = "640.5 ADC clock cycles"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(SMP19_A::B_0x7)
    }
}
impl R {
    #[doc = "Bits 0:2 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
    #[inline(always)]
    pub fn SMP10(&self) -> SMP10_R {
        SMP10_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
    #[inline(always)]
    pub fn SMP11(&self) -> SMP11_R {
        SMP11_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
    #[inline(always)]
    pub fn SMP12(&self) -> SMP12_R {
        SMP12_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
    #[inline(always)]
    pub fn SMP13(&self) -> SMP13_R {
        SMP13_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
    #[inline(always)]
    pub fn SMP14(&self) -> SMP14_R {
        SMP14_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
    #[inline(always)]
    pub fn SMP15(&self) -> SMP15_R {
        SMP15_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
    #[inline(always)]
    pub fn SMP16(&self) -> SMP16_R {
        SMP16_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
    #[inline(always)]
    pub fn SMP17(&self) -> SMP17_R {
        SMP17_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
    #[inline(always)]
    pub fn SMP18(&self) -> SMP18_R {
        SMP18_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
    #[inline(always)]
    pub fn SMP19(&self) -> SMP19_R {
        SMP19_R::new(((self.bits >> 27) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
    #[inline(always)]
    pub fn SMP10(&mut self) -> SMP10_W<'_, SMPR2_SPEC> {
        SMP10_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
    #[inline(always)]
    pub fn SMP11(&mut self) -> SMP11_W<'_, SMPR2_SPEC> {
        SMP11_W::new(self, 3)
    }
    #[doc = "Bits 6:8 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
    #[inline(always)]
    pub fn SMP12(&mut self) -> SMP12_W<'_, SMPR2_SPEC> {
        SMP12_W::new(self, 6)
    }
    #[doc = "Bits 9:11 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
    #[inline(always)]
    pub fn SMP13(&mut self) -> SMP13_W<'_, SMPR2_SPEC> {
        SMP13_W::new(self, 9)
    }
    #[doc = "Bits 12:14 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
    #[inline(always)]
    pub fn SMP14(&mut self) -> SMP14_W<'_, SMPR2_SPEC> {
        SMP14_W::new(self, 12)
    }
    #[doc = "Bits 15:17 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
    #[inline(always)]
    pub fn SMP15(&mut self) -> SMP15_W<'_, SMPR2_SPEC> {
        SMP15_W::new(self, 15)
    }
    #[doc = "Bits 18:20 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
    #[inline(always)]
    pub fn SMP16(&mut self) -> SMP16_W<'_, SMPR2_SPEC> {
        SMP16_W::new(self, 18)
    }
    #[doc = "Bits 21:23 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
    #[inline(always)]
    pub fn SMP17(&mut self) -> SMP17_W<'_, SMPR2_SPEC> {
        SMP17_W::new(self, 21)
    }
    #[doc = "Bits 24:26 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
    #[inline(always)]
    pub fn SMP18(&mut self) -> SMP18_W<'_, SMPR2_SPEC> {
        SMP18_W::new(self, 24)
    }
    #[doc = "Bits 27:29 - Channel x sampling time selection These bits are written by software to select the sampling time individually for each channel. During sampling cycles, the channel selection bits must remain unchanged. Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing). Some channels are not connected physically. Keep the corresponding SMPx\\[2:0\\] setting to the reset value."]
    #[inline(always)]
    pub fn SMP19(&mut self) -> SMP19_W<'_, SMPR2_SPEC> {
        SMP19_W::new(self, 27)
    }
}
#[doc = "ADC sample time register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`smpr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMPR2_SPEC;
impl crate::RegisterSpec for SMPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smpr2::R`](R) reader structure"]
impl crate::Readable for SMPR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smpr2::W`](W) writer structure"]
impl crate::Writable for SMPR2_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SMPR2 to value 0"]
impl crate::Resettable for SMPR2_SPEC {}
