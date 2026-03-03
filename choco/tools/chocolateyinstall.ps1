$ErrorActionPreference = 'Stop'

$packageName    = 'racktop'
$toolsDir       = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$exeName        = 'racktop.exe'
$exePath        = Join-Path $toolsDir $exeName

$url            = 'https://github.com/PatrickCalorioCarvalho/RackTop/releases/download/v0.1.8/RackTop-windows.exe'

$checksum       = '__CHECKSUM__'
$checksumType   = 'sha256'

Get-ChocolateyWebFile `
  -PackageName    $packageName `
  -FileFullPath   $exePath `
  -Url            $url `
  -Checksum       $checksum `
  -ChecksumType   $checksumType