#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 28usize],
    #[doc = "0x1c - FMC and Efuse Status"]
    pub stat: STAT,
    _reserved1: [u8; 4usize],
    #[doc = "0x24 - Internal. Only to be used through TI provided API."]
    pub cfg: CFG,
    #[doc = "0x28 - Internal. Only to be used through TI provided API."]
    pub syscode_start: SYSCODE_START,
    #[doc = "0x2c - Internal. Only to be used through TI provided API."]
    pub flash_size: FLASH_SIZE,
    _reserved2: [u8; 12usize],
    #[doc = "0x3c - Internal. Only to be used through TI provided API."]
    pub fwlock: FWLOCK,
    #[doc = "0x40 - Internal. Only to be used through TI provided API."]
    pub fwflag: FWFLAG,
    _reserved3: [u8; 4028usize],
    #[doc = "0x1000 - Internal. Only to be used through TI provided API."]
    pub efuse: EFUSE,
    #[doc = "0x1004 - Internal. Only to be used through TI provided API."]
    pub efuseaddr: EFUSEADDR,
    #[doc = "0x1008 - Internal. Only to be used through TI provided API."]
    pub dataupper: DATAUPPER,
    #[doc = "0x100c - Internal. Only to be used through TI provided API."]
    pub datalower: DATALOWER,
    #[doc = "0x1010 - Internal. Only to be used through TI provided API."]
    pub efusecfg: EFUSECFG,
    #[doc = "0x1014 - Internal. Only to be used through TI provided API."]
    pub efusestat: EFUSESTAT,
    #[doc = "0x1018 - Internal. Only to be used through TI provided API."]
    pub acc: ACC,
    #[doc = "0x101c - Internal. Only to be used through TI provided API."]
    pub boundary: BOUNDARY,
    #[doc = "0x1020 - Internal. Only to be used through TI provided API."]
    pub efuseflag: EFUSEFLAG,
    #[doc = "0x1024 - Internal. Only to be used through TI provided API."]
    pub efusekey: EFUSEKEY,
    #[doc = "0x1028 - Internal. Only to be used through TI provided API."]
    pub efuserelease: EFUSERELEASE,
    #[doc = "0x102c - Internal. Only to be used through TI provided API."]
    pub efusepins: EFUSEPINS,
    #[doc = "0x1030 - Internal. Only to be used through TI provided API."]
    pub efusecra: EFUSECRA,
    #[doc = "0x1034 - Internal. Only to be used through TI provided API."]
    pub efuseread: EFUSEREAD,
    #[doc = "0x1038 - Internal. Only to be used through TI provided API."]
    pub efuseprogram: EFUSEPROGRAM,
    #[doc = "0x103c - Internal. Only to be used through TI provided API."]
    pub efuseerror: EFUSEERROR,
    #[doc = "0x1040 - Internal. Only to be used through TI provided API."]
    pub singlebit: SINGLEBIT,
    #[doc = "0x1044 - Internal. Only to be used through TI provided API."]
    pub twobit: TWOBIT,
    #[doc = "0x1048 - Internal. Only to be used through TI provided API."]
    pub selftestcyc: SELFTESTCYC,
    #[doc = "0x104c - Internal. Only to be used through TI provided API."]
    pub selftestsign: SELFTESTSIGN,
    _reserved4: [u8; 4016usize],
    #[doc = "0x2000 - Internal. Only to be used through TI provided API."]
    pub frdctl: FRDCTL,
    #[doc = "0x2004 - Internal. Only to be used through TI provided API."]
    pub fsprd: FSPRD,
    #[doc = "0x2008 - Internal. Only to be used through TI provided API."]
    pub fedacctl1: FEDACCTL1,
    #[doc = "0x200c - Internal. Only to be used through TI provided API."]
    pub fedacctl2: FEDACCTL2,
    #[doc = "0x2010 - Internal. Only to be used through TI provided API."]
    pub fcor_err_cnt: FCOR_ERR_CNT,
    #[doc = "0x2014 - Internal. Only to be used through TI provided API."]
    pub fcor_err_add: FCOR_ERR_ADD,
    #[doc = "0x2018 - Internal. Only to be used through TI provided API."]
    pub fcor_err_pos: FCOR_ERR_POS,
    #[doc = "0x201c - Internal. Only to be used through TI provided API."]
    pub fedacstat: FEDACSTAT,
    #[doc = "0x2020 - Internal. Only to be used through TI provided API."]
    pub func_err_add: FUNC_ERR_ADD,
    #[doc = "0x2024 - Internal. Only to be used through TI provided API."]
    pub fedacsdis: FEDACSDIS,
    #[doc = "0x2028 - Internal. Only to be used through TI provided API."]
    pub fprim_add_tag: FPRIM_ADD_TAG,
    #[doc = "0x202c - Internal. Only to be used through TI provided API."]
    pub fredu_add_tag: FREDU_ADD_TAG,
    #[doc = "0x2030 - Internal. Only to be used through TI provided API."]
    pub fbprot: FBPROT,
    #[doc = "0x2034 - Internal. Only to be used through TI provided API."]
    pub fbse: FBSE,
    #[doc = "0x2038 - Internal. Only to be used through TI provided API."]
    pub fbbusy: FBBUSY,
    #[doc = "0x203c - Internal. Only to be used through TI provided API."]
    pub fbac: FBAC,
    #[doc = "0x2040 - Internal. Only to be used through TI provided API."]
    pub fbfallback: FBFALLBACK,
    #[doc = "0x2044 - Internal. Only to be used through TI provided API."]
    pub fbprdy: FBPRDY,
    #[doc = "0x2048 - Internal. Only to be used through TI provided API."]
    pub fpac1: FPAC1,
    #[doc = "0x204c - Internal. Only to be used through TI provided API."]
    pub fpac2: FPAC2,
    #[doc = "0x2050 - Internal. Only to be used through TI provided API."]
    pub fmac: FMAC,
    #[doc = "0x2054 - Internal. Only to be used through TI provided API."]
    pub fmstat: FMSTAT,
    #[doc = "0x2058 - Internal. Only to be used through TI provided API."]
    pub femu_dmsw: FEMU_DMSW,
    #[doc = "0x205c - Internal. Only to be used through TI provided API."]
    pub femu_dlsw: FEMU_DLSW,
    #[doc = "0x2060 - Internal. Only to be used through TI provided API."]
    pub femu_ecc: FEMU_ECC,
    #[doc = "0x2064 - Internal. Only to be used through TI provided API."]
    pub flock: FLOCK,
    #[doc = "0x2068 - Internal. Only to be used through TI provided API."]
    pub femu_addr: FEMU_ADDR,
    #[doc = "0x206c - Internal. Only to be used through TI provided API."]
    pub fdiagctl: FDIAGCTL,
    #[doc = "0x2070 - Internal. Only to be used through TI provided API."]
    pub fraw_datah: FRAW_DATAH,
    #[doc = "0x2074 - Internal. Only to be used through TI provided API."]
    pub fraw_datal: FRAW_DATAL,
    #[doc = "0x2078 - Internal. Only to be used through TI provided API."]
    pub fraw_ecc: FRAW_ECC,
    #[doc = "0x207c - Internal. Only to be used through TI provided API."]
    pub fpar_ovr: FPAR_OVR,
    #[doc = "0x2080 - Internal. Only to be used through TI provided API."]
    pub fvreadct: FVREADCT,
    #[doc = "0x2084 - Internal. Only to be used through TI provided API."]
    pub fvhvct1: FVHVCT1,
    #[doc = "0x2088 - Internal. Only to be used through TI provided API."]
    pub fvhvct2: FVHVCT2,
    #[doc = "0x208c - Internal. Only to be used through TI provided API."]
    pub fvhvct3: FVHVCT3,
    #[doc = "0x2090 - Internal. Only to be used through TI provided API."]
    pub fvnvct: FVNVCT,
    #[doc = "0x2094 - Internal. Only to be used through TI provided API."]
    pub fvslp: FVSLP,
    #[doc = "0x2098 - Internal. Only to be used through TI provided API."]
    pub fvwlct: FVWLCT,
    #[doc = "0x209c - Internal. Only to be used through TI provided API."]
    pub fefusectl: FEFUSECTL,
    #[doc = "0x20a0 - Internal. Only to be used through TI provided API."]
    pub fefusestat: FEFUSESTAT,
    #[doc = "0x20a4 - Internal. Only to be used through TI provided API."]
    pub fefusedata: FEFUSEDATA,
    #[doc = "0x20a8 - Internal. Only to be used through TI provided API."]
    pub fseqpmp: FSEQPMP,
    #[doc = "0x20ac - Internal. Only to be used through TI provided API."]
    pub fclktrim: FCLKTRIM,
    #[doc = "0x20b0 - Internal. Only to be used through TI provided API."]
    pub rom_test: ROM_TEST,
    _reserved5: [u8; 12usize],
    #[doc = "0x20c0 - Internal. Only to be used through TI provided API."]
    pub fedacsdis2: FEDACSDIS2,
    _reserved6: [u8; 60usize],
    #[doc = "0x2100 - Internal. Only to be used through TI provided API."]
    pub fbstrobes: FBSTROBES,
    #[doc = "0x2104 - Internal. Only to be used through TI provided API."]
    pub fpstrobes: FPSTROBES,
    #[doc = "0x2108 - Internal. Only to be used through TI provided API."]
    pub fbmode: FBMODE,
    #[doc = "0x210c - Internal. Only to be used through TI provided API."]
    pub ftcr: FTCR,
    #[doc = "0x2110 - Internal. Only to be used through TI provided API."]
    pub faddr: FADDR,
    #[doc = "0x2114 - Internal. Only to be used through TI provided API."]
    pub fpmtctl: FPMTCTL,
    #[doc = "0x2118 - Internal. Only to be used through TI provided API."]
    pub pbistctl: PBISTCTL,
    #[doc = "0x211c - Internal. Only to be used through TI provided API."]
    pub ftctl: FTCTL,
    #[doc = "0x2120 - Internal. Only to be used through TI provided API."]
    pub fwpwrite0: FWPWRITE0,
    #[doc = "0x2124 - Internal. Only to be used through TI provided API."]
    pub fwpwrite1: FWPWRITE1,
    #[doc = "0x2128 - Internal. Only to be used through TI provided API."]
    pub fwpwrite2: FWPWRITE2,
    #[doc = "0x212c - Internal. Only to be used through TI provided API."]
    pub fwpwrite3: FWPWRITE3,
    #[doc = "0x2130 - Internal. Only to be used through TI provided API."]
    pub fwpwrite4: FWPWRITE4,
    #[doc = "0x2134 - Internal. Only to be used through TI provided API."]
    pub fwpwrite5: FWPWRITE5,
    #[doc = "0x2138 - Internal. Only to be used through TI provided API."]
    pub fwpwrite6: FWPWRITE6,
    #[doc = "0x213c - Internal. Only to be used through TI provided API."]
    pub fwpwrite7: FWPWRITE7,
    #[doc = "0x2140 - Internal. Only to be used through TI provided API."]
    pub fwpwrite_ecc: FWPWRITE_ECC,
    #[doc = "0x2144 - Internal. Only to be used through TI provided API."]
    pub fswstat: FSWSTAT,
    _reserved7: [u8; 184usize],
    #[doc = "0x2200 - Internal. Only to be used through TI provided API."]
    pub fsm_glbctl: FSM_GLBCTL,
    #[doc = "0x2204 - Internal. Only to be used through TI provided API."]
    pub fsm_state: FSM_STATE,
    #[doc = "0x2208 - Internal. Only to be used through TI provided API."]
    pub fsm_stat: FSM_STAT,
    #[doc = "0x220c - Internal. Only to be used through TI provided API."]
    pub fsm_cmd: FSM_CMD,
    #[doc = "0x2210 - Internal. Only to be used through TI provided API."]
    pub fsm_pe_osu: FSM_PE_OSU,
    #[doc = "0x2214 - Internal. Only to be used through TI provided API."]
    pub fsm_vstat: FSM_VSTAT,
    #[doc = "0x2218 - Internal. Only to be used through TI provided API."]
    pub fsm_pe_vsu: FSM_PE_VSU,
    #[doc = "0x221c - Internal. Only to be used through TI provided API."]
    pub fsm_cmp_vsu: FSM_CMP_VSU,
    #[doc = "0x2220 - Internal. Only to be used through TI provided API."]
    pub fsm_ex_val: FSM_EX_VAL,
    #[doc = "0x2224 - Internal. Only to be used through TI provided API."]
    pub fsm_rd_h: FSM_RD_H,
    #[doc = "0x2228 - Internal. Only to be used through TI provided API."]
    pub fsm_p_oh: FSM_P_OH,
    #[doc = "0x222c - Internal. Only to be used through TI provided API."]
    pub fsm_era_oh: FSM_ERA_OH,
    #[doc = "0x2230 - Internal. Only to be used through TI provided API."]
    pub fsm_sav_ppul: FSM_SAV_PPUL,
    #[doc = "0x2234 - Internal. Only to be used through TI provided API."]
    pub fsm_pe_vh: FSM_PE_VH,
    _reserved8: [u8; 8usize],
    #[doc = "0x2240 - Internal. Only to be used through TI provided API."]
    pub fsm_prg_pw: FSM_PRG_PW,
    #[doc = "0x2244 - Internal. Only to be used through TI provided API."]
    pub fsm_era_pw: FSM_ERA_PW,
    _reserved9: [u8; 12usize],
    #[doc = "0x2254 - Internal. Only to be used through TI provided API."]
    pub fsm_sav_era_pul: FSM_SAV_ERA_PUL,
    #[doc = "0x2258 - Internal. Only to be used through TI provided API."]
    pub fsm_timer: FSM_TIMER,
    #[doc = "0x225c - Internal. Only to be used through TI provided API."]
    pub fsm_mode: FSM_MODE,
    #[doc = "0x2260 - Internal. Only to be used through TI provided API."]
    pub fsm_pgm: FSM_PGM,
    #[doc = "0x2264 - Internal. Only to be used through TI provided API."]
    pub fsm_era: FSM_ERA,
    #[doc = "0x2268 - Internal. Only to be used through TI provided API."]
    pub fsm_prg_pul: FSM_PRG_PUL,
    #[doc = "0x226c - Internal. Only to be used through TI provided API."]
    pub fsm_era_pul: FSM_ERA_PUL,
    #[doc = "0x2270 - Internal. Only to be used through TI provided API."]
    pub fsm_step_size: FSM_STEP_SIZE,
    #[doc = "0x2274 - Internal. Only to be used through TI provided API."]
    pub fsm_pul_cntr: FSM_PUL_CNTR,
    #[doc = "0x2278 - Internal. Only to be used through TI provided API."]
    pub fsm_ec_step_height: FSM_EC_STEP_HEIGHT,
    #[doc = "0x227c - Internal. Only to be used through TI provided API."]
    pub fsm_st_machine: FSM_ST_MACHINE,
    #[doc = "0x2280 - Internal. Only to be used through TI provided API."]
    pub fsm_fles: FSM_FLES,
    _reserved10: [u8; 4usize],
    #[doc = "0x2288 - Internal. Only to be used through TI provided API."]
    pub fsm_wr_ena: FSM_WR_ENA,
    #[doc = "0x228c - Internal. Only to be used through TI provided API."]
    pub fsm_acc_pp: FSM_ACC_PP,
    #[doc = "0x2290 - Internal. Only to be used through TI provided API."]
    pub fsm_acc_ep: FSM_ACC_EP,
    _reserved11: [u8; 12usize],
    #[doc = "0x22a0 - Internal. Only to be used through TI provided API."]
    pub fsm_addr: FSM_ADDR,
    #[doc = "0x22a4 - Internal. Only to be used through TI provided API."]
    pub fsm_sector: FSM_SECTOR,
    #[doc = "0x22a8 - Internal. Only to be used through TI provided API."]
    pub fmc_rev_id: FMC_REV_ID,
    #[doc = "0x22ac - Internal. Only to be used through TI provided API."]
    pub fsm_err_addr: FSM_ERR_ADDR,
    #[doc = "0x22b0 - Internal. Only to be used through TI provided API."]
    pub fsm_pgm_maxpul: FSM_PGM_MAXPUL,
    #[doc = "0x22b4 - Internal. Only to be used through TI provided API."]
    pub fsm_execute: FSM_EXECUTE,
    #[doc = "0x22b8 - Internal. Only to be used through TI provided API."]
    pub eeprom_cfg: EEPROM_CFG,
    _reserved12: [u8; 4usize],
    #[doc = "0x22c0 - Internal. Only to be used through TI provided API."]
    pub fsm_sector1: FSM_SECTOR1,
    #[doc = "0x22c4 - Internal. Only to be used through TI provided API."]
    pub fsm_sector2: FSM_SECTOR2,
    _reserved13: [u8; 24usize],
    #[doc = "0x22e0 - Internal. Only to be used through TI provided API."]
    pub fsm_bsle0: FSM_BSLE0,
    #[doc = "0x22e4 - Internal. Only to be used through TI provided API."]
    pub fsm_bsle1: FSM_BSLE1,
    _reserved14: [u8; 8usize],
    #[doc = "0x22f0 - Internal. Only to be used through TI provided API."]
    pub fsm_bslp0: FSM_BSLP0,
    #[doc = "0x22f4 - Internal. Only to be used through TI provided API."]
    pub fsm_bslp1: FSM_BSLP1,
    _reserved15: [u8; 264usize],
    #[doc = "0x2400 - Internal. Only to be used through TI provided API."]
    pub fcfg_bank: FCFG_BANK,
    #[doc = "0x2404 - Internal. Only to be used through TI provided API."]
    pub fcfg_wrapper: FCFG_WRAPPER,
    #[doc = "0x2408 - Internal. Only to be used through TI provided API."]
    pub fcfg_bnk_type: FCFG_BNK_TYPE,
    _reserved16: [u8; 4usize],
    #[doc = "0x2410 - Internal. Only to be used through TI provided API."]
    pub fcfg_b0_start: FCFG_B0_START,
    #[doc = "0x2414 - Internal. Only to be used through TI provided API."]
    pub fcfg_b1_start: FCFG_B1_START,
    #[doc = "0x2418 - Internal. Only to be used through TI provided API."]
    pub fcfg_b2_start: FCFG_B2_START,
    #[doc = "0x241c - Internal. Only to be used through TI provided API."]
    pub fcfg_b3_start: FCFG_B3_START,
    #[doc = "0x2420 - Internal. Only to be used through TI provided API."]
    pub fcfg_b4_start: FCFG_B4_START,
    #[doc = "0x2424 - Internal. Only to be used through TI provided API."]
    pub fcfg_b5_start: FCFG_B5_START,
    #[doc = "0x2428 - Internal. Only to be used through TI provided API."]
    pub fcfg_b6_start: FCFG_B6_START,
    #[doc = "0x242c - Internal. Only to be used through TI provided API."]
    pub fcfg_b7_start: FCFG_B7_START,
    #[doc = "0x2430 - Internal. Only to be used through TI provided API."]
    pub fcfg_b0_ssize0: FCFG_B0_SSIZE0,
    #[doc = "0x2434 - Internal. Only to be used through TI provided API."]
    pub fcfg_b0_ssize1: FCFG_B0_SSIZE1,
    #[doc = "0x2438 - Internal. Only to be used through TI provided API."]
    pub fcfg_b0_ssize2: FCFG_B0_SSIZE2,
    #[doc = "0x243c - Internal. Only to be used through TI provided API."]
    pub fcfg_b0_ssize3: FCFG_B0_SSIZE3,
    #[doc = "0x2440 - Internal. Only to be used through TI provided API."]
    pub fcfg_b1_ssize0: FCFG_B1_SSIZE0,
    #[doc = "0x2444 - Internal. Only to be used through TI provided API."]
    pub fcfg_b1_ssize1: FCFG_B1_SSIZE1,
    #[doc = "0x2448 - Internal. Only to be used through TI provided API."]
    pub fcfg_b1_ssize2: FCFG_B1_SSIZE2,
    #[doc = "0x244c - Internal. Only to be used through TI provided API."]
    pub fcfg_b1_ssize3: FCFG_B1_SSIZE3,
    #[doc = "0x2450 - Internal. Only to be used through TI provided API."]
    pub fcfg_b2_ssize0: FCFG_B2_SSIZE0,
    #[doc = "0x2454 - Internal. Only to be used through TI provided API."]
    pub fcfg_b2_ssize1: FCFG_B2_SSIZE1,
    #[doc = "0x2458 - Internal. Only to be used through TI provided API."]
    pub fcfg_b2_ssize2: FCFG_B2_SSIZE2,
    #[doc = "0x245c - Internal. Only to be used through TI provided API."]
    pub fcfg_b2_ssize3: FCFG_B2_SSIZE3,
    #[doc = "0x2460 - Internal. Only to be used through TI provided API."]
    pub fcfg_b3_ssize0: FCFG_B3_SSIZE0,
    #[doc = "0x2464 - Internal. Only to be used through TI provided API."]
    pub fcfg_b3_ssize1: FCFG_B3_SSIZE1,
    #[doc = "0x2468 - Internal. Only to be used through TI provided API."]
    pub fcfg_b3_ssize2: FCFG_B3_SSIZE2,
    #[doc = "0x246c - Internal. Only to be used through TI provided API."]
    pub fcfg_b3_ssize3: FCFG_B3_SSIZE3,
    #[doc = "0x2470 - Internal. Only to be used through TI provided API."]
    pub fcfg_b4_ssize0: FCFG_B4_SSIZE0,
    #[doc = "0x2474 - Internal. Only to be used through TI provided API."]
    pub fcfg_b4_ssize1: FCFG_B4_SSIZE1,
    #[doc = "0x2478 - Internal. Only to be used through TI provided API."]
    pub fcfg_b4_ssize2: FCFG_B4_SSIZE2,
    #[doc = "0x247c - Internal. Only to be used through TI provided API."]
    pub fcfg_b4_ssize3: FCFG_B4_SSIZE3,
    #[doc = "0x2480 - Internal. Only to be used through TI provided API."]
    pub fcfg_b5_ssize0: FCFG_B5_SSIZE0,
    #[doc = "0x2484 - Internal. Only to be used through TI provided API."]
    pub fcfg_b5_ssize1: FCFG_B5_SSIZE1,
    #[doc = "0x2488 - Internal. Only to be used through TI provided API."]
    pub fcfg_b5_ssize2: FCFG_B5_SSIZE2,
    #[doc = "0x248c - Internal. Only to be used through TI provided API."]
    pub fcfg_b5_ssize3: FCFG_B5_SSIZE3,
    #[doc = "0x2490 - Internal. Only to be used through TI provided API."]
    pub fcfg_b6_ssize0: FCFG_B6_SSIZE0,
    #[doc = "0x2494 - Internal. Only to be used through TI provided API."]
    pub fcfg_b6_ssize1: FCFG_B6_SSIZE1,
    #[doc = "0x2498 - Internal. Only to be used through TI provided API."]
    pub fcfg_b6_ssize2: FCFG_B6_SSIZE2,
    #[doc = "0x249c - Internal. Only to be used through TI provided API."]
    pub fcfg_b6_ssize3: FCFG_B6_SSIZE3,
    #[doc = "0x24a0 - Internal. Only to be used through TI provided API."]
    pub fcfg_b7_ssize0: FCFG_B7_SSIZE0,
    #[doc = "0x24a4 - Internal. Only to be used through TI provided API."]
    pub fcfg_b7_ssize1: FCFG_B7_SSIZE1,
    #[doc = "0x24a8 - Internal. Only to be used through TI provided API."]
    pub fcfg_b7_ssize2: FCFG_B7_SSIZE2,
    #[doc = "0x24ac - Internal. Only to be used through TI provided API."]
    pub fcfg_b7_ssize3: FCFG_B7_SSIZE3,
}
#[doc = "FMC and Efuse Status"]
pub struct STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FMC and Efuse Status"]
pub mod stat;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod cfg;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct SYSCODE_START {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod syscode_start;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FLASH_SIZE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_size;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FWLOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fwlock;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FWFLAG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fwflag;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct EFUSE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efuse;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct EFUSEADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efuseaddr;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct DATAUPPER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod dataupper;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct DATALOWER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod datalower;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct EFUSECFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efusecfg;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct EFUSESTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efusestat;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct ACC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod acc;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct BOUNDARY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod boundary;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct EFUSEFLAG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efuseflag;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct EFUSEKEY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efusekey;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct EFUSERELEASE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efuserelease;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct EFUSEPINS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efusepins;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct EFUSECRA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efusecra;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct EFUSEREAD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efuseread;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct EFUSEPROGRAM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efuseprogram;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct EFUSEERROR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efuseerror;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct SINGLEBIT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod singlebit;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct TWOBIT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod twobit;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct SELFTESTCYC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod selftestcyc;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct SELFTESTSIGN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod selftestsign;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FRDCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod frdctl;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FSPRD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsprd;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FEDACCTL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fedacctl1;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FEDACCTL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fedacctl2;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FCOR_ERR_CNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcor_err_cnt;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FCOR_ERR_ADD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcor_err_add;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FCOR_ERR_POS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcor_err_pos;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FEDACSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fedacstat;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FUNC_ERR_ADD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod func_err_add;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FEDACSDIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fedacsdis;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FPRIM_ADD_TAG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fprim_add_tag;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FREDU_ADD_TAG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fredu_add_tag;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FBPROT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fbprot;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FBSE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fbse;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FBBUSY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fbbusy;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FBAC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fbac;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FBFALLBACK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fbfallback;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FBPRDY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fbprdy;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FPAC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fpac1;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FPAC2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fpac2;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FMAC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fmac;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FMSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fmstat;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FEMU_DMSW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod femu_dmsw;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FEMU_DLSW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod femu_dlsw;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FEMU_ECC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod femu_ecc;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FLOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flock;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FEMU_ADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod femu_addr;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FDIAGCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fdiagctl;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FRAW_DATAH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fraw_datah;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FRAW_DATAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fraw_datal;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FRAW_ECC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fraw_ecc;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FPAR_OVR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fpar_ovr;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FVREADCT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fvreadct;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FVHVCT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fvhvct1;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FVHVCT2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fvhvct2;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FVHVCT3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fvhvct3;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FVNVCT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fvnvct;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FVSLP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fvslp;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FVWLCT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fvwlct;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FEFUSECTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fefusectl;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FEFUSESTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fefusestat;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FEFUSEDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fefusedata;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FSEQPMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fseqpmp;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FCLKTRIM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fclktrim;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct ROM_TEST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod rom_test;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FEDACSDIS2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fedacsdis2;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FBSTROBES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fbstrobes;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FPSTROBES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fpstrobes;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FBMODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fbmode;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FTCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod ftcr;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod faddr;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FPMTCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fpmtctl;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct PBISTCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod pbistctl;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FTCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod ftctl;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FWPWRITE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fwpwrite0;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FWPWRITE1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fwpwrite1;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FWPWRITE2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fwpwrite2;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FWPWRITE3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fwpwrite3;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FWPWRITE4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fwpwrite4;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FWPWRITE5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fwpwrite5;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FWPWRITE6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fwpwrite6;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FWPWRITE7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fwpwrite7;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FWPWRITE_ECC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fwpwrite_ecc;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FSWSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fswstat;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FSM_GLBCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_glbctl;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FSM_STATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_state;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FSM_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_stat;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FSM_CMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_cmd;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FSM_PE_OSU {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_pe_osu;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FSM_VSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_vstat;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FSM_PE_VSU {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_pe_vsu;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FSM_CMP_VSU {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_cmp_vsu;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FSM_EX_VAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_ex_val;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FSM_RD_H {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_rd_h;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FSM_P_OH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_p_oh;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FSM_ERA_OH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_era_oh;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FSM_SAV_PPUL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_sav_ppul;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FSM_PE_VH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_pe_vh;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FSM_PRG_PW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_prg_pw;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FSM_ERA_PW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_era_pw;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FSM_SAV_ERA_PUL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_sav_era_pul;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FSM_TIMER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_timer;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FSM_MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_mode;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FSM_PGM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_pgm;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FSM_ERA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_era;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FSM_PRG_PUL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_prg_pul;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FSM_ERA_PUL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_era_pul;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FSM_STEP_SIZE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_step_size;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FSM_PUL_CNTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_pul_cntr;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FSM_EC_STEP_HEIGHT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_ec_step_height;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FSM_ST_MACHINE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_st_machine;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FSM_FLES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_fles;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FSM_WR_ENA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_wr_ena;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FSM_ACC_PP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_acc_pp;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FSM_ACC_EP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_acc_ep;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FSM_ADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_addr;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FSM_SECTOR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_sector;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FMC_REV_ID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fmc_rev_id;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FSM_ERR_ADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_err_addr;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FSM_PGM_MAXPUL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_pgm_maxpul;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FSM_EXECUTE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_execute;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct EEPROM_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod eeprom_cfg;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FSM_SECTOR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_sector1;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FSM_SECTOR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_sector2;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FSM_BSLE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_bsle0;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FSM_BSLE1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_bsle1;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FSM_BSLP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_bslp0;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FSM_BSLP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_bslp1;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FCFG_BANK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_bank;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FCFG_WRAPPER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_wrapper;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FCFG_BNK_TYPE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_bnk_type;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FCFG_B0_START {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b0_start;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FCFG_B1_START {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b1_start;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FCFG_B2_START {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b2_start;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FCFG_B3_START {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b3_start;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FCFG_B4_START {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b4_start;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FCFG_B5_START {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b5_start;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FCFG_B6_START {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b6_start;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FCFG_B7_START {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b7_start;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FCFG_B0_SSIZE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b0_ssize0;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FCFG_B0_SSIZE1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b0_ssize1;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FCFG_B0_SSIZE2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b0_ssize2;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FCFG_B0_SSIZE3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b0_ssize3;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FCFG_B1_SSIZE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b1_ssize0;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FCFG_B1_SSIZE1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b1_ssize1;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FCFG_B1_SSIZE2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b1_ssize2;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FCFG_B1_SSIZE3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b1_ssize3;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FCFG_B2_SSIZE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b2_ssize0;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FCFG_B2_SSIZE1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b2_ssize1;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FCFG_B2_SSIZE2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b2_ssize2;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FCFG_B2_SSIZE3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b2_ssize3;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FCFG_B3_SSIZE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b3_ssize0;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FCFG_B3_SSIZE1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b3_ssize1;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FCFG_B3_SSIZE2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b3_ssize2;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FCFG_B3_SSIZE3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b3_ssize3;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FCFG_B4_SSIZE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b4_ssize0;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FCFG_B4_SSIZE1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b4_ssize1;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FCFG_B4_SSIZE2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b4_ssize2;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FCFG_B4_SSIZE3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b4_ssize3;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FCFG_B5_SSIZE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b5_ssize0;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FCFG_B5_SSIZE1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b5_ssize1;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FCFG_B5_SSIZE2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b5_ssize2;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FCFG_B5_SSIZE3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b5_ssize3;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FCFG_B6_SSIZE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b6_ssize0;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FCFG_B6_SSIZE1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b6_ssize1;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FCFG_B6_SSIZE2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b6_ssize2;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FCFG_B6_SSIZE3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b6_ssize3;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FCFG_B7_SSIZE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b7_ssize0;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FCFG_B7_SSIZE1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b7_ssize1;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FCFG_B7_SSIZE2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b7_ssize2;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FCFG_B7_SSIZE3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b7_ssize3;
