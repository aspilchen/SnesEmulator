var searchIndex = new Map(JSON.parse('[\
["main",{"doc":"","t":"H","n":["main"],"q":[[0,"main"]],"d":[""],"i":[0],"f":[[[],1]],"c":[],"p":[[1,"tuple"]],"b":[]}],\
["snes",{"doc":"","t":"CCCCCCHHHHHHHHHHHHHHHHHHHGPPPNNNHHHNCHHNNNHHCCSSSSSHHCSSSSHSSSSSHHCSSSSHCSSSSHTFTTTTFTTTOHNNNNNNNNNNNNNNNNNOOONONNNNNNNNNNNNHNNNNNNNNNNCCNONOOCCOHCNNNNNNNNNONOHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHFNNNNNCHHNNNCCHHHHHHHHFNNONHNNHOHHHNNN","n":["address_mode","cartridge","cpu","instructions","memory","snes","absolute","absolute_indexed_indirect","absolute_indexed_x","absolute_indexed_y","absolute_indirect","absolute_long","absolute_long_indexed","direct","direct_indexed_indirect","direct_indexed_x","direct_indexed_y","direct_indirect","direct_indirect_indexed","direct_indirect_long","immediate16","immediate8","pc_relative","pc_relative_long","stack_relative","CartridgeType","HiRom","LoRom","None","borrow","borrow_mut","from","get_bank","get_offset","get_page","into","lo_rom","make_address","split_addr","try_from","try_into","type_id","map_address","reserve_memory","rom","wram","BLOCK_SIZE","FIRST_BANK","LAST_BANK","MAX_SIZE","OFFSET","in_bounds","map_address","mirror","BLOCK_SIZE","FIRST_BANK","LAST_BANK","OFFSET","in_bounds","BANK_SHIFT","BLOCK_SIZE","FIRST_BANK","LAST_BANK","MAX_SIZE","in_bounds","map_address","mirror","BLOCK_SIZE","FIRST_BANK","LAST_BANK","REFLECT_BANK","in_bounds","second_mirror","BLOCK_SIZE","FIRST_BANK","LAST_BANK","REFLECT_BANK","in_bounds","C","Cpu","D","I","M","N","Status","V","XB","Z","a","add_cycles","all","and","bitand","bitand_assign","bitor","bitor_assign","bits","bitxor","bitxor_assign","borrow","borrow","borrow_mut","borrow_mut","clone","clone_into","cmp","contains","cycles","d","dbr","default","e","eq","eq","fmt","fmt","fmt","fmt","fmt","fmt","from","from","from","full","get_pc_address","hash","intersects","into","into","is_all","is_full","is_none","none","not","not","ops_16","ops_8","or","p","partial_cmp","pbr","pc","registers_16","registers_8","s","set_pc_address","status_flags","to_owned","to_string","truncate","try_from","try_from","try_into","try_into","type_id","type_id","x","xor","y","adc","and","asl","bit","compare","dec","eor","inc","lda","ldx","ldy","lsr","ora","rol","ror","sbc","set_zn","trb","tsb","adc","and","asl","bit","compare","dec","eor","inc","lda","ldx","ldy","lsr","ora","rol","ror","sbc","set_zn","trb","tsb","get_a","get_d","get_x","get_y","set_a","set_d","set_x","set_y","get_a","get_dbr","get_pbr","get_x","get_y","set_a","set_dbr","set_pbr","set_x","set_y","clear_status_bits","is_set_c","is_set_m","is_set_n","is_set_v","is_set_x","is_set_z","set_c","set_d","set_i","set_m","set_n","set_status_bits","set_v","set_x","set_z","adc_61","adc_63","adc_65","adc_67","adc_69","adc_6d","adc_6f","adc_71","adc_72","adc_73","adc_75","adc_77","adc_79","adc_7d","adc_7f","and_21","and_23","and_25","and_27","and_29","and_2d","and_2f","and_31","and_32","and_33","and_35","and_37","and_39","and_3d","and_3f","asl_06","asl_0a","asl_0e","asl_16","asl_1e","bcc_90","bcs_b0","beq_f0","bit_24","bit_2c","bit_34","bit_3c","bit_89","bmi_30","bne_d0","bpl_10","bra_80","brk_00","brl_82","bvc_50","bvs_70","clc_18","cld_d8","cli_58","clv_b8","cmp_c1","cmp_c3","cmp_c5","cmp_c7","cmp_c9","cmp_cd","cmp_cf","cmp_d1","cmp_d2","cmp_d3","cmp_d5","cmp_d7","cmp_d9","cmp_dd","cmp_df","cop_02","cpx_e0","cpx_e4","cpx_ec","cpy_c0","cpy_c4","cpy_cc","dec_3a","dec_c6","dec_ce","dec_d6","dec_de","dex_ca","dey_88","eor_41","eor_43","eor_45","eor_47","eor_49","eor_4d","eor_4f","eor_51","eor_52","eor_53","eor_55","eor_57","eor_59","eor_5d","eor_5f","inc_1a","inc_e6","inc_ee","inc_f6","inc_fe","inx_e8","iny_c8","jmp_4c","jmp_5c","jmp_6c","jmp_7c","jmp_dc","jsr_20","jsr_22","jsr_fc","lda_a1","lda_a3","lda_a5","lda_a7","lda_a9","lda_ad","lda_af","lda_b1","lda_b2","lda_b3","lda_b5","lda_b7","lda_b9","lda_bd","lda_bf","ldx_a2","ldx_a6","ldx_ae","ldx_b6","ldx_be","ldy_a0","ldy_a4","ldy_ac","ldy_b4","ldy_bc","lsr_46","lsr_4a","lsr_4e","lsr_56","lsr_5e","mvn_54","mvp_44","nop_ea","ora_01","ora_03","ora_05","ora_07","ora_09","ora_0d","ora_0f","ora_11","ora_12","ora_13","ora_15","ora_17","ora_19","ora_1d","ora_1f","pea_f4","pei_d4","per_62","pha_48","phb_8b","phd_0b","phk_4b","php_08","phx_da","phy_5a","pla_68","plb_ab","pld_2b","plp_28","plx_fa","ply_7a","rep_c2","rol_26","rol_2a","rol_2e","rol_36","rol_3e","ror_66","ror_6a","ror_6e","ror_76","ror_7e","rti_40","rtl_6b","rts_60","sbc_e1","sbc_e3","sbc_e5","sbc_e7","sbc_e9","sbc_ed","sbc_ef","sbc_f1","sbc_f2","sbc_f3","sbc_f5","sbc_f7","sbc_f9","sbc_fd","sbc_ff","sec_38","sed_f8","sei_78","sep_e2","sta_81","sta_83","sta_85","sta_87","sta_8d","sta_8f","sta_91","sta_92","sta_93","sta_95","sta_97","sta_99","sta_9d","sta_9f","stp_db","stx_86","stx_8e","stx_96","sty_84","sty_8c","sty_94","stz_64","stz_74","stz_9c","stz_9e","tax_aa","tay_a8","tcd_5b","tcs_1b","tdc_7b","trb_14","trb_1c","tsb_04","tsb_0c","tsc_3b","tsx_ba","txa_8a","txs_9a","txy_9b","tya_98","tyx_bb","wai_cb","wdm_42","xba_eb","xce_fb","Memory","borrow","borrow_mut","default","from","into","io","load_binary","map_address","try_from","try_into","type_id","byte","word","fetch","read","read_nomap","write","fetch","read","read_nomap","write","Snes","borrow","borrow_mut","cpu","default","execute","from","into","load_binary","mem","reset","run","run_for","try_from","try_into","type_id"],"q":[[0,"snes"],[6,"snes::address_mode"],[25,"snes::cartridge"],[42,"snes::cartridge::lo_rom"],[46,"snes::cartridge::lo_rom::rom"],[54,"snes::cartridge::lo_rom::rom::mirror"],[59,"snes::cartridge::lo_rom::wram"],[67,"snes::cartridge::lo_rom::wram::mirror"],[73,"snes::cartridge::lo_rom::wram::mirror::second_mirror"],[78,"snes::cpu"],[159,"snes::cpu::ops_16"],[178,"snes::cpu::ops_8"],[197,"snes::cpu::registers_16"],[205,"snes::cpu::registers_8"],[215,"snes::cpu::status_flags"],[231,"snes::instructions"],[487,"snes::memory"],[499,"snes::memory::io"],[501,"snes::memory::io::byte"],[505,"snes::memory::io::word"],[509,"snes::snes"],[525,"core::result"],[526,"core::any"],[527,"alloc::vec"],[528,"core::cmp"],[529,"core::fmt"],[530,"core::fmt"],[531,"core::option"],[532,"alloc::string"]],"d":["Snes Addressing Modes","Cartridge Memory mapping","CPU State","opcode implementation","System Memory","","Absolute","Todo","Todo","Todo","Todo","Todo","Todo","Direct Page","Todo","Todo","Todo","Todo","Todo","Todo","Immediate","","Program counter relative","Todo","stack pointer relative","","","","","","","Returns the argument unchanged.","","","","Calls <code>U::from(self)</code>.","LoRom Memory Map","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Carry","Cpu State","Decimal mode","IRQ","Memory mode","Negative","Status flags for the CPU","Overflow","In native mode","Zero","Accumulator","","Returns a bitmask that contains all values.","Returns the bitwise AND of the bitmask.","","","","","Returns the underlying bits of the bitmask.","","","","","","","","","","Returns <code>true</code> if <code>self</code> contains all values of <code>other</code>.","Cycle Count","Direct Page Register","Data Bank","","Emulation Flag","","","","","","","","","Returns the argument unchanged.","","Returns the argument unchanged.","Returns a bitmask that contains all flags.","","","Returns <code>true</code> if <code>self</code> intersects with any value in <code>other</code>, …","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Returns <code>true</code> if the bitmask contains all values.","Returns <code>true</code> if the bitmask contains all flags.","Returns <code>true</code> if the bitmask does not contain any values.","Returns a bitmask that does not contain any values.","","Returns the bitwise NOT of the bitmask.","16 bit operations","8 bit operations","Returns the bitwise OR of the bitmask.","Status Register","","Program Bank","Program Counter","Setters/getters for 16 bit registers","Setters/getters for 8 bit registers","Stack Pointer","","Set/Get status bits","","","Returns a bitmask that only has bits corresponding to flags","","","","","","","Index","Returns the bitwise XOR of the bitmask.","Index","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Memory I/O","","","","","","8 bit operations","16 bit operations","","Read using address mapping","Read without address mapping","Write using address mapping","","Read using address mapping","Read without address mapping","Write using address mapping","","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","","","","",""],"i":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,20,20,20,20,20,20,0,0,0,20,0,0,0,20,20,20,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,12,0,12,12,12,12,0,12,12,12,10,0,12,12,12,12,12,12,12,12,12,10,12,10,12,12,12,12,12,10,10,10,10,10,12,12,10,12,12,12,12,12,10,12,12,12,0,12,12,10,12,12,12,12,12,12,12,0,0,12,10,12,10,10,0,0,10,0,0,12,10,12,10,12,10,12,10,12,10,12,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,19,19,19,19,19,0,0,0,19,19,19,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1,0,1,1,0,1,0,0,0,1,1,1],"f":[0,0,0,0,0,0,[1,2],[1,2],[1,2],[1,2],[1,2],[1,2],[1,2],[1,2],[1,2],[1,2],[1,2],[1,2],[1,2],[1,2],[1,2],[1,2],[1,2],[1,2],[1,2],0,0,0,0,[-1,-2,[],[]],[-1,-2,[],[]],[-1,-1,[]],[2,3],[2,4],[2,2],[-1,-2,[],[]],0,[[3,4],2],[2,[[5,[3,4]]]],[-1,[[6,[-2]]],[],[]],[-1,[[6,[-2]]],[],[]],[-1,7,[]],[2,2],[[],[[8,[3]]]],0,0,0,0,0,0,0,[[3,4],9],[2,2],0,0,0,0,0,[[3,4],9],0,0,0,0,0,[[3,4],9],[2,2],0,0,0,0,0,[[3,4],9],0,0,0,0,0,[[3,4],9],0,0,0,0,0,0,0,0,0,0,0,[[10,11],5],[[],12],[[12,12],12],[[12,12],-1,[]],[[12,12],5],[[12,12],-1,[]],[[12,12],5],[12,3],[[12,12],-1,[]],[[12,12],5],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[12,12],[[-1,-2],5,[],[]],[[12,12],13],[[12,12],9],0,0,0,[[],10],0,[[12,12],9],[[12,3],9],[[10,14],15],[[12,14],15],[[12,14],15],[[12,14],15],[[12,14],15],[[12,14],15],[-1,-1,[]],[3,12],[-1,-1,[]],[[],12],[10,2],[[12,-1],5,16],[[12,12],9],[-1,-2,[],[]],[-1,-2,[],[]],[12,9],[12,9],[12,9],[[],12],[12,-1,[]],[12,12],0,0,[[12,12],12],0,[[12,12],[[17,[13]]]],0,0,0,0,0,[[10,2],5],0,[-1,-2,[],[]],[-1,18,[]],[12,12],[-1,[[6,[-2]]],[],[]],[-1,[[6,[-2]]],[],[]],[-1,[[6,[-2]]],[],[]],[-1,[[6,[-2]]],[],[]],[-1,7,[]],[-1,7,[]],0,[[12,12],12],0,[[10,4],4],[[10,4],4],[[10,4],4],[[10,4],4],[[10,4,4],5],[[10,4],4],[[10,4],4],[[10,4],4],[[10,4],5],[[10,4],5],[[10,4],5],[[10,4],4],[[10,4],4],[[10,4],4],[[10,4],4],[[10,4],4],[[10,4],5],[[10,4],4],[[10,4],4],[[10,3],3],[[10,3],3],[[10,3],3],[[10,3],3],[[10,3,3],5],[[10,3],3],[[10,3],3],[[10,3],3],[[10,3],5],[[10,3],5],[[10,3],5],[[10,3],3],[[10,3],3],[[10,3],3],[[10,3],3],[[10,3],3],[[10,3],5],[[10,3],3],[[10,3],3],[10,4],[10,4],[10,4],[10,4],[[10,4],5],[[10,4],5],[[10,4],5],[[10,4],5],[10,3],[10,3],[10,3],[10,3],[10,3],[[10,3],5],[[10,3],5],[[10,3],5],[[10,3],5],[[10,3],5],[[10,3],5],[10,9],[10,9],[10,9],[10,9],[10,9],[10,9],[[10,9],5],[[10,9],5],[[10,9],5],[[10,9],5],[[10,9],5],[[10,3],5],[[10,9],5],[[10,9],5],[[10,9],5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],[1,5],0,[-1,-2,[],[]],[-1,-2,[],[]],[[],19],[-1,-1,[]],[-1,-2,[],[]],0,[[19,[8,[3]]],5],[[2,20],2],[-1,[[6,[-2]]],[],[]],[-1,[[6,[-2]]],[],[]],[-1,7,[]],0,0,[[19,10],3],[[19,2],3],[[19,2],3],[[19,2,3],5],[[19,10],4],[[19,2],4],[[19,2],4],[[19,2,4],5],0,[-1,-2,[],[]],[-1,-2,[],[]],0,[[],1],[[1,3],5],[-1,-1,[]],[-1,-2,[],[]],[[1,[8,[3]]],5],0,[1,5],[1,5],[[1,11],5],[-1,[[6,[-2]]],[],[]],[-1,[[6,[-2]]],[],[]],[-1,7,[]]],"c":[],"p":[[5,"Snes",509],[1,"usize"],[1,"u8"],[1,"u16"],[1,"tuple"],[6,"Result",525],[5,"TypeId",526],[5,"Vec",527],[1,"bool"],[5,"Cpu",78],[1,"i32"],[5,"Status",78],[6,"Ordering",528],[5,"Formatter",529],[8,"Result",529],[10,"Hasher",530],[6,"Option",531],[5,"String",532],[5,"Memory",487],[6,"CartridgeType",25]],"b":[[112,"impl-PartialEq-for-Status"],[113,"impl-PartialEq%3Cu8%3E-for-Status"],[115,"impl-LowerHex-for-Status"],[116,"impl-UpperHex-for-Status"],[117,"impl-Debug-for-Status"],[118,"impl-Octal-for-Status"],[119,"impl-Binary-for-Status"],[133,"impl-Not-for-Status"],[134,"impl-Status"]]}]\
]'));
if (typeof exports !== 'undefined') exports.searchIndex = searchIndex;
else if (window.initSearch) window.initSearch(searchIndex);
