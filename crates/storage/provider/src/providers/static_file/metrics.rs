use std::time::Duration;

use metrics::{Counter, Gauge, Histogram};
use reth_metrics::Metrics;
use reth_primitives::StaticFileSegment;
use strum::EnumIter;

/// Metrics for the static file provider.
#[derive(Debug, Default)]
pub struct StaticFileProviderMetrics {
    segments: StaticFileSegmentMetrics,
    segment_operations: StaticFileProviderOperationMetrics,
}

impl StaticFileProviderMetrics {
    pub(crate) fn record_segment(
        &self,
        segment: StaticFileSegment,
        size: u64,
        files: usize,
        entries: usize,
    ) {
        match segment {
            StaticFileSegment::Headers => {
                self.segments.headers_size.set(size as f64);
                self.segments.headers_files.set(files as f64);
                self.segments.headers_entries.set(entries as f64);
            }
            StaticFileSegment::Transactions => {
                self.segments.transactions_size.set(size as f64);
                self.segments.transactions_files.set(files as f64);
                self.segments.transactions_entries.set(entries as f64);
            }
            StaticFileSegment::Receipts => {
                self.segments.receipts_size.set(size as f64);
                self.segments.receipts_files.set(files as f64);
                self.segments.receipts_entries.set(entries as f64);
            }
        }
    }

    pub(crate) fn record_segment_operation(
        &self,
        segment: StaticFileSegment,
        operation: StaticFileProviderOperation,
        duration: Option<Duration>,
    ) {
        macro_rules! record_operation {
            ($self:ident, $counter:ident, $histogram:ident, $duration:expr) => {
                $self.segment_operations.$counter.increment(1);
                if let Some(duration) = $duration {
                    $self.segment_operations.$histogram.record(duration.as_secs_f64());
                }
            };
        }

        match (segment, operation) {
            (StaticFileSegment::Headers, StaticFileProviderOperation::InitCursor) => {
                record_operation!(
                    self,
                    headers_init_cursor_calls_total,
                    headers_init_cursor_duration_seconds,
                    duration
                );
            }
            (StaticFileSegment::Headers, StaticFileProviderOperation::OpenWriter) => {
                record_operation!(
                    self,
                    headers_open_writer_calls_total,
                    headers_open_writer_duration_seconds,
                    duration
                );
            }
            (StaticFileSegment::Headers, StaticFileProviderOperation::Append) => {
                record_operation!(
                    self,
                    headers_append_calls_total,
                    headers_append_duration_seconds,
                    duration
                );
            }
            (StaticFileSegment::Headers, StaticFileProviderOperation::Prune) => {
                record_operation!(
                    self,
                    headers_prune_calls_total,
                    headers_prune_duration_seconds,
                    duration
                );
            }
            (StaticFileSegment::Headers, StaticFileProviderOperation::IncrementBlock) => {
                record_operation!(
                    self,
                    headers_increment_block_calls_total,
                    headers_increment_block_duration_seconds,
                    duration
                );
            }
            (StaticFileSegment::Headers, StaticFileProviderOperation::CommitWriter) => {
                record_operation!(
                    self,
                    headers_commit_writer_calls_total,
                    headers_commit_writer_duration_seconds,
                    duration
                );
            }
            (StaticFileSegment::Transactions, StaticFileProviderOperation::InitCursor) => {
                record_operation!(
                    self,
                    transactions_init_cursor_calls_total,
                    transactions_init_cursor_duration_seconds,
                    duration
                );
            }
            (StaticFileSegment::Transactions, StaticFileProviderOperation::OpenWriter) => {
                record_operation!(
                    self,
                    transactions_open_writer_calls_total,
                    transactions_open_writer_duration_seconds,
                    duration
                );
            }
            (StaticFileSegment::Transactions, StaticFileProviderOperation::Append) => {
                record_operation!(
                    self,
                    transactions_append_calls_total,
                    transactions_append_duration_seconds,
                    duration
                );
            }
            (StaticFileSegment::Transactions, StaticFileProviderOperation::Prune) => {
                record_operation!(
                    self,
                    transactions_prune_calls_total,
                    transactions_prune_duration_seconds,
                    duration
                );
            }
            (StaticFileSegment::Transactions, StaticFileProviderOperation::IncrementBlock) => {
                record_operation!(
                    self,
                    transactions_increment_block_calls_total,
                    transactions_increment_block_duration_seconds,
                    duration
                );
            }
            (StaticFileSegment::Transactions, StaticFileProviderOperation::CommitWriter) => {
                record_operation!(
                    self,
                    transactions_commit_writer_calls_total,
                    transactions_commit_writer_duration_seconds,
                    duration
                );
            }
            (StaticFileSegment::Receipts, StaticFileProviderOperation::InitCursor) => {
                record_operation!(
                    self,
                    receipts_init_cursor_calls_total,
                    receipts_init_cursor_duration_seconds,
                    duration
                );
            }
            (StaticFileSegment::Receipts, StaticFileProviderOperation::OpenWriter) => {
                record_operation!(
                    self,
                    receipts_open_writer_calls_total,
                    receipts_open_writer_duration_seconds,
                    duration
                );
            }
            (StaticFileSegment::Receipts, StaticFileProviderOperation::Append) => {
                record_operation!(
                    self,
                    receipts_append_calls_total,
                    receipts_append_duration_seconds,
                    duration
                );
            }
            (StaticFileSegment::Receipts, StaticFileProviderOperation::Prune) => {
                record_operation!(
                    self,
                    receipts_prune_calls_total,
                    receipts_prune_duration_seconds,
                    duration
                );
            }
            (StaticFileSegment::Receipts, StaticFileProviderOperation::IncrementBlock) => {
                record_operation!(
                    self,
                    receipts_increment_block_calls_total,
                    receipts_increment_block_duration_seconds,
                    duration
                );
            }
            (StaticFileSegment::Receipts, StaticFileProviderOperation::CommitWriter) => {
                record_operation!(
                    self,
                    receipts_commit_writer_calls_total,
                    receipts_commit_writer_duration_seconds,
                    duration
                );
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub(crate) enum StaticFileProviderOperation {
    InitCursor,
    OpenWriter,
    Append,
    Prune,
    IncrementBlock,
    CommitWriter,
}

/// Metrics for a specific static file segment.
#[derive(Metrics)]
#[metrics(scope = "static_files.segment")]
pub(crate) struct StaticFileSegmentMetrics {
    /// The size of a header static file segment.
    headers_size: Gauge,
    /// The size of a transaction static file segment.
    transactions_size: Gauge,
    /// The size of a receipt static file segment.
    receipts_size: Gauge,
    /// The number of files for a header static file segment.
    headers_files: Gauge,
    /// The number of files for a transaction static file segment.
    transactions_files: Gauge,
    /// The number of files for a receipt static file segment.
    receipts_files: Gauge,
    /// The number of entries for a receipt static file segment.
    headers_entries: Gauge,
    /// The number of entries for a transaction static file segment.
    transactions_entries: Gauge,
    /// The number of entries for a header static file segment.
    receipts_entries: Gauge,
}

#[derive(Metrics)]
#[metrics(scope = "static_files.jar_provider")]
pub(crate) struct StaticFileProviderOperationMetrics {
    /// Total number of calls to the init cursor operation on headers static file segment.
    headers_init_cursor_calls_total: Counter,
    /// Total number of calls to the open writer operation on headers static file segment.
    headers_open_writer_calls_total: Counter,
    /// Total number of calls to the append operation on headers static file segment.
    headers_append_calls_total: Counter,
    /// Total number of calls to the prune operation on headers static file segment.
    headers_prune_calls_total: Counter,
    /// Total number of calls to the increment block operation on headers static file segment.
    headers_increment_block_calls_total: Counter,
    /// Total number of calls to the commit writer operation on headers static file segment.
    headers_commit_writer_calls_total: Counter,
    /// Total number of calls to the init cursor operation on transactions static file segment.
    transactions_init_cursor_calls_total: Counter,
    /// Total number of calls to the open writer operation on transactions static file segment.
    transactions_open_writer_calls_total: Counter,
    /// Total number of calls to the append operation on transactions static file segment.
    transactions_append_calls_total: Counter,
    /// Total number of calls to the prune operation on transactions static file segment.
    transactions_prune_calls_total: Counter,
    /// Total number of calls to the increment block operation on transactions static file segment.
    transactions_increment_block_calls_total: Counter,
    /// Total number of calls to the commit writer operation on transactions static file segment.
    transactions_commit_writer_calls_total: Counter,
    /// Total number of calls to the init cursor operation on receipts static file segment.
    receipts_init_cursor_calls_total: Counter,
    /// Total number of calls to the open writer operation on receipts static file segment.
    receipts_open_writer_calls_total: Counter,
    /// Total number of calls to the append operation on receipts static file segment.
    receipts_append_calls_total: Counter,
    /// Total number of calls to the prune operation on receipts static file segment.
    receipts_prune_calls_total: Counter,
    /// Total number of calls to the increment block operation on receipts static file segment.
    receipts_increment_block_calls_total: Counter,
    /// Total number of calls to the commit writer operation on receipts static file segment.
    receipts_commit_writer_calls_total: Counter,
    /// The time it took to execute the headers static file jar provider operation that initializes
    /// a cursor.
    headers_init_cursor_duration_seconds: Histogram,
    /// The time it took to execute the headers static file jar provider operation that opens a
    /// writer.
    headers_open_writer_duration_seconds: Histogram,
    /// The time it took to execute the headers static file jar provider operation that appends
    /// data.
    headers_append_duration_seconds: Histogram,
    /// The time it took to execute the headers static file jar provider operation that prunes
    /// data.
    headers_prune_duration_seconds: Histogram,
    /// The time it took to execute the headers static file jar provider operation that increments
    /// the block.
    headers_increment_block_duration_seconds: Histogram,
    /// The time it took to execute the headers static file jar provider operation that commits
    /// the writer.
    headers_commit_writer_duration_seconds: Histogram,
    /// The time it took to execute the transactions static file jar provider operation that
    /// initializes a cursor.
    transactions_init_cursor_duration_seconds: Histogram,
    /// The time it took to execute the transactions static file jar provider operation that opens
    /// a writer.
    transactions_open_writer_duration_seconds: Histogram,
    /// The time it took to execute the transactions static file jar provider operation that
    /// appends data.
    transactions_append_duration_seconds: Histogram,
    /// The time it took to execute the transactions static file jar provider operation that prunes
    /// data.
    transactions_prune_duration_seconds: Histogram,
    /// The time it took to execute the transactions static file jar provider operation that
    /// increments the block.
    transactions_increment_block_duration_seconds: Histogram,
    /// The time it took to execute the transactions static file jar provider operation that
    /// commits the writer.
    transactions_commit_writer_duration_seconds: Histogram,
    /// The time it took to execute the receipts static file jar provider operation that
    /// initializes a cursor.
    receipts_init_cursor_duration_seconds: Histogram,
    /// The time it took to execute the receipts static file jar provider operation that opens a
    /// writer.
    receipts_open_writer_duration_seconds: Histogram,
    /// The time it took to execute the receipts static file jar provider operation that appends
    /// data.
    receipts_append_duration_seconds: Histogram,
    /// The time it took to execute the receipts static file jar provider operation that prunes
    /// data.
    receipts_prune_duration_seconds: Histogram,
    /// The time it took to execute the receipts static file jar provider operation that increments
    /// the block.
    receipts_increment_block_duration_seconds: Histogram,
    /// The time it took to execute the receipts static file jar provider operation that commits
    /// the writer.
    receipts_commit_writer_duration_seconds: Histogram,
}
