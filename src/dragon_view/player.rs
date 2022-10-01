pub struct Player {
    pub level: u8, //0-45 (1-45 + master)
    //attack = level * 4
    //defense = level * 2
}

impl Default for Player {
    fn default() -> Self {
        Self {
            level: 0,
        }
    }
}

impl Player {
    pub fn calc_xp_req() {
        let xp_req: [u16; 45] = [
            0x0004, 0x000D, 0x0020, 0x003E, 0x006C, 0x00AB, 0x0100, 0x016C, 0x01F4,
            0x0299, 0x0360, 0x044A, 0x055C, 0x0697, 0x0800, 0x0998, 0x0B64, 0x0D65,
            0x0FA0, 0x1216, 0x14CC, 0x17C3, 0x1B00, 0x1E84, 0x2254, 0x2671, 0x2AE0,
            0x2FA2, 0x34BC, 0x3A2F, 0x4000, 0x4630, 0x4CC4, 0x53BD, 0x5B20, 0x62EE,
            0x6B2C, 0x73DB, 0x7D00, 0x869C, 0x90B4, 0x9B49, 0xA660, 0xB1FA, 0xBE1C,
        ];
    
        for (lv, &req_table) in xp_req.iter().enumerate() {
            let extra = if lv >= 34 {
                let req_reduction = match lv == 44 {
                    true  => 0,
                    false => 1,
                };
    
                (lv as u32 - 32 - req_reduction) * req_table as u32
            } else {
                0
            };
    
            let req = ((req_table as u32) << 1) + extra;
    
            println!("{}: {}", lv+1, req);
        }
    }
}
