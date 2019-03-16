use std::borrow::Cow;
pub type StrCow = Cow<'static, str>;

/// List of category tags to apply to the event. Category tags can be used to hide events
/// in the Trace Viewer UI.
#[derive(Clone, Debug)]
pub enum CategoriesT {
    StaticArray(&'static [&'static str]),
    DynamicArray(Vec<String>),
}

/// Enumerates sample event types.
///
/// ## Duration Events
/// Duration events provide a way to mark a duration of work on a given thread. A duration is
/// marked by `DurationBegin` and `DurationEnd` events. A `DurationBegin` event must come
/// before the corresponding `DurationEnd` event. `DurationBegin`/`DurationEnd` events can be
/// nested within other `DurationBegin`/`DurationEnd` events allowing for the capture of
/// function calling behavior. Timestamps for duration events must be in increasing order
/// for a given thread. Timestamps in different threads do not have to be in increasing order.
/// The only required fields for `DurationEnd` events are the `pid` , `tid`, `SampleEventType` and
/// `timestamp` fields, all others are optional.
///
/// If you provide args to both corresponding `DurationBegin` and `DurationEnd` events then
/// the arguments will be merged. If there is a duplicate argument value provided the
/// `DurationEnd` event argument will be taken and the `DurationBegin` event argument will be
/// discarded.
///
/// If you need to have durations that do not nest properly you should use `AsyncStart`,
/// `AsyncInstant`, and `AsyncEnd` events instead.
///
/// # Complete Duration
/// A `CompleteDuration` event logically combines a `DurationBegin` and `DurationEnd` event. Such
/// an event should not have a `None` value for the `duration_us` field of a `Sample`, since this
/// field stores the duration of a complete event.
///
/// ## Instant Events
/// Instant events, i.e. events without duration information, are represented with `Sample`s having
/// `SampleEventType` value `Instant`.
///
/// ## Asynchronous Events
/// Asynchronous operations can be traced using `Sample`s with `SampleEventType` values:
/// `AsyncStart`, `AsyncInstant`, and `AsyncEnd`. Async events should not have a `None` value
/// stored in the `thread_name` field of a `Sample`, as this name distinguishes the source of the
/// asynchronous event.
///
/// Asynchronous events corresponding to each particular `thread_name` should be traced by a pair
/// of `AsyncStart`/`AsyncEnd` events, or be an `AsyncInstant` event, if no duration is associated
/// with it. Events corresponding to a particular `thread_name` may be nested, and will be
/// distinguished by the `name` field of a `Sample`.
///
/// ## Flow Events
/// A flow event is a similar in concept to an asynchronous event, but in the Trace Viewer UI,
/// links will be drawn between the starts and ends of flow, instead of the usual bar.
///
/// ## Object Events
///
/// `ObjectCreated`, `ObjectSnapshot`, and `ObjectDestroyed` event types allow for the tracing of
/// data structure lifetimes. `ObjectCreated` and `ObjectDestroyed` must come in pairs, while
/// `ObjectSnapshot` is meant to transmit information regarding the state of an object at a given
/// timepoint.
///
/// `ObjectCreated` and `ObjectDestroyed` events must have a None value for their `Sample`'s `args`
/// field. This is because it is rare that data related to an object remains constant during the
/// execution of a program. `ObjectSnapshot` events should be used to convey information regarding
/// an object: for example, its data, by associating it with a `snapshot` keyword under the `args`
/// data structure.
///
/// ## Metadata
///
/// Used to associate metadata with events. More details on pg.12 of the Trace Viewer format
/// document.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SampleEventType {
    DurationBegin,
    DurationEnd,
    CompleteDuration,
    Instant,
    AsyncStart,
    AsyncInstant,
    AsyncEnd,
    FlowStart,
    FlowInstant,
    FlowEnd,
    ObjectCreated,
    ObjectSnapshot,
    ObjectDestroyed,
    Metadata,
}

/// For more details see pg. 4, top list in
/// [Trace Event Format](https://docs.google.com/document/d/1CvAClvFfyA5R-PhYUmn5OOQtYMH4h6I0nSsKchNAySU) document.
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
