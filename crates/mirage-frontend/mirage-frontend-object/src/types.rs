use crate::size::Size;
use crate::*;

macro_rules! new_type {
    ($name:ident($mn:ident, $t:ty)($val:ty) = $e:expr) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $name {
            size: Size
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    size: Size::of::<$t>()
                }
            }

            pub fn size(&self) -> Size {
                self.size
            }

            pub fn print_to_string(&self) -> String {
                format!("{}", $e)
            }

            pub fn const_value(&self, val: $t) -> $val {
                <$val>::new(val)
            }
        }

        impl From<$name> for MirageTypeEnum {
            fn from(ty: $name) -> Self {
                Self::$mn(ty)
            }
        }
    };
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum MirageTypeEnum {
    Int8(Int8Type),
    Int16(Int16Type),
    Int32(Int32Type),
    Int64(Int64Type),
    UInt8(UInt8Type),
    UInt16(UInt16Type),
    UInt32(UInt32Type),
    UInt64(UInt64Type),
    Float32(Float32Type),
    Float64(Float64Type),
    Array(ArrayType),
    Pointer(PointerType),
    Struct(StructType),
}

impl MirageTypeEnum {
    pub fn size(&self) -> Size {
        match self {
            MirageTypeEnum::Int8(t) => t.size,
            MirageTypeEnum::Int16(t) => t.size,
            MirageTypeEnum::Int32(t) => t.size,
            MirageTypeEnum::Int64(t) => t.size,
            MirageTypeEnum::UInt8(t) => t.size,
            MirageTypeEnum::UInt16(t) => t.size,
            MirageTypeEnum::UInt32(t) => t.size,
            MirageTypeEnum::UInt64(t) => t.size,
            MirageTypeEnum::Float32(t) => t.size,
            MirageTypeEnum::Float64(t) => t.size,
            MirageTypeEnum::Array(t) => t.size(),
            MirageTypeEnum::Pointer(t) => t.size,
            MirageTypeEnum::Struct(t) => t.size,
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "int8" => Some(Self::Int8(Int8Type::new())),
            "int16" => Some(Self::Int16(Int16Type::new())),
            "int32" => Some(Self::Int32(Int32Type::new())),
            "int64" => Some(Self::Int64(Int64Type::new())),
            "uint8" => Some(Self::UInt8(UInt8Type::new())),
            "uint16" => Some(Self::UInt16(UInt16Type::new())),
            "uint32" => Some(Self::UInt32(UInt32Type::new())),
            "uint64" => Some(Self::UInt64(UInt64Type::new())),
            "float32" => Some(Self::Float32(Float32Type::new())),
            "float64" => Some(Self::Float64(Float64Type::new())),

            _ => None,
        }
    }

    pub fn is_int(&self) -> bool {
        matches!(
            self,
            MirageTypeEnum::Int8(_)
                | MirageTypeEnum::Int16(_)
                | MirageTypeEnum::Int32(_)
                | MirageTypeEnum::Int64(_)
        )
    }

    pub fn is_uint(&self) -> bool {
        match self {
            MirageTypeEnum::UInt8(_) => true,
            MirageTypeEnum::UInt16(_) => true,
            MirageTypeEnum::UInt32(_) => true,
            MirageTypeEnum::UInt64(_) => true,
            _ => false,
        }
    }

    pub fn is_float(&self) -> bool {
        match self {
            MirageTypeEnum::Float32(_) => true,
            MirageTypeEnum::Float64(_) => true,
            _ => false,
        }
    }

    pub fn get_max_bits(&self) -> usize {
        match self {
            MirageTypeEnum::Int8(_) => 8,
            MirageTypeEnum::Int16(_) => 16,
            MirageTypeEnum::Int32(_) => 32,
            MirageTypeEnum::Int64(_) => 64,
            MirageTypeEnum::UInt8(_) => 8,
            MirageTypeEnum::UInt16(_) => 16,
            MirageTypeEnum::UInt32(_) => 32,
            MirageTypeEnum::UInt64(_) => 64,
            MirageTypeEnum::Float32(_) => 32,
            MirageTypeEnum::Float64(_) => 64,
            MirageTypeEnum::Pointer(_) => Size::of::<usize>().size() * 8,
            MirageTypeEnum::Array(t) => t.element_ty().get_max_bits(),
            MirageTypeEnum::Struct(_) => 0,
        }
    }

    pub fn type_int8() -> Int8Type {
        Int8Type::new()
    }

    pub fn type_int16() -> Int16Type {
        Int16Type::new()
    }

    pub fn type_int32() -> Int32Type {
        Int32Type::new()
    }

    pub fn type_int64() -> Int64Type {
        Int64Type::new()
    }

    pub fn type_uint8() -> UInt8Type {
        UInt8Type::new()
    }

    pub fn type_uint16() -> UInt16Type {
        UInt16Type::new()
    }

    pub fn type_uint32() -> UInt32Type {
        UInt32Type::new()
    }

    pub fn type_uint64() -> UInt64Type {
        UInt64Type::new()
    }

    pub fn type_float32() -> Float32Type {
        Float32Type::new()
    }

    pub fn type_float64() -> Float64Type {
        Float64Type::new()
    }

    pub fn type_array(element_ty: MirageTypeEnum, length: usize) -> ArrayType {
        ArrayType::new(element_ty, length)
    }

    pub fn type_ptr(element_ty: MirageTypeEnum) -> PointerType {
        PointerType::new(element_ty)
    }

    pub fn type_struct(fields: Vec<MirageTypeEnum>) -> StructType {
        StructType::new(fields)
    }

    pub fn is_string(&self) -> bool {
        match self {
            MirageTypeEnum::Array(t) => {
                if let MirageTypeEnum::Int8(_) = t.element_ty() {
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    pub fn expect_ptr_type(&self) -> PointerType {
        match self {
            MirageTypeEnum::Pointer(t) => t.clone(),
            _ => panic!("Expected pointer type"),
        }
    }

    pub fn expect_struct_type(&self) -> StructType {
        match self {
            MirageTypeEnum::Struct(t) => t.clone(),
            _ => panic!("Expected struct type"),
        }
    }

    pub fn print_to_string(&self) -> String {
        match self {
            MirageTypeEnum::Int8(t) => t.print_to_string(),
            MirageTypeEnum::Int16(t) => t.print_to_string(),
            MirageTypeEnum::Int32(t) => t.print_to_string(),
            MirageTypeEnum::Int64(t) => t.print_to_string(),
            MirageTypeEnum::UInt8(t) => t.print_to_string(),
            MirageTypeEnum::UInt16(t) => t.print_to_string(),
            MirageTypeEnum::UInt32(t) => t.print_to_string(),
            MirageTypeEnum::UInt64(t) => t.print_to_string(),
            MirageTypeEnum::Float32(t) => t.print_to_string(),
            MirageTypeEnum::Float64(t) => t.print_to_string(),
            MirageTypeEnum::Array(t) => t.print_to_string(),
            MirageTypeEnum::Pointer(t) => t.print_to_string(),
            MirageTypeEnum::Struct(t) => t.print_to_string(),
        }
    }
}

new_type!(Int8Type(Int8, i8)(Int8Value) = "@int8");
new_type!(Int16Type(Int16, i16)(Int16Value) = "@int16");
new_type!(Int32Type(Int32, i32)(Int32Value) = "@int32");
new_type!(Int64Type(Int64, i64)(Int64Value) = "@int64");
new_type!(UInt8Type(UInt8, u8)(UInt8Value) = "@uint8");
new_type!(UInt16Type(UInt16, u16)(UInt16Value) = "@uint16");
new_type!(UInt32Type(UInt32, u32)(UInt32Value) = "@uint32");
new_type!(UInt64Type(UInt64, u64)(UInt64Value) = "@uint64");
new_type!(Float32Type(Float32, f32)(Float32Value) = "@float32");
new_type!(Float64Type(Float64, f64)(Float64Value) = "@float64");

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct ArrayType {
    pub element_ty: Box<MirageTypeEnum>,
    pub length: usize,
    pub size: Size,
}

impl ArrayType {
    pub fn new(element_ty: MirageTypeEnum, length: usize) -> Self {
        Self {
            element_ty: Box::new(element_ty.clone()),
            length,
            size: Size::new(element_ty.size().size() * length),
        }
    }

    pub fn element_ty(&self) -> MirageTypeEnum {
        *self.element_ty.clone()
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn print_to_string(&self) -> String {
        format!("[{} x {}]", self.length, self.element_ty.print_to_string())
    }

    pub fn const_value(&self, val: Vec<MirageValueEnum>) -> ArrayValue {
        ArrayValue::new(self.clone(), val)
    }
}

impl From<ArrayType> for MirageTypeEnum {
    fn from(ty: ArrayType) -> Self {
        Self::Array(ty)
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct PointerType {
    pub element_ty: Box<MirageTypeEnum>,
    pub size: Size,
}

impl PointerType {
    pub fn new(element_ty: MirageTypeEnum) -> Self {
        Self {
            element_ty: Box::new(element_ty.clone()),
            size: Size::of::<usize>(),
        }
    }

    pub fn print_to_string(&self) -> String {
        format!("{}*", self.element_ty.print_to_string())
    }
}

impl From<PointerType> for MirageTypeEnum {
    fn from(ty: PointerType) -> Self {
        Self::Pointer(ty)
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct StructType {
    pub fields: Vec<MirageTypeEnum>,
    pub size: Size,
}

impl StructType {
    pub fn new(fields: Vec<MirageTypeEnum>) -> Self {
        let size = fields.iter().map(|f| f.size()).sum();
        Self { fields, size }
    }
    pub fn print_to_string(&self) -> String {
        let fields = self
            .fields
            .iter()
            .map(|f| f.print_to_string())
            .collect::<Vec<_>>()
            .join(", ");
        format!("{{ {} }}", fields)
    }
}

impl From<StructType> for MirageTypeEnum {
    fn from(ty: StructType) -> Self {
        Self::Struct(ty)
    }
}

