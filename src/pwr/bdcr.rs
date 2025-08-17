#[doc = "Register `BDCR` reader"]
pub type R = crate::R<BDCR_SPEC>;
#[doc = "Register `BDCR` writer"]
pub type W = crate::W<BDCR_SPEC>;
#[doc = "Backup RAM retention in Standby and V BAT modes When this bit set, the backup regulator (used to maintain the backup RAM content in Standby and V BAT modes) is enabled. If BREN is cleared, the backup regulator is switched off. The backup RAM can still be used in Run and Stop modes. However its content is lost in Standby and V BAT modes. If BREN is set, the application must wait till the backup regulator ready flag (BRRDY) is set to indicate that the data written into the SRAM is maintained in Standby and V BAT modes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BREN_A {
    #[doc = "0: Backup RAM content lost in Standby and V BAT modes."]
    B_0x0 = 0,
    #[doc = "1: Backup RAM content preserved in Standby and V BAT modes"]
    B_0x1 = 1,
}
impl From<BREN_A> for bool {
    #[inline(always)]
    fn from(variant: BREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BREN` reader - Backup RAM retention in Standby and V BAT modes When this bit set, the backup regulator (used to maintain the backup RAM content in Standby and V BAT modes) is enabled. If BREN is cleared, the backup regulator is switched off. The backup RAM can still be used in Run and Stop modes. However its content is lost in Standby and V BAT modes. If BREN is set, the application must wait till the backup regulator ready flag (BRRDY) is set to indicate that the data written into the SRAM is maintained in Standby and V BAT modes."]
pub type BREN_R = crate::BitReader<BREN_A>;
impl BREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BREN_A {
        match self.bits {
            false => BREN_A::B_0x0,
            true => BREN_A::B_0x1,
        }
    }
    #[doc = "Backup RAM content lost in Standby and V BAT modes."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BREN_A::B_0x0
    }
    #[doc = "Backup RAM content preserved in Standby and V BAT modes"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BREN_A::B_0x1
    }
}
#[doc = "Field `BREN` writer - Backup RAM retention in Standby and V BAT modes When this bit set, the backup regulator (used to maintain the backup RAM content in Standby and V BAT modes) is enabled. If BREN is cleared, the backup regulator is switched off. The backup RAM can still be used in Run and Stop modes. However its content is lost in Standby and V BAT modes. If BREN is set, the application must wait till the backup regulator ready flag (BRRDY) is set to indicate that the data written into the SRAM is maintained in Standby and V BAT modes."]
pub type BREN_W<'a, REG> = crate::BitWriter<'a, REG, BREN_A>;
impl<'a, REG> BREN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Backup RAM content lost in Standby and V BAT modes."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BREN_A::B_0x0)
    }
    #[doc = "Backup RAM content preserved in Standby and V BAT modes"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BREN_A::B_0x1)
    }
}
#[doc = "Backup domain voltage and temperature monitoring enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MONEN_A {
    #[doc = "0: Backup domain voltage and temperature monitoring disabled"]
    B_0x0 = 0,
    #[doc = "1: Backup domain voltage and temperature monitoring enabled"]
    B_0x1 = 1,
}
impl From<MONEN_A> for bool {
    #[inline(always)]
    fn from(variant: MONEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MONEN` reader - Backup domain voltage and temperature monitoring enable"]
pub type MONEN_R = crate::BitReader<MONEN_A>;
impl MONEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MONEN_A {
        match self.bits {
            false => MONEN_A::B_0x0,
            true => MONEN_A::B_0x1,
        }
    }
    #[doc = "Backup domain voltage and temperature monitoring disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MONEN_A::B_0x0
    }
    #[doc = "Backup domain voltage and temperature monitoring enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MONEN_A::B_0x1
    }
}
#[doc = "Field `MONEN` writer - Backup domain voltage and temperature monitoring enable"]
pub type MONEN_W<'a, REG> = crate::BitWriter<'a, REG, MONEN_A>;
impl<'a, REG> MONEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Backup domain voltage and temperature monitoring disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MONEN_A::B_0x0)
    }
    #[doc = "Backup domain voltage and temperature monitoring enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MONEN_A::B_0x1)
    }
}
#[doc = "V BAT charging enable Note: Reset only by POR,.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBE_A {
    #[doc = "0: V BAT battery charging disabled."]
    B_0x0 = 0,
    #[doc = "1: V BAT battery charging enabled."]
    B_0x1 = 1,
}
impl From<VBE_A> for bool {
    #[inline(always)]
    fn from(variant: VBE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBE` reader - V BAT charging enable Note: Reset only by POR,."]
pub type VBE_R = crate::BitReader<VBE_A>;
impl VBE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VBE_A {
        match self.bits {
            false => VBE_A::B_0x0,
            true => VBE_A::B_0x1,
        }
    }
    #[doc = "V BAT battery charging disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == VBE_A::B_0x0
    }
    #[doc = "V BAT battery charging enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == VBE_A::B_0x1
    }
}
#[doc = "Field `VBE` writer - V BAT charging enable Note: Reset only by POR,."]
pub type VBE_W<'a, REG> = crate::BitWriter<'a, REG, VBE_A>;
impl<'a, REG> VBE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "V BAT battery charging disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(VBE_A::B_0x0)
    }
    #[doc = "V BAT battery charging enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(VBE_A::B_0x1)
    }
}
#[doc = "V BAT charging resistor selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBRS_A {
    #[doc = "0: Charge V BAT through a 5 kohm resistor."]
    B_0x0 = 0,
    #[doc = "1: Charge V BAT through a 1.5 kohm resistor."]
    B_0x1 = 1,
}
impl From<VBRS_A> for bool {
    #[inline(always)]
    fn from(variant: VBRS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBRS` reader - V BAT charging resistor selection"]
pub type VBRS_R = crate::BitReader<VBRS_A>;
impl VBRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VBRS_A {
        match self.bits {
            false => VBRS_A::B_0x0,
            true => VBRS_A::B_0x1,
        }
    }
    #[doc = "Charge V BAT through a 5 kohm resistor."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == VBRS_A::B_0x0
    }
    #[doc = "Charge V BAT through a 1.5 kohm resistor."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == VBRS_A::B_0x1
    }
}
#[doc = "Field `VBRS` writer - V BAT charging resistor selection"]
pub type VBRS_W<'a, REG> = crate::BitWriter<'a, REG, VBRS_A>;
impl<'a, REG> VBRS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Charge V BAT through a 5 kohm resistor."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(VBRS_A::B_0x0)
    }
    #[doc = "Charge V BAT through a 1.5 kohm resistor."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(VBRS_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Backup RAM retention in Standby and V BAT modes When this bit set, the backup regulator (used to maintain the backup RAM content in Standby and V BAT modes) is enabled. If BREN is cleared, the backup regulator is switched off. The backup RAM can still be used in Run and Stop modes. However its content is lost in Standby and V BAT modes. If BREN is set, the application must wait till the backup regulator ready flag (BRRDY) is set to indicate that the data written into the SRAM is maintained in Standby and V BAT modes."]
    #[inline(always)]
    pub fn BREN(&self) -> BREN_R {
        BREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Backup domain voltage and temperature monitoring enable"]
    #[inline(always)]
    pub fn MONEN(&self) -> MONEN_R {
        MONEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - V BAT charging enable Note: Reset only by POR,."]
    #[inline(always)]
    pub fn VBE(&self) -> VBE_R {
        VBE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - V BAT charging resistor selection"]
    #[inline(always)]
    pub fn VBRS(&self) -> VBRS_R {
        VBRS_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Backup RAM retention in Standby and V BAT modes When this bit set, the backup regulator (used to maintain the backup RAM content in Standby and V BAT modes) is enabled. If BREN is cleared, the backup regulator is switched off. The backup RAM can still be used in Run and Stop modes. However its content is lost in Standby and V BAT modes. If BREN is set, the application must wait till the backup regulator ready flag (BRRDY) is set to indicate that the data written into the SRAM is maintained in Standby and V BAT modes."]
    #[inline(always)]
    pub fn BREN(&mut self) -> BREN_W<'_, BDCR_SPEC> {
        BREN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Backup domain voltage and temperature monitoring enable"]
    #[inline(always)]
    pub fn MONEN(&mut self) -> MONEN_W<'_, BDCR_SPEC> {
        MONEN_W::new(self, 1)
    }
    #[doc = "Bit 8 - V BAT charging enable Note: Reset only by POR,."]
    #[inline(always)]
    pub fn VBE(&mut self) -> VBE_W<'_, BDCR_SPEC> {
        VBE_W::new(self, 8)
    }
    #[doc = "Bit 9 - V BAT charging resistor selection"]
    #[inline(always)]
    pub fn VBRS(&mut self) -> VBRS_W<'_, BDCR_SPEC> {
        VBRS_W::new(self, 9)
    }
}
#[doc = "PWR Backup domain control register\n\nYou can [`read`](crate::Reg::read) this register and get [`bdcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BDCR_SPEC;
impl crate::RegisterSpec for BDCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bdcr::R`](R) reader structure"]
impl crate::Readable for BDCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bdcr::W`](W) writer structure"]
impl crate::Writable for BDCR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets BDCR to value 0"]
impl crate::Resettable for BDCR_SPEC {}
