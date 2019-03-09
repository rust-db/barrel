# Get to the right location
sudo lxc-attach -n spacekookie
cd /var/www/website

# Update the data
git pull

# Re-generate
rm -rf output/
. env/bin/activate
pelican content/