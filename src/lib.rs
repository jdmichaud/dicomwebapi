#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]

use async_trait::async_trait;
use futures::Stream;
use std::error::Error;
use std::task::{Poll, Context};
use swagger::{ApiError, ContextWrapper};
use serde::{Serialize, Deserialize};

type ServiceError = Box<dyn Error + Send + Sync + 'static>;

pub const BASE_PATH: &'static str = "";
pub const API_VERSION: &'static str = "0.1.0";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ApiInstancesTagResponse {
    /// An instance field.
    AnInstanceField
    (serde_json::Value)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum InstancesGetResponse {
    /// A JSON array of instances.
    AJSONArrayOfInstances
    (Vec<models::StudiesStudyInstanceUidSeriesSeriesInstanceUidInstancesGet200ResponseInner>)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum InstancesSOPInstanceUIDFramesUidGetResponse {
    /// A series.
    ASeries
    (serde_json::Value)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum InstancesSOPInstanceUIDGetResponse {
    /// A full instance.
    AFullInstance
    (serde_json::Value)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum InstancesSOPInstanceUIDRenderedGetResponse {
    /// The rendered image corresponding to the requested instance.
    TheRenderedImageCorrespondingToTheRequestedInstance
    (swagger::ByteArray)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum InstancesSOPInstanceUIDThumbnailGetResponse {
    /// A representative image of an instance.
    ARepresentativeImageOfAnInstance
    (swagger::ByteArray)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SeriesGetResponse {
    /// A JSON array of series.
    AJSONArrayOfSeries
    (Vec<models::SeriesGet200ResponseInner>)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SeriesSeriesInstanceUIDGetResponse {
    /// A full series.
    AFullSeries
    (serde_json::Value)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SeriesSeriesInstanceUIDInstancesGetResponse {
    /// A JSON array of instances.
    AJSONArrayOfInstances
    (Vec<models::StudiesStudyInstanceUidSeriesSeriesInstanceUidInstancesGet200ResponseInner>)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SeriesSeriesInstanceUIDInstancesSOPInstanceUIDFramesUidGetResponse {
    /// A series.
    ASeries
    (serde_json::Value)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SeriesSeriesInstanceUIDInstancesSOPInstanceUIDGetResponse {
    /// A series.
    ASeries
    (serde_json::Value)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SeriesSeriesInstanceUIDInstancesSOPInstanceUIDRenderedGetResponse {
    /// The rendered image corresponding to the requested instance.
    TheRenderedImageCorrespondingToTheRequestedInstance
    (swagger::ByteArray)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SeriesSeriesInstanceUIDInstancesSOPInstanceUIDThumbnailGetResponse {
    /// A representative image of an instance.
    ARepresentativeImageOfAnInstance
    (swagger::ByteArray)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SeriesSeriesInstanceUIDRenderedGetResponse {
    /// The rendered images corresponding to the requested series.
    TheRenderedImagesCorrespondingToTheRequestedSeries
    (swagger::ByteArray)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SeriesSeriesInstanceUIDThumbnailGetResponse {
    /// A representative image of the series.
    ARepresentativeImageOfTheSeries
    (swagger::ByteArray)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum StudiesGetResponse {
    /// A JSON array of studies.
    AJSONArrayOfStudies
    (Vec<models::StudiesGet200ResponseInner>)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum StudiesStudyInstanceUIDGetResponse {
    /// A full study.
    AFullStudy
    (serde_json::Value)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum StudiesStudyInstanceUIDSeriesGetResponse {
    /// A JSON array of series.
    AJSONArrayOfSeries
    (Vec<models::StudiesGet200ResponseInner>)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum StudiesStudyInstanceUIDSeriesSeriesInstanceUIDGetResponse {
    /// A series.
    ASeries
    (serde_json::Value)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesGetResponse {
    /// A JSON array of instances.
    AJSONArrayOfInstances
    (Vec<models::StudiesStudyInstanceUidSeriesSeriesInstanceUidInstancesGet200ResponseInner>)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesSOPInstanceUIDFramesUidGetResponse {
    /// A series.
    ASeries
    (serde_json::Value)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesSOPInstanceUIDGetResponse {
    /// A series.
    ASeries
    (serde_json::Value)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesSOPInstanceUIDRenderedGetResponse {
    /// The rendered image corresponding to the requested instance.
    TheRenderedImageCorrespondingToTheRequestedInstance
    (swagger::ByteArray)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesSOPInstanceUIDThumbnailGetResponse {
    /// A representative image of an instance.
    ARepresentativeImageOfAnInstance
    (swagger::ByteArray)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum StudiesStudyInstanceUIDSeriesSeriesInstanceUIDRenderedGetResponse {
    /// The rendered image corresponding to the requested series.
    TheRenderedImageCorrespondingToTheRequestedSeries
    (swagger::ByteArray)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum StudiesStudyInstanceUIDSeriesSeriesInstanceUIDThumbnailGetResponse {
    /// A representative image of the series.
    ARepresentativeImageOfTheSeries
    (swagger::ByteArray)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum StudiesStudyInstanceUIDThumbnailGetResponse {
    /// A representative image of the study.
    ARepresentativeImageOfTheStudy
    (swagger::ByteArray)
}

/// API
#[async_trait]
pub trait Api<C: Send + Sync> {
    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>> {
        Poll::Ready(Ok(()))
    }

    /// Retrieve an instance field.
    async fn api_instances_tag(
        &self,
        sop_instance_uid: String,
        tag: String,
        context: &C) -> Result<ApiInstancesTagResponse, ApiError>;

    /// Query for instances.
    async fn instances_get(
        &self,
        limit: Option<f64>,
        offset: Option<f64>,
        fuzzymatching: Option<bool>,
        includefield: Option<String>,
        context: &C) -> Result<InstancesGetResponse, ApiError>;

    /// Query for a series in a study.
    async fn instances_sop_instance_uid_frames_uid_get(
        &self,
        sop_instance_uid: String,
        uid: String,
        context: &C) -> Result<InstancesSOPInstanceUIDFramesUidGetResponse, ApiError>;

    /// Retrieve a instance.
    async fn instances_sop_instance_uid_get(
        &self,
        sop_instance_uid: String,
        context: &C) -> Result<InstancesSOPInstanceUIDGetResponse, ApiError>;

    /// Render an instance.
    async fn instances_sop_instance_uid_rendered_get(
        &self,
        sop_instance_uid: String,
        annotation: Option<&Vec<String>>,
        quality: Option<f32>,
        viewport: Option<&Vec<i32>>,
        window: Option<&Vec<String>>,
        context: &C) -> Result<InstancesSOPInstanceUIDRenderedGetResponse, ApiError>;

    /// Render a thumbnail.
    async fn instances_sop_instance_uid_thumbnail_get(
        &self,
        sop_instance_uid: String,
        quality: Option<f32>,
        viewport: Option<&Vec<i32>>,
        context: &C) -> Result<InstancesSOPInstanceUIDThumbnailGetResponse, ApiError>;

    /// Query for series.
    async fn series_get(
        &self,
        limit: Option<f64>,
        offset: Option<f64>,
        fuzzymatching: Option<bool>,
        includefield: Option<String>,
        context: &C) -> Result<SeriesGetResponse, ApiError>;

    /// Retrieve a series.
    async fn series_series_instance_uid_get(
        &self,
        series_instance_uid: String,
        context: &C) -> Result<SeriesSeriesInstanceUIDGetResponse, ApiError>;

    /// Query for instances in the series of a study.
    async fn series_series_instance_uid_instances_get(
        &self,
        series_instance_uid: String,
        limit: Option<f64>,
        offset: Option<f64>,
        fuzzymatching: Option<bool>,
        includefield: Option<String>,
        context: &C) -> Result<SeriesSeriesInstanceUIDInstancesGetResponse, ApiError>;

    /// Query for a series in a study.
    async fn series_series_instance_uid_instances_sop_instance_uid_frames_uid_get(
        &self,
        series_instance_uid: String,
        sop_instance_uid: String,
        uid: String,
        context: &C) -> Result<SeriesSeriesInstanceUIDInstancesSOPInstanceUIDFramesUidGetResponse, ApiError>;

    /// Query for a series in a study.
    async fn series_series_instance_uid_instances_sop_instance_uid_get(
        &self,
        series_instance_uid: String,
        sop_instance_uid: String,
        context: &C) -> Result<SeriesSeriesInstanceUIDInstancesSOPInstanceUIDGetResponse, ApiError>;

    /// Render an instance.
    async fn series_series_instance_uid_instances_sop_instance_uid_rendered_get(
        &self,
        series_instance_uid: String,
        sop_instance_uid: String,
        annotation: Option<&Vec<String>>,
        quality: Option<f32>,
        viewport: Option<&Vec<i32>>,
        window: Option<&Vec<String>>,
        context: &C) -> Result<SeriesSeriesInstanceUIDInstancesSOPInstanceUIDRenderedGetResponse, ApiError>;

    /// Render a thumbnail.
    async fn series_series_instance_uid_instances_sop_instance_uid_thumbnail_get(
        &self,
        series_instance_uid: String,
        sop_instance_uid: String,
        quality: Option<f32>,
        viewport: Option<&Vec<i32>>,
        context: &C) -> Result<SeriesSeriesInstanceUIDInstancesSOPInstanceUIDThumbnailGetResponse, ApiError>;

    /// Render an instance.
    async fn series_series_instance_uid_rendered_get(
        &self,
        series_instance_uid: String,
        annotation: Option<&Vec<String>>,
        quality: Option<f32>,
        viewport: Option<&Vec<i32>>,
        window: Option<&Vec<String>>,
        context: &C) -> Result<SeriesSeriesInstanceUIDRenderedGetResponse, ApiError>;

    /// Render a thumbnail.
    async fn series_series_instance_uid_thumbnail_get(
        &self,
        series_instance_uid: String,
        quality: Option<f32>,
        viewport: Option<&Vec<i32>>,
        context: &C) -> Result<SeriesSeriesInstanceUIDThumbnailGetResponse, ApiError>;

    /// Query for studies.
    async fn studies_get(
        &self,
        limit: Option<f64>,
        offset: Option<f64>,
        fuzzymatching: Option<bool>,
        includefield: Option<String>,
        context: &C) -> Result<StudiesGetResponse, ApiError>;

    /// Retrieve a study.
    async fn studies_study_instance_uid_get(
        &self,
        study_instance_uid: String,
        context: &C) -> Result<StudiesStudyInstanceUIDGetResponse, ApiError>;

    /// Query for series in a study.
    async fn studies_study_instance_uid_series_get(
        &self,
        study_instance_uid: String,
        limit: Option<f64>,
        offset: Option<f64>,
        fuzzymatching: Option<bool>,
        includefield: Option<String>,
        context: &C) -> Result<StudiesStudyInstanceUIDSeriesGetResponse, ApiError>;

    /// Query for a series in a study.
    async fn studies_study_instance_uid_series_series_instance_uid_get(
        &self,
        study_instance_uid: String,
        series_instance_uid: String,
        context: &C) -> Result<StudiesStudyInstanceUIDSeriesSeriesInstanceUIDGetResponse, ApiError>;

    /// Query for instances in the series of a study.
    async fn studies_study_instance_uid_series_series_instance_uid_instances_get(
        &self,
        study_instance_uid: String,
        series_instance_uid: String,
        limit: Option<f64>,
        offset: Option<f64>,
        fuzzymatching: Option<bool>,
        includefield: Option<String>,
        context: &C) -> Result<StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesGetResponse, ApiError>;

    /// Query for a series in a study.
    async fn studies_study_instance_uid_series_series_instance_uid_instances_sop_instance_uid_frames_uid_get(
        &self,
        study_instance_uid: String,
        series_instance_uid: String,
        sop_instance_uid: String,
        uid: String,
        context: &C) -> Result<StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesSOPInstanceUIDFramesUidGetResponse, ApiError>;

    /// Query for a series in a study.
    async fn studies_study_instance_uid_series_series_instance_uid_instances_sop_instance_uid_get(
        &self,
        study_instance_uid: String,
        series_instance_uid: String,
        sop_instance_uid: String,
        context: &C) -> Result<StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesSOPInstanceUIDGetResponse, ApiError>;

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
        context: &C) -> Result<StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesSOPInstanceUIDRenderedGetResponse, ApiError>;

    /// Render a thumbnail.
    async fn studies_study_instance_uid_series_series_instance_uid_instances_sop_instance_uid_thumbnail_get(
        &self,
        study_instance_uid: String,
        series_instance_uid: String,
        sop_instance_uid: String,
        quality: Option<f32>,
        viewport: Option<&Vec<i32>>,
        context: &C) -> Result<StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesSOPInstanceUIDThumbnailGetResponse, ApiError>;

    /// Render a series.
    async fn studies_study_instance_uid_series_series_instance_uid_rendered_get(
        &self,
        study_instance_uid: String,
        series_instance_uid: String,
        annotation: Option<&Vec<String>>,
        quality: Option<f32>,
        viewport: Option<&Vec<i32>>,
        window: Option<&Vec<String>>,
        context: &C) -> Result<StudiesStudyInstanceUIDSeriesSeriesInstanceUIDRenderedGetResponse, ApiError>;

    /// Render a thumbnail.
    async fn studies_study_instance_uid_series_series_instance_uid_thumbnail_get(
        &self,
        study_instance_uid: String,
        series_instance_uid: String,
        quality: Option<f32>,
        viewport: Option<&Vec<i32>>,
        context: &C) -> Result<StudiesStudyInstanceUIDSeriesSeriesInstanceUIDThumbnailGetResponse, ApiError>;

    /// Render a thumbnail.
    async fn studies_study_instance_uid_thumbnail_get(
        &self,
        study_instance_uid: String,
        quality: Option<f32>,
        viewport: Option<&Vec<i32>>,
        context: &C) -> Result<StudiesStudyInstanceUIDThumbnailGetResponse, ApiError>;

}

/// API where `Context` isn't passed on every API call
#[async_trait]
pub trait ApiNoContext<C: Send + Sync> {

    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>>;

    fn context(&self) -> &C;

    /// Retrieve an instance field.
    async fn api_instances_tag(
        &self,
        sop_instance_uid: String,
        tag: String,
        ) -> Result<ApiInstancesTagResponse, ApiError>;

    /// Query for instances.
    async fn instances_get(
        &self,
        limit: Option<f64>,
        offset: Option<f64>,
        fuzzymatching: Option<bool>,
        includefield: Option<String>,
        ) -> Result<InstancesGetResponse, ApiError>;

    /// Query for a series in a study.
    async fn instances_sop_instance_uid_frames_uid_get(
        &self,
        sop_instance_uid: String,
        uid: String,
        ) -> Result<InstancesSOPInstanceUIDFramesUidGetResponse, ApiError>;

    /// Retrieve a instance.
    async fn instances_sop_instance_uid_get(
        &self,
        sop_instance_uid: String,
        ) -> Result<InstancesSOPInstanceUIDGetResponse, ApiError>;

    /// Render an instance.
    async fn instances_sop_instance_uid_rendered_get(
        &self,
        sop_instance_uid: String,
        annotation: Option<&Vec<String>>,
        quality: Option<f32>,
        viewport: Option<&Vec<i32>>,
        window: Option<&Vec<String>>,
        ) -> Result<InstancesSOPInstanceUIDRenderedGetResponse, ApiError>;

    /// Render a thumbnail.
    async fn instances_sop_instance_uid_thumbnail_get(
        &self,
        sop_instance_uid: String,
        quality: Option<f32>,
        viewport: Option<&Vec<i32>>,
        ) -> Result<InstancesSOPInstanceUIDThumbnailGetResponse, ApiError>;

    /// Query for series.
    async fn series_get(
        &self,
        limit: Option<f64>,
        offset: Option<f64>,
        fuzzymatching: Option<bool>,
        includefield: Option<String>,
        ) -> Result<SeriesGetResponse, ApiError>;

    /// Retrieve a series.
    async fn series_series_instance_uid_get(
        &self,
        series_instance_uid: String,
        ) -> Result<SeriesSeriesInstanceUIDGetResponse, ApiError>;

    /// Query for instances in the series of a study.
    async fn series_series_instance_uid_instances_get(
        &self,
        series_instance_uid: String,
        limit: Option<f64>,
        offset: Option<f64>,
        fuzzymatching: Option<bool>,
        includefield: Option<String>,
        ) -> Result<SeriesSeriesInstanceUIDInstancesGetResponse, ApiError>;

    /// Query for a series in a study.
    async fn series_series_instance_uid_instances_sop_instance_uid_frames_uid_get(
        &self,
        series_instance_uid: String,
        sop_instance_uid: String,
        uid: String,
        ) -> Result<SeriesSeriesInstanceUIDInstancesSOPInstanceUIDFramesUidGetResponse, ApiError>;

    /// Query for a series in a study.
    async fn series_series_instance_uid_instances_sop_instance_uid_get(
        &self,
        series_instance_uid: String,
        sop_instance_uid: String,
        ) -> Result<SeriesSeriesInstanceUIDInstancesSOPInstanceUIDGetResponse, ApiError>;

    /// Render an instance.
    async fn series_series_instance_uid_instances_sop_instance_uid_rendered_get(
        &self,
        series_instance_uid: String,
        sop_instance_uid: String,
        annotation: Option<&Vec<String>>,
        quality: Option<f32>,
        viewport: Option<&Vec<i32>>,
        window: Option<&Vec<String>>,
        ) -> Result<SeriesSeriesInstanceUIDInstancesSOPInstanceUIDRenderedGetResponse, ApiError>;

    /// Render a thumbnail.
    async fn series_series_instance_uid_instances_sop_instance_uid_thumbnail_get(
        &self,
        series_instance_uid: String,
        sop_instance_uid: String,
        quality: Option<f32>,
        viewport: Option<&Vec<i32>>,
        ) -> Result<SeriesSeriesInstanceUIDInstancesSOPInstanceUIDThumbnailGetResponse, ApiError>;

    /// Render an instance.
    async fn series_series_instance_uid_rendered_get(
        &self,
        series_instance_uid: String,
        annotation: Option<&Vec<String>>,
        quality: Option<f32>,
        viewport: Option<&Vec<i32>>,
        window: Option<&Vec<String>>,
        ) -> Result<SeriesSeriesInstanceUIDRenderedGetResponse, ApiError>;

    /// Render a thumbnail.
    async fn series_series_instance_uid_thumbnail_get(
        &self,
        series_instance_uid: String,
        quality: Option<f32>,
        viewport: Option<&Vec<i32>>,
        ) -> Result<SeriesSeriesInstanceUIDThumbnailGetResponse, ApiError>;

    /// Query for studies.
    async fn studies_get(
        &self,
        limit: Option<f64>,
        offset: Option<f64>,
        fuzzymatching: Option<bool>,
        includefield: Option<String>,
        ) -> Result<StudiesGetResponse, ApiError>;

    /// Retrieve a study.
    async fn studies_study_instance_uid_get(
        &self,
        study_instance_uid: String,
        ) -> Result<StudiesStudyInstanceUIDGetResponse, ApiError>;

    /// Query for series in a study.
    async fn studies_study_instance_uid_series_get(
        &self,
        study_instance_uid: String,
        limit: Option<f64>,
        offset: Option<f64>,
        fuzzymatching: Option<bool>,
        includefield: Option<String>,
        ) -> Result<StudiesStudyInstanceUIDSeriesGetResponse, ApiError>;

    /// Query for a series in a study.
    async fn studies_study_instance_uid_series_series_instance_uid_get(
        &self,
        study_instance_uid: String,
        series_instance_uid: String,
        ) -> Result<StudiesStudyInstanceUIDSeriesSeriesInstanceUIDGetResponse, ApiError>;

    /// Query for instances in the series of a study.
    async fn studies_study_instance_uid_series_series_instance_uid_instances_get(
        &self,
        study_instance_uid: String,
        series_instance_uid: String,
        limit: Option<f64>,
        offset: Option<f64>,
        fuzzymatching: Option<bool>,
        includefield: Option<String>,
        ) -> Result<StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesGetResponse, ApiError>;

    /// Query for a series in a study.
    async fn studies_study_instance_uid_series_series_instance_uid_instances_sop_instance_uid_frames_uid_get(
        &self,
        study_instance_uid: String,
        series_instance_uid: String,
        sop_instance_uid: String,
        uid: String,
        ) -> Result<StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesSOPInstanceUIDFramesUidGetResponse, ApiError>;

    /// Query for a series in a study.
    async fn studies_study_instance_uid_series_series_instance_uid_instances_sop_instance_uid_get(
        &self,
        study_instance_uid: String,
        series_instance_uid: String,
        sop_instance_uid: String,
        ) -> Result<StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesSOPInstanceUIDGetResponse, ApiError>;

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
        ) -> Result<StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesSOPInstanceUIDRenderedGetResponse, ApiError>;

    /// Render a thumbnail.
    async fn studies_study_instance_uid_series_series_instance_uid_instances_sop_instance_uid_thumbnail_get(
        &self,
        study_instance_uid: String,
        series_instance_uid: String,
        sop_instance_uid: String,
        quality: Option<f32>,
        viewport: Option<&Vec<i32>>,
        ) -> Result<StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesSOPInstanceUIDThumbnailGetResponse, ApiError>;

    /// Render a series.
    async fn studies_study_instance_uid_series_series_instance_uid_rendered_get(
        &self,
        study_instance_uid: String,
        series_instance_uid: String,
        annotation: Option<&Vec<String>>,
        quality: Option<f32>,
        viewport: Option<&Vec<i32>>,
        window: Option<&Vec<String>>,
        ) -> Result<StudiesStudyInstanceUIDSeriesSeriesInstanceUIDRenderedGetResponse, ApiError>;

    /// Render a thumbnail.
    async fn studies_study_instance_uid_series_series_instance_uid_thumbnail_get(
        &self,
        study_instance_uid: String,
        series_instance_uid: String,
        quality: Option<f32>,
        viewport: Option<&Vec<i32>>,
        ) -> Result<StudiesStudyInstanceUIDSeriesSeriesInstanceUIDThumbnailGetResponse, ApiError>;

    /// Render a thumbnail.
    async fn studies_study_instance_uid_thumbnail_get(
        &self,
        study_instance_uid: String,
        quality: Option<f32>,
        viewport: Option<&Vec<i32>>,
        ) -> Result<StudiesStudyInstanceUIDThumbnailGetResponse, ApiError>;

}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<C: Send + Sync> where Self: Sized
{
    /// Binds this API to a context.
    fn with_context(self: Self, context: C) -> ContextWrapper<Self, C>;
}

impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ContextWrapperExt<C> for T {
    fn with_context(self: T, context: C) -> ContextWrapper<T, C> {
         ContextWrapper::<T, C>::new(self, context)
    }
}

#[async_trait]
impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ApiNoContext<C> for ContextWrapper<T, C> {
    fn poll_ready(&self, cx: &mut Context) -> Poll<Result<(), ServiceError>> {
        self.api().poll_ready(cx)
    }

    fn context(&self) -> &C {
        ContextWrapper::context(self)
    }

    /// Retrieve an instance field.
    async fn api_instances_tag(
        &self,
        sop_instance_uid: String,
        tag: String,
        ) -> Result<ApiInstancesTagResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().api_instances_tag(sop_instance_uid, tag, &context).await
    }

    /// Query for instances.
    async fn instances_get(
        &self,
        limit: Option<f64>,
        offset: Option<f64>,
        fuzzymatching: Option<bool>,
        includefield: Option<String>,
        ) -> Result<InstancesGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().instances_get(limit, offset, fuzzymatching, includefield, &context).await
    }

    /// Query for a series in a study.
    async fn instances_sop_instance_uid_frames_uid_get(
        &self,
        sop_instance_uid: String,
        uid: String,
        ) -> Result<InstancesSOPInstanceUIDFramesUidGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().instances_sop_instance_uid_frames_uid_get(sop_instance_uid, uid, &context).await
    }

    /// Retrieve a instance.
    async fn instances_sop_instance_uid_get(
        &self,
        sop_instance_uid: String,
        ) -> Result<InstancesSOPInstanceUIDGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().instances_sop_instance_uid_get(sop_instance_uid, &context).await
    }

    /// Render an instance.
    async fn instances_sop_instance_uid_rendered_get(
        &self,
        sop_instance_uid: String,
        annotation: Option<&Vec<String>>,
        quality: Option<f32>,
        viewport: Option<&Vec<i32>>,
        window: Option<&Vec<String>>,
        ) -> Result<InstancesSOPInstanceUIDRenderedGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().instances_sop_instance_uid_rendered_get(sop_instance_uid, annotation, quality, viewport, window, &context).await
    }

    /// Render a thumbnail.
    async fn instances_sop_instance_uid_thumbnail_get(
        &self,
        sop_instance_uid: String,
        quality: Option<f32>,
        viewport: Option<&Vec<i32>>,
        ) -> Result<InstancesSOPInstanceUIDThumbnailGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().instances_sop_instance_uid_thumbnail_get(sop_instance_uid, quality, viewport, &context).await
    }

    /// Query for series.
    async fn series_get(
        &self,
        limit: Option<f64>,
        offset: Option<f64>,
        fuzzymatching: Option<bool>,
        includefield: Option<String>,
        ) -> Result<SeriesGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().series_get(limit, offset, fuzzymatching, includefield, &context).await
    }

    /// Retrieve a series.
    async fn series_series_instance_uid_get(
        &self,
        series_instance_uid: String,
        ) -> Result<SeriesSeriesInstanceUIDGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().series_series_instance_uid_get(series_instance_uid, &context).await
    }

    /// Query for instances in the series of a study.
    async fn series_series_instance_uid_instances_get(
        &self,
        series_instance_uid: String,
        limit: Option<f64>,
        offset: Option<f64>,
        fuzzymatching: Option<bool>,
        includefield: Option<String>,
        ) -> Result<SeriesSeriesInstanceUIDInstancesGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().series_series_instance_uid_instances_get(series_instance_uid, limit, offset, fuzzymatching, includefield, &context).await
    }

    /// Query for a series in a study.
    async fn series_series_instance_uid_instances_sop_instance_uid_frames_uid_get(
        &self,
        series_instance_uid: String,
        sop_instance_uid: String,
        uid: String,
        ) -> Result<SeriesSeriesInstanceUIDInstancesSOPInstanceUIDFramesUidGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().series_series_instance_uid_instances_sop_instance_uid_frames_uid_get(series_instance_uid, sop_instance_uid, uid, &context).await
    }

    /// Query for a series in a study.
    async fn series_series_instance_uid_instances_sop_instance_uid_get(
        &self,
        series_instance_uid: String,
        sop_instance_uid: String,
        ) -> Result<SeriesSeriesInstanceUIDInstancesSOPInstanceUIDGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().series_series_instance_uid_instances_sop_instance_uid_get(series_instance_uid, sop_instance_uid, &context).await
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
        ) -> Result<SeriesSeriesInstanceUIDInstancesSOPInstanceUIDRenderedGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().series_series_instance_uid_instances_sop_instance_uid_rendered_get(series_instance_uid, sop_instance_uid, annotation, quality, viewport, window, &context).await
    }

    /// Render a thumbnail.
    async fn series_series_instance_uid_instances_sop_instance_uid_thumbnail_get(
        &self,
        series_instance_uid: String,
        sop_instance_uid: String,
        quality: Option<f32>,
        viewport: Option<&Vec<i32>>,
        ) -> Result<SeriesSeriesInstanceUIDInstancesSOPInstanceUIDThumbnailGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().series_series_instance_uid_instances_sop_instance_uid_thumbnail_get(series_instance_uid, sop_instance_uid, quality, viewport, &context).await
    }

    /// Render an instance.
    async fn series_series_instance_uid_rendered_get(
        &self,
        series_instance_uid: String,
        annotation: Option<&Vec<String>>,
        quality: Option<f32>,
        viewport: Option<&Vec<i32>>,
        window: Option<&Vec<String>>,
        ) -> Result<SeriesSeriesInstanceUIDRenderedGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().series_series_instance_uid_rendered_get(series_instance_uid, annotation, quality, viewport, window, &context).await
    }

    /// Render a thumbnail.
    async fn series_series_instance_uid_thumbnail_get(
        &self,
        series_instance_uid: String,
        quality: Option<f32>,
        viewport: Option<&Vec<i32>>,
        ) -> Result<SeriesSeriesInstanceUIDThumbnailGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().series_series_instance_uid_thumbnail_get(series_instance_uid, quality, viewport, &context).await
    }

    /// Query for studies.
    async fn studies_get(
        &self,
        limit: Option<f64>,
        offset: Option<f64>,
        fuzzymatching: Option<bool>,
        includefield: Option<String>,
        ) -> Result<StudiesGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().studies_get(limit, offset, fuzzymatching, includefield, &context).await
    }

    /// Retrieve a study.
    async fn studies_study_instance_uid_get(
        &self,
        study_instance_uid: String,
        ) -> Result<StudiesStudyInstanceUIDGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().studies_study_instance_uid_get(study_instance_uid, &context).await
    }

    /// Query for series in a study.
    async fn studies_study_instance_uid_series_get(
        &self,
        study_instance_uid: String,
        limit: Option<f64>,
        offset: Option<f64>,
        fuzzymatching: Option<bool>,
        includefield: Option<String>,
        ) -> Result<StudiesStudyInstanceUIDSeriesGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().studies_study_instance_uid_series_get(study_instance_uid, limit, offset, fuzzymatching, includefield, &context).await
    }

    /// Query for a series in a study.
    async fn studies_study_instance_uid_series_series_instance_uid_get(
        &self,
        study_instance_uid: String,
        series_instance_uid: String,
        ) -> Result<StudiesStudyInstanceUIDSeriesSeriesInstanceUIDGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().studies_study_instance_uid_series_series_instance_uid_get(study_instance_uid, series_instance_uid, &context).await
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
        ) -> Result<StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().studies_study_instance_uid_series_series_instance_uid_instances_get(study_instance_uid, series_instance_uid, limit, offset, fuzzymatching, includefield, &context).await
    }

    /// Query for a series in a study.
    async fn studies_study_instance_uid_series_series_instance_uid_instances_sop_instance_uid_frames_uid_get(
        &self,
        study_instance_uid: String,
        series_instance_uid: String,
        sop_instance_uid: String,
        uid: String,
        ) -> Result<StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesSOPInstanceUIDFramesUidGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().studies_study_instance_uid_series_series_instance_uid_instances_sop_instance_uid_frames_uid_get(study_instance_uid, series_instance_uid, sop_instance_uid, uid, &context).await
    }

    /// Query for a series in a study.
    async fn studies_study_instance_uid_series_series_instance_uid_instances_sop_instance_uid_get(
        &self,
        study_instance_uid: String,
        series_instance_uid: String,
        sop_instance_uid: String,
        ) -> Result<StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesSOPInstanceUIDGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().studies_study_instance_uid_series_series_instance_uid_instances_sop_instance_uid_get(study_instance_uid, series_instance_uid, sop_instance_uid, &context).await
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
        ) -> Result<StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesSOPInstanceUIDRenderedGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().studies_study_instance_uid_series_series_instance_uid_instances_sop_instance_uid_rendered_get(study_instance_uid, series_instance_uid, sop_instance_uid, annotation, quality, viewport, window, &context).await
    }

    /// Render a thumbnail.
    async fn studies_study_instance_uid_series_series_instance_uid_instances_sop_instance_uid_thumbnail_get(
        &self,
        study_instance_uid: String,
        series_instance_uid: String,
        sop_instance_uid: String,
        quality: Option<f32>,
        viewport: Option<&Vec<i32>>,
        ) -> Result<StudiesStudyInstanceUIDSeriesSeriesInstanceUIDInstancesSOPInstanceUIDThumbnailGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().studies_study_instance_uid_series_series_instance_uid_instances_sop_instance_uid_thumbnail_get(study_instance_uid, series_instance_uid, sop_instance_uid, quality, viewport, &context).await
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
        ) -> Result<StudiesStudyInstanceUIDSeriesSeriesInstanceUIDRenderedGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().studies_study_instance_uid_series_series_instance_uid_rendered_get(study_instance_uid, series_instance_uid, annotation, quality, viewport, window, &context).await
    }

    /// Render a thumbnail.
    async fn studies_study_instance_uid_series_series_instance_uid_thumbnail_get(
        &self,
        study_instance_uid: String,
        series_instance_uid: String,
        quality: Option<f32>,
        viewport: Option<&Vec<i32>>,
        ) -> Result<StudiesStudyInstanceUIDSeriesSeriesInstanceUIDThumbnailGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().studies_study_instance_uid_series_series_instance_uid_thumbnail_get(study_instance_uid, series_instance_uid, quality, viewport, &context).await
    }

    /// Render a thumbnail.
    async fn studies_study_instance_uid_thumbnail_get(
        &self,
        study_instance_uid: String,
        quality: Option<f32>,
        viewport: Option<&Vec<i32>>,
        ) -> Result<StudiesStudyInstanceUIDThumbnailGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().studies_study_instance_uid_thumbnail_get(study_instance_uid, quality, viewport, &context).await
    }

}


#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use client::Client;

#[cfg(feature = "server")]
pub mod server;

// Re-export router() as a top-level name
#[cfg(feature = "server")]
pub use self::server::Service;

#[cfg(feature = "server")]
pub mod context;

pub mod models;

#[cfg(any(feature = "client", feature = "server"))]
pub(crate) mod header;
