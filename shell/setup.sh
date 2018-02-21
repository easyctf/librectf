FILE=`basename $0`
PROJECT=$(realpath `dirname FILE`)
INCLUDE=$PROJECT/include
source $PROJECT/.env

#========================================
echo "Copying configuration files..."
cp $INCLUDE/etc/pam.d/login /etc/pam.d
cp $INCLUDE/etc/sudoers /etc
chmod 440 /etc/sudoers
cp $INCLUDE/etc/adduser.conf /etc
cp $INCLUDE/etc/security/limits.conf /etc/security
cp $INCLUDE/bin/addctfuser /bin/addctfuser

#========================================
echo "Creating administrator user..."
groupadd ctfadmin
useradd --gid ctfadmin \
        --groups sudo \
        --home-dir /home/ctfadmin \
        --create-home \
        --shell /bin/addctfuser ctfadmin
echo "ctfadmin:$ADMIN_PASSWORD" | chpasswd

chown ctfadmin:ctfadmin /bin/addctfuser
chmod 0100 /bin/addctfuser

#========================================
echo "Creating ctfuser group..."
groupadd --gid 1337 ctfuser
