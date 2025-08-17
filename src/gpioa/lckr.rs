#[doc = "Register `LCKR` reader"]
pub type R = crate::R<LCKR_SPEC>;
#[doc = "Register `LCKR` writer"]
pub type W = crate::W<LCKR_SPEC>;
#[doc = "Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCK0_A {
    #[doc = "0: Port configuration not locked"]
    B_0x0 = 0,
    #[doc = "1: Port configuration locked"]
    B_0x1 = 1,
}
impl From<LCK0_A> for bool {
    #[inline(always)]
    fn from(variant: LCK0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCK0` reader - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type LCK0_R = crate::BitReader<LCK0_A>;
impl LCK0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LCK0_A {
        match self.bits {
            false => LCK0_A::B_0x0,
            true => LCK0_A::B_0x1,
        }
    }
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LCK0_A::B_0x0
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LCK0_A::B_0x1
    }
}
#[doc = "Field `LCK0` writer - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type LCK0_W<'a, REG> = crate::BitWriter<'a, REG, LCK0_A>;
impl<'a, REG> LCK0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LCK0_A::B_0x0)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LCK0_A::B_0x1)
    }
}
#[doc = "Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCK1_A {
    #[doc = "0: Port configuration not locked"]
    B_0x0 = 0,
    #[doc = "1: Port configuration locked"]
    B_0x1 = 1,
}
impl From<LCK1_A> for bool {
    #[inline(always)]
    fn from(variant: LCK1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCK1` reader - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type LCK1_R = crate::BitReader<LCK1_A>;
impl LCK1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LCK1_A {
        match self.bits {
            false => LCK1_A::B_0x0,
            true => LCK1_A::B_0x1,
        }
    }
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LCK1_A::B_0x0
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LCK1_A::B_0x1
    }
}
#[doc = "Field `LCK1` writer - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type LCK1_W<'a, REG> = crate::BitWriter<'a, REG, LCK1_A>;
impl<'a, REG> LCK1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LCK1_A::B_0x0)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LCK1_A::B_0x1)
    }
}
#[doc = "Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCK2_A {
    #[doc = "0: Port configuration not locked"]
    B_0x0 = 0,
    #[doc = "1: Port configuration locked"]
    B_0x1 = 1,
}
impl From<LCK2_A> for bool {
    #[inline(always)]
    fn from(variant: LCK2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCK2` reader - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type LCK2_R = crate::BitReader<LCK2_A>;
impl LCK2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LCK2_A {
        match self.bits {
            false => LCK2_A::B_0x0,
            true => LCK2_A::B_0x1,
        }
    }
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LCK2_A::B_0x0
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LCK2_A::B_0x1
    }
}
#[doc = "Field `LCK2` writer - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type LCK2_W<'a, REG> = crate::BitWriter<'a, REG, LCK2_A>;
impl<'a, REG> LCK2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LCK2_A::B_0x0)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LCK2_A::B_0x1)
    }
}
#[doc = "Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCK3_A {
    #[doc = "0: Port configuration not locked"]
    B_0x0 = 0,
    #[doc = "1: Port configuration locked"]
    B_0x1 = 1,
}
impl From<LCK3_A> for bool {
    #[inline(always)]
    fn from(variant: LCK3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCK3` reader - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type LCK3_R = crate::BitReader<LCK3_A>;
impl LCK3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LCK3_A {
        match self.bits {
            false => LCK3_A::B_0x0,
            true => LCK3_A::B_0x1,
        }
    }
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LCK3_A::B_0x0
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LCK3_A::B_0x1
    }
}
#[doc = "Field `LCK3` writer - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type LCK3_W<'a, REG> = crate::BitWriter<'a, REG, LCK3_A>;
impl<'a, REG> LCK3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LCK3_A::B_0x0)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LCK3_A::B_0x1)
    }
}
#[doc = "Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCK4_A {
    #[doc = "0: Port configuration not locked"]
    B_0x0 = 0,
    #[doc = "1: Port configuration locked"]
    B_0x1 = 1,
}
impl From<LCK4_A> for bool {
    #[inline(always)]
    fn from(variant: LCK4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCK4` reader - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type LCK4_R = crate::BitReader<LCK4_A>;
impl LCK4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LCK4_A {
        match self.bits {
            false => LCK4_A::B_0x0,
            true => LCK4_A::B_0x1,
        }
    }
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LCK4_A::B_0x0
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LCK4_A::B_0x1
    }
}
#[doc = "Field `LCK4` writer - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type LCK4_W<'a, REG> = crate::BitWriter<'a, REG, LCK4_A>;
impl<'a, REG> LCK4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LCK4_A::B_0x0)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LCK4_A::B_0x1)
    }
}
#[doc = "Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCK5_A {
    #[doc = "0: Port configuration not locked"]
    B_0x0 = 0,
    #[doc = "1: Port configuration locked"]
    B_0x1 = 1,
}
impl From<LCK5_A> for bool {
    #[inline(always)]
    fn from(variant: LCK5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCK5` reader - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type LCK5_R = crate::BitReader<LCK5_A>;
impl LCK5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LCK5_A {
        match self.bits {
            false => LCK5_A::B_0x0,
            true => LCK5_A::B_0x1,
        }
    }
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LCK5_A::B_0x0
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LCK5_A::B_0x1
    }
}
#[doc = "Field `LCK5` writer - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type LCK5_W<'a, REG> = crate::BitWriter<'a, REG, LCK5_A>;
impl<'a, REG> LCK5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LCK5_A::B_0x0)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LCK5_A::B_0x1)
    }
}
#[doc = "Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCK6_A {
    #[doc = "0: Port configuration not locked"]
    B_0x0 = 0,
    #[doc = "1: Port configuration locked"]
    B_0x1 = 1,
}
impl From<LCK6_A> for bool {
    #[inline(always)]
    fn from(variant: LCK6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCK6` reader - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type LCK6_R = crate::BitReader<LCK6_A>;
impl LCK6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LCK6_A {
        match self.bits {
            false => LCK6_A::B_0x0,
            true => LCK6_A::B_0x1,
        }
    }
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LCK6_A::B_0x0
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LCK6_A::B_0x1
    }
}
#[doc = "Field `LCK6` writer - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type LCK6_W<'a, REG> = crate::BitWriter<'a, REG, LCK6_A>;
impl<'a, REG> LCK6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LCK6_A::B_0x0)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LCK6_A::B_0x1)
    }
}
#[doc = "Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCK7_A {
    #[doc = "0: Port configuration not locked"]
    B_0x0 = 0,
    #[doc = "1: Port configuration locked"]
    B_0x1 = 1,
}
impl From<LCK7_A> for bool {
    #[inline(always)]
    fn from(variant: LCK7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCK7` reader - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type LCK7_R = crate::BitReader<LCK7_A>;
impl LCK7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LCK7_A {
        match self.bits {
            false => LCK7_A::B_0x0,
            true => LCK7_A::B_0x1,
        }
    }
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LCK7_A::B_0x0
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LCK7_A::B_0x1
    }
}
#[doc = "Field `LCK7` writer - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type LCK7_W<'a, REG> = crate::BitWriter<'a, REG, LCK7_A>;
impl<'a, REG> LCK7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LCK7_A::B_0x0)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LCK7_A::B_0x1)
    }
}
#[doc = "Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCK8_A {
    #[doc = "0: Port configuration not locked"]
    B_0x0 = 0,
    #[doc = "1: Port configuration locked"]
    B_0x1 = 1,
}
impl From<LCK8_A> for bool {
    #[inline(always)]
    fn from(variant: LCK8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCK8` reader - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type LCK8_R = crate::BitReader<LCK8_A>;
impl LCK8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LCK8_A {
        match self.bits {
            false => LCK8_A::B_0x0,
            true => LCK8_A::B_0x1,
        }
    }
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LCK8_A::B_0x0
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LCK8_A::B_0x1
    }
}
#[doc = "Field `LCK8` writer - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type LCK8_W<'a, REG> = crate::BitWriter<'a, REG, LCK8_A>;
impl<'a, REG> LCK8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LCK8_A::B_0x0)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LCK8_A::B_0x1)
    }
}
#[doc = "Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCK9_A {
    #[doc = "0: Port configuration not locked"]
    B_0x0 = 0,
    #[doc = "1: Port configuration locked"]
    B_0x1 = 1,
}
impl From<LCK9_A> for bool {
    #[inline(always)]
    fn from(variant: LCK9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCK9` reader - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type LCK9_R = crate::BitReader<LCK9_A>;
impl LCK9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LCK9_A {
        match self.bits {
            false => LCK9_A::B_0x0,
            true => LCK9_A::B_0x1,
        }
    }
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LCK9_A::B_0x0
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LCK9_A::B_0x1
    }
}
#[doc = "Field `LCK9` writer - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type LCK9_W<'a, REG> = crate::BitWriter<'a, REG, LCK9_A>;
impl<'a, REG> LCK9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LCK9_A::B_0x0)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LCK9_A::B_0x1)
    }
}
#[doc = "Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCK10_A {
    #[doc = "0: Port configuration not locked"]
    B_0x0 = 0,
    #[doc = "1: Port configuration locked"]
    B_0x1 = 1,
}
impl From<LCK10_A> for bool {
    #[inline(always)]
    fn from(variant: LCK10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCK10` reader - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type LCK10_R = crate::BitReader<LCK10_A>;
impl LCK10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LCK10_A {
        match self.bits {
            false => LCK10_A::B_0x0,
            true => LCK10_A::B_0x1,
        }
    }
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LCK10_A::B_0x0
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LCK10_A::B_0x1
    }
}
#[doc = "Field `LCK10` writer - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type LCK10_W<'a, REG> = crate::BitWriter<'a, REG, LCK10_A>;
impl<'a, REG> LCK10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LCK10_A::B_0x0)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LCK10_A::B_0x1)
    }
}
#[doc = "Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCK11_A {
    #[doc = "0: Port configuration not locked"]
    B_0x0 = 0,
    #[doc = "1: Port configuration locked"]
    B_0x1 = 1,
}
impl From<LCK11_A> for bool {
    #[inline(always)]
    fn from(variant: LCK11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCK11` reader - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type LCK11_R = crate::BitReader<LCK11_A>;
impl LCK11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LCK11_A {
        match self.bits {
            false => LCK11_A::B_0x0,
            true => LCK11_A::B_0x1,
        }
    }
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LCK11_A::B_0x0
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LCK11_A::B_0x1
    }
}
#[doc = "Field `LCK11` writer - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type LCK11_W<'a, REG> = crate::BitWriter<'a, REG, LCK11_A>;
impl<'a, REG> LCK11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LCK11_A::B_0x0)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LCK11_A::B_0x1)
    }
}
#[doc = "Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCK12_A {
    #[doc = "0: Port configuration not locked"]
    B_0x0 = 0,
    #[doc = "1: Port configuration locked"]
    B_0x1 = 1,
}
impl From<LCK12_A> for bool {
    #[inline(always)]
    fn from(variant: LCK12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCK12` reader - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type LCK12_R = crate::BitReader<LCK12_A>;
impl LCK12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LCK12_A {
        match self.bits {
            false => LCK12_A::B_0x0,
            true => LCK12_A::B_0x1,
        }
    }
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LCK12_A::B_0x0
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LCK12_A::B_0x1
    }
}
#[doc = "Field `LCK12` writer - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type LCK12_W<'a, REG> = crate::BitWriter<'a, REG, LCK12_A>;
impl<'a, REG> LCK12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LCK12_A::B_0x0)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LCK12_A::B_0x1)
    }
}
#[doc = "Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCK13_A {
    #[doc = "0: Port configuration not locked"]
    B_0x0 = 0,
    #[doc = "1: Port configuration locked"]
    B_0x1 = 1,
}
impl From<LCK13_A> for bool {
    #[inline(always)]
    fn from(variant: LCK13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCK13` reader - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type LCK13_R = crate::BitReader<LCK13_A>;
impl LCK13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LCK13_A {
        match self.bits {
            false => LCK13_A::B_0x0,
            true => LCK13_A::B_0x1,
        }
    }
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LCK13_A::B_0x0
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LCK13_A::B_0x1
    }
}
#[doc = "Field `LCK13` writer - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type LCK13_W<'a, REG> = crate::BitWriter<'a, REG, LCK13_A>;
impl<'a, REG> LCK13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LCK13_A::B_0x0)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LCK13_A::B_0x1)
    }
}
#[doc = "Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCK14_A {
    #[doc = "0: Port configuration not locked"]
    B_0x0 = 0,
    #[doc = "1: Port configuration locked"]
    B_0x1 = 1,
}
impl From<LCK14_A> for bool {
    #[inline(always)]
    fn from(variant: LCK14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCK14` reader - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type LCK14_R = crate::BitReader<LCK14_A>;
impl LCK14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LCK14_A {
        match self.bits {
            false => LCK14_A::B_0x0,
            true => LCK14_A::B_0x1,
        }
    }
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LCK14_A::B_0x0
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LCK14_A::B_0x1
    }
}
#[doc = "Field `LCK14` writer - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type LCK14_W<'a, REG> = crate::BitWriter<'a, REG, LCK14_A>;
impl<'a, REG> LCK14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LCK14_A::B_0x0)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LCK14_A::B_0x1)
    }
}
#[doc = "Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCK15_A {
    #[doc = "0: Port configuration not locked"]
    B_0x0 = 0,
    #[doc = "1: Port configuration locked"]
    B_0x1 = 1,
}
impl From<LCK15_A> for bool {
    #[inline(always)]
    fn from(variant: LCK15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCK15` reader - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type LCK15_R = crate::BitReader<LCK15_A>;
impl LCK15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LCK15_A {
        match self.bits {
            false => LCK15_A::B_0x0,
            true => LCK15_A::B_0x1,
        }
    }
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LCK15_A::B_0x0
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LCK15_A::B_0x1
    }
}
#[doc = "Field `LCK15` writer - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type LCK15_W<'a, REG> = crate::BitWriter<'a, REG, LCK15_A>;
impl<'a, REG> LCK15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LCK15_A::B_0x0)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LCK15_A::B_0x1)
    }
}
#[doc = "Lock key This bit can be read any time. It can only be modified using the lock key write sequence. - LOCK key write sequence: WR LCKR\\[16\\] = 1 + LCKR\\[15:0\\] WR LCKR\\[16\\] = 0 + LCKR\\[15:0\\] WR LCKR\\[16\\] = 1 + LCKR\\[15:0\\] - LOCK key read RD LCKR\\[16\\] = 1 (this read operation is optional but it confirms that the lock is active) Note: During the LOCK key write sequence, the value of LCK\\[15:0\\] must not change. Any error in the lock sequence aborts the LOCK. After the first LOCK sequence on any bit of the port, any read access on the LCKK bit returns 1 until the next MCU reset or peripheral reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCKK_A {
    #[doc = "0: Port configuration lock key not active"]
    B_0x0 = 0,
    #[doc = "1: Port configuration lock key active. The GPIOx_LCKR register is locked until the next MCU reset or peripheral reset."]
    B_0x1 = 1,
}
impl From<LCKK_A> for bool {
    #[inline(always)]
    fn from(variant: LCKK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCKK` reader - Lock key This bit can be read any time. It can only be modified using the lock key write sequence. - LOCK key write sequence: WR LCKR\\[16\\] = 1 + LCKR\\[15:0\\] WR LCKR\\[16\\] = 0 + LCKR\\[15:0\\] WR LCKR\\[16\\] = 1 + LCKR\\[15:0\\] - LOCK key read RD LCKR\\[16\\] = 1 (this read operation is optional but it confirms that the lock is active) Note: During the LOCK key write sequence, the value of LCK\\[15:0\\] must not change. Any error in the lock sequence aborts the LOCK. After the first LOCK sequence on any bit of the port, any read access on the LCKK bit returns 1 until the next MCU reset or peripheral reset."]
pub type LCKK_R = crate::BitReader<LCKK_A>;
impl LCKK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LCKK_A {
        match self.bits {
            false => LCKK_A::B_0x0,
            true => LCKK_A::B_0x1,
        }
    }
    #[doc = "Port configuration lock key not active"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LCKK_A::B_0x0
    }
    #[doc = "Port configuration lock key active. The GPIOx_LCKR register is locked until the next MCU reset or peripheral reset."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LCKK_A::B_0x1
    }
}
#[doc = "Field `LCKK` writer - Lock key This bit can be read any time. It can only be modified using the lock key write sequence. - LOCK key write sequence: WR LCKR\\[16\\] = 1 + LCKR\\[15:0\\] WR LCKR\\[16\\] = 0 + LCKR\\[15:0\\] WR LCKR\\[16\\] = 1 + LCKR\\[15:0\\] - LOCK key read RD LCKR\\[16\\] = 1 (this read operation is optional but it confirms that the lock is active) Note: During the LOCK key write sequence, the value of LCK\\[15:0\\] must not change. Any error in the lock sequence aborts the LOCK. After the first LOCK sequence on any bit of the port, any read access on the LCKK bit returns 1 until the next MCU reset or peripheral reset."]
pub type LCKK_W<'a, REG> = crate::BitWriter<'a, REG, LCKK_A>;
impl<'a, REG> LCKK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port configuration lock key not active"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LCKK_A::B_0x0)
    }
    #[doc = "Port configuration lock key active. The GPIOx_LCKR register is locked until the next MCU reset or peripheral reset."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LCKK_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn LCK0(&self) -> LCK0_R {
        LCK0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn LCK1(&self) -> LCK1_R {
        LCK1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn LCK2(&self) -> LCK2_R {
        LCK2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn LCK3(&self) -> LCK3_R {
        LCK3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn LCK4(&self) -> LCK4_R {
        LCK4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn LCK5(&self) -> LCK5_R {
        LCK5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn LCK6(&self) -> LCK6_R {
        LCK6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn LCK7(&self) -> LCK7_R {
        LCK7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn LCK8(&self) -> LCK8_R {
        LCK8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn LCK9(&self) -> LCK9_R {
        LCK9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn LCK10(&self) -> LCK10_R {
        LCK10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn LCK11(&self) -> LCK11_R {
        LCK11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn LCK12(&self) -> LCK12_R {
        LCK12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn LCK13(&self) -> LCK13_R {
        LCK13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn LCK14(&self) -> LCK14_R {
        LCK14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn LCK15(&self) -> LCK15_R {
        LCK15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Lock key This bit can be read any time. It can only be modified using the lock key write sequence. - LOCK key write sequence: WR LCKR\\[16\\] = 1 + LCKR\\[15:0\\] WR LCKR\\[16\\] = 0 + LCKR\\[15:0\\] WR LCKR\\[16\\] = 1 + LCKR\\[15:0\\] - LOCK key read RD LCKR\\[16\\] = 1 (this read operation is optional but it confirms that the lock is active) Note: During the LOCK key write sequence, the value of LCK\\[15:0\\] must not change. Any error in the lock sequence aborts the LOCK. After the first LOCK sequence on any bit of the port, any read access on the LCKK bit returns 1 until the next MCU reset or peripheral reset."]
    #[inline(always)]
    pub fn LCKK(&self) -> LCKK_R {
        LCKK_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn LCK0(&mut self) -> LCK0_W<'_, LCKR_SPEC> {
        LCK0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn LCK1(&mut self) -> LCK1_W<'_, LCKR_SPEC> {
        LCK1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn LCK2(&mut self) -> LCK2_W<'_, LCKR_SPEC> {
        LCK2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn LCK3(&mut self) -> LCK3_W<'_, LCKR_SPEC> {
        LCK3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn LCK4(&mut self) -> LCK4_W<'_, LCKR_SPEC> {
        LCK4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn LCK5(&mut self) -> LCK5_W<'_, LCKR_SPEC> {
        LCK5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn LCK6(&mut self) -> LCK6_W<'_, LCKR_SPEC> {
        LCK6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn LCK7(&mut self) -> LCK7_W<'_, LCKR_SPEC> {
        LCK7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn LCK8(&mut self) -> LCK8_W<'_, LCKR_SPEC> {
        LCK8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn LCK9(&mut self) -> LCK9_W<'_, LCKR_SPEC> {
        LCK9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn LCK10(&mut self) -> LCK10_W<'_, LCKR_SPEC> {
        LCK10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn LCK11(&mut self) -> LCK11_W<'_, LCKR_SPEC> {
        LCK11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn LCK12(&mut self) -> LCK12_W<'_, LCKR_SPEC> {
        LCK12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn LCK13(&mut self) -> LCK13_W<'_, LCKR_SPEC> {
        LCK13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn LCK14(&mut self) -> LCK14_W<'_, LCKR_SPEC> {
        LCK14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Port x lock I/O pin y (y = 15 to 0) These bits are read/write but can only be written when the LCKK bit is 0 Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn LCK15(&mut self) -> LCK15_W<'_, LCKR_SPEC> {
        LCK15_W::new(self, 15)
    }
    #[doc = "Bit 16 - Lock key This bit can be read any time. It can only be modified using the lock key write sequence. - LOCK key write sequence: WR LCKR\\[16\\] = 1 + LCKR\\[15:0\\] WR LCKR\\[16\\] = 0 + LCKR\\[15:0\\] WR LCKR\\[16\\] = 1 + LCKR\\[15:0\\] - LOCK key read RD LCKR\\[16\\] = 1 (this read operation is optional but it confirms that the lock is active) Note: During the LOCK key write sequence, the value of LCK\\[15:0\\] must not change. Any error in the lock sequence aborts the LOCK. After the first LOCK sequence on any bit of the port, any read access on the LCKK bit returns 1 until the next MCU reset or peripheral reset."]
    #[inline(always)]
    pub fn LCKK(&mut self) -> LCKK_W<'_, LCKR_SPEC> {
        LCKK_W::new(self, 16)
    }
}
#[doc = "GPIO port configuration lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`lckr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lckr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCKR_SPEC;
impl crate::RegisterSpec for LCKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lckr::R`](R) reader structure"]
impl crate::Readable for LCKR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lckr::W`](W) writer structure"]
impl crate::Writable for LCKR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets LCKR to value 0"]
impl crate::Resettable for LCKR_SPEC {}
