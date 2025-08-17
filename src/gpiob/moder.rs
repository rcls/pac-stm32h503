#[doc = "Register `MODER` reader"]
pub type R = crate::R<MODER_SPEC>;
#[doc = "Register `MODER` writer"]
pub type W = crate::W<MODER_SPEC>;
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE0_A {
    #[doc = "0: Input mode"]
    B_0x0 = 0,
    #[doc = "1: General purpose output mode"]
    B_0x1 = 1,
    #[doc = "2: Alternate function mode"]
    B_0x2 = 2,
    #[doc = "3: Analog mode (reset state)"]
    B_0x3 = 3,
}
impl From<MODE0_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE0_A {
    type Ux = u8;
}
impl crate::IsEnum for MODE0_A {}
#[doc = "Field `MODE0` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type MODE0_R = crate::FieldReader<MODE0_A>;
impl MODE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE0_A {
        match self.bits {
            0 => MODE0_A::B_0x0,
            1 => MODE0_A::B_0x1,
            2 => MODE0_A::B_0x2,
            3 => MODE0_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MODE0_A::B_0x0
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MODE0_A::B_0x1
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == MODE0_A::B_0x2
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == MODE0_A::B_0x3
    }
}
#[doc = "Field `MODE0` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type MODE0_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE0_A, crate::Safe>;
impl<'a, REG> MODE0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0_A::B_0x0)
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0_A::B_0x1)
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0_A::B_0x2)
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MODE0_A::B_0x3)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE1_A {
    #[doc = "0: Input mode"]
    B_0x0 = 0,
    #[doc = "1: General purpose output mode"]
    B_0x1 = 1,
    #[doc = "2: Alternate function mode"]
    B_0x2 = 2,
    #[doc = "3: Analog mode (reset state)"]
    B_0x3 = 3,
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
#[doc = "Field `MODE1` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
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
            _ => unreachable!(),
        }
    }
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MODE1_A::B_0x0
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MODE1_A::B_0x1
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == MODE1_A::B_0x2
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == MODE1_A::B_0x3
    }
}
#[doc = "Field `MODE1` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type MODE1_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE1_A, crate::Safe>;
impl<'a, REG> MODE1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1_A::B_0x0)
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1_A::B_0x1)
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1_A::B_0x2)
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1_A::B_0x3)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE2_A {
    #[doc = "0: Input mode"]
    B_0x0 = 0,
    #[doc = "1: General purpose output mode"]
    B_0x1 = 1,
    #[doc = "2: Alternate function mode"]
    B_0x2 = 2,
    #[doc = "3: Analog mode (reset state)"]
    B_0x3 = 3,
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
#[doc = "Field `MODE2` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type MODE2_R = crate::FieldReader<MODE2_A>;
impl MODE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE2_A {
        match self.bits {
            0 => MODE2_A::B_0x0,
            1 => MODE2_A::B_0x1,
            2 => MODE2_A::B_0x2,
            3 => MODE2_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MODE2_A::B_0x0
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MODE2_A::B_0x1
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == MODE2_A::B_0x2
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == MODE2_A::B_0x3
    }
}
#[doc = "Field `MODE2` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type MODE2_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE2_A, crate::Safe>;
impl<'a, REG> MODE2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MODE2_A::B_0x0)
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MODE2_A::B_0x1)
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MODE2_A::B_0x2)
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MODE2_A::B_0x3)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE3_A {
    #[doc = "0: Input mode"]
    B_0x0 = 0,
    #[doc = "1: General purpose output mode"]
    B_0x1 = 1,
    #[doc = "2: Alternate function mode"]
    B_0x2 = 2,
    #[doc = "3: Analog mode (reset state)"]
    B_0x3 = 3,
}
impl From<MODE3_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE3_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE3_A {
    type Ux = u8;
}
impl crate::IsEnum for MODE3_A {}
#[doc = "Field `MODE3` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type MODE3_R = crate::FieldReader<MODE3_A>;
impl MODE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE3_A {
        match self.bits {
            0 => MODE3_A::B_0x0,
            1 => MODE3_A::B_0x1,
            2 => MODE3_A::B_0x2,
            3 => MODE3_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MODE3_A::B_0x0
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MODE3_A::B_0x1
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == MODE3_A::B_0x2
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == MODE3_A::B_0x3
    }
}
#[doc = "Field `MODE3` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type MODE3_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE3_A, crate::Safe>;
impl<'a, REG> MODE3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MODE3_A::B_0x0)
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MODE3_A::B_0x1)
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MODE3_A::B_0x2)
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MODE3_A::B_0x3)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE4_A {
    #[doc = "0: Input mode"]
    B_0x0 = 0,
    #[doc = "1: General purpose output mode"]
    B_0x1 = 1,
    #[doc = "2: Alternate function mode"]
    B_0x2 = 2,
    #[doc = "3: Analog mode (reset state)"]
    B_0x3 = 3,
}
impl From<MODE4_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE4_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE4_A {
    type Ux = u8;
}
impl crate::IsEnum for MODE4_A {}
#[doc = "Field `MODE4` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type MODE4_R = crate::FieldReader<MODE4_A>;
impl MODE4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE4_A {
        match self.bits {
            0 => MODE4_A::B_0x0,
            1 => MODE4_A::B_0x1,
            2 => MODE4_A::B_0x2,
            3 => MODE4_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MODE4_A::B_0x0
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MODE4_A::B_0x1
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == MODE4_A::B_0x2
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == MODE4_A::B_0x3
    }
}
#[doc = "Field `MODE4` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type MODE4_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE4_A, crate::Safe>;
impl<'a, REG> MODE4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MODE4_A::B_0x0)
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MODE4_A::B_0x1)
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MODE4_A::B_0x2)
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MODE4_A::B_0x3)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE5_A {
    #[doc = "0: Input mode"]
    B_0x0 = 0,
    #[doc = "1: General purpose output mode"]
    B_0x1 = 1,
    #[doc = "2: Alternate function mode"]
    B_0x2 = 2,
    #[doc = "3: Analog mode (reset state)"]
    B_0x3 = 3,
}
impl From<MODE5_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE5_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE5_A {
    type Ux = u8;
}
impl crate::IsEnum for MODE5_A {}
#[doc = "Field `MODE5` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type MODE5_R = crate::FieldReader<MODE5_A>;
impl MODE5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE5_A {
        match self.bits {
            0 => MODE5_A::B_0x0,
            1 => MODE5_A::B_0x1,
            2 => MODE5_A::B_0x2,
            3 => MODE5_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MODE5_A::B_0x0
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MODE5_A::B_0x1
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == MODE5_A::B_0x2
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == MODE5_A::B_0x3
    }
}
#[doc = "Field `MODE5` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type MODE5_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE5_A, crate::Safe>;
impl<'a, REG> MODE5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MODE5_A::B_0x0)
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MODE5_A::B_0x1)
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MODE5_A::B_0x2)
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MODE5_A::B_0x3)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE6_A {
    #[doc = "0: Input mode"]
    B_0x0 = 0,
    #[doc = "1: General purpose output mode"]
    B_0x1 = 1,
    #[doc = "2: Alternate function mode"]
    B_0x2 = 2,
    #[doc = "3: Analog mode (reset state)"]
    B_0x3 = 3,
}
impl From<MODE6_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE6_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE6_A {
    type Ux = u8;
}
impl crate::IsEnum for MODE6_A {}
#[doc = "Field `MODE6` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type MODE6_R = crate::FieldReader<MODE6_A>;
impl MODE6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE6_A {
        match self.bits {
            0 => MODE6_A::B_0x0,
            1 => MODE6_A::B_0x1,
            2 => MODE6_A::B_0x2,
            3 => MODE6_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MODE6_A::B_0x0
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MODE6_A::B_0x1
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == MODE6_A::B_0x2
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == MODE6_A::B_0x3
    }
}
#[doc = "Field `MODE6` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type MODE6_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE6_A, crate::Safe>;
impl<'a, REG> MODE6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MODE6_A::B_0x0)
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MODE6_A::B_0x1)
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MODE6_A::B_0x2)
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MODE6_A::B_0x3)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE7_A {
    #[doc = "0: Input mode"]
    B_0x0 = 0,
    #[doc = "1: General purpose output mode"]
    B_0x1 = 1,
    #[doc = "2: Alternate function mode"]
    B_0x2 = 2,
    #[doc = "3: Analog mode (reset state)"]
    B_0x3 = 3,
}
impl From<MODE7_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE7_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE7_A {
    type Ux = u8;
}
impl crate::IsEnum for MODE7_A {}
#[doc = "Field `MODE7` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type MODE7_R = crate::FieldReader<MODE7_A>;
impl MODE7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE7_A {
        match self.bits {
            0 => MODE7_A::B_0x0,
            1 => MODE7_A::B_0x1,
            2 => MODE7_A::B_0x2,
            3 => MODE7_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MODE7_A::B_0x0
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MODE7_A::B_0x1
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == MODE7_A::B_0x2
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == MODE7_A::B_0x3
    }
}
#[doc = "Field `MODE7` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type MODE7_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE7_A, crate::Safe>;
impl<'a, REG> MODE7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MODE7_A::B_0x0)
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MODE7_A::B_0x1)
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MODE7_A::B_0x2)
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MODE7_A::B_0x3)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE8_A {
    #[doc = "0: Input mode"]
    B_0x0 = 0,
    #[doc = "1: General purpose output mode"]
    B_0x1 = 1,
    #[doc = "2: Alternate function mode"]
    B_0x2 = 2,
    #[doc = "3: Analog mode (reset state)"]
    B_0x3 = 3,
}
impl From<MODE8_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE8_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE8_A {
    type Ux = u8;
}
impl crate::IsEnum for MODE8_A {}
#[doc = "Field `MODE8` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type MODE8_R = crate::FieldReader<MODE8_A>;
impl MODE8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE8_A {
        match self.bits {
            0 => MODE8_A::B_0x0,
            1 => MODE8_A::B_0x1,
            2 => MODE8_A::B_0x2,
            3 => MODE8_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MODE8_A::B_0x0
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MODE8_A::B_0x1
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == MODE8_A::B_0x2
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == MODE8_A::B_0x3
    }
}
#[doc = "Field `MODE8` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type MODE8_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE8_A, crate::Safe>;
impl<'a, REG> MODE8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MODE8_A::B_0x0)
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MODE8_A::B_0x1)
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MODE8_A::B_0x2)
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MODE8_A::B_0x3)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE9_A {
    #[doc = "0: Input mode"]
    B_0x0 = 0,
    #[doc = "1: General purpose output mode"]
    B_0x1 = 1,
    #[doc = "2: Alternate function mode"]
    B_0x2 = 2,
    #[doc = "3: Analog mode (reset state)"]
    B_0x3 = 3,
}
impl From<MODE9_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE9_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE9_A {
    type Ux = u8;
}
impl crate::IsEnum for MODE9_A {}
#[doc = "Field `MODE9` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type MODE9_R = crate::FieldReader<MODE9_A>;
impl MODE9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE9_A {
        match self.bits {
            0 => MODE9_A::B_0x0,
            1 => MODE9_A::B_0x1,
            2 => MODE9_A::B_0x2,
            3 => MODE9_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MODE9_A::B_0x0
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MODE9_A::B_0x1
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == MODE9_A::B_0x2
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == MODE9_A::B_0x3
    }
}
#[doc = "Field `MODE9` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type MODE9_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE9_A, crate::Safe>;
impl<'a, REG> MODE9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MODE9_A::B_0x0)
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MODE9_A::B_0x1)
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MODE9_A::B_0x2)
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MODE9_A::B_0x3)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE10_A {
    #[doc = "0: Input mode"]
    B_0x0 = 0,
    #[doc = "1: General purpose output mode"]
    B_0x1 = 1,
    #[doc = "2: Alternate function mode"]
    B_0x2 = 2,
    #[doc = "3: Analog mode (reset state)"]
    B_0x3 = 3,
}
impl From<MODE10_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE10_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE10_A {
    type Ux = u8;
}
impl crate::IsEnum for MODE10_A {}
#[doc = "Field `MODE10` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type MODE10_R = crate::FieldReader<MODE10_A>;
impl MODE10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE10_A {
        match self.bits {
            0 => MODE10_A::B_0x0,
            1 => MODE10_A::B_0x1,
            2 => MODE10_A::B_0x2,
            3 => MODE10_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MODE10_A::B_0x0
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MODE10_A::B_0x1
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == MODE10_A::B_0x2
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == MODE10_A::B_0x3
    }
}
#[doc = "Field `MODE10` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type MODE10_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE10_A, crate::Safe>;
impl<'a, REG> MODE10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MODE10_A::B_0x0)
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MODE10_A::B_0x1)
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MODE10_A::B_0x2)
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MODE10_A::B_0x3)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE11_A {
    #[doc = "0: Input mode"]
    B_0x0 = 0,
    #[doc = "1: General purpose output mode"]
    B_0x1 = 1,
    #[doc = "2: Alternate function mode"]
    B_0x2 = 2,
    #[doc = "3: Analog mode (reset state)"]
    B_0x3 = 3,
}
impl From<MODE11_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE11_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE11_A {
    type Ux = u8;
}
impl crate::IsEnum for MODE11_A {}
#[doc = "Field `MODE11` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type MODE11_R = crate::FieldReader<MODE11_A>;
impl MODE11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE11_A {
        match self.bits {
            0 => MODE11_A::B_0x0,
            1 => MODE11_A::B_0x1,
            2 => MODE11_A::B_0x2,
            3 => MODE11_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MODE11_A::B_0x0
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MODE11_A::B_0x1
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == MODE11_A::B_0x2
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == MODE11_A::B_0x3
    }
}
#[doc = "Field `MODE11` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type MODE11_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE11_A, crate::Safe>;
impl<'a, REG> MODE11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MODE11_A::B_0x0)
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MODE11_A::B_0x1)
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MODE11_A::B_0x2)
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MODE11_A::B_0x3)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE12_A {
    #[doc = "0: Input mode"]
    B_0x0 = 0,
    #[doc = "1: General purpose output mode"]
    B_0x1 = 1,
    #[doc = "2: Alternate function mode"]
    B_0x2 = 2,
    #[doc = "3: Analog mode (reset state)"]
    B_0x3 = 3,
}
impl From<MODE12_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE12_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE12_A {
    type Ux = u8;
}
impl crate::IsEnum for MODE12_A {}
#[doc = "Field `MODE12` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type MODE12_R = crate::FieldReader<MODE12_A>;
impl MODE12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE12_A {
        match self.bits {
            0 => MODE12_A::B_0x0,
            1 => MODE12_A::B_0x1,
            2 => MODE12_A::B_0x2,
            3 => MODE12_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MODE12_A::B_0x0
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MODE12_A::B_0x1
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == MODE12_A::B_0x2
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == MODE12_A::B_0x3
    }
}
#[doc = "Field `MODE12` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type MODE12_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE12_A, crate::Safe>;
impl<'a, REG> MODE12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MODE12_A::B_0x0)
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MODE12_A::B_0x1)
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MODE12_A::B_0x2)
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MODE12_A::B_0x3)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE13_A {
    #[doc = "0: Input mode"]
    B_0x0 = 0,
    #[doc = "1: General purpose output mode"]
    B_0x1 = 1,
    #[doc = "2: Alternate function mode"]
    B_0x2 = 2,
    #[doc = "3: Analog mode (reset state)"]
    B_0x3 = 3,
}
impl From<MODE13_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE13_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE13_A {
    type Ux = u8;
}
impl crate::IsEnum for MODE13_A {}
#[doc = "Field `MODE13` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type MODE13_R = crate::FieldReader<MODE13_A>;
impl MODE13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE13_A {
        match self.bits {
            0 => MODE13_A::B_0x0,
            1 => MODE13_A::B_0x1,
            2 => MODE13_A::B_0x2,
            3 => MODE13_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MODE13_A::B_0x0
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MODE13_A::B_0x1
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == MODE13_A::B_0x2
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == MODE13_A::B_0x3
    }
}
#[doc = "Field `MODE13` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type MODE13_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE13_A, crate::Safe>;
impl<'a, REG> MODE13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MODE13_A::B_0x0)
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MODE13_A::B_0x1)
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MODE13_A::B_0x2)
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MODE13_A::B_0x3)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE14_A {
    #[doc = "0: Input mode"]
    B_0x0 = 0,
    #[doc = "1: General purpose output mode"]
    B_0x1 = 1,
    #[doc = "2: Alternate function mode"]
    B_0x2 = 2,
    #[doc = "3: Analog mode (reset state)"]
    B_0x3 = 3,
}
impl From<MODE14_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE14_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE14_A {
    type Ux = u8;
}
impl crate::IsEnum for MODE14_A {}
#[doc = "Field `MODE14` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type MODE14_R = crate::FieldReader<MODE14_A>;
impl MODE14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE14_A {
        match self.bits {
            0 => MODE14_A::B_0x0,
            1 => MODE14_A::B_0x1,
            2 => MODE14_A::B_0x2,
            3 => MODE14_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MODE14_A::B_0x0
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MODE14_A::B_0x1
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == MODE14_A::B_0x2
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == MODE14_A::B_0x3
    }
}
#[doc = "Field `MODE14` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type MODE14_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE14_A, crate::Safe>;
impl<'a, REG> MODE14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MODE14_A::B_0x0)
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MODE14_A::B_0x1)
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MODE14_A::B_0x2)
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MODE14_A::B_0x3)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE15_A {
    #[doc = "0: Input mode"]
    B_0x0 = 0,
    #[doc = "1: General purpose output mode"]
    B_0x1 = 1,
    #[doc = "2: Alternate function mode"]
    B_0x2 = 2,
    #[doc = "3: Analog mode (reset state)"]
    B_0x3 = 3,
}
impl From<MODE15_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE15_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE15_A {
    type Ux = u8;
}
impl crate::IsEnum for MODE15_A {}
#[doc = "Field `MODE15` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type MODE15_R = crate::FieldReader<MODE15_A>;
impl MODE15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE15_A {
        match self.bits {
            0 => MODE15_A::B_0x0,
            1 => MODE15_A::B_0x1,
            2 => MODE15_A::B_0x2,
            3 => MODE15_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MODE15_A::B_0x0
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MODE15_A::B_0x1
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == MODE15_A::B_0x2
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == MODE15_A::B_0x3
    }
}
#[doc = "Field `MODE15` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type MODE15_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE15_A, crate::Safe>;
impl<'a, REG> MODE15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MODE15_A::B_0x0)
    }
    #[doc = "General purpose output mode"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MODE15_A::B_0x1)
    }
    #[doc = "Alternate function mode"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MODE15_A::B_0x2)
    }
    #[doc = "Analog mode (reset state)"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MODE15_A::B_0x3)
    }
}
impl R {
    #[doc = "Bits 0:1 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn MODE0(&self) -> MODE0_R {
        MODE0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn MODE1(&self) -> MODE1_R {
        MODE1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn MODE2(&self) -> MODE2_R {
        MODE2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn MODE3(&self) -> MODE3_R {
        MODE3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn MODE4(&self) -> MODE4_R {
        MODE4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn MODE5(&self) -> MODE5_R {
        MODE5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn MODE6(&self) -> MODE6_R {
        MODE6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn MODE7(&self) -> MODE7_R {
        MODE7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn MODE8(&self) -> MODE8_R {
        MODE8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn MODE9(&self) -> MODE9_R {
        MODE9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn MODE10(&self) -> MODE10_R {
        MODE10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn MODE11(&self) -> MODE11_R {
        MODE11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn MODE12(&self) -> MODE12_R {
        MODE12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn MODE13(&self) -> MODE13_R {
        MODE13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn MODE14(&self) -> MODE14_R {
        MODE14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn MODE15(&self) -> MODE15_R {
        MODE15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn MODE0(&mut self) -> MODE0_W<'_, MODER_SPEC> {
        MODE0_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn MODE1(&mut self) -> MODE1_W<'_, MODER_SPEC> {
        MODE1_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn MODE2(&mut self) -> MODE2_W<'_, MODER_SPEC> {
        MODE2_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn MODE3(&mut self) -> MODE3_W<'_, MODER_SPEC> {
        MODE3_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn MODE4(&mut self) -> MODE4_W<'_, MODER_SPEC> {
        MODE4_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn MODE5(&mut self) -> MODE5_W<'_, MODER_SPEC> {
        MODE5_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn MODE6(&mut self) -> MODE6_W<'_, MODER_SPEC> {
        MODE6_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn MODE7(&mut self) -> MODE7_W<'_, MODER_SPEC> {
        MODE7_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn MODE8(&mut self) -> MODE8_W<'_, MODER_SPEC> {
        MODE8_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn MODE9(&mut self) -> MODE9_W<'_, MODER_SPEC> {
        MODE9_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn MODE10(&mut self) -> MODE10_W<'_, MODER_SPEC> {
        MODE10_W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn MODE11(&mut self) -> MODE11_W<'_, MODER_SPEC> {
        MODE11_W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn MODE12(&mut self) -> MODE12_W<'_, MODER_SPEC> {
        MODE12_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn MODE13(&mut self) -> MODE13_W<'_, MODER_SPEC> {
        MODE13_W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn MODE14(&mut self) -> MODE14_W<'_, MODER_SPEC> {
        MODE14_W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O mode. Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn MODE15(&mut self) -> MODE15_W<'_, MODER_SPEC> {
        MODE15_W::new(self, 30)
    }
}
#[doc = "GPIO port mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`moder::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`moder::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODER_SPEC;
impl crate::RegisterSpec for MODER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`moder::R`](R) reader structure"]
impl crate::Readable for MODER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`moder::W`](W) writer structure"]
impl crate::Writable for MODER_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets MODER to value 0xabff_ffff"]
impl crate::Resettable for MODER_SPEC {
    const RESET_VALUE: u32 = 0xabff_ffff;
}
