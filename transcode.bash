#!/bin/bash

INPUT_FILE="./assets/camaracapoeira.mp4"
OUTPUT_DIR="./assets"
BASE_NAME=$(basename "$INPUT_FILE" .mp4)

# Target resolutions (width:height)
RESOLUTIONS=("1920:1080" "1280:720" "854:480" "640:360" "426:240" "256:144")
RESOLUTION_NAMES=("1080p" "720p" "480p" "360p" "240p" "144p")

for i in "${!RESOLUTIONS[@]}"; do
    RESOLUTION="${RESOLUTIONS[i]}"
    OUTPUT_NAME="${RESOLUTION_NAMES[i]}"
    
    echo "Transcoding to $OUTPUT_NAME ($RESOLUTION)..."
    
    ffmpeg -y -hwaccel cuda -hwaccel_output_format cuda -i "$INPUT_FILE" \
        -vf "scale_cuda=${RESOLUTION}:interp_algo=lanczos" \
        -c:v h264_nvenc \
        -preset p7 \
        -profile:v high \
        -cq 45 \
        -b:v 0 \
        -c:a aac \
        -b:a 128k \
        "$OUTPUT_DIR/${BASE_NAME}_${OUTPUT_NAME}.mp4"
    
    if [ $? -eq 0 ]; then
        echo "✓ Successfully created $OUTPUT_NAME version"
    else
        echo "✗ Failed to create $OUTPUT_NAME version"
        exit 1
    fi
done

echo "All transcoding complete!"