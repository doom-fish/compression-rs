# compression-rs coverage audit (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 377
VERIFIED: 210
GAPS: 163
EXEMPT: 4
COVERAGE_PCT: 56.3%

Counted public typedefs, enum/object-style constants, and top-level/inline C functions from `compression.h` plus `AppleArchive/*.h`; macOS-unavailable entries were filtered out, and deprecated compatibility shims are listed as exempt.

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| AAArchiveStreamCancel | function | AAArchiveStream.h | ArchiveStream |
| AAArchiveStreamClose | function | AAArchiveStream.h | ArchiveStream |
| AAArchiveStreamProcess | function | AAArchiveStream.h | ArchiveStream |
| AAArchiveStreamReadBlob | function | AAArchiveStream.h | ArchiveStream |
| AAArchiveStreamReadHeader | function | AAArchiveStream.h | ArchiveStream |
| AAArchiveStreamWriteBlob | function | AAArchiveStream.h | ArchiveStream |
| AAArchiveStreamWriteHeader | function | AAArchiveStream.h | ArchiveStream |
| AAArchiveStreamWritePathList | function | AAArchiveStream.h | ArchiveStream |
| AAConvertArchiveOutputStreamOpen | function | AAArchiveStream.h | ArchiveStream |
| AADecodeArchiveInputStreamOpen | function | AAArchiveStream.h | ArchiveStream |
| AAEncodeArchiveOutputStreamOpen | function | AAArchiveStream.h | ArchiveStream |
| AAExtractArchiveOutputStreamOpen | function | AAArchiveStream.h | ArchiveStream |
| AAByteStreamCancel | function | AAByteStream.h | ByteStream |
| AAByteStreamClose | function | AAByteStream.h | ByteStream |
| AAByteStreamPRead | function | AAByteStream.h | ByteStream |
| AAByteStreamPWrite | function | AAByteStream.h | ByteStream |
| AAByteStreamProcess | function | AAByteStream.h | ByteStream |
| AAByteStreamRead | function | AAByteStream.h | ByteStream |
| AAByteStreamSeek | function | AAByteStream.h | ByteStream |
| AAByteStreamWrite | function | AAByteStream.h | ByteStream |
| AACompressionOutputStreamOpen | function | AAByteStream.h | ByteStream |
| AACompressionOutputStreamOpenExisting | function | AAByteStream.h | ByteStream |
| AADecompressionInputStreamOpen | function | AAByteStream.h | ByteStream |
| AADecompressionRandomAccessInputStreamOpen | function | AAByteStream.h | ByteStream |
| AAFileStreamOpenWithFD | function | AAByteStream.h | ByteStream |
| AAFileStreamOpenWithPath | function | AAByteStream.h | ByteStream |
| AARandomAccessByteStreamProcess | function | AAByteStream.h | ByteStream |
| AASharedBufferPipeOpen | function | AAByteStream.h | ByteStream |
| AATempFileStreamOpen | function | AAByteStream.h | ByteStream |
| AAArchiveStream | type | AADefs.h | ArchiveStream |
| AAByteStream | type | AADefs.h | ByteStream |
| AACompressionAlgorithm | type | AADefs.h | ArchiveCompressionAlgorithm |
| AAEntryType | type | AADefs.h | EntryType |
| AAFieldKeySet | type | AADefs.h | FieldKeySet |
| AAFieldType | type | AADefs.h | FieldType |
| AAHashFunction | type | AADefs.h | HashFunction |
| AAHeader | type | AADefs.h | Header |
| AAPathList | type | AADefs.h | PathList |
| AA_COMPRESSION_ALGORITHM_LZ4 | constant | AADefs.h | ArchiveCompressionAlgorithm |
| AA_COMPRESSION_ALGORITHM_LZBITMAP | constant | AADefs.h | ArchiveCompressionAlgorithm |
| AA_COMPRESSION_ALGORITHM_LZFSE | constant | AADefs.h | ArchiveCompressionAlgorithm |
| AA_COMPRESSION_ALGORITHM_LZMA | constant | AADefs.h | ArchiveCompressionAlgorithm |
| AA_COMPRESSION_ALGORITHM_NONE | constant | AADefs.h | ArchiveCompressionAlgorithm |
| AA_COMPRESSION_ALGORITHM_ZLIB | constant | AADefs.h | ArchiveCompressionAlgorithm |
| AA_ENTRY_TYPE_BLK | constant | AADefs.h | EntryType |
| AA_ENTRY_TYPE_CHR | constant | AADefs.h | EntryType |
| AA_ENTRY_TYPE_DIR | constant | AADefs.h | EntryType |
| AA_ENTRY_TYPE_DOOR | constant | AADefs.h | EntryType |
| AA_ENTRY_TYPE_FIFO | constant | AADefs.h | EntryType |
| AA_ENTRY_TYPE_LNK | constant | AADefs.h | EntryType |
| AA_ENTRY_TYPE_METADATA | constant | AADefs.h | EntryType |
| AA_ENTRY_TYPE_PORT | constant | AADefs.h | EntryType |
| AA_ENTRY_TYPE_REG | constant | AADefs.h | EntryType |
| AA_ENTRY_TYPE_SOCK | constant | AADefs.h | EntryType |
| AA_ENTRY_TYPE_WHT | constant | AADefs.h | EntryType |
| AA_FIELD_ACL | constant | AADefs.h | FieldKey |
| AA_FIELD_BTM | constant | AADefs.h | FieldKey |
| AA_FIELD_CKS | constant | AADefs.h | FieldKey |
| AA_FIELD_CLC | constant | AADefs.h | FieldKey |
| AA_FIELD_CTM | constant | AADefs.h | FieldKey |
| AA_FIELD_DAT | constant | AADefs.h | FieldKey |
| AA_FIELD_DE2 | constant | AADefs.h | FieldKey |
| AA_FIELD_DEV | constant | AADefs.h | FieldKey |
| AA_FIELD_DUZ | constant | AADefs.h | FieldKey |
| AA_FIELD_FLG | constant | AADefs.h | FieldKey |
| AA_FIELD_GID | constant | AADefs.h | FieldKey |
| AA_FIELD_GIN | constant | AADefs.h | FieldKey |
| AA_FIELD_HLC | constant | AADefs.h | FieldKey |
| AA_FIELD_IDX | constant | AADefs.h | FieldKey |
| AA_FIELD_IDZ | constant | AADefs.h | FieldKey |
| AA_FIELD_INO | constant | AADefs.h | FieldKey |
| AA_FIELD_LNK | constant | AADefs.h | FieldKey |
| AA_FIELD_MOD | constant | AADefs.h | FieldKey |
| AA_FIELD_MTM | constant | AADefs.h | FieldKey |
| AA_FIELD_NLK | constant | AADefs.h | FieldKey |
| AA_FIELD_PAT | constant | AADefs.h | FieldKey |
| AA_FIELD_SH1 | constant | AADefs.h | FieldKey |
| AA_FIELD_SH2 | constant | AADefs.h | FieldKey |
| AA_FIELD_SH3 | constant | AADefs.h | FieldKey |
| AA_FIELD_SH5 | constant | AADefs.h | FieldKey |
| AA_FIELD_SIZ | constant | AADefs.h | FieldKey |
| AA_FIELD_SLC | constant | AADefs.h | FieldKey |
| AA_FIELD_TYP | constant | AADefs.h | FieldKey |
| AA_FIELD_TYPE_BLOB | constant | AADefs.h | FieldType |
| AA_FIELD_TYPE_FLAG | constant | AADefs.h | FieldType |
| AA_FIELD_TYPE_HASH | constant | AADefs.h | FieldType |
| AA_FIELD_TYPE_STRING | constant | AADefs.h | FieldType |
| AA_FIELD_TYPE_TIMESPEC | constant | AADefs.h | FieldType |
| AA_FIELD_TYPE_UINT | constant | AADefs.h | FieldType |
| AA_FIELD_UID | constant | AADefs.h | FieldKey |
| AA_FIELD_UIN | constant | AADefs.h | FieldKey |
| AA_FIELD_XAT | constant | AADefs.h | FieldKey |
| AA_FIELD_YAF | constant | AADefs.h | FieldKey |
| AA_HASH_FUNCTION_CRC32 | constant | AADefs.h | HashFunction |
| AA_HASH_FUNCTION_SHA1 | constant | AADefs.h | HashFunction |
| AA_HASH_FUNCTION_SHA256 | constant | AADefs.h | HashFunction |
| AA_HASH_FUNCTION_SHA384 | constant | AADefs.h | HashFunction |
| AA_HASH_FUNCTION_SHA512 | constant | AADefs.h | HashFunction |
| AAEntryMessage | type | AAEntryMessage.h | EntryMessage |
| AA_ENTRY_MESSAGE_CONVERT_EXCLUDE | constant | AAEntryMessage.h | EntryMessage |
| AA_ENTRY_MESSAGE_DECODE_READING | constant | AAEntryMessage.h | EntryMessage |
| AA_ENTRY_MESSAGE_ENCODE_SCANNING | constant | AAEntryMessage.h | EntryMessage |
| AA_ENTRY_MESSAGE_ENCODE_WRITING | constant | AAEntryMessage.h | EntryMessage |
| AA_ENTRY_MESSAGE_EXTRACT_ACL | constant | AAEntryMessage.h | EntryMessage |
| AA_ENTRY_MESSAGE_EXTRACT_ATTRIBUTES | constant | AAEntryMessage.h | EntryMessage |
| AA_ENTRY_MESSAGE_EXTRACT_BEGIN | constant | AAEntryMessage.h | EntryMessage |
| AA_ENTRY_MESSAGE_EXTRACT_END | constant | AAEntryMessage.h | EntryMessage |
| AA_ENTRY_MESSAGE_EXTRACT_FAIL | constant | AAEntryMessage.h | EntryMessage |
| AA_ENTRY_MESSAGE_EXTRACT_XAT | constant | AAEntryMessage.h | EntryMessage |
| AA_ENTRY_MESSAGE_PROCESS_EXCLUDE | constant | AAEntryMessage.h | EntryMessage |
| AA_ENTRY_MESSAGE_SEARCH_EXCLUDE | constant | AAEntryMessage.h | EntryMessage |
| AA_ENTRY_MESSAGE_SEARCH_FAIL | constant | AAEntryMessage.h | EntryMessage |
| AA_ENTRY_MESSAGE_SEARCH_PRUNE_DIR | constant | AAEntryMessage.h | EntryMessage |
| AAFieldKeySetClear | function | AAFieldKeys.h | FieldKeySet |
| AAFieldKeySetClone | function | AAFieldKeys.h | FieldKeySet |
| AAFieldKeySetContainsKey | function | AAFieldKeys.h | FieldKeySet |
| AAFieldKeySetCreate | function | AAFieldKeys.h | FieldKeySet |
| AAFieldKeySetCreateWithString | function | AAFieldKeys.h | FieldKeySet |
| AAFieldKeySetDestroy | function | AAFieldKeys.h | FieldKeySet |
| AAFieldKeySetGetKey | function | AAFieldKeys.h | FieldKeySet |
| AAFieldKeySetGetKeyCount | function | AAFieldKeys.h | FieldKeySet |
| AAFieldKeySetInsertKey | function | AAFieldKeys.h | FieldKeySet |
| AAFieldKeySetInsertKeySet | function | AAFieldKeys.h | FieldKeySet |
| AAFieldKeySetRemoveKey | function | AAFieldKeys.h | FieldKeySet |
| AAFieldKeySetRemoveKeySet | function | AAFieldKeys.h | FieldKeySet |
| AAFieldKeySetSelectKeySet | function | AAFieldKeys.h | FieldKeySet |
| AAFieldKeySetSerialize | function | AAFieldKeys.h | FieldKeySet |
| AAFlagSet | type | AAFlagSet.h | ArchiveFlags |
| AA_FLAG_ARCHIVE_DEDUPLICATE_DAT | constant | AAFlagSet.h | ArchiveFlags |
| AA_FLAG_ARCHIVE_NO_RESOLVE_ACL_QUALIFIERS | constant | AAFlagSet.h | ArchiveFlags |
| AA_FLAG_CROSS_VOLUME_BOUNDARIES | constant | AAFlagSet.h | ArchiveFlags |
| AA_FLAG_DECODE_INSERT_IDX | constant | AAFlagSet.h | ArchiveFlags |
| AA_FLAG_EXCLUDE_METADATA_ENTRIES | constant | AAFlagSet.h | ArchiveFlags |
| AA_FLAG_EXTRACT_AUTO_DEDUP_AS_HARD_LINKS | constant | AAFlagSet.h | ArchiveFlags |
| AA_FLAG_EXTRACT_NO_AUTO_DEDUP | constant | AAFlagSet.h | ArchiveFlags |
| AA_FLAG_EXTRACT_NO_AUTO_SPARSE | constant | AAFlagSet.h | ArchiveFlags |
| AA_FLAG_IGNORE_EPERM | constant | AAFlagSet.h | ArchiveFlags |
| AA_FLAG_PROCESS_RANDOM_ACCESS_OUTPUT | constant | AAFlagSet.h | ArchiveFlags |
| AA_FLAG_REPLACE_ATTRIBUTES | constant | AAFlagSet.h | ArchiveFlags |
| AA_FLAG_VERBOSITY_0 | constant | AAFlagSet.h | ArchiveFlags |
| AA_FLAG_VERBOSITY_1 | constant | AAFlagSet.h | ArchiveFlags |
| AA_FLAG_VERBOSITY_2 | constant | AAFlagSet.h | ArchiveFlags |
| AA_FLAG_VERBOSITY_3 | constant | AAFlagSet.h | ArchiveFlags |
| AAHeaderAppendFieldBlob | function | AAHeader.h | Header::append_field_blob |
| AAHeaderAppendFieldCString | function | AAHeader.h | Header::append_field_string |
| AAHeaderAppendFieldFlag | function | AAHeader.h | Header::append_field_flag |
| AAHeaderAppendFieldHash | function | AAHeader.h | Header::append_field_hash |
| AAHeaderAppendFieldString | function | AAHeader.h | Header::append_field_string |
| AAHeaderAppendFieldTimespec | function | AAHeader.h | Header::append_field_timespec |
| AAHeaderAppendFieldUInt | function | AAHeader.h | Header::append_field_uint |
| AAHeaderAssign | function | AAHeader.h | Header |
| AAHeaderClear | function | AAHeader.h | Header |
| AAHeaderClone | function | AAHeader.h | Header |
| AAHeaderCreate | function | AAHeader.h | Header |
| AAHeaderCreateWithEncodedData | function | AAHeader.h | Header |
| AAHeaderCreateWithPath | function | AAHeader.h | Header |
| AAHeaderDestroy | function | AAHeader.h | Header |
| AAHeaderGetEncodedData | function | AAHeader.h | Header |
| AAHeaderGetEncodedSize | function | AAHeader.h | Header |
| AAHeaderGetFieldBlob | function | AAHeader.h | Header |
| AAHeaderGetFieldBlobWithKey | function | AAHeader.h | Header::blob_with_key |
| AAHeaderGetFieldCount | function | AAHeader.h | Header |
| AAHeaderGetFieldHash | function | AAHeader.h | Header |
| AAHeaderGetFieldHashWithKey | function | AAHeader.h | Header::hash_with_key |
| AAHeaderGetFieldKey | function | AAHeader.h | Header |
| AAHeaderGetFieldString | function | AAHeader.h | Header |
| AAHeaderGetFieldStringWithKey | function | AAHeader.h | Header::string_with_key |
| AAHeaderGetFieldTimespec | function | AAHeader.h | Header |
| AAHeaderGetFieldTimespecWithKey | function | AAHeader.h | Header::timespec_with_key |
| AAHeaderGetFieldType | function | AAHeader.h | Header |
| AAHeaderGetFieldUInt | function | AAHeader.h | Header |
| AAHeaderGetFieldUIntWithKey | function | AAHeader.h | Header::uint_with_key |
| AAHeaderGetKeyIndex | function | AAHeader.h | Header |
| AAHeaderGetPayloadSize | function | AAHeader.h | Header |
| AAHeaderRemoveField | function | AAHeader.h | Header |
| AAHeaderSetFieldBlob | function | AAHeader.h | Header |
| AAHeaderSetFieldFlag | function | AAHeader.h | Header |
| AAHeaderSetFieldHash | function | AAHeader.h | Header |
| AAHeaderSetFieldString | function | AAHeader.h | Header |
| AAHeaderSetFieldTimespec | function | AAHeader.h | Header |
| AAHeaderSetFieldUInt | function | AAHeader.h | Header |
| AAPathListCreateWithDirectoryContents | function | AAPathList.h | PathList |
| AAPathListCreateWithPath | function | AAPathList.h | PathList |
| AAPathListDestroy | function | AAPathList.h | PathList |
| AAPathListNodeFirst | function | AAPathList.h | PathList |
| AAPathListNodeGetPath | function | AAPathList.h | PathList |
| AAPathListNodeNext | function | AAPathList.h | PathList |
| COMPRESSION_BROTLI | constant | compression.h | Algorithm / raw_ffi |
| COMPRESSION_LZ4 | constant | compression.h | Algorithm / raw_ffi |
| COMPRESSION_LZ4_RAW | constant | compression.h | Algorithm / raw_ffi |
| COMPRESSION_LZBITMAP | constant | compression.h | Algorithm / raw_ffi |
| COMPRESSION_LZFSE | constant | compression.h | Algorithm / raw_ffi |
| COMPRESSION_LZMA | constant | compression.h | Algorithm / raw_ffi |
| COMPRESSION_STATUS_END | constant | compression.h | raw_ffi |
| COMPRESSION_STATUS_ERROR | constant | compression.h | raw_ffi |
| COMPRESSION_STATUS_OK | constant | compression.h | raw_ffi |
| COMPRESSION_STREAM_DECODE | constant | compression.h | CompressionStream / StreamOperation / raw_ffi |
| COMPRESSION_STREAM_ENCODE | constant | compression.h | CompressionStream / StreamOperation / raw_ffi |
| COMPRESSION_STREAM_FINALIZE | constant | compression.h | CompressionStream / StreamOperation / raw_ffi |
| COMPRESSION_ZLIB | constant | compression.h | Algorithm / raw_ffi |
| compression_algorithm | type | compression.h | Algorithm / raw_ffi |
| compression_decode_buffer | function | compression.h | decompress / compression_decode_buffer / raw_ffi |
| compression_decode_scratch_buffer_size | function | compression.h | compression_decode_scratch_buffer_size / raw_ffi |
| compression_encode_buffer | function | compression.h | compress / compression_encode_buffer / raw_ffi |
| compression_status | type | compression.h | raw_ffi |
| compression_stream_destroy | function | compression.h | CompressionStream::drop / raw_ffi |
| compression_stream_flags | type | compression.h | crate public API |
| compression_stream_init | function | compression.h | CompressionStream::new / raw_ffi |
| compression_stream_operation | type | compression.h | CompressionStream / StreamOperation / raw_ffi |
| compression_stream_process | function | compression.h | CompressionStream::process / Encoder / Decoder / raw_ffi |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
| AAArchiveStreamCancelProc | callback type | AACustomArchiveStream.h | Custom archive-stream callback API is not exposed. |
| AAArchiveStreamCloseProc | callback type | AACustomArchiveStream.h | Custom archive-stream callback API is not exposed. |
| AAArchiveStreamReadBlobProc | callback type | AACustomArchiveStream.h | Custom archive-stream callback API is not exposed. |
| AAArchiveStreamReadHeaderProc | callback type | AACustomArchiveStream.h | Custom archive-stream callback API is not exposed. |
| AAArchiveStreamWriteBlobProc | callback type | AACustomArchiveStream.h | Custom archive-stream callback API is not exposed. |
| AAArchiveStreamWriteHeaderProc | callback type | AACustomArchiveStream.h | Custom archive-stream callback API is not exposed. |
| AACustomArchiveStreamSetCancelProc | function | AACustomArchiveStream.h | Custom archive-stream callback API is not exposed. |
| AACustomArchiveStreamSetCloseProc | function | AACustomArchiveStream.h | Custom archive-stream callback API is not exposed. |
| AACustomArchiveStreamSetData | function | AACustomArchiveStream.h | Custom archive-stream callback API is not exposed. |
| AACustomArchiveStreamSetReadBlobProc | function | AACustomArchiveStream.h | Custom archive-stream callback API is not exposed. |
| AACustomArchiveStreamSetReadHeaderProc | function | AACustomArchiveStream.h | Custom archive-stream callback API is not exposed. |
| AACustomArchiveStreamSetWriteBlobProc | function | AACustomArchiveStream.h | Custom archive-stream callback API is not exposed. |
| AACustomArchiveStreamSetWriteHeaderProc | function | AACustomArchiveStream.h | Custom archive-stream callback API is not exposed. |
| AAByteStreamCancelProc | callback type | AACustomByteStream.h | Custom byte-stream callback API is not exposed. |
| AAByteStreamCloseProc | callback type | AACustomByteStream.h | Custom byte-stream callback API is not exposed. |
| AAByteStreamPReadProc | callback type | AACustomByteStream.h | Custom byte-stream callback API is not exposed. |
| AAByteStreamPWriteProc | callback type | AACustomByteStream.h | Custom byte-stream callback API is not exposed. |
| AAByteStreamReadProc | callback type | AACustomByteStream.h | Custom byte-stream callback API is not exposed. |
| AAByteStreamSeekProc | callback type | AACustomByteStream.h | Custom byte-stream callback API is not exposed. |
| AAByteStreamWriteProc | callback type | AACustomByteStream.h | Custom byte-stream callback API is not exposed. |
| AACustomByteStreamSetCancelProc | function | AACustomByteStream.h | Custom byte-stream callback API is not exposed. |
| AACustomByteStreamSetCloseProc | function | AACustomByteStream.h | Custom byte-stream callback API is not exposed. |
| AACustomByteStreamSetData | function | AACustomByteStream.h | Custom byte-stream callback API is not exposed. |
| AACustomByteStreamSetPReadProc | function | AACustomByteStream.h | Custom byte-stream callback API is not exposed. |
| AACustomByteStreamSetPWriteProc | function | AACustomByteStream.h | Custom byte-stream callback API is not exposed. |
| AACustomByteStreamSetReadProc | function | AACustomByteStream.h | Custom byte-stream callback API is not exposed. |
| AACustomByteStreamSetSeekProc | function | AACustomByteStream.h | Custom byte-stream callback API is not exposed. |
| AACustomByteStreamSetWriteProc | function | AACustomByteStream.h | Custom byte-stream callback API is not exposed. |
| AAEntryACLBlob | type | AADefs.h | ACL blob helpers are deferred outside the v0.2.0 scope. |
| AAEntryXATBlob | type | AADefs.h | Extended-attribute blob helpers are deferred outside the v0.2.0 scope. |
| AAACEFlagSet | type | AAEntryACLBlob.h | ACL blob helpers are deferred outside the v0.2.0 scope. |
| AAACEPermSet | type | AAEntryACLBlob.h | ACL blob helpers are deferred outside the v0.2.0 scope. |
| AAACEQualifierType | type | AAEntryACLBlob.h | ACL blob helpers are deferred outside the v0.2.0 scope. |
| AAACETag | type | AAEntryACLBlob.h | ACL blob helpers are deferred outside the v0.2.0 scope. |
| AAEntryACLBlobAppendEntry | function | AAEntryACLBlob.h | ACL blob helpers are deferred outside the v0.2.0 scope. |
| AAEntryACLBlobApplyToPath | function | AAEntryACLBlob.h | ACL blob helpers are deferred outside the v0.2.0 scope. |
| AAEntryACLBlobClear | function | AAEntryACLBlob.h | ACL blob helpers are deferred outside the v0.2.0 scope. |
| AAEntryACLBlobCreateWithEncodedData | function | AAEntryACLBlob.h | ACL blob helpers are deferred outside the v0.2.0 scope. |
| AAEntryACLBlobCreateWithPath | function | AAEntryACLBlob.h | ACL blob helpers are deferred outside the v0.2.0 scope. |
| AAEntryACLBlobDestroy | function | AAEntryACLBlob.h | ACL blob helpers are deferred outside the v0.2.0 scope. |
| AAEntryACLBlobGetEncodedData | function | AAEntryACLBlob.h | ACL blob helpers are deferred outside the v0.2.0 scope. |
| AAEntryACLBlobGetEncodedSize | function | AAEntryACLBlob.h | ACL blob helpers are deferred outside the v0.2.0 scope. |
| AAEntryACLBlobGetEntry | function | AAEntryACLBlob.h | ACL blob helpers are deferred outside the v0.2.0 scope. |
| AAEntryACLBlobGetEntryCount | function | AAEntryACLBlob.h | ACL blob helpers are deferred outside the v0.2.0 scope. |
| AAEntryACLBlobRemoveEntry | function | AAEntryACLBlob.h | ACL blob helpers are deferred outside the v0.2.0 scope. |
| AAEntryACLBlobSetEntry | function | AAEntryACLBlob.h | ACL blob helpers are deferred outside the v0.2.0 scope. |
| AA_ACE_QUALIFIER_TYPE_GROUP | constant | AAEntryACLBlob.h | ACL blob helpers are deferred outside the v0.2.0 scope. |
| AA_ACE_QUALIFIER_TYPE_SID | constant | AAEntryACLBlob.h | ACL blob helpers are deferred outside the v0.2.0 scope. |
| AA_ACE_QUALIFIER_TYPE_USER | constant | AAEntryACLBlob.h | ACL blob helpers are deferred outside the v0.2.0 scope. |
| AA_ACE_QUALIFIER_TYPE_UUID | constant | AAEntryACLBlob.h | ACL blob helpers are deferred outside the v0.2.0 scope. |
| AAEntryMessageProc | callback type | AAEntryMessage.h | Archive wrappers always pass nil msg_proc; callback hooks are not exposed. |
| AAEntryXATBlobAppendEntry | function | AAEntryXATBlob.h | Extended-attribute blob helpers are deferred outside the v0.2.0 scope. |
| AAEntryXATBlobApplyToPath | function | AAEntryXATBlob.h | Extended-attribute blob helpers are deferred outside the v0.2.0 scope. |
| AAEntryXATBlobClear | function | AAEntryXATBlob.h | Extended-attribute blob helpers are deferred outside the v0.2.0 scope. |
| AAEntryXATBlobCreate | function | AAEntryXATBlob.h | Extended-attribute blob helpers are deferred outside the v0.2.0 scope. |
| AAEntryXATBlobCreateWithEncodedData | function | AAEntryXATBlob.h | Extended-attribute blob helpers are deferred outside the v0.2.0 scope. |
| AAEntryXATBlobCreateWithPath | function | AAEntryXATBlob.h | Extended-attribute blob helpers are deferred outside the v0.2.0 scope. |
| AAEntryXATBlobDestroy | function | AAEntryXATBlob.h | Extended-attribute blob helpers are deferred outside the v0.2.0 scope. |
| AAEntryXATBlobGetEncodedData | function | AAEntryXATBlob.h | Extended-attribute blob helpers are deferred outside the v0.2.0 scope. |
| AAEntryXATBlobGetEncodedSize | function | AAEntryXATBlob.h | Extended-attribute blob helpers are deferred outside the v0.2.0 scope. |
| AAEntryXATBlobGetEntry | function | AAEntryXATBlob.h | Extended-attribute blob helpers are deferred outside the v0.2.0 scope. |
| AAEntryXATBlobGetEntryCount | function | AAEntryXATBlob.h | Extended-attribute blob helpers are deferred outside the v0.2.0 scope. |
| AAEntryXATBlobRemoveEntry | function | AAEntryXATBlob.h | Extended-attribute blob helpers are deferred outside the v0.2.0 scope. |
| AAEntryXATBlobSetEntry | function | AAEntryXATBlob.h | Extended-attribute blob helpers are deferred outside the v0.2.0 scope. |
| AEAAuthDataAppendEntry | function | AEAAuthData.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAAuthDataClear | function | AEAAuthData.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAAuthDataCreate | function | AEAAuthData.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAAuthDataCreateWithContext | function | AEAAuthData.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAAuthDataDestroy | function | AEAAuthData.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAAuthDataGetEncodedData | function | AEAAuthData.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAAuthDataGetEncodedSize | function | AEAAuthData.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAAuthDataGetEntry | function | AEAAuthData.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAAuthDataGetEntryCount | function | AEAAuthData.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAAuthDataRemoveEntry | function | AEAAuthData.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAAuthDataSetEntry | function | AEAAuthData.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAContextCreateWithEncryptedStream | function | AEAContext.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAContextCreateWithProfile | function | AEAContext.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAContextDestroy | function | AEAContext.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAContextGenerateFieldBlob | function | AEAContext.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAContextGetArchiveIdentifier | function | AEAContext.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAContextGetAuthData | function | AEAContext.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAContextGetChecksumMode | function | AEAContext.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAContextGetCompressionAlgorithm | function | AEAContext.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAContextGetCompressionBlockSize | function | AEAContext.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAContextGetContainerSize | function | AEAContext.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAContextGetFieldBlob | function | AEAContext.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAContextGetFieldUInt | function | AEAContext.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAContextGetMainKey | function | AEAContext.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAContextGetPaddingSize | function | AEAContext.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAContextGetProfile | function | AEAContext.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAContextGetRawSize | function | AEAContext.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAContextGetSignatureEncryptionKey | function | AEAContext.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAContextSetAuthData | function | AEAContext.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAContextSetChecksumMode | function | AEAContext.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAContextSetCompressionAlgorithm | function | AEAContext.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAContextSetCompressionBlockSize | function | AEAContext.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAContextSetFieldBlob | function | AEAContext.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAContextSetFieldUInt | function | AEAContext.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAContextSetMainKey | function | AEAContext.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAContextSetPaddingSize | function | AEAContext.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAContextSetPassword | function | AEAContext.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAContextSetRecipientPrivateKey | function | AEAContext.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAContextSetRecipientPublicKey | function | AEAContext.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAContextSetSignatureEncryptionKey | function | AEAContext.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAContextSetSigningPrivateKey | function | AEAContext.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAContextSetSigningPublicKey | function | AEAContext.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAContextSetSymmetricKey | function | AEAContext.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAProfileGetCiphersuite | function | AEAContext.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAProfileGetEncryptionMode | function | AEAContext.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAProfileGetSignatureMode | function | AEAContext.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAAuthData | type | AEADefs.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAContext | type | AEADefs.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAContextField | type | AEADefs.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAContextFieldRepresentation | type | AEADefs.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAProfile | type | AEADefs.h | Encrypted-archive (AEA) surface is not exposed. |
| AEA_CONTEXT_CHECKSUM_MURMURHASH64 | constant | AEADefs.h | Encrypted-archive (AEA) surface is not exposed. |
| AEA_CONTEXT_CHECKSUM_NONE | constant | AEADefs.h | Encrypted-archive (AEA) surface is not exposed. |
| AEA_CONTEXT_CHECKSUM_SHA256 | constant | AEADefs.h | Encrypted-archive (AEA) surface is not exposed. |
| AEA_CONTEXT_CIPHERSUITE_HKDF_SHA256_AESCTR_HMAC | constant | AEADefs.h | Encrypted-archive (AEA) surface is not exposed. |
| AEA_CONTEXT_CIPHERSUITE_HKDF_SHA256_HMAC | constant | AEADefs.h | Encrypted-archive (AEA) surface is not exposed. |
| AEA_CONTEXT_ENCRYPTION_ECDHE_P256 | constant | AEADefs.h | Encrypted-archive (AEA) surface is not exposed. |
| AEA_CONTEXT_ENCRYPTION_NONE | constant | AEADefs.h | Encrypted-archive (AEA) surface is not exposed. |
| AEA_CONTEXT_ENCRYPTION_SCRYPT | constant | AEADefs.h | Encrypted-archive (AEA) surface is not exposed. |
| AEA_CONTEXT_ENCRYPTION_SYMMETRIC | constant | AEADefs.h | Encrypted-archive (AEA) surface is not exposed. |
| AEA_CONTEXT_FIELD_ARCHIVE_IDENTIFIER | constant | AEADefs.h | Encrypted-archive (AEA) surface is not exposed. |
| AEA_CONTEXT_FIELD_AUTH_DATA | constant | AEADefs.h | Encrypted-archive (AEA) surface is not exposed. |
| AEA_CONTEXT_FIELD_BLOCKS_PER_CLUSTER | constant | AEADefs.h | Encrypted-archive (AEA) surface is not exposed. |
| AEA_CONTEXT_FIELD_CHECKSUM_MODE | constant | AEADefs.h | Encrypted-archive (AEA) surface is not exposed. |
| AEA_CONTEXT_FIELD_COMPRESSION_ALGORITHM | constant | AEADefs.h | Encrypted-archive (AEA) surface is not exposed. |
| AEA_CONTEXT_FIELD_COMPRESSION_BLOCK_SIZE | constant | AEADefs.h | Encrypted-archive (AEA) surface is not exposed. |
| AEA_CONTEXT_FIELD_CONTAINER_SIZE | constant | AEADefs.h | Encrypted-archive (AEA) surface is not exposed. |
| AEA_CONTEXT_FIELD_MAIN_KEY | constant | AEADefs.h | Encrypted-archive (AEA) surface is not exposed. |
| AEA_CONTEXT_FIELD_PADDING_SIZE | constant | AEADefs.h | Encrypted-archive (AEA) surface is not exposed. |
| AEA_CONTEXT_FIELD_PASSWORD | constant | AEADefs.h | Encrypted-archive (AEA) surface is not exposed. |
| AEA_CONTEXT_FIELD_PROFILE | constant | AEADefs.h | Encrypted-archive (AEA) surface is not exposed. |
| AEA_CONTEXT_FIELD_RAW_SIZE | constant | AEADefs.h | Encrypted-archive (AEA) surface is not exposed. |
| AEA_CONTEXT_FIELD_RECIPIENT_PRIVATE_KEY | constant | AEADefs.h | Encrypted-archive (AEA) surface is not exposed. |
| AEA_CONTEXT_FIELD_RECIPIENT_PUBLIC_KEY | constant | AEADefs.h | Encrypted-archive (AEA) surface is not exposed. |
| AEA_CONTEXT_FIELD_REPRESENTATION_GENERATE | constant | AEADefs.h | Encrypted-archive (AEA) surface is not exposed. |
| AEA_CONTEXT_FIELD_REPRESENTATION_RAW | constant | AEADefs.h | Encrypted-archive (AEA) surface is not exposed. |
| AEA_CONTEXT_FIELD_REPRESENTATION_X963 | constant | AEADefs.h | Encrypted-archive (AEA) surface is not exposed. |
| AEA_CONTEXT_FIELD_SIGNATURE_ENCRYPTION_KEY | constant | AEADefs.h | Encrypted-archive (AEA) surface is not exposed. |
| AEA_CONTEXT_FIELD_SIGNING_PRIVATE_KEY | constant | AEADefs.h | Encrypted-archive (AEA) surface is not exposed. |
| AEA_CONTEXT_FIELD_SIGNING_PUBLIC_KEY | constant | AEADefs.h | Encrypted-archive (AEA) surface is not exposed. |
| AEA_CONTEXT_FIELD_SYMMETRIC_KEY | constant | AEADefs.h | Encrypted-archive (AEA) surface is not exposed. |
| AEA_CONTEXT_PADDING_ADAPTIVE | constant | AEADefs.h | Encrypted-archive (AEA) surface is not exposed. |
| AEA_CONTEXT_PADDING_MIN_SIZE | constant | AEADefs.h | Encrypted-archive (AEA) surface is not exposed. |
| AEA_CONTEXT_PADDING_NONE | constant | AEADefs.h | Encrypted-archive (AEA) surface is not exposed. |
| AEA_CONTEXT_SIGNATURE_ECDSA_P256 | constant | AEADefs.h | Encrypted-archive (AEA) surface is not exposed. |
| AEA_CONTEXT_SIGNATURE_NONE | constant | AEADefs.h | Encrypted-archive (AEA) surface is not exposed. |
| AEA_PROFILE__HKDF_SHA256_AESCTR_HMAC__ECDHE_P256__ECDSA_P256 | constant | AEADefs.h | Encrypted-archive (AEA) surface is not exposed. |
| AEA_PROFILE__HKDF_SHA256_AESCTR_HMAC__ECDHE_P256__NONE | constant | AEADefs.h | Encrypted-archive (AEA) surface is not exposed. |
| AEA_PROFILE__HKDF_SHA256_AESCTR_HMAC__SCRYPT__NONE | constant | AEADefs.h | Encrypted-archive (AEA) surface is not exposed. |
| AEA_PROFILE__HKDF_SHA256_AESCTR_HMAC__SYMMETRIC__ECDSA_P256 | constant | AEADefs.h | Encrypted-archive (AEA) surface is not exposed. |
| AEA_PROFILE__HKDF_SHA256_AESCTR_HMAC__SYMMETRIC__NONE | constant | AEADefs.h | Encrypted-archive (AEA) surface is not exposed. |
| AEA_PROFILE__HKDF_SHA256_HMAC__NONE__ECDSA_P256 | constant | AEADefs.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAContextDecryptAttributes | function | AEAStreams.h | Encrypted-archive (AEA) surface is not exposed. |
| AEADecryptionInputStreamOpen | function | AEAStreams.h | Encrypted-archive (AEA) surface is not exposed. |
| AEADecryptionRandomAccessInputStreamOpen | function | AEAStreams.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAEncryptionOutputStreamCloseAndUpdateContext | function | AEAStreams.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAEncryptionOutputStreamOpen | function | AEAStreams.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAEncryptionOutputStreamOpenExisting | function | AEAStreams.h | Encrypted-archive (AEA) surface is not exposed. |
| AEAStreamSign | function | AEAStreams.h | Encrypted-archive (AEA) surface is not exposed. |

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| AAArchiveStreamAbort | function | AppleArchive.h | Deprecated on macOS; skipped per audit instructions. | APPLE_ARCHIVE_AVAILABLE(macos(11.0), ios(14.0), watchos(7.0), tvos(14.0)) __attribute__((availability(macos,deprecated=11.0, replacement="AAArchiveStreamCancel"))) |
| AAByteStreamAbort | function | AppleArchive.h | Deprecated on macOS; skipped per audit instructions. | APPLE_ARCHIVE_AVAILABLE(macos(11.0), ios(14.0), watchos(7.0), tvos(14.0)) __attribute__((availability(macos,deprecated=11.0, replacement="AAByteStreamCancel"))) { AAByteStreamCancel(s) |
| AACustomArchiveStreamSetAbortProc | function | AppleArchive.h | Deprecated on macOS; skipped per audit instructions. | APPLE_ARCHIVE_AVAILABLE(macos(11.0), ios(14.0), watchos(7.0), tvos(14.0)) __attribute__((availability(macos,deprecated=11.0, replacement="AACustomArchiveStreamSetCancelProc"))) |
| AACustomByteStreamSetAbortProc | function | AppleArchive.h | Deprecated on macOS; skipped per audit instructions. | APPLE_ARCHIVE_AVAILABLE(macos(11.0), ios(14.0), watchos(7.0), tvos(14.0)) __attribute__((availability(macos,deprecated=11.0, replacement="AACustomByteStreamSetCancelProc"))) |
