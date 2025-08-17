#[doc = "Register `APB1LFZR` reader"]
pub type R = crate::R<APB1LFZR_SPEC>;
#[doc = "Register `APB1LFZR` writer"]
pub type W = crate::W<APB1LFZR_SPEC>;
#[doc = "TIM2 stop in debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_TIM2_STOP_A {
    #[doc = "0: normal operation. TIM2 continues to operate while CPU is in debug mode."]
    B_0x0 = 0,
    #[doc = "1: stop in debug. TIM2 is frozen while CPU is in debug mode."]
    B_0x1 = 1,
}
impl From<DBG_TIM2_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_TIM2_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_TIM2_STOP` reader - TIM2 stop in debug"]
pub type DBG_TIM2_STOP_R = crate::BitReader<DBG_TIM2_STOP_A>;
impl DBG_TIM2_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_TIM2_STOP_A {
        match self.bits {
            false => DBG_TIM2_STOP_A::B_0x0,
            true => DBG_TIM2_STOP_A::B_0x1,
        }
    }
    #[doc = "normal operation. TIM2 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DBG_TIM2_STOP_A::B_0x0
    }
    #[doc = "stop in debug. TIM2 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DBG_TIM2_STOP_A::B_0x1
    }
}
#[doc = "Field `DBG_TIM2_STOP` writer - TIM2 stop in debug"]
pub type DBG_TIM2_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_TIM2_STOP_A>;
impl<'a, REG> DBG_TIM2_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation. TIM2 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_TIM2_STOP_A::B_0x0)
    }
    #[doc = "stop in debug. TIM2 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_TIM2_STOP_A::B_0x1)
    }
}
#[doc = "TIM3 stop in debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_TIM3_STOP_A {
    #[doc = "0: normal operation. TIM3 continues to operate while CPU is in debug mode."]
    B_0x0 = 0,
    #[doc = "1: stop in debug. TIM3 is frozen while CPU is in debug mode."]
    B_0x1 = 1,
}
impl From<DBG_TIM3_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_TIM3_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_TIM3_STOP` reader - TIM3 stop in debug"]
pub type DBG_TIM3_STOP_R = crate::BitReader<DBG_TIM3_STOP_A>;
impl DBG_TIM3_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_TIM3_STOP_A {
        match self.bits {
            false => DBG_TIM3_STOP_A::B_0x0,
            true => DBG_TIM3_STOP_A::B_0x1,
        }
    }
    #[doc = "normal operation. TIM3 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DBG_TIM3_STOP_A::B_0x0
    }
    #[doc = "stop in debug. TIM3 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DBG_TIM3_STOP_A::B_0x1
    }
}
#[doc = "Field `DBG_TIM3_STOP` writer - TIM3 stop in debug"]
pub type DBG_TIM3_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_TIM3_STOP_A>;
impl<'a, REG> DBG_TIM3_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation. TIM3 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_TIM3_STOP_A::B_0x0)
    }
    #[doc = "stop in debug. TIM3 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_TIM3_STOP_A::B_0x1)
    }
}
#[doc = "TIM6 stop in debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_TIM6_STOP_A {
    #[doc = "0: normal operation. TIM6 continues to operate while CPU is in debug mode."]
    B_0x0 = 0,
    #[doc = "1: stop in debug. TIM6 is frozen while CPU is in debug mode."]
    B_0x1 = 1,
}
impl From<DBG_TIM6_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_TIM6_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_TIM6_STOP` reader - TIM6 stop in debug"]
pub type DBG_TIM6_STOP_R = crate::BitReader<DBG_TIM6_STOP_A>;
impl DBG_TIM6_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_TIM6_STOP_A {
        match self.bits {
            false => DBG_TIM6_STOP_A::B_0x0,
            true => DBG_TIM6_STOP_A::B_0x1,
        }
    }
    #[doc = "normal operation. TIM6 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DBG_TIM6_STOP_A::B_0x0
    }
    #[doc = "stop in debug. TIM6 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DBG_TIM6_STOP_A::B_0x1
    }
}
#[doc = "Field `DBG_TIM6_STOP` writer - TIM6 stop in debug"]
pub type DBG_TIM6_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_TIM6_STOP_A>;
impl<'a, REG> DBG_TIM6_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation. TIM6 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_TIM6_STOP_A::B_0x0)
    }
    #[doc = "stop in debug. TIM6 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_TIM6_STOP_A::B_0x1)
    }
}
#[doc = "TIM7 stop in debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_TIM7_STOP_A {
    #[doc = "0: normal operation. TIM7 continues to operate while CPU is in debug mode."]
    B_0x0 = 0,
    #[doc = "1: stop in debug. TIM7 is frozen while CPU is in debug mode."]
    B_0x1 = 1,
}
impl From<DBG_TIM7_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_TIM7_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_TIM7_STOP` reader - TIM7 stop in debug"]
pub type DBG_TIM7_STOP_R = crate::BitReader<DBG_TIM7_STOP_A>;
impl DBG_TIM7_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_TIM7_STOP_A {
        match self.bits {
            false => DBG_TIM7_STOP_A::B_0x0,
            true => DBG_TIM7_STOP_A::B_0x1,
        }
    }
    #[doc = "normal operation. TIM7 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DBG_TIM7_STOP_A::B_0x0
    }
    #[doc = "stop in debug. TIM7 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DBG_TIM7_STOP_A::B_0x1
    }
}
#[doc = "Field `DBG_TIM7_STOP` writer - TIM7 stop in debug"]
pub type DBG_TIM7_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_TIM7_STOP_A>;
impl<'a, REG> DBG_TIM7_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation. TIM7 continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_TIM7_STOP_A::B_0x0)
    }
    #[doc = "stop in debug. TIM7 is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_TIM7_STOP_A::B_0x1)
    }
}
#[doc = "WWDG stop in debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_WWDG_STOP_A {
    #[doc = "0: normal operation. WWDG continues to operate while CPU is in debug mode."]
    B_0x0 = 0,
    #[doc = "1: stop in debug. WWDG is frozen while CPU is in debug mode."]
    B_0x1 = 1,
}
impl From<DBG_WWDG_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_WWDG_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_WWDG_STOP` reader - WWDG stop in debug"]
pub type DBG_WWDG_STOP_R = crate::BitReader<DBG_WWDG_STOP_A>;
impl DBG_WWDG_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_WWDG_STOP_A {
        match self.bits {
            false => DBG_WWDG_STOP_A::B_0x0,
            true => DBG_WWDG_STOP_A::B_0x1,
        }
    }
    #[doc = "normal operation. WWDG continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DBG_WWDG_STOP_A::B_0x0
    }
    #[doc = "stop in debug. WWDG is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DBG_WWDG_STOP_A::B_0x1
    }
}
#[doc = "Field `DBG_WWDG_STOP` writer - WWDG stop in debug"]
pub type DBG_WWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_WWDG_STOP_A>;
impl<'a, REG> DBG_WWDG_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation. WWDG continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_WWDG_STOP_A::B_0x0)
    }
    #[doc = "stop in debug. WWDG is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_WWDG_STOP_A::B_0x1)
    }
}
#[doc = "IWDG stop in debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_IWDG_STOP_A {
    #[doc = "0: normal operation. IWDG continues to operate while CPU is in debug mode."]
    B_0x0 = 0,
    #[doc = "1: stop in debug. IWDG is frozen while CPU is in debug mode."]
    B_0x1 = 1,
}
impl From<DBG_IWDG_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_IWDG_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_IWDG_STOP` reader - IWDG stop in debug"]
pub type DBG_IWDG_STOP_R = crate::BitReader<DBG_IWDG_STOP_A>;
impl DBG_IWDG_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_IWDG_STOP_A {
        match self.bits {
            false => DBG_IWDG_STOP_A::B_0x0,
            true => DBG_IWDG_STOP_A::B_0x1,
        }
    }
    #[doc = "normal operation. IWDG continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DBG_IWDG_STOP_A::B_0x0
    }
    #[doc = "stop in debug. IWDG is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DBG_IWDG_STOP_A::B_0x1
    }
}
#[doc = "Field `DBG_IWDG_STOP` writer - IWDG stop in debug"]
pub type DBG_IWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_IWDG_STOP_A>;
impl<'a, REG> DBG_IWDG_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation. IWDG continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_IWDG_STOP_A::B_0x0)
    }
    #[doc = "stop in debug. IWDG is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_IWDG_STOP_A::B_0x1)
    }
}
#[doc = "I2C1 SMBUS timeout stop in debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_I2C1_STOP_A {
    #[doc = "0: normal operation. I2C1 SMBUS timeout continues to operate while CPU is in debug mode."]
    B_0x0 = 0,
    #[doc = "1: stop in debug. I2C1 SMBUS timeout is frozen while CPU is in debug mode."]
    B_0x1 = 1,
}
impl From<DBG_I2C1_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_I2C1_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_I2C1_STOP` reader - I2C1 SMBUS timeout stop in debug"]
pub type DBG_I2C1_STOP_R = crate::BitReader<DBG_I2C1_STOP_A>;
impl DBG_I2C1_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_I2C1_STOP_A {
        match self.bits {
            false => DBG_I2C1_STOP_A::B_0x0,
            true => DBG_I2C1_STOP_A::B_0x1,
        }
    }
    #[doc = "normal operation. I2C1 SMBUS timeout continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DBG_I2C1_STOP_A::B_0x0
    }
    #[doc = "stop in debug. I2C1 SMBUS timeout is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DBG_I2C1_STOP_A::B_0x1
    }
}
#[doc = "Field `DBG_I2C1_STOP` writer - I2C1 SMBUS timeout stop in debug"]
pub type DBG_I2C1_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_I2C1_STOP_A>;
impl<'a, REG> DBG_I2C1_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation. I2C1 SMBUS timeout continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_I2C1_STOP_A::B_0x0)
    }
    #[doc = "stop in debug. I2C1 SMBUS timeout is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_I2C1_STOP_A::B_0x1)
    }
}
#[doc = "I2C2 SMBUS timeout stop in debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_I2C2_STOP_A {
    #[doc = "0: normal operation. I2C2 SMBUS timeout continues to operate while CPU is in debug mode."]
    B_0x0 = 0,
    #[doc = "1: stop in debug. I2C2 SMBUS timeout is frozen while CPU is in debug mode."]
    B_0x1 = 1,
}
impl From<DBG_I2C2_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_I2C2_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_I2C2_STOP` reader - I2C2 SMBUS timeout stop in debug"]
pub type DBG_I2C2_STOP_R = crate::BitReader<DBG_I2C2_STOP_A>;
impl DBG_I2C2_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_I2C2_STOP_A {
        match self.bits {
            false => DBG_I2C2_STOP_A::B_0x0,
            true => DBG_I2C2_STOP_A::B_0x1,
        }
    }
    #[doc = "normal operation. I2C2 SMBUS timeout continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DBG_I2C2_STOP_A::B_0x0
    }
    #[doc = "stop in debug. I2C2 SMBUS timeout is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DBG_I2C2_STOP_A::B_0x1
    }
}
#[doc = "Field `DBG_I2C2_STOP` writer - I2C2 SMBUS timeout stop in debug"]
pub type DBG_I2C2_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_I2C2_STOP_A>;
impl<'a, REG> DBG_I2C2_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation. I2C2 SMBUS timeout continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_I2C2_STOP_A::B_0x0)
    }
    #[doc = "stop in debug. I2C2 SMBUS timeout is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_I2C2_STOP_A::B_0x1)
    }
}
#[doc = "I3C1 SCL stall counter stop in debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_I3C1_STOP_A {
    #[doc = "0: normal operation. I3C1 SCL stall timeout counter continues to operate while CPU is in debug mode."]
    B_0x0 = 0,
    #[doc = "1: stop in debug. I3C1 SCL stall timeout counter is frozen while CPU is in debug mode."]
    B_0x1 = 1,
}
impl From<DBG_I3C1_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_I3C1_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_I3C1_STOP` reader - I3C1 SCL stall counter stop in debug"]
pub type DBG_I3C1_STOP_R = crate::BitReader<DBG_I3C1_STOP_A>;
impl DBG_I3C1_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_I3C1_STOP_A {
        match self.bits {
            false => DBG_I3C1_STOP_A::B_0x0,
            true => DBG_I3C1_STOP_A::B_0x1,
        }
    }
    #[doc = "normal operation. I3C1 SCL stall timeout counter continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DBG_I3C1_STOP_A::B_0x0
    }
    #[doc = "stop in debug. I3C1 SCL stall timeout counter is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DBG_I3C1_STOP_A::B_0x1
    }
}
#[doc = "Field `DBG_I3C1_STOP` writer - I3C1 SCL stall counter stop in debug"]
pub type DBG_I3C1_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_I3C1_STOP_A>;
impl<'a, REG> DBG_I3C1_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation. I3C1 SCL stall timeout counter continues to operate while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_I3C1_STOP_A::B_0x0)
    }
    #[doc = "stop in debug. I3C1 SCL stall timeout counter is frozen while CPU is in debug mode."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_I3C1_STOP_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - TIM2 stop in debug"]
    #[inline(always)]
    pub fn DBG_TIM2_STOP(&self) -> DBG_TIM2_STOP_R {
        DBG_TIM2_STOP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 stop in debug"]
    #[inline(always)]
    pub fn DBG_TIM3_STOP(&self) -> DBG_TIM3_STOP_R {
        DBG_TIM3_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 stop in debug"]
    #[inline(always)]
    pub fn DBG_TIM6_STOP(&self) -> DBG_TIM6_STOP_R {
        DBG_TIM6_STOP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 stop in debug"]
    #[inline(always)]
    pub fn DBG_TIM7_STOP(&self) -> DBG_TIM7_STOP_R {
        DBG_TIM7_STOP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 11 - WWDG stop in debug"]
    #[inline(always)]
    pub fn DBG_WWDG_STOP(&self) -> DBG_WWDG_STOP_R {
        DBG_WWDG_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - IWDG stop in debug"]
    #[inline(always)]
    pub fn DBG_IWDG_STOP(&self) -> DBG_IWDG_STOP_R {
        DBG_IWDG_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 SMBUS timeout stop in debug"]
    #[inline(always)]
    pub fn DBG_I2C1_STOP(&self) -> DBG_I2C1_STOP_R {
        DBG_I2C1_STOP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 SMBUS timeout stop in debug"]
    #[inline(always)]
    pub fn DBG_I2C2_STOP(&self) -> DBG_I2C2_STOP_R {
        DBG_I2C2_STOP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I3C1 SCL stall counter stop in debug"]
    #[inline(always)]
    pub fn DBG_I3C1_STOP(&self) -> DBG_I3C1_STOP_R {
        DBG_I3C1_STOP_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 stop in debug"]
    #[inline(always)]
    pub fn DBG_TIM2_STOP(&mut self) -> DBG_TIM2_STOP_W<'_, APB1LFZR_SPEC> {
        DBG_TIM2_STOP_W::new(self, 0)
    }
    #[doc = "Bit 1 - TIM3 stop in debug"]
    #[inline(always)]
    pub fn DBG_TIM3_STOP(&mut self) -> DBG_TIM3_STOP_W<'_, APB1LFZR_SPEC> {
        DBG_TIM3_STOP_W::new(self, 1)
    }
    #[doc = "Bit 4 - TIM6 stop in debug"]
    #[inline(always)]
    pub fn DBG_TIM6_STOP(&mut self) -> DBG_TIM6_STOP_W<'_, APB1LFZR_SPEC> {
        DBG_TIM6_STOP_W::new(self, 4)
    }
    #[doc = "Bit 5 - TIM7 stop in debug"]
    #[inline(always)]
    pub fn DBG_TIM7_STOP(&mut self) -> DBG_TIM7_STOP_W<'_, APB1LFZR_SPEC> {
        DBG_TIM7_STOP_W::new(self, 5)
    }
    #[doc = "Bit 11 - WWDG stop in debug"]
    #[inline(always)]
    pub fn DBG_WWDG_STOP(&mut self) -> DBG_WWDG_STOP_W<'_, APB1LFZR_SPEC> {
        DBG_WWDG_STOP_W::new(self, 11)
    }
    #[doc = "Bit 12 - IWDG stop in debug"]
    #[inline(always)]
    pub fn DBG_IWDG_STOP(&mut self) -> DBG_IWDG_STOP_W<'_, APB1LFZR_SPEC> {
        DBG_IWDG_STOP_W::new(self, 12)
    }
    #[doc = "Bit 21 - I2C1 SMBUS timeout stop in debug"]
    #[inline(always)]
    pub fn DBG_I2C1_STOP(&mut self) -> DBG_I2C1_STOP_W<'_, APB1LFZR_SPEC> {
        DBG_I2C1_STOP_W::new(self, 21)
    }
    #[doc = "Bit 22 - I2C2 SMBUS timeout stop in debug"]
    #[inline(always)]
    pub fn DBG_I2C2_STOP(&mut self) -> DBG_I2C2_STOP_W<'_, APB1LFZR_SPEC> {
        DBG_I2C2_STOP_W::new(self, 22)
    }
    #[doc = "Bit 23 - I3C1 SCL stall counter stop in debug"]
    #[inline(always)]
    pub fn DBG_I3C1_STOP(&mut self) -> DBG_I3C1_STOP_W<'_, APB1LFZR_SPEC> {
        DBG_I3C1_STOP_W::new(self, 23)
    }
}
#[doc = "DBGMCU APB1L peripheral freeze register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1lfzr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1lfzr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1LFZR_SPEC;
impl crate::RegisterSpec for APB1LFZR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1lfzr::R`](R) reader structure"]
impl crate::Readable for APB1LFZR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb1lfzr::W`](W) writer structure"]
impl crate::Writable for APB1LFZR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets APB1LFZR to value 0"]
impl crate::Resettable for APB1LFZR_SPEC {}
