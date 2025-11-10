#!/usr/bin/env bash

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
OPENAPI_DIR="$(dirname "$SCRIPT_DIR")/openapi"
OPENAPI_FILE="$OPENAPI_DIR/openapi.json"

RUST_CRATE="$OPENAPI_DIR/rust"
FLUTTER_CLIENT="$OPENAPI_DIR/flutter_client"

openapi-generator generate \
    -i "$OPENAPI_FILE" \
    -g rust \
    -o "$RUST_CRATE" \
    --global-property models,modelDocs=false,apiDocs=false,apis=false,supportingFiles=

openapi-generator generate \
  -i "$OPENAPI_FILE" \
  -g dart \
  -o "$FLUTTER_CLIENT" \
  --additional-properties pubName=my_api_client,pubVersion=1.0.0,pubAuthor="Robert Sale",pubHomepage="https://example.com",pubDescription="Auto-generated Flutter API client",pubRepository="https://github.com/robertsale/my_api_client",serializationLibrary=native_serialization,disallowAdditionalPropertiesIfNotPresent=false
