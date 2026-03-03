$packageName = 'RackTop'
$url = 'https://github.com/PatrickCalorioCarvalho/RackTop/releases/latest/download/RackTop-x86_64-pc-windows-gnu.exe'

$packageArgs = @{
  packageName   = 'racktop'
  fileType      = 'exe'
  url           = 'https://github.com/PatrickCalorioCarvalho/RackTop/releases/download/v0.1.6/RackTop-windows.exe'

  checksum      = '4e7a63d3d461c9f3e25753b5343ce1c49c8d9b2e9c98d5a519aa465f2c75b885'
  checksumType  = 'sha256'

  silentArgs    = ''
  validExitCodes= @(0)
}

Install-ChocolateyPackage `
  -PackageName $packageName `
  -FileType exe `
  -Url $url
