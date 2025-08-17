#[doc = "Register `OTYPER` reader"]
pub type R = crate::R<OTYPER_SPEC>;
#[doc = "Register `OTYPER` writer"]
pub type W = crate::W<OTYPER_SPEC>;
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OT0_A {
    #[doc = "0: Output push-pull (reset state)"]
    B_0x0 = 0,
    #[doc = "1: Output open-drain"]
    B_0x1 = 1,
}
impl From<OT0_A> for bool {
    #[inline(always)]
    fn from(variant: OT0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OT0` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT0_R = crate::BitReader<OT0_A>;
impl OT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OT0_A {
        match self.bits {
            false => OT0_A::B_0x0,
            true => OT0_A::B_0x1,
        }
    }
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OT0_A::B_0x0
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OT0_A::B_0x1
    }
}
#[doc = "Field `OT0` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT0_W<'a, REG> = crate::BitWriter<'a, REG, OT0_A>;
impl<'a, REG> OT0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OT0_A::B_0x0)
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OT0_A::B_0x1)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OT1_A {
    #[doc = "0: Output push-pull (reset state)"]
    B_0x0 = 0,
    #[doc = "1: Output open-drain"]
    B_0x1 = 1,
}
impl From<OT1_A> for bool {
    #[inline(always)]
    fn from(variant: OT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OT1` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT1_R = crate::BitReader<OT1_A>;
impl OT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OT1_A {
        match self.bits {
            false => OT1_A::B_0x0,
            true => OT1_A::B_0x1,
        }
    }
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OT1_A::B_0x0
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OT1_A::B_0x1
    }
}
#[doc = "Field `OT1` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT1_W<'a, REG> = crate::BitWriter<'a, REG, OT1_A>;
impl<'a, REG> OT1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OT1_A::B_0x0)
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OT1_A::B_0x1)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OT2_A {
    #[doc = "0: Output push-pull (reset state)"]
    B_0x0 = 0,
    #[doc = "1: Output open-drain"]
    B_0x1 = 1,
}
impl From<OT2_A> for bool {
    #[inline(always)]
    fn from(variant: OT2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OT2` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT2_R = crate::BitReader<OT2_A>;
impl OT2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OT2_A {
        match self.bits {
            false => OT2_A::B_0x0,
            true => OT2_A::B_0x1,
        }
    }
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OT2_A::B_0x0
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OT2_A::B_0x1
    }
}
#[doc = "Field `OT2` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT2_W<'a, REG> = crate::BitWriter<'a, REG, OT2_A>;
impl<'a, REG> OT2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OT2_A::B_0x0)
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OT2_A::B_0x1)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OT3_A {
    #[doc = "0: Output push-pull (reset state)"]
    B_0x0 = 0,
    #[doc = "1: Output open-drain"]
    B_0x1 = 1,
}
impl From<OT3_A> for bool {
    #[inline(always)]
    fn from(variant: OT3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OT3` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT3_R = crate::BitReader<OT3_A>;
impl OT3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OT3_A {
        match self.bits {
            false => OT3_A::B_0x0,
            true => OT3_A::B_0x1,
        }
    }
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OT3_A::B_0x0
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OT3_A::B_0x1
    }
}
#[doc = "Field `OT3` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT3_W<'a, REG> = crate::BitWriter<'a, REG, OT3_A>;
impl<'a, REG> OT3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OT3_A::B_0x0)
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OT3_A::B_0x1)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OT4_A {
    #[doc = "0: Output push-pull (reset state)"]
    B_0x0 = 0,
    #[doc = "1: Output open-drain"]
    B_0x1 = 1,
}
impl From<OT4_A> for bool {
    #[inline(always)]
    fn from(variant: OT4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OT4` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT4_R = crate::BitReader<OT4_A>;
impl OT4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OT4_A {
        match self.bits {
            false => OT4_A::B_0x0,
            true => OT4_A::B_0x1,
        }
    }
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OT4_A::B_0x0
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OT4_A::B_0x1
    }
}
#[doc = "Field `OT4` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT4_W<'a, REG> = crate::BitWriter<'a, REG, OT4_A>;
impl<'a, REG> OT4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OT4_A::B_0x0)
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OT4_A::B_0x1)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OT5_A {
    #[doc = "0: Output push-pull (reset state)"]
    B_0x0 = 0,
    #[doc = "1: Output open-drain"]
    B_0x1 = 1,
}
impl From<OT5_A> for bool {
    #[inline(always)]
    fn from(variant: OT5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OT5` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT5_R = crate::BitReader<OT5_A>;
impl OT5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OT5_A {
        match self.bits {
            false => OT5_A::B_0x0,
            true => OT5_A::B_0x1,
        }
    }
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OT5_A::B_0x0
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OT5_A::B_0x1
    }
}
#[doc = "Field `OT5` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT5_W<'a, REG> = crate::BitWriter<'a, REG, OT5_A>;
impl<'a, REG> OT5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OT5_A::B_0x0)
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OT5_A::B_0x1)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OT6_A {
    #[doc = "0: Output push-pull (reset state)"]
    B_0x0 = 0,
    #[doc = "1: Output open-drain"]
    B_0x1 = 1,
}
impl From<OT6_A> for bool {
    #[inline(always)]
    fn from(variant: OT6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OT6` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT6_R = crate::BitReader<OT6_A>;
impl OT6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OT6_A {
        match self.bits {
            false => OT6_A::B_0x0,
            true => OT6_A::B_0x1,
        }
    }
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OT6_A::B_0x0
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OT6_A::B_0x1
    }
}
#[doc = "Field `OT6` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT6_W<'a, REG> = crate::BitWriter<'a, REG, OT6_A>;
impl<'a, REG> OT6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OT6_A::B_0x0)
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OT6_A::B_0x1)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OT7_A {
    #[doc = "0: Output push-pull (reset state)"]
    B_0x0 = 0,
    #[doc = "1: Output open-drain"]
    B_0x1 = 1,
}
impl From<OT7_A> for bool {
    #[inline(always)]
    fn from(variant: OT7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OT7` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT7_R = crate::BitReader<OT7_A>;
impl OT7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OT7_A {
        match self.bits {
            false => OT7_A::B_0x0,
            true => OT7_A::B_0x1,
        }
    }
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OT7_A::B_0x0
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OT7_A::B_0x1
    }
}
#[doc = "Field `OT7` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT7_W<'a, REG> = crate::BitWriter<'a, REG, OT7_A>;
impl<'a, REG> OT7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OT7_A::B_0x0)
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OT7_A::B_0x1)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OT8_A {
    #[doc = "0: Output push-pull (reset state)"]
    B_0x0 = 0,
    #[doc = "1: Output open-drain"]
    B_0x1 = 1,
}
impl From<OT8_A> for bool {
    #[inline(always)]
    fn from(variant: OT8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OT8` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT8_R = crate::BitReader<OT8_A>;
impl OT8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OT8_A {
        match self.bits {
            false => OT8_A::B_0x0,
            true => OT8_A::B_0x1,
        }
    }
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OT8_A::B_0x0
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OT8_A::B_0x1
    }
}
#[doc = "Field `OT8` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT8_W<'a, REG> = crate::BitWriter<'a, REG, OT8_A>;
impl<'a, REG> OT8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OT8_A::B_0x0)
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OT8_A::B_0x1)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OT9_A {
    #[doc = "0: Output push-pull (reset state)"]
    B_0x0 = 0,
    #[doc = "1: Output open-drain"]
    B_0x1 = 1,
}
impl From<OT9_A> for bool {
    #[inline(always)]
    fn from(variant: OT9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OT9` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT9_R = crate::BitReader<OT9_A>;
impl OT9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OT9_A {
        match self.bits {
            false => OT9_A::B_0x0,
            true => OT9_A::B_0x1,
        }
    }
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OT9_A::B_0x0
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OT9_A::B_0x1
    }
}
#[doc = "Field `OT9` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT9_W<'a, REG> = crate::BitWriter<'a, REG, OT9_A>;
impl<'a, REG> OT9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OT9_A::B_0x0)
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OT9_A::B_0x1)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OT10_A {
    #[doc = "0: Output push-pull (reset state)"]
    B_0x0 = 0,
    #[doc = "1: Output open-drain"]
    B_0x1 = 1,
}
impl From<OT10_A> for bool {
    #[inline(always)]
    fn from(variant: OT10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OT10` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT10_R = crate::BitReader<OT10_A>;
impl OT10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OT10_A {
        match self.bits {
            false => OT10_A::B_0x0,
            true => OT10_A::B_0x1,
        }
    }
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OT10_A::B_0x0
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OT10_A::B_0x1
    }
}
#[doc = "Field `OT10` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT10_W<'a, REG> = crate::BitWriter<'a, REG, OT10_A>;
impl<'a, REG> OT10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OT10_A::B_0x0)
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OT10_A::B_0x1)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OT11_A {
    #[doc = "0: Output push-pull (reset state)"]
    B_0x0 = 0,
    #[doc = "1: Output open-drain"]
    B_0x1 = 1,
}
impl From<OT11_A> for bool {
    #[inline(always)]
    fn from(variant: OT11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OT11` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT11_R = crate::BitReader<OT11_A>;
impl OT11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OT11_A {
        match self.bits {
            false => OT11_A::B_0x0,
            true => OT11_A::B_0x1,
        }
    }
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OT11_A::B_0x0
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OT11_A::B_0x1
    }
}
#[doc = "Field `OT11` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT11_W<'a, REG> = crate::BitWriter<'a, REG, OT11_A>;
impl<'a, REG> OT11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OT11_A::B_0x0)
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OT11_A::B_0x1)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OT12_A {
    #[doc = "0: Output push-pull (reset state)"]
    B_0x0 = 0,
    #[doc = "1: Output open-drain"]
    B_0x1 = 1,
}
impl From<OT12_A> for bool {
    #[inline(always)]
    fn from(variant: OT12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OT12` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT12_R = crate::BitReader<OT12_A>;
impl OT12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OT12_A {
        match self.bits {
            false => OT12_A::B_0x0,
            true => OT12_A::B_0x1,
        }
    }
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OT12_A::B_0x0
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OT12_A::B_0x1
    }
}
#[doc = "Field `OT12` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT12_W<'a, REG> = crate::BitWriter<'a, REG, OT12_A>;
impl<'a, REG> OT12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OT12_A::B_0x0)
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OT12_A::B_0x1)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OT13_A {
    #[doc = "0: Output push-pull (reset state)"]
    B_0x0 = 0,
    #[doc = "1: Output open-drain"]
    B_0x1 = 1,
}
impl From<OT13_A> for bool {
    #[inline(always)]
    fn from(variant: OT13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OT13` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT13_R = crate::BitReader<OT13_A>;
impl OT13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OT13_A {
        match self.bits {
            false => OT13_A::B_0x0,
            true => OT13_A::B_0x1,
        }
    }
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OT13_A::B_0x0
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OT13_A::B_0x1
    }
}
#[doc = "Field `OT13` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT13_W<'a, REG> = crate::BitWriter<'a, REG, OT13_A>;
impl<'a, REG> OT13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OT13_A::B_0x0)
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OT13_A::B_0x1)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OT14_A {
    #[doc = "0: Output push-pull (reset state)"]
    B_0x0 = 0,
    #[doc = "1: Output open-drain"]
    B_0x1 = 1,
}
impl From<OT14_A> for bool {
    #[inline(always)]
    fn from(variant: OT14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OT14` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT14_R = crate::BitReader<OT14_A>;
impl OT14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OT14_A {
        match self.bits {
            false => OT14_A::B_0x0,
            true => OT14_A::B_0x1,
        }
    }
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OT14_A::B_0x0
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OT14_A::B_0x1
    }
}
#[doc = "Field `OT14` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT14_W<'a, REG> = crate::BitWriter<'a, REG, OT14_A>;
impl<'a, REG> OT14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OT14_A::B_0x0)
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OT14_A::B_0x1)
    }
}
#[doc = "Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OT15_A {
    #[doc = "0: Output push-pull (reset state)"]
    B_0x0 = 0,
    #[doc = "1: Output open-drain"]
    B_0x1 = 1,
}
impl From<OT15_A> for bool {
    #[inline(always)]
    fn from(variant: OT15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OT15` reader - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT15_R = crate::BitReader<OT15_A>;
impl OT15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OT15_A {
        match self.bits {
            false => OT15_A::B_0x0,
            true => OT15_A::B_0x1,
        }
    }
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OT15_A::B_0x0
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OT15_A::B_0x1
    }
}
#[doc = "Field `OT15` writer - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type OT15_W<'a, REG> = crate::BitWriter<'a, REG, OT15_A>;
impl<'a, REG> OT15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OT15_A::B_0x0)
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OT15_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OT0(&self) -> OT0_R {
        OT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OT1(&self) -> OT1_R {
        OT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OT2(&self) -> OT2_R {
        OT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OT3(&self) -> OT3_R {
        OT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OT4(&self) -> OT4_R {
        OT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OT5(&self) -> OT5_R {
        OT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OT6(&self) -> OT6_R {
        OT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OT7(&self) -> OT7_R {
        OT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OT8(&self) -> OT8_R {
        OT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OT9(&self) -> OT9_R {
        OT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OT10(&self) -> OT10_R {
        OT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OT11(&self) -> OT11_R {
        OT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OT12(&self) -> OT12_R {
        OT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OT13(&self) -> OT13_R {
        OT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OT14(&self) -> OT14_R {
        OT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OT15(&self) -> OT15_R {
        OT15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OT0(&mut self) -> OT0_W<'_, OTYPER_SPEC> {
        OT0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OT1(&mut self) -> OT1_W<'_, OTYPER_SPEC> {
        OT1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OT2(&mut self) -> OT2_W<'_, OTYPER_SPEC> {
        OT2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OT3(&mut self) -> OT3_W<'_, OTYPER_SPEC> {
        OT3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OT4(&mut self) -> OT4_W<'_, OTYPER_SPEC> {
        OT4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OT5(&mut self) -> OT5_W<'_, OTYPER_SPEC> {
        OT5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OT6(&mut self) -> OT6_W<'_, OTYPER_SPEC> {
        OT6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OT7(&mut self) -> OT7_W<'_, OTYPER_SPEC> {
        OT7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OT8(&mut self) -> OT8_W<'_, OTYPER_SPEC> {
        OT8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OT9(&mut self) -> OT9_W<'_, OTYPER_SPEC> {
        OT9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OT10(&mut self) -> OT10_W<'_, OTYPER_SPEC> {
        OT10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OT11(&mut self) -> OT11_W<'_, OTYPER_SPEC> {
        OT11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OT12(&mut self) -> OT12_W<'_, OTYPER_SPEC> {
        OT12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OT13(&mut self) -> OT13_W<'_, OTYPER_SPEC> {
        OT13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OT14(&mut self) -> OT14_W<'_, OTYPER_SPEC> {
        OT14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Port x configuration I/O pin y (y = 15 to 0) These bits are written by software to configure the I/O output type. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn OT15(&mut self) -> OT15_W<'_, OTYPER_SPEC> {
        OT15_W::new(self, 15)
    }
}
#[doc = "GPIO port output type register\n\nYou can [`read`](crate::Reg::read) this register and get [`otyper::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otyper::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTYPER_SPEC;
impl crate::RegisterSpec for OTYPER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otyper::R`](R) reader structure"]
impl crate::Readable for OTYPER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`otyper::W`](W) writer structure"]
impl crate::Writable for OTYPER_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets OTYPER to value 0"]
impl crate::Resettable for OTYPER_SPEC {}
