/**
 *  AnyType,
 *   -- anySimpleType,
 *      -- anyAtmoicType
 */
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

    pub fn to_native_type_simple(&self) -> &str {
        match self {
            AtomicType::Any => "String",
            AtomicType::String => "String",
            AtomicType::Interger => "i32",
            AtomicType::Byte => "i8",
            AtomicType::Short => "i16",
            AtomicType::Int => "i32",
            AtomicType::Long => "i64",
            AtomicType::Float => "f32",
            AtomicType::Double => "f64",
            AtomicType::Boolean => "bool",
            AtomicType::Date => "String",
            AtomicType::DateTime => "String",
            AtomicType::DateTimeStamp => "u64",
            AtomicType::Decimal => "f64", // Assuming Decimal is represented as f64
            AtomicType::AnyURI => "String", // Assuming URI is represented as a String
            AtomicType::Base64Binary => "String", // Base64Binary as a String
            AtomicType::HexBinary => "String", // HexBinary as a String
            AtomicType::GDay => "String", // GDay as a String
            AtomicType::GMonth => "String", // GMonth as a String
            AtomicType::GMonthDay => "String", // GMonthDay as a String
            AtomicType::GYear => "String", // GYear as a String
            AtomicType::GYearMonth => "String", // GYearMonth as a String
            AtomicType::Duration => "String", // Duration as a String
            AtomicType::Time => "String", // Time as a String
            AtomicType::NormalizedString => "String", // NormalizedString as a String
            AtomicType::Token => "String", // Token as a String
            AtomicType::Language => "String", // Language as a String
            AtomicType::Name => "String", // Name as a String
            AtomicType::NCName => "String", // NCName as a String
            AtomicType::ENTITY => "String", // ENTITY as a String
            AtomicType::ID => "String", // ID as a String
            AtomicType::IDREF => "String", // IDREF as a String
            AtomicType::NMTOKEN => "String", // NMTOKEN as a String
            AtomicType::NonNegativeInteger => "u64", // Non-negative integers as u64
            AtomicType::PositiveInteger => "u64", // Positive integers as u64
            AtomicType::UnsignedLong => "u64",
            AtomicType::UnsignedInt => "u32",
            AtomicType::UnsignedShort => "u16",
            AtomicType::UnsignedByte => "u8",
            AtomicType::NonPositiveInteger => "i64", // Non-positive integers as i64
            AtomicType::NegativeInteger => "i64", // Negative integers as i64
            AtomicType::NOTATION => "String", // NOTATION as a String
            AtomicType::QName => "String", // QName as a String
            AtomicType::DayTimeDuration => "String", // DayTimeDuration as a String
            AtomicType::YearMonthDuration => "String", // YearMonthDuration as a String
            AtomicType::None => "String",
            AtomicType::Other=> "String"
        }
    }

}
