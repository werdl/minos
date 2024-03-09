//DISK DRIVER
//Driver for ATA disk supporting PIO MODE

use core::arch::asm;
use core::str;

use crate::{
    println, print
};

//Warning! Mutable static here
//TODO: Implement a mutex to get safe access to this
pub static mut DISK: Disk = Disk { enabled: false };

//controller registers ports
const DATA_REGISTER: u16 = 0x1f0;
const SECTOR_COUNT_REGISTER: u16 = 0x1f2;
const LBA_LOW_REGISTER: u16 = 0x1f3;
const LBA_MID_REGISTER: u16 = 0x1f4;
const LBA_HIGH_REGISTER: u16 = 0x1f5;
const DRIVE_REGISTER: u16 = 0x1f6;

//port used for both sending command and getting status
const STATUS_COMMAND_REGISTER: u16 = 0x1f7;

//read write command codes
const READ_COMMAND: u8 = 0x20;
//const WRITE_COMMAND: u8 = 0x30;

//status register bits
const STATUS_BSY: u8 = 0b10000000;
const STATUS_RDY: u8 = 0b01000000;
//const STATUS_DFE: u8 = 0b00100000;
//const STATUS_DRQ: u8 = 0b00001000;
//const STATUS_ERR: u8 = 0b00000001;

pub struct Disk {
    pub enabled: bool,
}

impl Disk {
    //write multiple sectors from specified source to lba
    pub fn write<T>(&self, source: *mut T, lba: u64, sectors: u16) {
        if !self.enabled {
            println!("[ERROR] Cannot write! Disk not enabled");
            return;
        }

        //wait until not busy
        while self.is_busy() {}

        unsafe {
            //disable ata interrupt
            asm!("out dx, al", in("dx") 0x3f6, in("al") 0b00000010 as u8);

            //setup registers
            asm!("out dx, al", in("dx") SECTOR_COUNT_REGISTER, in("al") sectors as u8); //number of setcors to write
            asm!("out dx, al", in("dx") LBA_LOW_REGISTER, in("al") lba as u8); //low 8 bits of lba
            asm!("out dx, al", in("dx") LBA_MID_REGISTER, in("al") (lba >> 8) as u8); //next 8 bits of lba
            asm!("out dx, al", in("dx") LBA_HIGH_REGISTER, in("al") (lba >> 16) as u8); //next 8 bits of lba
            asm!("out dx, al", in("dx") DRIVE_REGISTER, in("al") (0xE0 | ((lba >> 24) & 0xF)) as u8); //0xe0 (master drive) ORed with highest 4 bits of lba

            //send write command to port
            asm!("out dx, al", in("dx") STATUS_COMMAND_REGISTER, in("al") READ_COMMAND);
        }

        let mut sectors_left = sectors;
        let mut source_pointer = source;
        while sectors_left > 0 {
            //a sector is 512 byte, buffer size is 1 byte, so loop for 512/1
            for _i in 0..512 {
                //wait until not busy
                while self.is_busy() {}

                //wait until ready
                while !self.is_ready() {}

                let buffer: u8;
                unsafe {
                    //copy buffer in memory pointed by source
                    buffer = core::ptr::read_unaligned(source_pointer as *const u8);

                    //write 8 bit to controller buffer
                    asm!("out dx, al", in("dx")
                        DATA_REGISTER, in("al") buffer);
                    source_pointer = source_pointer.byte_add(1);
                }
            }
            sectors_left -= 1;
        }

        self.reset();
    }



    //read multiple sectors from lba to specified target
    pub fn read<T>(&self, target: *mut T, lba: u64, sectors: u16) {
        if !self.enabled {
            println!("[ERROR] Cannot read! Disk not enabled");
            return;
        }

        //wait until not busy
        while self.is_busy() {}

        unsafe {
            //disable ata interrupt
            asm!("out dx, al", in("dx") 0x3f6, in("al") 0b00000010 as u8);

            //setup registers
            asm!("out dx, al", in("dx") SECTOR_COUNT_REGISTER, in("al") sectors as u8); //number of setcors to read
            asm!("out dx, al", in("dx") LBA_LOW_REGISTER, in("al") lba as u8); //low 8 bits of lba
            asm!("out dx, al", in("dx") LBA_MID_REGISTER, in("al") (lba >> 8) as u8); //next 8 bits of lba
            asm!("out dx, al", in("dx") LBA_HIGH_REGISTER, in("al") (lba >> 16) as u8); //next 8 bits of lba
            asm!("out dx, al", in("dx") DRIVE_REGISTER, in("al") (0xE0 | ((lba >> 24) & 0xF)) as u8); //0xe0 (master drive) ORed with highest 4 bits of lba

            //send read command to port
            asm!("out dx, al", in("dx") STATUS_COMMAND_REGISTER, in("al") READ_COMMAND);
        }

        let mut sectors_left = sectors;
        let mut target_pointer = target;
        while sectors_left > 0 {
            //a sector is 512 byte, buffer size is 1 byte, so loop for 512/1
            for _i in 0..512 {
                //wait until not busy
                while self.is_busy() {}

                //wait until ready
                while !self.is_ready() {}

                let buffer: u8;
                unsafe {
                    //read 8 bit from controller buffer
                    asm!("in al, dx", out("al") buffer, in("dx") DATA_REGISTER);

                    //copy buffer in memory pointed by target
                    //*(target_pointer as *mut u8) = buffer;
                    core::ptr::write_unaligned(target_pointer as *mut u8, buffer);

                    target_pointer = target_pointer.byte_add(1);
                }
            }
            sectors_left -= 1;
        }

        self.reset();
    }

    //check if disk is busy
    pub fn is_busy(&self) -> bool {
        let status: u8;
        unsafe {
            asm!("in al, dx", out("al") status, in("dx") STATUS_COMMAND_REGISTER);
        }

        //if bsy bit is not 0 return true
        (status & STATUS_BSY) != 0
    }

    //check if disk is ready
    pub fn is_ready(&self) -> bool {
        let status: u8;
        unsafe {
            asm!("in al, dx", out("al") status, in("dx") STATUS_COMMAND_REGISTER);
        }

        //if rdy bit is not 0 return true
        (status & STATUS_RDY) != 0
    }

    //check if ata drive is working
    pub fn check(&mut self) {
        let status: u8;
        unsafe {
            asm!("in al, dx", out("al") status, in("dx") STATUS_COMMAND_REGISTER);
        }

        if status != 0 && status != 0xff {
            self.enabled = true;
            println!("[!] ATA drive found! Status register: {:X}", status);
        } else {
            self.enabled = false;
            println!(
                "[ERROR] ATA drive not working! Status register: {:X}",
                status
            );
        }
    }

    pub fn reset(&self) {
        unsafe {
            asm!("out dx, al", in("dx") 0x3f6, in("al") 0b00000110 as u8);
            asm!("out dx, al", in("dx") 0x3f6, in("al") 0b00000010 as u8);
        }
    }
}

/// Test the disk driver
pub fn ata_test() {
    unsafe {
        DISK.check();
    }

    unsafe {
        if DISK.enabled {
            println!("[!] ATA drive is working!");
        } else {
            println!("[ERROR] ATA drive is not working!");
        }
    }

    // first, read the first sector of the disk. then write some data to the first sector and read it again. then compare the data
    let mut buffer: [u8; 512] = [0; 512];
    unsafe {
        DISK.read(buffer.as_mut_ptr(), 0, 1);
    }

    let mut buffer2: [u8; 512] = [42; 512];
    unsafe {
        DISK.write(buffer.as_mut_ptr(), 0, 1);
        DISK.read(buffer2.as_mut_ptr(), 0, 1);
    }

    // so now, buffer != buffer2
    let mut equal = true;
    for i in 0..512 {
        if buffer[i] != buffer2[i] {
            equal = false;
            break;
        }
    }

    if equal {
        println!("[!] ATA test passed!");
    } else {
        println!("[ERROR] ATA test failed!");
    }
}
