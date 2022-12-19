/// クエリ結果を格納する際のDTOを記述する
mod room_header;
mod room_header_summary;

pub use room_header::RoomHeaderRecord;
pub use room_header_summary::{RoomHeaderSummaryRecord, RoomHeaderSummaryTable};
