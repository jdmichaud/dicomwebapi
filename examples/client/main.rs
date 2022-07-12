#![allow(missing_docs, unused_variables, trivial_casts)]


#[allow(unused_imports)]
use futures::{future, Stream, stream};
#[allow(unused_imports)]
use dicomwebapi::{Api, ApiNoContext, Client, ContextWrapperExt, models,
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
                  StudiesStudyInstanceUIDThumbnailGetResponse,
                 };
use clap::{App, Arg};

#[allow(unused_imports)]
use log::info;

// swagger::Has may be unused if there are no examples
#[allow(unused_imports)]
use swagger::{AuthData, ContextBuilder, EmptyContext, Has, Push, XSpanIdString};

type ClientContext = swagger::make_context_ty!(ContextBuilder, EmptyContext, Option<AuthData>, XSpanIdString);

// rt may be unused if there are no examples
#[allow(unused_mut)]
fn main() {
    env_logger::init();

    let matches = App::new("client")
        .arg(Arg::with_name("operation")
            .help("Sets the operation to run")
            .possible_values(&[
                "ApiInstancesTag",
                "InstancesGet",
                "InstancesSOPInstanceUIDFramesUidGet",
                "InstancesSOPInstanceUIDGet",
                "InstancesSOPInstanceUIDRenderedGet",
                "InstancesSOPInstanceUIDThumbnailGet",
                "SeriesGet",
                "SeriesSeriesInstanceUIDGet",
                "SeriesSeriesInstanceUIDInstancesGet",
                "SeriesSeriesInstanceUIDInstancesSOPInstanceUIDFramesUidGet",
                "SeriesSeriesInstanceUIDInstancesSOPInstanceUIDGet",
                "SeriesSeriesInstanceUIDInstancesSOPInstanceUIDRenderedGet",
                "SeriesSeriesInstanceUIDInstancesSOPInstanceUIDThumbnailGet",
                "SeriesSeriesInstanceUIDRenderedGet",
                "SeriesSeriesInstanceUIDThumbnailGet",
                "StudiesGet",
                "StudiesStudyInstanceUIDGet",
                "StudiesStudyInstanceUIDSeriesGet",
                "StudiesStudyInstanceUIDSeriesSeriesInstanceUIDGet",
                "StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesGet",
                "StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesSOPInstanceUIDFramesUidGet",
                "StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesSOPInstanceUIDGet",
                "StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesSOPInstanceUIDRenderedGet",
                "StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesSOPInstanceUIDThumbnailGet",
                "StudiesStudyInstanceUIDSeriesSeriesInstanceUIDRenderedGet",
                "StudiesStudyInstanceUIDSeriesSeriesInstanceUIDThumbnailGet",
                "StudiesStudyInstanceUIDThumbnailGet",
            ])
            .required(true)
            .index(1))
        .arg(Arg::with_name("https")
            .long("https")
            .help("Whether to use HTTPS or not"))
        .arg(Arg::with_name("host")
            .long("host")
            .takes_value(true)
            .default_value("localhost")
            .help("Hostname to contact"))
        .arg(Arg::with_name("port")
            .long("port")
            .takes_value(true)
            .default_value("8080")
            .help("Port to contact"))
        .get_matches();

    let is_https = matches.is_present("https");
    let base_url = format!("{}://{}:{}",
                           if is_https { "https" } else { "http" },
                           matches.value_of("host").unwrap(),
                           matches.value_of("port").unwrap());

    let context: ClientContext =
        swagger::make_context!(ContextBuilder, EmptyContext, None as Option<AuthData>, XSpanIdString::default());

    let mut client : Box<dyn ApiNoContext<ClientContext>> = if matches.is_present("https") {
        // Using Simple HTTPS
        let client = Box::new(Client::try_new_https(&base_url)
            .expect("Failed to create HTTPS client"));
        Box::new(client.with_context(context))
    } else {
        // Using HTTP
        let client = Box::new(Client::try_new_http(
            &base_url)
            .expect("Failed to create HTTP client"));
        Box::new(client.with_context(context))
    };

    let mut rt = tokio::runtime::Runtime::new().unwrap();

    match matches.value_of("operation") {
        Some("ApiInstancesTag") => {
            let result = rt.block_on(client.api_instances_tag(
                  "sop_instance_uid_example".to_string(),
                  "tag_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("InstancesGet") => {
            let result = rt.block_on(client.instances_get(
                  Some(8.14),
                  Some(8.14),
                  Some(true),
                  Some("includefield_example".to_string())
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("InstancesSOPInstanceUIDFramesUidGet") => {
            let result = rt.block_on(client.instances_sop_instance_uid_frames_uid_get(
                  "sop_instance_uid_example".to_string(),
                  "uid_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("InstancesSOPInstanceUIDGet") => {
            let result = rt.block_on(client.instances_sop_instance_uid_get(
                  "sop_instance_uid_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("InstancesSOPInstanceUIDRenderedGet") => {
            let result = rt.block_on(client.instances_sop_instance_uid_rendered_get(
                  "sop_instance_uid_example".to_string(),
                  Some(&Vec::new()),
                  Some(3.4),
                  Some(&Vec::new()),
                  Some(&Vec::new())
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("InstancesSOPInstanceUIDThumbnailGet") => {
            let result = rt.block_on(client.instances_sop_instance_uid_thumbnail_get(
                  "sop_instance_uid_example".to_string(),
                  Some(3.4),
                  Some(&Vec::new())
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("SeriesGet") => {
            let result = rt.block_on(client.series_get(
                  Some(8.14),
                  Some(8.14),
                  Some(true),
                  Some("includefield_example".to_string())
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("SeriesSeriesInstanceUIDGet") => {
            let result = rt.block_on(client.series_series_instance_uid_get(
                  "series_instance_uid_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("SeriesSeriesInstanceUIDInstancesGet") => {
            let result = rt.block_on(client.series_series_instance_uid_instances_get(
                  "series_instance_uid_example".to_string(),
                  Some(8.14),
                  Some(8.14),
                  Some(true),
                  Some("includefield_example".to_string())
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("SeriesSeriesInstanceUIDInstancesSOPInstanceUIDFramesUidGet") => {
            let result = rt.block_on(client.series_series_instance_uid_instances_sop_instance_uid_frames_uid_get(
                  "series_instance_uid_example".to_string(),
                  "sop_instance_uid_example".to_string(),
                  "uid_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("SeriesSeriesInstanceUIDInstancesSOPInstanceUIDGet") => {
            let result = rt.block_on(client.series_series_instance_uid_instances_sop_instance_uid_get(
                  "series_instance_uid_example".to_string(),
                  "sop_instance_uid_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("SeriesSeriesInstanceUIDInstancesSOPInstanceUIDRenderedGet") => {
            let result = rt.block_on(client.series_series_instance_uid_instances_sop_instance_uid_rendered_get(
                  "series_instance_uid_example".to_string(),
                  "sop_instance_uid_example".to_string(),
                  Some(&Vec::new()),
                  Some(3.4),
                  Some(&Vec::new()),
                  Some(&Vec::new())
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("SeriesSeriesInstanceUIDInstancesSOPInstanceUIDThumbnailGet") => {
            let result = rt.block_on(client.series_series_instance_uid_instances_sop_instance_uid_thumbnail_get(
                  "series_instance_uid_example".to_string(),
                  "sop_instance_uid_example".to_string(),
                  Some(3.4),
                  Some(&Vec::new())
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("SeriesSeriesInstanceUIDRenderedGet") => {
            let result = rt.block_on(client.series_series_instance_uid_rendered_get(
                  "series_instance_uid_example".to_string(),
                  Some(&Vec::new()),
                  Some(3.4),
                  Some(&Vec::new()),
                  Some(&Vec::new())
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("SeriesSeriesInstanceUIDThumbnailGet") => {
            let result = rt.block_on(client.series_series_instance_uid_thumbnail_get(
                  "series_instance_uid_example".to_string(),
                  Some(3.4),
                  Some(&Vec::new())
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("StudiesGet") => {
            let result = rt.block_on(client.studies_get(
                  Some(8.14),
                  Some(8.14),
                  Some(true),
                  Some("includefield_example".to_string())
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("StudiesStudyInstanceUIDGet") => {
            let result = rt.block_on(client.studies_study_instance_uid_get(
                  "study_instance_uid_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("StudiesStudyInstanceUIDSeriesGet") => {
            let result = rt.block_on(client.studies_study_instance_uid_series_get(
                  "study_instance_uid_example".to_string(),
                  Some(8.14),
                  Some(8.14),
                  Some(true),
                  Some("includefield_example".to_string())
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("StudiesStudyInstanceUIDSeriesSeriesInstanceUIDGet") => {
            let result = rt.block_on(client.studies_study_instance_uid_series_series_instance_uid_get(
                  "study_instance_uid_example".to_string(),
                  "series_instance_uid_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesGet") => {
            let result = rt.block_on(client.studies_study_instance_uid_series_series_instance_uid_instances_get(
                  "study_instance_uid_example".to_string(),
                  "series_instance_uid_example".to_string(),
                  Some(8.14),
                  Some(8.14),
                  Some(true),
                  Some("includefield_example".to_string())
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesSOPInstanceUIDFramesUidGet") => {
            let result = rt.block_on(client.studies_study_instance_uid_series_series_instance_uid_instances_sop_instance_uid_frames_uid_get(
                  "study_instance_uid_example".to_string(),
                  "series_instance_uid_example".to_string(),
                  "sop_instance_uid_example".to_string(),
                  "uid_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesSOPInstanceUIDGet") => {
            let result = rt.block_on(client.studies_study_instance_uid_series_series_instance_uid_instances_sop_instance_uid_get(
                  "study_instance_uid_example".to_string(),
                  "series_instance_uid_example".to_string(),
                  "sop_instance_uid_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesSOPInstanceUIDRenderedGet") => {
            let result = rt.block_on(client.studies_study_instance_uid_series_series_instance_uid_instances_sop_instance_uid_rendered_get(
                  "study_instance_uid_example".to_string(),
                  "series_instance_uid_example".to_string(),
                  "sop_instance_uid_example".to_string(),
                  Some(&Vec::new()),
                  Some(3.4),
                  Some(&Vec::new()),
                  Some(&Vec::new())
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesSOPInstanceUIDThumbnailGet") => {
            let result = rt.block_on(client.studies_study_instance_uid_series_series_instance_uid_instances_sop_instance_uid_thumbnail_get(
                  "study_instance_uid_example".to_string(),
                  "series_instance_uid_example".to_string(),
                  "sop_instance_uid_example".to_string(),
                  Some(3.4),
                  Some(&Vec::new())
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("StudiesStudyInstanceUIDSeriesSeriesInstanceUIDRenderedGet") => {
            let result = rt.block_on(client.studies_study_instance_uid_series_series_instance_uid_rendered_get(
                  "study_instance_uid_example".to_string(),
                  "series_instance_uid_example".to_string(),
                  Some(&Vec::new()),
                  Some(3.4),
                  Some(&Vec::new()),
                  Some(&Vec::new())
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("StudiesStudyInstanceUIDSeriesSeriesInstanceUIDThumbnailGet") => {
            let result = rt.block_on(client.studies_study_instance_uid_series_series_instance_uid_thumbnail_get(
                  "study_instance_uid_example".to_string(),
                  "series_instance_uid_example".to_string(),
                  Some(3.4),
                  Some(&Vec::new())
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("StudiesStudyInstanceUIDThumbnailGet") => {
            let result = rt.block_on(client.studies_study_instance_uid_thumbnail_get(
                  "study_instance_uid_example".to_string(),
                  Some(3.4),
                  Some(&Vec::new())
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        _ => {
            panic!("Invalid operation provided")
        }
    }
}
