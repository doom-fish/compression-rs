# compression-rs coverage audit (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 377
VERIFIED: 373
GAPS: 0
EXEMPT: 4
COVERAGE_PCT: 98.9%

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
| AAArchiveStreamCancelProc | callback type | AACustomArchiveStream.h | ArchiveStream::custom / CustomArchiveStreamCallbacks |
| AAArchiveStreamCloseProc | callback type | AACustomArchiveStream.h | ArchiveStream::custom / CustomArchiveStreamCallbacks |
| AAArchiveStreamReadBlobProc | callback type | AACustomArchiveStream.h | ArchiveStream::custom / CustomArchiveStreamCallbacks |
| AAArchiveStreamReadHeaderProc | callback type | AACustomArchiveStream.h | ArchiveStream::custom / CustomArchiveStreamCallbacks |
| AAArchiveStreamWriteBlobProc | callback type | AACustomArchiveStream.h | ArchiveStream::custom / CustomArchiveStreamCallbacks |
| AAArchiveStreamWriteHeaderProc | callback type | AACustomArchiveStream.h | ArchiveStream::custom / CustomArchiveStreamCallbacks |
| AACustomArchiveStreamSetCancelProc | function | AACustomArchiveStream.h | ArchiveStream::custom / CustomArchiveStreamCallbacks |
| AACustomArchiveStreamSetCloseProc | function | AACustomArchiveStream.h | ArchiveStream::custom / CustomArchiveStreamCallbacks |
| AACustomArchiveStreamSetData | function | AACustomArchiveStream.h | ArchiveStream::custom / CustomArchiveStreamCallbacks |
| AACustomArchiveStreamSetReadBlobProc | function | AACustomArchiveStream.h | ArchiveStream::custom / CustomArchiveStreamCallbacks |
| AACustomArchiveStreamSetReadHeaderProc | function | AACustomArchiveStream.h | ArchiveStream::custom / CustomArchiveStreamCallbacks |
| AACustomArchiveStreamSetWriteBlobProc | function | AACustomArchiveStream.h | ArchiveStream::custom / CustomArchiveStreamCallbacks |
| AACustomArchiveStreamSetWriteHeaderProc | function | AACustomArchiveStream.h | ArchiveStream::custom / CustomArchiveStreamCallbacks |
| AAByteStreamCancelProc | callback type | AACustomByteStream.h | ByteStream::custom / CustomByteStreamCallbacks |
| AAByteStreamCloseProc | callback type | AACustomByteStream.h | ByteStream::custom / CustomByteStreamCallbacks |
| AAByteStreamPReadProc | callback type | AACustomByteStream.h | ByteStream::custom / CustomByteStreamCallbacks |
| AAByteStreamPWriteProc | callback type | AACustomByteStream.h | ByteStream::custom / CustomByteStreamCallbacks |
| AAByteStreamReadProc | callback type | AACustomByteStream.h | ByteStream::custom / CustomByteStreamCallbacks |
| AAByteStreamSeekProc | callback type | AACustomByteStream.h | ByteStream::custom / CustomByteStreamCallbacks |
| AAByteStreamWriteProc | callback type | AACustomByteStream.h | ByteStream::custom / CustomByteStreamCallbacks |
| AACustomByteStreamSetCancelProc | function | AACustomByteStream.h | ByteStream::custom / CustomByteStreamCallbacks |
| AACustomByteStreamSetCloseProc | function | AACustomByteStream.h | ByteStream::custom / CustomByteStreamCallbacks |
| AACustomByteStreamSetData | function | AACustomByteStream.h | ByteStream::custom / CustomByteStreamCallbacks |
| AACustomByteStreamSetPReadProc | function | AACustomByteStream.h | ByteStream::custom / CustomByteStreamCallbacks |
| AACustomByteStreamSetPWriteProc | function | AACustomByteStream.h | ByteStream::custom / CustomByteStreamCallbacks |
| AACustomByteStreamSetReadProc | function | AACustomByteStream.h | ByteStream::custom / CustomByteStreamCallbacks |
| AACustomByteStreamSetSeekProc | function | AACustomByteStream.h | ByteStream::custom / CustomByteStreamCallbacks |
| AACustomByteStreamSetWriteProc | function | AACustomByteStream.h | ByteStream::custom / CustomByteStreamCallbacks |
| AAEntryACLBlob | type | AADefs.h | EntryAclBlob |
| AAEntryXATBlob | type | AADefs.h | EntryXatBlob |
| AAACEFlagSet | type | AAEntryACLBlob.h | AccessControlEntry |
| AAACEPermSet | type | AAEntryACLBlob.h | AccessControlEntry |
| AAACEQualifierType | type | AAEntryACLBlob.h | AceQualifierType |
| AAACETag | type | AAEntryACLBlob.h | AccessControlEntry |
| AAEntryACLBlobAppendEntry | function | AAEntryACLBlob.h | EntryAclBlob |
| AAEntryACLBlobApplyToPath | function | AAEntryACLBlob.h | EntryAclBlob |
| AAEntryACLBlobClear | function | AAEntryACLBlob.h | EntryAclBlob |
| AAEntryACLBlobCreateWithEncodedData | function | AAEntryACLBlob.h | EntryAclBlob |
| AAEntryACLBlobCreateWithPath | function | AAEntryACLBlob.h | EntryAclBlob |
| AAEntryACLBlobDestroy | function | AAEntryACLBlob.h | EntryAclBlob |
| AAEntryACLBlobGetEncodedData | function | AAEntryACLBlob.h | EntryAclBlob |
| AAEntryACLBlobGetEncodedSize | function | AAEntryACLBlob.h | EntryAclBlob |
| AAEntryACLBlobGetEntry | function | AAEntryACLBlob.h | EntryAclBlob |
| AAEntryACLBlobGetEntryCount | function | AAEntryACLBlob.h | EntryAclBlob |
| AAEntryACLBlobRemoveEntry | function | AAEntryACLBlob.h | EntryAclBlob |
| AAEntryACLBlobSetEntry | function | AAEntryACLBlob.h | EntryAclBlob |
| AA_ACE_QUALIFIER_TYPE_GROUP | constant | AAEntryACLBlob.h | AceQualifierType |
| AA_ACE_QUALIFIER_TYPE_SID | constant | AAEntryACLBlob.h | AceQualifierType |
| AA_ACE_QUALIFIER_TYPE_USER | constant | AAEntryACLBlob.h | AceQualifierType |
| AA_ACE_QUALIFIER_TYPE_UUID | constant | AAEntryACLBlob.h | AceQualifierType |
| AAEntryMessageProc | callback type | AAEntryMessage.h | ArchiveStream::{*_with_messages} / EntryMessageHandler |
| AAEntryXATBlobAppendEntry | function | AAEntryXATBlob.h | EntryXatBlob / NamedBlobEntry |
| AAEntryXATBlobApplyToPath | function | AAEntryXATBlob.h | EntryXatBlob / NamedBlobEntry |
| AAEntryXATBlobClear | function | AAEntryXATBlob.h | EntryXatBlob / NamedBlobEntry |
| AAEntryXATBlobCreate | function | AAEntryXATBlob.h | EntryXatBlob / NamedBlobEntry |
| AAEntryXATBlobCreateWithEncodedData | function | AAEntryXATBlob.h | EntryXatBlob / NamedBlobEntry |
| AAEntryXATBlobCreateWithPath | function | AAEntryXATBlob.h | EntryXatBlob / NamedBlobEntry |
| AAEntryXATBlobDestroy | function | AAEntryXATBlob.h | EntryXatBlob / NamedBlobEntry |
| AAEntryXATBlobGetEncodedData | function | AAEntryXATBlob.h | EntryXatBlob / NamedBlobEntry |
| AAEntryXATBlobGetEncodedSize | function | AAEntryXATBlob.h | EntryXatBlob / NamedBlobEntry |
| AAEntryXATBlobGetEntry | function | AAEntryXATBlob.h | EntryXatBlob / NamedBlobEntry |
| AAEntryXATBlobGetEntryCount | function | AAEntryXATBlob.h | EntryXatBlob / NamedBlobEntry |
| AAEntryXATBlobRemoveEntry | function | AAEntryXATBlob.h | EntryXatBlob / NamedBlobEntry |
| AAEntryXATBlobSetEntry | function | AAEntryXATBlob.h | EntryXatBlob / NamedBlobEntry |
| AEAAuthDataAppendEntry | function | AEAAuthData.h | AeaAuthData |
| AEAAuthDataClear | function | AEAAuthData.h | AeaAuthData |
| AEAAuthDataCreate | function | AEAAuthData.h | AeaAuthData |
| AEAAuthDataCreateWithContext | function | AEAAuthData.h | AeaAuthData |
| AEAAuthDataDestroy | function | AEAAuthData.h | AeaAuthData |
| AEAAuthDataGetEncodedData | function | AEAAuthData.h | AeaAuthData |
| AEAAuthDataGetEncodedSize | function | AEAAuthData.h | AeaAuthData |
| AEAAuthDataGetEntry | function | AEAAuthData.h | AeaAuthData |
| AEAAuthDataGetEntryCount | function | AEAAuthData.h | AeaAuthData |
| AEAAuthDataRemoveEntry | function | AEAAuthData.h | AeaAuthData |
| AEAAuthDataSetEntry | function | AEAAuthData.h | AeaAuthData |
| AEAContextCreateWithEncryptedStream | function | AEAContext.h | AeaContext |
| AEAContextCreateWithProfile | function | AEAContext.h | AeaContext |
| AEAContextDestroy | function | AEAContext.h | AeaContext |
| AEAContextGenerateFieldBlob | function | AEAContext.h | AeaContext |
| AEAContextGetArchiveIdentifier | function | AEAContext.h | AeaContext |
| AEAContextGetAuthData | function | AEAContext.h | AeaContext |
| AEAContextGetChecksumMode | function | AEAContext.h | AeaContext |
| AEAContextGetCompressionAlgorithm | function | AEAContext.h | AeaContext |
| AEAContextGetCompressionBlockSize | function | AEAContext.h | AeaContext |
| AEAContextGetContainerSize | function | AEAContext.h | AeaContext |
| AEAContextGetFieldBlob | function | AEAContext.h | AeaContext |
| AEAContextGetFieldUInt | function | AEAContext.h | AeaContext |
| AEAContextGetMainKey | function | AEAContext.h | AeaContext |
| AEAContextGetPaddingSize | function | AEAContext.h | AeaContext |
| AEAContextGetProfile | function | AEAContext.h | AeaContext |
| AEAContextGetRawSize | function | AEAContext.h | AeaContext |
| AEAContextGetSignatureEncryptionKey | function | AEAContext.h | AeaContext |
| AEAContextSetAuthData | function | AEAContext.h | AeaContext |
| AEAContextSetChecksumMode | function | AEAContext.h | AeaContext |
| AEAContextSetCompressionAlgorithm | function | AEAContext.h | AeaContext |
| AEAContextSetCompressionBlockSize | function | AEAContext.h | AeaContext |
| AEAContextSetFieldBlob | function | AEAContext.h | AeaContext |
| AEAContextSetFieldUInt | function | AEAContext.h | AeaContext |
| AEAContextSetMainKey | function | AEAContext.h | AeaContext |
| AEAContextSetPaddingSize | function | AEAContext.h | AeaContext |
| AEAContextSetPassword | function | AEAContext.h | AeaContext |
| AEAContextSetRecipientPrivateKey | function | AEAContext.h | AeaContext |
| AEAContextSetRecipientPublicKey | function | AEAContext.h | AeaContext |
| AEAContextSetSignatureEncryptionKey | function | AEAContext.h | AeaContext |
| AEAContextSetSigningPrivateKey | function | AEAContext.h | AeaContext |
| AEAContextSetSigningPublicKey | function | AEAContext.h | AeaContext |
| AEAContextSetSymmetricKey | function | AEAContext.h | AeaContext |
| AEAProfileGetCiphersuite | function | AEAContext.h | AeaProfile |
| AEAProfileGetEncryptionMode | function | AEAContext.h | AeaProfile |
| AEAProfileGetSignatureMode | function | AEAContext.h | AeaProfile |
| AEAAuthData | type | AEADefs.h | AeaAuthData |
| AEAContext | type | AEADefs.h | AeaContext |
| AEAContextField | type | AEADefs.h | AeaContextField |
| AEAContextFieldRepresentation | type | AEADefs.h | AeaContextFieldRepresentation |
| AEAProfile | type | AEADefs.h | AeaProfile |
| AEA_CONTEXT_CHECKSUM_MURMURHASH64 | constant | AEADefs.h | AeaChecksumMode |
| AEA_CONTEXT_CHECKSUM_NONE | constant | AEADefs.h | AeaChecksumMode |
| AEA_CONTEXT_CHECKSUM_SHA256 | constant | AEADefs.h | AeaChecksumMode |
| AEA_CONTEXT_CIPHERSUITE_HKDF_SHA256_AESCTR_HMAC | constant | AEADefs.h | AeaCiphersuite |
| AEA_CONTEXT_CIPHERSUITE_HKDF_SHA256_HMAC | constant | AEADefs.h | AeaCiphersuite |
| AEA_CONTEXT_ENCRYPTION_ECDHE_P256 | constant | AEADefs.h | AeaEncryptionMode |
| AEA_CONTEXT_ENCRYPTION_NONE | constant | AEADefs.h | AeaEncryptionMode |
| AEA_CONTEXT_ENCRYPTION_SCRYPT | constant | AEADefs.h | AeaEncryptionMode |
| AEA_CONTEXT_ENCRYPTION_SYMMETRIC | constant | AEADefs.h | AeaEncryptionMode |
| AEA_CONTEXT_FIELD_ARCHIVE_IDENTIFIER | constant | AEADefs.h | AeaContextField |
| AEA_CONTEXT_FIELD_AUTH_DATA | constant | AEADefs.h | AeaContextField |
| AEA_CONTEXT_FIELD_BLOCKS_PER_CLUSTER | constant | AEADefs.h | AeaContextField |
| AEA_CONTEXT_FIELD_CHECKSUM_MODE | constant | AEADefs.h | AeaContextField |
| AEA_CONTEXT_FIELD_COMPRESSION_ALGORITHM | constant | AEADefs.h | AeaContextField |
| AEA_CONTEXT_FIELD_COMPRESSION_BLOCK_SIZE | constant | AEADefs.h | AeaContextField |
| AEA_CONTEXT_FIELD_CONTAINER_SIZE | constant | AEADefs.h | AeaContextField |
| AEA_CONTEXT_FIELD_MAIN_KEY | constant | AEADefs.h | AeaContextField |
| AEA_CONTEXT_FIELD_PADDING_SIZE | constant | AEADefs.h | AeaContextField |
| AEA_CONTEXT_FIELD_PASSWORD | constant | AEADefs.h | AeaContextField |
| AEA_CONTEXT_FIELD_PROFILE | constant | AEADefs.h | AeaContextField |
| AEA_CONTEXT_FIELD_RAW_SIZE | constant | AEADefs.h | AeaContextField |
| AEA_CONTEXT_FIELD_RECIPIENT_PRIVATE_KEY | constant | AEADefs.h | AeaContextField |
| AEA_CONTEXT_FIELD_RECIPIENT_PUBLIC_KEY | constant | AEADefs.h | AeaContextField |
| AEA_CONTEXT_FIELD_REPRESENTATION_GENERATE | constant | AEADefs.h | AeaContextField |
| AEA_CONTEXT_FIELD_REPRESENTATION_RAW | constant | AEADefs.h | AeaContextField |
| AEA_CONTEXT_FIELD_REPRESENTATION_X963 | constant | AEADefs.h | AeaContextField |
| AEA_CONTEXT_FIELD_SIGNATURE_ENCRYPTION_KEY | constant | AEADefs.h | AeaContextField |
| AEA_CONTEXT_FIELD_SIGNING_PRIVATE_KEY | constant | AEADefs.h | AeaContextField |
| AEA_CONTEXT_FIELD_SIGNING_PUBLIC_KEY | constant | AEADefs.h | AeaContextField |
| AEA_CONTEXT_FIELD_SYMMETRIC_KEY | constant | AEADefs.h | AeaContextField |
| AEA_CONTEXT_PADDING_ADAPTIVE | constant | AEADefs.h | AeaPadding |
| AEA_CONTEXT_PADDING_MIN_SIZE | constant | AEADefs.h | AeaPadding |
| AEA_CONTEXT_PADDING_NONE | constant | AEADefs.h | AeaPadding |
| AEA_CONTEXT_SIGNATURE_ECDSA_P256 | constant | AEADefs.h | AeaSignatureMode |
| AEA_CONTEXT_SIGNATURE_NONE | constant | AEADefs.h | AeaSignatureMode |
| AEA_PROFILE__HKDF_SHA256_AESCTR_HMAC__ECDHE_P256__ECDSA_P256 | constant | AEADefs.h | AeaProfile |
| AEA_PROFILE__HKDF_SHA256_AESCTR_HMAC__ECDHE_P256__NONE | constant | AEADefs.h | AeaProfile |
| AEA_PROFILE__HKDF_SHA256_AESCTR_HMAC__SCRYPT__NONE | constant | AEADefs.h | AeaProfile |
| AEA_PROFILE__HKDF_SHA256_AESCTR_HMAC__SYMMETRIC__ECDSA_P256 | constant | AEADefs.h | AeaProfile |
| AEA_PROFILE__HKDF_SHA256_AESCTR_HMAC__SYMMETRIC__NONE | constant | AEADefs.h | AeaProfile |
| AEA_PROFILE__HKDF_SHA256_HMAC__NONE__ECDSA_P256 | constant | AEADefs.h | AeaProfile |
| AEAContextDecryptAttributes | function | AEAStreams.h | AeaContext / ByteStream |
| AEADecryptionInputStreamOpen | function | AEAStreams.h | AeaContext / ByteStream |
| AEADecryptionRandomAccessInputStreamOpen | function | AEAStreams.h | AeaContext / ByteStream |
| AEAEncryptionOutputStreamCloseAndUpdateContext | function | AEAStreams.h | AeaContext / ByteStream |
| AEAEncryptionOutputStreamOpen | function | AEAStreams.h | AeaContext / ByteStream |
| AEAEncryptionOutputStreamOpenExisting | function | AEAStreams.h | AeaContext / ByteStream |
| AEAStreamSign | function | AEAStreams.h | AeaContext / ByteStream |

## 🔴 GAPS

No remaining gaps in the audited macOS 26.2 public surface.

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| AAArchiveStreamAbort | function | AppleArchive.h | Deprecated on macOS; skipped per audit instructions. | APPLE_ARCHIVE_AVAILABLE(macos(11.0), ios(14.0), watchos(7.0), tvos(14.0)) __attribute__((availability(macos,deprecated=11.0, replacement="AAArchiveStreamCancel"))) |
| AAByteStreamAbort | function | AppleArchive.h | Deprecated on macOS; skipped per audit instructions. | APPLE_ARCHIVE_AVAILABLE(macos(11.0), ios(14.0), watchos(7.0), tvos(14.0)) __attribute__((availability(macos,deprecated=11.0, replacement="AAByteStreamCancel"))) { AAByteStreamCancel(s) |
| AACustomArchiveStreamSetAbortProc | function | AppleArchive.h | Deprecated on macOS; skipped per audit instructions. | APPLE_ARCHIVE_AVAILABLE(macos(11.0), ios(14.0), watchos(7.0), tvos(14.0)) __attribute__((availability(macos,deprecated=11.0, replacement="AACustomArchiveStreamSetCancelProc"))) |
| AACustomByteStreamSetAbortProc | function | AppleArchive.h | Deprecated on macOS; skipped per audit instructions. | APPLE_ARCHIVE_AVAILABLE(macos(11.0), ios(14.0), watchos(7.0), tvos(14.0)) __attribute__((availability(macos,deprecated=11.0, replacement="AACustomByteStreamSetCancelProc"))) |
