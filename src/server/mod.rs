use futures::{future, future::BoxFuture, Stream, stream, future::FutureExt, stream::TryStreamExt};
use hyper::{Request, Response, StatusCode, Body, HeaderMap};
use hyper::header::{HeaderName, HeaderValue, CONTENT_TYPE};
use log::warn;
#[allow(unused_imports)]
use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::future::Future;
use std::marker::PhantomData;
use std::task::{Context, Poll};
use swagger::{ApiError, BodyExt, Has, RequestParser, XSpanIdString};
pub use swagger::auth::Authorization;
use swagger::auth::Scopes;
use url::form_urlencoded;

#[allow(unused_imports)]
use crate::models;
use crate::header;

pub use crate::context;

type ServiceFuture = BoxFuture<'static, Result<Response<Body>, crate::ServiceError>>;

use crate::{Api,
     ApiInstancesTagResponse,
     InstancesGetResponse,
     InstancesSOPInstanceUIDFramesUidGetResponse,
     InstancesSOPInstanceUIDGetResponse,
     InstancesSOPInstanceUIDRenderedGetResponse,
     InstancesSOPInstanceUIDThumbnailGetResponse,
     SeriesGetResponse,
     SeriesSeriesInstanceUIDGetResponse,
     SeriesSeriesInstanceUIDInstancesGetResponse,
     SeriesSeriesInstanceUIDInstancesSOPInstanceUIDFramesUidGetResponse,
     SeriesSeriesInstanceUIDInstancesSOPInstanceUIDGetResponse,
     SeriesSeriesInstanceUIDInstancesSOPInstanceUIDRenderedGetResponse,
     SeriesSeriesInstanceUIDInstancesSOPInstanceUIDThumbnailGetResponse,
     SeriesSeriesInstanceUIDRenderedGetResponse,
     SeriesSeriesInstanceUIDThumbnailGetResponse,
     StudiesGetResponse,
     StudiesStudyInstanceUIDGetResponse,
     StudiesStudyInstanceUIDSeriesGetResponse,
     StudiesStudyInstanceUIDSeriesSeriesInstanceUIDGetResponse,
     StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesGetResponse,
     StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesSOPInstanceUIDFramesUidGetResponse,
     StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesSOPInstanceUIDGetResponse,
     StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesSOPInstanceUIDRenderedGetResponse,
     StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesSOPInstanceUIDThumbnailGetResponse,
     StudiesStudyInstanceUIDSeriesSeriesInstanceUIDRenderedGetResponse,
     StudiesStudyInstanceUIDSeriesSeriesInstanceUIDThumbnailGetResponse,
     StudiesStudyInstanceUIDThumbnailGetResponse
};

mod paths {
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref GLOBAL_REGEX_SET: regex::RegexSet = regex::RegexSet::new(vec![
            r"^/instances$",
            r"^/instances/(?P<SOPInstanceUID>[^/?#]*)$",
            r"^/instances/(?P<SOPInstanceUID>[^/?#]*)/frames/(?P<uid>[^/?#]*)$",
            r"^/instances/(?P<SOPInstanceUID>[^/?#]*)/rendered$",
            r"^/instances/(?P<SOPInstanceUID>[^/?#]*)/thumbnail$",
            r"^/instances/(?P<SOPInstanceUID>[^/?#]*)/(?P<tag>[^/?#]*)$",
            r"^/series$",
            r"^/series/(?P<SeriesInstanceUID>[^/?#]*)$",
            r"^/series/(?P<SeriesInstanceUID>[^/?#]*)/instances$",
            r"^/series/(?P<SeriesInstanceUID>[^/?#]*)/instances/(?P<SOPInstanceUID>[^/?#]*)$",
            r"^/series/(?P<SeriesInstanceUID>[^/?#]*)/instances/(?P<SOPInstanceUID>[^/?#]*)/frames/(?P<uid>[^/?#]*)$",
            r"^/series/(?P<SeriesInstanceUID>[^/?#]*)/instances/(?P<SOPInstanceUID>[^/?#]*)/rendered$",
            r"^/series/(?P<SeriesInstanceUID>[^/?#]*)/instances/(?P<SOPInstanceUID>[^/?#]*)/thumbnail$",
            r"^/series/(?P<SeriesInstanceUID>[^/?#]*)/rendered$",
            r"^/series/(?P<SeriesInstanceUID>[^/?#]*)/thumbnail$",
            r"^/studies$",
            r"^/studies/(?P<StudyInstanceUID>[^/?#]*)$",
            r"^/studies/(?P<StudyInstanceUID>[^/?#]*)/series$",
            r"^/studies/(?P<StudyInstanceUID>[^/?#]*)/series/(?P<SeriesInstanceUID>[^/?#]*)$",
            r"^/studies/(?P<StudyInstanceUID>[^/?#]*)/series/(?P<SeriesInstanceUID>[^/?#]*)/instances$",
            r"^/studies/(?P<StudyInstanceUID>[^/?#]*)/series/(?P<SeriesInstanceUID>[^/?#]*)/instances/(?P<SOPInstanceUID>[^/?#]*)$",
            r"^/studies/(?P<StudyInstanceUID>[^/?#]*)/series/(?P<SeriesInstanceUID>[^/?#]*)/instances/(?P<SOPInstanceUID>[^/?#]*)/frames/(?P<uid>[^/?#]*)$",
            r"^/studies/(?P<StudyInstanceUID>[^/?#]*)/series/(?P<SeriesInstanceUID>[^/?#]*)/instances/(?P<SOPInstanceUID>[^/?#]*)/rendered$",
            r"^/studies/(?P<StudyInstanceUID>[^/?#]*)/series/(?P<SeriesInstanceUID>[^/?#]*)/instances/(?P<SOPInstanceUID>[^/?#]*)/thumbnail$",
            r"^/studies/(?P<StudyInstanceUID>[^/?#]*)/series/(?P<SeriesInstanceUID>[^/?#]*)/rendered$",
            r"^/studies/(?P<StudyInstanceUID>[^/?#]*)/series/(?P<SeriesInstanceUID>[^/?#]*)/thumbnail$",
            r"^/studies/(?P<StudyInstanceUID>[^/?#]*)/thumbnail$"
        ])
        .expect("Unable to create global regex set");
    }
    pub(crate) static ID_INSTANCES: usize = 0;
    pub(crate) static ID_INSTANCES_SOPINSTANCEUID: usize = 1;
    lazy_static! {
        pub static ref REGEX_INSTANCES_SOPINSTANCEUID: regex::Regex =
            regex::Regex::new(r"^/instances/(?P<SOPInstanceUID>[^/?#]*)$")
                .expect("Unable to create regex for INSTANCES_SOPINSTANCEUID");
    }
    pub(crate) static ID_INSTANCES_SOPINSTANCEUID_FRAMES_UID: usize = 2;
    lazy_static! {
        pub static ref REGEX_INSTANCES_SOPINSTANCEUID_FRAMES_UID: regex::Regex =
            regex::Regex::new(r"^/instances/(?P<SOPInstanceUID>[^/?#]*)/frames/(?P<uid>[^/?#]*)$")
                .expect("Unable to create regex for INSTANCES_SOPINSTANCEUID_FRAMES_UID");
    }
    pub(crate) static ID_INSTANCES_SOPINSTANCEUID_RENDERED: usize = 3;
    lazy_static! {
        pub static ref REGEX_INSTANCES_SOPINSTANCEUID_RENDERED: regex::Regex =
            regex::Regex::new(r"^/instances/(?P<SOPInstanceUID>[^/?#]*)/rendered$")
                .expect("Unable to create regex for INSTANCES_SOPINSTANCEUID_RENDERED");
    }
    pub(crate) static ID_INSTANCES_SOPINSTANCEUID_THUMBNAIL: usize = 4;
    lazy_static! {
        pub static ref REGEX_INSTANCES_SOPINSTANCEUID_THUMBNAIL: regex::Regex =
            regex::Regex::new(r"^/instances/(?P<SOPInstanceUID>[^/?#]*)/thumbnail$")
                .expect("Unable to create regex for INSTANCES_SOPINSTANCEUID_THUMBNAIL");
    }
    pub(crate) static ID_INSTANCES_SOPINSTANCEUID_TAG: usize = 5;
    lazy_static! {
        pub static ref REGEX_INSTANCES_SOPINSTANCEUID_TAG: regex::Regex =
            regex::Regex::new(r"^/instances/(?P<SOPInstanceUID>[^/?#]*)/(?P<tag>[^/?#]*)$")
                .expect("Unable to create regex for INSTANCES_SOPINSTANCEUID_TAG");
    }
    pub(crate) static ID_SERIES: usize = 6;
    pub(crate) static ID_SERIES_SERIESINSTANCEUID: usize = 7;
    lazy_static! {
        pub static ref REGEX_SERIES_SERIESINSTANCEUID: regex::Regex =
            regex::Regex::new(r"^/series/(?P<SeriesInstanceUID>[^/?#]*)$")
                .expect("Unable to create regex for SERIES_SERIESINSTANCEUID");
    }
    pub(crate) static ID_SERIES_SERIESINSTANCEUID_INSTANCES: usize = 8;
    lazy_static! {
        pub static ref REGEX_SERIES_SERIESINSTANCEUID_INSTANCES: regex::Regex =
            regex::Regex::new(r"^/series/(?P<SeriesInstanceUID>[^/?#]*)/instances$")
                .expect("Unable to create regex for SERIES_SERIESINSTANCEUID_INSTANCES");
    }
    pub(crate) static ID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID: usize = 9;
    lazy_static! {
        pub static ref REGEX_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID: regex::Regex =
            regex::Regex::new(r"^/series/(?P<SeriesInstanceUID>[^/?#]*)/instances/(?P<SOPInstanceUID>[^/?#]*)$")
                .expect("Unable to create regex for SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID");
    }
    pub(crate) static ID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_FRAMES_UID: usize = 10;
    lazy_static! {
        pub static ref REGEX_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_FRAMES_UID: regex::Regex =
            regex::Regex::new(r"^/series/(?P<SeriesInstanceUID>[^/?#]*)/instances/(?P<SOPInstanceUID>[^/?#]*)/frames/(?P<uid>[^/?#]*)$")
                .expect("Unable to create regex for SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_FRAMES_UID");
    }
    pub(crate) static ID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_RENDERED: usize = 11;
    lazy_static! {
        pub static ref REGEX_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_RENDERED: regex::Regex =
            regex::Regex::new(r"^/series/(?P<SeriesInstanceUID>[^/?#]*)/instances/(?P<SOPInstanceUID>[^/?#]*)/rendered$")
                .expect("Unable to create regex for SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_RENDERED");
    }
    pub(crate) static ID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_THUMBNAIL: usize = 12;
    lazy_static! {
        pub static ref REGEX_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_THUMBNAIL: regex::Regex =
            regex::Regex::new(r"^/series/(?P<SeriesInstanceUID>[^/?#]*)/instances/(?P<SOPInstanceUID>[^/?#]*)/thumbnail$")
                .expect("Unable to create regex for SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_THUMBNAIL");
    }
    pub(crate) static ID_SERIES_SERIESINSTANCEUID_RENDERED: usize = 13;
    lazy_static! {
        pub static ref REGEX_SERIES_SERIESINSTANCEUID_RENDERED: regex::Regex =
            regex::Regex::new(r"^/series/(?P<SeriesInstanceUID>[^/?#]*)/rendered$")
                .expect("Unable to create regex for SERIES_SERIESINSTANCEUID_RENDERED");
    }
    pub(crate) static ID_SERIES_SERIESINSTANCEUID_THUMBNAIL: usize = 14;
    lazy_static! {
        pub static ref REGEX_SERIES_SERIESINSTANCEUID_THUMBNAIL: regex::Regex =
            regex::Regex::new(r"^/series/(?P<SeriesInstanceUID>[^/?#]*)/thumbnail$")
                .expect("Unable to create regex for SERIES_SERIESINSTANCEUID_THUMBNAIL");
    }
    pub(crate) static ID_STUDIES: usize = 15;
    pub(crate) static ID_STUDIES_STUDYINSTANCEUID: usize = 16;
    lazy_static! {
        pub static ref REGEX_STUDIES_STUDYINSTANCEUID: regex::Regex =
            regex::Regex::new(r"^/studies/(?P<StudyInstanceUID>[^/?#]*)$")
                .expect("Unable to create regex for STUDIES_STUDYINSTANCEUID");
    }
    pub(crate) static ID_STUDIES_STUDYINSTANCEUID_SERIES: usize = 17;
    lazy_static! {
        pub static ref REGEX_STUDIES_STUDYINSTANCEUID_SERIES: regex::Regex =
            regex::Regex::new(r"^/studies/(?P<StudyInstanceUID>[^/?#]*)/series$")
                .expect("Unable to create regex for STUDIES_STUDYINSTANCEUID_SERIES");
    }
    pub(crate) static ID_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID: usize = 18;
    lazy_static! {
        pub static ref REGEX_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID: regex::Regex =
            regex::Regex::new(r"^/studies/(?P<StudyInstanceUID>[^/?#]*)/series/(?P<SeriesInstanceUID>[^/?#]*)$")
                .expect("Unable to create regex for STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID");
    }
    pub(crate) static ID_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_INSTANCES: usize = 19;
    lazy_static! {
        pub static ref REGEX_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_INSTANCES: regex::Regex =
            regex::Regex::new(r"^/studies/(?P<StudyInstanceUID>[^/?#]*)/series/(?P<SeriesInstanceUID>[^/?#]*)/instances$")
                .expect("Unable to create regex for STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_INSTANCES");
    }
    pub(crate) static ID_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID: usize = 20;
    lazy_static! {
        pub static ref REGEX_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID: regex::Regex =
            regex::Regex::new(r"^/studies/(?P<StudyInstanceUID>[^/?#]*)/series/(?P<SeriesInstanceUID>[^/?#]*)/instances/(?P<SOPInstanceUID>[^/?#]*)$")
                .expect("Unable to create regex for STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID");
    }
    pub(crate) static ID_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_FRAMES_UID: usize = 21;
    lazy_static! {
        pub static ref REGEX_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_FRAMES_UID: regex::Regex =
            regex::Regex::new(r"^/studies/(?P<StudyInstanceUID>[^/?#]*)/series/(?P<SeriesInstanceUID>[^/?#]*)/instances/(?P<SOPInstanceUID>[^/?#]*)/frames/(?P<uid>[^/?#]*)$")
                .expect("Unable to create regex for STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_FRAMES_UID");
    }
    pub(crate) static ID_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_RENDERED: usize = 22;
    lazy_static! {
        pub static ref REGEX_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_RENDERED: regex::Regex =
            regex::Regex::new(r"^/studies/(?P<StudyInstanceUID>[^/?#]*)/series/(?P<SeriesInstanceUID>[^/?#]*)/instances/(?P<SOPInstanceUID>[^/?#]*)/rendered$")
                .expect("Unable to create regex for STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_RENDERED");
    }
    pub(crate) static ID_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_THUMBNAIL: usize = 23;
    lazy_static! {
        pub static ref REGEX_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_THUMBNAIL: regex::Regex =
            regex::Regex::new(r"^/studies/(?P<StudyInstanceUID>[^/?#]*)/series/(?P<SeriesInstanceUID>[^/?#]*)/instances/(?P<SOPInstanceUID>[^/?#]*)/thumbnail$")
                .expect("Unable to create regex for STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_THUMBNAIL");
    }
    pub(crate) static ID_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_RENDERED: usize = 24;
    lazy_static! {
        pub static ref REGEX_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_RENDERED: regex::Regex =
            regex::Regex::new(r"^/studies/(?P<StudyInstanceUID>[^/?#]*)/series/(?P<SeriesInstanceUID>[^/?#]*)/rendered$")
                .expect("Unable to create regex for STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_RENDERED");
    }
    pub(crate) static ID_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_THUMBNAIL: usize = 25;
    lazy_static! {
        pub static ref REGEX_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_THUMBNAIL: regex::Regex =
            regex::Regex::new(r"^/studies/(?P<StudyInstanceUID>[^/?#]*)/series/(?P<SeriesInstanceUID>[^/?#]*)/thumbnail$")
                .expect("Unable to create regex for STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_THUMBNAIL");
    }
    pub(crate) static ID_STUDIES_STUDYINSTANCEUID_THUMBNAIL: usize = 26;
    lazy_static! {
        pub static ref REGEX_STUDIES_STUDYINSTANCEUID_THUMBNAIL: regex::Regex =
            regex::Regex::new(r"^/studies/(?P<StudyInstanceUID>[^/?#]*)/thumbnail$")
                .expect("Unable to create regex for STUDIES_STUDYINSTANCEUID_THUMBNAIL");
    }
}

pub struct MakeService<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    api_impl: T,
    marker: PhantomData<C>,
}

impl<T, C> MakeService<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    pub fn new(api_impl: T) -> Self {
        MakeService {
            api_impl,
            marker: PhantomData
        }
    }
}

impl<T, C, Target> hyper::service::Service<Target> for MakeService<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    type Response = Service<T, C>;
    type Error = crate::ServiceError;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, target: Target) -> Self::Future {
        futures::future::ok(Service::new(
            self.api_impl.clone(),
        ))
    }
}

fn method_not_allowed() -> Result<Response<Body>, crate::ServiceError> {
    Ok(
        Response::builder().status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::empty())
            .expect("Unable to create Method Not Allowed response")
    )
}

pub struct Service<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    api_impl: T,
    marker: PhantomData<C>,
}

impl<T, C> Service<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    pub fn new(api_impl: T) -> Self {
        Service {
            api_impl: api_impl,
            marker: PhantomData
        }
    }
}

impl<T, C> Clone for Service<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    fn clone(&self) -> Self {
        Service {
            api_impl: self.api_impl.clone(),
            marker: self.marker.clone(),
        }
    }
}

impl<T, C> hyper::service::Service<(Request<Body>, C)> for Service<T, C> where
    T: Api<C> + Clone + Send + Sync + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    type Response = Response<Body>;
    type Error = crate::ServiceError;
    type Future = ServiceFuture;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.api_impl.poll_ready(cx)
    }

    fn call(&mut self, req: (Request<Body>, C)) -> Self::Future { async fn run<T, C>(mut api_impl: T, req: (Request<Body>, C)) -> Result<Response<Body>, crate::ServiceError> where
        T: Api<C> + Clone + Send + 'static,
        C: Has<XSpanIdString>  + Send + Sync + 'static
    {
        let (request, context) = req;
        let (parts, body) = request.into_parts();
        let (method, uri, headers) = (parts.method, parts.uri, parts.headers);
        let path = paths::GLOBAL_REGEX_SET.matches(uri.path());

        match &method {

            // ApiInstancesTag - GET /instances/{SOPInstanceUID}/{tag}
            &hyper::Method::GET if path.matched(paths::ID_INSTANCES_SOPINSTANCEUID_TAG) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_INSTANCES_SOPINSTANCEUID_TAG
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE INSTANCES_SOPINSTANCEUID_TAG in set but failed match against \"{}\"", path, paths::REGEX_INSTANCES_SOPINSTANCEUID_TAG.as_str())
                    );

                let param_sop_instance_uid = match percent_encoding::percent_decode(path_params["SOPInstanceUID"].as_bytes()).decode_utf8() {
                    Ok(param_sop_instance_uid) => match param_sop_instance_uid.parse::<String>() {
                        Ok(param_sop_instance_uid) => param_sop_instance_uid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter SOPInstanceUID: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["SOPInstanceUID"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_tag = match percent_encoding::percent_decode(path_params["tag"].as_bytes()).decode_utf8() {
                    Ok(param_tag) => match param_tag.parse::<String>() {
                        Ok(param_tag) => param_tag,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter tag: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["tag"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.api_instances_tag(
                                            param_sop_instance_uid,
                                            param_tag,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ApiInstancesTagResponse::AnInstanceField
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for API_INSTANCES_TAG_AN_INSTANCE_FIELD"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // InstancesGet - GET /instances
            &hyper::Method::GET if path.matched(paths::ID_INSTANCES) => {
                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_limit = query_params.iter().filter(|e| e.0 == "limit").map(|e| e.1.to_owned())
                    .nth(0);
                let param_limit = match param_limit {
                    Some(param_limit) => {
                        let param_limit =
                            <f64 as std::str::FromStr>::from_str
                                (&param_limit);
                        match param_limit {
                            Ok(param_limit) => Some(param_limit),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter limit - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter limit")),
                        }
                    },
                    None => None,
                };
                let param_offset = query_params.iter().filter(|e| e.0 == "offset").map(|e| e.1.to_owned())
                    .nth(0);
                let param_offset = match param_offset {
                    Some(param_offset) => {
                        let param_offset =
                            <f64 as std::str::FromStr>::from_str
                                (&param_offset);
                        match param_offset {
                            Ok(param_offset) => Some(param_offset),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter offset - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter offset")),
                        }
                    },
                    None => None,
                };
                let param_fuzzymatching = query_params.iter().filter(|e| e.0 == "fuzzymatching").map(|e| e.1.to_owned())
                    .nth(0);
                let param_fuzzymatching = match param_fuzzymatching {
                    Some(param_fuzzymatching) => {
                        let param_fuzzymatching =
                            <bool as std::str::FromStr>::from_str
                                (&param_fuzzymatching);
                        match param_fuzzymatching {
                            Ok(param_fuzzymatching) => Some(param_fuzzymatching),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter fuzzymatching - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter fuzzymatching")),
                        }
                    },
                    None => None,
                };
                let param_includefield = query_params.iter().filter(|e| e.0 == "includefield").map(|e| e.1.to_owned())
                    .nth(0);
                let param_includefield = match param_includefield {
                    Some(param_includefield) => {
                        let param_includefield =
                            <String as std::str::FromStr>::from_str
                                (&param_includefield);
                        match param_includefield {
                            Ok(param_includefield) => Some(param_includefield),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter includefield - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter includefield")),
                        }
                    },
                    None => None,
                };

                                let result = api_impl.instances_get(
                                            param_limit,
                                            param_offset,
                                            param_fuzzymatching,
                                            param_includefield,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                InstancesGetResponse::AJSONArrayOfInstances
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for INSTANCES_GET_AJSON_ARRAY_OF_INSTANCES"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // InstancesSOPInstanceUIDFramesUidGet - GET /instances/{SOPInstanceUID}/frames/{uid}
            &hyper::Method::GET if path.matched(paths::ID_INSTANCES_SOPINSTANCEUID_FRAMES_UID) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_INSTANCES_SOPINSTANCEUID_FRAMES_UID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE INSTANCES_SOPINSTANCEUID_FRAMES_UID in set but failed match against \"{}\"", path, paths::REGEX_INSTANCES_SOPINSTANCEUID_FRAMES_UID.as_str())
                    );

                let param_sop_instance_uid = match percent_encoding::percent_decode(path_params["SOPInstanceUID"].as_bytes()).decode_utf8() {
                    Ok(param_sop_instance_uid) => match param_sop_instance_uid.parse::<String>() {
                        Ok(param_sop_instance_uid) => param_sop_instance_uid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter SOPInstanceUID: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["SOPInstanceUID"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_uid = match percent_encoding::percent_decode(path_params["uid"].as_bytes()).decode_utf8() {
                    Ok(param_uid) => match param_uid.parse::<String>() {
                        Ok(param_uid) => param_uid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter uid: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["uid"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.instances_sop_instance_uid_frames_uid_get(
                                            param_sop_instance_uid,
                                            param_uid,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                InstancesSOPInstanceUIDFramesUidGetResponse::ASeries
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for INSTANCES_SOP_INSTANCE_UID_FRAMES_UID_GET_A_SERIES"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // InstancesSOPInstanceUIDGet - GET /instances/{SOPInstanceUID}
            &hyper::Method::GET if path.matched(paths::ID_INSTANCES_SOPINSTANCEUID) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_INSTANCES_SOPINSTANCEUID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE INSTANCES_SOPINSTANCEUID in set but failed match against \"{}\"", path, paths::REGEX_INSTANCES_SOPINSTANCEUID.as_str())
                    );

                let param_sop_instance_uid = match percent_encoding::percent_decode(path_params["SOPInstanceUID"].as_bytes()).decode_utf8() {
                    Ok(param_sop_instance_uid) => match param_sop_instance_uid.parse::<String>() {
                        Ok(param_sop_instance_uid) => param_sop_instance_uid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter SOPInstanceUID: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["SOPInstanceUID"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.instances_sop_instance_uid_get(
                                            param_sop_instance_uid,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                InstancesSOPInstanceUIDGetResponse::AFullInstance
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for INSTANCES_SOP_INSTANCE_UID_GET_A_FULL_INSTANCE"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // InstancesSOPInstanceUIDRenderedGet - GET /instances/{SOPInstanceUID}/rendered
            &hyper::Method::GET if path.matched(paths::ID_INSTANCES_SOPINSTANCEUID_RENDERED) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_INSTANCES_SOPINSTANCEUID_RENDERED
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE INSTANCES_SOPINSTANCEUID_RENDERED in set but failed match against \"{}\"", path, paths::REGEX_INSTANCES_SOPINSTANCEUID_RENDERED.as_str())
                    );

                let param_sop_instance_uid = match percent_encoding::percent_decode(path_params["SOPInstanceUID"].as_bytes()).decode_utf8() {
                    Ok(param_sop_instance_uid) => match param_sop_instance_uid.parse::<String>() {
                        Ok(param_sop_instance_uid) => param_sop_instance_uid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter SOPInstanceUID: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["SOPInstanceUID"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_annotation = query_params.iter().filter(|e| e.0 == "annotation").map(|e| e.1.to_owned())
                    .filter_map(|param_annotation| param_annotation.parse().ok())
                    .collect::<Vec<_>>();
                let param_annotation = if !param_annotation.is_empty() {
                    Some(param_annotation)
                } else {
                    None
                };
                let param_quality = query_params.iter().filter(|e| e.0 == "quality").map(|e| e.1.to_owned())
                    .nth(0);
                let param_quality = match param_quality {
                    Some(param_quality) => {
                        let param_quality =
                            <f32 as std::str::FromStr>::from_str
                                (&param_quality);
                        match param_quality {
                            Ok(param_quality) => Some(param_quality),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter quality - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter quality")),
                        }
                    },
                    None => None,
                };
                let param_viewport = query_params.iter().filter(|e| e.0 == "viewport").map(|e| e.1.to_owned())
                    .filter_map(|param_viewport| param_viewport.parse().ok())
                    .collect::<Vec<_>>();
                let param_viewport = if !param_viewport.is_empty() {
                    Some(param_viewport)
                } else {
                    None
                };
                let param_window = query_params.iter().filter(|e| e.0 == "window").map(|e| e.1.to_owned())
                    .filter_map(|param_window| param_window.parse().ok())
                    .collect::<Vec<_>>();
                let param_window = if !param_window.is_empty() {
                    Some(param_window)
                } else {
                    None
                };

                                let result = api_impl.instances_sop_instance_uid_rendered_get(
                                            param_sop_instance_uid,
                                            param_annotation.as_ref(),
                                            param_quality,
                                            param_viewport.as_ref(),
                                            param_window.as_ref(),
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                InstancesSOPInstanceUIDRenderedGetResponse::TheRenderedImageCorrespondingToTheRequestedInstance
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("image/*")
                                                            .expect("Unable to create Content-Type header for INSTANCES_SOP_INSTANCE_UID_RENDERED_GET_THE_RENDERED_IMAGE_CORRESPONDING_TO_THE_REQUESTED_INSTANCE"));
                                                    let body = body.0;
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // InstancesSOPInstanceUIDThumbnailGet - GET /instances/{SOPInstanceUID}/thumbnail
            &hyper::Method::GET if path.matched(paths::ID_INSTANCES_SOPINSTANCEUID_THUMBNAIL) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_INSTANCES_SOPINSTANCEUID_THUMBNAIL
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE INSTANCES_SOPINSTANCEUID_THUMBNAIL in set but failed match against \"{}\"", path, paths::REGEX_INSTANCES_SOPINSTANCEUID_THUMBNAIL.as_str())
                    );

                let param_sop_instance_uid = match percent_encoding::percent_decode(path_params["SOPInstanceUID"].as_bytes()).decode_utf8() {
                    Ok(param_sop_instance_uid) => match param_sop_instance_uid.parse::<String>() {
                        Ok(param_sop_instance_uid) => param_sop_instance_uid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter SOPInstanceUID: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["SOPInstanceUID"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_quality = query_params.iter().filter(|e| e.0 == "quality").map(|e| e.1.to_owned())
                    .nth(0);
                let param_quality = match param_quality {
                    Some(param_quality) => {
                        let param_quality =
                            <f32 as std::str::FromStr>::from_str
                                (&param_quality);
                        match param_quality {
                            Ok(param_quality) => Some(param_quality),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter quality - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter quality")),
                        }
                    },
                    None => None,
                };
                let param_viewport = query_params.iter().filter(|e| e.0 == "viewport").map(|e| e.1.to_owned())
                    .filter_map(|param_viewport| param_viewport.parse().ok())
                    .collect::<Vec<_>>();
                let param_viewport = if !param_viewport.is_empty() {
                    Some(param_viewport)
                } else {
                    None
                };

                                let result = api_impl.instances_sop_instance_uid_thumbnail_get(
                                            param_sop_instance_uid,
                                            param_quality,
                                            param_viewport.as_ref(),
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                InstancesSOPInstanceUIDThumbnailGetResponse::ARepresentativeImageOfAnInstance
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("image/gif")
                                                            .expect("Unable to create Content-Type header for INSTANCES_SOP_INSTANCE_UID_THUMBNAIL_GET_A_REPRESENTATIVE_IMAGE_OF_AN_INSTANCE"));
                                                    let body = body.0;
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // SeriesGet - GET /series
            &hyper::Method::GET if path.matched(paths::ID_SERIES) => {
                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_limit = query_params.iter().filter(|e| e.0 == "limit").map(|e| e.1.to_owned())
                    .nth(0);
                let param_limit = match param_limit {
                    Some(param_limit) => {
                        let param_limit =
                            <f64 as std::str::FromStr>::from_str
                                (&param_limit);
                        match param_limit {
                            Ok(param_limit) => Some(param_limit),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter limit - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter limit")),
                        }
                    },
                    None => None,
                };
                let param_offset = query_params.iter().filter(|e| e.0 == "offset").map(|e| e.1.to_owned())
                    .nth(0);
                let param_offset = match param_offset {
                    Some(param_offset) => {
                        let param_offset =
                            <f64 as std::str::FromStr>::from_str
                                (&param_offset);
                        match param_offset {
                            Ok(param_offset) => Some(param_offset),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter offset - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter offset")),
                        }
                    },
                    None => None,
                };
                let param_fuzzymatching = query_params.iter().filter(|e| e.0 == "fuzzymatching").map(|e| e.1.to_owned())
                    .nth(0);
                let param_fuzzymatching = match param_fuzzymatching {
                    Some(param_fuzzymatching) => {
                        let param_fuzzymatching =
                            <bool as std::str::FromStr>::from_str
                                (&param_fuzzymatching);
                        match param_fuzzymatching {
                            Ok(param_fuzzymatching) => Some(param_fuzzymatching),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter fuzzymatching - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter fuzzymatching")),
                        }
                    },
                    None => None,
                };
                let param_includefield = query_params.iter().filter(|e| e.0 == "includefield").map(|e| e.1.to_owned())
                    .nth(0);
                let param_includefield = match param_includefield {
                    Some(param_includefield) => {
                        let param_includefield =
                            <String as std::str::FromStr>::from_str
                                (&param_includefield);
                        match param_includefield {
                            Ok(param_includefield) => Some(param_includefield),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter includefield - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter includefield")),
                        }
                    },
                    None => None,
                };

                                let result = api_impl.series_get(
                                            param_limit,
                                            param_offset,
                                            param_fuzzymatching,
                                            param_includefield,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                SeriesGetResponse::AJSONArrayOfSeries
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SERIES_GET_AJSON_ARRAY_OF_SERIES"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // SeriesSeriesInstanceUIDGet - GET /series/{SeriesInstanceUID}
            &hyper::Method::GET if path.matched(paths::ID_SERIES_SERIESINSTANCEUID) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_SERIES_SERIESINSTANCEUID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE SERIES_SERIESINSTANCEUID in set but failed match against \"{}\"", path, paths::REGEX_SERIES_SERIESINSTANCEUID.as_str())
                    );

                let param_series_instance_uid = match percent_encoding::percent_decode(path_params["SeriesInstanceUID"].as_bytes()).decode_utf8() {
                    Ok(param_series_instance_uid) => match param_series_instance_uid.parse::<String>() {
                        Ok(param_series_instance_uid) => param_series_instance_uid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter SeriesInstanceUID: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["SeriesInstanceUID"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.series_series_instance_uid_get(
                                            param_series_instance_uid,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                SeriesSeriesInstanceUIDGetResponse::AFullSeries
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SERIES_SERIES_INSTANCE_UID_GET_A_FULL_SERIES"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // SeriesSeriesInstanceUIDInstancesGet - GET /series/{SeriesInstanceUID}/instances
            &hyper::Method::GET if path.matched(paths::ID_SERIES_SERIESINSTANCEUID_INSTANCES) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_SERIES_SERIESINSTANCEUID_INSTANCES
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE SERIES_SERIESINSTANCEUID_INSTANCES in set but failed match against \"{}\"", path, paths::REGEX_SERIES_SERIESINSTANCEUID_INSTANCES.as_str())
                    );

                let param_series_instance_uid = match percent_encoding::percent_decode(path_params["SeriesInstanceUID"].as_bytes()).decode_utf8() {
                    Ok(param_series_instance_uid) => match param_series_instance_uid.parse::<String>() {
                        Ok(param_series_instance_uid) => param_series_instance_uid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter SeriesInstanceUID: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["SeriesInstanceUID"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_limit = query_params.iter().filter(|e| e.0 == "limit").map(|e| e.1.to_owned())
                    .nth(0);
                let param_limit = match param_limit {
                    Some(param_limit) => {
                        let param_limit =
                            <f64 as std::str::FromStr>::from_str
                                (&param_limit);
                        match param_limit {
                            Ok(param_limit) => Some(param_limit),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter limit - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter limit")),
                        }
                    },
                    None => None,
                };
                let param_offset = query_params.iter().filter(|e| e.0 == "offset").map(|e| e.1.to_owned())
                    .nth(0);
                let param_offset = match param_offset {
                    Some(param_offset) => {
                        let param_offset =
                            <f64 as std::str::FromStr>::from_str
                                (&param_offset);
                        match param_offset {
                            Ok(param_offset) => Some(param_offset),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter offset - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter offset")),
                        }
                    },
                    None => None,
                };
                let param_fuzzymatching = query_params.iter().filter(|e| e.0 == "fuzzymatching").map(|e| e.1.to_owned())
                    .nth(0);
                let param_fuzzymatching = match param_fuzzymatching {
                    Some(param_fuzzymatching) => {
                        let param_fuzzymatching =
                            <bool as std::str::FromStr>::from_str
                                (&param_fuzzymatching);
                        match param_fuzzymatching {
                            Ok(param_fuzzymatching) => Some(param_fuzzymatching),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter fuzzymatching - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter fuzzymatching")),
                        }
                    },
                    None => None,
                };
                let param_includefield = query_params.iter().filter(|e| e.0 == "includefield").map(|e| e.1.to_owned())
                    .nth(0);
                let param_includefield = match param_includefield {
                    Some(param_includefield) => {
                        let param_includefield =
                            <String as std::str::FromStr>::from_str
                                (&param_includefield);
                        match param_includefield {
                            Ok(param_includefield) => Some(param_includefield),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter includefield - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter includefield")),
                        }
                    },
                    None => None,
                };

                                let result = api_impl.series_series_instance_uid_instances_get(
                                            param_series_instance_uid,
                                            param_limit,
                                            param_offset,
                                            param_fuzzymatching,
                                            param_includefield,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                SeriesSeriesInstanceUIDInstancesGetResponse::AJSONArrayOfInstances
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SERIES_SERIES_INSTANCE_UID_INSTANCES_GET_AJSON_ARRAY_OF_INSTANCES"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // SeriesSeriesInstanceUIDInstancesSOPInstanceUIDFramesUidGet - GET /series/{SeriesInstanceUID}/instances/{SOPInstanceUID}/frames/{uid}
            &hyper::Method::GET if path.matched(paths::ID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_FRAMES_UID) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_FRAMES_UID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_FRAMES_UID in set but failed match against \"{}\"", path, paths::REGEX_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_FRAMES_UID.as_str())
                    );

                let param_series_instance_uid = match percent_encoding::percent_decode(path_params["SeriesInstanceUID"].as_bytes()).decode_utf8() {
                    Ok(param_series_instance_uid) => match param_series_instance_uid.parse::<String>() {
                        Ok(param_series_instance_uid) => param_series_instance_uid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter SeriesInstanceUID: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["SeriesInstanceUID"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_sop_instance_uid = match percent_encoding::percent_decode(path_params["SOPInstanceUID"].as_bytes()).decode_utf8() {
                    Ok(param_sop_instance_uid) => match param_sop_instance_uid.parse::<String>() {
                        Ok(param_sop_instance_uid) => param_sop_instance_uid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter SOPInstanceUID: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["SOPInstanceUID"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_uid = match percent_encoding::percent_decode(path_params["uid"].as_bytes()).decode_utf8() {
                    Ok(param_uid) => match param_uid.parse::<String>() {
                        Ok(param_uid) => param_uid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter uid: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["uid"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.series_series_instance_uid_instances_sop_instance_uid_frames_uid_get(
                                            param_series_instance_uid,
                                            param_sop_instance_uid,
                                            param_uid,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                SeriesSeriesInstanceUIDInstancesSOPInstanceUIDFramesUidGetResponse::ASeries
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SERIES_SERIES_INSTANCE_UID_INSTANCES_SOP_INSTANCE_UID_FRAMES_UID_GET_A_SERIES"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // SeriesSeriesInstanceUIDInstancesSOPInstanceUIDGet - GET /series/{SeriesInstanceUID}/instances/{SOPInstanceUID}
            &hyper::Method::GET if path.matched(paths::ID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID in set but failed match against \"{}\"", path, paths::REGEX_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID.as_str())
                    );

                let param_series_instance_uid = match percent_encoding::percent_decode(path_params["SeriesInstanceUID"].as_bytes()).decode_utf8() {
                    Ok(param_series_instance_uid) => match param_series_instance_uid.parse::<String>() {
                        Ok(param_series_instance_uid) => param_series_instance_uid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter SeriesInstanceUID: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["SeriesInstanceUID"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_sop_instance_uid = match percent_encoding::percent_decode(path_params["SOPInstanceUID"].as_bytes()).decode_utf8() {
                    Ok(param_sop_instance_uid) => match param_sop_instance_uid.parse::<String>() {
                        Ok(param_sop_instance_uid) => param_sop_instance_uid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter SOPInstanceUID: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["SOPInstanceUID"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.series_series_instance_uid_instances_sop_instance_uid_get(
                                            param_series_instance_uid,
                                            param_sop_instance_uid,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                SeriesSeriesInstanceUIDInstancesSOPInstanceUIDGetResponse::ASeries
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SERIES_SERIES_INSTANCE_UID_INSTANCES_SOP_INSTANCE_UID_GET_A_SERIES"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // SeriesSeriesInstanceUIDInstancesSOPInstanceUIDRenderedGet - GET /series/{SeriesInstanceUID}/instances/{SOPInstanceUID}/rendered
            &hyper::Method::GET if path.matched(paths::ID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_RENDERED) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_RENDERED
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_RENDERED in set but failed match against \"{}\"", path, paths::REGEX_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_RENDERED.as_str())
                    );

                let param_series_instance_uid = match percent_encoding::percent_decode(path_params["SeriesInstanceUID"].as_bytes()).decode_utf8() {
                    Ok(param_series_instance_uid) => match param_series_instance_uid.parse::<String>() {
                        Ok(param_series_instance_uid) => param_series_instance_uid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter SeriesInstanceUID: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["SeriesInstanceUID"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_sop_instance_uid = match percent_encoding::percent_decode(path_params["SOPInstanceUID"].as_bytes()).decode_utf8() {
                    Ok(param_sop_instance_uid) => match param_sop_instance_uid.parse::<String>() {
                        Ok(param_sop_instance_uid) => param_sop_instance_uid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter SOPInstanceUID: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["SOPInstanceUID"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_annotation = query_params.iter().filter(|e| e.0 == "annotation").map(|e| e.1.to_owned())
                    .filter_map(|param_annotation| param_annotation.parse().ok())
                    .collect::<Vec<_>>();
                let param_annotation = if !param_annotation.is_empty() {
                    Some(param_annotation)
                } else {
                    None
                };
                let param_quality = query_params.iter().filter(|e| e.0 == "quality").map(|e| e.1.to_owned())
                    .nth(0);
                let param_quality = match param_quality {
                    Some(param_quality) => {
                        let param_quality =
                            <f32 as std::str::FromStr>::from_str
                                (&param_quality);
                        match param_quality {
                            Ok(param_quality) => Some(param_quality),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter quality - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter quality")),
                        }
                    },
                    None => None,
                };
                let param_viewport = query_params.iter().filter(|e| e.0 == "viewport").map(|e| e.1.to_owned())
                    .filter_map(|param_viewport| param_viewport.parse().ok())
                    .collect::<Vec<_>>();
                let param_viewport = if !param_viewport.is_empty() {
                    Some(param_viewport)
                } else {
                    None
                };
                let param_window = query_params.iter().filter(|e| e.0 == "window").map(|e| e.1.to_owned())
                    .filter_map(|param_window| param_window.parse().ok())
                    .collect::<Vec<_>>();
                let param_window = if !param_window.is_empty() {
                    Some(param_window)
                } else {
                    None
                };

                                let result = api_impl.series_series_instance_uid_instances_sop_instance_uid_rendered_get(
                                            param_series_instance_uid,
                                            param_sop_instance_uid,
                                            param_annotation.as_ref(),
                                            param_quality,
                                            param_viewport.as_ref(),
                                            param_window.as_ref(),
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                SeriesSeriesInstanceUIDInstancesSOPInstanceUIDRenderedGetResponse::TheRenderedImageCorrespondingToTheRequestedInstance
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("image/*")
                                                            .expect("Unable to create Content-Type header for SERIES_SERIES_INSTANCE_UID_INSTANCES_SOP_INSTANCE_UID_RENDERED_GET_THE_RENDERED_IMAGE_CORRESPONDING_TO_THE_REQUESTED_INSTANCE"));
                                                    let body = body.0;
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // SeriesSeriesInstanceUIDInstancesSOPInstanceUIDThumbnailGet - GET /series/{SeriesInstanceUID}/instances/{SOPInstanceUID}/thumbnail
            &hyper::Method::GET if path.matched(paths::ID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_THUMBNAIL) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_THUMBNAIL
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_THUMBNAIL in set but failed match against \"{}\"", path, paths::REGEX_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_THUMBNAIL.as_str())
                    );

                let param_series_instance_uid = match percent_encoding::percent_decode(path_params["SeriesInstanceUID"].as_bytes()).decode_utf8() {
                    Ok(param_series_instance_uid) => match param_series_instance_uid.parse::<String>() {
                        Ok(param_series_instance_uid) => param_series_instance_uid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter SeriesInstanceUID: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["SeriesInstanceUID"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_sop_instance_uid = match percent_encoding::percent_decode(path_params["SOPInstanceUID"].as_bytes()).decode_utf8() {
                    Ok(param_sop_instance_uid) => match param_sop_instance_uid.parse::<String>() {
                        Ok(param_sop_instance_uid) => param_sop_instance_uid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter SOPInstanceUID: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["SOPInstanceUID"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_quality = query_params.iter().filter(|e| e.0 == "quality").map(|e| e.1.to_owned())
                    .nth(0);
                let param_quality = match param_quality {
                    Some(param_quality) => {
                        let param_quality =
                            <f32 as std::str::FromStr>::from_str
                                (&param_quality);
                        match param_quality {
                            Ok(param_quality) => Some(param_quality),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter quality - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter quality")),
                        }
                    },
                    None => None,
                };
                let param_viewport = query_params.iter().filter(|e| e.0 == "viewport").map(|e| e.1.to_owned())
                    .filter_map(|param_viewport| param_viewport.parse().ok())
                    .collect::<Vec<_>>();
                let param_viewport = if !param_viewport.is_empty() {
                    Some(param_viewport)
                } else {
                    None
                };

                                let result = api_impl.series_series_instance_uid_instances_sop_instance_uid_thumbnail_get(
                                            param_series_instance_uid,
                                            param_sop_instance_uid,
                                            param_quality,
                                            param_viewport.as_ref(),
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                SeriesSeriesInstanceUIDInstancesSOPInstanceUIDThumbnailGetResponse::ARepresentativeImageOfAnInstance
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("image/gif")
                                                            .expect("Unable to create Content-Type header for SERIES_SERIES_INSTANCE_UID_INSTANCES_SOP_INSTANCE_UID_THUMBNAIL_GET_A_REPRESENTATIVE_IMAGE_OF_AN_INSTANCE"));
                                                    let body = body.0;
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // SeriesSeriesInstanceUIDRenderedGet - GET /series/{SeriesInstanceUID}/rendered
            &hyper::Method::GET if path.matched(paths::ID_SERIES_SERIESINSTANCEUID_RENDERED) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_SERIES_SERIESINSTANCEUID_RENDERED
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE SERIES_SERIESINSTANCEUID_RENDERED in set but failed match against \"{}\"", path, paths::REGEX_SERIES_SERIESINSTANCEUID_RENDERED.as_str())
                    );

                let param_series_instance_uid = match percent_encoding::percent_decode(path_params["SeriesInstanceUID"].as_bytes()).decode_utf8() {
                    Ok(param_series_instance_uid) => match param_series_instance_uid.parse::<String>() {
                        Ok(param_series_instance_uid) => param_series_instance_uid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter SeriesInstanceUID: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["SeriesInstanceUID"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_annotation = query_params.iter().filter(|e| e.0 == "annotation").map(|e| e.1.to_owned())
                    .filter_map(|param_annotation| param_annotation.parse().ok())
                    .collect::<Vec<_>>();
                let param_annotation = if !param_annotation.is_empty() {
                    Some(param_annotation)
                } else {
                    None
                };
                let param_quality = query_params.iter().filter(|e| e.0 == "quality").map(|e| e.1.to_owned())
                    .nth(0);
                let param_quality = match param_quality {
                    Some(param_quality) => {
                        let param_quality =
                            <f32 as std::str::FromStr>::from_str
                                (&param_quality);
                        match param_quality {
                            Ok(param_quality) => Some(param_quality),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter quality - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter quality")),
                        }
                    },
                    None => None,
                };
                let param_viewport = query_params.iter().filter(|e| e.0 == "viewport").map(|e| e.1.to_owned())
                    .filter_map(|param_viewport| param_viewport.parse().ok())
                    .collect::<Vec<_>>();
                let param_viewport = if !param_viewport.is_empty() {
                    Some(param_viewport)
                } else {
                    None
                };
                let param_window = query_params.iter().filter(|e| e.0 == "window").map(|e| e.1.to_owned())
                    .filter_map(|param_window| param_window.parse().ok())
                    .collect::<Vec<_>>();
                let param_window = if !param_window.is_empty() {
                    Some(param_window)
                } else {
                    None
                };

                                let result = api_impl.series_series_instance_uid_rendered_get(
                                            param_series_instance_uid,
                                            param_annotation.as_ref(),
                                            param_quality,
                                            param_viewport.as_ref(),
                                            param_window.as_ref(),
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                SeriesSeriesInstanceUIDRenderedGetResponse::TheRenderedImagesCorrespondingToTheRequestedSeries
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("image/gif")
                                                            .expect("Unable to create Content-Type header for SERIES_SERIES_INSTANCE_UID_RENDERED_GET_THE_RENDERED_IMAGES_CORRESPONDING_TO_THE_REQUESTED_SERIES"));
                                                    let body = body.0;
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // SeriesSeriesInstanceUIDThumbnailGet - GET /series/{SeriesInstanceUID}/thumbnail
            &hyper::Method::GET if path.matched(paths::ID_SERIES_SERIESINSTANCEUID_THUMBNAIL) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_SERIES_SERIESINSTANCEUID_THUMBNAIL
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE SERIES_SERIESINSTANCEUID_THUMBNAIL in set but failed match against \"{}\"", path, paths::REGEX_SERIES_SERIESINSTANCEUID_THUMBNAIL.as_str())
                    );

                let param_series_instance_uid = match percent_encoding::percent_decode(path_params["SeriesInstanceUID"].as_bytes()).decode_utf8() {
                    Ok(param_series_instance_uid) => match param_series_instance_uid.parse::<String>() {
                        Ok(param_series_instance_uid) => param_series_instance_uid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter SeriesInstanceUID: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["SeriesInstanceUID"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_quality = query_params.iter().filter(|e| e.0 == "quality").map(|e| e.1.to_owned())
                    .nth(0);
                let param_quality = match param_quality {
                    Some(param_quality) => {
                        let param_quality =
                            <f32 as std::str::FromStr>::from_str
                                (&param_quality);
                        match param_quality {
                            Ok(param_quality) => Some(param_quality),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter quality - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter quality")),
                        }
                    },
                    None => None,
                };
                let param_viewport = query_params.iter().filter(|e| e.0 == "viewport").map(|e| e.1.to_owned())
                    .filter_map(|param_viewport| param_viewport.parse().ok())
                    .collect::<Vec<_>>();
                let param_viewport = if !param_viewport.is_empty() {
                    Some(param_viewport)
                } else {
                    None
                };

                                let result = api_impl.series_series_instance_uid_thumbnail_get(
                                            param_series_instance_uid,
                                            param_quality,
                                            param_viewport.as_ref(),
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                SeriesSeriesInstanceUIDThumbnailGetResponse::ARepresentativeImageOfTheSeries
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("image/gif")
                                                            .expect("Unable to create Content-Type header for SERIES_SERIES_INSTANCE_UID_THUMBNAIL_GET_A_REPRESENTATIVE_IMAGE_OF_THE_SERIES"));
                                                    let body = body.0;
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // StudiesGet - GET /studies
            &hyper::Method::GET if path.matched(paths::ID_STUDIES) => {
                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_limit = query_params.iter().filter(|e| e.0 == "limit").map(|e| e.1.to_owned())
                    .nth(0);
                let param_limit = match param_limit {
                    Some(param_limit) => {
                        let param_limit =
                            <f64 as std::str::FromStr>::from_str
                                (&param_limit);
                        match param_limit {
                            Ok(param_limit) => Some(param_limit),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter limit - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter limit")),
                        }
                    },
                    None => None,
                };
                let param_offset = query_params.iter().filter(|e| e.0 == "offset").map(|e| e.1.to_owned())
                    .nth(0);
                let param_offset = match param_offset {
                    Some(param_offset) => {
                        let param_offset =
                            <f64 as std::str::FromStr>::from_str
                                (&param_offset);
                        match param_offset {
                            Ok(param_offset) => Some(param_offset),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter offset - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter offset")),
                        }
                    },
                    None => None,
                };
                let param_fuzzymatching = query_params.iter().filter(|e| e.0 == "fuzzymatching").map(|e| e.1.to_owned())
                    .nth(0);
                let param_fuzzymatching = match param_fuzzymatching {
                    Some(param_fuzzymatching) => {
                        let param_fuzzymatching =
                            <bool as std::str::FromStr>::from_str
                                (&param_fuzzymatching);
                        match param_fuzzymatching {
                            Ok(param_fuzzymatching) => Some(param_fuzzymatching),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter fuzzymatching - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter fuzzymatching")),
                        }
                    },
                    None => None,
                };
                let param_includefield = query_params.iter().filter(|e| e.0 == "includefield").map(|e| e.1.to_owned())
                    .nth(0);
                let param_includefield = match param_includefield {
                    Some(param_includefield) => {
                        let param_includefield =
                            <String as std::str::FromStr>::from_str
                                (&param_includefield);
                        match param_includefield {
                            Ok(param_includefield) => Some(param_includefield),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter includefield - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter includefield")),
                        }
                    },
                    None => None,
                };

                                let result = api_impl.studies_get(
                                            param_limit,
                                            param_offset,
                                            param_fuzzymatching,
                                            param_includefield,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                StudiesGetResponse::AJSONArrayOfStudies
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for STUDIES_GET_AJSON_ARRAY_OF_STUDIES"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // StudiesStudyInstanceUIDGet - GET /studies/{StudyInstanceUID}
            &hyper::Method::GET if path.matched(paths::ID_STUDIES_STUDYINSTANCEUID) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_STUDIES_STUDYINSTANCEUID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE STUDIES_STUDYINSTANCEUID in set but failed match against \"{}\"", path, paths::REGEX_STUDIES_STUDYINSTANCEUID.as_str())
                    );

                let param_study_instance_uid = match percent_encoding::percent_decode(path_params["StudyInstanceUID"].as_bytes()).decode_utf8() {
                    Ok(param_study_instance_uid) => match param_study_instance_uid.parse::<String>() {
                        Ok(param_study_instance_uid) => param_study_instance_uid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter StudyInstanceUID: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["StudyInstanceUID"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.studies_study_instance_uid_get(
                                            param_study_instance_uid,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                StudiesStudyInstanceUIDGetResponse::AFullStudy
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for STUDIES_STUDY_INSTANCE_UID_GET_A_FULL_STUDY"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // StudiesStudyInstanceUIDSeriesGet - GET /studies/{StudyInstanceUID}/series
            &hyper::Method::GET if path.matched(paths::ID_STUDIES_STUDYINSTANCEUID_SERIES) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_STUDIES_STUDYINSTANCEUID_SERIES
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE STUDIES_STUDYINSTANCEUID_SERIES in set but failed match against \"{}\"", path, paths::REGEX_STUDIES_STUDYINSTANCEUID_SERIES.as_str())
                    );

                let param_study_instance_uid = match percent_encoding::percent_decode(path_params["StudyInstanceUID"].as_bytes()).decode_utf8() {
                    Ok(param_study_instance_uid) => match param_study_instance_uid.parse::<String>() {
                        Ok(param_study_instance_uid) => param_study_instance_uid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter StudyInstanceUID: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["StudyInstanceUID"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_limit = query_params.iter().filter(|e| e.0 == "limit").map(|e| e.1.to_owned())
                    .nth(0);
                let param_limit = match param_limit {
                    Some(param_limit) => {
                        let param_limit =
                            <f64 as std::str::FromStr>::from_str
                                (&param_limit);
                        match param_limit {
                            Ok(param_limit) => Some(param_limit),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter limit - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter limit")),
                        }
                    },
                    None => None,
                };
                let param_offset = query_params.iter().filter(|e| e.0 == "offset").map(|e| e.1.to_owned())
                    .nth(0);
                let param_offset = match param_offset {
                    Some(param_offset) => {
                        let param_offset =
                            <f64 as std::str::FromStr>::from_str
                                (&param_offset);
                        match param_offset {
                            Ok(param_offset) => Some(param_offset),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter offset - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter offset")),
                        }
                    },
                    None => None,
                };
                let param_fuzzymatching = query_params.iter().filter(|e| e.0 == "fuzzymatching").map(|e| e.1.to_owned())
                    .nth(0);
                let param_fuzzymatching = match param_fuzzymatching {
                    Some(param_fuzzymatching) => {
                        let param_fuzzymatching =
                            <bool as std::str::FromStr>::from_str
                                (&param_fuzzymatching);
                        match param_fuzzymatching {
                            Ok(param_fuzzymatching) => Some(param_fuzzymatching),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter fuzzymatching - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter fuzzymatching")),
                        }
                    },
                    None => None,
                };
                let param_includefield = query_params.iter().filter(|e| e.0 == "includefield").map(|e| e.1.to_owned())
                    .nth(0);
                let param_includefield = match param_includefield {
                    Some(param_includefield) => {
                        let param_includefield =
                            <String as std::str::FromStr>::from_str
                                (&param_includefield);
                        match param_includefield {
                            Ok(param_includefield) => Some(param_includefield),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter includefield - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter includefield")),
                        }
                    },
                    None => None,
                };

                                let result = api_impl.studies_study_instance_uid_series_get(
                                            param_study_instance_uid,
                                            param_limit,
                                            param_offset,
                                            param_fuzzymatching,
                                            param_includefield,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                StudiesStudyInstanceUIDSeriesGetResponse::AJSONArrayOfSeries
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for STUDIES_STUDY_INSTANCE_UID_SERIES_GET_AJSON_ARRAY_OF_SERIES"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // StudiesStudyInstanceUIDSeriesSeriesInstanceUIDGet - GET /studies/{StudyInstanceUID}/series/{SeriesInstanceUID}
            &hyper::Method::GET if path.matched(paths::ID_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID in set but failed match against \"{}\"", path, paths::REGEX_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID.as_str())
                    );

                let param_study_instance_uid = match percent_encoding::percent_decode(path_params["StudyInstanceUID"].as_bytes()).decode_utf8() {
                    Ok(param_study_instance_uid) => match param_study_instance_uid.parse::<String>() {
                        Ok(param_study_instance_uid) => param_study_instance_uid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter StudyInstanceUID: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["StudyInstanceUID"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_series_instance_uid = match percent_encoding::percent_decode(path_params["SeriesInstanceUID"].as_bytes()).decode_utf8() {
                    Ok(param_series_instance_uid) => match param_series_instance_uid.parse::<String>() {
                        Ok(param_series_instance_uid) => param_series_instance_uid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter SeriesInstanceUID: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["SeriesInstanceUID"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.studies_study_instance_uid_series_series_instance_uid_get(
                                            param_study_instance_uid,
                                            param_series_instance_uid,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                StudiesStudyInstanceUIDSeriesSeriesInstanceUIDGetResponse::ASeries
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for STUDIES_STUDY_INSTANCE_UID_SERIES_SERIES_INSTANCE_UID_GET_A_SERIES"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesGet - GET /studies/{StudyInstanceUID}/series/{SeriesInstanceUID}/instances
            &hyper::Method::GET if path.matched(paths::ID_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_INSTANCES) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_INSTANCES
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_INSTANCES in set but failed match against \"{}\"", path, paths::REGEX_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_INSTANCES.as_str())
                    );

                let param_study_instance_uid = match percent_encoding::percent_decode(path_params["StudyInstanceUID"].as_bytes()).decode_utf8() {
                    Ok(param_study_instance_uid) => match param_study_instance_uid.parse::<String>() {
                        Ok(param_study_instance_uid) => param_study_instance_uid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter StudyInstanceUID: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["StudyInstanceUID"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_series_instance_uid = match percent_encoding::percent_decode(path_params["SeriesInstanceUID"].as_bytes()).decode_utf8() {
                    Ok(param_series_instance_uid) => match param_series_instance_uid.parse::<String>() {
                        Ok(param_series_instance_uid) => param_series_instance_uid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter SeriesInstanceUID: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["SeriesInstanceUID"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_limit = query_params.iter().filter(|e| e.0 == "limit").map(|e| e.1.to_owned())
                    .nth(0);
                let param_limit = match param_limit {
                    Some(param_limit) => {
                        let param_limit =
                            <f64 as std::str::FromStr>::from_str
                                (&param_limit);
                        match param_limit {
                            Ok(param_limit) => Some(param_limit),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter limit - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter limit")),
                        }
                    },
                    None => None,
                };
                let param_offset = query_params.iter().filter(|e| e.0 == "offset").map(|e| e.1.to_owned())
                    .nth(0);
                let param_offset = match param_offset {
                    Some(param_offset) => {
                        let param_offset =
                            <f64 as std::str::FromStr>::from_str
                                (&param_offset);
                        match param_offset {
                            Ok(param_offset) => Some(param_offset),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter offset - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter offset")),
                        }
                    },
                    None => None,
                };
                let param_fuzzymatching = query_params.iter().filter(|e| e.0 == "fuzzymatching").map(|e| e.1.to_owned())
                    .nth(0);
                let param_fuzzymatching = match param_fuzzymatching {
                    Some(param_fuzzymatching) => {
                        let param_fuzzymatching =
                            <bool as std::str::FromStr>::from_str
                                (&param_fuzzymatching);
                        match param_fuzzymatching {
                            Ok(param_fuzzymatching) => Some(param_fuzzymatching),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter fuzzymatching - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter fuzzymatching")),
                        }
                    },
                    None => None,
                };
                let param_includefield = query_params.iter().filter(|e| e.0 == "includefield").map(|e| e.1.to_owned())
                    .nth(0);
                let param_includefield = match param_includefield {
                    Some(param_includefield) => {
                        let param_includefield =
                            <String as std::str::FromStr>::from_str
                                (&param_includefield);
                        match param_includefield {
                            Ok(param_includefield) => Some(param_includefield),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter includefield - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter includefield")),
                        }
                    },
                    None => None,
                };

                                let result = api_impl.studies_study_instance_uid_series_series_instance_uid_instances_get(
                                            param_study_instance_uid,
                                            param_series_instance_uid,
                                            param_limit,
                                            param_offset,
                                            param_fuzzymatching,
                                            param_includefield,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesGetResponse::AJSONArrayOfInstances
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for STUDIES_STUDY_INSTANCE_UID_SERIES_SERIES_INSTANCE_UID_INSTANCES_GET_AJSON_ARRAY_OF_INSTANCES"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesSOPInstanceUIDFramesUidGet - GET /studies/{StudyInstanceUID}/series/{SeriesInstanceUID}/instances/{SOPInstanceUID}/frames/{uid}
            &hyper::Method::GET if path.matched(paths::ID_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_FRAMES_UID) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_FRAMES_UID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_FRAMES_UID in set but failed match against \"{}\"", path, paths::REGEX_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_FRAMES_UID.as_str())
                    );

                let param_study_instance_uid = match percent_encoding::percent_decode(path_params["StudyInstanceUID"].as_bytes()).decode_utf8() {
                    Ok(param_study_instance_uid) => match param_study_instance_uid.parse::<String>() {
                        Ok(param_study_instance_uid) => param_study_instance_uid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter StudyInstanceUID: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["StudyInstanceUID"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_series_instance_uid = match percent_encoding::percent_decode(path_params["SeriesInstanceUID"].as_bytes()).decode_utf8() {
                    Ok(param_series_instance_uid) => match param_series_instance_uid.parse::<String>() {
                        Ok(param_series_instance_uid) => param_series_instance_uid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter SeriesInstanceUID: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["SeriesInstanceUID"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_sop_instance_uid = match percent_encoding::percent_decode(path_params["SOPInstanceUID"].as_bytes()).decode_utf8() {
                    Ok(param_sop_instance_uid) => match param_sop_instance_uid.parse::<String>() {
                        Ok(param_sop_instance_uid) => param_sop_instance_uid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter SOPInstanceUID: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["SOPInstanceUID"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_uid = match percent_encoding::percent_decode(path_params["uid"].as_bytes()).decode_utf8() {
                    Ok(param_uid) => match param_uid.parse::<String>() {
                        Ok(param_uid) => param_uid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter uid: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["uid"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.studies_study_instance_uid_series_series_instance_uid_instances_sop_instance_uid_frames_uid_get(
                                            param_study_instance_uid,
                                            param_series_instance_uid,
                                            param_sop_instance_uid,
                                            param_uid,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesSOPInstanceUIDFramesUidGetResponse::ASeries
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for STUDIES_STUDY_INSTANCE_UID_SERIES_SERIES_INSTANCE_UID_INSTANCES_SOP_INSTANCE_UID_FRAMES_UID_GET_A_SERIES"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesSOPInstanceUIDGet - GET /studies/{StudyInstanceUID}/series/{SeriesInstanceUID}/instances/{SOPInstanceUID}
            &hyper::Method::GET if path.matched(paths::ID_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID in set but failed match against \"{}\"", path, paths::REGEX_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID.as_str())
                    );

                let param_study_instance_uid = match percent_encoding::percent_decode(path_params["StudyInstanceUID"].as_bytes()).decode_utf8() {
                    Ok(param_study_instance_uid) => match param_study_instance_uid.parse::<String>() {
                        Ok(param_study_instance_uid) => param_study_instance_uid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter StudyInstanceUID: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["StudyInstanceUID"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_series_instance_uid = match percent_encoding::percent_decode(path_params["SeriesInstanceUID"].as_bytes()).decode_utf8() {
                    Ok(param_series_instance_uid) => match param_series_instance_uid.parse::<String>() {
                        Ok(param_series_instance_uid) => param_series_instance_uid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter SeriesInstanceUID: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["SeriesInstanceUID"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_sop_instance_uid = match percent_encoding::percent_decode(path_params["SOPInstanceUID"].as_bytes()).decode_utf8() {
                    Ok(param_sop_instance_uid) => match param_sop_instance_uid.parse::<String>() {
                        Ok(param_sop_instance_uid) => param_sop_instance_uid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter SOPInstanceUID: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["SOPInstanceUID"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.studies_study_instance_uid_series_series_instance_uid_instances_sop_instance_uid_get(
                                            param_study_instance_uid,
                                            param_series_instance_uid,
                                            param_sop_instance_uid,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesSOPInstanceUIDGetResponse::ASeries
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for STUDIES_STUDY_INSTANCE_UID_SERIES_SERIES_INSTANCE_UID_INSTANCES_SOP_INSTANCE_UID_GET_A_SERIES"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesSOPInstanceUIDRenderedGet - GET /studies/{StudyInstanceUID}/series/{SeriesInstanceUID}/instances/{SOPInstanceUID}/rendered
            &hyper::Method::GET if path.matched(paths::ID_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_RENDERED) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_RENDERED
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_RENDERED in set but failed match against \"{}\"", path, paths::REGEX_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_RENDERED.as_str())
                    );

                let param_study_instance_uid = match percent_encoding::percent_decode(path_params["StudyInstanceUID"].as_bytes()).decode_utf8() {
                    Ok(param_study_instance_uid) => match param_study_instance_uid.parse::<String>() {
                        Ok(param_study_instance_uid) => param_study_instance_uid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter StudyInstanceUID: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["StudyInstanceUID"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_series_instance_uid = match percent_encoding::percent_decode(path_params["SeriesInstanceUID"].as_bytes()).decode_utf8() {
                    Ok(param_series_instance_uid) => match param_series_instance_uid.parse::<String>() {
                        Ok(param_series_instance_uid) => param_series_instance_uid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter SeriesInstanceUID: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["SeriesInstanceUID"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_sop_instance_uid = match percent_encoding::percent_decode(path_params["SOPInstanceUID"].as_bytes()).decode_utf8() {
                    Ok(param_sop_instance_uid) => match param_sop_instance_uid.parse::<String>() {
                        Ok(param_sop_instance_uid) => param_sop_instance_uid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter SOPInstanceUID: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["SOPInstanceUID"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_annotation = query_params.iter().filter(|e| e.0 == "annotation").map(|e| e.1.to_owned())
                    .filter_map(|param_annotation| param_annotation.parse().ok())
                    .collect::<Vec<_>>();
                let param_annotation = if !param_annotation.is_empty() {
                    Some(param_annotation)
                } else {
                    None
                };
                let param_quality = query_params.iter().filter(|e| e.0 == "quality").map(|e| e.1.to_owned())
                    .nth(0);
                let param_quality = match param_quality {
                    Some(param_quality) => {
                        let param_quality =
                            <f32 as std::str::FromStr>::from_str
                                (&param_quality);
                        match param_quality {
                            Ok(param_quality) => Some(param_quality),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter quality - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter quality")),
                        }
                    },
                    None => None,
                };
                let param_viewport = query_params.iter().filter(|e| e.0 == "viewport").map(|e| e.1.to_owned())
                    .filter_map(|param_viewport| param_viewport.parse().ok())
                    .collect::<Vec<_>>();
                let param_viewport = if !param_viewport.is_empty() {
                    Some(param_viewport)
                } else {
                    None
                };
                let param_window = query_params.iter().filter(|e| e.0 == "window").map(|e| e.1.to_owned())
                    .filter_map(|param_window| param_window.parse().ok())
                    .collect::<Vec<_>>();
                let param_window = if !param_window.is_empty() {
                    Some(param_window)
                } else {
                    None
                };

                                let result = api_impl.studies_study_instance_uid_series_series_instance_uid_instances_sop_instance_uid_rendered_get(
                                            param_study_instance_uid,
                                            param_series_instance_uid,
                                            param_sop_instance_uid,
                                            param_annotation.as_ref(),
                                            param_quality,
                                            param_viewport.as_ref(),
                                            param_window.as_ref(),
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesSOPInstanceUIDRenderedGetResponse::TheRenderedImageCorrespondingToTheRequestedInstance
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("image/*")
                                                            .expect("Unable to create Content-Type header for STUDIES_STUDY_INSTANCE_UID_SERIES_SERIES_INSTANCE_UID_INSTANCES_SOP_INSTANCE_UID_RENDERED_GET_THE_RENDERED_IMAGE_CORRESPONDING_TO_THE_REQUESTED_INSTANCE"));
                                                    let body = body.0;
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesSOPInstanceUIDThumbnailGet - GET /studies/{StudyInstanceUID}/series/{SeriesInstanceUID}/instances/{SOPInstanceUID}/thumbnail
            &hyper::Method::GET if path.matched(paths::ID_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_THUMBNAIL) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_THUMBNAIL
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_THUMBNAIL in set but failed match against \"{}\"", path, paths::REGEX_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_THUMBNAIL.as_str())
                    );

                let param_study_instance_uid = match percent_encoding::percent_decode(path_params["StudyInstanceUID"].as_bytes()).decode_utf8() {
                    Ok(param_study_instance_uid) => match param_study_instance_uid.parse::<String>() {
                        Ok(param_study_instance_uid) => param_study_instance_uid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter StudyInstanceUID: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["StudyInstanceUID"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_series_instance_uid = match percent_encoding::percent_decode(path_params["SeriesInstanceUID"].as_bytes()).decode_utf8() {
                    Ok(param_series_instance_uid) => match param_series_instance_uid.parse::<String>() {
                        Ok(param_series_instance_uid) => param_series_instance_uid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter SeriesInstanceUID: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["SeriesInstanceUID"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_sop_instance_uid = match percent_encoding::percent_decode(path_params["SOPInstanceUID"].as_bytes()).decode_utf8() {
                    Ok(param_sop_instance_uid) => match param_sop_instance_uid.parse::<String>() {
                        Ok(param_sop_instance_uid) => param_sop_instance_uid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter SOPInstanceUID: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["SOPInstanceUID"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_quality = query_params.iter().filter(|e| e.0 == "quality").map(|e| e.1.to_owned())
                    .nth(0);
                let param_quality = match param_quality {
                    Some(param_quality) => {
                        let param_quality =
                            <f32 as std::str::FromStr>::from_str
                                (&param_quality);
                        match param_quality {
                            Ok(param_quality) => Some(param_quality),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter quality - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter quality")),
                        }
                    },
                    None => None,
                };
                let param_viewport = query_params.iter().filter(|e| e.0 == "viewport").map(|e| e.1.to_owned())
                    .filter_map(|param_viewport| param_viewport.parse().ok())
                    .collect::<Vec<_>>();
                let param_viewport = if !param_viewport.is_empty() {
                    Some(param_viewport)
                } else {
                    None
                };

                                let result = api_impl.studies_study_instance_uid_series_series_instance_uid_instances_sop_instance_uid_thumbnail_get(
                                            param_study_instance_uid,
                                            param_series_instance_uid,
                                            param_sop_instance_uid,
                                            param_quality,
                                            param_viewport.as_ref(),
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesSOPInstanceUIDThumbnailGetResponse::ARepresentativeImageOfAnInstance
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("image/gif")
                                                            .expect("Unable to create Content-Type header for STUDIES_STUDY_INSTANCE_UID_SERIES_SERIES_INSTANCE_UID_INSTANCES_SOP_INSTANCE_UID_THUMBNAIL_GET_A_REPRESENTATIVE_IMAGE_OF_AN_INSTANCE"));
                                                    let body = body.0;
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // StudiesStudyInstanceUIDSeriesSeriesInstanceUIDRenderedGet - GET /studies/{StudyInstanceUID}/series/{SeriesInstanceUID}/rendered
            &hyper::Method::GET if path.matched(paths::ID_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_RENDERED) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_RENDERED
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_RENDERED in set but failed match against \"{}\"", path, paths::REGEX_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_RENDERED.as_str())
                    );

                let param_study_instance_uid = match percent_encoding::percent_decode(path_params["StudyInstanceUID"].as_bytes()).decode_utf8() {
                    Ok(param_study_instance_uid) => match param_study_instance_uid.parse::<String>() {
                        Ok(param_study_instance_uid) => param_study_instance_uid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter StudyInstanceUID: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["StudyInstanceUID"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_series_instance_uid = match percent_encoding::percent_decode(path_params["SeriesInstanceUID"].as_bytes()).decode_utf8() {
                    Ok(param_series_instance_uid) => match param_series_instance_uid.parse::<String>() {
                        Ok(param_series_instance_uid) => param_series_instance_uid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter SeriesInstanceUID: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["SeriesInstanceUID"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_annotation = query_params.iter().filter(|e| e.0 == "annotation").map(|e| e.1.to_owned())
                    .filter_map(|param_annotation| param_annotation.parse().ok())
                    .collect::<Vec<_>>();
                let param_annotation = if !param_annotation.is_empty() {
                    Some(param_annotation)
                } else {
                    None
                };
                let param_quality = query_params.iter().filter(|e| e.0 == "quality").map(|e| e.1.to_owned())
                    .nth(0);
                let param_quality = match param_quality {
                    Some(param_quality) => {
                        let param_quality =
                            <f32 as std::str::FromStr>::from_str
                                (&param_quality);
                        match param_quality {
                            Ok(param_quality) => Some(param_quality),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter quality - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter quality")),
                        }
                    },
                    None => None,
                };
                let param_viewport = query_params.iter().filter(|e| e.0 == "viewport").map(|e| e.1.to_owned())
                    .filter_map(|param_viewport| param_viewport.parse().ok())
                    .collect::<Vec<_>>();
                let param_viewport = if !param_viewport.is_empty() {
                    Some(param_viewport)
                } else {
                    None
                };
                let param_window = query_params.iter().filter(|e| e.0 == "window").map(|e| e.1.to_owned())
                    .filter_map(|param_window| param_window.parse().ok())
                    .collect::<Vec<_>>();
                let param_window = if !param_window.is_empty() {
                    Some(param_window)
                } else {
                    None
                };

                                let result = api_impl.studies_study_instance_uid_series_series_instance_uid_rendered_get(
                                            param_study_instance_uid,
                                            param_series_instance_uid,
                                            param_annotation.as_ref(),
                                            param_quality,
                                            param_viewport.as_ref(),
                                            param_window.as_ref(),
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                StudiesStudyInstanceUIDSeriesSeriesInstanceUIDRenderedGetResponse::TheRenderedImageCorrespondingToTheRequestedSeries
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("image/gif")
                                                            .expect("Unable to create Content-Type header for STUDIES_STUDY_INSTANCE_UID_SERIES_SERIES_INSTANCE_UID_RENDERED_GET_THE_RENDERED_IMAGE_CORRESPONDING_TO_THE_REQUESTED_SERIES"));
                                                    let body = body.0;
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // StudiesStudyInstanceUIDSeriesSeriesInstanceUIDThumbnailGet - GET /studies/{StudyInstanceUID}/series/{SeriesInstanceUID}/thumbnail
            &hyper::Method::GET if path.matched(paths::ID_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_THUMBNAIL) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_THUMBNAIL
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_THUMBNAIL in set but failed match against \"{}\"", path, paths::REGEX_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_THUMBNAIL.as_str())
                    );

                let param_study_instance_uid = match percent_encoding::percent_decode(path_params["StudyInstanceUID"].as_bytes()).decode_utf8() {
                    Ok(param_study_instance_uid) => match param_study_instance_uid.parse::<String>() {
                        Ok(param_study_instance_uid) => param_study_instance_uid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter StudyInstanceUID: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["StudyInstanceUID"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_series_instance_uid = match percent_encoding::percent_decode(path_params["SeriesInstanceUID"].as_bytes()).decode_utf8() {
                    Ok(param_series_instance_uid) => match param_series_instance_uid.parse::<String>() {
                        Ok(param_series_instance_uid) => param_series_instance_uid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter SeriesInstanceUID: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["SeriesInstanceUID"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_quality = query_params.iter().filter(|e| e.0 == "quality").map(|e| e.1.to_owned())
                    .nth(0);
                let param_quality = match param_quality {
                    Some(param_quality) => {
                        let param_quality =
                            <f32 as std::str::FromStr>::from_str
                                (&param_quality);
                        match param_quality {
                            Ok(param_quality) => Some(param_quality),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter quality - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter quality")),
                        }
                    },
                    None => None,
                };
                let param_viewport = query_params.iter().filter(|e| e.0 == "viewport").map(|e| e.1.to_owned())
                    .filter_map(|param_viewport| param_viewport.parse().ok())
                    .collect::<Vec<_>>();
                let param_viewport = if !param_viewport.is_empty() {
                    Some(param_viewport)
                } else {
                    None
                };

                                let result = api_impl.studies_study_instance_uid_series_series_instance_uid_thumbnail_get(
                                            param_study_instance_uid,
                                            param_series_instance_uid,
                                            param_quality,
                                            param_viewport.as_ref(),
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                StudiesStudyInstanceUIDSeriesSeriesInstanceUIDThumbnailGetResponse::ARepresentativeImageOfTheSeries
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("image/gif")
                                                            .expect("Unable to create Content-Type header for STUDIES_STUDY_INSTANCE_UID_SERIES_SERIES_INSTANCE_UID_THUMBNAIL_GET_A_REPRESENTATIVE_IMAGE_OF_THE_SERIES"));
                                                    let body = body.0;
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // StudiesStudyInstanceUIDThumbnailGet - GET /studies/{StudyInstanceUID}/thumbnail
            &hyper::Method::GET if path.matched(paths::ID_STUDIES_STUDYINSTANCEUID_THUMBNAIL) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_STUDIES_STUDYINSTANCEUID_THUMBNAIL
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE STUDIES_STUDYINSTANCEUID_THUMBNAIL in set but failed match against \"{}\"", path, paths::REGEX_STUDIES_STUDYINSTANCEUID_THUMBNAIL.as_str())
                    );

                let param_study_instance_uid = match percent_encoding::percent_decode(path_params["StudyInstanceUID"].as_bytes()).decode_utf8() {
                    Ok(param_study_instance_uid) => match param_study_instance_uid.parse::<String>() {
                        Ok(param_study_instance_uid) => param_study_instance_uid,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter StudyInstanceUID: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["StudyInstanceUID"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_quality = query_params.iter().filter(|e| e.0 == "quality").map(|e| e.1.to_owned())
                    .nth(0);
                let param_quality = match param_quality {
                    Some(param_quality) => {
                        let param_quality =
                            <f32 as std::str::FromStr>::from_str
                                (&param_quality);
                        match param_quality {
                            Ok(param_quality) => Some(param_quality),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter quality - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter quality")),
                        }
                    },
                    None => None,
                };
                let param_viewport = query_params.iter().filter(|e| e.0 == "viewport").map(|e| e.1.to_owned())
                    .filter_map(|param_viewport| param_viewport.parse().ok())
                    .collect::<Vec<_>>();
                let param_viewport = if !param_viewport.is_empty() {
                    Some(param_viewport)
                } else {
                    None
                };

                                let result = api_impl.studies_study_instance_uid_thumbnail_get(
                                            param_study_instance_uid,
                                            param_quality,
                                            param_viewport.as_ref(),
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                StudiesStudyInstanceUIDThumbnailGetResponse::ARepresentativeImageOfTheStudy
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("image/gif")
                                                            .expect("Unable to create Content-Type header for STUDIES_STUDY_INSTANCE_UID_THUMBNAIL_GET_A_REPRESENTATIVE_IMAGE_OF_THE_STUDY"));
                                                    let body = body.0;
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            _ if path.matched(paths::ID_INSTANCES) => method_not_allowed(),
            _ if path.matched(paths::ID_INSTANCES_SOPINSTANCEUID) => method_not_allowed(),
            _ if path.matched(paths::ID_INSTANCES_SOPINSTANCEUID_FRAMES_UID) => method_not_allowed(),
            _ if path.matched(paths::ID_INSTANCES_SOPINSTANCEUID_RENDERED) => method_not_allowed(),
            _ if path.matched(paths::ID_INSTANCES_SOPINSTANCEUID_THUMBNAIL) => method_not_allowed(),
            _ if path.matched(paths::ID_INSTANCES_SOPINSTANCEUID_TAG) => method_not_allowed(),
            _ if path.matched(paths::ID_SERIES) => method_not_allowed(),
            _ if path.matched(paths::ID_SERIES_SERIESINSTANCEUID) => method_not_allowed(),
            _ if path.matched(paths::ID_SERIES_SERIESINSTANCEUID_INSTANCES) => method_not_allowed(),
            _ if path.matched(paths::ID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID) => method_not_allowed(),
            _ if path.matched(paths::ID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_FRAMES_UID) => method_not_allowed(),
            _ if path.matched(paths::ID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_RENDERED) => method_not_allowed(),
            _ if path.matched(paths::ID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_THUMBNAIL) => method_not_allowed(),
            _ if path.matched(paths::ID_SERIES_SERIESINSTANCEUID_RENDERED) => method_not_allowed(),
            _ if path.matched(paths::ID_SERIES_SERIESINSTANCEUID_THUMBNAIL) => method_not_allowed(),
            _ if path.matched(paths::ID_STUDIES) => method_not_allowed(),
            _ if path.matched(paths::ID_STUDIES_STUDYINSTANCEUID) => method_not_allowed(),
            _ if path.matched(paths::ID_STUDIES_STUDYINSTANCEUID_SERIES) => method_not_allowed(),
            _ if path.matched(paths::ID_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID) => method_not_allowed(),
            _ if path.matched(paths::ID_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_INSTANCES) => method_not_allowed(),
            _ if path.matched(paths::ID_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID) => method_not_allowed(),
            _ if path.matched(paths::ID_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_FRAMES_UID) => method_not_allowed(),
            _ if path.matched(paths::ID_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_RENDERED) => method_not_allowed(),
            _ if path.matched(paths::ID_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_THUMBNAIL) => method_not_allowed(),
            _ if path.matched(paths::ID_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_RENDERED) => method_not_allowed(),
            _ if path.matched(paths::ID_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_THUMBNAIL) => method_not_allowed(),
            _ if path.matched(paths::ID_STUDIES_STUDYINSTANCEUID_THUMBNAIL) => method_not_allowed(),
            _ => Ok(Response::builder().status(StatusCode::NOT_FOUND)
                    .body(Body::empty())
                    .expect("Unable to create Not Found response"))
        }
    } Box::pin(run(self.api_impl.clone(), req)) }
}

/// Request parser for `Api`.
pub struct ApiRequestParser;
impl<T> RequestParser<T> for ApiRequestParser {
    fn parse_operation_id(request: &Request<T>) -> Option<&'static str> {
        let path = paths::GLOBAL_REGEX_SET.matches(request.uri().path());
        match request.method() {
            // ApiInstancesTag - GET /instances/{SOPInstanceUID}/{tag}
            &hyper::Method::GET if path.matched(paths::ID_INSTANCES_SOPINSTANCEUID_TAG) => Some("ApiInstancesTag"),
            // InstancesGet - GET /instances
            &hyper::Method::GET if path.matched(paths::ID_INSTANCES) => Some("InstancesGet"),
            // InstancesSOPInstanceUIDFramesUidGet - GET /instances/{SOPInstanceUID}/frames/{uid}
            &hyper::Method::GET if path.matched(paths::ID_INSTANCES_SOPINSTANCEUID_FRAMES_UID) => Some("InstancesSOPInstanceUIDFramesUidGet"),
            // InstancesSOPInstanceUIDGet - GET /instances/{SOPInstanceUID}
            &hyper::Method::GET if path.matched(paths::ID_INSTANCES_SOPINSTANCEUID) => Some("InstancesSOPInstanceUIDGet"),
            // InstancesSOPInstanceUIDRenderedGet - GET /instances/{SOPInstanceUID}/rendered
            &hyper::Method::GET if path.matched(paths::ID_INSTANCES_SOPINSTANCEUID_RENDERED) => Some("InstancesSOPInstanceUIDRenderedGet"),
            // InstancesSOPInstanceUIDThumbnailGet - GET /instances/{SOPInstanceUID}/thumbnail
            &hyper::Method::GET if path.matched(paths::ID_INSTANCES_SOPINSTANCEUID_THUMBNAIL) => Some("InstancesSOPInstanceUIDThumbnailGet"),
            // SeriesGet - GET /series
            &hyper::Method::GET if path.matched(paths::ID_SERIES) => Some("SeriesGet"),
            // SeriesSeriesInstanceUIDGet - GET /series/{SeriesInstanceUID}
            &hyper::Method::GET if path.matched(paths::ID_SERIES_SERIESINSTANCEUID) => Some("SeriesSeriesInstanceUIDGet"),
            // SeriesSeriesInstanceUIDInstancesGet - GET /series/{SeriesInstanceUID}/instances
            &hyper::Method::GET if path.matched(paths::ID_SERIES_SERIESINSTANCEUID_INSTANCES) => Some("SeriesSeriesInstanceUIDInstancesGet"),
            // SeriesSeriesInstanceUIDInstancesSOPInstanceUIDFramesUidGet - GET /series/{SeriesInstanceUID}/instances/{SOPInstanceUID}/frames/{uid}
            &hyper::Method::GET if path.matched(paths::ID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_FRAMES_UID) => Some("SeriesSeriesInstanceUIDInstancesSOPInstanceUIDFramesUidGet"),
            // SeriesSeriesInstanceUIDInstancesSOPInstanceUIDGet - GET /series/{SeriesInstanceUID}/instances/{SOPInstanceUID}
            &hyper::Method::GET if path.matched(paths::ID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID) => Some("SeriesSeriesInstanceUIDInstancesSOPInstanceUIDGet"),
            // SeriesSeriesInstanceUIDInstancesSOPInstanceUIDRenderedGet - GET /series/{SeriesInstanceUID}/instances/{SOPInstanceUID}/rendered
            &hyper::Method::GET if path.matched(paths::ID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_RENDERED) => Some("SeriesSeriesInstanceUIDInstancesSOPInstanceUIDRenderedGet"),
            // SeriesSeriesInstanceUIDInstancesSOPInstanceUIDThumbnailGet - GET /series/{SeriesInstanceUID}/instances/{SOPInstanceUID}/thumbnail
            &hyper::Method::GET if path.matched(paths::ID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_THUMBNAIL) => Some("SeriesSeriesInstanceUIDInstancesSOPInstanceUIDThumbnailGet"),
            // SeriesSeriesInstanceUIDRenderedGet - GET /series/{SeriesInstanceUID}/rendered
            &hyper::Method::GET if path.matched(paths::ID_SERIES_SERIESINSTANCEUID_RENDERED) => Some("SeriesSeriesInstanceUIDRenderedGet"),
            // SeriesSeriesInstanceUIDThumbnailGet - GET /series/{SeriesInstanceUID}/thumbnail
            &hyper::Method::GET if path.matched(paths::ID_SERIES_SERIESINSTANCEUID_THUMBNAIL) => Some("SeriesSeriesInstanceUIDThumbnailGet"),
            // StudiesGet - GET /studies
            &hyper::Method::GET if path.matched(paths::ID_STUDIES) => Some("StudiesGet"),
            // StudiesStudyInstanceUIDGet - GET /studies/{StudyInstanceUID}
            &hyper::Method::GET if path.matched(paths::ID_STUDIES_STUDYINSTANCEUID) => Some("StudiesStudyInstanceUIDGet"),
            // StudiesStudyInstanceUIDSeriesGet - GET /studies/{StudyInstanceUID}/series
            &hyper::Method::GET if path.matched(paths::ID_STUDIES_STUDYINSTANCEUID_SERIES) => Some("StudiesStudyInstanceUIDSeriesGet"),
            // StudiesStudyInstanceUIDSeriesSeriesInstanceUIDGet - GET /studies/{StudyInstanceUID}/series/{SeriesInstanceUID}
            &hyper::Method::GET if path.matched(paths::ID_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID) => Some("StudiesStudyInstanceUIDSeriesSeriesInstanceUIDGet"),
            // StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesGet - GET /studies/{StudyInstanceUID}/series/{SeriesInstanceUID}/instances
            &hyper::Method::GET if path.matched(paths::ID_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_INSTANCES) => Some("StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesGet"),
            // StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesSOPInstanceUIDFramesUidGet - GET /studies/{StudyInstanceUID}/series/{SeriesInstanceUID}/instances/{SOPInstanceUID}/frames/{uid}
            &hyper::Method::GET if path.matched(paths::ID_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_FRAMES_UID) => Some("StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesSOPInstanceUIDFramesUidGet"),
            // StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesSOPInstanceUIDGet - GET /studies/{StudyInstanceUID}/series/{SeriesInstanceUID}/instances/{SOPInstanceUID}
            &hyper::Method::GET if path.matched(paths::ID_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID) => Some("StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesSOPInstanceUIDGet"),
            // StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesSOPInstanceUIDRenderedGet - GET /studies/{StudyInstanceUID}/series/{SeriesInstanceUID}/instances/{SOPInstanceUID}/rendered
            &hyper::Method::GET if path.matched(paths::ID_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_RENDERED) => Some("StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesSOPInstanceUIDRenderedGet"),
            // StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesSOPInstanceUIDThumbnailGet - GET /studies/{StudyInstanceUID}/series/{SeriesInstanceUID}/instances/{SOPInstanceUID}/thumbnail
            &hyper::Method::GET if path.matched(paths::ID_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_INSTANCES_SOPINSTANCEUID_THUMBNAIL) => Some("StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesSOPInstanceUIDThumbnailGet"),
            // StudiesStudyInstanceUIDSeriesSeriesInstanceUIDRenderedGet - GET /studies/{StudyInstanceUID}/series/{SeriesInstanceUID}/rendered
            &hyper::Method::GET if path.matched(paths::ID_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_RENDERED) => Some("StudiesStudyInstanceUIDSeriesSeriesInstanceUIDRenderedGet"),
            // StudiesStudyInstanceUIDSeriesSeriesInstanceUIDThumbnailGet - GET /studies/{StudyInstanceUID}/series/{SeriesInstanceUID}/thumbnail
            &hyper::Method::GET if path.matched(paths::ID_STUDIES_STUDYINSTANCEUID_SERIES_SERIESINSTANCEUID_THUMBNAIL) => Some("StudiesStudyInstanceUIDSeriesSeriesInstanceUIDThumbnailGet"),
            // StudiesStudyInstanceUIDThumbnailGet - GET /studies/{StudyInstanceUID}/thumbnail
            &hyper::Method::GET if path.matched(paths::ID_STUDIES_STUDYINSTANCEUID_THUMBNAIL) => Some("StudiesStudyInstanceUIDThumbnailGet"),
            _ => None,
        }
    }
}
