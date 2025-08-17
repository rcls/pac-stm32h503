#[doc = "Register `CR` reader"]
pub type R = crate::R<CR_SPEC>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CR_SPEC>;
#[doc = "Allows debug in Stop mode All clocks are disabled automatically in Stop mode. All active clocks and oscillators continue to run during Stop mode, allowing full debug capability. On exit from Stop mode, the clock settings are set to the Stop mode exit state.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_STOP_A {
    #[doc = "0: normal operation"]
    B_0x0 = 0,
    #[doc = "1: automatic clock stop disabled"]
    B_0x1 = 1,
}
impl From<DBG_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_STOP` reader - Allows debug in Stop mode All clocks are disabled automatically in Stop mode. All active clocks and oscillators continue to run during Stop mode, allowing full debug capability. On exit from Stop mode, the clock settings are set to the Stop mode exit state."]
pub type DBG_STOP_R = crate::BitReader<DBG_STOP_A>;
impl DBG_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_STOP_A {
        match self.bits {
            false => DBG_STOP_A::B_0x0,
            true => DBG_STOP_A::B_0x1,
        }
    }
    #[doc = "normal operation"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DBG_STOP_A::B_0x0
    }
    #[doc = "automatic clock stop disabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DBG_STOP_A::B_0x1
    }
}
#[doc = "Field `DBG_STOP` writer - Allows debug in Stop mode All clocks are disabled automatically in Stop mode. All active clocks and oscillators continue to run during Stop mode, allowing full debug capability. On exit from Stop mode, the clock settings are set to the Stop mode exit state."]
pub type DBG_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_STOP_A>;
impl<'a, REG> DBG_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_STOP_A::B_0x0)
    }
    #[doc = "automatic clock stop disabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_STOP_A::B_0x1)
    }
}
#[doc = "Allows debug in Standby mode All clocks are disabled and the core powered down automatically in Standby mode. All active clocks and oscillators continue to run during Standby mode, and the core supply is maintained, allowing full debug capability. On exit from Standby mode, a system reset is performed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_STANDBY_A {
    #[doc = "0: normal operation"]
    B_0x0 = 0,
    #[doc = "1: automatic clock stop/power down disabled"]
    B_0x1 = 1,
}
impl From<DBG_STANDBY_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_STANDBY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_STANDBY` reader - Allows debug in Standby mode All clocks are disabled and the core powered down automatically in Standby mode. All active clocks and oscillators continue to run during Standby mode, and the core supply is maintained, allowing full debug capability. On exit from Standby mode, a system reset is performed."]
pub type DBG_STANDBY_R = crate::BitReader<DBG_STANDBY_A>;
impl DBG_STANDBY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_STANDBY_A {
        match self.bits {
            false => DBG_STANDBY_A::B_0x0,
            true => DBG_STANDBY_A::B_0x1,
        }
    }
    #[doc = "normal operation"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DBG_STANDBY_A::B_0x0
    }
    #[doc = "automatic clock stop/power down disabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DBG_STANDBY_A::B_0x1
    }
}
#[doc = "Field `DBG_STANDBY` writer - Allows debug in Standby mode All clocks are disabled and the core powered down automatically in Standby mode. All active clocks and oscillators continue to run during Standby mode, and the core supply is maintained, allowing full debug capability. On exit from Standby mode, a system reset is performed."]
pub type DBG_STANDBY_W<'a, REG> = crate::BitWriter<'a, REG, DBG_STANDBY_A>;
impl<'a, REG> DBG_STANDBY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_STANDBY_A::B_0x0)
    }
    #[doc = "automatic clock stop/power down disabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_STANDBY_A::B_0x1)
    }
}
#[doc = "trace pin enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRACE_IOEN_A {
    #[doc = "0: disabled - trace pins not assigned"]
    B_0x0 = 0,
    #[doc = "1: enabled - trace pins assigned according to the value of TRACE_MODE field"]
    B_0x1 = 1,
}
impl From<TRACE_IOEN_A> for bool {
    #[inline(always)]
    fn from(variant: TRACE_IOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRACE_IOEN` reader - trace pin enable"]
pub type TRACE_IOEN_R = crate::BitReader<TRACE_IOEN_A>;
impl TRACE_IOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TRACE_IOEN_A {
        match self.bits {
            false => TRACE_IOEN_A::B_0x0,
            true => TRACE_IOEN_A::B_0x1,
        }
    }
    #[doc = "disabled - trace pins not assigned"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TRACE_IOEN_A::B_0x0
    }
    #[doc = "enabled - trace pins assigned according to the value of TRACE_MODE field"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TRACE_IOEN_A::B_0x1
    }
}
#[doc = "Field `TRACE_IOEN` writer - trace pin enable"]
pub type TRACE_IOEN_W<'a, REG> = crate::BitWriter<'a, REG, TRACE_IOEN_A>;
impl<'a, REG> TRACE_IOEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disabled - trace pins not assigned"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TRACE_IOEN_A::B_0x0)
    }
    #[doc = "enabled - trace pins assigned according to the value of TRACE_MODE field"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TRACE_IOEN_A::B_0x1)
    }
}
#[doc = "trace port and clock enable. This bit enables the trace port clock, TRACECK.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRACE_EN_A {
    #[doc = "0: disabled"]
    B_0x0 = 0,
    #[doc = "1: enabled"]
    B_0x1 = 1,
}
impl From<TRACE_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TRACE_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRACE_EN` reader - trace port and clock enable. This bit enables the trace port clock, TRACECK."]
pub type TRACE_EN_R = crate::BitReader<TRACE_EN_A>;
impl TRACE_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TRACE_EN_A {
        match self.bits {
            false => TRACE_EN_A::B_0x0,
            true => TRACE_EN_A::B_0x1,
        }
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TRACE_EN_A::B_0x0
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TRACE_EN_A::B_0x1
    }
}
#[doc = "Field `TRACE_EN` writer - trace port and clock enable. This bit enables the trace port clock, TRACECK."]
pub type TRACE_EN_W<'a, REG> = crate::BitWriter<'a, REG, TRACE_EN_A>;
impl<'a, REG> TRACE_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TRACE_EN_A::B_0x0)
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TRACE_EN_A::B_0x1)
    }
}
#[doc = "trace pin assignment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRACE_MODE_A {
    #[doc = "0: trace pins assigned for asynchronous mode (TRACESWO)"]
    B_0x0 = 0,
    #[doc = "1: trace pins assigned for synchronous mode with a port width of 1 (TRACECK, TRACED0)"]
    B_0x1 = 1,
    #[doc = "2: trace pins assigned for synchronous mode with a port width of 2 ((TRACECK, TRACED0-1)"]
    B_0x2 = 2,
    #[doc = "3: trace pins assigned for synchronous mode with a port width of 4 ((TRACECK, TRACED0-3)"]
    B_0x3 = 3,
}
impl From<TRACE_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: TRACE_MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TRACE_MODE_A {
    type Ux = u8;
}
impl crate::IsEnum for TRACE_MODE_A {}
#[doc = "Field `TRACE_MODE` reader - trace pin assignment"]
pub type TRACE_MODE_R = crate::FieldReader<TRACE_MODE_A>;
impl TRACE_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TRACE_MODE_A {
        match self.bits {
            0 => TRACE_MODE_A::B_0x0,
            1 => TRACE_MODE_A::B_0x1,
            2 => TRACE_MODE_A::B_0x2,
            3 => TRACE_MODE_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "trace pins assigned for asynchronous mode (TRACESWO)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TRACE_MODE_A::B_0x0
    }
    #[doc = "trace pins assigned for synchronous mode with a port width of 1 (TRACECK, TRACED0)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TRACE_MODE_A::B_0x1
    }
    #[doc = "trace pins assigned for synchronous mode with a port width of 2 ((TRACECK, TRACED0-1)"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == TRACE_MODE_A::B_0x2
    }
    #[doc = "trace pins assigned for synchronous mode with a port width of 4 ((TRACECK, TRACED0-3)"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == TRACE_MODE_A::B_0x3
    }
}
#[doc = "Field `TRACE_MODE` writer - trace pin assignment"]
pub type TRACE_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TRACE_MODE_A, crate::Safe>;
impl<'a, REG> TRACE_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "trace pins assigned for asynchronous mode (TRACESWO)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TRACE_MODE_A::B_0x0)
    }
    #[doc = "trace pins assigned for synchronous mode with a port width of 1 (TRACECK, TRACED0)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TRACE_MODE_A::B_0x1)
    }
    #[doc = "trace pins assigned for synchronous mode with a port width of 2 ((TRACECK, TRACED0-1)"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(TRACE_MODE_A::B_0x2)
    }
    #[doc = "trace pins assigned for synchronous mode with a port width of 4 ((TRACECK, TRACED0-3)"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(TRACE_MODE_A::B_0x3)
    }
}
#[doc = "Debug credentials reset type This bit selects which type of reset is used to revoke the debug authentication credentials\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCRT_A {
    #[doc = "0: System reset"]
    B_0x0 = 0,
    #[doc = "1: Power reset"]
    B_0x1 = 1,
}
impl From<DCRT_A> for bool {
    #[inline(always)]
    fn from(variant: DCRT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCRT` reader - Debug credentials reset type This bit selects which type of reset is used to revoke the debug authentication credentials"]
pub type DCRT_R = crate::BitReader<DCRT_A>;
impl DCRT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DCRT_A {
        match self.bits {
            false => DCRT_A::B_0x0,
            true => DCRT_A::B_0x1,
        }
    }
    #[doc = "System reset"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DCRT_A::B_0x0
    }
    #[doc = "Power reset"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DCRT_A::B_0x1
    }
}
#[doc = "Field `DCRT` writer - Debug credentials reset type This bit selects which type of reset is used to revoke the debug authentication credentials"]
pub type DCRT_W<'a, REG> = crate::BitWriter<'a, REG, DCRT_A>;
impl<'a, REG> DCRT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "System reset"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DCRT_A::B_0x0)
    }
    #[doc = "Power reset"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DCRT_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 1 - Allows debug in Stop mode All clocks are disabled automatically in Stop mode. All active clocks and oscillators continue to run during Stop mode, allowing full debug capability. On exit from Stop mode, the clock settings are set to the Stop mode exit state."]
    #[inline(always)]
    pub fn DBG_STOP(&self) -> DBG_STOP_R {
        DBG_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Allows debug in Standby mode All clocks are disabled and the core powered down automatically in Standby mode. All active clocks and oscillators continue to run during Standby mode, and the core supply is maintained, allowing full debug capability. On exit from Standby mode, a system reset is performed."]
    #[inline(always)]
    pub fn DBG_STANDBY(&self) -> DBG_STANDBY_R {
        DBG_STANDBY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - trace pin enable"]
    #[inline(always)]
    pub fn TRACE_IOEN(&self) -> TRACE_IOEN_R {
        TRACE_IOEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - trace port and clock enable. This bit enables the trace port clock, TRACECK."]
    #[inline(always)]
    pub fn TRACE_EN(&self) -> TRACE_EN_R {
        TRACE_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - trace pin assignment"]
    #[inline(always)]
    pub fn TRACE_MODE(&self) -> TRACE_MODE_R {
        TRACE_MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 16 - Debug credentials reset type This bit selects which type of reset is used to revoke the debug authentication credentials"]
    #[inline(always)]
    pub fn DCRT(&self) -> DCRT_R {
        DCRT_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Allows debug in Stop mode All clocks are disabled automatically in Stop mode. All active clocks and oscillators continue to run during Stop mode, allowing full debug capability. On exit from Stop mode, the clock settings are set to the Stop mode exit state."]
    #[inline(always)]
    pub fn DBG_STOP(&mut self) -> DBG_STOP_W<'_, CR_SPEC> {
        DBG_STOP_W::new(self, 1)
    }
    #[doc = "Bit 2 - Allows debug in Standby mode All clocks are disabled and the core powered down automatically in Standby mode. All active clocks and oscillators continue to run during Standby mode, and the core supply is maintained, allowing full debug capability. On exit from Standby mode, a system reset is performed."]
    #[inline(always)]
    pub fn DBG_STANDBY(&mut self) -> DBG_STANDBY_W<'_, CR_SPEC> {
        DBG_STANDBY_W::new(self, 2)
    }
    #[doc = "Bit 4 - trace pin enable"]
    #[inline(always)]
    pub fn TRACE_IOEN(&mut self) -> TRACE_IOEN_W<'_, CR_SPEC> {
        TRACE_IOEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - trace port and clock enable. This bit enables the trace port clock, TRACECK."]
    #[inline(always)]
    pub fn TRACE_EN(&mut self) -> TRACE_EN_W<'_, CR_SPEC> {
        TRACE_EN_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - trace pin assignment"]
    #[inline(always)]
    pub fn TRACE_MODE(&mut self) -> TRACE_MODE_W<'_, CR_SPEC> {
        TRACE_MODE_W::new(self, 6)
    }
    #[doc = "Bit 16 - Debug credentials reset type This bit selects which type of reset is used to revoke the debug authentication credentials"]
    #[inline(always)]
    pub fn DCRT(&mut self) -> DCRT_W<'_, CR_SPEC> {
        DCRT_W::new(self, 16)
    }
}
#[doc = "DBGMCU configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {}
