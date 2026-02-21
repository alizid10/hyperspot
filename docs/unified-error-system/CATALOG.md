# GTS System Errors Catalog

All types below show the full chained GTS type identifier used in the Problem `type` field (`gts://` prefix stripped for brevity). All identifiers end with `~` since they are schemas.

---

## 1. Platform Core Errors (`cf.core.errors.*`)

Defined in [`libs/modkit-errors/src/catalog.rs`](../../libs/modkit-errors/src/catalog.rs). Reusable across all modules.

| Full GTS Type | Status | Title | Metadata Fields |
|---------------|--------|-------|-----------------|
| `gts.cf.core.errors.err.v1~cf.core.errors.bad_request.v1~` | 400 | Bad Request | `message` |
| `gts.cf.core.errors.err.v1~cf.core.errors.forbidden.v1~` | 403 | Forbidden | |
| `gts.cf.core.errors.err.v1~cf.core.errors.not_found.v1~` | 404 | Not Found | `message` |
| `gts.cf.core.errors.err.v1~cf.core.errors.conflict.v1~` | 409 | Conflict | `message` |
| `gts.cf.core.errors.err.v1~cf.core.errors.unsupported_media_type.v1~` | 415 | Unsupported Media Type | `message` |
| `gts.cf.core.errors.err.v1~cf.core.errors.validation_failed.v1~` | 422 | Validation Failed | `message` |
| `gts.cf.core.errors.err.v1~cf.core.errors.internal.v1~` | 500 | Internal Server Error | |
| `gts.cf.core.errors.err.v1~cf.core.errors.config.v1~` | 500 | Configuration Error | `message` |
| `gts.cf.core.errors.err.v1~cf.core.errors.unknown.v1~` | 500 | Unknown Error | |

## 2. Types Registry Errors (`cf.types_registry.errors.*`)

Defined in [`modules/system/types-registry/types-registry/src/errors.rs`](../../modules/system/types-registry/types-registry/src/errors.rs).

| Full GTS Type | Status | Title | Metadata Fields |
|---------------|--------|-------|-----------------|
| `gts.cf.core.errors.err.v1~cf.types_registry.errors.invalid_gts_id.v1~` | 400 | Invalid GTS ID | `message` |
| `gts.cf.core.errors.err.v1~cf.types_registry.errors.not_found.v1~` | 404 | Entity Not Found | `gts_id` |
| `gts.cf.core.errors.err.v1~cf.types_registry.errors.already_exists.v1~` | 409 | Entity Already Exists | `gts_id` |
| `gts.cf.core.errors.err.v1~cf.types_registry.errors.validation_failed.v1~` | 422 | Validation Failed | `message` |
| `gts.cf.core.errors.err.v1~cf.types_registry.errors.not_ready.v1~` | 503 | Service Not Ready | |
| `gts.cf.core.errors.err.v1~cf.types_registry.errors.activation_failed.v1~` | 500 | Registry Activation Failed | `error_count` |
| `gts.cf.core.errors.err.v1~cf.types_registry.errors.internal.v1~` | 500 | Internal Server Error | |

## 3. File Parser Errors (`cf.file_parser.errors.*`)

Defined in [`modules/file-parser/src/errors.rs`](../../modules/file-parser/src/errors.rs).

| Full GTS Type | Status | Title | Metadata Fields |
|---------------|--------|-------|-----------------|
| `gts.cf.core.errors.err.v1~cf.file_parser.errors.file_not_found.v1~` | 404 | File Not Found | `path` |
| `gts.cf.core.errors.err.v1~cf.file_parser.errors.unsupported_file_type.v1~` | 400 | Unsupported File Type | `extension` |
| `gts.cf.core.errors.err.v1~cf.file_parser.errors.no_parser_available.v1~` | 415 | No Parser Available | `extension` |
| `gts.cf.core.errors.err.v1~cf.file_parser.errors.parse_error.v1~` | 422 | Parse Error | `message` |
| `gts.cf.core.errors.err.v1~cf.file_parser.errors.invalid_url.v1~` | 400 | Invalid URL | `url` |
| `gts.cf.core.errors.err.v1~cf.file_parser.errors.invalid_request.v1~` | 400 | Invalid Request | `message` |
| `gts.cf.core.errors.err.v1~cf.file_parser.errors.download_error.v1~` | 502 | Download Error | |
| `gts.cf.core.errors.err.v1~cf.file_parser.errors.io_error.v1~` | 500 | IO Error | |

## 4. Nodes Registry Errors (`cf.nodes_registry.errors.*`)

Defined in [`modules/system/nodes-registry/nodes-registry/src/errors.rs`](../../modules/system/nodes-registry/nodes-registry/src/errors.rs).

| Full GTS Type | Status | Title | Metadata Fields |
|---------------|--------|-------|-----------------|
| `gts.cf.core.errors.err.v1~cf.nodes_registry.errors.not_found.v1~` | 404 | Node Not Found | `node_id` |
| `gts.cf.core.errors.err.v1~cf.nodes_registry.errors.validation.v1~` | 400 | Validation Error | `message` |
| `gts.cf.core.errors.err.v1~cf.nodes_registry.errors.sysinfo_failed.v1~` | 500 | System Information Collection Failed | |
| `gts.cf.core.errors.err.v1~cf.nodes_registry.errors.syscap_failed.v1~` | 500 | System Capabilities Collection Failed | |
| `gts.cf.core.errors.err.v1~cf.nodes_registry.errors.internal.v1~` | 500 | Internal Server Error | |

## 5. OData Errors (`cf.odata.errors.*`)

Defined in [`libs/modkit-odata/src/errors.rs`](../../libs/modkit-odata/src/errors.rs).

| Full GTS Type | Status | Title | Metadata Fields |
|---------------|--------|-------|-----------------|
| `gts.cf.core.errors.err.v1~cf.odata.errors.invalid_filter.v1~` | 422 | Invalid Filter | `message` |
| `gts.cf.core.errors.err.v1~cf.odata.errors.invalid_orderby.v1~` | 422 | Invalid OrderBy | `message` |
| `gts.cf.core.errors.err.v1~cf.odata.errors.invalid_cursor.v1~` | 422 | Invalid Cursor | `message` |
| `gts.cf.core.errors.err.v1~cf.odata.errors.internal.v1~` | 500 | Internal OData Error | |

## 6. Tenant Resolver Errors (`cf.tenant_resolver.errors.*`)

Defined in [`examples/plugin-modules/tenant-resolver/tenant-resolver-gw/src/errors.rs`](../../examples/plugin-modules/tenant-resolver/tenant-resolver-gw/src/errors.rs).

| Full GTS Type | Status | Title | Metadata Fields |
|---------------|--------|-------|-----------------|
| `gts.cf.core.errors.err.v1~cf.tenant_resolver.errors.plugin_not_found.v1~` | 404 | Plugin Not Found | `vendor` |
| `gts.cf.core.errors.err.v1~cf.tenant_resolver.errors.plugin_unavailable.v1~` | 503 | Plugin Unavailable | `gts_id` |
| `gts.cf.core.errors.err.v1~cf.tenant_resolver.errors.invalid_plugin_instance.v1~` | 400 | Invalid Plugin Instance | `gts_id`, `message` |
| `gts.cf.core.errors.err.v1~cf.tenant_resolver.errors.types_registry_unavailable.v1~` | 500 | Types Registry Unavailable | |
| `gts.cf.core.errors.err.v1~cf.tenant_resolver.errors.tenant_not_found.v1~` | 404 | Tenant Not Found | |
| `gts.cf.core.errors.err.v1~cf.tenant_resolver.errors.permission_denied.v1~` | 403 | Permission Denied | |
| `gts.cf.core.errors.err.v1~cf.tenant_resolver.errors.internal.v1~` | 500 | Internal Error | |

## 7. Simple User Settings Errors (`hx.settings.simple_user_settings.*`)

Defined in [`modules/simple-user-settings/simple-user-settings/src/errors.rs`](../../modules/simple-user-settings/simple-user-settings/src/errors.rs).

| Full GTS Type | Status | Title | Metadata Fields |
|---------------|--------|-------|-----------------|
| `gts.cf.core.errors.err.v1~hx.settings.simple_user_settings.not_found.v1~` | 404 | Settings Not Found | `message` |
| `gts.cf.core.errors.err.v1~hx.settings.simple_user_settings.validation.v1~` | 422 | Validation Error | `message` |
| `gts.cf.core.errors.err.v1~hx.settings.simple_user_settings.internal_database.v1~` | 500 | Internal Database Error | |

---

## Error Code Quick Reference

| Status | Core Errors |
|--------|-------------|
| **400** | `core.errors.bad_request` |
| **403** | `core.errors.forbidden` |
| **404** | `core.errors.not_found` |
| **409** | `core.errors.conflict` |
| **415** | `core.errors.unsupported_media_type` |
| **422** | `core.errors.validation_failed` |
| **500** | `core.errors.internal`, `core.errors.config`, `core.errors.unknown` |
