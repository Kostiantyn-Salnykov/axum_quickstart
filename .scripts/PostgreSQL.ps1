param(
    [ValidateSet("start", "stop", "status", "manual", "auto")]
    [string]$Action = "status"
)

$ServiceName = "postgresql-x64-18"

switch ($Action) {
    "start" {
        Start-Service -Name $ServiceName
        Get-Service -Name $ServiceName
    }
    "stop" {
        Stop-Service -Name $ServiceName
        Get-Service -Name $ServiceName
    }
    "status" {
        Get-Service -Name $ServiceName
    }
    "manual" {
        Set-Service -Name $ServiceName -StartupType Manual
        Get-CimInstance Win32_Service -Filter "Name='$ServiceName'" |
            Select-Object Name, State, StartMode
    }
    "auto" {
        Set-Service -Name $ServiceName -StartupType Automatic
        Get-CimInstance Win32_Service -Filter "Name='$ServiceName'" |
            Select-Object Name, State, StartMode
    }
}