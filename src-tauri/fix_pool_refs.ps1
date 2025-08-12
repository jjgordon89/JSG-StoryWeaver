# PowerShell script to fix all &**pool references to pool in database operations files

$files = @(
    "src\database\operations\generated_image_ops.rs",
    "src\database\operations\streaming_session_ops.rs",
    "src\database\operations\prose_mode_ops.rs",
    "src\database\operations\collaboration.rs",
    "src\database\operations\app_settings_ops.rs",
    "src\database\operations\ai_provider_ops.rs",
    "src\database\operations\ai_model_configuration_ops.rs",
    "src\database\operations\ai_history_ops.rs",
    "src\database\operations\project_ops.rs",
    "src\database\operations\brainstorm_session_ops.rs",
    "src\database\operations\credit_usage_ops.rs",
    "src\database\operations\plugin.rs"
)

foreach ($file in $files) {
    if (Test-Path $file) {
        Write-Host "Fixing $file"
        $content = Get-Content $file -Raw
        $content = $content -replace '&\*\*pool', 'pool'
        Set-Content $file $content -NoNewline
        Write-Host "Fixed $file"
    } else {
        Write-Host "File not found: $file"
    }
}

Write-Host "All files processed!"