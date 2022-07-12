#![allow(unused_qualifications)]

use crate::models;
#[cfg(any(feature = "client", feature = "server"))]
use crate::header;

#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ImageFormat(swagger::ByteArray);

impl std::convert::From<swagger::ByteArray> for ImageFormat {
    fn from(x: swagger::ByteArray) -> Self {
        ImageFormat(x)
    }
}

impl std::convert::From<ImageFormat> for swagger::ByteArray {
    fn from(x: ImageFormat) -> Self {
        x.0
    }
}

impl std::ops::Deref for ImageFormat {
    type Target = swagger::ByteArray;
    fn deref(&self) -> &swagger::ByteArray {
        &self.0
    }
}

impl std::ops::DerefMut for ImageFormat {
    fn deref_mut(&mut self) -> &mut swagger::ByteArray {
        &mut self.0
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SeriesGet200ResponseInner {
    #[serde(rename = "SeriesDescription")]
    #[serde(deserialize_with = "swagger::nullable_format::deserialize_optional_nullable")]
    #[serde(default = "swagger::nullable_format::default_optional_nullable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub series_description: Option<swagger::Nullable<String>>,

    #[serde(rename = "SeriesDate")]
    #[serde(deserialize_with = "swagger::nullable_format::deserialize_optional_nullable")]
    #[serde(default = "swagger::nullable_format::default_optional_nullable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub series_date: Option<swagger::Nullable<String>>,

    #[serde(rename = "link")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub link: Option<String>,

}

impl SeriesGet200ResponseInner {
    pub fn new() -> SeriesGet200ResponseInner {
        SeriesGet200ResponseInner {
            series_description: None,
            series_date: None,
            link: None,
        }
    }
}

/// Converts the SeriesGet200ResponseInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for SeriesGet200ResponseInner {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref series_description) = self.series_description {
            params.push("SeriesDescription".to_string());
            params.push(series_description.as_ref().map_or("null".to_string(), |x| x.to_string()));
        }


        if let Some(ref series_date) = self.series_date {
            params.push("SeriesDate".to_string());
            params.push(series_date.as_ref().map_or("null".to_string(), |x| x.to_string()));
        }


        if let Some(ref link) = self.link {
            params.push("link".to_string());
            params.push(link.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SeriesGet200ResponseInner value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SeriesGet200ResponseInner {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub series_description: Vec<String>,
            pub series_date: Vec<String>,
            pub link: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing SeriesGet200ResponseInner".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "SeriesDescription" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in SeriesGet200ResponseInner".to_string()),
                    "SeriesDate" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in SeriesGet200ResponseInner".to_string()),
                    "link" => intermediate_rep.link.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing SeriesGet200ResponseInner".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(SeriesGet200ResponseInner {
            series_description: std::result::Result::Err("Nullable types not supported in SeriesGet200ResponseInner".to_string())?,
            series_date: std::result::Result::Err("Nullable types not supported in SeriesGet200ResponseInner".to_string())?,
            link: intermediate_rep.link.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<SeriesGet200ResponseInner> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<SeriesGet200ResponseInner>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<SeriesGet200ResponseInner>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for SeriesGet200ResponseInner - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<SeriesGet200ResponseInner> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <SeriesGet200ResponseInner as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into SeriesGet200ResponseInner - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct StudiesGet200ResponseInner {
    #[serde(rename = "study")]
    #[serde(deserialize_with = "swagger::nullable_format::deserialize_optional_nullable")]
    #[serde(default = "swagger::nullable_format::default_optional_nullable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub study: Option<swagger::Nullable<String>>,

    #[serde(rename = "date")]
    #[serde(deserialize_with = "swagger::nullable_format::deserialize_optional_nullable")]
    #[serde(default = "swagger::nullable_format::default_optional_nullable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub date: Option<swagger::Nullable<String>>,

    #[serde(rename = "link")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub link: Option<String>,

}

impl StudiesGet200ResponseInner {
    pub fn new() -> StudiesGet200ResponseInner {
        StudiesGet200ResponseInner {
            study: None,
            date: None,
            link: None,
        }
    }
}

/// Converts the StudiesGet200ResponseInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for StudiesGet200ResponseInner {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref study) = self.study {
            params.push("study".to_string());
            params.push(study.as_ref().map_or("null".to_string(), |x| x.to_string()));
        }


        if let Some(ref date) = self.date {
            params.push("date".to_string());
            params.push(date.as_ref().map_or("null".to_string(), |x| x.to_string()));
        }


        if let Some(ref link) = self.link {
            params.push("link".to_string());
            params.push(link.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a StudiesGet200ResponseInner value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for StudiesGet200ResponseInner {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub study: Vec<String>,
            pub date: Vec<String>,
            pub link: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing StudiesGet200ResponseInner".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "study" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in StudiesGet200ResponseInner".to_string()),
                    "date" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in StudiesGet200ResponseInner".to_string()),
                    "link" => intermediate_rep.link.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing StudiesGet200ResponseInner".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(StudiesGet200ResponseInner {
            study: std::result::Result::Err("Nullable types not supported in StudiesGet200ResponseInner".to_string())?,
            date: std::result::Result::Err("Nullable types not supported in StudiesGet200ResponseInner".to_string())?,
            link: intermediate_rep.link.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<StudiesGet200ResponseInner> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<StudiesGet200ResponseInner>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<StudiesGet200ResponseInner>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for StudiesGet200ResponseInner - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<StudiesGet200ResponseInner> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <StudiesGet200ResponseInner as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into StudiesGet200ResponseInner - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct StudiesStudyInstanceUidSeriesSeriesInstanceUidInstancesGet200ResponseInner {
    #[serde(rename = "link")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub link: Option<String>,

}

impl StudiesStudyInstanceUidSeriesSeriesInstanceUidInstancesGet200ResponseInner {
    pub fn new() -> StudiesStudyInstanceUidSeriesSeriesInstanceUidInstancesGet200ResponseInner {
        StudiesStudyInstanceUidSeriesSeriesInstanceUidInstancesGet200ResponseInner {
            link: None,
        }
    }
}

/// Converts the StudiesStudyInstanceUidSeriesSeriesInstanceUidInstancesGet200ResponseInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for StudiesStudyInstanceUidSeriesSeriesInstanceUidInstancesGet200ResponseInner {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        if let Some(ref link) = self.link {
            params.push("link".to_string());
            params.push(link.to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a StudiesStudyInstanceUidSeriesSeriesInstanceUidInstancesGet200ResponseInner value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for StudiesStudyInstanceUidSeriesSeriesInstanceUidInstancesGet200ResponseInner {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub link: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing StudiesStudyInstanceUidSeriesSeriesInstanceUidInstancesGet200ResponseInner".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "link" => intermediate_rep.link.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing StudiesStudyInstanceUidSeriesSeriesInstanceUidInstancesGet200ResponseInner".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(StudiesStudyInstanceUidSeriesSeriesInstanceUidInstancesGet200ResponseInner {
            link: intermediate_rep.link.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<StudiesStudyInstanceUidSeriesSeriesInstanceUidInstancesGet200ResponseInner> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<StudiesStudyInstanceUidSeriesSeriesInstanceUidInstancesGet200ResponseInner>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<StudiesStudyInstanceUidSeriesSeriesInstanceUidInstancesGet200ResponseInner>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for StudiesStudyInstanceUidSeriesSeriesInstanceUidInstancesGet200ResponseInner - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<StudiesStudyInstanceUidSeriesSeriesInstanceUidInstancesGet200ResponseInner> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <StudiesStudyInstanceUidSeriesSeriesInstanceUidInstancesGet200ResponseInner as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into StudiesStudyInstanceUidSeriesSeriesInstanceUidInstancesGet200ResponseInner - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

