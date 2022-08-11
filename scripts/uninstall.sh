echo "Uninstalling rclc will delete all chat history on your device. Do you wish to proceed?"

select yn in "Yes" "No"; do
    case $yn in
        Yes ) rm -R ~/.rclc/; break;;
        No ) exit;;
    esac
done
