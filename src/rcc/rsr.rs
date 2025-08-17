#[doc = "Register `RSR` reader"]
pub type R = crate::R<RSR_SPEC>;
#[doc = "Register `RSR` writer"]
pub type W = crate::W<RSR_SPEC>;
#[doc = "remove reset flag Set and reset by software to reset the value of the reset flags.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMVF_A {
    #[doc = "0: reset of the reset flags not activated (default after power-on reset)"]
    B_0x0 = 0,
    #[doc = "1: resets the value of the reset flags"]
    B_0x1 = 1,
}
impl From<RMVF_A> for bool {
    #[inline(always)]
    fn from(variant: RMVF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RMVF` reader - remove reset flag Set and reset by software to reset the value of the reset flags."]
pub type RMVF_R = crate::BitReader<RMVF_A>;
impl RMVF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RMVF_A {
        match self.bits {
            false => RMVF_A::B_0x0,
            true => RMVF_A::B_0x1,
        }
    }
    #[doc = "reset of the reset flags not activated (default after power-on reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RMVF_A::B_0x0
    }
    #[doc = "resets the value of the reset flags"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RMVF_A::B_0x1
    }
}
#[doc = "Field `RMVF` writer - remove reset flag Set and reset by software to reset the value of the reset flags."]
pub type RMVF_W<'a, REG> = crate::BitWriter<'a, REG, RMVF_A>;
impl<'a, REG> RMVF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "reset of the reset flags not activated (default after power-on reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RMVF_A::B_0x0)
    }
    #[doc = "resets the value of the reset flags"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RMVF_A::B_0x1)
    }
}
#[doc = "pin reset flag (NRST) Reset by software by writing the RMVF bit. Set by hardware when a reset from pin occurs.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PINRSTF_A {
    #[doc = "0: no reset from pin occurred"]
    B_0x0 = 0,
    #[doc = "1: reset from pin occurred (default after power-on reset)"]
    B_0x1 = 1,
}
impl From<PINRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: PINRSTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PINRSTF` reader - pin reset flag (NRST) Reset by software by writing the RMVF bit. Set by hardware when a reset from pin occurs."]
pub type PINRSTF_R = crate::BitReader<PINRSTF_A>;
impl PINRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PINRSTF_A {
        match self.bits {
            false => PINRSTF_A::B_0x0,
            true => PINRSTF_A::B_0x1,
        }
    }
    #[doc = "no reset from pin occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PINRSTF_A::B_0x0
    }
    #[doc = "reset from pin occurred (default after power-on reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PINRSTF_A::B_0x1
    }
}
#[doc = "Field `PINRSTF` writer - pin reset flag (NRST) Reset by software by writing the RMVF bit. Set by hardware when a reset from pin occurs."]
pub type PINRSTF_W<'a, REG> = crate::BitWriter<'a, REG, PINRSTF_A>;
impl<'a, REG> PINRSTF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no reset from pin occurred"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PINRSTF_A::B_0x0)
    }
    #[doc = "reset from pin occurred (default after power-on reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PINRSTF_A::B_0x1)
    }
}
#[doc = "BOR reset flag Reset by software by writing the RMVF bit. Set by hardware when a BOR reset occurs (pwr_bor_rst).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BORRSTF_A {
    #[doc = "0: no BOR reset occurred"]
    B_0x0 = 0,
    #[doc = "1: BOR reset occurred (default after power-on reset)"]
    B_0x1 = 1,
}
impl From<BORRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: BORRSTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BORRSTF` reader - BOR reset flag Reset by software by writing the RMVF bit. Set by hardware when a BOR reset occurs (pwr_bor_rst)."]
pub type BORRSTF_R = crate::BitReader<BORRSTF_A>;
impl BORRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BORRSTF_A {
        match self.bits {
            false => BORRSTF_A::B_0x0,
            true => BORRSTF_A::B_0x1,
        }
    }
    #[doc = "no BOR reset occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BORRSTF_A::B_0x0
    }
    #[doc = "BOR reset occurred (default after power-on reset)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BORRSTF_A::B_0x1
    }
}
#[doc = "Field `BORRSTF` writer - BOR reset flag Reset by software by writing the RMVF bit. Set by hardware when a BOR reset occurs (pwr_bor_rst)."]
pub type BORRSTF_W<'a, REG> = crate::BitWriter<'a, REG, BORRSTF_A>;
impl<'a, REG> BORRSTF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no BOR reset occurred"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BORRSTF_A::B_0x0)
    }
    #[doc = "BOR reset occurred (default after power-on reset)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BORRSTF_A::B_0x1)
    }
}
#[doc = "system reset from CPU reset flag Reset by software by writing the RMVF bit. Set by hardware when the system reset is due to CPU.The CPU can generate a system reset by writing SYSRESETREQ bit of AIRCR register of the core M33.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFTRSTF_A {
    #[doc = "0: no CPU software reset occurred (default after power-on reset)"]
    B_0x0 = 0,
    #[doc = "1: a system reset has been generated by the CPU"]
    B_0x1 = 1,
}
impl From<SFTRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: SFTRSTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SFTRSTF` reader - system reset from CPU reset flag Reset by software by writing the RMVF bit. Set by hardware when the system reset is due to CPU.The CPU can generate a system reset by writing SYSRESETREQ bit of AIRCR register of the core M33."]
pub type SFTRSTF_R = crate::BitReader<SFTRSTF_A>;
impl SFTRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SFTRSTF_A {
        match self.bits {
            false => SFTRSTF_A::B_0x0,
            true => SFTRSTF_A::B_0x1,
        }
    }
    #[doc = "no CPU software reset occurred (default after power-on reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SFTRSTF_A::B_0x0
    }
    #[doc = "a system reset has been generated by the CPU"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SFTRSTF_A::B_0x1
    }
}
#[doc = "Field `SFTRSTF` writer - system reset from CPU reset flag Reset by software by writing the RMVF bit. Set by hardware when the system reset is due to CPU.The CPU can generate a system reset by writing SYSRESETREQ bit of AIRCR register of the core M33."]
pub type SFTRSTF_W<'a, REG> = crate::BitWriter<'a, REG, SFTRSTF_A>;
impl<'a, REG> SFTRSTF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no CPU software reset occurred (default after power-on reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SFTRSTF_A::B_0x0)
    }
    #[doc = "a system reset has been generated by the CPU"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SFTRSTF_A::B_0x1)
    }
}
#[doc = "independent watchdog reset flag Reset by software by writing the RMVF bit. Set by hardware when an independent watchdog reset occurs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IWDGRSTF_A {
    #[doc = "0: no independent watchdog reset occurred (default after power-on reset)"]
    B_0x0 = 0,
    #[doc = "1: independent watchdog reset occurred"]
    B_0x1 = 1,
}
impl From<IWDGRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: IWDGRSTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IWDGRSTF` reader - independent watchdog reset flag Reset by software by writing the RMVF bit. Set by hardware when an independent watchdog reset occurs."]
pub type IWDGRSTF_R = crate::BitReader<IWDGRSTF_A>;
impl IWDGRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IWDGRSTF_A {
        match self.bits {
            false => IWDGRSTF_A::B_0x0,
            true => IWDGRSTF_A::B_0x1,
        }
    }
    #[doc = "no independent watchdog reset occurred (default after power-on reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IWDGRSTF_A::B_0x0
    }
    #[doc = "independent watchdog reset occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IWDGRSTF_A::B_0x1
    }
}
#[doc = "Field `IWDGRSTF` writer - independent watchdog reset flag Reset by software by writing the RMVF bit. Set by hardware when an independent watchdog reset occurs."]
pub type IWDGRSTF_W<'a, REG> = crate::BitWriter<'a, REG, IWDGRSTF_A>;
impl<'a, REG> IWDGRSTF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no independent watchdog reset occurred (default after power-on reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IWDGRSTF_A::B_0x0)
    }
    #[doc = "independent watchdog reset occurred"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IWDGRSTF_A::B_0x1)
    }
}
#[doc = "window watchdog reset flag Reset by software by writing the RMVF bit. Set by hardware when a window watchdog reset occurs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WWDGRSTF_A {
    #[doc = "0: no window watchdog reset occurred from WWDG (default after power-on reset)"]
    B_0x0 = 0,
    #[doc = "1: window watchdog reset occurred from WWDG"]
    B_0x1 = 1,
}
impl From<WWDGRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: WWDGRSTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WWDGRSTF` reader - window watchdog reset flag Reset by software by writing the RMVF bit. Set by hardware when a window watchdog reset occurs."]
pub type WWDGRSTF_R = crate::BitReader<WWDGRSTF_A>;
impl WWDGRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WWDGRSTF_A {
        match self.bits {
            false => WWDGRSTF_A::B_0x0,
            true => WWDGRSTF_A::B_0x1,
        }
    }
    #[doc = "no window watchdog reset occurred from WWDG (default after power-on reset)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == WWDGRSTF_A::B_0x0
    }
    #[doc = "window watchdog reset occurred from WWDG"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == WWDGRSTF_A::B_0x1
    }
}
#[doc = "Field `WWDGRSTF` writer - window watchdog reset flag Reset by software by writing the RMVF bit. Set by hardware when a window watchdog reset occurs."]
pub type WWDGRSTF_W<'a, REG> = crate::BitWriter<'a, REG, WWDGRSTF_A>;
impl<'a, REG> WWDGRSTF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no window watchdog reset occurred from WWDG (default after power-on reset)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WWDGRSTF_A::B_0x0)
    }
    #[doc = "window watchdog reset occurred from WWDG"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WWDGRSTF_A::B_0x1)
    }
}
#[doc = "Low-power reset flag Set by hardware when a reset occurs due to Stop or Standby mode entry, whereas the corresponding nRST_STOP, nRST_STBY option bit is cleared. Cleared by writing to the RMVF bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPWRRSTF_A {
    #[doc = "0: No illegal low-power mode reset occurred"]
    B_0x0 = 0,
    #[doc = "1: Illegal low-power mode reset occurred"]
    B_0x1 = 1,
}
impl From<LPWRRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: LPWRRSTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPWRRSTF` reader - Low-power reset flag Set by hardware when a reset occurs due to Stop or Standby mode entry, whereas the corresponding nRST_STOP, nRST_STBY option bit is cleared. Cleared by writing to the RMVF bit."]
pub type LPWRRSTF_R = crate::BitReader<LPWRRSTF_A>;
impl LPWRRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPWRRSTF_A {
        match self.bits {
            false => LPWRRSTF_A::B_0x0,
            true => LPWRRSTF_A::B_0x1,
        }
    }
    #[doc = "No illegal low-power mode reset occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LPWRRSTF_A::B_0x0
    }
    #[doc = "Illegal low-power mode reset occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LPWRRSTF_A::B_0x1
    }
}
#[doc = "Field `LPWRRSTF` writer - Low-power reset flag Set by hardware when a reset occurs due to Stop or Standby mode entry, whereas the corresponding nRST_STOP, nRST_STBY option bit is cleared. Cleared by writing to the RMVF bit."]
pub type LPWRRSTF_W<'a, REG> = crate::BitWriter<'a, REG, LPWRRSTF_A>;
impl<'a, REG> LPWRRSTF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No illegal low-power mode reset occurred"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LPWRRSTF_A::B_0x0)
    }
    #[doc = "Illegal low-power mode reset occurred"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LPWRRSTF_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 23 - remove reset flag Set and reset by software to reset the value of the reset flags."]
    #[inline(always)]
    pub fn RMVF(&self) -> RMVF_R {
        RMVF_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 26 - pin reset flag (NRST) Reset by software by writing the RMVF bit. Set by hardware when a reset from pin occurs."]
    #[inline(always)]
    pub fn PINRSTF(&self) -> PINRSTF_R {
        PINRSTF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - BOR reset flag Reset by software by writing the RMVF bit. Set by hardware when a BOR reset occurs (pwr_bor_rst)."]
    #[inline(always)]
    pub fn BORRSTF(&self) -> BORRSTF_R {
        BORRSTF_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - system reset from CPU reset flag Reset by software by writing the RMVF bit. Set by hardware when the system reset is due to CPU.The CPU can generate a system reset by writing SYSRESETREQ bit of AIRCR register of the core M33."]
    #[inline(always)]
    pub fn SFTRSTF(&self) -> SFTRSTF_R {
        SFTRSTF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - independent watchdog reset flag Reset by software by writing the RMVF bit. Set by hardware when an independent watchdog reset occurs."]
    #[inline(always)]
    pub fn IWDGRSTF(&self) -> IWDGRSTF_R {
        IWDGRSTF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - window watchdog reset flag Reset by software by writing the RMVF bit. Set by hardware when a window watchdog reset occurs."]
    #[inline(always)]
    pub fn WWDGRSTF(&self) -> WWDGRSTF_R {
        WWDGRSTF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low-power reset flag Set by hardware when a reset occurs due to Stop or Standby mode entry, whereas the corresponding nRST_STOP, nRST_STBY option bit is cleared. Cleared by writing to the RMVF bit."]
    #[inline(always)]
    pub fn LPWRRSTF(&self) -> LPWRRSTF_R {
        LPWRRSTF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 23 - remove reset flag Set and reset by software to reset the value of the reset flags."]
    #[inline(always)]
    pub fn RMVF(&mut self) -> RMVF_W<'_, RSR_SPEC> {
        RMVF_W::new(self, 23)
    }
    #[doc = "Bit 26 - pin reset flag (NRST) Reset by software by writing the RMVF bit. Set by hardware when a reset from pin occurs."]
    #[inline(always)]
    pub fn PINRSTF(&mut self) -> PINRSTF_W<'_, RSR_SPEC> {
        PINRSTF_W::new(self, 26)
    }
    #[doc = "Bit 27 - BOR reset flag Reset by software by writing the RMVF bit. Set by hardware when a BOR reset occurs (pwr_bor_rst)."]
    #[inline(always)]
    pub fn BORRSTF(&mut self) -> BORRSTF_W<'_, RSR_SPEC> {
        BORRSTF_W::new(self, 27)
    }
    #[doc = "Bit 28 - system reset from CPU reset flag Reset by software by writing the RMVF bit. Set by hardware when the system reset is due to CPU.The CPU can generate a system reset by writing SYSRESETREQ bit of AIRCR register of the core M33."]
    #[inline(always)]
    pub fn SFTRSTF(&mut self) -> SFTRSTF_W<'_, RSR_SPEC> {
        SFTRSTF_W::new(self, 28)
    }
    #[doc = "Bit 29 - independent watchdog reset flag Reset by software by writing the RMVF bit. Set by hardware when an independent watchdog reset occurs."]
    #[inline(always)]
    pub fn IWDGRSTF(&mut self) -> IWDGRSTF_W<'_, RSR_SPEC> {
        IWDGRSTF_W::new(self, 29)
    }
    #[doc = "Bit 30 - window watchdog reset flag Reset by software by writing the RMVF bit. Set by hardware when a window watchdog reset occurs."]
    #[inline(always)]
    pub fn WWDGRSTF(&mut self) -> WWDGRSTF_W<'_, RSR_SPEC> {
        WWDGRSTF_W::new(self, 30)
    }
    #[doc = "Bit 31 - Low-power reset flag Set by hardware when a reset occurs due to Stop or Standby mode entry, whereas the corresponding nRST_STOP, nRST_STBY option bit is cleared. Cleared by writing to the RMVF bit."]
    #[inline(always)]
    pub fn LPWRRSTF(&mut self) -> LPWRRSTF_W<'_, RSR_SPEC> {
        LPWRRSTF_W::new(self, 31)
    }
}
#[doc = "RCC reset status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSR_SPEC;
impl crate::RegisterSpec for RSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsr::R`](R) reader structure"]
impl crate::Readable for RSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rsr::W`](W) writer structure"]
impl crate::Writable for RSR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets RSR to value 0x0c00_0000"]
impl crate::Resettable for RSR_SPEC {
    const RESET_VALUE: u32 = 0x0c00_0000;
}
