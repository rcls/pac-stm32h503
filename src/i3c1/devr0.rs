#[doc = "Register `DEVR0` reader"]
pub type R = crate::R<DEVR0_SPEC>;
#[doc = "Register `DEVR0` writer"]
pub type W = crate::W<DEVR0_SPEC>;
#[doc = "Field `DAVAL` reader - dynamic address is valid (when the I3C is acting as target) When the I3C is acting as controller, this field can be written by software, for validating its own dynamic address, for example before a controller-role hand-off. When the I3C is acting as target, this field is asserted by hardware on the acknowledge of the broadcast ENTDAA CCC or the direct SETNEWDA CCC, and this field is cleared by hardware on the acknowledge of the broadcast RSTDAA CCC."]
pub type DAVAL_R = crate::BitReader;
#[doc = "Field `DAVAL` writer - dynamic address is valid (when the I3C is acting as target) When the I3C is acting as controller, this field can be written by software, for validating its own dynamic address, for example before a controller-role hand-off. When the I3C is acting as target, this field is asserted by hardware on the acknowledge of the broadcast ENTDAA CCC or the direct SETNEWDA CCC, and this field is cleared by hardware on the acknowledge of the broadcast RSTDAA CCC."]
pub type DAVAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DA` reader - 7-bit dynamic address When the I3C is acting as controller, this field can be written by software, for defining its own dynamic address. When the I3C is acting as target, this field is updated by hardware on the reception of either the broadcast ENTDAA CCC or the direct SETNEWDA CCC."]
pub type DA_R = crate::FieldReader;
#[doc = "Field `DA` writer - 7-bit dynamic address When the I3C is acting as controller, this field can be written by software, for defining its own dynamic address. When the I3C is acting as target, this field is updated by hardware on the reception of either the broadcast ENTDAA CCC or the direct SETNEWDA CCC."]
pub type DA_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "IBI request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISINT=1 (i.e. cleared) and the reception of ENEC CCC with ENINT=1 (i.e. set).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IBIEN_A {
    #[doc = "0: IBI request disabled"]
    B_0x0 = 0,
    #[doc = "1: IBI request enabled"]
    B_0x1 = 1,
}
impl From<IBIEN_A> for bool {
    #[inline(always)]
    fn from(variant: IBIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IBIEN` reader - IBI request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISINT=1 (i.e. cleared) and the reception of ENEC CCC with ENINT=1 (i.e. set)."]
pub type IBIEN_R = crate::BitReader<IBIEN_A>;
impl IBIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IBIEN_A {
        match self.bits {
            false => IBIEN_A::B_0x0,
            true => IBIEN_A::B_0x1,
        }
    }
    #[doc = "IBI request disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IBIEN_A::B_0x0
    }
    #[doc = "IBI request enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IBIEN_A::B_0x1
    }
}
#[doc = "Field `IBIEN` writer - IBI request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISINT=1 (i.e. cleared) and the reception of ENEC CCC with ENINT=1 (i.e. set)."]
pub type IBIEN_W<'a, REG> = crate::BitWriter<'a, REG, IBIEN_A>;
impl<'a, REG> IBIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IBI request disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IBIEN_A::B_0x0)
    }
    #[doc = "IBI request enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IBIEN_A::B_0x1)
    }
}
#[doc = "controller-role request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISCR=1 (i.e. cleared) and the reception of ENEC CCC with ENCR=1 (i.e. set).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CREN_A {
    #[doc = "0: controller-role request disabled"]
    B_0x0 = 0,
    #[doc = "1: controller-role request enabled"]
    B_0x1 = 1,
}
impl From<CREN_A> for bool {
    #[inline(always)]
    fn from(variant: CREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CREN` reader - controller-role request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISCR=1 (i.e. cleared) and the reception of ENEC CCC with ENCR=1 (i.e. set)."]
pub type CREN_R = crate::BitReader<CREN_A>;
impl CREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CREN_A {
        match self.bits {
            false => CREN_A::B_0x0,
            true => CREN_A::B_0x1,
        }
    }
    #[doc = "controller-role request disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CREN_A::B_0x0
    }
    #[doc = "controller-role request enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CREN_A::B_0x1
    }
}
#[doc = "Field `CREN` writer - controller-role request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISCR=1 (i.e. cleared) and the reception of ENEC CCC with ENCR=1 (i.e. set)."]
pub type CREN_W<'a, REG> = crate::BitWriter<'a, REG, CREN_A>;
impl<'a, REG> CREN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "controller-role request disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CREN_A::B_0x0)
    }
    #[doc = "controller-role request enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CREN_A::B_0x1)
    }
}
#[doc = "hot-join request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISHJ=1 (i.e. cleared) and the reception of ENEC CCC with ENHJ=1 (i.e. set).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HJEN_A {
    #[doc = "0: hot-join request disabled"]
    B_0x0 = 0,
    #[doc = "1: hot-join request enabled"]
    B_0x1 = 1,
}
impl From<HJEN_A> for bool {
    #[inline(always)]
    fn from(variant: HJEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HJEN` reader - hot-join request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISHJ=1 (i.e. cleared) and the reception of ENEC CCC with ENHJ=1 (i.e. set)."]
pub type HJEN_R = crate::BitReader<HJEN_A>;
impl HJEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HJEN_A {
        match self.bits {
            false => HJEN_A::B_0x0,
            true => HJEN_A::B_0x1,
        }
    }
    #[doc = "hot-join request disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HJEN_A::B_0x0
    }
    #[doc = "hot-join request enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HJEN_A::B_0x1
    }
}
#[doc = "Field `HJEN` writer - hot-join request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISHJ=1 (i.e. cleared) and the reception of ENEC CCC with ENHJ=1 (i.e. set)."]
pub type HJEN_W<'a, REG> = crate::BitWriter<'a, REG, HJEN_A>;
impl<'a, REG> HJEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "hot-join request disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HJEN_A::B_0x0)
    }
    #[doc = "hot-join request enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HJEN_A::B_0x1)
    }
}
#[doc = "activity state (when the I3C is acting as target) This read field is updated by hardware on the reception of a ENTASx CCC (enter activity state, with x=0-3):\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AS_A {
    #[doc = "0: activity state 0"]
    B_0x0 = 0,
    #[doc = "1: activity state 1"]
    B_0x1 = 1,
    #[doc = "2: activity state 2"]
    B_0x2 = 2,
    #[doc = "3: activity state 3"]
    B_0x3 = 3,
}
impl From<AS_A> for u8 {
    #[inline(always)]
    fn from(variant: AS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AS_A {
    type Ux = u8;
}
impl crate::IsEnum for AS_A {}
#[doc = "Field `AS` reader - activity state (when the I3C is acting as target) This read field is updated by hardware on the reception of a ENTASx CCC (enter activity state, with x=0-3):"]
pub type AS_R = crate::FieldReader<AS_A>;
impl AS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AS_A {
        match self.bits {
            0 => AS_A::B_0x0,
            1 => AS_A::B_0x1,
            2 => AS_A::B_0x2,
            3 => AS_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "activity state 0"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == AS_A::B_0x0
    }
    #[doc = "activity state 1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == AS_A::B_0x1
    }
    #[doc = "activity state 2"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == AS_A::B_0x2
    }
    #[doc = "activity state 3"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == AS_A::B_0x3
    }
}
#[doc = "reset action/level on received reset pattern (when the I3C is acting as target) This read field is used by hardware on the reception of a direct read RSTACT CCC in order to return the corresponding data byte on the I3C bus. This read field is updated by hardware on the reception of a broadcast or direct write RSTACT CCC (target reset action).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RSTACT_A {
    #[doc = "0: no reset action"]
    B_0x0 = 0,
    #[doc = "1: first level of reset: the application software should either:"]
    B_0x1 = 1,
    #[doc = "2: second level of reset: the application software should issue a warm reset, also known as"]
    B_0x2 = 2,
    #[doc = "3: no reset action"]
    B_0x3 = 3,
}
impl From<RSTACT_A> for u8 {
    #[inline(always)]
    fn from(variant: RSTACT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RSTACT_A {
    type Ux = u8;
}
impl crate::IsEnum for RSTACT_A {}
#[doc = "Field `RSTACT` reader - reset action/level on received reset pattern (when the I3C is acting as target) This read field is used by hardware on the reception of a direct read RSTACT CCC in order to return the corresponding data byte on the I3C bus. This read field is updated by hardware on the reception of a broadcast or direct write RSTACT CCC (target reset action)."]
pub type RSTACT_R = crate::FieldReader<RSTACT_A>;
impl RSTACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RSTACT_A {
        match self.bits {
            0 => RSTACT_A::B_0x0,
            1 => RSTACT_A::B_0x1,
            2 => RSTACT_A::B_0x2,
            3 => RSTACT_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "no reset action"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RSTACT_A::B_0x0
    }
    #[doc = "first level of reset: the application software should either:"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RSTACT_A::B_0x1
    }
    #[doc = "second level of reset: the application software should issue a warm reset, also known as"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == RSTACT_A::B_0x2
    }
    #[doc = "no reset action"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == RSTACT_A::B_0x3
    }
}
#[doc = "Field `RSTVAL` reader - reset action is valid (when the I3C is acting as target) This read bit is asserted by hardware to indicate that the RTSACT\\[1:0\\] field has been updated on the reception of a broadcast or direct write RSTACT CCC (target reset action) and is valid. This field is cleared by hardware when the target receives a frame start. If RSTVAL=1: when the RSTF is asserted (and/or the corresponding interrupt if enabled), I3C_DEVR0.RSTACT\\[1:0\\] dictates the reset action to be performed by the software if any. If RSTVAL=0: when the RSTF is asserted (and/or the corresponding interrupt if enabled), the software should issue an I3C reset after a first detected reset pattern, and a system reset on the second one."]
pub type RSTVAL_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - dynamic address is valid (when the I3C is acting as target) When the I3C is acting as controller, this field can be written by software, for validating its own dynamic address, for example before a controller-role hand-off. When the I3C is acting as target, this field is asserted by hardware on the acknowledge of the broadcast ENTDAA CCC or the direct SETNEWDA CCC, and this field is cleared by hardware on the acknowledge of the broadcast RSTDAA CCC."]
    #[inline(always)]
    pub fn DAVAL(&self) -> DAVAL_R {
        DAVAL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7-bit dynamic address When the I3C is acting as controller, this field can be written by software, for defining its own dynamic address. When the I3C is acting as target, this field is updated by hardware on the reception of either the broadcast ENTDAA CCC or the direct SETNEWDA CCC."]
    #[inline(always)]
    pub fn DA(&self) -> DA_R {
        DA_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - IBI request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISINT=1 (i.e. cleared) and the reception of ENEC CCC with ENINT=1 (i.e. set)."]
    #[inline(always)]
    pub fn IBIEN(&self) -> IBIEN_R {
        IBIEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - controller-role request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISCR=1 (i.e. cleared) and the reception of ENEC CCC with ENCR=1 (i.e. set)."]
    #[inline(always)]
    pub fn CREN(&self) -> CREN_R {
        CREN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - hot-join request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISHJ=1 (i.e. cleared) and the reception of ENEC CCC with ENHJ=1 (i.e. set)."]
    #[inline(always)]
    pub fn HJEN(&self) -> HJEN_R {
        HJEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - activity state (when the I3C is acting as target) This read field is updated by hardware on the reception of a ENTASx CCC (enter activity state, with x=0-3):"]
    #[inline(always)]
    pub fn AS(&self) -> AS_R {
        AS_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - reset action/level on received reset pattern (when the I3C is acting as target) This read field is used by hardware on the reception of a direct read RSTACT CCC in order to return the corresponding data byte on the I3C bus. This read field is updated by hardware on the reception of a broadcast or direct write RSTACT CCC (target reset action)."]
    #[inline(always)]
    pub fn RSTACT(&self) -> RSTACT_R {
        RSTACT_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - reset action is valid (when the I3C is acting as target) This read bit is asserted by hardware to indicate that the RTSACT\\[1:0\\] field has been updated on the reception of a broadcast or direct write RSTACT CCC (target reset action) and is valid. This field is cleared by hardware when the target receives a frame start. If RSTVAL=1: when the RSTF is asserted (and/or the corresponding interrupt if enabled), I3C_DEVR0.RSTACT\\[1:0\\] dictates the reset action to be performed by the software if any. If RSTVAL=0: when the RSTF is asserted (and/or the corresponding interrupt if enabled), the software should issue an I3C reset after a first detected reset pattern, and a system reset on the second one."]
    #[inline(always)]
    pub fn RSTVAL(&self) -> RSTVAL_R {
        RSTVAL_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - dynamic address is valid (when the I3C is acting as target) When the I3C is acting as controller, this field can be written by software, for validating its own dynamic address, for example before a controller-role hand-off. When the I3C is acting as target, this field is asserted by hardware on the acknowledge of the broadcast ENTDAA CCC or the direct SETNEWDA CCC, and this field is cleared by hardware on the acknowledge of the broadcast RSTDAA CCC."]
    #[inline(always)]
    pub fn DAVAL(&mut self) -> DAVAL_W<'_, DEVR0_SPEC> {
        DAVAL_W::new(self, 0)
    }
    #[doc = "Bits 1:7 - 7-bit dynamic address When the I3C is acting as controller, this field can be written by software, for defining its own dynamic address. When the I3C is acting as target, this field is updated by hardware on the reception of either the broadcast ENTDAA CCC or the direct SETNEWDA CCC."]
    #[inline(always)]
    pub fn DA(&mut self) -> DA_W<'_, DEVR0_SPEC> {
        DA_W::new(self, 1)
    }
    #[doc = "Bit 16 - IBI request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISINT=1 (i.e. cleared) and the reception of ENEC CCC with ENINT=1 (i.e. set)."]
    #[inline(always)]
    pub fn IBIEN(&mut self) -> IBIEN_W<'_, DEVR0_SPEC> {
        IBIEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - controller-role request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISCR=1 (i.e. cleared) and the reception of ENEC CCC with ENCR=1 (i.e. set)."]
    #[inline(always)]
    pub fn CREN(&mut self) -> CREN_W<'_, DEVR0_SPEC> {
        CREN_W::new(self, 17)
    }
    #[doc = "Bit 19 - hot-join request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISHJ=1 (i.e. cleared) and the reception of ENEC CCC with ENHJ=1 (i.e. set)."]
    #[inline(always)]
    pub fn HJEN(&mut self) -> HJEN_W<'_, DEVR0_SPEC> {
        HJEN_W::new(self, 19)
    }
}
#[doc = "I3C own device characteristics register\n\nYou can [`read`](crate::Reg::read) this register and get [`devr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVR0_SPEC;
impl crate::RegisterSpec for DEVR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devr0::R`](R) reader structure"]
impl crate::Readable for DEVR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`devr0::W`](W) writer structure"]
impl crate::Writable for DEVR0_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DEVR0 to value 0"]
impl crate::Resettable for DEVR0_SPEC {}
