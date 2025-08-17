#[doc = "Register `CR` reader"]
pub type R = crate::R<CR_SPEC>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CR_SPEC>;
#[doc = "SYNC event OK interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNCOKIE_A {
    #[doc = "0: SYNC event OK (SYNCOKF) interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: SYNC event OK (SYNCOKF) interrupt enabled"]
    B_0x1 = 1,
}
impl From<SYNCOKIE_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCOKIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNCOKIE` reader - SYNC event OK interrupt enable"]
pub type SYNCOKIE_R = crate::BitReader<SYNCOKIE_A>;
impl SYNCOKIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SYNCOKIE_A {
        match self.bits {
            false => SYNCOKIE_A::B_0x0,
            true => SYNCOKIE_A::B_0x1,
        }
    }
    #[doc = "SYNC event OK (SYNCOKF) interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SYNCOKIE_A::B_0x0
    }
    #[doc = "SYNC event OK (SYNCOKF) interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SYNCOKIE_A::B_0x1
    }
}
#[doc = "Field `SYNCOKIE` writer - SYNC event OK interrupt enable"]
pub type SYNCOKIE_W<'a, REG> = crate::BitWriter<'a, REG, SYNCOKIE_A>;
impl<'a, REG> SYNCOKIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SYNC event OK (SYNCOKF) interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCOKIE_A::B_0x0)
    }
    #[doc = "SYNC event OK (SYNCOKF) interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCOKIE_A::B_0x1)
    }
}
#[doc = "SYNC warning interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNCWARNIE_A {
    #[doc = "0: SYNC warning (SYNCWARNF) interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: SYNC warning (SYNCWARNF) interrupt enabled"]
    B_0x1 = 1,
}
impl From<SYNCWARNIE_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCWARNIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNCWARNIE` reader - SYNC warning interrupt enable"]
pub type SYNCWARNIE_R = crate::BitReader<SYNCWARNIE_A>;
impl SYNCWARNIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SYNCWARNIE_A {
        match self.bits {
            false => SYNCWARNIE_A::B_0x0,
            true => SYNCWARNIE_A::B_0x1,
        }
    }
    #[doc = "SYNC warning (SYNCWARNF) interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SYNCWARNIE_A::B_0x0
    }
    #[doc = "SYNC warning (SYNCWARNF) interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SYNCWARNIE_A::B_0x1
    }
}
#[doc = "Field `SYNCWARNIE` writer - SYNC warning interrupt enable"]
pub type SYNCWARNIE_W<'a, REG> = crate::BitWriter<'a, REG, SYNCWARNIE_A>;
impl<'a, REG> SYNCWARNIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SYNC warning (SYNCWARNF) interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCWARNIE_A::B_0x0)
    }
    #[doc = "SYNC warning (SYNCWARNF) interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCWARNIE_A::B_0x1)
    }
}
#[doc = "Synchronization or trimming error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRIE_A {
    #[doc = "0: Synchronization or trimming error (ERRF) interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: Synchronization or trimming error (ERRF) interrupt enabled"]
    B_0x1 = 1,
}
impl From<ERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: ERRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIE` reader - Synchronization or trimming error interrupt enable"]
pub type ERRIE_R = crate::BitReader<ERRIE_A>;
impl ERRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ERRIE_A {
        match self.bits {
            false => ERRIE_A::B_0x0,
            true => ERRIE_A::B_0x1,
        }
    }
    #[doc = "Synchronization or trimming error (ERRF) interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ERRIE_A::B_0x0
    }
    #[doc = "Synchronization or trimming error (ERRF) interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ERRIE_A::B_0x1
    }
}
#[doc = "Field `ERRIE` writer - Synchronization or trimming error interrupt enable"]
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG, ERRIE_A>;
impl<'a, REG> ERRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Synchronization or trimming error (ERRF) interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE_A::B_0x0)
    }
    #[doc = "Synchronization or trimming error (ERRF) interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE_A::B_0x1)
    }
}
#[doc = "Expected SYNC interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ESYNCIE_A {
    #[doc = "0: Expected SYNC (ESYNCF) interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: Expected SYNC (ESYNCF) interrupt enabled"]
    B_0x1 = 1,
}
impl From<ESYNCIE_A> for bool {
    #[inline(always)]
    fn from(variant: ESYNCIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ESYNCIE` reader - Expected SYNC interrupt enable"]
pub type ESYNCIE_R = crate::BitReader<ESYNCIE_A>;
impl ESYNCIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ESYNCIE_A {
        match self.bits {
            false => ESYNCIE_A::B_0x0,
            true => ESYNCIE_A::B_0x1,
        }
    }
    #[doc = "Expected SYNC (ESYNCF) interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ESYNCIE_A::B_0x0
    }
    #[doc = "Expected SYNC (ESYNCF) interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ESYNCIE_A::B_0x1
    }
}
#[doc = "Field `ESYNCIE` writer - Expected SYNC interrupt enable"]
pub type ESYNCIE_W<'a, REG> = crate::BitWriter<'a, REG, ESYNCIE_A>;
impl<'a, REG> ESYNCIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Expected SYNC (ESYNCF) interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ESYNCIE_A::B_0x0)
    }
    #[doc = "Expected SYNC (ESYNCF) interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ESYNCIE_A::B_0x1)
    }
}
#[doc = "Frequency error counter enable This bit enables the oscillator clock for the frequency error counter. When this bit is set, the CRS_CFGR register is write-protected and cannot be modified.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEN_A {
    #[doc = "0: Frequency error counter disabled"]
    B_0x0 = 0,
    #[doc = "1: Frequency error counter enabled"]
    B_0x1 = 1,
}
impl From<CEN_A> for bool {
    #[inline(always)]
    fn from(variant: CEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEN` reader - Frequency error counter enable This bit enables the oscillator clock for the frequency error counter. When this bit is set, the CRS_CFGR register is write-protected and cannot be modified."]
pub type CEN_R = crate::BitReader<CEN_A>;
impl CEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CEN_A {
        match self.bits {
            false => CEN_A::B_0x0,
            true => CEN_A::B_0x1,
        }
    }
    #[doc = "Frequency error counter disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CEN_A::B_0x0
    }
    #[doc = "Frequency error counter enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CEN_A::B_0x1
    }
}
#[doc = "Field `CEN` writer - Frequency error counter enable This bit enables the oscillator clock for the frequency error counter. When this bit is set, the CRS_CFGR register is write-protected and cannot be modified."]
pub type CEN_W<'a, REG> = crate::BitWriter<'a, REG, CEN_A>;
impl<'a, REG> CEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Frequency error counter disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CEN_A::B_0x0)
    }
    #[doc = "Frequency error counter enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CEN_A::B_0x1)
    }
}
#[doc = "Automatic trimming enable This bit enables the automatic hardware adjustment of TRIM bits according to the measured frequency error between two SYNC events. If this bit is set, the TRIM bits are read-only. The TRIM value can be adjusted by hardware by one or two steps at a time, depending on the measured frequency error value. Refer to Section 10.5.3 for more details.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTOTRIMEN_A {
    #[doc = "0: Automatic trimming disabled, TRIM bits can be adjusted by the user."]
    B_0x0 = 0,
    #[doc = "1: Automatic trimming enabled, TRIM bits are read-only and under hardware control."]
    B_0x1 = 1,
}
impl From<AUTOTRIMEN_A> for bool {
    #[inline(always)]
    fn from(variant: AUTOTRIMEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTOTRIMEN` reader - Automatic trimming enable This bit enables the automatic hardware adjustment of TRIM bits according to the measured frequency error between two SYNC events. If this bit is set, the TRIM bits are read-only. The TRIM value can be adjusted by hardware by one or two steps at a time, depending on the measured frequency error value. Refer to Section 10.5.3 for more details."]
pub type AUTOTRIMEN_R = crate::BitReader<AUTOTRIMEN_A>;
impl AUTOTRIMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AUTOTRIMEN_A {
        match self.bits {
            false => AUTOTRIMEN_A::B_0x0,
            true => AUTOTRIMEN_A::B_0x1,
        }
    }
    #[doc = "Automatic trimming disabled, TRIM bits can be adjusted by the user."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == AUTOTRIMEN_A::B_0x0
    }
    #[doc = "Automatic trimming enabled, TRIM bits are read-only and under hardware control."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == AUTOTRIMEN_A::B_0x1
    }
}
#[doc = "Field `AUTOTRIMEN` writer - Automatic trimming enable This bit enables the automatic hardware adjustment of TRIM bits according to the measured frequency error between two SYNC events. If this bit is set, the TRIM bits are read-only. The TRIM value can be adjusted by hardware by one or two steps at a time, depending on the measured frequency error value. Refer to Section 10.5.3 for more details."]
pub type AUTOTRIMEN_W<'a, REG> = crate::BitWriter<'a, REG, AUTOTRIMEN_A>;
impl<'a, REG> AUTOTRIMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic trimming disabled, TRIM bits can be adjusted by the user."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AUTOTRIMEN_A::B_0x0)
    }
    #[doc = "Automatic trimming enabled, TRIM bits are read-only and under hardware control."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AUTOTRIMEN_A::B_0x1)
    }
}
#[doc = "Generate software SYNC event This bit is set by software in order to generate a software SYNC event. It is automatically cleared by hardware.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWSYNC_A {
    #[doc = "0: No action"]
    B_0x0 = 0,
    #[doc = "1: A software SYNC event is generated."]
    B_0x1 = 1,
}
impl From<SWSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: SWSYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWSYNC` reader - Generate software SYNC event This bit is set by software in order to generate a software SYNC event. It is automatically cleared by hardware."]
pub type SWSYNC_R = crate::BitReader<SWSYNC_A>;
impl SWSYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SWSYNC_A {
        match self.bits {
            false => SWSYNC_A::B_0x0,
            true => SWSYNC_A::B_0x1,
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SWSYNC_A::B_0x0
    }
    #[doc = "A software SYNC event is generated."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SWSYNC_A::B_0x1
    }
}
#[doc = "Field `SWSYNC` writer - Generate software SYNC event This bit is set by software in order to generate a software SYNC event. It is automatically cleared by hardware."]
pub type SWSYNC_W<'a, REG> = crate::BitWriter<'a, REG, SWSYNC_A>;
impl<'a, REG> SWSYNC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SWSYNC_A::B_0x0)
    }
    #[doc = "A software SYNC event is generated."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SWSYNC_A::B_0x1)
    }
}
#[doc = "Field `TRIM` reader - HSI48 oscillator smooth trimming These bits provide a user-programmable trimming value to the HSI48 oscillator. They can be programmed to adjust to variations in voltage and temperature that influence the frequency of the HSI48 oscillator. The default value is 32, which corresponds to the middle of the trimming interval. The trimming step is specified in the product datasheet. A higher TRIM value corresponds to a higher output frequency. When the AUTOTRIMEN bit is set, this field is controlled by hardware and is read-only."]
pub type TRIM_R = crate::FieldReader;
#[doc = "Field `TRIM` writer - HSI48 oscillator smooth trimming These bits provide a user-programmable trimming value to the HSI48 oscillator. They can be programmed to adjust to variations in voltage and temperature that influence the frequency of the HSI48 oscillator. The default value is 32, which corresponds to the middle of the trimming interval. The trimming step is specified in the product datasheet. A higher TRIM value corresponds to a higher output frequency. When the AUTOTRIMEN bit is set, this field is controlled by hardware and is read-only."]
pub type TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - SYNC event OK interrupt enable"]
    #[inline(always)]
    pub fn SYNCOKIE(&self) -> SYNCOKIE_R {
        SYNCOKIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SYNC warning interrupt enable"]
    #[inline(always)]
    pub fn SYNCWARNIE(&self) -> SYNCWARNIE_R {
        SYNCWARNIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Synchronization or trimming error interrupt enable"]
    #[inline(always)]
    pub fn ERRIE(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Expected SYNC interrupt enable"]
    #[inline(always)]
    pub fn ESYNCIE(&self) -> ESYNCIE_R {
        ESYNCIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Frequency error counter enable This bit enables the oscillator clock for the frequency error counter. When this bit is set, the CRS_CFGR register is write-protected and cannot be modified."]
    #[inline(always)]
    pub fn CEN(&self) -> CEN_R {
        CEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Automatic trimming enable This bit enables the automatic hardware adjustment of TRIM bits according to the measured frequency error between two SYNC events. If this bit is set, the TRIM bits are read-only. The TRIM value can be adjusted by hardware by one or two steps at a time, depending on the measured frequency error value. Refer to Section 10.5.3 for more details."]
    #[inline(always)]
    pub fn AUTOTRIMEN(&self) -> AUTOTRIMEN_R {
        AUTOTRIMEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Generate software SYNC event This bit is set by software in order to generate a software SYNC event. It is automatically cleared by hardware."]
    #[inline(always)]
    pub fn SWSYNC(&self) -> SWSYNC_R {
        SWSYNC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13 - HSI48 oscillator smooth trimming These bits provide a user-programmable trimming value to the HSI48 oscillator. They can be programmed to adjust to variations in voltage and temperature that influence the frequency of the HSI48 oscillator. The default value is 32, which corresponds to the middle of the trimming interval. The trimming step is specified in the product datasheet. A higher TRIM value corresponds to a higher output frequency. When the AUTOTRIMEN bit is set, this field is controlled by hardware and is read-only."]
    #[inline(always)]
    pub fn TRIM(&self) -> TRIM_R {
        TRIM_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - SYNC event OK interrupt enable"]
    #[inline(always)]
    pub fn SYNCOKIE(&mut self) -> SYNCOKIE_W<'_, CR_SPEC> {
        SYNCOKIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - SYNC warning interrupt enable"]
    #[inline(always)]
    pub fn SYNCWARNIE(&mut self) -> SYNCWARNIE_W<'_, CR_SPEC> {
        SYNCWARNIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Synchronization or trimming error interrupt enable"]
    #[inline(always)]
    pub fn ERRIE(&mut self) -> ERRIE_W<'_, CR_SPEC> {
        ERRIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Expected SYNC interrupt enable"]
    #[inline(always)]
    pub fn ESYNCIE(&mut self) -> ESYNCIE_W<'_, CR_SPEC> {
        ESYNCIE_W::new(self, 3)
    }
    #[doc = "Bit 5 - Frequency error counter enable This bit enables the oscillator clock for the frequency error counter. When this bit is set, the CRS_CFGR register is write-protected and cannot be modified."]
    #[inline(always)]
    pub fn CEN(&mut self) -> CEN_W<'_, CR_SPEC> {
        CEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Automatic trimming enable This bit enables the automatic hardware adjustment of TRIM bits according to the measured frequency error between two SYNC events. If this bit is set, the TRIM bits are read-only. The TRIM value can be adjusted by hardware by one or two steps at a time, depending on the measured frequency error value. Refer to Section 10.5.3 for more details."]
    #[inline(always)]
    pub fn AUTOTRIMEN(&mut self) -> AUTOTRIMEN_W<'_, CR_SPEC> {
        AUTOTRIMEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Generate software SYNC event This bit is set by software in order to generate a software SYNC event. It is automatically cleared by hardware."]
    #[inline(always)]
    pub fn SWSYNC(&mut self) -> SWSYNC_W<'_, CR_SPEC> {
        SWSYNC_W::new(self, 7)
    }
    #[doc = "Bits 8:13 - HSI48 oscillator smooth trimming These bits provide a user-programmable trimming value to the HSI48 oscillator. They can be programmed to adjust to variations in voltage and temperature that influence the frequency of the HSI48 oscillator. The default value is 32, which corresponds to the middle of the trimming interval. The trimming step is specified in the product datasheet. A higher TRIM value corresponds to a higher output frequency. When the AUTOTRIMEN bit is set, this field is controlled by hardware and is read-only."]
    #[inline(always)]
    pub fn TRIM(&mut self) -> TRIM_W<'_, CR_SPEC> {
        TRIM_W::new(self, 8)
    }
}
#[doc = "CRS control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CR to value 0x2000"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: u32 = 0x2000;
}
