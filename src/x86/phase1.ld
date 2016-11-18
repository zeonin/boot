OUTPUT_FORMAT(binary)
OUTPUT_ARCH(i386)
ENTRY(boot-init)
SECTIONS
{
    . = 0x0800;

    boot-init = .;

    .fs_pre : AT(. + 0x7400) {
        *(.fs.pre.first)
        *(.fs.pre)
        *(.fs.pre.last)
    }
    .boot_code : {
        *(.text.init)
        *(.text)
    }
    .fs_post : {
        *(.fs.post.first)
        *(.fs.post)
        *(.fs.post.last)
    }
    . = 0x9FE;
    .signature : {
        SHORT(0xAA55)
    }
    /DISCARD/ : {
        *(*)
    }
}


    /* Layout:
     *  FS_PRE (if exists)
     *  boot_code
     *  FS_POST (if exsists)
     *  0x55AA
     */
