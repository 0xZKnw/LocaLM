//! File system tools - Create, Edit, Delete, Move, Info, Mkdir
//!
//! Provides comprehensive file manipulation capabilities matching
//! Claude Code and OpenCode's tool sets.

use async_trait::async_trait;
use serde_json::Value;
use std::path::PathBuf;

use crate::agent::tools::{Tool, ToolError, ToolResult};

// ============================================================================
// FileEditTool - String replacement editing (like Claude Code's StrReplace)
// Supports Hashline format: line_number|hash|content
// ============================================================================

pub struct FileEditTool;

#[async_trait]
impl Tool for FileEditTool {
    fn name(&self) -> &str {
        "file_edit"
    }

    fn description(&self) -> &str {
        "Edit a file by replacing an exact string with a new string. Supports two modes:\n1. str_replace: Provide old_string (exact match) + new_string\n2. Hashline: Provide line_number + hash + new_string (hash from file_read output)\n\nThe hash format improves edit success rates by 10-68% for various models.\nREQUIRES APPROVAL."
    }

    fn parameters_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Path to the file to edit"
                },
                "old_string": {
                    "type": "string",
                    "description": "Exact string to find (must be unique in file unless replace_all=true). Use this OR hash+line_number."
                },
                "new_string": {
                    "type": "string",
                    "description": "Replacement string"
                },
                "replace_all": {
                    "type": "boolean",
                    "description": "Replace ALL occurrences (default: false, replaces first unique match)",
                    "default": false
                },
                "line_number": {
                    "type": "number",
                    "description": "Line number to edit (for Hashline mode). Use instead of old_string."
                },
                "hash": {
                    "type": "string",
                    "description": "2-char hash of the line content (from file_read output). Required for Hashline mode."
                }
            },
            "required": ["path", "new_string"]
        })
    }

    async fn execute(&self, params: Value) -> Result<ToolResult, ToolError> {
        let path = params["path"]
            .as_str()
            .ok_or_else(|| ToolError::InvalidParameters("path is required".into()))?;
        let new_string = params["new_string"]
            .as_str()
            .ok_or_else(|| ToolError::InvalidParameters("new_string is required".into()))?;
        
        // Hashline mode: line_number + hash provided
        let hashline_mode = params.get("line_number").is_some() && params.get("hash").is_some();
        
        let content = tokio::fs::read_to_string(path)
            .await
            .map_err(|e| ToolError::ExecutionFailed(format!("Impossible de lire le fichier: {}", e)))?;

        let new_content = if hashline_mode {
            // Hashline mode: edit by line number + hash
            let line_number = params["line_number"]
                .as_u64()
                .ok_or_else(|| ToolError::InvalidParameters("line_number must be a number".into()))? as usize;
            let hash = params["hash"]
                .as_str()
                .ok_or_else(|| ToolError::InvalidParameters("hash must be a string".into()))?;
            
            let lines: Vec<&str> = content.lines().collect();
            let line_idx = line_number.saturating_sub(1);
            
            if line_idx >= lines.len() {
                return Err(ToolError::ExecutionFailed(format!(
                    "Line {} does not exist (file has {} lines)", line_number, lines.len()
                )));
            }
            
            let target_line = lines[line_idx];
            
            // Compute hash of current line content (without the hash prefix)
            let current_hash = compute_line_hash(target_line);
            if current_hash != hash {
                return Err(ToolError::ExecutionFailed(format!(
                    "Hash mismatch! Expected '{}' but found '{}'. The line content has changed since file_read.",
                    hash, current_hash
                )));
            }
            
            // Replace the line
            let mut new_lines: Vec<&str> = lines.clone();
            new_lines[line_idx] = new_string;
            new_lines.join("\n")
        } else {
            // Classic str_replace mode
            let old_string = params["old_string"]
                .as_str()
                .ok_or_else(|| ToolError::InvalidParameters("old_string is required (or use hashline mode with line_number + hash)".into()))?;
            let replace_all = params["replace_all"].as_bool().unwrap_or(false);

            if old_string == new_string {
                return Err(ToolError::InvalidParameters(
                    "old_string and new_string must be different".into(),
                ));
            }

            let count = content.matches(old_string).count();
            if count == 0 {
                return Err(ToolError::ExecutionFailed(
                    "old_string introuvable dans le fichier. Vérifiez l'indentation et les espaces.".into(),
                ));
            }
            if count > 1 && !replace_all {
                return Err(ToolError::ExecutionFailed(format!(
                    "old_string trouvé {} fois. Ajoutez plus de contexte pour le rendre unique, ou utilisez replace_all=true.",
                    count
                )));
            }

            if replace_all {
                content.replace(old_string, new_string)
            } else {
                content.replacen(old_string, new_string, 1)
            }
        };

        tokio::fs::write(path, &new_content)
            .await
            .map_err(|e| ToolError::ExecutionFailed(format!("Impossible d'écrire le fichier: {}", e)))?;

        let count = new_content.matches(new_string).count();
        Ok(ToolResult {
            success: true,
            data: serde_json::json!({
                "path": path,
                "replacements": 1,
                "mode": if hashline_mode { "hashline" } else { "str_replace" },
                "total_lines": new_content.lines().count()
            }),
            message: format!(
                "Fichier édité: {} (1 remplacement, mode: {})",
                path,
                if hashline_mode { "hashline" } else { "str_replace" }
            ),
        })
    }
}

/// Compute hash for a line (must match the one in tools.rs)
fn compute_line_hash(line: &str) -> String {
    let mut hash: u32 = 2166136261u32;
    for byte in line.bytes() {
        hash = hash.wrapping_mul(16777619u32);
        hash ^= byte as u32;
    }
    format!("{:02x}", hash & 0xFFF)
}

// ============================================================================
// FileCreateTool - Create new files (fail if exists)
// ============================================================================

pub struct FileCreateTool;

#[async_trait]
impl Tool for FileCreateTool {
    fn name(&self) -> &str {
        "file_create"
    }

    fn description(&self) -> &str {
        "Create a new file with content. Fails if the file already exists. Creates parent directories automatically. REQUIRES APPROVAL."
    }

    fn parameters_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Path for the new file"
                },
                "content": {
                    "type": "string",
                    "description": "Content to write to the new file"
                },
                "overwrite": {
                    "type": "boolean",
                    "description": "If true, overwrite existing file (default: false)",
                    "default": false
                }
            },
            "required": ["path", "content"]
        })
    }

    async fn execute(&self, params: Value) -> Result<ToolResult, ToolError> {
        let path = params["path"]
            .as_str()
            .ok_or_else(|| ToolError::InvalidParameters("path is required".into()))?;
        let content = params["content"]
            .as_str()
            .ok_or_else(|| ToolError::InvalidParameters("content is required".into()))?;
        let overwrite = params["overwrite"].as_bool().unwrap_or(false);

        let path_buf = PathBuf::from(path);

        // Check if file already exists
        if path_buf.exists() && !overwrite {
            return Err(ToolError::ExecutionFailed(format!(
                "Le fichier '{}' existe déjà. Utilisez overwrite=true pour écraser, ou file_edit pour modifier.",
                path
            )));
        }

        // Create parent directories
        if let Some(parent) = path_buf.parent() {
            if !parent.exists() {
                tokio::fs::create_dir_all(parent)
                    .await
                    .map_err(|e| ToolError::ExecutionFailed(format!("Impossible de créer le dossier parent: {}", e)))?;
            }
        }

        tokio::fs::write(&path_buf, content)
            .await
            .map_err(|e| ToolError::ExecutionFailed(format!("Impossible de créer le fichier: {}", e)))?;

        let lines = content.lines().count();
        let bytes = content.len();

        Ok(ToolResult {
            success: true,
            data: serde_json::json!({
                "path": path,
                "bytes": bytes,
                "lines": lines,
                "created": true
            }),
            message: format!("Fichier créé: {} ({} lignes, {} octets)", path, lines, bytes),
        })
    }
}

// ============================================================================
// FileDeleteTool - Delete files and directories
// ============================================================================

pub struct FileDeleteTool;

#[async_trait]
impl Tool for FileDeleteTool {
    fn name(&self) -> &str {
        "file_delete"
    }

    fn description(&self) -> &str {
        "Delete a file or empty directory. For safety, cannot delete non-empty directories unless recursive=true. REQUIRES APPROVAL."
    }

    fn parameters_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Path to the file or directory to delete"
                },
                "recursive": {
                    "type": "boolean",
                    "description": "If true, delete directory and all contents recursively (DANGEROUS)",
                    "default": false
                }
            },
            "required": ["path"]
        })
    }

    async fn execute(&self, params: Value) -> Result<ToolResult, ToolError> {
        let path = params["path"]
            .as_str()
            .ok_or_else(|| ToolError::InvalidParameters("path is required".into()))?;
        let recursive = params["recursive"].as_bool().unwrap_or(false);

        let path_buf = PathBuf::from(path);

        if !path_buf.exists() {
            return Err(ToolError::ExecutionFailed(format!(
                "Le chemin '{}' n'existe pas",
                path
            )));
        }

        if path_buf.is_file() {
            tokio::fs::remove_file(&path_buf)
                .await
                .map_err(|e| ToolError::ExecutionFailed(format!("Impossible de supprimer: {}", e)))?;

            Ok(ToolResult {
                success: true,
                data: serde_json::json!({ "path": path, "type": "file" }),
                message: format!("Fichier supprimé: {}", path),
            })
        } else if path_buf.is_dir() {
            if recursive {
                tokio::fs::remove_dir_all(&path_buf)
                    .await
                    .map_err(|e| ToolError::ExecutionFailed(format!("Impossible de supprimer le dossier: {}", e)))?;
            } else {
                tokio::fs::remove_dir(&path_buf)
                    .await
                    .map_err(|e| ToolError::ExecutionFailed(format!("Dossier non vide. Utilisez recursive=true: {}", e)))?;
            }

            Ok(ToolResult {
                success: true,
                data: serde_json::json!({ "path": path, "type": "directory", "recursive": recursive }),
                message: format!("Dossier supprimé: {}", path),
            })
        } else {
            Err(ToolError::ExecutionFailed(format!(
                "Type de chemin non supporté: {}",
                path
            )))
        }
    }
}

// ============================================================================
// FileMoveTool - Move/rename files and directories
// ============================================================================

pub struct FileMoveTool;

#[async_trait]
impl Tool for FileMoveTool {
    fn name(&self) -> &str {
        "file_move"
    }

    fn description(&self) -> &str {
        "Move or rename a file or directory. Creates parent directories for destination automatically. REQUIRES APPROVAL."
    }

    fn parameters_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "source": {
                    "type": "string",
                    "description": "Source path (file or directory)"
                },
                "destination": {
                    "type": "string",
                    "description": "Destination path"
                }
            },
            "required": ["source", "destination"]
        })
    }

    async fn execute(&self, params: Value) -> Result<ToolResult, ToolError> {
        let source = params["source"]
            .as_str()
            .ok_or_else(|| ToolError::InvalidParameters("source is required".into()))?;
        let destination = params["destination"]
            .as_str()
            .ok_or_else(|| ToolError::InvalidParameters("destination is required".into()))?;

        let src = PathBuf::from(source);
        let dst = PathBuf::from(destination);

        if !src.exists() {
            return Err(ToolError::ExecutionFailed(format!(
                "Source '{}' n'existe pas",
                source
            )));
        }

        if dst.exists() {
            return Err(ToolError::ExecutionFailed(format!(
                "Destination '{}' existe déjà",
                destination
            )));
        }

        // Create parent directories
        if let Some(parent) = dst.parent() {
            if !parent.exists() {
                tokio::fs::create_dir_all(parent)
                    .await
                    .map_err(|e| ToolError::ExecutionFailed(format!("Impossible de créer le dossier: {}", e)))?;
            }
        }

        tokio::fs::rename(&src, &dst)
            .await
            .map_err(|e| ToolError::ExecutionFailed(format!("Impossible de déplacer: {}", e)))?;

        Ok(ToolResult {
            success: true,
            data: serde_json::json!({
                "source": source,
                "destination": destination
            }),
            message: format!("Déplacé: {} -> {}", source, destination),
        })
    }
}

// ============================================================================
// FileInfoTool - Get file metadata
// ============================================================================

pub struct FileInfoTool;

#[async_trait]
impl Tool for FileInfoTool {
    fn name(&self) -> &str {
        "file_info"
    }

    fn description(&self) -> &str {
        "Get detailed information about a file or directory (size, permissions, timestamps, type)."
    }

    fn parameters_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Path to the file or directory"
                }
            },
            "required": ["path"]
        })
    }

    async fn execute(&self, params: Value) -> Result<ToolResult, ToolError> {
        let path = params["path"]
            .as_str()
            .ok_or_else(|| ToolError::InvalidParameters("path is required".into()))?;

        let path_buf = PathBuf::from(path);
        let metadata = tokio::fs::metadata(&path_buf)
            .await
            .map_err(|e| ToolError::ExecutionFailed(format!("Impossible de lire les métadonnées: {}", e)))?;

        let file_type = if metadata.is_file() {
            "file"
        } else if metadata.is_dir() {
            "directory"
        } else if metadata.is_symlink() {
            "symlink"
        } else {
            "other"
        };

        let size = metadata.len();
        let readonly = metadata.permissions().readonly();

        let modified = metadata
            .modified()
            .ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs());

        let created = metadata
            .created()
            .ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs());

        // Get extension and line count for files
        let extension = path_buf
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_string();

        let line_count = if metadata.is_file() && size < 10_000_000 {
            // Only count lines for files < 10MB
            tokio::fs::read_to_string(&path_buf)
                .await
                .ok()
                .map(|c| c.lines().count())
        } else {
            None
        };

        let size_human = format_size(size);

        Ok(ToolResult {
            success: true,
            data: serde_json::json!({
                "path": path,
                "type": file_type,
                "size": size,
                "size_human": size_human,
                "readonly": readonly,
                "extension": extension,
                "modified_timestamp": modified,
                "created_timestamp": created,
                "line_count": line_count
            }),
            message: format!(
                "{}: {} ({}, {}{})",
                path,
                file_type,
                size_human,
                if readonly { "lecture seule" } else { "lecture/écriture" },
                line_count
                    .map(|c| format!(", {} lignes", c))
                    .unwrap_or_default()
            ),
        })
    }
}

// ============================================================================
// DirectoryCreateTool - mkdir -p
// ============================================================================

pub struct DirectoryCreateTool;

#[async_trait]
impl Tool for DirectoryCreateTool {
    fn name(&self) -> &str {
        "directory_create"
    }

    fn description(&self) -> &str {
        "Create a directory and all parent directories if they don't exist (like mkdir -p). REQUIRES APPROVAL."
    }

    fn parameters_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Path of the directory to create"
                }
            },
            "required": ["path"]
        })
    }

    async fn execute(&self, params: Value) -> Result<ToolResult, ToolError> {
        let path = params["path"]
            .as_str()
            .ok_or_else(|| ToolError::InvalidParameters("path is required".into()))?;

        let path_buf = PathBuf::from(path);

        if path_buf.exists() {
            if path_buf.is_dir() {
                return Ok(ToolResult {
                    success: true,
                    data: serde_json::json!({ "path": path, "already_existed": true }),
                    message: format!("Le dossier existe déjà: {}", path),
                });
            } else {
                return Err(ToolError::ExecutionFailed(format!(
                    "Un fichier existe déjà à ce chemin: {}",
                    path
                )));
            }
        }

        tokio::fs::create_dir_all(&path_buf)
            .await
            .map_err(|e| ToolError::ExecutionFailed(format!("Impossible de créer le dossier: {}", e)))?;

        Ok(ToolResult {
            success: true,
            data: serde_json::json!({ "path": path, "created": true }),
            message: format!("Dossier créé: {}", path),
        })
    }
}

// ============================================================================
// FileCopyTool - Copy files
// ============================================================================

pub struct FileCopyTool;

#[async_trait]
impl Tool for FileCopyTool {
    fn name(&self) -> &str {
        "file_copy"
    }

    fn description(&self) -> &str {
        "Copy a file to a new location. Creates parent directories automatically. REQUIRES APPROVAL."
    }

    fn parameters_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "source": {
                    "type": "string",
                    "description": "Source file path"
                },
                "destination": {
                    "type": "string",
                    "description": "Destination file path"
                }
            },
            "required": ["source", "destination"]
        })
    }

    async fn execute(&self, params: Value) -> Result<ToolResult, ToolError> {
        let source = params["source"]
            .as_str()
            .ok_or_else(|| ToolError::InvalidParameters("source is required".into()))?;
        let destination = params["destination"]
            .as_str()
            .ok_or_else(|| ToolError::InvalidParameters("destination is required".into()))?;

        let src = PathBuf::from(source);
        if !src.exists() {
            return Err(ToolError::ExecutionFailed(format!(
                "Source '{}' n'existe pas",
                source
            )));
        }

        let dst = PathBuf::from(destination);
        if let Some(parent) = dst.parent() {
            if !parent.exists() {
                tokio::fs::create_dir_all(parent)
                    .await
                    .map_err(|e| ToolError::ExecutionFailed(format!("Impossible de créer le dossier: {}", e)))?;
            }
        }

        let bytes = tokio::fs::copy(&src, &dst)
            .await
            .map_err(|e| ToolError::ExecutionFailed(format!("Impossible de copier: {}", e)))?;

        Ok(ToolResult {
            success: true,
            data: serde_json::json!({
                "source": source,
                "destination": destination,
                "bytes": bytes
            }),
            message: format!("Copié: {} -> {} ({} octets)", source, destination, bytes),
        })
    }
}

// ============================================================================
// FileSearchContentTool - Search file content with context
// ============================================================================

pub struct FileSearchContentTool;

#[async_trait]
impl Tool for FileSearchContentTool {
    fn name(&self) -> &str {
        "file_search"
    }

    fn description(&self) -> &str {
        "Search for text content across files in a directory. Returns matching files with line numbers and context. More user-friendly than grep for simple text searches."
    }

    fn parameters_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "Text to search for"
                },
                "path": {
                    "type": "string",
                    "description": "Directory or file to search in",
                    "default": "."
                },
                "file_pattern": {
                    "type": "string",
                    "description": "File extension filter (e.g., 'rs', 'py', 'js')"
                },
                "case_sensitive": {
                    "type": "boolean",
                    "description": "Case sensitive search (default: false)",
                    "default": false
                },
                "max_results": {
                    "type": "integer",
                    "description": "Maximum results to return",
                    "default": 30
                }
            },
            "required": ["query", "path"]
        })
    }

    async fn execute(&self, params: Value) -> Result<ToolResult, ToolError> {
        let query = params["query"]
            .as_str()
            .ok_or_else(|| ToolError::InvalidParameters("query is required".into()))?;
        let path = params["path"].as_str().unwrap_or(".");
        let file_pattern = params["file_pattern"].as_str();
        let case_sensitive = params["case_sensitive"].as_bool().unwrap_or(false);
        let max_results = params["max_results"].as_u64().unwrap_or(30) as usize;

        let search_query = if case_sensitive {
            query.to_string()
        } else {
            query.to_lowercase()
        };

        let path_buf = PathBuf::from(path);
        let mut results = Vec::new();

        search_content_recursive(
            &path_buf,
            &search_query,
            case_sensitive,
            file_pattern,
            &mut results,
            max_results,
        )
        .await?;

        let total = results.len();
        Ok(ToolResult {
            success: true,
            data: serde_json::json!({
                "matches": results,
                "total": total,
                "query": query
            }),
            message: format!("{} résultat(s) pour \"{}\"", total, query),
        })
    }
}

fn search_content_recursive<'a>(
    path: &'a PathBuf,
    query: &'a str,
    case_sensitive: bool,
    file_pattern: Option<&'a str>,
    results: &'a mut Vec<Value>,
    max_results: usize,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), ToolError>> + Send + 'a>> {
    Box::pin(async move {
        if results.len() >= max_results {
            return Ok(());
        }

        if path.is_file() {
            // Check file pattern
            if let Some(pattern) = file_pattern {
                let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
                if ext != pattern {
                    return Ok(());
                }
            }

            if let Ok(content) = tokio::fs::read_to_string(path).await {
                for (i, line) in content.lines().enumerate() {
                    if results.len() >= max_results {
                        break;
                    }
                    let matches = if case_sensitive {
                        line.contains(query)
                    } else {
                        line.to_lowercase().contains(query)
                    };
                    if matches {
                        results.push(serde_json::json!({
                            "file": path.display().to_string(),
                            "line_number": i + 1,
                            "content": line.trim()
                        }));
                    }
                }
            }
        } else if path.is_dir() {
            let mut entries = match tokio::fs::read_dir(path).await {
                Ok(e) => e,
                Err(_) => return Ok(()),
            };

            while let Ok(Some(entry)) = entries.next_entry().await {
                if results.len() >= max_results {
                    break;
                }
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with('.')
                    || name == "node_modules"
                    || name == "target"
                    || name == "__pycache__"
                    || name == ".git"
                {
                    continue;
                }
                search_content_recursive(
                    &entry.path(),
                    query,
                    case_sensitive,
                    file_pattern,
                    results,
                    max_results,
                )
                .await?;
            }
        }
        Ok(())
    })
}

// ============================================================================
// Helpers
// ============================================================================

fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}
