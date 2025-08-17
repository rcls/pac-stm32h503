#[doc = "Register `BSRR` writer"]
pub type W = crate::W<BSRR_SPEC>;
#[doc = "Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BS0_A {
    #[doc = "0: No action on the corresponding ODy bit"]
    B_0x0 = 0,
    #[doc = "1: Sets the corresponding ODy bit"]
    B_0x1 = 1,
}
impl From<BS0_A> for bool {
    #[inline(always)]
    fn from(variant: BS0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BS0` writer - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type BS0_W<'a, REG> = crate::BitWriter<'a, REG, BS0_A>;
impl<'a, REG> BS0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BS0_A::B_0x0)
    }
    #[doc = "Sets the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BS0_A::B_0x1)
    }
}
#[doc = "Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BS1_A {
    #[doc = "0: No action on the corresponding ODy bit"]
    B_0x0 = 0,
    #[doc = "1: Sets the corresponding ODy bit"]
    B_0x1 = 1,
}
impl From<BS1_A> for bool {
    #[inline(always)]
    fn from(variant: BS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BS1` writer - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type BS1_W<'a, REG> = crate::BitWriter<'a, REG, BS1_A>;
impl<'a, REG> BS1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BS1_A::B_0x0)
    }
    #[doc = "Sets the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BS1_A::B_0x1)
    }
}
#[doc = "Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BS2_A {
    #[doc = "0: No action on the corresponding ODy bit"]
    B_0x0 = 0,
    #[doc = "1: Sets the corresponding ODy bit"]
    B_0x1 = 1,
}
impl From<BS2_A> for bool {
    #[inline(always)]
    fn from(variant: BS2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BS2` writer - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type BS2_W<'a, REG> = crate::BitWriter<'a, REG, BS2_A>;
impl<'a, REG> BS2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BS2_A::B_0x0)
    }
    #[doc = "Sets the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BS2_A::B_0x1)
    }
}
#[doc = "Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BS3_A {
    #[doc = "0: No action on the corresponding ODy bit"]
    B_0x0 = 0,
    #[doc = "1: Sets the corresponding ODy bit"]
    B_0x1 = 1,
}
impl From<BS3_A> for bool {
    #[inline(always)]
    fn from(variant: BS3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BS3` writer - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type BS3_W<'a, REG> = crate::BitWriter<'a, REG, BS3_A>;
impl<'a, REG> BS3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BS3_A::B_0x0)
    }
    #[doc = "Sets the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BS3_A::B_0x1)
    }
}
#[doc = "Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BS4_A {
    #[doc = "0: No action on the corresponding ODy bit"]
    B_0x0 = 0,
    #[doc = "1: Sets the corresponding ODy bit"]
    B_0x1 = 1,
}
impl From<BS4_A> for bool {
    #[inline(always)]
    fn from(variant: BS4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BS4` writer - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type BS4_W<'a, REG> = crate::BitWriter<'a, REG, BS4_A>;
impl<'a, REG> BS4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BS4_A::B_0x0)
    }
    #[doc = "Sets the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BS4_A::B_0x1)
    }
}
#[doc = "Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BS5_A {
    #[doc = "0: No action on the corresponding ODy bit"]
    B_0x0 = 0,
    #[doc = "1: Sets the corresponding ODy bit"]
    B_0x1 = 1,
}
impl From<BS5_A> for bool {
    #[inline(always)]
    fn from(variant: BS5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BS5` writer - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type BS5_W<'a, REG> = crate::BitWriter<'a, REG, BS5_A>;
impl<'a, REG> BS5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BS5_A::B_0x0)
    }
    #[doc = "Sets the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BS5_A::B_0x1)
    }
}
#[doc = "Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BS6_A {
    #[doc = "0: No action on the corresponding ODy bit"]
    B_0x0 = 0,
    #[doc = "1: Sets the corresponding ODy bit"]
    B_0x1 = 1,
}
impl From<BS6_A> for bool {
    #[inline(always)]
    fn from(variant: BS6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BS6` writer - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type BS6_W<'a, REG> = crate::BitWriter<'a, REG, BS6_A>;
impl<'a, REG> BS6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BS6_A::B_0x0)
    }
    #[doc = "Sets the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BS6_A::B_0x1)
    }
}
#[doc = "Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BS7_A {
    #[doc = "0: No action on the corresponding ODy bit"]
    B_0x0 = 0,
    #[doc = "1: Sets the corresponding ODy bit"]
    B_0x1 = 1,
}
impl From<BS7_A> for bool {
    #[inline(always)]
    fn from(variant: BS7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BS7` writer - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type BS7_W<'a, REG> = crate::BitWriter<'a, REG, BS7_A>;
impl<'a, REG> BS7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BS7_A::B_0x0)
    }
    #[doc = "Sets the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BS7_A::B_0x1)
    }
}
#[doc = "Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BS8_A {
    #[doc = "0: No action on the corresponding ODy bit"]
    B_0x0 = 0,
    #[doc = "1: Sets the corresponding ODy bit"]
    B_0x1 = 1,
}
impl From<BS8_A> for bool {
    #[inline(always)]
    fn from(variant: BS8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BS8` writer - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type BS8_W<'a, REG> = crate::BitWriter<'a, REG, BS8_A>;
impl<'a, REG> BS8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BS8_A::B_0x0)
    }
    #[doc = "Sets the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BS8_A::B_0x1)
    }
}
#[doc = "Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BS9_A {
    #[doc = "0: No action on the corresponding ODy bit"]
    B_0x0 = 0,
    #[doc = "1: Sets the corresponding ODy bit"]
    B_0x1 = 1,
}
impl From<BS9_A> for bool {
    #[inline(always)]
    fn from(variant: BS9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BS9` writer - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type BS9_W<'a, REG> = crate::BitWriter<'a, REG, BS9_A>;
impl<'a, REG> BS9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BS9_A::B_0x0)
    }
    #[doc = "Sets the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BS9_A::B_0x1)
    }
}
#[doc = "Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BS10_A {
    #[doc = "0: No action on the corresponding ODy bit"]
    B_0x0 = 0,
    #[doc = "1: Sets the corresponding ODy bit"]
    B_0x1 = 1,
}
impl From<BS10_A> for bool {
    #[inline(always)]
    fn from(variant: BS10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BS10` writer - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type BS10_W<'a, REG> = crate::BitWriter<'a, REG, BS10_A>;
impl<'a, REG> BS10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BS10_A::B_0x0)
    }
    #[doc = "Sets the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BS10_A::B_0x1)
    }
}
#[doc = "Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BS11_A {
    #[doc = "0: No action on the corresponding ODy bit"]
    B_0x0 = 0,
    #[doc = "1: Sets the corresponding ODy bit"]
    B_0x1 = 1,
}
impl From<BS11_A> for bool {
    #[inline(always)]
    fn from(variant: BS11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BS11` writer - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type BS11_W<'a, REG> = crate::BitWriter<'a, REG, BS11_A>;
impl<'a, REG> BS11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BS11_A::B_0x0)
    }
    #[doc = "Sets the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BS11_A::B_0x1)
    }
}
#[doc = "Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BS12_A {
    #[doc = "0: No action on the corresponding ODy bit"]
    B_0x0 = 0,
    #[doc = "1: Sets the corresponding ODy bit"]
    B_0x1 = 1,
}
impl From<BS12_A> for bool {
    #[inline(always)]
    fn from(variant: BS12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BS12` writer - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type BS12_W<'a, REG> = crate::BitWriter<'a, REG, BS12_A>;
impl<'a, REG> BS12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BS12_A::B_0x0)
    }
    #[doc = "Sets the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BS12_A::B_0x1)
    }
}
#[doc = "Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BS13_A {
    #[doc = "0: No action on the corresponding ODy bit"]
    B_0x0 = 0,
    #[doc = "1: Sets the corresponding ODy bit"]
    B_0x1 = 1,
}
impl From<BS13_A> for bool {
    #[inline(always)]
    fn from(variant: BS13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BS13` writer - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type BS13_W<'a, REG> = crate::BitWriter<'a, REG, BS13_A>;
impl<'a, REG> BS13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BS13_A::B_0x0)
    }
    #[doc = "Sets the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BS13_A::B_0x1)
    }
}
#[doc = "Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BS14_A {
    #[doc = "0: No action on the corresponding ODy bit"]
    B_0x0 = 0,
    #[doc = "1: Sets the corresponding ODy bit"]
    B_0x1 = 1,
}
impl From<BS14_A> for bool {
    #[inline(always)]
    fn from(variant: BS14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BS14` writer - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type BS14_W<'a, REG> = crate::BitWriter<'a, REG, BS14_A>;
impl<'a, REG> BS14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BS14_A::B_0x0)
    }
    #[doc = "Sets the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BS14_A::B_0x1)
    }
}
#[doc = "Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BS15_A {
    #[doc = "0: No action on the corresponding ODy bit"]
    B_0x0 = 0,
    #[doc = "1: Sets the corresponding ODy bit"]
    B_0x1 = 1,
}
impl From<BS15_A> for bool {
    #[inline(always)]
    fn from(variant: BS15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BS15` writer - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type BS15_W<'a, REG> = crate::BitWriter<'a, REG, BS15_A>;
impl<'a, REG> BS15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BS15_A::B_0x0)
    }
    #[doc = "Sets the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BS15_A::B_0x1)
    }
}
#[doc = "Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR0_A {
    #[doc = "0: No action on the corresponding ODy bit"]
    B_0x0 = 0,
    #[doc = "1: Resets the corresponding ODy bit"]
    B_0x1 = 1,
}
impl From<BR0_A> for bool {
    #[inline(always)]
    fn from(variant: BR0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR0` writer - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type BR0_W<'a, REG> = crate::BitWriter<'a, REG, BR0_A>;
impl<'a, REG> BR0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BR0_A::B_0x0)
    }
    #[doc = "Resets the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR0_A::B_0x1)
    }
}
#[doc = "Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR1_A {
    #[doc = "0: No action on the corresponding ODy bit"]
    B_0x0 = 0,
    #[doc = "1: Resets the corresponding ODy bit"]
    B_0x1 = 1,
}
impl From<BR1_A> for bool {
    #[inline(always)]
    fn from(variant: BR1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR1` writer - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type BR1_W<'a, REG> = crate::BitWriter<'a, REG, BR1_A>;
impl<'a, REG> BR1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BR1_A::B_0x0)
    }
    #[doc = "Resets the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR1_A::B_0x1)
    }
}
#[doc = "Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR2_A {
    #[doc = "0: No action on the corresponding ODy bit"]
    B_0x0 = 0,
    #[doc = "1: Resets the corresponding ODy bit"]
    B_0x1 = 1,
}
impl From<BR2_A> for bool {
    #[inline(always)]
    fn from(variant: BR2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR2` writer - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type BR2_W<'a, REG> = crate::BitWriter<'a, REG, BR2_A>;
impl<'a, REG> BR2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BR2_A::B_0x0)
    }
    #[doc = "Resets the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR2_A::B_0x1)
    }
}
#[doc = "Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR3_A {
    #[doc = "0: No action on the corresponding ODy bit"]
    B_0x0 = 0,
    #[doc = "1: Resets the corresponding ODy bit"]
    B_0x1 = 1,
}
impl From<BR3_A> for bool {
    #[inline(always)]
    fn from(variant: BR3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR3` writer - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type BR3_W<'a, REG> = crate::BitWriter<'a, REG, BR3_A>;
impl<'a, REG> BR3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BR3_A::B_0x0)
    }
    #[doc = "Resets the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR3_A::B_0x1)
    }
}
#[doc = "Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR4_A {
    #[doc = "0: No action on the corresponding ODy bit"]
    B_0x0 = 0,
    #[doc = "1: Resets the corresponding ODy bit"]
    B_0x1 = 1,
}
impl From<BR4_A> for bool {
    #[inline(always)]
    fn from(variant: BR4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR4` writer - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type BR4_W<'a, REG> = crate::BitWriter<'a, REG, BR4_A>;
impl<'a, REG> BR4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BR4_A::B_0x0)
    }
    #[doc = "Resets the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR4_A::B_0x1)
    }
}
#[doc = "Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR5_A {
    #[doc = "0: No action on the corresponding ODy bit"]
    B_0x0 = 0,
    #[doc = "1: Resets the corresponding ODy bit"]
    B_0x1 = 1,
}
impl From<BR5_A> for bool {
    #[inline(always)]
    fn from(variant: BR5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR5` writer - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type BR5_W<'a, REG> = crate::BitWriter<'a, REG, BR5_A>;
impl<'a, REG> BR5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BR5_A::B_0x0)
    }
    #[doc = "Resets the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR5_A::B_0x1)
    }
}
#[doc = "Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR6_A {
    #[doc = "0: No action on the corresponding ODy bit"]
    B_0x0 = 0,
    #[doc = "1: Resets the corresponding ODy bit"]
    B_0x1 = 1,
}
impl From<BR6_A> for bool {
    #[inline(always)]
    fn from(variant: BR6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR6` writer - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type BR6_W<'a, REG> = crate::BitWriter<'a, REG, BR6_A>;
impl<'a, REG> BR6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BR6_A::B_0x0)
    }
    #[doc = "Resets the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR6_A::B_0x1)
    }
}
#[doc = "Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR7_A {
    #[doc = "0: No action on the corresponding ODy bit"]
    B_0x0 = 0,
    #[doc = "1: Resets the corresponding ODy bit"]
    B_0x1 = 1,
}
impl From<BR7_A> for bool {
    #[inline(always)]
    fn from(variant: BR7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR7` writer - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type BR7_W<'a, REG> = crate::BitWriter<'a, REG, BR7_A>;
impl<'a, REG> BR7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BR7_A::B_0x0)
    }
    #[doc = "Resets the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR7_A::B_0x1)
    }
}
#[doc = "Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR8_A {
    #[doc = "0: No action on the corresponding ODy bit"]
    B_0x0 = 0,
    #[doc = "1: Resets the corresponding ODy bit"]
    B_0x1 = 1,
}
impl From<BR8_A> for bool {
    #[inline(always)]
    fn from(variant: BR8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR8` writer - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type BR8_W<'a, REG> = crate::BitWriter<'a, REG, BR8_A>;
impl<'a, REG> BR8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BR8_A::B_0x0)
    }
    #[doc = "Resets the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR8_A::B_0x1)
    }
}
#[doc = "Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR9_A {
    #[doc = "0: No action on the corresponding ODy bit"]
    B_0x0 = 0,
    #[doc = "1: Resets the corresponding ODy bit"]
    B_0x1 = 1,
}
impl From<BR9_A> for bool {
    #[inline(always)]
    fn from(variant: BR9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR9` writer - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type BR9_W<'a, REG> = crate::BitWriter<'a, REG, BR9_A>;
impl<'a, REG> BR9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BR9_A::B_0x0)
    }
    #[doc = "Resets the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR9_A::B_0x1)
    }
}
#[doc = "Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR10_A {
    #[doc = "0: No action on the corresponding ODy bit"]
    B_0x0 = 0,
    #[doc = "1: Resets the corresponding ODy bit"]
    B_0x1 = 1,
}
impl From<BR10_A> for bool {
    #[inline(always)]
    fn from(variant: BR10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR10` writer - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type BR10_W<'a, REG> = crate::BitWriter<'a, REG, BR10_A>;
impl<'a, REG> BR10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BR10_A::B_0x0)
    }
    #[doc = "Resets the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR10_A::B_0x1)
    }
}
#[doc = "Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR11_A {
    #[doc = "0: No action on the corresponding ODy bit"]
    B_0x0 = 0,
    #[doc = "1: Resets the corresponding ODy bit"]
    B_0x1 = 1,
}
impl From<BR11_A> for bool {
    #[inline(always)]
    fn from(variant: BR11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR11` writer - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type BR11_W<'a, REG> = crate::BitWriter<'a, REG, BR11_A>;
impl<'a, REG> BR11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BR11_A::B_0x0)
    }
    #[doc = "Resets the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR11_A::B_0x1)
    }
}
#[doc = "Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR12_A {
    #[doc = "0: No action on the corresponding ODy bit"]
    B_0x0 = 0,
    #[doc = "1: Resets the corresponding ODy bit"]
    B_0x1 = 1,
}
impl From<BR12_A> for bool {
    #[inline(always)]
    fn from(variant: BR12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR12` writer - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type BR12_W<'a, REG> = crate::BitWriter<'a, REG, BR12_A>;
impl<'a, REG> BR12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BR12_A::B_0x0)
    }
    #[doc = "Resets the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR12_A::B_0x1)
    }
}
#[doc = "Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR13_A {
    #[doc = "0: No action on the corresponding ODy bit"]
    B_0x0 = 0,
    #[doc = "1: Resets the corresponding ODy bit"]
    B_0x1 = 1,
}
impl From<BR13_A> for bool {
    #[inline(always)]
    fn from(variant: BR13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR13` writer - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type BR13_W<'a, REG> = crate::BitWriter<'a, REG, BR13_A>;
impl<'a, REG> BR13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BR13_A::B_0x0)
    }
    #[doc = "Resets the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR13_A::B_0x1)
    }
}
#[doc = "Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR14_A {
    #[doc = "0: No action on the corresponding ODy bit"]
    B_0x0 = 0,
    #[doc = "1: Resets the corresponding ODy bit"]
    B_0x1 = 1,
}
impl From<BR14_A> for bool {
    #[inline(always)]
    fn from(variant: BR14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR14` writer - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type BR14_W<'a, REG> = crate::BitWriter<'a, REG, BR14_A>;
impl<'a, REG> BR14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BR14_A::B_0x0)
    }
    #[doc = "Resets the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR14_A::B_0x1)
    }
}
#[doc = "Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR15_A {
    #[doc = "0: No action on the corresponding ODy bit"]
    B_0x0 = 0,
    #[doc = "1: Resets the corresponding ODy bit"]
    B_0x1 = 1,
}
impl From<BR15_A> for bool {
    #[inline(always)]
    fn from(variant: BR15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR15` writer - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
pub type BR15_W<'a, REG> = crate::BitWriter<'a, REG, BR15_A>;
impl<'a, REG> BR15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action on the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BR15_A::B_0x0)
    }
    #[doc = "Resets the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR15_A::B_0x1)
    }
}
impl W {
    #[doc = "Bit 0 - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn BS0(&mut self) -> BS0_W<'_, BSRR_SPEC> {
        BS0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn BS1(&mut self) -> BS1_W<'_, BSRR_SPEC> {
        BS1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn BS2(&mut self) -> BS2_W<'_, BSRR_SPEC> {
        BS2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn BS3(&mut self) -> BS3_W<'_, BSRR_SPEC> {
        BS3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn BS4(&mut self) -> BS4_W<'_, BSRR_SPEC> {
        BS4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn BS5(&mut self) -> BS5_W<'_, BSRR_SPEC> {
        BS5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn BS6(&mut self) -> BS6_W<'_, BSRR_SPEC> {
        BS6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn BS7(&mut self) -> BS7_W<'_, BSRR_SPEC> {
        BS7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn BS8(&mut self) -> BS8_W<'_, BSRR_SPEC> {
        BS8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn BS9(&mut self) -> BS9_W<'_, BSRR_SPEC> {
        BS9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn BS10(&mut self) -> BS10_W<'_, BSRR_SPEC> {
        BS10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn BS11(&mut self) -> BS11_W<'_, BSRR_SPEC> {
        BS11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn BS12(&mut self) -> BS12_W<'_, BSRR_SPEC> {
        BS12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn BS13(&mut self) -> BS13_W<'_, BSRR_SPEC> {
        BS13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn BS14(&mut self) -> BS14_W<'_, BSRR_SPEC> {
        BS14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn BS15(&mut self) -> BS15_W<'_, BSRR_SPEC> {
        BS15_W::new(self, 15)
    }
    #[doc = "Bit 16 - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn BR0(&mut self) -> BR0_W<'_, BSRR_SPEC> {
        BR0_W::new(self, 16)
    }
    #[doc = "Bit 17 - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn BR1(&mut self) -> BR1_W<'_, BSRR_SPEC> {
        BR1_W::new(self, 17)
    }
    #[doc = "Bit 18 - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn BR2(&mut self) -> BR2_W<'_, BSRR_SPEC> {
        BR2_W::new(self, 18)
    }
    #[doc = "Bit 19 - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn BR3(&mut self) -> BR3_W<'_, BSRR_SPEC> {
        BR3_W::new(self, 19)
    }
    #[doc = "Bit 20 - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn BR4(&mut self) -> BR4_W<'_, BSRR_SPEC> {
        BR4_W::new(self, 20)
    }
    #[doc = "Bit 21 - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn BR5(&mut self) -> BR5_W<'_, BSRR_SPEC> {
        BR5_W::new(self, 21)
    }
    #[doc = "Bit 22 - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn BR6(&mut self) -> BR6_W<'_, BSRR_SPEC> {
        BR6_W::new(self, 22)
    }
    #[doc = "Bit 23 - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn BR7(&mut self) -> BR7_W<'_, BSRR_SPEC> {
        BR7_W::new(self, 23)
    }
    #[doc = "Bit 24 - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn BR8(&mut self) -> BR8_W<'_, BSRR_SPEC> {
        BR8_W::new(self, 24)
    }
    #[doc = "Bit 25 - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn BR9(&mut self) -> BR9_W<'_, BSRR_SPEC> {
        BR9_W::new(self, 25)
    }
    #[doc = "Bit 26 - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn BR10(&mut self) -> BR10_W<'_, BSRR_SPEC> {
        BR10_W::new(self, 26)
    }
    #[doc = "Bit 27 - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn BR11(&mut self) -> BR11_W<'_, BSRR_SPEC> {
        BR11_W::new(self, 27)
    }
    #[doc = "Bit 28 - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn BR12(&mut self) -> BR12_W<'_, BSRR_SPEC> {
        BR12_W::new(self, 28)
    }
    #[doc = "Bit 29 - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn BR13(&mut self) -> BR13_W<'_, BSRR_SPEC> {
        BR13_W::new(self, 29)
    }
    #[doc = "Bit 30 - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn BR14(&mut self) -> BR14_W<'_, BSRR_SPEC> {
        BR14_W::new(self, 30)
    }
    #[doc = "Bit 31 - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn BR15(&mut self) -> BR15_W<'_, BSRR_SPEC> {
        BR15_W::new(self, 31)
    }
}
#[doc = "GPIO port bit set/reset register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsrr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BSRR_SPEC;
impl crate::RegisterSpec for BSRR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`bsrr::W`](W) writer structure"]
impl crate::Writable for BSRR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets BSRR to value 0"]
impl crate::Resettable for BSRR_SPEC {}
