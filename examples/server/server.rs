//! Main library entry point for dicomwebapi implementation.

#![allow(unused_imports)]

use async_trait::async_trait;
use futures::{future, Stream, StreamExt, TryFutureExt, TryStreamExt};
use hyper::server::conn::Http;
use hyper::service::Service;
use log::info;
use std::future::Future;
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use swagger::{Has, XSpanIdString};
use swagger::auth::MakeAllowAllAuthenticator;
use swagger::EmptyContext;
use tokio::net::TcpListener;

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
use openssl::ssl::{Ssl, SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};

use dicomwebapi::models;

/// Builds an SSL implementation for Simple HTTPS from some hard-coded file names
pub async fn create(addr: &str, https: bool) {
    let addr = addr.parse().expect("Failed to parse bind address");

    let server = Server::new();

    let service = MakeService::new(server);

    let service = MakeAllowAllAuthenticator::new(service, "cosmo");

    let mut service =
        dicomwebapi::server::context::MakeAddContext::<_, EmptyContext>::new(
            service
        );

    if https {
        #[cfg(any(target_os = "macos", target_os = "windows", target_os = "ios"))]
        {
            unimplemented!("SSL is not implemented for the examples on MacOS, Windows or iOS");
        }

        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
        {
            let mut ssl = SslAcceptor::mozilla_intermediate_v5(SslMethod::tls()).expect("Failed to create SSL Acceptor");

            // Server authentication
            ssl.set_private_key_file("examples/server-key.pem", SslFiletype::PEM).expect("Failed to set private key");
            ssl.set_certificate_chain_file("examples/server-chain.pem").expect("Failed to set certificate chain");
            ssl.check_private_key().expect("Failed to check private key");

            let tls_acceptor = ssl.build();
            let tcp_listener = TcpListener::bind(&addr).await.unwrap();

            loop {
                if let Ok((tcp, _)) = tcp_listener.accept().await {
                    let ssl = Ssl::new(tls_acceptor.context()).unwrap();
                    let addr = tcp.peer_addr().expect("Unable to get remote address");
                    let service = service.call(addr);

                    tokio::spawn(async move {
                        let tls = tokio_openssl::SslStream::new(ssl, tcp).map_err(|_| ())?;
                        let service = service.await.map_err(|_| ())?;

                        Http::new()
                            .serve_connection(tls, service)
                            .await
                            .map_err(|_| ())
                    });
                }
            }
        }
    } else {
        // Using HTTP
        hyper::server::Server::bind(&addr).serve(service).await.unwrap()
    }
}

#[derive(Copy, Clone)]
pub struct Server<C> {
    marker: PhantomData<C>,
}

impl<C> Server<C> {
    pub fn new() -> Self {
        Server{marker: PhantomData}
    }
}


use dicomwebapi::{
    Api,
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
use dicomwebapi::server::MakeService;
use std::error::Error;
use swagger::ApiError;

#[async_trait]
impl<C> Api<C> for Server<C> where C: Has<XSpanIdString> + Send + Sync
{
    /// Retrieve an instance field.
    async fn api_instances_tag(
        &self,
        sop_instance_uid: String,
        tag: String,
        context: &C) -> Result<ApiInstancesTagResponse, ApiError>
    {
        let context = context.clone();
        info!("api_instances_tag(\"{}\", \"{}\") - X-Span-ID: {:?}", sop_instance_uid, tag, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Query for instances.
    async fn instances_get(
        &self,
        limit: Option<f64>,
        offset: Option<f64>,
        fuzzymatching: Option<bool>,
        includefield: Option<String>,
        context: &C) -> Result<InstancesGetResponse, ApiError>
    {
        let context = context.clone();
        info!("instances_get({:?}, {:?}, {:?}, {:?}) - X-Span-ID: {:?}", limit, offset, fuzzymatching, includefield, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Query for a series in a study.
    async fn instances_sop_instance_uid_frames_uid_get(
        &self,
        sop_instance_uid: String,
        uid: String,
        context: &C) -> Result<InstancesSOPInstanceUIDFramesUidGetResponse, ApiError>
    {
        let context = context.clone();
        info!("instances_sop_instance_uid_frames_uid_get(\"{}\", \"{}\") - X-Span-ID: {:?}", sop_instance_uid, uid, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Retrieve a instance.
    async fn instances_sop_instance_uid_get(
        &self,
        sop_instance_uid: String,
        context: &C) -> Result<InstancesSOPInstanceUIDGetResponse, ApiError>
    {
        let context = context.clone();
        info!("instances_sop_instance_uid_get(\"{}\") - X-Span-ID: {:?}", sop_instance_uid, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Render an instance.
    async fn instances_sop_instance_uid_rendered_get(
        &self,
        sop_instance_uid: String,
        annotation: Option<&Vec<String>>,
        quality: Option<f32>,
        viewport: Option<&Vec<i32>>,
        window: Option<&Vec<String>>,
        context: &C) -> Result<InstancesSOPInstanceUIDRenderedGetResponse, ApiError>
    {
        let context = context.clone();
        info!("instances_sop_instance_uid_rendered_get(\"{}\", {:?}, {:?}, {:?}, {:?}) - X-Span-ID: {:?}", sop_instance_uid, annotation, quality, viewport, window, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Render a thumbnail.
    async fn instances_sop_instance_uid_thumbnail_get(
        &self,
        sop_instance_uid: String,
        quality: Option<f32>,
        viewport: Option<&Vec<i32>>,
        context: &C) -> Result<InstancesSOPInstanceUIDThumbnailGetResponse, ApiError>
    {
        let context = context.clone();
        info!("instances_sop_instance_uid_thumbnail_get(\"{}\", {:?}, {:?}) - X-Span-ID: {:?}", sop_instance_uid, quality, viewport, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Query for series.
    async fn series_get(
        &self,
        limit: Option<f64>,
        offset: Option<f64>,
        fuzzymatching: Option<bool>,
        includefield: Option<String>,
        context: &C) -> Result<SeriesGetResponse, ApiError>
    {
        let context = context.clone();
        info!("series_get({:?}, {:?}, {:?}, {:?}) - X-Span-ID: {:?}", limit, offset, fuzzymatching, includefield, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Retrieve a series.
    async fn series_series_instance_uid_get(
        &self,
        series_instance_uid: String,
        context: &C) -> Result<SeriesSeriesInstanceUIDGetResponse, ApiError>
    {
        let context = context.clone();
        info!("series_series_instance_uid_get(\"{}\") - X-Span-ID: {:?}", series_instance_uid, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Query for instances in the series of a study.
    async fn series_series_instance_uid_instances_get(
        &self,
        series_instance_uid: String,
        limit: Option<f64>,
        offset: Option<f64>,
        fuzzymatching: Option<bool>,
        includefield: Option<String>,
        context: &C) -> Result<SeriesSeriesInstanceUIDInstancesGetResponse, ApiError>
    {
        let context = context.clone();
        info!("series_series_instance_uid_instances_get(\"{}\", {:?}, {:?}, {:?}, {:?}) - X-Span-ID: {:?}", series_instance_uid, limit, offset, fuzzymatching, includefield, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Query for a series in a study.
    async fn series_series_instance_uid_instances_sop_instance_uid_frames_uid_get(
        &self,
        series_instance_uid: String,
        sop_instance_uid: String,
        uid: String,
        context: &C) -> Result<SeriesSeriesInstanceUIDInstancesSOPInstanceUIDFramesUidGetResponse, ApiError>
    {
        let context = context.clone();
        info!("series_series_instance_uid_instances_sop_instance_uid_frames_uid_get(\"{}\", \"{}\", \"{}\") - X-Span-ID: {:?}", series_instance_uid, sop_instance_uid, uid, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Query for a series in a study.
    async fn series_series_instance_uid_instances_sop_instance_uid_get(
        &self,
        series_instance_uid: String,
        sop_instance_uid: String,
        context: &C) -> Result<SeriesSeriesInstanceUIDInstancesSOPInstanceUIDGetResponse, ApiError>
    {
        let context = context.clone();
        info!("series_series_instance_uid_instances_sop_instance_uid_get(\"{}\", \"{}\") - X-Span-ID: {:?}", series_instance_uid, sop_instance_uid, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Render an instance.
    async fn series_series_instance_uid_instances_sop_instance_uid_rendered_get(
        &self,
        series_instance_uid: String,
        sop_instance_uid: String,
        annotation: Option<&Vec<String>>,
        quality: Option<f32>,
        viewport: Option<&Vec<i32>>,
        window: Option<&Vec<String>>,
        context: &C) -> Result<SeriesSeriesInstanceUIDInstancesSOPInstanceUIDRenderedGetResponse, ApiError>
    {
        let context = context.clone();
        info!("series_series_instance_uid_instances_sop_instance_uid_rendered_get(\"{}\", \"{}\", {:?}, {:?}, {:?}, {:?}) - X-Span-ID: {:?}", series_instance_uid, sop_instance_uid, annotation, quality, viewport, window, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Render a thumbnail.
    async fn series_series_instance_uid_instances_sop_instance_uid_thumbnail_get(
        &self,
        series_instance_uid: String,
        sop_instance_uid: String,
        quality: Option<f32>,
        viewport: Option<&Vec<i32>>,
        context: &C) -> Result<SeriesSeriesInstanceUIDInstancesSOPInstanceUIDThumbnailGetResponse, ApiError>
    {
        let context = context.clone();
        info!("series_series_instance_uid_instances_sop_instance_uid_thumbnail_get(\"{}\", \"{}\", {:?}, {:?}) - X-Span-ID: {:?}", series_instance_uid, sop_instance_uid, quality, viewport, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Render an instance.
    async fn series_series_instance_uid_rendered_get(
        &self,
        series_instance_uid: String,
        annotation: Option<&Vec<String>>,
        quality: Option<f32>,
        viewport: Option<&Vec<i32>>,
        window: Option<&Vec<String>>,
        context: &C) -> Result<SeriesSeriesInstanceUIDRenderedGetResponse, ApiError>
    {
        let context = context.clone();
        info!("series_series_instance_uid_rendered_get(\"{}\", {:?}, {:?}, {:?}, {:?}) - X-Span-ID: {:?}", series_instance_uid, annotation, quality, viewport, window, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Render a thumbnail.
    async fn series_series_instance_uid_thumbnail_get(
        &self,
        series_instance_uid: String,
        quality: Option<f32>,
        viewport: Option<&Vec<i32>>,
        context: &C) -> Result<SeriesSeriesInstanceUIDThumbnailGetResponse, ApiError>
    {
        let context = context.clone();
        info!("series_series_instance_uid_thumbnail_get(\"{}\", {:?}, {:?}) - X-Span-ID: {:?}", series_instance_uid, quality, viewport, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Query for studies.
    async fn studies_get(
        &self,
        limit: Option<f64>,
        offset: Option<f64>,
        fuzzymatching: Option<bool>,
        includefield: Option<String>,
        context: &C) -> Result<StudiesGetResponse, ApiError>
    {
        let context = context.clone();
        info!("studies_get({:?}, {:?}, {:?}, {:?}) - X-Span-ID: {:?}", limit, offset, fuzzymatching, includefield, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Retrieve a study.
    async fn studies_study_instance_uid_get(
        &self,
        study_instance_uid: String,
        context: &C) -> Result<StudiesStudyInstanceUIDGetResponse, ApiError>
    {
        let context = context.clone();
        info!("studies_study_instance_uid_get(\"{}\") - X-Span-ID: {:?}", study_instance_uid, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Query for series in a study.
    async fn studies_study_instance_uid_series_get(
        &self,
        study_instance_uid: String,
        limit: Option<f64>,
        offset: Option<f64>,
        fuzzymatching: Option<bool>,
        includefield: Option<String>,
        context: &C) -> Result<StudiesStudyInstanceUIDSeriesGetResponse, ApiError>
    {
        let context = context.clone();
        info!("studies_study_instance_uid_series_get(\"{}\", {:?}, {:?}, {:?}, {:?}) - X-Span-ID: {:?}", study_instance_uid, limit, offset, fuzzymatching, includefield, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Query for a series in a study.
    async fn studies_study_instance_uid_series_series_instance_uid_get(
        &self,
        study_instance_uid: String,
        series_instance_uid: String,
        context: &C) -> Result<StudiesStudyInstanceUIDSeriesSeriesInstanceUIDGetResponse, ApiError>
    {
        let context = context.clone();
        info!("studies_study_instance_uid_series_series_instance_uid_get(\"{}\", \"{}\") - X-Span-ID: {:?}", study_instance_uid, series_instance_uid, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Query for instances in the series of a study.
    async fn studies_study_instance_uid_series_series_instance_uid_instances_get(
        &self,
        study_instance_uid: String,
        series_instance_uid: String,
        limit: Option<f64>,
        offset: Option<f64>,
        fuzzymatching: Option<bool>,
        includefield: Option<String>,
        context: &C) -> Result<StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesGetResponse, ApiError>
    {
        let context = context.clone();
        info!("studies_study_instance_uid_series_series_instance_uid_instances_get(\"{}\", \"{}\", {:?}, {:?}, {:?}, {:?}) - X-Span-ID: {:?}", study_instance_uid, series_instance_uid, limit, offset, fuzzymatching, includefield, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Query for a series in a study.
    async fn studies_study_instance_uid_series_series_instance_uid_instances_sop_instance_uid_frames_uid_get(
        &self,
        study_instance_uid: String,
        series_instance_uid: String,
        sop_instance_uid: String,
        uid: String,
        context: &C) -> Result<StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesSOPInstanceUIDFramesUidGetResponse, ApiError>
    {
        let context = context.clone();
        info!("studies_study_instance_uid_series_series_instance_uid_instances_sop_instance_uid_frames_uid_get(\"{}\", \"{}\", \"{}\", \"{}\") - X-Span-ID: {:?}", study_instance_uid, series_instance_uid, sop_instance_uid, uid, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Query for a series in a study.
    async fn studies_study_instance_uid_series_series_instance_uid_instances_sop_instance_uid_get(
        &self,
        study_instance_uid: String,
        series_instance_uid: String,
        sop_instance_uid: String,
        context: &C) -> Result<StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesSOPInstanceUIDGetResponse, ApiError>
    {
        let context = context.clone();
        info!("studies_study_instance_uid_series_series_instance_uid_instances_sop_instance_uid_get(\"{}\", \"{}\", \"{}\") - X-Span-ID: {:?}", study_instance_uid, series_instance_uid, sop_instance_uid, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Render an instance.
    async fn studies_study_instance_uid_series_series_instance_uid_instances_sop_instance_uid_rendered_get(
        &self,
        study_instance_uid: String,
        series_instance_uid: String,
        sop_instance_uid: String,
        annotation: Option<&Vec<String>>,
        quality: Option<f32>,
        viewport: Option<&Vec<i32>>,
        window: Option<&Vec<String>>,
        context: &C) -> Result<StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesSOPInstanceUIDRenderedGetResponse, ApiError>
    {
        let context = context.clone();
        info!("studies_study_instance_uid_series_series_instance_uid_instances_sop_instance_uid_rendered_get(\"{}\", \"{}\", \"{}\", {:?}, {:?}, {:?}, {:?}) - X-Span-ID: {:?}", study_instance_uid, series_instance_uid, sop_instance_uid, annotation, quality, viewport, window, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Render a thumbnail.
    async fn studies_study_instance_uid_series_series_instance_uid_instances_sop_instance_uid_thumbnail_get(
        &self,
        study_instance_uid: String,
        series_instance_uid: String,
        sop_instance_uid: String,
        quality: Option<f32>,
        viewport: Option<&Vec<i32>>,
        context: &C) -> Result<StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesSOPInstanceUIDThumbnailGetResponse, ApiError>
    {
        let context = context.clone();
        info!("studies_study_instance_uid_series_series_instance_uid_instances_sop_instance_uid_thumbnail_get(\"{}\", \"{}\", \"{}\", {:?}, {:?}) - X-Span-ID: {:?}", study_instance_uid, series_instance_uid, sop_instance_uid, quality, viewport, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Render a series.
    async fn studies_study_instance_uid_series_series_instance_uid_rendered_get(
        &self,
        study_instance_uid: String,
        series_instance_uid: String,
        annotation: Option<&Vec<String>>,
        quality: Option<f32>,
        viewport: Option<&Vec<i32>>,
        window: Option<&Vec<String>>,
        context: &C) -> Result<StudiesStudyInstanceUIDSeriesSeriesInstanceUIDRenderedGetResponse, ApiError>
    {
        let context = context.clone();
        info!("studies_study_instance_uid_series_series_instance_uid_rendered_get(\"{}\", \"{}\", {:?}, {:?}, {:?}, {:?}) - X-Span-ID: {:?}", study_instance_uid, series_instance_uid, annotation, quality, viewport, window, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Render a thumbnail.
    async fn studies_study_instance_uid_series_series_instance_uid_thumbnail_get(
        &self,
        study_instance_uid: String,
        series_instance_uid: String,
        quality: Option<f32>,
        viewport: Option<&Vec<i32>>,
        context: &C) -> Result<StudiesStudyInstanceUIDSeriesSeriesInstanceUIDThumbnailGetResponse, ApiError>
    {
        let context = context.clone();
        info!("studies_study_instance_uid_series_series_instance_uid_thumbnail_get(\"{}\", \"{}\", {:?}, {:?}) - X-Span-ID: {:?}", study_instance_uid, series_instance_uid, quality, viewport, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Render a thumbnail.
    async fn studies_study_instance_uid_thumbnail_get(
        &self,
        study_instance_uid: String,
        quality: Option<f32>,
        viewport: Option<&Vec<i32>>,
        context: &C) -> Result<StudiesStudyInstanceUIDThumbnailGetResponse, ApiError>
    {
        let context = context.clone();
        info!("studies_study_instance_uid_thumbnail_get(\"{}\", {:?}, {:?}) - X-Span-ID: {:?}", study_instance_uid, quality, viewport, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

}
