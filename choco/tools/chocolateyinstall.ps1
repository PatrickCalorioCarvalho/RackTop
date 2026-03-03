$packageName = 'RackTop'
$url = 'https://github.com/PatrickCalorioCarvalho/RackTop/releases/latest/download/RackTop-x86_64-pc-windows-gnu.exe'

Install-ChocolateyPackage `
  -PackageName $packageName `
  -FileType exe `
  -Url $url