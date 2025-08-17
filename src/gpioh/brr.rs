#[doc = "Register `BRR` writer"]
pub type W = crate::W<BRR_SPEC>;
#[doc = "Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR0_A {
    #[doc = "0: No action on the corresponding ODy bit"]
    B_0x0 = 0,
    #[doc = "1: Reset the corresponding ODy bit"]
    B_0x1 = 1,
}
impl From<BR0_A> for bool {
    #[inline(always)]
    fn from(variant: BR0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR0` writer - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
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
    #[doc = "Reset the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR0_A::B_0x1)
    }
}
#[doc = "Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR1_A {
    #[doc = "0: No action on the corresponding ODy bit"]
    B_0x0 = 0,
    #[doc = "1: Reset the corresponding ODy bit"]
    B_0x1 = 1,
}
impl From<BR1_A> for bool {
    #[inline(always)]
    fn from(variant: BR1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR1` writer - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
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
    #[doc = "Reset the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR1_A::B_0x1)
    }
}
#[doc = "Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR2_A {
    #[doc = "0: No action on the corresponding ODy bit"]
    B_0x0 = 0,
    #[doc = "1: Reset the corresponding ODy bit"]
    B_0x1 = 1,
}
impl From<BR2_A> for bool {
    #[inline(always)]
    fn from(variant: BR2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR2` writer - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
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
    #[doc = "Reset the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR2_A::B_0x1)
    }
}
#[doc = "Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR3_A {
    #[doc = "0: No action on the corresponding ODy bit"]
    B_0x0 = 0,
    #[doc = "1: Reset the corresponding ODy bit"]
    B_0x1 = 1,
}
impl From<BR3_A> for bool {
    #[inline(always)]
    fn from(variant: BR3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR3` writer - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
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
    #[doc = "Reset the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR3_A::B_0x1)
    }
}
#[doc = "Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR4_A {
    #[doc = "0: No action on the corresponding ODy bit"]
    B_0x0 = 0,
    #[doc = "1: Reset the corresponding ODy bit"]
    B_0x1 = 1,
}
impl From<BR4_A> for bool {
    #[inline(always)]
    fn from(variant: BR4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR4` writer - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
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
    #[doc = "Reset the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR4_A::B_0x1)
    }
}
#[doc = "Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR5_A {
    #[doc = "0: No action on the corresponding ODy bit"]
    B_0x0 = 0,
    #[doc = "1: Reset the corresponding ODy bit"]
    B_0x1 = 1,
}
impl From<BR5_A> for bool {
    #[inline(always)]
    fn from(variant: BR5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR5` writer - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
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
    #[doc = "Reset the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR5_A::B_0x1)
    }
}
#[doc = "Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR6_A {
    #[doc = "0: No action on the corresponding ODy bit"]
    B_0x0 = 0,
    #[doc = "1: Reset the corresponding ODy bit"]
    B_0x1 = 1,
}
impl From<BR6_A> for bool {
    #[inline(always)]
    fn from(variant: BR6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR6` writer - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
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
    #[doc = "Reset the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR6_A::B_0x1)
    }
}
#[doc = "Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR7_A {
    #[doc = "0: No action on the corresponding ODy bit"]
    B_0x0 = 0,
    #[doc = "1: Reset the corresponding ODy bit"]
    B_0x1 = 1,
}
impl From<BR7_A> for bool {
    #[inline(always)]
    fn from(variant: BR7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR7` writer - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
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
    #[doc = "Reset the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR7_A::B_0x1)
    }
}
#[doc = "Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR8_A {
    #[doc = "0: No action on the corresponding ODy bit"]
    B_0x0 = 0,
    #[doc = "1: Reset the corresponding ODy bit"]
    B_0x1 = 1,
}
impl From<BR8_A> for bool {
    #[inline(always)]
    fn from(variant: BR8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR8` writer - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
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
    #[doc = "Reset the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR8_A::B_0x1)
    }
}
#[doc = "Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR9_A {
    #[doc = "0: No action on the corresponding ODy bit"]
    B_0x0 = 0,
    #[doc = "1: Reset the corresponding ODy bit"]
    B_0x1 = 1,
}
impl From<BR9_A> for bool {
    #[inline(always)]
    fn from(variant: BR9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR9` writer - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
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
    #[doc = "Reset the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR9_A::B_0x1)
    }
}
#[doc = "Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR10_A {
    #[doc = "0: No action on the corresponding ODy bit"]
    B_0x0 = 0,
    #[doc = "1: Reset the corresponding ODy bit"]
    B_0x1 = 1,
}
impl From<BR10_A> for bool {
    #[inline(always)]
    fn from(variant: BR10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR10` writer - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
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
    #[doc = "Reset the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR10_A::B_0x1)
    }
}
#[doc = "Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR11_A {
    #[doc = "0: No action on the corresponding ODy bit"]
    B_0x0 = 0,
    #[doc = "1: Reset the corresponding ODy bit"]
    B_0x1 = 1,
}
impl From<BR11_A> for bool {
    #[inline(always)]
    fn from(variant: BR11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR11` writer - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
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
    #[doc = "Reset the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR11_A::B_0x1)
    }
}
#[doc = "Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR12_A {
    #[doc = "0: No action on the corresponding ODy bit"]
    B_0x0 = 0,
    #[doc = "1: Reset the corresponding ODy bit"]
    B_0x1 = 1,
}
impl From<BR12_A> for bool {
    #[inline(always)]
    fn from(variant: BR12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR12` writer - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
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
    #[doc = "Reset the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR12_A::B_0x1)
    }
}
#[doc = "Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR13_A {
    #[doc = "0: No action on the corresponding ODy bit"]
    B_0x0 = 0,
    #[doc = "1: Reset the corresponding ODy bit"]
    B_0x1 = 1,
}
impl From<BR13_A> for bool {
    #[inline(always)]
    fn from(variant: BR13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR13` writer - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
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
    #[doc = "Reset the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR13_A::B_0x1)
    }
}
#[doc = "Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR14_A {
    #[doc = "0: No action on the corresponding ODy bit"]
    B_0x0 = 0,
    #[doc = "1: Reset the corresponding ODy bit"]
    B_0x1 = 1,
}
impl From<BR14_A> for bool {
    #[inline(always)]
    fn from(variant: BR14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR14` writer - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
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
    #[doc = "Reset the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR14_A::B_0x1)
    }
}
#[doc = "Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR15_A {
    #[doc = "0: No action on the corresponding ODy bit"]
    B_0x0 = 0,
    #[doc = "1: Reset the corresponding ODy bit"]
    B_0x1 = 1,
}
impl From<BR15_A> for bool {
    #[inline(always)]
    fn from(variant: BR15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR15` writer - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
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
    #[doc = "Reset the corresponding ODy bit"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR15_A::B_0x1)
    }
}
impl W {
    #[doc = "Bit 0 - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn BR0(&mut self) -> BR0_W<'_, BRR_SPEC> {
        BR0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn BR1(&mut self) -> BR1_W<'_, BRR_SPEC> {
        BR1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn BR2(&mut self) -> BR2_W<'_, BRR_SPEC> {
        BR2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn BR3(&mut self) -> BR3_W<'_, BRR_SPEC> {
        BR3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn BR4(&mut self) -> BR4_W<'_, BRR_SPEC> {
        BR4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn BR5(&mut self) -> BR5_W<'_, BRR_SPEC> {
        BR5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn BR6(&mut self) -> BR6_W<'_, BRR_SPEC> {
        BR6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn BR7(&mut self) -> BR7_W<'_, BRR_SPEC> {
        BR7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn BR8(&mut self) -> BR8_W<'_, BRR_SPEC> {
        BR8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn BR9(&mut self) -> BR9_W<'_, BRR_SPEC> {
        BR9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn BR10(&mut self) -> BR10_W<'_, BRR_SPEC> {
        BR10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn BR11(&mut self) -> BR11_W<'_, BRR_SPEC> {
        BR11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn BR12(&mut self) -> BR12_W<'_, BRR_SPEC> {
        BR12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn BR13(&mut self) -> BR13_W<'_, BRR_SPEC> {
        BR13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn BR14(&mut self) -> BR14_W<'_, BRR_SPEC> {
        BR14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Port x reset IO pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package."]
    #[inline(always)]
    pub fn BR15(&mut self) -> BR15_W<'_, BRR_SPEC> {
        BR15_W::new(self, 15)
    }
}
#[doc = "GPIO port bit reset register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BRR_SPEC;
impl crate::RegisterSpec for BRR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`brr::W`](W) writer structure"]
impl crate::Writable for BRR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets BRR to value 0"]
impl crate::Resettable for BRR_SPEC {}
