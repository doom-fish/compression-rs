use compression::{
    ArchiveStream, ByteStream, CustomArchiveStreamCallbacks, CustomByteStreamCallbacks, FieldKey,
    Header,
};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

#[derive(Default)]
struct MemoryByteState {
    data: Vec<u8>,
    position: usize,
}

#[derive(Clone)]
struct SharedByteCallbacks {
    inner: Arc<Mutex<MemoryByteState>>,
}

impl CustomByteStreamCallbacks for SharedByteCallbacks {
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

    fn read(&self, buffer: &mut [u8]) -> compression::Result<usize> {
        let mut state = self.inner.lock().unwrap();
        let available = state.data.len().saturating_sub(state.position);
        let count = available.min(buffer.len());
        buffer[..count].copy_from_slice(&state.data[state.position..state.position + count]);
        state.position += count;
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
}

#[derive(Default)]
struct MemoryArchiveState {
    headers: VecDeque<Vec<u8>>,
    blobs: VecDeque<(FieldKey, Vec<u8>)>,
}

#[derive(Clone)]
struct SharedArchiveCallbacks {
    inner: Arc<Mutex<MemoryArchiveState>>,
}

impl CustomArchiveStreamCallbacks for SharedArchiveCallbacks {
    fn write_header(&self, header: &Header) -> compression::Result<()> {
        self.inner
            .lock()
            .unwrap()
            .headers
            .push_back(header.encoded_data()?);
        Ok(())
    }

    fn write_blob(&self, key: FieldKey, buffer: &[u8]) -> compression::Result<()> {
        self.inner
            .lock()
            .unwrap()
            .blobs
            .push_back((key, buffer.to_vec()));
        Ok(())
    }

    fn read_header(&self) -> compression::Result<Option<Header>> {
        let Some(header) = self.inner.lock().unwrap().headers.pop_front() else {
            return Ok(None);
        };
        Ok(Some(Header::from_encoded_data(&header)?))
    }

    fn read_blob(&self, key: FieldKey, buffer: &mut [u8]) -> compression::Result<()> {
        let (expected_key, bytes) = self.inner.lock().unwrap().blobs.pop_front().expect("blob");
        assert_eq!(expected_key, key);
        buffer.copy_from_slice(&bytes);
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let byte_state = Arc::new(Mutex::new(MemoryByteState::default()));
    let mut stream = ByteStream::custom(SharedByteCallbacks { inner: byte_state })?;
    stream.write_all(b"hello custom stream")?;
    stream.seek(0, 0)?;
    let mut buffer = vec![0_u8; 19];
    stream.read(&mut buffer)?;
    assert_eq!(&buffer, b"hello custom stream");

    let archive_state = Arc::new(Mutex::new(MemoryArchiveState::default()));
    let mut writer = ArchiveStream::custom(SharedArchiveCallbacks {
        inner: archive_state.clone(),
    })?;
    let mut header = Header::new()?;
    header.append_field_uint(FieldKey::TYP, u64::from(b'F'))?;
    header.append_field_string(FieldKey::PAT, "custom.txt")?;
    header.append_field_uint(FieldKey::SIZ, buffer.len() as u64)?;
    header.append_field_blob(FieldKey::DAT, buffer.len() as u64)?;
    writer.write_header(&header)?;
    writer.write_blob(FieldKey::DAT, &buffer)?;

    let mut reader = ArchiveStream::custom(SharedArchiveCallbacks {
        inner: archive_state,
    })?;
    let decoded_header = reader.read_header()?.expect("header");
    assert_eq!(decoded_header.path()?.as_deref(), Some("custom.txt"));
    let mut decoded = vec![0_u8; buffer.len()];
    reader.read_blob(FieldKey::DAT, &mut decoded)?;
    assert_eq!(decoded, buffer);

    println!("✅ Custom AppleArchive byte/archive stream callbacks OK");
    Ok(())
}
