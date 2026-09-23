mod support;

use compression::{
    ArchiveFlags, ArchiveStream, ByteStream, CustomArchiveStreamCallbacks,
    CustomByteStreamCallbacks, EntryMessage, EntryMessageEvent, FieldKey, FieldKeySet, Header,
    PathList, DEFAULT_FILE_MODE, OPEN_CREATE, OPEN_READ_ONLY, OPEN_TRUNCATE, OPEN_WRITE_ONLY,
};
use std::collections::VecDeque;
use std::fs;
use std::sync::{Arc, Mutex};
use support::{artifact_dir, path_string};

#[derive(Debug, Default)]
struct MemoryByteState {
    data: Vec<u8>,
    position: usize,
    cancelled: bool,
    closed: bool,
}

#[derive(Clone, Debug)]
struct SharedMemoryByteStream {
    inner: Arc<Mutex<MemoryByteState>>,
}

impl CustomByteStreamCallbacks for SharedMemoryByteStream {
    fn write(&self, buffer: &[u8]) -> compression::Result<usize> {
        let mut state = self.inner.lock().unwrap();
        let start = state.position;
        let end = start + buffer.len();
        if end > state.data.len() {
            state.data.resize(end, 0);
        }
        state.data[start..end].copy_from_slice(buffer);
        state.position = end;
        drop(state);
        Ok(buffer.len())
    }

    fn pwrite(&self, buffer: &[u8], offset: i64) -> compression::Result<usize> {
        let mut state = self.inner.lock().unwrap();
        let start = usize::try_from(offset).unwrap_or(0);
        let end = start + buffer.len();
        if end > state.data.len() {
            state.data.resize(end, 0);
        }
        state.data[start..end].copy_from_slice(buffer);
        drop(state);
        Ok(buffer.len())
    }

    fn read(&self, buffer: &mut [u8]) -> compression::Result<usize> {
        let mut state = self.inner.lock().unwrap();
        let available = state.data.len().saturating_sub(state.position);
        let count = available.min(buffer.len());
        buffer[..count].copy_from_slice(&state.data[state.position..state.position + count]);
        state.position += count;
        drop(state);
        Ok(count)
    }

    fn pread(&self, buffer: &mut [u8], offset: i64) -> compression::Result<usize> {
        let state = self.inner.lock().unwrap();
        let start = usize::try_from(offset).unwrap_or(0);
        let available = state.data.len().saturating_sub(start);
        let count = available.min(buffer.len());
        buffer[..count].copy_from_slice(&state.data[start..start + count]);
        drop(state);
        Ok(count)
    }

    fn seek(&self, offset: i64, whence: i32) -> compression::Result<i64> {
        let mut state = self.inner.lock().unwrap();
        let base = match whence {
            0 => 0_i64,
            1 => i64::try_from(state.position).unwrap_or(i64::MAX),
            2 => i64::try_from(state.data.len()).unwrap_or(i64::MAX),
            _ => return Ok(-1),
        };
        let next = base + offset;
        state.position = usize::try_from(next).unwrap_or(0);
        drop(state);
        Ok(next)
    }

    fn cancel(&self) {
        self.inner.lock().unwrap().cancelled = true;
    }

    fn close(&mut self) -> compression::Result<()> {
        self.inner.lock().unwrap().closed = true;
        Ok(())
    }
}

#[derive(Debug, Default)]
struct MemoryArchiveState {
    written_headers: Vec<Vec<u8>>,
    written_blobs: Vec<(FieldKey, Vec<u8>)>,
    read_headers: VecDeque<Vec<u8>>,
    read_blobs: VecDeque<(FieldKey, Vec<u8>)>,
    cancelled: bool,
    closed: bool,
}

#[derive(Clone, Debug)]
struct SharedMemoryArchiveStream {
    inner: Arc<Mutex<MemoryArchiveState>>,
}

impl CustomArchiveStreamCallbacks for SharedMemoryArchiveStream {
    fn write_header(&self, header: &Header) -> compression::Result<()> {
        self.inner
            .lock()
            .unwrap()
            .written_headers
            .push(header.encoded_data()?);
        Ok(())
    }

    fn write_blob(&self, key: FieldKey, buffer: &[u8]) -> compression::Result<()> {
        self.inner
            .lock()
            .unwrap()
            .written_blobs
            .push((key, buffer.to_vec()));
        Ok(())
    }

    fn read_header(&self) -> compression::Result<Option<Header>> {
        let Some(data) = self.inner.lock().unwrap().read_headers.pop_front() else {
            return Ok(None);
        };
        Ok(Some(Header::from_encoded_data(&data)?))
    }

    fn read_blob(&self, key: FieldKey, buffer: &mut [u8]) -> compression::Result<()> {
        let (expected_key, data) = self
            .inner
            .lock()
            .unwrap()
            .read_blobs
            .pop_front()
            .expect("blob queued");
        assert_eq!(expected_key, key);
        buffer.copy_from_slice(&data);
        Ok(())
    }

    fn cancel(&self) {
        self.inner.lock().unwrap().cancelled = true;
    }

    fn close(&mut self) -> compression::Result<()> {
        self.inner.lock().unwrap().closed = true;
        Ok(())
    }
}

#[test]
#[allow(deprecated)]
fn custom_byte_stream_supports_callbacks() -> Result<(), Box<dyn std::error::Error>> {
    let state = Arc::new(Mutex::new(MemoryByteState::default()));
    let mut stream = ByteStream::custom(SharedMemoryByteStream {
        inner: state.clone(),
    })?;
    stream.write_all(b"hello")?;
    stream.pwrite(b" world", 5)?;
    assert_eq!(stream.seek(0, 0)?, 0);
    let mut buffer = vec![0_u8; 11];
    assert_eq!(stream.read(&mut buffer)?, 11);
    assert_eq!(&buffer, b"hello world");
    let mut slice = vec![0_u8; 5];
    assert_eq!(stream.pread(&mut slice, 6)?, 5);
    assert_eq!(&slice, b"world");
    stream.abort()?;
    stream.close()?;

    let state = state.lock().unwrap();
    assert!(state.cancelled);
    assert!(state.closed);
    assert_eq!(state.data, b"hello world");
    Ok(())
}

#[test]
#[allow(deprecated, clippy::too_many_lines)]
fn custom_archive_stream_and_message_callbacks_work() -> Result<(), Box<dyn std::error::Error>> {
    let payload = b"custom archive payload".to_vec();
    let mut header = Header::new()?;
    header.append_field_uint(FieldKey::TYP, u64::from(b'F'))?;
    header.append_field_string(FieldKey::PAT, "custom.txt")?;
    header.append_field_uint(FieldKey::SIZ, payload.len() as u64)?;
    header.append_field_blob(FieldKey::DAT, payload.len() as u64)?;

    let writer_state = Arc::new(Mutex::new(MemoryArchiveState::default()));
    let mut writer = ArchiveStream::custom(SharedMemoryArchiveStream {
        inner: writer_state.clone(),
    })?;
    writer.write_header(&header)?;
    writer.write_blob(FieldKey::DAT, &payload)?;
    writer.close()?;

    let written_header = writer_state.lock().unwrap().written_headers[0].clone();
    let written_blob = writer_state.lock().unwrap().written_blobs[0].clone();
    assert_eq!(written_blob.0, FieldKey::DAT);
    assert_eq!(written_blob.1, payload);

    let reader_state = Arc::new(Mutex::new(MemoryArchiveState {
        read_headers: VecDeque::from([written_header]),
        read_blobs: VecDeque::from([written_blob]),
        ..MemoryArchiveState::default()
    }));
    let mut reader = ArchiveStream::custom(SharedMemoryArchiveStream {
        inner: reader_state.clone(),
    })?;
    let decoded_header = reader.read_header()?.expect("header");
    assert_eq!(decoded_header.path()?.as_deref(), Some("custom.txt"));
    let mut decoded_blob = vec![0_u8; payload.len()];
    reader.read_blob(FieldKey::DAT, &mut decoded_blob)?;
    assert_eq!(decoded_blob, payload);
    assert!(reader.read_header()?.is_none());
    reader.abort()?;
    reader.close()?;
    assert!(reader_state.lock().unwrap().cancelled);
    assert!(reader_state.lock().unwrap().closed);

    let artifact_dir = artifact_dir("archive-message-callbacks");
    let source_dir = artifact_dir.join("source");
    fs::create_dir_all(&source_dir)?;
    let source_dir_string = path_string(&source_dir);
    fs::write(source_dir.join("hello.txt"), b"archive callback")?;
    let archive_path = path_string(&artifact_dir.join("payload.aar"));
    let path_list =
        PathList::from_directory_contents(&source_dir_string, None, ArchiveFlags::empty(), 0)?;
    let key_set = FieldKeySet::from_csv("TYP,PAT,SIZ,DAT")?;
    let stream = ByteStream::open_with_path(
        &archive_path,
        OPEN_WRITE_ONLY | OPEN_CREATE | OPEN_TRUNCATE,
        DEFAULT_FILE_MODE,
    )?;
    let mut archive = ArchiveStream::encode_output(stream, ArchiveFlags::empty(), 0)?;
    let write_events = Arc::new(Mutex::new(Vec::new()));
    let write_capture = write_events.clone();
    archive.write_path_list_with_messages(
        &path_list,
        &key_set,
        &source_dir_string,
        ArchiveFlags::empty(),
        0,
        move |event: &mut EntryMessageEvent| {
            write_capture
                .lock()
                .unwrap()
                .push((event.message, event.path.clone()));
            Ok(0)
        },
    )?;
    archive.close()?;
    assert!(write_events.lock().unwrap().iter().any(|(message, _)| {
        matches!(
            message,
            EntryMessage::EncodeScanning | EntryMessage::EncodeWriting
        )
    }));

    let stream = ByteStream::open_with_path(&archive_path, OPEN_READ_ONLY, 0)?;
    let read_events = Arc::new(Mutex::new(Vec::new()));
    let read_capture = read_events.clone();
    let mut archive = ArchiveStream::decode_input_with_messages(
        stream,
        ArchiveFlags::empty(),
        0,
        move |event: &mut EntryMessageEvent| {
            read_capture
                .lock()
                .unwrap()
                .push((event.message, event.path.clone()));
            Ok(0)
        },
    )?;
    let mut decoded = None;
    while let Some(decoded_header) = archive.read_header()? {
        if let Some(blob) = decoded_header.blob_with_key(FieldKey::DAT)? {
            let mut bytes = vec![0_u8; usize::try_from(blob.size)?];
            archive.read_blob(FieldKey::DAT, &mut bytes)?;
            decoded = Some(bytes);
            break;
        }
    }
    assert_eq!(decoded.as_deref(), Some(b"archive callback".as_slice()));
    assert!(read_events
        .lock()
        .unwrap()
        .iter()
        .any(|(message, _)| *message == EntryMessage::DecodeReading));

    Ok(())
}

struct PanicOnDrop;

impl Drop for PanicOnDrop {
    fn drop(&mut self) {
        panic!("state destructor panic");
    }
}

struct PanickingTeardown {
    _guard: PanicOnDrop,
}

impl CustomByteStreamCallbacks for PanickingTeardown {}

impl CustomArchiveStreamCallbacks for PanickingTeardown {}

#[test]
fn panicking_state_destructors_fail_close_instead_of_aborting(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = ByteStream::custom(PanickingTeardown {
        _guard: PanicOnDrop,
    })?;
    assert!(stream.close().is_err());

    let mut archive = ArchiveStream::custom(PanickingTeardown {
        _guard: PanicOnDrop,
    })?;
    assert!(archive.close().is_err());
    Ok(())
}

#[test]
fn panicking_message_handler_fails_the_operation_instead_of_aborting(
) -> Result<(), Box<dyn std::error::Error>> {
    let artifact_dir = artifact_dir("panicking-message-handler");
    let source_dir = artifact_dir.join("source");
    fs::create_dir_all(&source_dir)?;
    fs::write(source_dir.join("file.txt"), b"payload")?;
    let source_dir_string = path_string(&source_dir);
    let path_list =
        PathList::from_directory_contents(&source_dir_string, None, ArchiveFlags::empty(), 0)?;
    let key_set = FieldKeySet::from_csv("TYP,PAT,SIZ,DAT")?;
    let stream = ByteStream::open_with_path(
        &path_string(&artifact_dir.join("payload.aar")),
        OPEN_WRITE_ONLY | OPEN_CREATE | OPEN_TRUNCATE,
        DEFAULT_FILE_MODE,
    )?;
    let mut archive = ArchiveStream::encode_output(stream, ArchiveFlags::empty(), 0)?;
    let result = archive.write_path_list_with_messages(
        &path_list,
        &key_set,
        &source_dir_string,
        ArchiveFlags::empty(),
        0,
        |_event: &mut EntryMessageEvent| -> compression::Result<i32> {
            panic!("message handler panic")
        },
    );
    assert!(result.is_err());
    Ok(())
}

struct RandomAccessSource {
    data: Vec<u8>,
    threads: Arc<Mutex<std::collections::HashSet<std::thread::ThreadId>>>,
}

impl CustomByteStreamCallbacks for RandomAccessSource {
    fn pread(&self, buffer: &mut [u8], offset: i64) -> compression::Result<usize> {
        self.threads
            .lock()
            .unwrap()
            .insert(std::thread::current().id());
        let start = usize::try_from(offset)
            .unwrap_or(usize::MAX)
            .min(self.data.len());
        let count = (self.data.len() - start).min(buffer.len());
        buffer[..count].copy_from_slice(&self.data[start..start + count]);
        Ok(count)
    }
}

struct CollectingSink {
    data: Arc<Mutex<Vec<u8>>>,
}

impl CustomByteStreamCallbacks for CollectingSink {
    fn write(&self, buffer: &[u8]) -> compression::Result<usize> {
        self.data.lock().unwrap().extend_from_slice(buffer);
        Ok(buffer.len())
    }
}

#[test]
fn custom_streams_serve_concurrent_reader_threads() -> Result<(), Box<dyn std::error::Error>> {
    let input = support::pseudo_random_bytes(1 << 20);
    let threads = Arc::new(Mutex::new(std::collections::HashSet::new()));
    let mut source = ByteStream::custom(RandomAccessSource {
        data: input.clone(),
        threads: threads.clone(),
    })?;
    let output = Arc::new(Mutex::new(Vec::new()));
    let mut sink = ByteStream::custom(CollectingSink {
        data: output.clone(),
    })?;
    let processed =
        source.process_random_access_into(&mut sink, -1, 16 * 1024, ArchiveFlags::empty(), 4)?;
    assert_eq!(processed, input.len() as u64);
    sink.close()?;
    source.close()?;
    assert_eq!(*output.lock().unwrap(), input);
    assert!(!threads.lock().unwrap().is_empty());
    Ok(())
}

const fn assert_send<T: Send>() {}

#[test]
fn stream_and_payload_handles_are_send() {
    assert_send::<ByteStream>();
    assert_send::<ArchiveStream>();
    assert_send::<Header>();
    assert_send::<compression::EntryXatBlob>();
    assert_send::<compression::EntryAclBlob>();
    assert_send::<EntryMessageEvent>();
}
