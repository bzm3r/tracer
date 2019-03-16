use std::borrow::Cow;
pub type StrCow = Cow<'static, str>;

/// For more details see pg. 4, top list in [Trace Event Format](https://docs.google.com/document/d/1CvAClvFfyA5R-PhYUmn5OOQtYMH4h6I0nSsKchNAySU) doc.
pub struct Sample {
    /// The name of the event to be shown.
    pub name: StrCow,
    /// List of categories the event applies to.
    pub categories: Option<CategoriesT>,
    /// When was the sample started.
    pub timestamp_us: u64,
    /// What kind of sample this is.
    pub event_type: SampleEventType,
    pub duration_us: Option<u64>,
    /// The process the sample was captured in.
    pub pid: u64,
    /// The thread the sample was captured on.  Omitted for Metadata events that
    /// want to set the process name (if provided then sets the thread name).
    pub tid: u64,
    pub thread_name: Option<StrCow>,
    pub args: Option<SampleArgs>,
}
