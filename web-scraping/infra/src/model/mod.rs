/// クエリ結果を格納する際のDTOを記述する
mod raw_room;
mod room_header;
mod room_header_summary;

pub use raw_room::RawRoomRecord;
pub use room_header::RoomHeaderRecord;
pub use room_header_summary::{RoomHeaderSummaryRecord, RoomHeaderSummaryTable};
