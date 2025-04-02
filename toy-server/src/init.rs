use crate::config;
use crate::config::{Config, GLOBAL_CONFIG};
use arc_swap::access::Access;
use fastrace_opentelemetry::OpenTelemetryReporter;
use log::LevelFilter;
use logforth::append;
use logforth::append::rolling_file::{RollingFileWriter, Rotation};
use logforth::append::{rolling_file, RollingFile};
use logforth::diagnostic::FastraceDiagnostic;
use logforth::layout::TextLayout;
use logforth::non_blocking::WorkerGuard;
use opentelemetry::trace::SpanKind;
use opentelemetry::InstrumentationScope;
use opentelemetry_otlp::{Protocol, SpanExporter, WithExportConfig, WithTonicConfig};
use opentelemetry_sdk::Resource;
use std::borrow::Cow;
use std::str::FromStr;
use std::time::Duration;
use tonic::metadata::MetadataMap;

const SERVICE_NAME: &str = env!("CARGO_PKG_NAME");
pub(crate) fn init_log() -> WorkerGuard {
    //加载配置
    config::reload();
    let cfg = GLOBAL_CONFIG
        .get()
        .unwrap()
        .map(|cfg: &Config| &cfg.log)
        .load();

    let rolling_writer = RollingFileWriter::builder()
        .rotation(Rotation::Daily)
        .filename_prefix(SERVICE_NAME)
        .filename_suffix("log")
        .max_file_size(128 * 1024 * 1024)
        .build(&cfg.directory)
        .unwrap();

    let (non_blocking, guard) = rolling_file::non_blocking(rolling_writer)
        .shutdown_timeout(Duration::from_secs(10))
        .finish();

    let level = LevelFilter::from_str(&cfg.level).unwrap();
    logforth::builder()
        .dispatch(|d| {
            d.filter(level)
                .diagnostic(FastraceDiagnostic::default())
                .append(
                    RollingFile::new(non_blocking).with_layout(TextLayout::default().no_color()),
                )
        })
        .dispatch(|d| {
            d.filter(LevelFilter::Info)
                .append(append::FastraceEvent::default())
        })
        .apply();

    guard
}

pub(crate) fn init_trace() {
    let mut metadata = MetadataMap::with_capacity(3);
    metadata.insert(
        "authorization",
        "Basic cm9vdEBleGFtcGxlLmNvbTpEbDF4RFAwWGlzanR0UEZa"
            .parse()
            .unwrap(),
    );
    metadata.insert("organization", "default".parse().unwrap());
    metadata.insert("stream-name", SERVICE_NAME.parse().unwrap());

    let reporter = OpenTelemetryReporter::new(
        SpanExporter::builder()
            .with_tonic()
            .with_endpoint("http://localhost:5081")
            .with_metadata(metadata)
            .with_protocol(Protocol::Grpc)
            .build()
            .expect("initialize oltp exporter"),
        SpanKind::Server,
        Cow::Owned(Resource::builder().with_service_name(SERVICE_NAME).build()),
        InstrumentationScope::builder(SERVICE_NAME)
            .with_version(env!("CARGO_PKG_VERSION"))
            .build(),
    );

    fastrace::set_reporter(
        reporter,
        fastrace::collector::Config::default().report_interval(Duration::ZERO),
    );
}
