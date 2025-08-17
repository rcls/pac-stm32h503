#[doc = "Register `CCCSR` reader"]
pub type R = crate::R<CCCSR_SPEC>;
#[doc = "Register `CCCSR` writer"]
pub type W = crate::W<CCCSR_SPEC>;
#[doc = "enable compensation cell for VDDIO power rail This bit enables the I/O compensation cell.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN1_A {
    #[doc = "0: I/O compensation cell disabled"]
    B_0x0 = 0,
    #[doc = "1: I/O compensation cell enabled"]
    B_0x1 = 1,
}
impl From<EN1_A> for bool {
    #[inline(always)]
    fn from(variant: EN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN1` reader - enable compensation cell for VDDIO power rail This bit enables the I/O compensation cell."]
pub type EN1_R = crate::BitReader<EN1_A>;
impl EN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EN1_A {
        match self.bits {
            false => EN1_A::B_0x0,
            true => EN1_A::B_0x1,
        }
    }
    #[doc = "I/O compensation cell disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EN1_A::B_0x0
    }
    #[doc = "I/O compensation cell enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EN1_A::B_0x1
    }
}
#[doc = "Field `EN1` writer - enable compensation cell for VDDIO power rail This bit enables the I/O compensation cell."]
pub type EN1_W<'a, REG> = crate::BitWriter<'a, REG, EN1_A>;
impl<'a, REG> EN1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I/O compensation cell disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EN1_A::B_0x0)
    }
    #[doc = "I/O compensation cell enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EN1_A::B_0x1)
    }
}
#[doc = "code selection for VDDIO power rail (reset value set to 1) This bit selects the code to be applied for the I/O compensation cell.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CS1_A {
    #[doc = "0: Code from the cell (available in the SBS_CCVR)"]
    B_0x0 = 0,
    #[doc = "1: Code from SBS_CCCR"]
    B_0x1 = 1,
}
impl From<CS1_A> for bool {
    #[inline(always)]
    fn from(variant: CS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CS1` reader - code selection for VDDIO power rail (reset value set to 1) This bit selects the code to be applied for the I/O compensation cell."]
pub type CS1_R = crate::BitReader<CS1_A>;
impl CS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CS1_A {
        match self.bits {
            false => CS1_A::B_0x0,
            true => CS1_A::B_0x1,
        }
    }
    #[doc = "Code from the cell (available in the SBS_CCVR)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CS1_A::B_0x0
    }
    #[doc = "Code from SBS_CCCR"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CS1_A::B_0x1
    }
}
#[doc = "Field `CS1` writer - code selection for VDDIO power rail (reset value set to 1) This bit selects the code to be applied for the I/O compensation cell."]
pub type CS1_W<'a, REG> = crate::BitWriter<'a, REG, CS1_A>;
impl<'a, REG> CS1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Code from the cell (available in the SBS_CCVR)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CS1_A::B_0x0)
    }
    #[doc = "Code from SBS_CCCR"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CS1_A::B_0x1)
    }
}
#[doc = "enable compensation cell for VDDIO2 power rail This bit enables the I/O compensation cell.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN2_A {
    #[doc = "0: I/O compensation cell disabled"]
    B_0x0 = 0,
    #[doc = "1: I/O compensation cell enabled"]
    B_0x1 = 1,
}
impl From<EN2_A> for bool {
    #[inline(always)]
    fn from(variant: EN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN2` reader - enable compensation cell for VDDIO2 power rail This bit enables the I/O compensation cell."]
pub type EN2_R = crate::BitReader<EN2_A>;
impl EN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EN2_A {
        match self.bits {
            false => EN2_A::B_0x0,
            true => EN2_A::B_0x1,
        }
    }
    #[doc = "I/O compensation cell disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EN2_A::B_0x0
    }
    #[doc = "I/O compensation cell enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EN2_A::B_0x1
    }
}
#[doc = "Field `EN2` writer - enable compensation cell for VDDIO2 power rail This bit enables the I/O compensation cell."]
pub type EN2_W<'a, REG> = crate::BitWriter<'a, REG, EN2_A>;
impl<'a, REG> EN2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I/O compensation cell disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EN2_A::B_0x0)
    }
    #[doc = "I/O compensation cell enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EN2_A::B_0x1)
    }
}
#[doc = "code selection for VDDIO2 power rail (reset value set to 1) This bit selects the code to be applied for the I/O compensation cell.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CS2_A {
    #[doc = "0: Code from the cell (available in SBS_CCVR)"]
    B_0x0 = 0,
    #[doc = "1: Code from SBS_CCCR"]
    B_0x1 = 1,
}
impl From<CS2_A> for bool {
    #[inline(always)]
    fn from(variant: CS2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CS2` reader - code selection for VDDIO2 power rail (reset value set to 1) This bit selects the code to be applied for the I/O compensation cell."]
pub type CS2_R = crate::BitReader<CS2_A>;
impl CS2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CS2_A {
        match self.bits {
            false => CS2_A::B_0x0,
            true => CS2_A::B_0x1,
        }
    }
    #[doc = "Code from the cell (available in SBS_CCVR)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CS2_A::B_0x0
    }
    #[doc = "Code from SBS_CCCR"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CS2_A::B_0x1
    }
}
#[doc = "Field `CS2` writer - code selection for VDDIO2 power rail (reset value set to 1) This bit selects the code to be applied for the I/O compensation cell."]
pub type CS2_W<'a, REG> = crate::BitWriter<'a, REG, CS2_A>;
impl<'a, REG> CS2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Code from the cell (available in SBS_CCVR)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CS2_A::B_0x0)
    }
    #[doc = "Code from SBS_CCCR"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CS2_A::B_0x1)
    }
}
#[doc = "VDDIO compensation cell ready flag This bit provides the status of the compensation cell.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDY1_A {
    #[doc = "0: VDDIO compensation cell not ready"]
    B_0x0 = 0,
    #[doc = "1: VDDIO compensation cell ready (code value provided by the cell can be used)"]
    B_0x1 = 1,
}
impl From<RDY1_A> for bool {
    #[inline(always)]
    fn from(variant: RDY1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDY1` reader - VDDIO compensation cell ready flag This bit provides the status of the compensation cell."]
pub type RDY1_R = crate::BitReader<RDY1_A>;
impl RDY1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RDY1_A {
        match self.bits {
            false => RDY1_A::B_0x0,
            true => RDY1_A::B_0x1,
        }
    }
    #[doc = "VDDIO compensation cell not ready"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RDY1_A::B_0x0
    }
    #[doc = "VDDIO compensation cell ready (code value provided by the cell can be used)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RDY1_A::B_0x1
    }
}
#[doc = "VDDIO2 compensation cell ready flag This bit provides the status of the VDDIO2 compensation cell.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDY2_A {
    #[doc = "0: VDDIO2 compensation cell not ready"]
    B_0x0 = 0,
    #[doc = "1: VDDIO2 compensation cell ready (code value provided by the cell can be used)"]
    B_0x1 = 1,
}
impl From<RDY2_A> for bool {
    #[inline(always)]
    fn from(variant: RDY2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDY2` reader - VDDIO2 compensation cell ready flag This bit provides the status of the VDDIO2 compensation cell."]
pub type RDY2_R = crate::BitReader<RDY2_A>;
impl RDY2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RDY2_A {
        match self.bits {
            false => RDY2_A::B_0x0,
            true => RDY2_A::B_0x1,
        }
    }
    #[doc = "VDDIO2 compensation cell not ready"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RDY2_A::B_0x0
    }
    #[doc = "VDDIO2 compensation cell ready (code value provided by the cell can be used)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RDY2_A::B_0x1
    }
}
impl R {
    #[doc = "Bit 0 - enable compensation cell for VDDIO power rail This bit enables the I/O compensation cell."]
    #[inline(always)]
    pub fn EN1(&self) -> EN1_R {
        EN1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - code selection for VDDIO power rail (reset value set to 1) This bit selects the code to be applied for the I/O compensation cell."]
    #[inline(always)]
    pub fn CS1(&self) -> CS1_R {
        CS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - enable compensation cell for VDDIO2 power rail This bit enables the I/O compensation cell."]
    #[inline(always)]
    pub fn EN2(&self) -> EN2_R {
        EN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - code selection for VDDIO2 power rail (reset value set to 1) This bit selects the code to be applied for the I/O compensation cell."]
    #[inline(always)]
    pub fn CS2(&self) -> CS2_R {
        CS2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - VDDIO compensation cell ready flag This bit provides the status of the compensation cell."]
    #[inline(always)]
    pub fn RDY1(&self) -> RDY1_R {
        RDY1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - VDDIO2 compensation cell ready flag This bit provides the status of the VDDIO2 compensation cell."]
    #[inline(always)]
    pub fn RDY2(&self) -> RDY2_R {
        RDY2_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable compensation cell for VDDIO power rail This bit enables the I/O compensation cell."]
    #[inline(always)]
    pub fn EN1(&mut self) -> EN1_W<'_, CCCSR_SPEC> {
        EN1_W::new(self, 0)
    }
    #[doc = "Bit 1 - code selection for VDDIO power rail (reset value set to 1) This bit selects the code to be applied for the I/O compensation cell."]
    #[inline(always)]
    pub fn CS1(&mut self) -> CS1_W<'_, CCCSR_SPEC> {
        CS1_W::new(self, 1)
    }
    #[doc = "Bit 2 - enable compensation cell for VDDIO2 power rail This bit enables the I/O compensation cell."]
    #[inline(always)]
    pub fn EN2(&mut self) -> EN2_W<'_, CCCSR_SPEC> {
        EN2_W::new(self, 2)
    }
    #[doc = "Bit 3 - code selection for VDDIO2 power rail (reset value set to 1) This bit selects the code to be applied for the I/O compensation cell."]
    #[inline(always)]
    pub fn CS2(&mut self) -> CS2_W<'_, CCCSR_SPEC> {
        CS2_W::new(self, 3)
    }
}
#[doc = "SBS compensation cell for I/Os control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`cccsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cccsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCCSR_SPEC;
impl crate::RegisterSpec for CCCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cccsr::R`](R) reader structure"]
impl crate::Readable for CCCSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cccsr::W`](W) writer structure"]
impl crate::Writable for CCCSR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CCCSR to value 0"]
impl crate::Resettable for CCCSR_SPEC {}
