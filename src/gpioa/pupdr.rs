#[doc = "Register `PUPDR` reader"]
pub type R = crate::R<PUPDR_SPEC>;
#[doc = "Register `PUPDR` writer"]
pub type W = crate::W<PUPDR_SPEC>;
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PUPD0_A {
    #[doc = "0: No pull-up, pull-down"]
    B_0x0 = 0,
    #[doc = "1: Pull-up"]
    B_0x1 = 1,
    #[doc = "2: Pull-down"]
    B_0x2 = 2,
}
impl From<PUPD0_A> for u8 {
    #[inline(always)]
    fn from(variant: PUPD0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PUPD0_A {
    type Ux = u8;
}
impl crate::IsEnum for PUPD0_A {}
#[doc = "Field `PUPD0` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type PUPD0_R = crate::FieldReader<PUPD0_A>;
impl PUPD0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PUPD0_A> {
        match self.bits {
            0 => Some(PUPD0_A::B_0x0),
            1 => Some(PUPD0_A::B_0x1),
            2 => Some(PUPD0_A::B_0x2),
            _ => None,
        }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PUPD0_A::B_0x0
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PUPD0_A::B_0x1
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == PUPD0_A::B_0x2
    }
}
#[doc = "Field `PUPD0` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type PUPD0_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PUPD0_A>;
impl<'a, REG> PUPD0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD0_A::B_0x0)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD0_A::B_0x1)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD0_A::B_0x2)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PUPD1_A {
    #[doc = "0: No pull-up, pull-down"]
    B_0x0 = 0,
    #[doc = "1: Pull-up"]
    B_0x1 = 1,
    #[doc = "2: Pull-down"]
    B_0x2 = 2,
}
impl From<PUPD1_A> for u8 {
    #[inline(always)]
    fn from(variant: PUPD1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PUPD1_A {
    type Ux = u8;
}
impl crate::IsEnum for PUPD1_A {}
#[doc = "Field `PUPD1` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type PUPD1_R = crate::FieldReader<PUPD1_A>;
impl PUPD1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PUPD1_A> {
        match self.bits {
            0 => Some(PUPD1_A::B_0x0),
            1 => Some(PUPD1_A::B_0x1),
            2 => Some(PUPD1_A::B_0x2),
            _ => None,
        }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PUPD1_A::B_0x0
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PUPD1_A::B_0x1
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == PUPD1_A::B_0x2
    }
}
#[doc = "Field `PUPD1` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type PUPD1_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PUPD1_A>;
impl<'a, REG> PUPD1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD1_A::B_0x0)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD1_A::B_0x1)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD1_A::B_0x2)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PUPD2_A {
    #[doc = "0: No pull-up, pull-down"]
    B_0x0 = 0,
    #[doc = "1: Pull-up"]
    B_0x1 = 1,
    #[doc = "2: Pull-down"]
    B_0x2 = 2,
}
impl From<PUPD2_A> for u8 {
    #[inline(always)]
    fn from(variant: PUPD2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PUPD2_A {
    type Ux = u8;
}
impl crate::IsEnum for PUPD2_A {}
#[doc = "Field `PUPD2` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type PUPD2_R = crate::FieldReader<PUPD2_A>;
impl PUPD2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PUPD2_A> {
        match self.bits {
            0 => Some(PUPD2_A::B_0x0),
            1 => Some(PUPD2_A::B_0x1),
            2 => Some(PUPD2_A::B_0x2),
            _ => None,
        }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PUPD2_A::B_0x0
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PUPD2_A::B_0x1
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == PUPD2_A::B_0x2
    }
}
#[doc = "Field `PUPD2` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type PUPD2_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PUPD2_A>;
impl<'a, REG> PUPD2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD2_A::B_0x0)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD2_A::B_0x1)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD2_A::B_0x2)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PUPD3_A {
    #[doc = "0: No pull-up, pull-down"]
    B_0x0 = 0,
    #[doc = "1: Pull-up"]
    B_0x1 = 1,
    #[doc = "2: Pull-down"]
    B_0x2 = 2,
}
impl From<PUPD3_A> for u8 {
    #[inline(always)]
    fn from(variant: PUPD3_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PUPD3_A {
    type Ux = u8;
}
impl crate::IsEnum for PUPD3_A {}
#[doc = "Field `PUPD3` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type PUPD3_R = crate::FieldReader<PUPD3_A>;
impl PUPD3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PUPD3_A> {
        match self.bits {
            0 => Some(PUPD3_A::B_0x0),
            1 => Some(PUPD3_A::B_0x1),
            2 => Some(PUPD3_A::B_0x2),
            _ => None,
        }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PUPD3_A::B_0x0
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PUPD3_A::B_0x1
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == PUPD3_A::B_0x2
    }
}
#[doc = "Field `PUPD3` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type PUPD3_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PUPD3_A>;
impl<'a, REG> PUPD3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD3_A::B_0x0)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD3_A::B_0x1)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD3_A::B_0x2)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PUPD4_A {
    #[doc = "0: No pull-up, pull-down"]
    B_0x0 = 0,
    #[doc = "1: Pull-up"]
    B_0x1 = 1,
    #[doc = "2: Pull-down"]
    B_0x2 = 2,
}
impl From<PUPD4_A> for u8 {
    #[inline(always)]
    fn from(variant: PUPD4_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PUPD4_A {
    type Ux = u8;
}
impl crate::IsEnum for PUPD4_A {}
#[doc = "Field `PUPD4` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type PUPD4_R = crate::FieldReader<PUPD4_A>;
impl PUPD4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PUPD4_A> {
        match self.bits {
            0 => Some(PUPD4_A::B_0x0),
            1 => Some(PUPD4_A::B_0x1),
            2 => Some(PUPD4_A::B_0x2),
            _ => None,
        }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PUPD4_A::B_0x0
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PUPD4_A::B_0x1
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == PUPD4_A::B_0x2
    }
}
#[doc = "Field `PUPD4` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type PUPD4_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PUPD4_A>;
impl<'a, REG> PUPD4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD4_A::B_0x0)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD4_A::B_0x1)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD4_A::B_0x2)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PUPD5_A {
    #[doc = "0: No pull-up, pull-down"]
    B_0x0 = 0,
    #[doc = "1: Pull-up"]
    B_0x1 = 1,
    #[doc = "2: Pull-down"]
    B_0x2 = 2,
}
impl From<PUPD5_A> for u8 {
    #[inline(always)]
    fn from(variant: PUPD5_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PUPD5_A {
    type Ux = u8;
}
impl crate::IsEnum for PUPD5_A {}
#[doc = "Field `PUPD5` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type PUPD5_R = crate::FieldReader<PUPD5_A>;
impl PUPD5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PUPD5_A> {
        match self.bits {
            0 => Some(PUPD5_A::B_0x0),
            1 => Some(PUPD5_A::B_0x1),
            2 => Some(PUPD5_A::B_0x2),
            _ => None,
        }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PUPD5_A::B_0x0
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PUPD5_A::B_0x1
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == PUPD5_A::B_0x2
    }
}
#[doc = "Field `PUPD5` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type PUPD5_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PUPD5_A>;
impl<'a, REG> PUPD5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD5_A::B_0x0)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD5_A::B_0x1)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD5_A::B_0x2)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PUPD6_A {
    #[doc = "0: No pull-up, pull-down"]
    B_0x0 = 0,
    #[doc = "1: Pull-up"]
    B_0x1 = 1,
    #[doc = "2: Pull-down"]
    B_0x2 = 2,
}
impl From<PUPD6_A> for u8 {
    #[inline(always)]
    fn from(variant: PUPD6_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PUPD6_A {
    type Ux = u8;
}
impl crate::IsEnum for PUPD6_A {}
#[doc = "Field `PUPD6` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type PUPD6_R = crate::FieldReader<PUPD6_A>;
impl PUPD6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PUPD6_A> {
        match self.bits {
            0 => Some(PUPD6_A::B_0x0),
            1 => Some(PUPD6_A::B_0x1),
            2 => Some(PUPD6_A::B_0x2),
            _ => None,
        }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PUPD6_A::B_0x0
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PUPD6_A::B_0x1
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == PUPD6_A::B_0x2
    }
}
#[doc = "Field `PUPD6` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type PUPD6_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PUPD6_A>;
impl<'a, REG> PUPD6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD6_A::B_0x0)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD6_A::B_0x1)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD6_A::B_0x2)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PUPD7_A {
    #[doc = "0: No pull-up, pull-down"]
    B_0x0 = 0,
    #[doc = "1: Pull-up"]
    B_0x1 = 1,
    #[doc = "2: Pull-down"]
    B_0x2 = 2,
}
impl From<PUPD7_A> for u8 {
    #[inline(always)]
    fn from(variant: PUPD7_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PUPD7_A {
    type Ux = u8;
}
impl crate::IsEnum for PUPD7_A {}
#[doc = "Field `PUPD7` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type PUPD7_R = crate::FieldReader<PUPD7_A>;
impl PUPD7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PUPD7_A> {
        match self.bits {
            0 => Some(PUPD7_A::B_0x0),
            1 => Some(PUPD7_A::B_0x1),
            2 => Some(PUPD7_A::B_0x2),
            _ => None,
        }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PUPD7_A::B_0x0
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PUPD7_A::B_0x1
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == PUPD7_A::B_0x2
    }
}
#[doc = "Field `PUPD7` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type PUPD7_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PUPD7_A>;
impl<'a, REG> PUPD7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD7_A::B_0x0)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD7_A::B_0x1)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD7_A::B_0x2)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PUPD8_A {
    #[doc = "0: No pull-up, pull-down"]
    B_0x0 = 0,
    #[doc = "1: Pull-up"]
    B_0x1 = 1,
    #[doc = "2: Pull-down"]
    B_0x2 = 2,
}
impl From<PUPD8_A> for u8 {
    #[inline(always)]
    fn from(variant: PUPD8_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PUPD8_A {
    type Ux = u8;
}
impl crate::IsEnum for PUPD8_A {}
#[doc = "Field `PUPD8` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type PUPD8_R = crate::FieldReader<PUPD8_A>;
impl PUPD8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PUPD8_A> {
        match self.bits {
            0 => Some(PUPD8_A::B_0x0),
            1 => Some(PUPD8_A::B_0x1),
            2 => Some(PUPD8_A::B_0x2),
            _ => None,
        }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PUPD8_A::B_0x0
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PUPD8_A::B_0x1
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == PUPD8_A::B_0x2
    }
}
#[doc = "Field `PUPD8` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type PUPD8_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PUPD8_A>;
impl<'a, REG> PUPD8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD8_A::B_0x0)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD8_A::B_0x1)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD8_A::B_0x2)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PUPD9_A {
    #[doc = "0: No pull-up, pull-down"]
    B_0x0 = 0,
    #[doc = "1: Pull-up"]
    B_0x1 = 1,
    #[doc = "2: Pull-down"]
    B_0x2 = 2,
}
impl From<PUPD9_A> for u8 {
    #[inline(always)]
    fn from(variant: PUPD9_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PUPD9_A {
    type Ux = u8;
}
impl crate::IsEnum for PUPD9_A {}
#[doc = "Field `PUPD9` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type PUPD9_R = crate::FieldReader<PUPD9_A>;
impl PUPD9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PUPD9_A> {
        match self.bits {
            0 => Some(PUPD9_A::B_0x0),
            1 => Some(PUPD9_A::B_0x1),
            2 => Some(PUPD9_A::B_0x2),
            _ => None,
        }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PUPD9_A::B_0x0
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PUPD9_A::B_0x1
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == PUPD9_A::B_0x2
    }
}
#[doc = "Field `PUPD9` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type PUPD9_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PUPD9_A>;
impl<'a, REG> PUPD9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD9_A::B_0x0)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD9_A::B_0x1)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD9_A::B_0x2)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PUPD10_A {
    #[doc = "0: No pull-up, pull-down"]
    B_0x0 = 0,
    #[doc = "1: Pull-up"]
    B_0x1 = 1,
    #[doc = "2: Pull-down"]
    B_0x2 = 2,
}
impl From<PUPD10_A> for u8 {
    #[inline(always)]
    fn from(variant: PUPD10_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PUPD10_A {
    type Ux = u8;
}
impl crate::IsEnum for PUPD10_A {}
#[doc = "Field `PUPD10` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type PUPD10_R = crate::FieldReader<PUPD10_A>;
impl PUPD10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PUPD10_A> {
        match self.bits {
            0 => Some(PUPD10_A::B_0x0),
            1 => Some(PUPD10_A::B_0x1),
            2 => Some(PUPD10_A::B_0x2),
            _ => None,
        }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PUPD10_A::B_0x0
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PUPD10_A::B_0x1
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == PUPD10_A::B_0x2
    }
}
#[doc = "Field `PUPD10` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type PUPD10_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PUPD10_A>;
impl<'a, REG> PUPD10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD10_A::B_0x0)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD10_A::B_0x1)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD10_A::B_0x2)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PUPD11_A {
    #[doc = "0: No pull-up, pull-down"]
    B_0x0 = 0,
    #[doc = "1: Pull-up"]
    B_0x1 = 1,
    #[doc = "2: Pull-down"]
    B_0x2 = 2,
}
impl From<PUPD11_A> for u8 {
    #[inline(always)]
    fn from(variant: PUPD11_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PUPD11_A {
    type Ux = u8;
}
impl crate::IsEnum for PUPD11_A {}
#[doc = "Field `PUPD11` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type PUPD11_R = crate::FieldReader<PUPD11_A>;
impl PUPD11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PUPD11_A> {
        match self.bits {
            0 => Some(PUPD11_A::B_0x0),
            1 => Some(PUPD11_A::B_0x1),
            2 => Some(PUPD11_A::B_0x2),
            _ => None,
        }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PUPD11_A::B_0x0
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PUPD11_A::B_0x1
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == PUPD11_A::B_0x2
    }
}
#[doc = "Field `PUPD11` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type PUPD11_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PUPD11_A>;
impl<'a, REG> PUPD11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD11_A::B_0x0)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD11_A::B_0x1)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD11_A::B_0x2)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PUPD12_A {
    #[doc = "0: No pull-up, pull-down"]
    B_0x0 = 0,
    #[doc = "1: Pull-up"]
    B_0x1 = 1,
    #[doc = "2: Pull-down"]
    B_0x2 = 2,
}
impl From<PUPD12_A> for u8 {
    #[inline(always)]
    fn from(variant: PUPD12_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PUPD12_A {
    type Ux = u8;
}
impl crate::IsEnum for PUPD12_A {}
#[doc = "Field `PUPD12` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type PUPD12_R = crate::FieldReader<PUPD12_A>;
impl PUPD12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PUPD12_A> {
        match self.bits {
            0 => Some(PUPD12_A::B_0x0),
            1 => Some(PUPD12_A::B_0x1),
            2 => Some(PUPD12_A::B_0x2),
            _ => None,
        }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PUPD12_A::B_0x0
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PUPD12_A::B_0x1
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == PUPD12_A::B_0x2
    }
}
#[doc = "Field `PUPD12` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type PUPD12_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PUPD12_A>;
impl<'a, REG> PUPD12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD12_A::B_0x0)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD12_A::B_0x1)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD12_A::B_0x2)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PUPD13_A {
    #[doc = "0: No pull-up, pull-down"]
    B_0x0 = 0,
    #[doc = "1: Pull-up"]
    B_0x1 = 1,
    #[doc = "2: Pull-down"]
    B_0x2 = 2,
}
impl From<PUPD13_A> for u8 {
    #[inline(always)]
    fn from(variant: PUPD13_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PUPD13_A {
    type Ux = u8;
}
impl crate::IsEnum for PUPD13_A {}
#[doc = "Field `PUPD13` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type PUPD13_R = crate::FieldReader<PUPD13_A>;
impl PUPD13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PUPD13_A> {
        match self.bits {
            0 => Some(PUPD13_A::B_0x0),
            1 => Some(PUPD13_A::B_0x1),
            2 => Some(PUPD13_A::B_0x2),
            _ => None,
        }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PUPD13_A::B_0x0
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PUPD13_A::B_0x1
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == PUPD13_A::B_0x2
    }
}
#[doc = "Field `PUPD13` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type PUPD13_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PUPD13_A>;
impl<'a, REG> PUPD13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD13_A::B_0x0)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD13_A::B_0x1)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD13_A::B_0x2)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PUPD14_A {
    #[doc = "0: No pull-up, pull-down"]
    B_0x0 = 0,
    #[doc = "1: Pull-up"]
    B_0x1 = 1,
    #[doc = "2: Pull-down"]
    B_0x2 = 2,
}
impl From<PUPD14_A> for u8 {
    #[inline(always)]
    fn from(variant: PUPD14_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PUPD14_A {
    type Ux = u8;
}
impl crate::IsEnum for PUPD14_A {}
#[doc = "Field `PUPD14` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type PUPD14_R = crate::FieldReader<PUPD14_A>;
impl PUPD14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PUPD14_A> {
        match self.bits {
            0 => Some(PUPD14_A::B_0x0),
            1 => Some(PUPD14_A::B_0x1),
            2 => Some(PUPD14_A::B_0x2),
            _ => None,
        }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PUPD14_A::B_0x0
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PUPD14_A::B_0x1
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == PUPD14_A::B_0x2
    }
}
#[doc = "Field `PUPD14` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type PUPD14_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PUPD14_A>;
impl<'a, REG> PUPD14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD14_A::B_0x0)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD14_A::B_0x1)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD14_A::B_0x2)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PUPD15_A {
    #[doc = "0: No pull-up, pull-down"]
    B_0x0 = 0,
    #[doc = "1: Pull-up"]
    B_0x1 = 1,
    #[doc = "2: Pull-down"]
    B_0x2 = 2,
}
impl From<PUPD15_A> for u8 {
    #[inline(always)]
    fn from(variant: PUPD15_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PUPD15_A {
    type Ux = u8;
}
impl crate::IsEnum for PUPD15_A {}
#[doc = "Field `PUPD15` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type PUPD15_R = crate::FieldReader<PUPD15_A>;
impl PUPD15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PUPD15_A> {
        match self.bits {
            0 => Some(PUPD15_A::B_0x0),
            1 => Some(PUPD15_A::B_0x1),
            2 => Some(PUPD15_A::B_0x2),
            _ => None,
        }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PUPD15_A::B_0x0
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PUPD15_A::B_0x1
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == PUPD15_A::B_0x2
    }
}
#[doc = "Field `PUPD15` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type PUPD15_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PUPD15_A>;
impl<'a, REG> PUPD15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD15_A::B_0x0)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD15_A::B_0x1)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PUPD15_A::B_0x2)
    }
}
impl R {
    #[doc = "Bits 0:1 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn PUPD0(&self) -> PUPD0_R {
        PUPD0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn PUPD1(&self) -> PUPD1_R {
        PUPD1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn PUPD2(&self) -> PUPD2_R {
        PUPD2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn PUPD3(&self) -> PUPD3_R {
        PUPD3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn PUPD4(&self) -> PUPD4_R {
        PUPD4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn PUPD5(&self) -> PUPD5_R {
        PUPD5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn PUPD6(&self) -> PUPD6_R {
        PUPD6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn PUPD7(&self) -> PUPD7_R {
        PUPD7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn PUPD8(&self) -> PUPD8_R {
        PUPD8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn PUPD9(&self) -> PUPD9_R {
        PUPD9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn PUPD10(&self) -> PUPD10_R {
        PUPD10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn PUPD11(&self) -> PUPD11_R {
        PUPD11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn PUPD12(&self) -> PUPD12_R {
        PUPD12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn PUPD13(&self) -> PUPD13_R {
        PUPD13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn PUPD14(&self) -> PUPD14_R {
        PUPD14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn PUPD15(&self) -> PUPD15_R {
        PUPD15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn PUPD0(&mut self) -> PUPD0_W<'_, PUPDR_SPEC> {
        PUPD0_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn PUPD1(&mut self) -> PUPD1_W<'_, PUPDR_SPEC> {
        PUPD1_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn PUPD2(&mut self) -> PUPD2_W<'_, PUPDR_SPEC> {
        PUPD2_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn PUPD3(&mut self) -> PUPD3_W<'_, PUPDR_SPEC> {
        PUPD3_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn PUPD4(&mut self) -> PUPD4_W<'_, PUPDR_SPEC> {
        PUPD4_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn PUPD5(&mut self) -> PUPD5_W<'_, PUPDR_SPEC> {
        PUPD5_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn PUPD6(&mut self) -> PUPD6_W<'_, PUPDR_SPEC> {
        PUPD6_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn PUPD7(&mut self) -> PUPD7_W<'_, PUPDR_SPEC> {
        PUPD7_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn PUPD8(&mut self) -> PUPD8_W<'_, PUPDR_SPEC> {
        PUPD8_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn PUPD9(&mut self) -> PUPD9_W<'_, PUPDR_SPEC> {
        PUPD9_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn PUPD10(&mut self) -> PUPD10_W<'_, PUPDR_SPEC> {
        PUPD10_W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn PUPD11(&mut self) -> PUPD11_W<'_, PUPDR_SPEC> {
        PUPD11_W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn PUPD12(&mut self) -> PUPD12_W<'_, PUPDR_SPEC> {
        PUPD12_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn PUPD13(&mut self) -> PUPD13_W<'_, PUPDR_SPEC> {
        PUPD13_W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn PUPD14(&mut self) -> PUPD14_W<'_, PUPDR_SPEC> {
        PUPD14_W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O pull-up or pull-down Note: The bitfield is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn PUPD15(&mut self) -> PUPD15_W<'_, PUPDR_SPEC> {
        PUPD15_W::new(self, 30)
    }
}
#[doc = "GPIO port pull-up/pull-down register\n\nYou can [`read`](crate::Reg::read) this register and get [`pupdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pupdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PUPDR_SPEC;
impl crate::RegisterSpec for PUPDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pupdr::R`](R) reader structure"]
impl crate::Readable for PUPDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pupdr::W`](W) writer structure"]
impl crate::Writable for PUPDR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets PUPDR to value 0x6400_0000"]
impl crate::Resettable for PUPDR_SPEC {
    const RESET_VALUE: u32 = 0x6400_0000;
}
