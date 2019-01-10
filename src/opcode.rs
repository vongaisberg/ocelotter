pub mod Opcode {

    pub const ACONST_NULL: u8 = 0x01;
    pub const ALOAD: u8 = 0x19;
    pub const ALOAD_0: u8 = 0x2a;
    pub const ALOAD_1: u8 = 0x2b;
    pub const ARETURN: u8 = 0xb0;
    pub const ASTORE: u8 = 0x53;
    pub const ASTORE_0: u8 = 0x4b;
    pub const ASTORE_1: u8 = 0x4c;
    pub const BIPUSH: u8 = 0x10;
    pub const BREAKPOINT: u8 = 0xca;
    pub const DADD: u8 = 0x63;
    pub const DCONST_0: u8 = 0x0e;
    pub const DCONST_1: u8 = 0x0f;
    pub const DLOAD: u8 = 0x18;
    pub const DLOAD_0: u8 = 0x26;
    pub const DLOAD_1: u8 = 0x27;
    pub const DLOAD_2: u8 = 0x28;
    pub const DLOAD_3: u8 = 0x29;
    pub const DRETURN: u8 = 0xaf;
    pub const DSTORE: u8 = 0x39;
    pub const DSTORE_0: u8 = 0x47;
    pub const DSTORE_1: u8 = 0x48;
    pub const DSTORE_2: u8 = 0x49;
    pub const DSTORE_3: u8 = 0x4a;
    pub const DSUB: u8 = 0x67;
    pub const DUP: u8 = 0x59;
    pub const DUP_X1: u8 = 0x5a;
    pub const GETFIELD: u8 = 0xb4;
    pub const GETSTATIC: u8 = 0xb2;
    pub const GOTO: u8 = 0xa7;
    pub const I2D: u8 = 0x87;
    pub const IADD: u8 = 0x60;
    pub const IAND: u8 = 0x7e;
    pub const ICONST_M1: u8 = 0x02;
    pub const ICONST_0: u8 = 0x03;
    pub const ICONST_1: u8 = 0x04;
    pub const ICONST_2: u8 = 0x05;
    pub const ICONST_3: u8 = 0x06;
    pub const ICONST_4: u8 = 0x07;
    pub const ICONST_5: u8 = 0x08;
    pub const IDIV: u8 = 0x6c;
    pub const IF_ICMPEQ: u8 = 0x9f;
    pub const IFEQ: u8 = 0x99;
    pub const IFGE: u8 = 0x9c;
    pub const IFGT: u8 = 0x9d;
    pub const IFLE: u8 = 0x9e;
    pub const IFLT: u8 = 0x9b;
    pub const IFNE: u8 = 0x9a;
    pub const IFNONNULL: u8 = 0xc7;
    pub const IFNULL: u8 = 0xc6;
    pub const IINC: u8 = 0x84;
    pub const ILOAD: u8 = 0x15;
    pub const ILOAD_0: u8 = 0x1a;
    pub const ILOAD_1: u8 = 0x1b;
    pub const ILOAD_2: u8 = 0x1c;
    pub const ILOAD_3: u8 = 0x1d;
    pub const IMPDEP1: u8 = 0xfe;
    pub const IMPDEP2: u8 = 0xff;
    pub const IMUL: u8 = 0x68;
    pub const INEG: u8 = 0x74;
    pub const INVOKESPECIAL: u8 = 0xb7;
    pub const INVOKESTATIC: u8 = 0xb8;
    pub const INVOKEVIRTUAL: u8 = 0xb6;
    pub const IOR: u8 = 0x80;
    pub const IREM: u8 = 0x70;
    pub const IRETURN: u8 = 0xac;
    pub const ISTORE: u8 = 0x36;
    pub const ISTORE_0: u8 = 0x3b;
    pub const ISTORE_1: u8 = 0x3c;
    pub const ISTORE_2: u8 = 0x3d;
    pub const ISTORE_3: u8 = 0x3e;
    pub const ISUB: u8 = 0x64;
    pub const MONITORENTER: u8 = 0xc2;
    pub const MONITOREXIT: u8 = 0xc3;
    pub const NEW: u8 = 0xbb;
    pub const JSR: u8 = 0xa8;
    pub const JSR_W: u8 = 0xc9;
    pub const LDC: u8 = 0x12;
    pub const NOP: u8 = 0x00;
    pub const POP: u8 = 0x57;
    pub const POP2: u8 = 0x58;
    pub const PUTFIELD: u8 = 0xb5;
    pub const PUTSTATIC: u8 = 0xb3;
    pub const RET: u8 = 0xa9;
    pub const RETURN: u8 = 0xb1;
    pub const SIPUSH: u8 = 0x11;
    pub const SWAP: u8 = 0x5f;

    fn num_params(c: u8) -> u8 {
        match c {
            ALOAD => 1,
            ASTORE => 1,
            BIPUSH => 1,
            DLOAD => 1,
            DSTORE => 1,
            GETFIELD => 2,
            GETSTATIC => 2,
            GOTO => 2,
            IF_ICMPEQ => 2,
            IFEQ => 2,
            IFGE => 2,
            IFGT => 2,
            IFLE => 2,
            IFLT => 2,
            IFNE => 2,
            IFNONNULL => 2,
            IFNULL => 2,
            IINC => 2,
            ILOAD => 1,
            INVOKESPECIAL => 2,
            INVOKESTATIC => 2,
            INVOKEVIRTUAL => 2,
            ISTORE => 1,
            NEW => 2,
            JSR => 2,
            JSR_W => 2,
            LDC => 1,
            PUTFIELD => 2,
            PUTSTATIC => 2,
            RET => 1,
            SIPUSH => 2,
            _ => 0,
        }
    }
}

pub enum JVMValue {
    Boolean { val: bool },
    Byte { val: i8 },
    Short { val: i16 },
    Int { val: i32 },
    Long { val: i64 },
    Float { val: f32 },
    Double { val: f64 },
    Char,
    ObjRef,
}

impl JVMValue {
    fn name(&self) -> char {
        match *self {
            JVMValue::Boolean { val } => 'Z',
            JVMValue::Byte { val } => 'B',
            JVMValue::Short { val } => 'S',
            JVMValue::Int { val } => 'I',
            JVMValue::Long { val } => 'J',
            JVMValue::Float { val } => 'F',
            JVMValue::Double { val } => 'D',
            JVMValue::Char => 'C',
            JVMValue::ObjRef => 'A',
        }
    }
}
