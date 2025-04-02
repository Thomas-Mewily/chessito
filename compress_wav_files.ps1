
<# 
   You need FFmpeg in order to run it :
   https://ffmpeg.org/download.html
   
   .\compress_wav_files.ps1
#>

# Path to the directory containing WAV files
$directory = ".\board_graphic\assets\sound_uncompressed.ignore"

# Define the output directory (optional, if you want to keep original files and store compressed files separately)
$outputDirectory = ".\board_graphic\assets\sound"

# Ensure the output directory exists
if (!(Test-Path -Path $outputDirectory)) {
    New-Item -ItemType Directory -Path $outputDirectory
}

# Recursively find all WAV files in the directory
$wavFiles = Get-ChildItem -Path $directory -Recurse -Filter "*.wav"

foreach ($wavFile in $wavFiles) {
    # Get the relative path of the WAV file
    $relativePath = $wavFile.FullName.Substring($directory.Length)

    # Construct the output file path
    $outputFilePath = Join-Path $outputDirectory $relativePath

    # Ensure the output subdirectory exists
    $outputSubDir = Split-Path -Parent $outputFilePath
    if (!(Test-Path -Path $outputSubDir)) {
        New-Item -ItemType Directory -Path $outputSubDir
    }

    # FFmpeg command to compress the WAV file
    $ffmpegCommand = "ffmpeg -i `"$($wavFile.FullName)`" -acodec libmp3lame -b:a 192k `"$outputFilePath`""
    #$ffmpegCommand = "ffmpeg -i `"$($wavFile.FullName)`" -acodec libmp3lame -b:a 128k `"$outputFilePath`""
    
    # Run the FFmpeg command
    Invoke-Expression $ffmpegCommand
}
