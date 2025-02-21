/**
 *  AnyType,
 *   -- anySimpleType,
 *      -- anyAtmoicType
 */
#[derive(Debug,Clone,PartialEq)]
pub struct BaseType{ }
pub enum AtomicType{
    None,
    Other,
    /* xsd */
    Any,
    AnyURI,
    Base64Binary,
    Boolean,
    Date,
    DateTime,
        DateTimeStamp,
    Decimal,
        Interger,
            Long,
            Int,
            Short,
            Byte,
        NonNegativeInteger,
            PositiveInteger,
                UnsignedLong,
                UnsignedInt,
                UnsignedShort,
                UnsignedByte,
        NonPositiveInteger,
            NegativeInteger,
    Double,
    Duration,
        DayTimeDuration,
        YearMonthDuration,
    Float,
    GDay,
    GMonth,
    GMonthDay,
    GYear,
    GYearMonth,
    HexBinary,
    NOTATION,
    QName,
    String,
        NormalizedString,
            Token,
                Language,
            Name,
                NCName,
                    ENTITY,
                    ID,
                    IDREF,
            NMTOKEN,
    Time
}

pub enum NativeType{
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    Num,
    F32,
    F64,
    String,
    Boolean
}

impl AtomicType {
    pub fn from_str_type(atomic_type: &str) -> Self {
        if atomic_type.is_empty() {
            return AtomicType::None;
        }
        match atomic_type.to_lowercase().as_str() {
            "anyatomic" => AtomicType::Any,
            "anysimple" => AtomicType::Any,
            "anyuri" => AtomicType::AnyURI,
            "base64binary" => AtomicType::Base64Binary,
            "boolean" => AtomicType::Boolean,
            "date" => AtomicType::Date,
            "datetime" => AtomicType::DateTime,
            "datetimestamp" => AtomicType::DateTimeStamp,
            "decimal" => AtomicType::Decimal,
            "integer" => AtomicType::Interger, // Note: Typo in the enum variant name
            "long" => AtomicType::Long,
            "int" => AtomicType::Int,
            "short" => AtomicType::Short,
            "byte" => AtomicType::Byte,
            "nonnegativeinteger" => AtomicType::NonNegativeInteger,
            "positiveinteger" => AtomicType::PositiveInteger,
            "unsignedlong" => AtomicType::UnsignedLong, // Note: Typo in the enum variant name
            "unsignedint" => AtomicType::UnsignedInt,
            "unsignedshort" => AtomicType::UnsignedShort,
            "unsignedbyte" => AtomicType::UnsignedByte,
            "nonpositiveinteger" => AtomicType::NonPositiveInteger,
            "negativeinteger" => AtomicType::NegativeInteger,
            "double" => AtomicType::Double,
            "duration" => AtomicType::Duration,
            "daytimeduration" => AtomicType::DayTimeDuration,
            "yearmonthduration" => AtomicType::YearMonthDuration,
            "float" => AtomicType::Float,
            "gday" => AtomicType::GDay,
            "gmonth" => AtomicType::GMonth,
            "gmonthday" => AtomicType::GMonthDay,
            "gyear" => AtomicType::GYear,
            "gyearmonth" => AtomicType::GYearMonth,
            "hexbinary" => AtomicType::HexBinary,
            "notation" => AtomicType::NOTATION,
            "qname" => AtomicType::QName,
            "string" => AtomicType::String,
            "normalizedstring" => AtomicType::NormalizedString,
            "token" => AtomicType::Token,
            "language" => AtomicType::Language,
            "name" => AtomicType::Name,
            "ncname" => AtomicType::NCName,
            "entity" => AtomicType::ENTITY,
            "id" => AtomicType::ID,
            "idref" => AtomicType::IDREF,
            "nmtoken" => AtomicType::NMTOKEN,
            "time" => AtomicType::Time,
            _ => AtomicType::Other,
        }
    }

    pub fn to_native_type_simple(&self) -> NativeType {
        match self {
            AtomicType::Any => NativeType::String,
            AtomicType::String => NativeType::String,
            AtomicType::Interger => NativeType::I32,
            AtomicType::Byte => NativeType::I8,
            AtomicType::Short => NativeType::I16,
            AtomicType::Int => NativeType::I32,
            AtomicType::Long => NativeType::I64,
            AtomicType::Float => NativeType::F32,
            AtomicType::Double => NativeType::F64,
            AtomicType::Boolean => NativeType::Boolean,
            AtomicType::Date => NativeType::String,
            AtomicType::DateTime => NativeType::String,
            AtomicType::DateTimeStamp => NativeType::U64,
            AtomicType::Decimal => NativeType::Num,
            AtomicType::AnyURI => NativeType::String, // Assuming URI is represented as a String
            AtomicType::Base64Binary => NativeType::String, // Base64Binary as a String
            AtomicType::HexBinary => NativeType::String, // HexBinary as a String
            AtomicType::GDay => NativeType::String, // GDay as a String
            AtomicType::GMonth => NativeType::String, // GMonth as a String
            AtomicType::GMonthDay => NativeType::String, // GMonthDay as a String
            AtomicType::GYear => NativeType::String, // GYear as a String
            AtomicType::GYearMonth => NativeType::String, // GYearMonth as a String
            AtomicType::Duration => NativeType::String, // Duration as a String
            AtomicType::Time => NativeType::String, // Time as a String
            AtomicType::NormalizedString => NativeType::String, // NormalizedString as a String
            AtomicType::Token => NativeType::String, // Token as a String
            AtomicType::Language => NativeType::String, // Language as a String
            AtomicType::Name => NativeType::String, // Name as a String
            AtomicType::NCName => NativeType::String, // NCName as a String
            AtomicType::ENTITY => NativeType::String, // ENTITY as a String
            AtomicType::ID => NativeType::String, // ID as a String
            AtomicType::IDREF => NativeType::String, // IDREF as a String
            AtomicType::NMTOKEN => NativeType::String, // NMTOKEN as a String
            AtomicType::NonNegativeInteger => NativeType::U64, // Non-negative integers as u64
            AtomicType::PositiveInteger => NativeType::U64, // Positive integers as u64
            AtomicType::UnsignedLong => NativeType::U64,
            AtomicType::UnsignedInt => NativeType::U32,
            AtomicType::UnsignedShort => NativeType::U16,
            AtomicType::UnsignedByte => NativeType::U8,
            AtomicType::NonPositiveInteger => NativeType::I64, // Non-positive integers as i64
            AtomicType::NegativeInteger => NativeType::I64, // Negative integers as i64
            AtomicType::NOTATION => NativeType::String, // NOTATION as a String
            AtomicType::QName => NativeType::String, // QName as a String
            AtomicType::DayTimeDuration => NativeType::String, // DayTimeDuration as a String
            AtomicType::YearMonthDuration => NativeType::String, // YearMonthDuration as a String
            AtomicType::None => NativeType::String,
            AtomicType::Other=> NativeType::String
        }
    }

}