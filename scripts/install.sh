mkdir   ~/.rclc/ || echo "Could not create directory for rclc in ~/.rclc/ - try running with sudo."

# If that failed, all else will also fail.
mkdir   ~/.rclc/messages/
mkfifo  ~/.rclc/dtocbuf # Daemon-to-client
mkfifo  ~/.rclc/ctodbuf # Client-to-daemon
