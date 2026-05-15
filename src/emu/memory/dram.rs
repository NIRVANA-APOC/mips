use colored::Colorize;

const BURST_LEN: usize = 8;
const BURST_MASK: usize = BURST_LEN - 1;

const COL_WIDTH: usize = 10;
const ROW_WIDTH: usize = 10;
const BANK_WIDTH: usize = 3;
const RANK_WIDTH: usize = 29 - COL_WIDTH - ROW_WIDTH - BANK_WIDTH;

const NR_COL: usize = 1 << COL_WIDTH;
const NR_ROW: usize = 1 << ROW_WIDTH;
const NR_BANK: usize = 1 << BANK_WIDTH;
const NR_RANK: usize = 1 << RANK_WIDTH;

const HW_MEM_SIZE: usize = 1 << (COL_WIDTH + ROW_WIDTH + BANK_WIDTH + RANK_WIDTH);

#[derive(Clone)]
struct RowBuf {
    buf: Vec<u8>,
    row_idx: i32,
    valid: bool,
}

impl RowBuf {
    fn new() -> Self {
        Self {
            buf: vec![0; NR_COL],
            row_idx: 0,
            valid: false,
        }
    }
}

/// DDR3 DRAM 模拟器，含 row buffer 机制。
pub struct Memory {
    dram: Vec<u8>,
    row_bufs: Vec<Vec<RowBuf>>,
}

impl Memory {
    pub fn new() -> Self {
        let mut row_bufs = Vec::with_capacity(NR_RANK);
        for _ in 0..NR_RANK {
            let mut bank = Vec::with_capacity(NR_BANK);
            for _ in 0..NR_BANK {
                bank.push(RowBuf::new());
            }
            row_bufs.push(bank);
        }
        Self {
            dram: vec![0; HW_MEM_SIZE],
            row_bufs,
        }
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.dram.fill(0);
        for rank in &mut self.row_bufs {
            for buf in rank {
                buf.valid = false;
            }
        }
    }

    fn dram_addr(addr: u32) -> (usize, usize, usize, usize) {
        let col = (addr as usize) & ((1 << COL_WIDTH) - 1);
        let row = ((addr as usize) >> COL_WIDTH) & ((1 << ROW_WIDTH) - 1);
        let bank = ((addr as usize) >> (COL_WIDTH + ROW_WIDTH)) & ((1 << BANK_WIDTH) - 1);
        let rank =
            ((addr as usize) >> (COL_WIDTH + ROW_WIDTH + BANK_WIDTH)) & ((1 << RANK_WIDTH) - 1);
        (rank, bank, row, col)
    }

    fn dram_offset(rank: usize, bank: usize, row: usize) -> usize {
        ((rank * NR_BANK + bank) * NR_ROW + row) * NR_COL
    }

    fn ddr3_read(&mut self, addr: u32, data: &mut [u8]) {
        assert!(
            addr < HW_MEM_SIZE as u32,
            "{}",
            format!(
                "physical address {:08x} is outside of the physical memory!",
                addr
            )
            .red()
        );
        assert_eq!(data.len(), BURST_LEN);

        let aligned = addr & !(BURST_MASK as u32);
        let (rank, bank, row, col) = Self::dram_addr(aligned);

        let rb = &mut self.row_bufs[rank][bank];
        if !(rb.valid && rb.row_idx == row as i32) {
            let off = Self::dram_offset(rank, bank, row);
            rb.buf.copy_from_slice(&self.dram[off..off + NR_COL]);
            rb.row_idx = row as i32;
            rb.valid = true;
        }

        data.copy_from_slice(&rb.buf[col..col + BURST_LEN]);
    }

    fn ddr3_write(&mut self, addr: u32, data: &[u8], mask: &[u8]) {
        assert!(
            addr < HW_MEM_SIZE as u32,
            "{}",
            format!(
                "physical address {:08x} is outside of the physical memory!",
                addr
            )
            .red()
        );
        assert_eq!(data.len(), BURST_LEN);
        assert_eq!(mask.len(), BURST_LEN);

        let aligned = addr & !(BURST_MASK as u32);
        let (rank, bank, row, col) = Self::dram_addr(aligned);

        let rb = &mut self.row_bufs[rank][bank];
        if !(rb.valid && rb.row_idx == row as i32) {
            let off = Self::dram_offset(rank, bank, row);
            rb.buf.copy_from_slice(&self.dram[off..off + NR_COL]);
            rb.row_idx = row as i32;
            rb.valid = true;
        }

        for i in 0..BURST_LEN {
            if mask[i] != 0 {
                rb.buf[col + i] = data[i];
            }
        }

        let off = Self::dram_offset(rank, bank, row);
        self.dram[off..off + NR_COL].copy_from_slice(&rb.buf);
    }

    /// 从内存读取 `len` 字节（1/2/4），按小端序返回 u32。
    pub fn read(&mut self, addr: u32, len: usize) -> u32 {
        let offset = (addr as usize) & BURST_MASK;
        let mut temp = [0u8; 2 * BURST_LEN];

        self.ddr3_read(addr, &mut temp[..BURST_LEN]);
        if offset + len > BURST_LEN {
            self.ddr3_read(addr + BURST_LEN as u32, &mut temp[BURST_LEN..]);
        }

        let mut val = 0u32;
        for i in 0..len {
            val |= (temp[offset + i] as u32) << (i * 8);
        }
        val
    }

    /// 向内存写入 `len` 字节（1/2/4），`data` 按小端序解析。
    pub fn write(&mut self, addr: u32, len: usize, data: u32) {
        let offset = (addr as usize) & BURST_MASK;
        let mut temp = [0u8; 2 * BURST_LEN];
        let mut mask = [0u8; 2 * BURST_LEN];

        for i in 0..len {
            temp[offset + i] = ((data >> (i * 8)) & 0xFF) as u8;
            mask[offset + i] = 1;
        }

        self.ddr3_write(addr, &temp[..BURST_LEN], &mask[..BURST_LEN]);
        if offset + len > BURST_LEN {
            self.ddr3_write(
                addr + BURST_LEN as u32,
                &temp[BURST_LEN..],
                &mask[BURST_LEN..],
            );
        }
    }

    /// 将二进制数据加载到指定物理地址。
    pub fn load_binary(&mut self, data: &[u8], addr: u32) {
        let start = addr as usize;
        self.dram[start..start + data.len()].copy_from_slice(data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ddr3_rw() {
        let mut mem = Memory::new();
        let addr = 0x1fc0_0000;
        let data: u32 = 0x1234_5678;

        mem.write(addr, 4, data);
        let ret = mem.read(addr, 4);
        assert_eq!(ret, data);
    }

    #[test]
    fn dram_partial_rw() {
        let mut mem = Memory::new();
        let addr = 0x1fc0_0000;

        mem.write(addr + 2, 1, 0xAB);
        assert_eq!(mem.read(addr, 1), 0);
        assert_eq!(mem.read(addr + 2, 1), 0xAB);
    }

    #[test]
    fn dram_unaligned_cross_burst() {
        let mut mem = Memory::new();
        let addr = 0x1fc0_0007; // 距 burst 边界 1 字节

        mem.write(addr, 2, 0xBEEF);
        assert_eq!(mem.read(addr, 2), 0xBEEF);
    }
}
