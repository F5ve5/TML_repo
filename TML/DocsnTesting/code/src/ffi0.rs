use windows::Win32::System::Diagnostics::Etw;
use windows::core::PWSTR;

fn main() {
    unsafe {
        extern "system" fn callback(event_record: *mut EVENT_RECORD) {
            // callback body
        }

        let mut logfile = EVENT_TRACE_LOGFILEW::default();

        logfile.EventRecordCallback = Some(callback);
    }
}