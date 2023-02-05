/// LINT.IfChange
/// Containers to hold repeated fundamental values.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BytesList {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub value: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FloatList {
    #[prost(float, repeated, tag = "1")]
    pub value: ::prost::alloc::vec::Vec<f32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Int64List {
    #[prost(int64, repeated, tag = "1")]
    pub value: ::prost::alloc::vec::Vec<i64>,
}
/// Containers for non-sequential data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Feature {
    /// Each feature can be exactly one kind.
    #[prost(oneof = "feature::Kind", tags = "1, 2, 3")]
    pub kind: ::core::option::Option<feature::Kind>,
}
/// Nested message and enum types in `Feature`.
pub mod feature {
    /// Each feature can be exactly one kind.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        #[prost(message, tag = "1")]
        BytesList(super::BytesList),
        #[prost(message, tag = "2")]
        FloatList(super::FloatList),
        #[prost(message, tag = "3")]
        Int64List(super::Int64List),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Features {
    /// Map from feature name to feature.
    #[prost(map = "string, message", tag = "1")]
    pub feature: ::std::collections::HashMap<::prost::alloc::string::String, Feature>,
}
/// Containers for sequential data.
///
/// A FeatureList contains lists of Features.  These may hold zero or more
/// Feature values.
///
/// FeatureLists are organized into categories by name.  The FeatureLists message
/// contains the mapping from name to FeatureList.
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeatureList {
    #[prost(message, repeated, tag = "1")]
    pub feature: ::prost::alloc::vec::Vec<Feature>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeatureLists {
    /// Map from feature name to feature list.
    #[prost(map = "string, message", tag = "1")]
    pub feature_list: ::std::collections::HashMap<::prost::alloc::string::String, FeatureList>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Example {
    #[prost(message, optional, tag = "1")]
    pub features: ::core::option::Option<Features>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SequenceExample {
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<Features>,
    #[prost(message, optional, tag = "2")]
    pub feature_lists: ::core::option::Option<FeatureLists>,
}
/// Dimensions of a tensor.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TensorShapeProto {
    /// Dimensions of the tensor, such as {"input", 30}, {"output", 40}
    /// for a 30 x 40 2D tensor.  If an entry has size -1, this
    /// corresponds to a dimension of unknown size. The names are
    /// optional.
    ///
    /// The order of entries in "dim" matters: It indicates the layout of the
    /// values in the tensor in-memory representation.
    ///
    /// The first entry in "dim" is the outermost dimension used to layout the
    /// values, the last entry is the innermost dimension.  This matches the
    /// in-memory layout of RowMajor Eigen tensors.
    ///
    /// If "dim.size()" > 0, "unknown_rank" must be false.
    #[prost(message, repeated, tag = "2")]
    pub dim: ::prost::alloc::vec::Vec<tensor_shape_proto::Dim>,
    /// If true, the number of dimensions in the shape is unknown.
    ///
    /// If true, "dim.size()" must be 0.
    #[prost(bool, tag = "3")]
    pub unknown_rank: bool,
}
/// Nested message and enum types in `TensorShapeProto`.
pub mod tensor_shape_proto {
    /// One dimension of the tensor.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Dim {
        /// Size of the tensor in that dimension.
        /// This value must be >= -1, but values of -1 are reserved for "unknown"
        /// shapes (values of -1 mean "unknown" dimension).  Certain wrappers
        /// that work with TensorShapeProto may fail at runtime when deserializing
        /// a TensorShapeProto containing a dim value of -1.
        #[prost(int64, tag = "1")]
        pub size: i64,
        /// Optional name of the tensor dimension.
        #[prost(string, tag = "2")]
        pub name: ::prost::alloc::string::String,
    }
}
/// Represents a serialized tf.dtypes.Dtype
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SerializedDType {
    #[prost(enumeration = "DataType", tag = "1")]
    pub datatype: i32,
}
/// (== suppress_warning documentation-presence ==)
/// LINT.IfChange
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DataType {
    /// Not a legal value for DataType.  Used to indicate a DataType field
    /// has not been set.
    DtInvalid = 0,
    /// Data types that all computation devices are expected to be
    /// capable to support.
    DtFloat = 1,
    DtDouble = 2,
    DtInt32 = 3,
    DtUint8 = 4,
    DtInt16 = 5,
    DtInt8 = 6,
    DtString = 7,
    /// Single-precision complex
    DtComplex64 = 8,
    DtInt64 = 9,
    DtBool = 10,
    /// Quantized int8
    DtQint8 = 11,
    /// Quantized uint8
    DtQuint8 = 12,
    /// Quantized int32
    DtQint32 = 13,
    /// Float32 truncated to 16 bits.
    DtBfloat16 = 14,
    /// Quantized int16
    DtQint16 = 15,
    /// Quantized uint16
    DtQuint16 = 16,
    DtUint16 = 17,
    /// Double-precision complex
    DtComplex128 = 18,
    DtHalf = 19,
    DtResource = 20,
    /// Arbitrary C++ data types
    DtVariant = 21,
    DtUint32 = 22,
    DtUint64 = 23,
    /// 5 exponent bits, 2 mantissa bits.
    DtFloat8E5m2 = 24,
    /// 4 exponent bits, 3 mantissa bits, finite-only, with
    DtFloat8E4m3fn = 25,
    /// Do not use!  These are only for parameters.  Every enum above
    /// should have a corresponding value below (verified by types_test).
    DtFloatRef = 101,
    DtDoubleRef = 102,
    DtInt32Ref = 103,
    DtUint8Ref = 104,
    DtInt16Ref = 105,
    DtInt8Ref = 106,
    DtStringRef = 107,
    DtComplex64Ref = 108,
    DtInt64Ref = 109,
    DtBoolRef = 110,
    DtQint8Ref = 111,
    DtQuint8Ref = 112,
    DtQint32Ref = 113,
    DtBfloat16Ref = 114,
    DtQint16Ref = 115,
    DtQuint16Ref = 116,
    DtUint16Ref = 117,
    DtComplex128Ref = 118,
    DtHalfRef = 119,
    DtResourceRef = 120,
    DtVariantRef = 121,
    DtUint32Ref = 122,
    DtUint64Ref = 123,
    DtFloat8E5m2Ref = 124,
    DtFloat8E4m3fnRef = 125,
}
impl DataType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DataType::DtInvalid => "DT_INVALID",
            DataType::DtFloat => "DT_FLOAT",
            DataType::DtDouble => "DT_DOUBLE",
            DataType::DtInt32 => "DT_INT32",
            DataType::DtUint8 => "DT_UINT8",
            DataType::DtInt16 => "DT_INT16",
            DataType::DtInt8 => "DT_INT8",
            DataType::DtString => "DT_STRING",
            DataType::DtComplex64 => "DT_COMPLEX64",
            DataType::DtInt64 => "DT_INT64",
            DataType::DtBool => "DT_BOOL",
            DataType::DtQint8 => "DT_QINT8",
            DataType::DtQuint8 => "DT_QUINT8",
            DataType::DtQint32 => "DT_QINT32",
            DataType::DtBfloat16 => "DT_BFLOAT16",
            DataType::DtQint16 => "DT_QINT16",
            DataType::DtQuint16 => "DT_QUINT16",
            DataType::DtUint16 => "DT_UINT16",
            DataType::DtComplex128 => "DT_COMPLEX128",
            DataType::DtHalf => "DT_HALF",
            DataType::DtResource => "DT_RESOURCE",
            DataType::DtVariant => "DT_VARIANT",
            DataType::DtUint32 => "DT_UINT32",
            DataType::DtUint64 => "DT_UINT64",
            DataType::DtFloat8E5m2 => "DT_FLOAT8_E5M2",
            DataType::DtFloat8E4m3fn => "DT_FLOAT8_E4M3FN",
            DataType::DtFloatRef => "DT_FLOAT_REF",
            DataType::DtDoubleRef => "DT_DOUBLE_REF",
            DataType::DtInt32Ref => "DT_INT32_REF",
            DataType::DtUint8Ref => "DT_UINT8_REF",
            DataType::DtInt16Ref => "DT_INT16_REF",
            DataType::DtInt8Ref => "DT_INT8_REF",
            DataType::DtStringRef => "DT_STRING_REF",
            DataType::DtComplex64Ref => "DT_COMPLEX64_REF",
            DataType::DtInt64Ref => "DT_INT64_REF",
            DataType::DtBoolRef => "DT_BOOL_REF",
            DataType::DtQint8Ref => "DT_QINT8_REF",
            DataType::DtQuint8Ref => "DT_QUINT8_REF",
            DataType::DtQint32Ref => "DT_QINT32_REF",
            DataType::DtBfloat16Ref => "DT_BFLOAT16_REF",
            DataType::DtQint16Ref => "DT_QINT16_REF",
            DataType::DtQuint16Ref => "DT_QUINT16_REF",
            DataType::DtUint16Ref => "DT_UINT16_REF",
            DataType::DtComplex128Ref => "DT_COMPLEX128_REF",
            DataType::DtHalfRef => "DT_HALF_REF",
            DataType::DtResourceRef => "DT_RESOURCE_REF",
            DataType::DtVariantRef => "DT_VARIANT_REF",
            DataType::DtUint32Ref => "DT_UINT32_REF",
            DataType::DtUint64Ref => "DT_UINT64_REF",
            DataType::DtFloat8E5m2Ref => "DT_FLOAT8_E5M2_REF",
            DataType::DtFloat8E4m3fnRef => "DT_FLOAT8_E4M3FN_REF",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DT_INVALID" => Some(Self::DtInvalid),
            "DT_FLOAT" => Some(Self::DtFloat),
            "DT_DOUBLE" => Some(Self::DtDouble),
            "DT_INT32" => Some(Self::DtInt32),
            "DT_UINT8" => Some(Self::DtUint8),
            "DT_INT16" => Some(Self::DtInt16),
            "DT_INT8" => Some(Self::DtInt8),
            "DT_STRING" => Some(Self::DtString),
            "DT_COMPLEX64" => Some(Self::DtComplex64),
            "DT_INT64" => Some(Self::DtInt64),
            "DT_BOOL" => Some(Self::DtBool),
            "DT_QINT8" => Some(Self::DtQint8),
            "DT_QUINT8" => Some(Self::DtQuint8),
            "DT_QINT32" => Some(Self::DtQint32),
            "DT_BFLOAT16" => Some(Self::DtBfloat16),
            "DT_QINT16" => Some(Self::DtQint16),
            "DT_QUINT16" => Some(Self::DtQuint16),
            "DT_UINT16" => Some(Self::DtUint16),
            "DT_COMPLEX128" => Some(Self::DtComplex128),
            "DT_HALF" => Some(Self::DtHalf),
            "DT_RESOURCE" => Some(Self::DtResource),
            "DT_VARIANT" => Some(Self::DtVariant),
            "DT_UINT32" => Some(Self::DtUint32),
            "DT_UINT64" => Some(Self::DtUint64),
            "DT_FLOAT8_E5M2" => Some(Self::DtFloat8E5m2),
            "DT_FLOAT8_E4M3FN" => Some(Self::DtFloat8E4m3fn),
            "DT_FLOAT_REF" => Some(Self::DtFloatRef),
            "DT_DOUBLE_REF" => Some(Self::DtDoubleRef),
            "DT_INT32_REF" => Some(Self::DtInt32Ref),
            "DT_UINT8_REF" => Some(Self::DtUint8Ref),
            "DT_INT16_REF" => Some(Self::DtInt16Ref),
            "DT_INT8_REF" => Some(Self::DtInt8Ref),
            "DT_STRING_REF" => Some(Self::DtStringRef),
            "DT_COMPLEX64_REF" => Some(Self::DtComplex64Ref),
            "DT_INT64_REF" => Some(Self::DtInt64Ref),
            "DT_BOOL_REF" => Some(Self::DtBoolRef),
            "DT_QINT8_REF" => Some(Self::DtQint8Ref),
            "DT_QUINT8_REF" => Some(Self::DtQuint8Ref),
            "DT_QINT32_REF" => Some(Self::DtQint32Ref),
            "DT_BFLOAT16_REF" => Some(Self::DtBfloat16Ref),
            "DT_QINT16_REF" => Some(Self::DtQint16Ref),
            "DT_QUINT16_REF" => Some(Self::DtQuint16Ref),
            "DT_UINT16_REF" => Some(Self::DtUint16Ref),
            "DT_COMPLEX128_REF" => Some(Self::DtComplex128Ref),
            "DT_HALF_REF" => Some(Self::DtHalfRef),
            "DT_RESOURCE_REF" => Some(Self::DtResourceRef),
            "DT_VARIANT_REF" => Some(Self::DtVariantRef),
            "DT_UINT32_REF" => Some(Self::DtUint32Ref),
            "DT_UINT64_REF" => Some(Self::DtUint64Ref),
            "DT_FLOAT8_E5M2_REF" => Some(Self::DtFloat8E5m2Ref),
            "DT_FLOAT8_E4M3FN_REF" => Some(Self::DtFloat8E4m3fnRef),
            _ => None,
        }
    }
}
/// Protocol buffer representing a handle to a tensorflow resource. Handles are
/// not valid across executions, but can be serialized back and forth from within
/// a single run.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceHandleProto {
    /// Unique name for the device containing the resource.
    #[prost(string, tag = "1")]
    pub device: ::prost::alloc::string::String,
    /// Container in which this resource is placed.
    #[prost(string, tag = "2")]
    pub container: ::prost::alloc::string::String,
    /// Unique name of this resource.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// Hash code for the type of the resource. Is only valid in the same device
    /// and in the same execution.
    #[prost(uint64, tag = "4")]
    pub hash_code: u64,
    /// For debug-only, the name of the type pointed to by this handle, if
    /// available.
    #[prost(string, tag = "5")]
    pub maybe_type_name: ::prost::alloc::string::String,
    /// Data types and shapes for the underlying resource.
    #[prost(message, repeated, tag = "6")]
    pub dtypes_and_shapes: ::prost::alloc::vec::Vec<resource_handle_proto::DtypeAndShape>,
}
/// Nested message and enum types in `ResourceHandleProto`.
pub mod resource_handle_proto {
    /// Protocol buffer representing a pair of (data type, tensor shape).
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DtypeAndShape {
        #[prost(enumeration = "super::DataType", tag = "1")]
        pub dtype: i32,
        #[prost(message, optional, tag = "2")]
        pub shape: ::core::option::Option<super::TensorShapeProto>,
    }
}
/// Protocol buffer representing a tensor.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TensorProto {
    #[prost(enumeration = "DataType", tag = "1")]
    pub dtype: i32,
    /// Shape of the tensor.  TODO(touts): sort out the 0-rank issues.
    #[prost(message, optional, tag = "2")]
    pub tensor_shape: ::core::option::Option<TensorShapeProto>,
    /// Version number.
    ///
    /// In version 0, if the "repeated xxx" representations contain only one
    /// element, that element is repeated to fill the shape.  This makes it easy
    /// to represent a constant Tensor with a single value.
    #[prost(int32, tag = "3")]
    pub version_number: i32,
    /// Serialized raw tensor content from either Tensor::AsProtoTensorContent or
    /// memcpy in tensorflow::grpc::EncodeTensorToByteBuffer. This representation
    /// can be used for all tensor types. The purpose of this representation is to
    /// reduce serialization overhead during RPC call by avoiding serialization of
    /// many repeated small items.
    #[prost(bytes = "vec", tag = "4")]
    pub tensor_content: ::prost::alloc::vec::Vec<u8>,
    /// DT_HALF, DT_BFLOAT16. Note that since protobuf has no int16 type, we'll
    /// have some pointless zero padding for each value here.
    #[prost(int32, repeated, tag = "13")]
    pub half_val: ::prost::alloc::vec::Vec<i32>,
    /// DT_FLOAT.
    #[prost(float, repeated, tag = "5")]
    pub float_val: ::prost::alloc::vec::Vec<f32>,
    /// DT_DOUBLE.
    #[prost(double, repeated, tag = "6")]
    pub double_val: ::prost::alloc::vec::Vec<f64>,
    /// DT_INT32, DT_INT16, DT_UINT16, DT_INT8, DT_UINT8.
    #[prost(int32, repeated, tag = "7")]
    pub int_val: ::prost::alloc::vec::Vec<i32>,
    /// DT_STRING
    #[prost(bytes = "vec", repeated, tag = "8")]
    pub string_val: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// DT_COMPLEX64. scomplex_val(2*i) and scomplex_val(2*i+1) are real
    /// and imaginary parts of i-th single precision complex.
    #[prost(float, repeated, tag = "9")]
    pub scomplex_val: ::prost::alloc::vec::Vec<f32>,
    /// DT_INT64
    #[prost(int64, repeated, tag = "10")]
    pub int64_val: ::prost::alloc::vec::Vec<i64>,
    /// DT_BOOL
    #[prost(bool, repeated, tag = "11")]
    pub bool_val: ::prost::alloc::vec::Vec<bool>,
    /// DT_COMPLEX128. dcomplex_val(2*i) and dcomplex_val(2*i+1) are real
    /// and imaginary parts of i-th double precision complex.
    #[prost(double, repeated, tag = "12")]
    pub dcomplex_val: ::prost::alloc::vec::Vec<f64>,
    /// DT_RESOURCE
    #[prost(message, repeated, tag = "14")]
    pub resource_handle_val: ::prost::alloc::vec::Vec<ResourceHandleProto>,
    /// DT_VARIANT
    #[prost(message, repeated, tag = "15")]
    pub variant_val: ::prost::alloc::vec::Vec<VariantTensorDataProto>,
    /// DT_UINT32
    #[prost(uint32, repeated, tag = "16")]
    pub uint32_val: ::prost::alloc::vec::Vec<u32>,
    /// DT_UINT64
    #[prost(uint64, repeated, tag = "17")]
    pub uint64_val: ::prost::alloc::vec::Vec<u64>,
    /// DT_FLOAT8_*, use variable-sized set of bytes
    /// (i.e. the equivalent of repeated uint8, if such a thing existed).
    #[prost(bytes = "vec", tag = "18")]
    pub float8_val: ::prost::alloc::vec::Vec<u8>,
}
/// Protocol buffer representing the serialization format of DT_VARIANT tensors.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VariantTensorDataProto {
    /// Name of the type of objects being serialized.
    #[prost(string, tag = "1")]
    pub type_name: ::prost::alloc::string::String,
    /// Portions of the object that are not Tensors.
    #[prost(bytes = "vec", tag = "2")]
    pub metadata: ::prost::alloc::vec::Vec<u8>,
    /// Tensors contained within objects being serialized.
    #[prost(message, repeated, tag = "3")]
    pub tensors: ::prost::alloc::vec::Vec<TensorProto>,
}
/// Protocol buffer representing the value for an attr used to configure an Op.
/// Comment indicates the corresponding attr type.  Only the field matching the
/// attr type may be filled.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttrValue {
    #[prost(oneof = "attr_value::Value", tags = "2, 3, 4, 5, 6, 7, 8, 1, 10, 9")]
    pub value: ::core::option::Option<attr_value::Value>,
}
/// Nested message and enum types in `AttrValue`.
pub mod attr_value {
    /// LINT.IfChange
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ListValue {
        /// "list(string)"
        #[prost(bytes = "vec", repeated, tag = "2")]
        pub s: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
        /// "list(int)"
        #[prost(int64, repeated, tag = "3")]
        pub i: ::prost::alloc::vec::Vec<i64>,
        /// "list(float)"
        #[prost(float, repeated, tag = "4")]
        pub f: ::prost::alloc::vec::Vec<f32>,
        /// "list(bool)"
        #[prost(bool, repeated, tag = "5")]
        pub b: ::prost::alloc::vec::Vec<bool>,
        /// "list(type)"
        #[prost(enumeration = "super::DataType", repeated, tag = "6")]
        pub r#type: ::prost::alloc::vec::Vec<i32>,
        /// "list(shape)"
        #[prost(message, repeated, tag = "7")]
        pub shape: ::prost::alloc::vec::Vec<super::TensorShapeProto>,
        /// "list(tensor)"
        #[prost(message, repeated, tag = "8")]
        pub tensor: ::prost::alloc::vec::Vec<super::TensorProto>,
        /// "list(attr)"
        #[prost(message, repeated, tag = "9")]
        pub func: ::prost::alloc::vec::Vec<super::NameAttrList>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// "string"
        #[prost(bytes, tag = "2")]
        S(::prost::alloc::vec::Vec<u8>),
        /// "int"
        #[prost(int64, tag = "3")]
        I(i64),
        /// "float"
        #[prost(float, tag = "4")]
        F(f32),
        /// "bool"
        #[prost(bool, tag = "5")]
        B(bool),
        /// "type"
        #[prost(enumeration = "super::DataType", tag = "6")]
        Type(i32),
        /// "shape"
        #[prost(message, tag = "7")]
        Shape(super::TensorShapeProto),
        /// "tensor"
        #[prost(message, tag = "8")]
        Tensor(super::TensorProto),
        /// any "list(...)"
        #[prost(message, tag = "1")]
        List(ListValue),
        /// "func" represents a function. func.name is a function's name or
        /// a primitive op's name. func.attr.first is the name of an attr
        /// defined for that function. func.attr.second is the value for
        /// that attr in the instantiation.
        #[prost(message, tag = "10")]
        Func(super::NameAttrList),
        /// This is a placeholder only used in nodes defined inside a
        /// function.  It indicates the attr value will be supplied when
        /// the function is instantiated.  For example, let us suppose a
        /// node "N" in function "FN". "N" has an attr "A" with value
        /// placeholder = "foo". When FN is instantiated with attr "foo"
        /// set to "bar", the instantiated node N's attr A will have been
        /// given the value "bar".
        #[prost(string, tag = "9")]
        Placeholder(::prost::alloc::string::String),
    }
}
/// A list of attr names and their values. The whole list is attached
/// with a string name.  E.g., MatMul\[T=float\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NameAttrList {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(map = "string, message", tag = "2")]
    pub attr: ::std::collections::HashMap<::prost::alloc::string::String, AttrValue>,
}
/// Highly experimental and very likely to change.
/// This encoding uses tags instead of dedicated messages for regularity. In
/// particular the encoding imposes no restrictions on what the parameters of any
/// type should be, which in particular needs to be true for type symbols.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FullTypeDef {
    /// The principal type represented by this object. This may be a concrete type
    /// (Tensor, Dataset) a type variable (used for dependent types) a type
    /// symbol (Any, Union). See FullTypeId for details.
    #[prost(enumeration = "FullTypeId", tag = "1")]
    pub type_id: i32,
    #[prost(message, repeated, tag = "2")]
    pub args: ::prost::alloc::vec::Vec<FullTypeDef>,
    /// Literal values of this type object, if the type admits one.
    /// For example, a type variable admits a string attribute - its name.
    /// Shape-related types may admit int attributes - their static shape values.
    /// Fields for more data types to be added as needed.
    #[prost(oneof = "full_type_def::Attr", tags = "3, 4")]
    pub attr: ::core::option::Option<full_type_def::Attr>,
}
/// Nested message and enum types in `FullTypeDef`.
pub mod full_type_def {
    /// Literal values of this type object, if the type admits one.
    /// For example, a type variable admits a string attribute - its name.
    /// Shape-related types may admit int attributes - their static shape values.
    /// Fields for more data types to be added as needed.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Attr {
        #[prost(string, tag = "3")]
        S(::prost::alloc::string::String),
        /// TODO(mdan): list/tensor, map? Need to reconcile with TFT_RECORD, etc.
        #[prost(int64, tag = "4")]
        I(i64),
    }
}
/// LINT.IfChange
/// Experimental. Represents the complete type information of a TensorFlow value.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FullTypeId {
    /// The default represents an uninitialized values.
    TftUnset = 0,
    /// Type variables may serve as placeholder for any other type ID in type
    /// templates.
    ///
    /// Examples:
    ///    TFT_DATASET\[TFT_VAR["T"]\] is a Dataset returning a type indicated by "T".
    ///    TFT_TENSOR\[TFT_VAR["T"]\] is a Tensor of n element type indicated by "T".
    ///    TFT_TENSOR\[TFT_VAR["T"]\], TFT_TENSOR\[TFT_VAR["T"]\] are two tensors of
    ///      identical element types.
    ///    TFT_TENSOR\[TFT_VAR["P"]\], TFT_TENSOR\[TFT_VAR["Q"]\] are two tensors of
    ///      independent element types.
    ///
    TftVar = 1,
    /// Wildcard type. Describes a parameter of unknown type. In TensorFlow, that
    /// can mean either a "Top" type (accepts any type), or a dynamically typed
    /// object whose type is unknown in context.
    /// Important: "unknown" does not necessarily mean undeterminable!
    TftAny = 2,
    /// The algebraic product type. This is an algebraic type that may be used just
    /// for logical grouping. Not to confused with TFT_TUPLE which describes a
    /// concrete object of several elements.
    ///
    /// Example:
    ///    TFT_DATASET\[TFT_PRODUCT[TFT_TENSOR[TFT_INT32\], TFT_TENSOR\[TFT_FLOAT64]]\]
    ///      is a Dataset producing two tensors, an integer one and a float one.
    ///
    TftProduct = 3,
    /// Represents a named field, with the name stored in the attribute.
    ///
    /// Parametrization:
    ///    TFT_NAMED\[<type>\]{<name>}
    ///    * <type> is the type of the field
    ///    * <name> is the field name, as string (thpugh can theoretically be an int
    ///      as well)
    ///
    /// Example:
    ///    TFT_RECORD[
    ///      TFT_NAMED\[TFT_TENSOR[TFT_INT32]\]{'foo'},
    ///      TFT_NAMED\[TFT_TENSOR[TFT_FLOAT32]\]{'bar'},
    ///    ]
    ///      is a structure with two fields, an int tensor "foo" and a float tensor
    ///      "bar".
    TftNamed = 4,
    /// Template definition. Expands the variables by repeating a template as
    /// arguments of container.
    ///
    /// Parametrization:
    ///    TFT_FOR_EACH[<container_type>, <template>, <expansions>]
    ///    * <container_type> is the type of the container that the template will be
    ///      expanded into
    ///    * <template> is any type definition that potentially contains type
    ///      variables
    ///    * <expansions> is a TFT_VAR and may include more types in the future
    ///
    /// Example:
    ///    TFT_FOR_EACH[
    ///          TFT_PRODUCT,
    ///          TFT_TENSOR\[TFT_VAR["t"]\],
    ///          TFT_VAR\["t"\]
    ///      ]
    ///      will substitute a T = TFT_INT32 to TFT_PRODUCT\[TFT_TENSOR[TFT_INT32]\]
    ///      and a T = (TFT_INT32, TFT_INT64) to
    ///      TFT_PRODUCT\[TFT_TENSOR[TFT_INT32\], TFT_TENSOR\[TFT_INT64]\].
    TftForEach = 20,
    /// Callable types describe functions and ops.
    ///
    /// Parametrization:
    ///    TFT_CALLABLE[<arg type>, <return type>]
    ///    * <arg type> is the type of the arguments; TFT_PRODUCT represents
    ///    multiple
    ///      arguments.
    ///    * <return type> is the return type; TFT_PRODUCT represents multiple
    ///      return values (that means that callables returning multiple things
    ///      don't necessarily return a single tuple).
    ///
    /// Example:
    ///    TFT_CALLABLE[
    ///      TFT_ANY,
    ///      TFT_PRODUCT\[TFT_TENSOR[TFT_INT32\], TFT_TENSOR\[TFT_FLOAT64]\],
    ///    ]
    ///      is a callable with unspecified (for now) input arguments, and
    ///      two return values of type tensor.
    ///
    TftCallable = 100,
    /// The usual Tensor. This is a parametric type.
    ///
    /// Parametrization:
    ///    TFT_TENSOR[<element type>, <shape type>]
    ///    * <element type> is currently limited to one of the element types
    ///      defined below.
    ///    * <shape type> is not yet defined, and may only be TFT_UNKNOWN for now.
    ///
    /// A TFT_SHAPE type will be defined in the future.
    ///
    /// Example:
    ///    TFT_TENSOR[TFT_INT32, TFT_UNKNOWN]
    ///      is a Tensor of int32 element type and unknown shape.
    ///
    /// TODO(mdan): Define TFT_SHAPE and add more examples.
    TftTensor = 1000,
    /// Array (or tensorflow::TensorList in the variant type registry).
    /// Note: this is not to be confused with the deprecated `TensorArray*` ops
    /// which are not supported by FullType.
    /// This type represents a random-access list whose elements can be
    /// described by a single type. Although immutable, Array is expected to
    /// support efficient mutation semantics (i.e. element update) in the
    /// user-facing API.
    /// The element type may be generic or even TFT_ANY for a heterogenous list.
    ///
    /// Parametrization:
    ///    TFT_ARRAY[<element type>]
    ///    * <element type> may be any concrete type.
    ///
    /// Examples:
    ///    TFT_ARRAY\[TFT_TENSOR[TFT_INT32]\] is a TensorArray holding int32 Tensors
    ///      of any shape.
    ///    TFT_ARRAY\[TFT_TENSOR[TFT_UNKNOWN]\] is a TensorArray holding Tensors of
    ///      mixed element types.
    ///    TFT_ARRAY\[TFT_UNKNOWN\] is a TensorArray holding any element type.
    ///    TFT_ARRAY[] is equivalent to TFT_ARRAY\[TFT_UNKNOWN\].
    ///    TFT_ARRAY\[TFT_ARRAY[]\] is an array or arrays (of unknown types).
    TftArray = 1001,
    /// Optional (or tensorflow::OptionalVariant in the variant type registry).
    /// This type represents a value that may either hold an element of a single
    /// specified type, or nothing at all.
    ///
    /// Parametrization:
    ///    TFT_OPTIONAL[<element type>]
    ///    * <element type> may be any concrete type.
    ///
    /// Examples:
    ///    TFT_OPTIONAL\[TFT_TENSOR[TFT_INT32]\] is an Optional holding an int32
    ///      Tensor of any shape.
    TftOptional = 1002,
    /// Literal types describe compile-time constant values.
    /// Literal types may also participate in dependent types.
    ///
    /// Parametrization:
    ///    TFT_LITERAL[<value type>]{<value>}
    ///    * <value type> may be any concrete type compatible that can hold <value>
    ///    * <value> is the type's attribute, and holds the actual literal value
    ///
    /// Examples:
    ///    TFT_LITERAL\[TFT_INT32\]{1} is the compile-time constant 1.
    TftLiteral = 1003,
    /// Encoding types describe a value of a certain type, encoded as a different
    /// type.
    ///
    /// Parametrization:
    ///    TFT_ENCODED[<encoded type>, <encoding type>]
    ///    * <encoded type> may be any type
    ///    * <encoding type> may be any type
    ///
    /// Examples:
    ///    TFT_ENCODING[TFT_INT32, TFT_STRING] is an integer encoded as string.
    TftEncoded = 1004,
    /// The type of "shape tensors" where the runtime value is the shape of
    /// some tensor(s), i.e. the output of tf.shape.
    /// Shape tensors have special, host-only placement, in contrast to
    /// TFT_TENSOR\[TFT_INT32\] which is the type of a normal numeric tensor
    /// with no special placement.
    ///
    /// Examples:
    ///    TFT_SHAPE_TENSOR\[TFT_INT32\] is the most common
    ///    TFT_SHAPE_TENSOR\[TFT_INT64\] is also allowed
    TftShapeTensor = 1005,
    /// The bool element type.
    /// TODO(mdan): Quantized types, legacy representations (e.g. ref)
    TftBool = 200,
    /// Integer element types.
    TftUint8 = 201,
    TftUint16 = 202,
    TftUint32 = 203,
    TftUint64 = 204,
    TftInt8 = 205,
    TftInt16 = 206,
    TftInt32 = 207,
    TftInt64 = 208,
    /// Floating-point element types.
    TftHalf = 209,
    TftFloat = 210,
    TftDouble = 211,
    TftBfloat16 = 215,
    /// Complex element types.
    /// TODO(mdan): Represent as TFT_COMPLEX\[TFT_DOUBLE\] instead?
    TftComplex64 = 212,
    TftComplex128 = 213,
    /// The string element type.
    TftString = 214,
    /// Datasets created by tf.data ops and APIs. Datasets have generator/iterable
    /// semantics, that is, one can construct an iterator from them. Like
    /// Array, they are considered to return elements that can be described
    /// by a single type. Unlike Array, they do not support random access or
    /// mutation, and can potentially produce an infinite number of elements.
    /// A datasets can produce logical structures (e.g. multiple elements). This
    /// is expressed using TFT_PRODUCT.
    ///
    ///
    /// Parametrization: TFT_DATASET[<element type>].
    ///    * <element type> may be a concrete type or a type symbol. It represents
    ///      the data type of the elements produced by the dataset.
    ///
    /// Examples:
    ///    TFT_DATSET\[TFT_TENSOR[TFT_INT32]\] is a Dataset producing single int32
    ///      Tensors of unknown shape.
    ///    TFT_DATSET\[TFT_PRODUCT[TFT_TENSOR[TFT_INT32\], TFT_TENSOR\[TFT_FLOAT32]\] is
    ///      a Dataset producing pairs of Tensors, one integer and one float.
    /// Note: The high ID number is to prepare for the eventuality that Datasets
    /// will be supported by user types in the future.
    TftDataset = 10102,
    /// A ragged tensor created by tf.ragged ops and APIs.
    ///
    /// Parametrization: TFT_RAGGED\[<element_type>\].
    TftRagged = 10103,
    /// Iterators created by tf.data ops and APIs. Very similar to Datasets, except
    /// they are mutable.
    ///
    ///
    /// Parametrization: TFT_ITERATOR[<element type>].
    ///    * <element type> may be a concrete type or a type symbol. It represents
    ///      the data type of the elements produced by the dataset.
    TftIterator = 10104,
    /// A mutex lock tensor, produced by tf.raw_ops.MutexLock.
    /// Unlike strict execution models, where ownership of a lock is denoted by
    /// "running after the lock has been acquired", in non-strict mode, lock
    /// ownership is in the true sense: "the op argument representing the lock is
    /// available".
    /// Mutex locks are the dynamic counterpart of control dependencies.
    /// TODO(mdan): Properly document this thing.
    ///
    /// Parametrization: TFT_MUTEX_LOCK[].
    TftMutexLock = 10202,
    /// The equivalent of a Tensor with DT_VARIANT dtype, kept here to simplify
    /// translation. This type should not normally appear after type inference.
    /// Note that LEGACY_VARIANT != ANY: TENSOR\[INT32\] is a subtype of ANY, but is
    /// not a subtype of LEGACY_VARIANT.
    TftLegacyVariant = 10203,
}
impl FullTypeId {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FullTypeId::TftUnset => "TFT_UNSET",
            FullTypeId::TftVar => "TFT_VAR",
            FullTypeId::TftAny => "TFT_ANY",
            FullTypeId::TftProduct => "TFT_PRODUCT",
            FullTypeId::TftNamed => "TFT_NAMED",
            FullTypeId::TftForEach => "TFT_FOR_EACH",
            FullTypeId::TftCallable => "TFT_CALLABLE",
            FullTypeId::TftTensor => "TFT_TENSOR",
            FullTypeId::TftArray => "TFT_ARRAY",
            FullTypeId::TftOptional => "TFT_OPTIONAL",
            FullTypeId::TftLiteral => "TFT_LITERAL",
            FullTypeId::TftEncoded => "TFT_ENCODED",
            FullTypeId::TftShapeTensor => "TFT_SHAPE_TENSOR",
            FullTypeId::TftBool => "TFT_BOOL",
            FullTypeId::TftUint8 => "TFT_UINT8",
            FullTypeId::TftUint16 => "TFT_UINT16",
            FullTypeId::TftUint32 => "TFT_UINT32",
            FullTypeId::TftUint64 => "TFT_UINT64",
            FullTypeId::TftInt8 => "TFT_INT8",
            FullTypeId::TftInt16 => "TFT_INT16",
            FullTypeId::TftInt32 => "TFT_INT32",
            FullTypeId::TftInt64 => "TFT_INT64",
            FullTypeId::TftHalf => "TFT_HALF",
            FullTypeId::TftFloat => "TFT_FLOAT",
            FullTypeId::TftDouble => "TFT_DOUBLE",
            FullTypeId::TftBfloat16 => "TFT_BFLOAT16",
            FullTypeId::TftComplex64 => "TFT_COMPLEX64",
            FullTypeId::TftComplex128 => "TFT_COMPLEX128",
            FullTypeId::TftString => "TFT_STRING",
            FullTypeId::TftDataset => "TFT_DATASET",
            FullTypeId::TftRagged => "TFT_RAGGED",
            FullTypeId::TftIterator => "TFT_ITERATOR",
            FullTypeId::TftMutexLock => "TFT_MUTEX_LOCK",
            FullTypeId::TftLegacyVariant => "TFT_LEGACY_VARIANT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TFT_UNSET" => Some(Self::TftUnset),
            "TFT_VAR" => Some(Self::TftVar),
            "TFT_ANY" => Some(Self::TftAny),
            "TFT_PRODUCT" => Some(Self::TftProduct),
            "TFT_NAMED" => Some(Self::TftNamed),
            "TFT_FOR_EACH" => Some(Self::TftForEach),
            "TFT_CALLABLE" => Some(Self::TftCallable),
            "TFT_TENSOR" => Some(Self::TftTensor),
            "TFT_ARRAY" => Some(Self::TftArray),
            "TFT_OPTIONAL" => Some(Self::TftOptional),
            "TFT_LITERAL" => Some(Self::TftLiteral),
            "TFT_ENCODED" => Some(Self::TftEncoded),
            "TFT_SHAPE_TENSOR" => Some(Self::TftShapeTensor),
            "TFT_BOOL" => Some(Self::TftBool),
            "TFT_UINT8" => Some(Self::TftUint8),
            "TFT_UINT16" => Some(Self::TftUint16),
            "TFT_UINT32" => Some(Self::TftUint32),
            "TFT_UINT64" => Some(Self::TftUint64),
            "TFT_INT8" => Some(Self::TftInt8),
            "TFT_INT16" => Some(Self::TftInt16),
            "TFT_INT32" => Some(Self::TftInt32),
            "TFT_INT64" => Some(Self::TftInt64),
            "TFT_HALF" => Some(Self::TftHalf),
            "TFT_FLOAT" => Some(Self::TftFloat),
            "TFT_DOUBLE" => Some(Self::TftDouble),
            "TFT_BFLOAT16" => Some(Self::TftBfloat16),
            "TFT_COMPLEX64" => Some(Self::TftComplex64),
            "TFT_COMPLEX128" => Some(Self::TftComplex128),
            "TFT_STRING" => Some(Self::TftString),
            "TFT_DATASET" => Some(Self::TftDataset),
            "TFT_RAGGED" => Some(Self::TftRagged),
            "TFT_ITERATOR" => Some(Self::TftIterator),
            "TFT_MUTEX_LOCK" => Some(Self::TftMutexLock),
            "TFT_LEGACY_VARIANT" => Some(Self::TftLegacyVariant),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeDef {
    /// The name given to this operator. Used for naming inputs,
    /// logging, visualization, etc.  Unique within a single GraphDef.
    /// Must match the regexp "\[A-Za-z0-9.][A-Za-z0-9_>./\]*".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The operation name.  There may be custom parameters in attrs.
    /// Op names starting with an underscore are reserved for internal use.
    #[prost(string, tag = "2")]
    pub op: ::prost::alloc::string::String,
    /// Each input is "node:src_output" with "node" being a string name and
    /// "src_output" indicating which output tensor to use from "node". If
    /// "src_output" is 0 the ":0" suffix can be omitted.  Regular inputs
    /// may optionally be followed by control inputs that have the format
    /// "^node".
    #[prost(string, repeated, tag = "3")]
    pub input: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A (possibly partial) specification for the device on which this
    /// node should be placed.
    /// The expected syntax for this string is as follows:
    ///
    /// DEVICE_SPEC ::= PARTIAL_SPEC
    ///
    /// PARTIAL_SPEC ::= ("/" CONSTRAINT) *
    /// CONSTRAINT ::= ("job:" JOB_NAME)
    ///               | ("replica:" \[1-9][0-9\]*)
    ///               | ("task:" \[1-9][0-9\]*)
    ///               | ("device:" \[A-Za-z\]* ":" (\[1-9][0-9\]* | "*") )
    ///
    /// Valid values for this string include:
    /// * "/job:worker/replica:0/task:1/device:GPU:3"  (full specification)
    /// * "/job:worker/device:GPU:3"                   (partial specification)
    /// * ""                                    (no specification)
    ///
    /// If the constraints do not resolve to a single device (or if this
    /// field is empty or not present), the runtime will attempt to
    /// choose a device automatically.
    #[prost(string, tag = "4")]
    pub device: ::prost::alloc::string::String,
    /// Operation-specific graph-construction-time configuration.
    /// Note that this should include all attrs defined in the
    /// corresponding OpDef, including those with a value matching
    /// the default -- this allows the default to change and makes
    /// NodeDefs easier to interpret on their own.  However, if
    /// an attr with a default is not specified in this list, the
    /// default will be used.
    /// The "names" (keys) must match the regexp "\[a-z][a-z0-9_\]+" (and
    /// one of the names from the corresponding OpDef's attr field).
    /// The values must have a type matching the corresponding OpDef
    /// attr's type field.
    /// TODO(josh11b): Add some examples here showing best practices.
    #[prost(map = "string, message", tag = "5")]
    pub attr: ::std::collections::HashMap<::prost::alloc::string::String, AttrValue>,
    /// This stores debug information associated with the node.
    #[prost(message, optional, tag = "6")]
    pub experimental_debug_info: ::core::option::Option<node_def::ExperimentalDebugInfo>,
    /// The complete type of this node. Experimental and subject to change.
    /// Currently, the field only contains the return types of the node. That will
    /// extend in the future to contain the entire signature of the node, as a
    /// function type.
    #[prost(message, optional, tag = "7")]
    pub experimental_type: ::core::option::Option<FullTypeDef>,
}
/// Nested message and enum types in `NodeDef`.
pub mod node_def {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExperimentalDebugInfo {
        /// Opaque string inserted into error messages created by the runtime.
        ///
        /// This is intended to store the list of names of the nodes from the
        /// original graph that this node was derived. For example if this node, say
        /// C, was result of a fusion of 2 nodes A and B, then 'original_node' would
        /// be {A, B}. This information can be used to map errors originating at the
        /// current node to some top level source code.
        #[prost(string, repeated, tag = "1")]
        pub original_node_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// This is intended to store the list of names of the functions from the
        /// original graph that this node was derived. For example if this node, say
        /// C, was result of a fusion of node A in function FA and node B in function
        /// FB, then `original_funcs` would be {FA, FB}. If the node is in the top
        /// level graph, the `original_func` is empty. This information, with the
        /// `original_node_names` can be used to map errors originating at the
        /// current ndoe to some top level source code.
        #[prost(string, repeated, tag = "2")]
        pub original_func_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
/// Defines an operation. A NodeDef in a GraphDef specifies an Op by
/// using the "op" field which should match the name of a OpDef.
/// LINT.IfChange
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpDef {
    /// Op names starting with an underscore are reserved for internal use.
    /// Names should be CamelCase and match the regexp "\[A-Z][a-zA-Z0-9>_\]*".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Description of the input(s).
    #[prost(message, repeated, tag = "2")]
    pub input_arg: ::prost::alloc::vec::Vec<op_def::ArgDef>,
    /// Description of the output(s).
    #[prost(message, repeated, tag = "3")]
    pub output_arg: ::prost::alloc::vec::Vec<op_def::ArgDef>,
    /// Named control outputs for this operation. Useful only for composite
    /// operations (i.e. functions) which want to name different control outputs.
    #[prost(string, repeated, tag = "20")]
    pub control_output: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "4")]
    pub attr: ::prost::alloc::vec::Vec<op_def::AttrDef>,
    /// Optional deprecation based on GraphDef versions.
    #[prost(message, optional, tag = "8")]
    pub deprecation: ::core::option::Option<OpDeprecation>,
    /// One-line human-readable description of what the Op does.
    #[prost(string, tag = "5")]
    pub summary: ::prost::alloc::string::String,
    /// Additional, longer human-readable description of what the Op does.
    #[prost(string, tag = "6")]
    pub description: ::prost::alloc::string::String,
    /// True if the operation is commutative ("op(a,b) == op(b,a)" for all inputs)
    #[prost(bool, tag = "18")]
    pub is_commutative: bool,
    /// If is_aggregate is true, then this operation accepts N >= 2
    /// inputs and produces 1 output all of the same type.  Should be
    /// associative and commutative, and produce output with the same
    /// shape as the input.  The optimizer may replace an aggregate op
    /// taking input from multiple devices with a tree of aggregate ops
    /// that aggregate locally within each device (and possibly within
    /// groups of nearby devices) before communicating.
    /// TODO(josh11b): Implement that optimization.
    ///
    /// for things like add
    #[prost(bool, tag = "16")]
    pub is_aggregate: bool,
    /// Ops are marked as stateful if their behavior depends on some state beyond
    /// their input tensors (e.g. variable reading op) or if they have
    /// a side-effect (e.g. printing or asserting ops). Equivalently, stateless ops
    /// must always produce the same output for the same input and have
    /// no side-effects.
    ///
    /// By default Ops may be moved between devices.  Stateful ops should
    /// either not be moved, or should only be moved if that state can also
    /// be moved (e.g. via some sort of save / restore).
    /// Stateful ops are guaranteed to never be optimized away by Common
    /// Subexpression Elimination (CSE).
    ///
    /// for things like variables, queue
    #[prost(bool, tag = "17")]
    pub is_stateful: bool,
    /// By default, all inputs to an Op must be initialized Tensors.  Ops
    /// that may initialize tensors for the first time should set this
    /// field to true, to allow the Op to take an uninitialized Tensor as
    /// input.
    ///
    /// for Assign, etc.
    #[prost(bool, tag = "19")]
    pub allows_uninitialized_input: bool,
    /// Indicates whether the op implementation uses distributed communication.
    /// If True, the op is allowed to return errors for network disconnection and
    /// trigger TF network failure handling logics.
    #[prost(bool, tag = "21")]
    pub is_distributed_communication: bool,
}
/// Nested message and enum types in `OpDef`.
pub mod op_def {
    /// For describing inputs and outputs.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ArgDef {
        /// Name for the input/output.  Should match the regexp "\[a-z][a-z0-9_\]*".
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// Human readable description.
        #[prost(string, tag = "2")]
        pub description: ::prost::alloc::string::String,
        /// Describes the type of one or more tensors that are accepted/produced
        /// by this input/output arg.  The only legal combinations are:
        /// * For a single tensor: either the "type" field is set or the
        ///    "type_attr" field is set to the name of an attr with type "type".
        /// * For a sequence of tensors with the same type: the "number_attr"
        ///    field will be set to the name of an attr with type "int", and
        ///    either the "type" or "type_attr" field will be set as for
        ///    single tensors.
        /// * For a sequence of tensors, the "type_list_attr" field will be set
        ///    to the name of an attr with type "list(type)".
        #[prost(enumeration = "super::DataType", tag = "3")]
        pub r#type: i32,
        /// if specified, attr must have type "type"
        #[prost(string, tag = "4")]
        pub type_attr: ::prost::alloc::string::String,
        /// if specified, attr must have type "int"
        #[prost(string, tag = "5")]
        pub number_attr: ::prost::alloc::string::String,
        /// If specified, attr must have type "list(type)", and none of
        /// type, type_attr, and number_attr may be specified.
        #[prost(string, tag = "6")]
        pub type_list_attr: ::prost::alloc::string::String,
        /// The handle data for resource inputs.
        #[prost(message, repeated, tag = "7")]
        pub handle_data: ::prost::alloc::vec::Vec<super::resource_handle_proto::DtypeAndShape>,
        /// For inputs: if true, the inputs are required to be refs.
        ///    By default, inputs can be either refs or non-refs.
        /// For outputs: if true, outputs are refs, otherwise they are not.
        #[prost(bool, tag = "16")]
        pub is_ref: bool,
        /// Experimental. Full type declaration for this argument.
        /// The full type specification combines type, type_attr, type_list_attr,
        /// etc. into a unified representation.
        /// This declaration may contain non-concrete types (for example,
        /// Tensor<TypeVar<'T'>> is a valid type declaration.
        ///
        /// Note: this is a transient field. The long-term aim is to represent the
        /// entire OpDef as a single type: a callable. In that context, this field is
        /// just the type of a single argument.
        #[prost(message, optional, tag = "17")]
        pub experimental_full_type: ::core::option::Option<super::FullTypeDef>,
    }
    /// Description of the graph-construction-time configuration of this
    /// Op.  That is to say, this describes the attr fields that will
    /// be specified in the NodeDef.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AttrDef {
        /// A descriptive name for the argument.  May be used, e.g. by the
        /// Python client, as a keyword argument name, and so should match
        /// the regexp "\[a-z][a-z0-9_\]+".
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// One of the type names from attr_value.proto ("string", "list(string)",
        /// "int", etc.).
        #[prost(string, tag = "2")]
        pub r#type: ::prost::alloc::string::String,
        /// A reasonable default for this attribute if the user does not supply
        /// a value.  If not specified, the user must supply a value.
        #[prost(message, optional, tag = "3")]
        pub default_value: ::core::option::Option<super::AttrValue>,
        /// Human-readable description.
        #[prost(string, tag = "4")]
        pub description: ::prost::alloc::string::String,
        /// For type == "int", this is a minimum value.  For "list(___)"
        /// types, this is the minimum length.
        #[prost(bool, tag = "5")]
        pub has_minimum: bool,
        #[prost(int64, tag = "6")]
        pub minimum: i64,
        /// The set of allowed values.  Has type that is the "list" version
        /// of the "type" field above (uses the "list" field of AttrValue).
        /// If type == "type" or "list(type)" above, then the "type" field
        /// of "allowed_values.list" has the set of allowed DataTypes.
        /// If type == "string" or "list(string)", then the "s" field of
        /// "allowed_values.list" has the set of allowed strings.
        #[prost(message, optional, tag = "7")]
        pub allowed_values: ::core::option::Option<super::AttrValue>,
    }
}
/// Information about version-dependent deprecation of an op
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpDeprecation {
    /// First GraphDef version at which the op is disallowed.
    #[prost(int32, tag = "1")]
    pub version: i32,
    /// Explanation of why it was deprecated and what to use instead.
    #[prost(string, tag = "2")]
    pub explanation: ::prost::alloc::string::String,
}
/// A collection of OpDefs
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpList {
    #[prost(message, repeated, tag = "1")]
    pub op: ::prost::alloc::vec::Vec<OpDef>,
}
/// A library is a set of named functions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunctionDefLibrary {
    #[prost(message, repeated, tag = "1")]
    pub function: ::prost::alloc::vec::Vec<FunctionDef>,
    #[prost(message, repeated, tag = "2")]
    pub gradient: ::prost::alloc::vec::Vec<GradientDef>,
    #[prost(message, repeated, tag = "3")]
    pub registered_gradients: ::prost::alloc::vec::Vec<RegisteredGradient>,
}
/// A function can be instantiated when the runtime can bind every attr
/// with a value. When a GraphDef has a call to a function, it must
/// have binding for every attr defined in the signature.
///
/// TODO(zhifengc):
///    * device spec, etc.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunctionDef {
    /// The definition of the function's name, arguments, return values,
    /// attrs etc.
    #[prost(message, optional, tag = "1")]
    pub signature: ::core::option::Option<OpDef>,
    /// Attributes specific to this function definition.
    #[prost(map = "string, message", tag = "5")]
    pub attr: ::std::collections::HashMap<::prost::alloc::string::String, AttrValue>,
    #[prost(map = "uint32, message", tag = "7")]
    pub arg_attr: ::std::collections::HashMap<u32, function_def::ArgAttrs>,
    /// Unique IDs for each resource argument, used to track aliasing resources. If
    /// Argument A and Argument B alias each other, then
    /// resource_arg_unique_ids\[A.index\] == resource_arg_unique_ids\[B.index\].
    ///
    /// If this field is empty, none of the arguments could alias; otherwise, every
    /// resource argument should have an entry in this field.
    ///
    /// When instantiated, the unique IDs will be attached to the _Arg nodes'
    /// "_resource_arg_unique_id" attribute.
    #[prost(map = "uint32, uint32", tag = "8")]
    pub resource_arg_unique_id: ::std::collections::HashMap<u32, u32>,
    /// By convention, "op" in node_def is resolved by consulting with a
    /// user-defined library first. If not resolved, "func" is assumed to
    /// be a builtin op.
    #[prost(message, repeated, tag = "3")]
    pub node_def: ::prost::alloc::vec::Vec<NodeDef>,
    /// A mapping from the output arg names from `signature` to the
    /// outputs from `node_def` that should be returned by the function.
    #[prost(map = "string, string", tag = "4")]
    pub ret:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// A mapping from control output names from `signature` to node names in
    /// `node_def` which should be control outputs of this function.
    #[prost(map = "string, string", tag = "6")]
    pub control_ret:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Nested message and enum types in `FunctionDef`.
pub mod function_def {
    /// Attributes for function arguments. These attributes are the same set of
    /// valid attributes as to _Arg nodes.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ArgAttrs {
        #[prost(map = "string, message", tag = "1")]
        pub attr: ::std::collections::HashMap<::prost::alloc::string::String, super::AttrValue>,
    }
}
/// GradientDef defines the gradient function of a function defined in
/// a function library.
///
/// A gradient function g (specified by gradient_func) for a function f
/// (specified by function_name) must follow the following:
///
/// The function 'f' must be a numerical function which takes N inputs
/// and produces M outputs. Its gradient function 'g', which is a
/// function taking N + M inputs and produces N outputs.
///
/// I.e. if we have
///     (y1, y2, ..., y_M) = f(x1, x2, ..., x_N),
/// then, g is
///     (dL/dx1, dL/dx2, ..., dL/dx_N) = g(x1, x2, ..., x_N,
///                                       dL/dy1, dL/dy2, ..., dL/dy_M),
/// where L is a scalar-value function of (x1, x2, ..., xN) (e.g., the
/// loss function). dL/dx_i is the partial derivative of L with respect
/// to x_i.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GradientDef {
    /// The function name.
    #[prost(string, tag = "1")]
    pub function_name: ::prost::alloc::string::String,
    /// The gradient function's name.
    #[prost(string, tag = "2")]
    pub gradient_func: ::prost::alloc::string::String,
}
/// RegisteredGradient stores a gradient function that is registered in the
/// gradients library and used in the ops of a function in the function library.
/// Unlike GradientDef, these gradients are identified by op type, and not
/// directly linked to any function.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisteredGradient {
    /// The gradient function's name.
    #[prost(string, tag = "1")]
    pub gradient_func: ::prost::alloc::string::String,
    /// The gradient function's registered op type.
    #[prost(string, tag = "2")]
    pub registered_op_type: ::prost::alloc::string::String,
}
/// Version information for a piece of serialized data
///
/// There are different types of versions for each type of data
/// (GraphDef, etc.), but they all have the same common shape
/// described here.
///
/// Each consumer has "consumer" and "min_producer" versions (specified
/// elsewhere).  A consumer is allowed to consume this data if
///
///    producer >= min_producer
///    consumer >= min_consumer
///    consumer not in bad_consumers
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VersionDef {
    /// The version of the code that produced this data.
    #[prost(int32, tag = "1")]
    pub producer: i32,
    /// Any consumer below this version is not allowed to consume this data.
    #[prost(int32, tag = "2")]
    pub min_consumer: i32,
    /// Specific consumer versions which are disallowed (e.g. due to bugs).
    #[prost(int32, repeated, tag = "3")]
    pub bad_consumers: ::prost::alloc::vec::Vec<i32>,
}
/// Represents the graph of operations
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GraphDef {
    #[prost(message, repeated, tag = "1")]
    pub node: ::prost::alloc::vec::Vec<NodeDef>,
    /// Compatibility versions of the graph.  See core/public/version.h for version
    /// history.  The GraphDef version is distinct from the TensorFlow version, and
    /// each release of TensorFlow will support a range of GraphDef versions.
    #[prost(message, optional, tag = "4")]
    pub versions: ::core::option::Option<VersionDef>,
    /// Deprecated single version field; use versions above instead.  Since all
    /// GraphDef changes before "versions" was introduced were forward
    /// compatible, this field is entirely ignored.
    #[deprecated]
    #[prost(int32, tag = "3")]
    pub version: i32,
    /// "library" provides user-defined functions.
    ///
    /// Naming:
    ///    * library.function.name are in a flat namespace.
    ///      NOTE: We may need to change it to be hierarchical to support
    ///      different orgs. E.g.,
    ///      { "/google/nn", { ... }},
    ///      { "/google/vision", { ... }}
    ///      { "/org_foo/module_bar", { ... }}
    ///      map<string, FunctionDefLib> named_lib;
    ///    * If node\[i\].op is the name of one function in "library",
    ///      node\[i\] is deemed as a function call. Otherwise, node\[i\].op
    ///      must be a primitive operation supported by the runtime.
    ///
    ///
    /// Function call semantics:
    ///
    ///    * The callee may start execution as soon as some of its inputs
    ///      are ready. The caller may want to use Tuple() mechanism to
    ///      ensure all inputs are ready in the same time.
    ///
    ///    * The consumer of return values may start executing as soon as
    ///      the return values the consumer depends on are ready.  The
    ///      consumer may want to use Tuple() mechanism to ensure the
    ///      consumer does not start until all return values of the callee
    ///      function are ready.
    #[prost(message, optional, tag = "2")]
    pub library: ::core::option::Option<FunctionDefLibrary>,
}
/// Protocol buffer representing a Variable.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VariableDef {
    /// Name of the variable tensor.
    #[prost(string, tag = "1")]
    pub variable_name: ::prost::alloc::string::String,
    /// Name of the tensor holding the variable's initial value.
    #[prost(string, tag = "6")]
    pub initial_value_name: ::prost::alloc::string::String,
    /// Name of the initializer op.
    #[prost(string, tag = "2")]
    pub initializer_name: ::prost::alloc::string::String,
    /// Name of the snapshot tensor.
    #[prost(string, tag = "3")]
    pub snapshot_name: ::prost::alloc::string::String,
    /// Support for saving variables as slices of a larger variable.
    #[prost(message, optional, tag = "4")]
    pub save_slice_info_def: ::core::option::Option<SaveSliceInfoDef>,
    /// Whether to represent this as a ResourceVariable.
    #[prost(bool, tag = "5")]
    pub is_resource: bool,
    /// Whether this variable should be trained.
    #[prost(bool, tag = "7")]
    pub trainable: bool,
    /// Indicates when a distributed variable will be synced.
    #[prost(enumeration = "VariableSynchronization", tag = "8")]
    pub synchronization: i32,
    /// Indicates how a distributed variable will be aggregated.
    #[prost(enumeration = "VariableAggregation", tag = "9")]
    pub aggregation: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SaveSliceInfoDef {
    /// Name of the full variable of which this is a slice.
    #[prost(string, tag = "1")]
    pub full_name: ::prost::alloc::string::String,
    /// Shape of the full variable.
    #[prost(int64, repeated, tag = "2")]
    pub full_shape: ::prost::alloc::vec::Vec<i64>,
    /// Offset of this variable into the full variable.
    #[prost(int64, repeated, tag = "3")]
    pub var_offset: ::prost::alloc::vec::Vec<i64>,
    /// Shape of this variable.
    #[prost(int64, repeated, tag = "4")]
    pub var_shape: ::prost::alloc::vec::Vec<i64>,
}
/// Indicates when a distributed variable will be synced.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VariableSynchronization {
    /// `AUTO`: Indicates that the synchronization will be determined by the
    /// current `DistributionStrategy` (eg. With `MirroredStrategy` this would be
    /// `ON_WRITE`).
    Auto = 0,
    /// `NONE`: Indicates that there will only be one copy of the variable, so
    /// there is no need to sync.
    None = 1,
    /// `ON_WRITE`: Indicates that the variable will be updated across devices
    /// every time it is written.
    OnWrite = 2,
    /// `ON_READ`: Indicates that the variable will be aggregated across devices
    /// when it is read (eg. when checkpointing or when evaluating an op that uses
    /// the variable).
    OnRead = 3,
}
impl VariableSynchronization {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VariableSynchronization::Auto => "VARIABLE_SYNCHRONIZATION_AUTO",
            VariableSynchronization::None => "VARIABLE_SYNCHRONIZATION_NONE",
            VariableSynchronization::OnWrite => "VARIABLE_SYNCHRONIZATION_ON_WRITE",
            VariableSynchronization::OnRead => "VARIABLE_SYNCHRONIZATION_ON_READ",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "VARIABLE_SYNCHRONIZATION_AUTO" => Some(Self::Auto),
            "VARIABLE_SYNCHRONIZATION_NONE" => Some(Self::None),
            "VARIABLE_SYNCHRONIZATION_ON_WRITE" => Some(Self::OnWrite),
            "VARIABLE_SYNCHRONIZATION_ON_READ" => Some(Self::OnRead),
            _ => None,
        }
    }
}
/// Indicates how a distributed variable will be aggregated.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VariableAggregation {
    /// `NONE`: This is the default, giving an error if you use a
    /// variable-update operation with multiple replicas.
    None = 0,
    /// `SUM`: Add the updates across replicas.
    Sum = 1,
    /// `MEAN`: Take the arithmetic mean ("average") of the updates across
    /// replicas.
    Mean = 2,
    /// `ONLY_FIRST_REPLICA`: This is for when every replica is performing the same
    /// update, but we only want to perform the update once. Used, e.g., for the
    /// global step counter.
    OnlyFirstReplica = 3,
}
impl VariableAggregation {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VariableAggregation::None => "VARIABLE_AGGREGATION_NONE",
            VariableAggregation::Sum => "VARIABLE_AGGREGATION_SUM",
            VariableAggregation::Mean => "VARIABLE_AGGREGATION_MEAN",
            VariableAggregation::OnlyFirstReplica => "VARIABLE_AGGREGATION_ONLY_FIRST_REPLICA",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "VARIABLE_AGGREGATION_NONE" => Some(Self::None),
            "VARIABLE_AGGREGATION_SUM" => Some(Self::Sum),
            "VARIABLE_AGGREGATION_MEAN" => Some(Self::Mean),
            "VARIABLE_AGGREGATION_ONLY_FIRST_REPLICA" => Some(Self::OnlyFirstReplica),
            _ => None,
        }
    }
}
/// `StructuredValue` represents a dynamically typed value representing various
/// data structures that are inspired by Python data structures typically used in
/// TensorFlow functions as inputs and outputs.
///
/// For example when saving a Layer there may be a `training` argument. If the
/// user passes a boolean True/False, that switches between two concrete
/// TensorFlow functions. In order to switch between them in the same way after
/// loading the SavedModel, we need to represent "True" and "False".
///
/// A more advanced example might be a function which takes a list of
/// dictionaries mapping from strings to Tensors. In order to map from
/// user-specified arguments `[{"a": tf.constant(1.)}, {"q": tf.constant(3.)}]`
/// after load to the right saved TensorFlow function, we need to represent the
/// nested structure and the strings, recording that we have a trace for anything
/// matching `[{"a": tf.TensorSpec(None, tf.float32)}, {"q": tf.TensorSpec([],
/// tf.float64)}]` as an example.
///
/// Likewise functions may return nested structures of Tensors, for example
/// returning a dictionary mapping from strings to Tensors. In order for the
/// loaded function to return the same structure we need to serialize it.
///
/// This is an ergonomic aid for working with loaded SavedModels, not a promise
/// to serialize all possible function signatures. For example we do not expect
/// to pickle generic Python objects, and ideally we'd stay language-agnostic.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StructuredValue {
    /// The kind of value.
    #[prost(
        oneof = "structured_value::Kind",
        tags = "1, 11, 12, 13, 14, 31, 32, 33, 34, 35, 51, 52, 53, 54"
    )]
    pub kind: ::core::option::Option<structured_value::Kind>,
}
/// Nested message and enum types in `StructuredValue`.
pub mod structured_value {
    /// The kind of value.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        /// Represents None.
        #[prost(message, tag = "1")]
        NoneValue(super::NoneValue),
        /// Represents a double-precision floating-point value (a Python `float`).
        #[prost(double, tag = "11")]
        Float64Value(f64),
        /// Represents a signed integer value, limited to 64 bits.
        /// Larger values from Python's arbitrary-precision integers are unsupported.
        #[prost(sint64, tag = "12")]
        Int64Value(i64),
        /// Represents a string of Unicode characters stored in a Python `str`.
        /// In Python 3, this is exactly what type `str` is.
        /// In Python 2, this is the UTF-8 encoding of the characters.
        /// For strings with ASCII characters only (as often used in TensorFlow code)
        /// there is effectively no difference between the language versions.
        /// The obsolescent `unicode` type of Python 2 is not supported here.
        #[prost(string, tag = "13")]
        StringValue(::prost::alloc::string::String),
        /// Represents a boolean value.
        #[prost(bool, tag = "14")]
        BoolValue(bool),
        /// Represents a TensorShape.
        #[prost(message, tag = "31")]
        TensorShapeValue(super::TensorShapeProto),
        /// Represents an enum value for dtype.
        #[prost(enumeration = "super::DataType", tag = "32")]
        TensorDtypeValue(i32),
        /// Represents a value for tf.TensorSpec.
        #[prost(message, tag = "33")]
        TensorSpecValue(super::TensorSpecProto),
        /// Represents a value for tf.TypeSpec.
        #[prost(message, tag = "34")]
        TypeSpecValue(::prost::alloc::boxed::Box<super::TypeSpecProto>),
        /// Represents a value for tf.BoundedTensorSpec.
        #[prost(message, tag = "35")]
        BoundedTensorSpecValue(super::BoundedTensorSpecProto),
        /// Represents a list of `Value`.
        #[prost(message, tag = "51")]
        ListValue(super::ListValue),
        /// Represents a tuple of `Value`.
        #[prost(message, tag = "52")]
        TupleValue(super::TupleValue),
        /// Represents a dict `Value`.
        #[prost(message, tag = "53")]
        DictValue(super::DictValue),
        /// Represents Python's namedtuple.
        #[prost(message, tag = "54")]
        NamedTupleValue(super::NamedTupleValue),
    }
}
/// Represents None.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NoneValue {}
/// Represents a Python list.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListValue {
    #[prost(message, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<StructuredValue>,
}
/// Represents a Python tuple.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TupleValue {
    #[prost(message, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<StructuredValue>,
}
/// Represents a Python dict keyed by `str`.
/// The comment on Unicode from Value.string_value applies analogously.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DictValue {
    #[prost(map = "string, message", tag = "1")]
    pub fields: ::std::collections::HashMap<::prost::alloc::string::String, StructuredValue>,
}
/// Represents a (key, value) pair.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PairValue {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<StructuredValue>,
}
/// Represents Python's namedtuple.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NamedTupleValue {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub values: ::prost::alloc::vec::Vec<PairValue>,
}
/// A protobuf to represent tf.TensorSpec.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TensorSpecProto {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub shape: ::core::option::Option<TensorShapeProto>,
    #[prost(enumeration = "DataType", tag = "3")]
    pub dtype: i32,
}
/// A protobuf to represent tf.BoundedTensorSpec.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoundedTensorSpecProto {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub shape: ::core::option::Option<TensorShapeProto>,
    #[prost(enumeration = "DataType", tag = "3")]
    pub dtype: i32,
    #[prost(message, optional, tag = "4")]
    pub minimum: ::core::option::Option<TensorProto>,
    #[prost(message, optional, tag = "5")]
    pub maximum: ::core::option::Option<TensorProto>,
}
/// Represents a tf.TypeSpec
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypeSpecProto {
    #[prost(enumeration = "type_spec_proto::TypeSpecClass", tag = "1")]
    pub type_spec_class: i32,
    /// The value returned by TypeSpec._serialize().
    #[prost(message, optional, boxed, tag = "2")]
    pub type_state: ::core::option::Option<::prost::alloc::boxed::Box<StructuredValue>>,
    /// The name of the TypeSpec class.
    ///   * If type_spec_class == REGISTERED_TYPE_SPEC, the TypeSpec class is
    ///     the one registered under this name. For types registered outside
    ///     core TensorFlow by an add-on library, that library must be loaded
    ///     before this value can be deserialized by nested_structure_coder.
    ///   * If type_spec_class specifies a particular TypeSpec class, this field is
    ///     redundant with the type_spec_class enum, and is only used for error
    ///     reporting in older binaries that do not know the tupe_spec_class enum.
    #[prost(string, tag = "3")]
    pub type_spec_class_name: ::prost::alloc::string::String,
    /// The number of flat tensor components required by this TypeSpec.
    #[prost(int32, tag = "4")]
    pub num_flat_components: i32,
}
/// Nested message and enum types in `TypeSpecProto`.
pub mod type_spec_proto {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TypeSpecClass {
        Unknown = 0,
        /// tf.SparseTensorSpec
        SparseTensorSpec = 1,
        /// tf.IndexedSlicesSpec
        IndexedSlicesSpec = 2,
        /// tf.RaggedTensorSpec
        RaggedTensorSpec = 3,
        /// tf.TensorArraySpec
        TensorArraySpec = 4,
        /// tf.data.DatasetSpec
        DataDatasetSpec = 5,
        /// IteratorSpec from data/ops/iterator_ops.py
        DataIteratorSpec = 6,
        /// tf.OptionalSpec
        OptionalSpec = 7,
        /// PerReplicaSpec from distribute/values.py
        PerReplicaSpec = 8,
        /// tf.VariableSpec
        VariableSpec = 9,
        /// RowPartitionSpec from ragged/row_partition.py
        RowPartitionSpec = 10,
        /// The type registered as type_spec_class_name.
        RegisteredTypeSpec = 12,
        /// Subclasses of tf.ExtensionType
        ExtensionTypeSpec = 13,
    }
    impl TypeSpecClass {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TypeSpecClass::Unknown => "UNKNOWN",
                TypeSpecClass::SparseTensorSpec => "SPARSE_TENSOR_SPEC",
                TypeSpecClass::IndexedSlicesSpec => "INDEXED_SLICES_SPEC",
                TypeSpecClass::RaggedTensorSpec => "RAGGED_TENSOR_SPEC",
                TypeSpecClass::TensorArraySpec => "TENSOR_ARRAY_SPEC",
                TypeSpecClass::DataDatasetSpec => "DATA_DATASET_SPEC",
                TypeSpecClass::DataIteratorSpec => "DATA_ITERATOR_SPEC",
                TypeSpecClass::OptionalSpec => "OPTIONAL_SPEC",
                TypeSpecClass::PerReplicaSpec => "PER_REPLICA_SPEC",
                TypeSpecClass::VariableSpec => "VARIABLE_SPEC",
                TypeSpecClass::RowPartitionSpec => "ROW_PARTITION_SPEC",
                TypeSpecClass::RegisteredTypeSpec => "REGISTERED_TYPE_SPEC",
                TypeSpecClass::ExtensionTypeSpec => "EXTENSION_TYPE_SPEC",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNKNOWN" => Some(Self::Unknown),
                "SPARSE_TENSOR_SPEC" => Some(Self::SparseTensorSpec),
                "INDEXED_SLICES_SPEC" => Some(Self::IndexedSlicesSpec),
                "RAGGED_TENSOR_SPEC" => Some(Self::RaggedTensorSpec),
                "TENSOR_ARRAY_SPEC" => Some(Self::TensorArraySpec),
                "DATA_DATASET_SPEC" => Some(Self::DataDatasetSpec),
                "DATA_ITERATOR_SPEC" => Some(Self::DataIteratorSpec),
                "OPTIONAL_SPEC" => Some(Self::OptionalSpec),
                "PER_REPLICA_SPEC" => Some(Self::PerReplicaSpec),
                "VARIABLE_SPEC" => Some(Self::VariableSpec),
                "ROW_PARTITION_SPEC" => Some(Self::RowPartitionSpec),
                "REGISTERED_TYPE_SPEC" => Some(Self::RegisteredTypeSpec),
                "EXTENSION_TYPE_SPEC" => Some(Self::ExtensionTypeSpec),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrackableObjectGraph {
    #[prost(message, repeated, tag = "1")]
    pub nodes: ::prost::alloc::vec::Vec<trackable_object_graph::TrackableObject>,
}
/// Nested message and enum types in `TrackableObjectGraph`.
pub mod trackable_object_graph {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TrackableObject {
        /// Objects which this object depends on.
        #[prost(message, repeated, tag = "1")]
        pub children: ::prost::alloc::vec::Vec<trackable_object::ObjectReference>,
        /// Serialized data specific to this object.
        #[prost(message, repeated, tag = "2")]
        pub attributes: ::prost::alloc::vec::Vec<trackable_object::SerializedTensor>,
        /// Slot variables owned by this object.
        #[prost(message, repeated, tag = "3")]
        pub slot_variables: ::prost::alloc::vec::Vec<trackable_object::SlotVariableReference>,
        /// The registered saver used to save this object. If this saver is not
        /// present when loading the checkpoint, then loading will fail.
        #[prost(message, optional, tag = "4")]
        pub registered_saver: ::core::option::Option<super::RegisteredSaver>,
        /// Whether this object has checkpoint values or descendants with checkpoint
        /// values. This is computed at save time to avoid traversing the entire
        /// object graph proto when restoring (which also has to traverse the live
        /// object graph).
        #[prost(message, optional, tag = "5")]
        pub has_checkpoint_values: ::core::option::Option<bool>,
    }
    /// Nested message and enum types in `TrackableObject`.
    pub mod trackable_object {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ObjectReference {
            /// An index into `TrackableObjectGraph.nodes`, indicating the object
            /// being referenced.
            #[prost(int32, tag = "1")]
            pub node_id: i32,
            /// A user-provided name for the edge.
            #[prost(string, tag = "2")]
            pub local_name: ::prost::alloc::string::String,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct SerializedTensor {
            /// A name for the Tensor. Simple variables have only one
            /// `SerializedTensor` named "VARIABLE_VALUE" by convention. This value may
            /// be restored on object creation as an optimization.
            #[prost(string, tag = "1")]
            pub name: ::prost::alloc::string::String,
            /// The full name of the variable/tensor, if applicable. Used to allow
            /// name-based loading of checkpoints which were saved using an
            /// object-based API. Should match the checkpoint key which would have been
            /// assigned by tf.train.Saver.
            #[prost(string, tag = "2")]
            pub full_name: ::prost::alloc::string::String,
            /// The generated name of the Tensor in the checkpoint.
            #[prost(string, tag = "3")]
            pub checkpoint_key: ::prost::alloc::string::String,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct SlotVariableReference {
            /// An index into `TrackableObjectGraph.nodes`, indicating the
            /// variable object this slot was created for.
            #[prost(int32, tag = "1")]
            pub original_variable_node_id: i32,
            /// The name of the slot (e.g. "m"/"v").
            #[prost(string, tag = "2")]
            pub slot_name: ::prost::alloc::string::String,
            /// An index into `TrackableObjectGraph.nodes`, indicating the
            /// `Object` with the value of the slot variable.
            #[prost(int32, tag = "3")]
            pub slot_variable_node_id: i32,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisteredSaver {
    /// The name of the registered saver/restore function.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Unique auto-generated name of the object.
    #[prost(string, tag = "2")]
    pub object_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SavedObjectGraph {
    /// Flattened list of objects in the object graph.
    ///
    /// The position of the object in this list indicates its id.
    /// Nodes\[0\] is considered the root node.
    #[prost(message, repeated, tag = "1")]
    pub nodes: ::prost::alloc::vec::Vec<SavedObject>,
    /// Information about captures and output structures in concrete functions.
    /// Referenced from SavedBareConcreteFunction and SavedFunction.
    #[prost(map = "string, message", tag = "2")]
    pub concrete_functions:
        ::std::collections::HashMap<::prost::alloc::string::String, SavedConcreteFunction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SavedObject {
    /// Objects which this object depends on: named edges in the dependency
    /// graph.
    ///
    /// Note: All kinds of SavedObject may have children, except
    /// "constant" and "captured_tensor".
    #[prost(message, repeated, tag = "1")]
    pub children:
        ::prost::alloc::vec::Vec<trackable_object_graph::trackable_object::ObjectReference>,
    /// Ordered list of dependencies that must be loaded before this object.
    /// SavedModel loads with the bottom-up approach, by first creating all objects
    /// (in the order defined by the dependencies), then connecting the edges.
    #[prost(message, repeated, tag = "15")]
    pub dependencies:
        ::prost::alloc::vec::Vec<trackable_object_graph::trackable_object::ObjectReference>,
    /// Slot variables owned by this object. This describes the three-way
    /// (optimizer, variable, slot variable) relationship; none of the three
    /// depend on the others directly.
    ///
    /// Note: currently only valid if kind == "user_object".
    #[prost(message, repeated, tag = "3")]
    pub slot_variables:
        ::prost::alloc::vec::Vec<trackable_object_graph::trackable_object::SlotVariableReference>,
    /// Stores the functions used to save and restore this object. At most one of
    /// `saveable_objects` or `registered_saver` is defined for each SavedObject.
    /// See the comment below for the difference between SaveableObject and
    /// registered savers.
    #[prost(map = "string, message", tag = "11")]
    pub saveable_objects:
        ::std::collections::HashMap<::prost::alloc::string::String, SaveableObject>,
    /// The name of the registered class of the form "{package}.{class_name}".
    /// This field is used to search for the registered class at loading time.
    #[prost(string, tag = "13")]
    pub registered_name: ::prost::alloc::string::String,
    /// The user-generated proto storing metadata for this object, to be passed to
    /// the registered classes's _deserialize_from_proto method when this object is
    /// loaded from the SavedModel.
    #[prost(message, optional, tag = "14")]
    pub serialized_user_proto: ::core::option::Option<::prost_types::Any>,
    /// String name of the registered saver. At most one of `saveable_objects` or
    /// `registered_saver` is defined for each SavedObject.
    #[prost(string, tag = "16")]
    pub registered_saver: ::prost::alloc::string::String,
    #[prost(oneof = "saved_object::Kind", tags = "4, 5, 6, 7, 8, 9, 10, 12")]
    pub kind: ::core::option::Option<saved_object::Kind>,
}
/// Nested message and enum types in `SavedObject`.
pub mod saved_object {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        #[prost(message, tag = "4")]
        UserObject(super::SavedUserObject),
        #[prost(message, tag = "5")]
        Asset(super::SavedAsset),
        #[prost(message, tag = "6")]
        Function(super::SavedFunction),
        #[prost(message, tag = "7")]
        Variable(super::SavedVariable),
        #[prost(message, tag = "8")]
        BareConcreteFunction(super::SavedBareConcreteFunction),
        #[prost(message, tag = "9")]
        Constant(super::SavedConstant),
        #[prost(message, tag = "10")]
        Resource(super::SavedResource),
        #[prost(message, tag = "12")]
        CapturedTensor(super::CapturedTensor),
    }
}
/// A SavedUserObject is an object (in the object-oriented language of the
/// TensorFlow program) of some user- or framework-defined class other than
/// those handled specifically by the other kinds of SavedObjects.
///
/// This object cannot be evaluated as a tensor, and therefore cannot be bound
/// to an input of a function.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SavedUserObject {
    /// Corresponds to a registration of the type to use in the loading program.
    #[prost(string, tag = "1")]
    pub identifier: ::prost::alloc::string::String,
    /// Version information from the producer of this SavedUserObject.
    #[prost(message, optional, tag = "2")]
    pub version: ::core::option::Option<VersionDef>,
    /// Metadata for deserializing this object.
    ///
    /// Deprecated! At the time of deprecation, Keras was the only user of this
    /// field, and its saving and loading code will be updated shortly.
    /// Please save your application-specific metadata to a separate file.
    #[deprecated]
    #[prost(string, tag = "3")]
    pub metadata: ::prost::alloc::string::String,
}
/// A SavedAsset points to an asset in the MetaGraph.
///
/// When bound to a function this object evaluates to a tensor with the absolute
/// filename. Users should not depend on a particular part of the filename to
/// remain stable (e.g. basename could be changed).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SavedAsset {
    /// Index into `MetaGraphDef.asset_file_def[]` that describes the Asset.
    ///
    /// Only the field `AssetFileDef.filename` is used. Other fields, such as
    /// `AssetFileDef.tensor_info`, MUST be ignored.
    #[prost(int32, tag = "1")]
    pub asset_file_def_index: i32,
}
/// A function with multiple signatures, possibly with non-Tensor arguments.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SavedFunction {
    #[prost(string, repeated, tag = "1")]
    pub concrete_functions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "2")]
    pub function_spec: ::core::option::Option<FunctionSpec>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CapturedTensor {
    /// Name of captured tensor
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Name of concrete function which contains the computed graph tensor.
    #[prost(string, tag = "2")]
    pub concrete_function: ::prost::alloc::string::String,
}
/// Stores low-level information about a concrete function. Referenced in either
/// a SavedFunction or a SavedBareConcreteFunction.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SavedConcreteFunction {
    #[prost(int32, repeated, tag = "2")]
    pub bound_inputs: ::prost::alloc::vec::Vec<i32>,
    /// Input in canonicalized form that was received to create this concrete
    /// function.
    #[prost(message, optional, tag = "3")]
    pub canonicalized_input_signature: ::core::option::Option<StructuredValue>,
    /// Output that was the return value of this function after replacing all
    /// Tensors with TensorSpecs. This can be an arbitrary nested function and will
    /// be used to reconstruct the full structure from pure tensors.
    #[prost(message, optional, tag = "4")]
    pub output_signature: ::core::option::Option<StructuredValue>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SavedBareConcreteFunction {
    /// Identifies a SavedConcreteFunction.
    #[prost(string, tag = "1")]
    pub concrete_function_name: ::prost::alloc::string::String,
    /// A sequence of unique strings, one per Tensor argument.
    #[prost(string, repeated, tag = "2")]
    pub argument_keywords: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The prefix of `argument_keywords` which may be identified by position.
    #[prost(int64, tag = "3")]
    pub allowed_positional_arguments: i64,
    /// The spec of the function that this ConcreteFunction is traced from. This
    /// allows the ConcreteFunction to be called with nest structure inputs. This
    /// field may not be populated. If this field is absent, the concrete function
    /// can only be called with flat inputs.
    /// TODO(b/169361281): support calling saved ConcreteFunction with structured
    /// inputs in C++ SavedModel API.
    #[prost(message, optional, tag = "4")]
    pub function_spec: ::core::option::Option<FunctionSpec>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SavedConstant {
    /// An Operation name for a ConstantOp in this SavedObjectGraph's MetaGraph.
    #[prost(string, tag = "1")]
    pub operation: ::prost::alloc::string::String,
}
/// Represents a Variable that is initialized by loading the contents from the
/// checkpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SavedVariable {
    #[prost(enumeration = "DataType", tag = "1")]
    pub dtype: i32,
    #[prost(message, optional, tag = "2")]
    pub shape: ::core::option::Option<TensorShapeProto>,
    #[prost(bool, tag = "3")]
    pub trainable: bool,
    #[prost(enumeration = "VariableSynchronization", tag = "4")]
    pub synchronization: i32,
    #[prost(enumeration = "VariableAggregation", tag = "5")]
    pub aggregation: i32,
    #[prost(string, tag = "6")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub device: ::prost::alloc::string::String,
    /// List of component variables for a distributed variable.
    ///
    /// When this field is non-empty, the SavedVariable will be assumed
    /// to be a distributed variable defined by the components listed here.
    ///
    /// This is only supported by experimental loaders at the moment.
    #[prost(message, repeated, tag = "8")]
    pub experimental_distributed_variable_components: ::prost::alloc::vec::Vec<SavedVariable>,
}
/// Represents `FunctionSpec` used in `Function`. This represents a
/// function that has been wrapped as a TensorFlow `Function`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunctionSpec {
    /// Full arg spec from inspect.getfullargspec().
    #[prost(message, optional, tag = "1")]
    pub fullargspec: ::core::option::Option<StructuredValue>,
    /// Whether this represents a class method.
    #[prost(bool, tag = "2")]
    pub is_method: bool,
    /// The input signature, if specified.
    #[prost(message, optional, tag = "5")]
    pub input_signature: ::core::option::Option<StructuredValue>,
    #[prost(enumeration = "function_spec::JitCompile", tag = "6")]
    pub jit_compile: i32,
}
/// Nested message and enum types in `FunctionSpec`.
pub mod function_spec {
    /// Whether the function should be compiled by XLA.
    ///
    /// The public interface to `tf.function` uses an optional boolean to
    /// represent three distinct states for this field.  Unfortunately, proto3
    /// removes the ability to explicitly check for the presence or absence of a
    /// field, so we instead map to an enum.
    ///
    /// See `tf.function` for details.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum JitCompile {
        Default = 0,
        On = 1,
        Off = 2,
    }
    impl JitCompile {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                JitCompile::Default => "DEFAULT",
                JitCompile::On => "ON",
                JitCompile::Off => "OFF",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DEFAULT" => Some(Self::Default),
                "ON" => Some(Self::On),
                "OFF" => Some(Self::Off),
                _ => None,
            }
        }
    }
}
/// A SavedResource represents a TF object that holds state during its lifetime.
/// An object of this type can have a reference to a:
/// create_resource() and an initialize() function.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SavedResource {
    /// A device specification indicating a required placement for the resource
    /// creation function, e.g. "CPU". An empty string allows the user to select a
    /// device.
    #[prost(string, tag = "1")]
    pub device: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SaveableObject {
    /// Node ids of concrete functions for saving and loading from a checkpoint.
    /// These functions save and restore directly from tensors.
    #[prost(int32, tag = "2")]
    pub save_function: i32,
    #[prost(int32, tag = "3")]
    pub restore_function: i32,
}
/// Protocol buffer representing the configuration of a Saver.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SaverDef {
    /// The name of the tensor in which to specify the filename when saving or
    /// restoring a model checkpoint.
    #[prost(string, tag = "1")]
    pub filename_tensor_name: ::prost::alloc::string::String,
    /// The operation to run when saving a model checkpoint.
    #[prost(string, tag = "2")]
    pub save_tensor_name: ::prost::alloc::string::String,
    /// The operation to run when restoring a model checkpoint.
    #[prost(string, tag = "3")]
    pub restore_op_name: ::prost::alloc::string::String,
    /// Maximum number of checkpoints to keep.  If 0, no checkpoints are deleted.
    #[prost(int32, tag = "4")]
    pub max_to_keep: i32,
    /// Shard the save files, one per device that has Variable nodes.
    #[prost(bool, tag = "5")]
    pub sharded: bool,
    /// How often to keep an additional checkpoint. If not specified, only the last
    /// "max_to_keep" checkpoints are kept; if specified, in addition to keeping
    /// the last "max_to_keep" checkpoints, an additional checkpoint will be kept
    /// for every n hours of training.
    #[prost(float, tag = "6")]
    pub keep_checkpoint_every_n_hours: f32,
    #[prost(enumeration = "saver_def::CheckpointFormatVersion", tag = "7")]
    pub version: i32,
}
/// Nested message and enum types in `SaverDef`.
pub mod saver_def {
    /// A version number that identifies a different on-disk checkpoint format.
    /// Usually, each subclass of BaseSaverBuilder works with a particular
    /// version/format.  However, it is possible that the same builder may be
    /// upgraded to support a newer checkpoint format in the future.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CheckpointFormatVersion {
        /// Internal legacy format.
        Legacy = 0,
        /// Deprecated format: tf.Saver() which works with tensorflow::table::Table.
        V1 = 1,
        /// Current format: more efficient.
        V2 = 2,
    }
    impl CheckpointFormatVersion {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CheckpointFormatVersion::Legacy => "LEGACY",
                CheckpointFormatVersion::V1 => "V1",
                CheckpointFormatVersion::V2 => "V2",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "LEGACY" => Some(Self::Legacy),
                "V1" => Some(Self::V1),
                "V2" => Some(Self::V2),
                _ => None,
            }
        }
    }
}
/// Protocol buffer containing the following which are necessary to restart
/// training, run inference. It can be used to serialize/de-serialize memory
/// objects necessary for running computation in a graph when crossing the
/// process boundary. It can be used for long term storage of graphs,
/// cross-language execution of graphs, etc.
///    MetaInfoDef
///    GraphDef
///    SaverDef
///    CollectionDef
///    TensorInfo
///    SignatureDef
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetaGraphDef {
    #[prost(message, optional, tag = "1")]
    pub meta_info_def: ::core::option::Option<meta_graph_def::MetaInfoDef>,
    /// GraphDef.
    #[prost(message, optional, tag = "2")]
    pub graph_def: ::core::option::Option<GraphDef>,
    /// SaverDef.
    #[prost(message, optional, tag = "3")]
    pub saver_def: ::core::option::Option<SaverDef>,
    /// collection_def: Map from collection name to collections.
    /// See CollectionDef section for details.
    #[prost(map = "string, message", tag = "4")]
    pub collection_def: ::std::collections::HashMap<::prost::alloc::string::String, CollectionDef>,
    /// signature_def: Map from user supplied key for a signature to a single
    /// SignatureDef.
    #[prost(map = "string, message", tag = "5")]
    pub signature_def: ::std::collections::HashMap<::prost::alloc::string::String, SignatureDef>,
    /// Asset file def to be used with the defined graph.
    #[prost(message, repeated, tag = "6")]
    pub asset_file_def: ::prost::alloc::vec::Vec<AssetFileDef>,
    /// Extra information about the structure of functions and stateful objects.
    #[prost(message, optional, tag = "7")]
    pub object_graph_def: ::core::option::Option<SavedObjectGraph>,
}
/// Nested message and enum types in `MetaGraphDef`.
pub mod meta_graph_def {
    /// Meta information regarding the graph to be exported.  To be used by users
    /// of this protocol buffer to encode information regarding their meta graph.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MetaInfoDef {
        /// User specified Version string. Can be the name of the model and revision,
        /// steps this model has been trained to, etc.
        #[prost(string, tag = "1")]
        pub meta_graph_version: ::prost::alloc::string::String,
        /// A copy of the OpDefs used by the producer of this graph_def.
        /// Descriptions and Ops not used in graph_def are stripped out.
        #[prost(message, optional, tag = "2")]
        pub stripped_op_list: ::core::option::Option<super::OpList>,
        /// A serialized protobuf. Can be the time this meta graph is created, or
        /// modified, or name of the model.
        #[prost(message, optional, tag = "3")]
        pub any_info: ::core::option::Option<::prost_types::Any>,
        /// User supplied tag(s) on the meta_graph and included graph_def.
        ///
        /// MetaGraphDefs should be tagged with their capabilities or use-cases.
        /// Examples: "train", "serve", "gpu", "tpu", etc.
        /// These tags enable loaders to access the MetaGraph(s) appropriate for a
        /// specific use-case or runtime environment.
        #[prost(string, repeated, tag = "4")]
        pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// The __version__ string of the tensorflow build used to write this graph.
        /// This will be populated by the framework, which will overwrite any user
        /// supplied value.
        #[prost(string, tag = "5")]
        pub tensorflow_version: ::prost::alloc::string::String,
        /// The __git_version__ string of the tensorflow build used to write this
        /// graph. This will be populated by the framework, which will overwrite any
        /// user supplied value.
        #[prost(string, tag = "6")]
        pub tensorflow_git_version: ::prost::alloc::string::String,
        /// A flag to denote whether default-valued attrs have been stripped from
        /// the nodes in this graph_def.
        #[prost(bool, tag = "7")]
        pub stripped_default_attrs: bool,
        /// FunctionDef name to aliases mapping.
        #[prost(map = "string, string", tag = "8")]
        pub function_aliases: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            ::prost::alloc::string::String,
        >,
    }
}
/// CollectionDef should cover most collections.
/// To add a user-defined collection, do one of the following:
/// 1. For simple data types, such as string, int, float:
///       tf.add_to_collection("your_collection_name", your_simple_value)
///     strings will be stored as bytes_list.
///
/// 2. For Protobuf types, there are three ways to add them:
///     1) tf.add_to_collection("your_collection_name",
///          your_proto.SerializeToString())
///
///        collection_def {
///          key: "user_defined_bytes_collection"
///          value {
///            bytes_list {
///              value: "queue_name: \"test_queue\"\n"
///            }
///          }
///        }
///
///   or
///
///     2) tf.add_to_collection("your_collection_name", str(your_proto))
///
///        collection_def {
///          key: "user_defined_string_collection"
///          value {
///           bytes_list {
///              value: "\n\ntest_queue"
///            }
///          }
///        }
///
///   or
///
///     3) any_buf = any_pb2.Any()
///        tf.add_to_collection("your_collection_name",
///          any_buf.Pack(your_proto))
///
///        collection_def {
///          key: "user_defined_any_collection"
///          value {
///            any_list {
///              value {
///                type_url: "type.googleapis.com/tensorflow.QueueRunnerDef"
///                value: "\n\ntest_queue"
///              }
///            }
///          }
///        }
///
/// 3. For Python objects, implement to_proto() and from_proto(), and register
///     them in the following manner:
///     ops.register_proto_function("your_collection_name",
///                                 proto_type,
///                                 to_proto=YourPythonObject.to_proto,
///                                 from_proto=YourPythonObject.from_proto)
///     These functions will be invoked to serialize and de-serialize the
///     collection. For example,
///     ops.register_proto_function(ops.GraphKeys.GLOBAL_VARIABLES,
///                                 proto_type=variable_pb2.VariableDef,
///                                 to_proto=Variable.to_proto,
///                                 from_proto=Variable.from_proto)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectionDef {
    #[prost(oneof = "collection_def::Kind", tags = "1, 2, 3, 4, 5")]
    pub kind: ::core::option::Option<collection_def::Kind>,
}
/// Nested message and enum types in `CollectionDef`.
pub mod collection_def {
    /// NodeList is used for collecting nodes in graph. For example
    /// collection_def {
    ///    key: "summaries"
    ///    value {
    ///      node_list {
    ///        value: "input_producer/ScalarSummary:0"
    ///        value: "shuffle_batch/ScalarSummary:0"
    ///        value: "ImageSummary:0"
    ///      }
    ///    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NodeList {
        #[prost(string, repeated, tag = "1")]
        pub value: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// BytesList is used for collecting strings and serialized protobufs. For
    /// example:
    /// collection_def {
    ///    key: "trainable_variables"
    ///    value {
    ///      bytes_list {
    ///        value: "\n\017conv1/weights:0\022\024conv1/weights/Assign
    ///               \032\024conv1/weights/read:0"
    ///        value: "\n\016conv1/biases:0\022\023conv1/biases/Assign\032
    ///               \023conv1/biases/read:0"
    ///      }
    ///    }
    /// }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BytesList {
        #[prost(bytes = "vec", repeated, tag = "1")]
        pub value: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    }
    /// Int64List is used for collecting int, int64 and long values.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Int64List {
        #[prost(int64, repeated, tag = "1")]
        pub value: ::prost::alloc::vec::Vec<i64>,
    }
    /// FloatList is used for collecting float values.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FloatList {
        #[prost(float, repeated, tag = "1")]
        pub value: ::prost::alloc::vec::Vec<f32>,
    }
    /// AnyList is used for collecting Any protos.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AnyList {
        #[prost(message, repeated, tag = "1")]
        pub value: ::prost::alloc::vec::Vec<::prost_types::Any>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        #[prost(message, tag = "1")]
        NodeList(NodeList),
        #[prost(message, tag = "2")]
        BytesList(BytesList),
        #[prost(message, tag = "3")]
        Int64List(Int64List),
        #[prost(message, tag = "4")]
        FloatList(FloatList),
        #[prost(message, tag = "5")]
        AnyList(AnyList),
    }
}
/// Information about a Tensor necessary for feeding or retrieval.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TensorInfo {
    #[prost(enumeration = "DataType", tag = "2")]
    pub dtype: i32,
    /// The static shape should be recorded here, to the extent that it can
    /// be known in advance.  In the case of a SparseTensor, this field describes
    /// the logical shape of the represented tensor (aka dense_shape).
    #[prost(message, optional, tag = "3")]
    pub tensor_shape: ::core::option::Option<TensorShapeProto>,
    #[prost(oneof = "tensor_info::Encoding", tags = "1, 4, 5")]
    pub encoding: ::core::option::Option<tensor_info::Encoding>,
}
/// Nested message and enum types in `TensorInfo`.
pub mod tensor_info {
    /// For sparse tensors, The COO encoding stores a triple of values, indices,
    /// and shape.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CooSparse {
        /// The shape of the values Tensor is \[?\].  Its dtype must be the dtype of
        /// the SparseTensor as a whole, given in the enclosing TensorInfo.
        #[prost(string, tag = "1")]
        pub values_tensor_name: ::prost::alloc::string::String,
        /// The indices Tensor must have dtype int64 and shape [?, ?].
        #[prost(string, tag = "2")]
        pub indices_tensor_name: ::prost::alloc::string::String,
        /// The dynamic logical shape represented by the SparseTensor is recorded in
        /// the Tensor referenced here.  It must have dtype int64 and shape \[?\].
        #[prost(string, tag = "3")]
        pub dense_shape_tensor_name: ::prost::alloc::string::String,
    }
    /// Generic encoding for composite tensors.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CompositeTensor {
        /// The serialized TypeSpec for the composite tensor.
        #[prost(message, optional, tag = "1")]
        pub type_spec: ::core::option::Option<super::TypeSpecProto>,
        /// A TensorInfo for each flattened component tensor.
        #[prost(message, repeated, tag = "2")]
        pub components: ::prost::alloc::vec::Vec<super::TensorInfo>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Encoding {
        /// For dense `Tensor`s, the name of the tensor in the graph.
        #[prost(string, tag = "1")]
        Name(::prost::alloc::string::String),
        /// There are many possible encodings of sparse matrices
        /// (<https://en.wikipedia.org/wiki/Sparse_matrix>).  Currently, TensorFlow
        /// uses only the COO encoding.  This is supported and documented in the
        /// SparseTensor Python class.
        #[prost(message, tag = "4")]
        CooSparse(CooSparse),
        /// Generic encoding for CompositeTensors.
        #[prost(message, tag = "5")]
        CompositeTensor(CompositeTensor),
    }
}
/// SignatureDef defines the signature of a computation supported by a TensorFlow
/// graph.
///
/// For example, a model with two loss computations, sharing a single input,
/// might have the following signature_def map, in a MetaGraphDef message.
///
/// Note that across the two SignatureDefs "loss_A" and "loss_B", the input key,
/// output key, and method_name are identical, and will be used by system(s) that
/// implement or rely upon this particular loss method. The output tensor names
/// differ, demonstrating how different outputs can exist for the same method.
///
/// signature_def {
///    key: "loss_A"
///    value {
///      inputs {
///        key: "input"
///        value {
///          name: "input:0"
///          dtype: DT_STRING
///          tensor_shape: ...
///        }
///      }
///      outputs {
///        key: "loss_output"
///        value {
///          name: "loss_output_A:0"
///          dtype: DT_FLOAT
///          tensor_shape: ...
///        }
///      }
///      method_name: "some/package/compute_loss"
///    }
///    ...
/// }
/// signature_def {
///    key: "loss_B"
///    value {
///      inputs {
///        key: "input"
///        value {
///          name: "input:0"
///          dtype: DT_STRING
///          tensor_shape: ...
///        }
///      }
///      outputs {
///        key: "loss_output"
///        value {
///          name: "loss_output_B:0"
///          dtype: DT_FLOAT
///          tensor_shape: ...
///        }
///      }
///      method_name: "some/package/compute_loss"
///    }
///    ...
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignatureDef {
    /// Named input parameters.
    #[prost(map = "string, message", tag = "1")]
    pub inputs: ::std::collections::HashMap<::prost::alloc::string::String, TensorInfo>,
    /// Named output parameters.
    #[prost(map = "string, message", tag = "2")]
    pub outputs: ::std::collections::HashMap<::prost::alloc::string::String, TensorInfo>,
    /// Extensible method_name information enabling third-party users to mark a
    /// SignatureDef as supporting a particular method. This enables producers and
    /// consumers of SignatureDefs, e.g. a model definition library and a serving
    /// library to have a clear hand-off regarding the semantics of a computation.
    ///
    /// Note that multiple SignatureDefs in a single MetaGraphDef may have the same
    /// method_name. This is commonly used to support multi-headed computation,
    /// where a single graph computation may return multiple results.
    #[prost(string, tag = "3")]
    pub method_name: ::prost::alloc::string::String,
}
/// An asset file def for a single file or a set of sharded files with the same
/// name.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetFileDef {
    /// The tensor to bind the asset filename to.
    #[prost(message, optional, tag = "1")]
    pub tensor_info: ::core::option::Option<TensorInfo>,
    /// The filename within an assets directory. Note: does not include the path
    /// prefix, i.e. directories. For an asset at /tmp/path/vocab.txt, the filename
    /// would be "vocab.txt".
    #[prost(string, tag = "2")]
    pub filename: ::prost::alloc::string::String,
}
