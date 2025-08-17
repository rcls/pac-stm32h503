#[doc = "Register `OSPEEDR` reader"]
pub type R = crate::R<OSPEEDR_SPEC>;
#[doc = "Register `OSPEEDR` writer"]
pub type W = crate::W<OSPEEDR_SPEC>;
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSPEED0_A {
    #[doc = "0: Low speed"]
    B_0x0 = 0,
    #[doc = "1: Medium speed"]
    B_0x1 = 1,
    #[doc = "2: High speed"]
    B_0x2 = 2,
    #[doc = "3: Very-high speed"]
    B_0x3 = 3,
}
impl From<OSPEED0_A> for u8 {
    #[inline(always)]
    fn from(variant: OSPEED0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OSPEED0_A {
    type Ux = u8;
}
impl crate::IsEnum for OSPEED0_A {}
#[doc = "Field `OSPEED0` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OSPEED0_R = crate::FieldReader<OSPEED0_A>;
impl OSPEED0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OSPEED0_A {
        match self.bits {
            0 => OSPEED0_A::B_0x0,
            1 => OSPEED0_A::B_0x1,
            2 => OSPEED0_A::B_0x2,
            3 => OSPEED0_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OSPEED0_A::B_0x0
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OSPEED0_A::B_0x1
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == OSPEED0_A::B_0x2
    }
    #[doc = "Very-high speed"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == OSPEED0_A::B_0x3
    }
}
#[doc = "Field `OSPEED0` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OSPEED0_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OSPEED0_A, crate::Safe>;
impl<'a, REG> OSPEED0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED0_A::B_0x0)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED0_A::B_0x1)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED0_A::B_0x2)
    }
    #[doc = "Very-high speed"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED0_A::B_0x3)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSPEED1_A {
    #[doc = "0: Low speed"]
    B_0x0 = 0,
    #[doc = "1: Medium speed"]
    B_0x1 = 1,
    #[doc = "2: High speed"]
    B_0x2 = 2,
    #[doc = "3: Very-high speed"]
    B_0x3 = 3,
}
impl From<OSPEED1_A> for u8 {
    #[inline(always)]
    fn from(variant: OSPEED1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OSPEED1_A {
    type Ux = u8;
}
impl crate::IsEnum for OSPEED1_A {}
#[doc = "Field `OSPEED1` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OSPEED1_R = crate::FieldReader<OSPEED1_A>;
impl OSPEED1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OSPEED1_A {
        match self.bits {
            0 => OSPEED1_A::B_0x0,
            1 => OSPEED1_A::B_0x1,
            2 => OSPEED1_A::B_0x2,
            3 => OSPEED1_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OSPEED1_A::B_0x0
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OSPEED1_A::B_0x1
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == OSPEED1_A::B_0x2
    }
    #[doc = "Very-high speed"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == OSPEED1_A::B_0x3
    }
}
#[doc = "Field `OSPEED1` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OSPEED1_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OSPEED1_A, crate::Safe>;
impl<'a, REG> OSPEED1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED1_A::B_0x0)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED1_A::B_0x1)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED1_A::B_0x2)
    }
    #[doc = "Very-high speed"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED1_A::B_0x3)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSPEED2_A {
    #[doc = "0: Low speed"]
    B_0x0 = 0,
    #[doc = "1: Medium speed"]
    B_0x1 = 1,
    #[doc = "2: High speed"]
    B_0x2 = 2,
    #[doc = "3: Very-high speed"]
    B_0x3 = 3,
}
impl From<OSPEED2_A> for u8 {
    #[inline(always)]
    fn from(variant: OSPEED2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OSPEED2_A {
    type Ux = u8;
}
impl crate::IsEnum for OSPEED2_A {}
#[doc = "Field `OSPEED2` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OSPEED2_R = crate::FieldReader<OSPEED2_A>;
impl OSPEED2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OSPEED2_A {
        match self.bits {
            0 => OSPEED2_A::B_0x0,
            1 => OSPEED2_A::B_0x1,
            2 => OSPEED2_A::B_0x2,
            3 => OSPEED2_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OSPEED2_A::B_0x0
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OSPEED2_A::B_0x1
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == OSPEED2_A::B_0x2
    }
    #[doc = "Very-high speed"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == OSPEED2_A::B_0x3
    }
}
#[doc = "Field `OSPEED2` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OSPEED2_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OSPEED2_A, crate::Safe>;
impl<'a, REG> OSPEED2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED2_A::B_0x0)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED2_A::B_0x1)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED2_A::B_0x2)
    }
    #[doc = "Very-high speed"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED2_A::B_0x3)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSPEED3_A {
    #[doc = "0: Low speed"]
    B_0x0 = 0,
    #[doc = "1: Medium speed"]
    B_0x1 = 1,
    #[doc = "2: High speed"]
    B_0x2 = 2,
    #[doc = "3: Very-high speed"]
    B_0x3 = 3,
}
impl From<OSPEED3_A> for u8 {
    #[inline(always)]
    fn from(variant: OSPEED3_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OSPEED3_A {
    type Ux = u8;
}
impl crate::IsEnum for OSPEED3_A {}
#[doc = "Field `OSPEED3` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OSPEED3_R = crate::FieldReader<OSPEED3_A>;
impl OSPEED3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OSPEED3_A {
        match self.bits {
            0 => OSPEED3_A::B_0x0,
            1 => OSPEED3_A::B_0x1,
            2 => OSPEED3_A::B_0x2,
            3 => OSPEED3_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OSPEED3_A::B_0x0
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OSPEED3_A::B_0x1
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == OSPEED3_A::B_0x2
    }
    #[doc = "Very-high speed"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == OSPEED3_A::B_0x3
    }
}
#[doc = "Field `OSPEED3` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OSPEED3_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OSPEED3_A, crate::Safe>;
impl<'a, REG> OSPEED3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED3_A::B_0x0)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED3_A::B_0x1)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED3_A::B_0x2)
    }
    #[doc = "Very-high speed"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED3_A::B_0x3)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSPEED4_A {
    #[doc = "0: Low speed"]
    B_0x0 = 0,
    #[doc = "1: Medium speed"]
    B_0x1 = 1,
    #[doc = "2: High speed"]
    B_0x2 = 2,
    #[doc = "3: Very-high speed"]
    B_0x3 = 3,
}
impl From<OSPEED4_A> for u8 {
    #[inline(always)]
    fn from(variant: OSPEED4_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OSPEED4_A {
    type Ux = u8;
}
impl crate::IsEnum for OSPEED4_A {}
#[doc = "Field `OSPEED4` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OSPEED4_R = crate::FieldReader<OSPEED4_A>;
impl OSPEED4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OSPEED4_A {
        match self.bits {
            0 => OSPEED4_A::B_0x0,
            1 => OSPEED4_A::B_0x1,
            2 => OSPEED4_A::B_0x2,
            3 => OSPEED4_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OSPEED4_A::B_0x0
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OSPEED4_A::B_0x1
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == OSPEED4_A::B_0x2
    }
    #[doc = "Very-high speed"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == OSPEED4_A::B_0x3
    }
}
#[doc = "Field `OSPEED4` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OSPEED4_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OSPEED4_A, crate::Safe>;
impl<'a, REG> OSPEED4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED4_A::B_0x0)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED4_A::B_0x1)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED4_A::B_0x2)
    }
    #[doc = "Very-high speed"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED4_A::B_0x3)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSPEED5_A {
    #[doc = "0: Low speed"]
    B_0x0 = 0,
    #[doc = "1: Medium speed"]
    B_0x1 = 1,
    #[doc = "2: High speed"]
    B_0x2 = 2,
    #[doc = "3: Very-high speed"]
    B_0x3 = 3,
}
impl From<OSPEED5_A> for u8 {
    #[inline(always)]
    fn from(variant: OSPEED5_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OSPEED5_A {
    type Ux = u8;
}
impl crate::IsEnum for OSPEED5_A {}
#[doc = "Field `OSPEED5` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OSPEED5_R = crate::FieldReader<OSPEED5_A>;
impl OSPEED5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OSPEED5_A {
        match self.bits {
            0 => OSPEED5_A::B_0x0,
            1 => OSPEED5_A::B_0x1,
            2 => OSPEED5_A::B_0x2,
            3 => OSPEED5_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OSPEED5_A::B_0x0
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OSPEED5_A::B_0x1
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == OSPEED5_A::B_0x2
    }
    #[doc = "Very-high speed"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == OSPEED5_A::B_0x3
    }
}
#[doc = "Field `OSPEED5` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OSPEED5_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OSPEED5_A, crate::Safe>;
impl<'a, REG> OSPEED5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED5_A::B_0x0)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED5_A::B_0x1)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED5_A::B_0x2)
    }
    #[doc = "Very-high speed"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED5_A::B_0x3)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSPEED6_A {
    #[doc = "0: Low speed"]
    B_0x0 = 0,
    #[doc = "1: Medium speed"]
    B_0x1 = 1,
    #[doc = "2: High speed"]
    B_0x2 = 2,
    #[doc = "3: Very-high speed"]
    B_0x3 = 3,
}
impl From<OSPEED6_A> for u8 {
    #[inline(always)]
    fn from(variant: OSPEED6_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OSPEED6_A {
    type Ux = u8;
}
impl crate::IsEnum for OSPEED6_A {}
#[doc = "Field `OSPEED6` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OSPEED6_R = crate::FieldReader<OSPEED6_A>;
impl OSPEED6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OSPEED6_A {
        match self.bits {
            0 => OSPEED6_A::B_0x0,
            1 => OSPEED6_A::B_0x1,
            2 => OSPEED6_A::B_0x2,
            3 => OSPEED6_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OSPEED6_A::B_0x0
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OSPEED6_A::B_0x1
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == OSPEED6_A::B_0x2
    }
    #[doc = "Very-high speed"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == OSPEED6_A::B_0x3
    }
}
#[doc = "Field `OSPEED6` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OSPEED6_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OSPEED6_A, crate::Safe>;
impl<'a, REG> OSPEED6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED6_A::B_0x0)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED6_A::B_0x1)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED6_A::B_0x2)
    }
    #[doc = "Very-high speed"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED6_A::B_0x3)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSPEED7_A {
    #[doc = "0: Low speed"]
    B_0x0 = 0,
    #[doc = "1: Medium speed"]
    B_0x1 = 1,
    #[doc = "2: High speed"]
    B_0x2 = 2,
    #[doc = "3: Very-high speed"]
    B_0x3 = 3,
}
impl From<OSPEED7_A> for u8 {
    #[inline(always)]
    fn from(variant: OSPEED7_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OSPEED7_A {
    type Ux = u8;
}
impl crate::IsEnum for OSPEED7_A {}
#[doc = "Field `OSPEED7` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OSPEED7_R = crate::FieldReader<OSPEED7_A>;
impl OSPEED7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OSPEED7_A {
        match self.bits {
            0 => OSPEED7_A::B_0x0,
            1 => OSPEED7_A::B_0x1,
            2 => OSPEED7_A::B_0x2,
            3 => OSPEED7_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OSPEED7_A::B_0x0
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OSPEED7_A::B_0x1
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == OSPEED7_A::B_0x2
    }
    #[doc = "Very-high speed"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == OSPEED7_A::B_0x3
    }
}
#[doc = "Field `OSPEED7` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OSPEED7_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OSPEED7_A, crate::Safe>;
impl<'a, REG> OSPEED7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED7_A::B_0x0)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED7_A::B_0x1)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED7_A::B_0x2)
    }
    #[doc = "Very-high speed"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED7_A::B_0x3)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSPEED8_A {
    #[doc = "0: Low speed"]
    B_0x0 = 0,
    #[doc = "1: Medium speed"]
    B_0x1 = 1,
    #[doc = "2: High speed"]
    B_0x2 = 2,
    #[doc = "3: Very-high speed"]
    B_0x3 = 3,
}
impl From<OSPEED8_A> for u8 {
    #[inline(always)]
    fn from(variant: OSPEED8_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OSPEED8_A {
    type Ux = u8;
}
impl crate::IsEnum for OSPEED8_A {}
#[doc = "Field `OSPEED8` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OSPEED8_R = crate::FieldReader<OSPEED8_A>;
impl OSPEED8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OSPEED8_A {
        match self.bits {
            0 => OSPEED8_A::B_0x0,
            1 => OSPEED8_A::B_0x1,
            2 => OSPEED8_A::B_0x2,
            3 => OSPEED8_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OSPEED8_A::B_0x0
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OSPEED8_A::B_0x1
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == OSPEED8_A::B_0x2
    }
    #[doc = "Very-high speed"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == OSPEED8_A::B_0x3
    }
}
#[doc = "Field `OSPEED8` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OSPEED8_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OSPEED8_A, crate::Safe>;
impl<'a, REG> OSPEED8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED8_A::B_0x0)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED8_A::B_0x1)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED8_A::B_0x2)
    }
    #[doc = "Very-high speed"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED8_A::B_0x3)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSPEED9_A {
    #[doc = "0: Low speed"]
    B_0x0 = 0,
    #[doc = "1: Medium speed"]
    B_0x1 = 1,
    #[doc = "2: High speed"]
    B_0x2 = 2,
    #[doc = "3: Very-high speed"]
    B_0x3 = 3,
}
impl From<OSPEED9_A> for u8 {
    #[inline(always)]
    fn from(variant: OSPEED9_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OSPEED9_A {
    type Ux = u8;
}
impl crate::IsEnum for OSPEED9_A {}
#[doc = "Field `OSPEED9` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OSPEED9_R = crate::FieldReader<OSPEED9_A>;
impl OSPEED9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OSPEED9_A {
        match self.bits {
            0 => OSPEED9_A::B_0x0,
            1 => OSPEED9_A::B_0x1,
            2 => OSPEED9_A::B_0x2,
            3 => OSPEED9_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OSPEED9_A::B_0x0
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OSPEED9_A::B_0x1
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == OSPEED9_A::B_0x2
    }
    #[doc = "Very-high speed"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == OSPEED9_A::B_0x3
    }
}
#[doc = "Field `OSPEED9` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OSPEED9_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OSPEED9_A, crate::Safe>;
impl<'a, REG> OSPEED9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED9_A::B_0x0)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED9_A::B_0x1)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED9_A::B_0x2)
    }
    #[doc = "Very-high speed"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED9_A::B_0x3)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSPEED10_A {
    #[doc = "0: Low speed"]
    B_0x0 = 0,
    #[doc = "1: Medium speed"]
    B_0x1 = 1,
    #[doc = "2: High speed"]
    B_0x2 = 2,
    #[doc = "3: Very-high speed"]
    B_0x3 = 3,
}
impl From<OSPEED10_A> for u8 {
    #[inline(always)]
    fn from(variant: OSPEED10_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OSPEED10_A {
    type Ux = u8;
}
impl crate::IsEnum for OSPEED10_A {}
#[doc = "Field `OSPEED10` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OSPEED10_R = crate::FieldReader<OSPEED10_A>;
impl OSPEED10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OSPEED10_A {
        match self.bits {
            0 => OSPEED10_A::B_0x0,
            1 => OSPEED10_A::B_0x1,
            2 => OSPEED10_A::B_0x2,
            3 => OSPEED10_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OSPEED10_A::B_0x0
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OSPEED10_A::B_0x1
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == OSPEED10_A::B_0x2
    }
    #[doc = "Very-high speed"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == OSPEED10_A::B_0x3
    }
}
#[doc = "Field `OSPEED10` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OSPEED10_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OSPEED10_A, crate::Safe>;
impl<'a, REG> OSPEED10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED10_A::B_0x0)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED10_A::B_0x1)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED10_A::B_0x2)
    }
    #[doc = "Very-high speed"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED10_A::B_0x3)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSPEED11_A {
    #[doc = "0: Low speed"]
    B_0x0 = 0,
    #[doc = "1: Medium speed"]
    B_0x1 = 1,
    #[doc = "2: High speed"]
    B_0x2 = 2,
    #[doc = "3: Very-high speed"]
    B_0x3 = 3,
}
impl From<OSPEED11_A> for u8 {
    #[inline(always)]
    fn from(variant: OSPEED11_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OSPEED11_A {
    type Ux = u8;
}
impl crate::IsEnum for OSPEED11_A {}
#[doc = "Field `OSPEED11` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OSPEED11_R = crate::FieldReader<OSPEED11_A>;
impl OSPEED11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OSPEED11_A {
        match self.bits {
            0 => OSPEED11_A::B_0x0,
            1 => OSPEED11_A::B_0x1,
            2 => OSPEED11_A::B_0x2,
            3 => OSPEED11_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OSPEED11_A::B_0x0
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OSPEED11_A::B_0x1
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == OSPEED11_A::B_0x2
    }
    #[doc = "Very-high speed"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == OSPEED11_A::B_0x3
    }
}
#[doc = "Field `OSPEED11` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OSPEED11_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OSPEED11_A, crate::Safe>;
impl<'a, REG> OSPEED11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED11_A::B_0x0)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED11_A::B_0x1)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED11_A::B_0x2)
    }
    #[doc = "Very-high speed"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED11_A::B_0x3)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSPEED12_A {
    #[doc = "0: Low speed"]
    B_0x0 = 0,
    #[doc = "1: Medium speed"]
    B_0x1 = 1,
    #[doc = "2: High speed"]
    B_0x2 = 2,
    #[doc = "3: Very-high speed"]
    B_0x3 = 3,
}
impl From<OSPEED12_A> for u8 {
    #[inline(always)]
    fn from(variant: OSPEED12_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OSPEED12_A {
    type Ux = u8;
}
impl crate::IsEnum for OSPEED12_A {}
#[doc = "Field `OSPEED12` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OSPEED12_R = crate::FieldReader<OSPEED12_A>;
impl OSPEED12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OSPEED12_A {
        match self.bits {
            0 => OSPEED12_A::B_0x0,
            1 => OSPEED12_A::B_0x1,
            2 => OSPEED12_A::B_0x2,
            3 => OSPEED12_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OSPEED12_A::B_0x0
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OSPEED12_A::B_0x1
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == OSPEED12_A::B_0x2
    }
    #[doc = "Very-high speed"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == OSPEED12_A::B_0x3
    }
}
#[doc = "Field `OSPEED12` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OSPEED12_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OSPEED12_A, crate::Safe>;
impl<'a, REG> OSPEED12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED12_A::B_0x0)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED12_A::B_0x1)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED12_A::B_0x2)
    }
    #[doc = "Very-high speed"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED12_A::B_0x3)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSPEED13_A {
    #[doc = "0: Low speed"]
    B_0x0 = 0,
    #[doc = "1: Medium speed"]
    B_0x1 = 1,
    #[doc = "2: High speed"]
    B_0x2 = 2,
    #[doc = "3: Very-high speed"]
    B_0x3 = 3,
}
impl From<OSPEED13_A> for u8 {
    #[inline(always)]
    fn from(variant: OSPEED13_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OSPEED13_A {
    type Ux = u8;
}
impl crate::IsEnum for OSPEED13_A {}
#[doc = "Field `OSPEED13` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OSPEED13_R = crate::FieldReader<OSPEED13_A>;
impl OSPEED13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OSPEED13_A {
        match self.bits {
            0 => OSPEED13_A::B_0x0,
            1 => OSPEED13_A::B_0x1,
            2 => OSPEED13_A::B_0x2,
            3 => OSPEED13_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OSPEED13_A::B_0x0
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OSPEED13_A::B_0x1
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == OSPEED13_A::B_0x2
    }
    #[doc = "Very-high speed"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == OSPEED13_A::B_0x3
    }
}
#[doc = "Field `OSPEED13` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OSPEED13_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OSPEED13_A, crate::Safe>;
impl<'a, REG> OSPEED13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED13_A::B_0x0)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED13_A::B_0x1)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED13_A::B_0x2)
    }
    #[doc = "Very-high speed"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED13_A::B_0x3)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSPEED14_A {
    #[doc = "0: Low speed"]
    B_0x0 = 0,
    #[doc = "1: Medium speed"]
    B_0x1 = 1,
    #[doc = "2: High speed"]
    B_0x2 = 2,
    #[doc = "3: Very-high speed"]
    B_0x3 = 3,
}
impl From<OSPEED14_A> for u8 {
    #[inline(always)]
    fn from(variant: OSPEED14_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OSPEED14_A {
    type Ux = u8;
}
impl crate::IsEnum for OSPEED14_A {}
#[doc = "Field `OSPEED14` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OSPEED14_R = crate::FieldReader<OSPEED14_A>;
impl OSPEED14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OSPEED14_A {
        match self.bits {
            0 => OSPEED14_A::B_0x0,
            1 => OSPEED14_A::B_0x1,
            2 => OSPEED14_A::B_0x2,
            3 => OSPEED14_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OSPEED14_A::B_0x0
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OSPEED14_A::B_0x1
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == OSPEED14_A::B_0x2
    }
    #[doc = "Very-high speed"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == OSPEED14_A::B_0x3
    }
}
#[doc = "Field `OSPEED14` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OSPEED14_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OSPEED14_A, crate::Safe>;
impl<'a, REG> OSPEED14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED14_A::B_0x0)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED14_A::B_0x1)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED14_A::B_0x2)
    }
    #[doc = "Very-high speed"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED14_A::B_0x3)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSPEED15_A {
    #[doc = "0: Low speed"]
    B_0x0 = 0,
    #[doc = "1: Medium speed"]
    B_0x1 = 1,
    #[doc = "2: High speed"]
    B_0x2 = 2,
    #[doc = "3: Very-high speed"]
    B_0x3 = 3,
}
impl From<OSPEED15_A> for u8 {
    #[inline(always)]
    fn from(variant: OSPEED15_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OSPEED15_A {
    type Ux = u8;
}
impl crate::IsEnum for OSPEED15_A {}
#[doc = "Field `OSPEED15` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OSPEED15_R = crate::FieldReader<OSPEED15_A>;
impl OSPEED15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OSPEED15_A {
        match self.bits {
            0 => OSPEED15_A::B_0x0,
            1 => OSPEED15_A::B_0x1,
            2 => OSPEED15_A::B_0x2,
            3 => OSPEED15_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OSPEED15_A::B_0x0
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OSPEED15_A::B_0x1
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == OSPEED15_A::B_0x2
    }
    #[doc = "Very-high speed"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == OSPEED15_A::B_0x3
    }
}
#[doc = "Field `OSPEED15` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OSPEED15_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OSPEED15_A, crate::Safe>;
impl<'a, REG> OSPEED15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED15_A::B_0x0)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED15_A::B_0x1)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED15_A::B_0x2)
    }
    #[doc = "Very-high speed"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED15_A::B_0x3)
    }
}
impl R {
    #[doc = "Bits 0:1 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OSPEED0(&self) -> OSPEED0_R {
        OSPEED0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OSPEED1(&self) -> OSPEED1_R {
        OSPEED1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OSPEED2(&self) -> OSPEED2_R {
        OSPEED2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OSPEED3(&self) -> OSPEED3_R {
        OSPEED3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OSPEED4(&self) -> OSPEED4_R {
        OSPEED4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OSPEED5(&self) -> OSPEED5_R {
        OSPEED5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OSPEED6(&self) -> OSPEED6_R {
        OSPEED6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OSPEED7(&self) -> OSPEED7_R {
        OSPEED7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OSPEED8(&self) -> OSPEED8_R {
        OSPEED8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OSPEED9(&self) -> OSPEED9_R {
        OSPEED9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OSPEED10(&self) -> OSPEED10_R {
        OSPEED10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OSPEED11(&self) -> OSPEED11_R {
        OSPEED11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OSPEED12(&self) -> OSPEED12_R {
        OSPEED12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OSPEED13(&self) -> OSPEED13_R {
        OSPEED13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OSPEED14(&self) -> OSPEED14_R {
        OSPEED14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OSPEED15(&self) -> OSPEED15_R {
        OSPEED15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OSPEED0(&mut self) -> OSPEED0_W<'_, OSPEEDR_SPEC> {
        OSPEED0_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OSPEED1(&mut self) -> OSPEED1_W<'_, OSPEEDR_SPEC> {
        OSPEED1_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OSPEED2(&mut self) -> OSPEED2_W<'_, OSPEEDR_SPEC> {
        OSPEED2_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OSPEED3(&mut self) -> OSPEED3_W<'_, OSPEEDR_SPEC> {
        OSPEED3_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OSPEED4(&mut self) -> OSPEED4_W<'_, OSPEEDR_SPEC> {
        OSPEED4_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OSPEED5(&mut self) -> OSPEED5_W<'_, OSPEEDR_SPEC> {
        OSPEED5_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OSPEED6(&mut self) -> OSPEED6_W<'_, OSPEEDR_SPEC> {
        OSPEED6_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OSPEED7(&mut self) -> OSPEED7_W<'_, OSPEEDR_SPEC> {
        OSPEED7_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OSPEED8(&mut self) -> OSPEED8_W<'_, OSPEEDR_SPEC> {
        OSPEED8_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OSPEED9(&mut self) -> OSPEED9_W<'_, OSPEEDR_SPEC> {
        OSPEED9_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OSPEED10(&mut self) -> OSPEED10_W<'_, OSPEEDR_SPEC> {
        OSPEED10_W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OSPEED11(&mut self) -> OSPEED11_W<'_, OSPEEDR_SPEC> {
        OSPEED11_W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OSPEED12(&mut self) -> OSPEED12_W<'_, OSPEEDR_SPEC> {
        OSPEED12_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OSPEED13(&mut self) -> OSPEED13_W<'_, OSPEEDR_SPEC> {
        OSPEED13_W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OSPEED14(&mut self) -> OSPEED14_W<'_, OSPEEDR_SPEC> {
        OSPEED14_W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed. The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OSPEED15(&mut self) -> OSPEED15_W<'_, OSPEEDR_SPEC> {
        OSPEED15_W::new(self, 30)
    }
}
#[doc = "GPIO port output speed register\n\nYou can [`read`](crate::Reg::read) this register and get [`ospeedr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ospeedr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSPEEDR_SPEC;
impl crate::RegisterSpec for OSPEEDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ospeedr::R`](R) reader structure"]
impl crate::Readable for OSPEEDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ospeedr::W`](W) writer structure"]
impl crate::Writable for OSPEEDR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets OSPEEDR to value 0x0c00_0000"]
impl crate::Resettable for OSPEEDR_SPEC {
    const RESET_VALUE: u32 = 0x0c00_0000;
}
