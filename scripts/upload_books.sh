#!/bin/sh

# Get script directory and project root
SCRIPT_DIR=$(dirname "$0")
PROJECT_ROOT=$(cd "$SCRIPT_DIR/.." && pwd)
BOOKS_DIR="$PROJECT_ROOT/books"

# Load .env file
if [ -f "$SCRIPT_DIR/.env" ]; then
    export $(grep -v '^#' "$SCRIPT_DIR/.env" | xargs)
else
    echo "Error: .env file not found!"
    exit 1
fi

# Check if all variables are set
if [ -z "$SFTP_HOST" ] || [ -z "$SFTP_USER" ] || [ -z "$SFTP_KEY" ]; then
    echo "Error: SFTP credentials incomplete in .env"
    exit 1
fi

# Set default values
SFTP_PORT="${SFTP_PORT:-22}"
SFTP_REMOTE_PATH="${SFTP_REMOTE_PATH:-.}"

# Check for dry-run flag
DRY_RUN=""
if [ "$1" = "--dry-run" ] || [ "$1" = "-d" ]; then
    DRY_RUN="--dry-run"
    echo "DRY-RUN MODE - No files will be uploaded"
    echo ""
fi


echo "Starting upload..."

# Build lftp commands dynamically
LFTP_COMMANDS="set sftp:connect-program 'ssh -i $SFTP_KEY';"
for book_dir in "$BOOKS_DIR"/*/book/; do
    if [ -d "$book_dir" ]; then
        book_name=$(echo "$book_dir" | awk -F'/' '{print $(NF-2)}')
        LFTP_COMMANDS="$LFTP_COMMANDS mirror -R --delete --verbose $DRY_RUN $book_dir $SFTP_REMOTE_PATH/$book_name/;"
    fi
done
LFTP_COMMANDS="$LFTP_COMMANDS quit"

# Upload via lftp
lftp -e "$LFTP_COMMANDS" sftp://$SFTP_USER@$SFTP_HOST:$SFTP_PORT

if [ $? -eq 0 ]; then
    if [ -n "$DRY_RUN" ]; then
        echo ""
        echo "Dry-run complete! Run without --dry-run to upload."
    else
        echo ""
        echo "Upload successful!"
    fi
else
    echo "Upload failed!"
    exit 1
fi