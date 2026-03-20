# installing RHEL

[Download RHEL 9](deverlopers.redhat.com)


Patition
 - boot 500Mib
 - root 12Gib
 - others
    - log
    - home 3Gib
    - server document roots
    - container imgs
    - swap 1Gib

# some of the important commands

`chvt(1)` - change virtual terminals eg. `sudo chvt 2` check `man 1 chvt`

use of `man(1)`, rhcsa only focus on `man` 1 5 8

`man <n> intro`

`man -a <command>`

`man hier` directory hierarchy

`man 7 file-hierarchy` description of filesystem hirerachy

search keyword by using `man -k` or `appropos` rebuild `mandb` by `sudo mandb` if nothing appropriate shown

# some of the `bash` `emacs` shortcuts mapping

`ctrl-l` clears screen <br>
`ctrl-u` remove current line <br>
`alt-b/f` moves cursor **B**ackwards/**F**orwads


# Shell expansion

`*` anything <br> `a?*` `?` = any char  <br> `[a-z]*` range a-z <br> `{a,b,c}*` <br> brace extension, can be used as `{0..9}, {a..z}`

use `env` to show all defined environment variables

use `export` to make variable global and can be accessible in sub-shells not in new shell, persistent variable should be stored in `~/.bash_profile`

`alias <key>=<val>` eg `alias dead='rm -rf /'` `unalias` can remove the alias made

`/etc/profile` is generic bash startup file containing all system settings that are processed in login shell

`/etc/bashrc` process opening a subshell

`~/.bash_profile` user specific version of `/etc/profile`

`~/.bashrc` user specific version of `/etc/bashrc`

# systems management
mounting filesystems

`mount /dev/<device> /mnt`

`findmnt` can show all current mounted devices

`umount /mnt` remove device

`lsblk` list block devices

`ln <file> <linkname>` or use `-s` to create symlinks

# `tar` the tap archiver

`tar -cvf archive.tar <dir>` to create a tarball

`tar -tvf` show contents

`tar -xvf archive.tar` extracts to current dir
 - use `-C` to change output path

add `-z -j -J` to add compression/extraction options
 - `gzip(1) -z`
 - `bzip2(1) -j`
 - `xz(1) -J`
 - `zip(1)` windows compatible 

# `awk`

`$NF` predefined variable whose value is the number of fields in the current record

check [Built-in Variables](https://www.gnu.org/software/gawk/manual/html_node/Built_002din-Variables.html)


# `sudo` and `su`
use `sudo -i` than `su -` to gain root access

`visudo` is the **ONLY** tool for editing `/etc/sudoers` file

create drop in files in `/etc/sudoers.d` which will never be overwritten

 - `<user> ALL=/usr/bin/<command>,[!][command]`, generic rules for user to access sudo access but also can disallow users to access certain commands

user in the group `wheel` gets sudo access, use `usermod -aG wheel <user>` to grant sudo access to user

to update the authentication token, add `Defaults timestamp_type=global,timestamp_timeout=<min>`


# `ssh`

use `systemctl status sshd` to verify working status

# managing users

user properties are in `/etc/passwd`

structure of it
 - name
 - password (usually disabled check `/etc/shadow`)
 - uid (id for users, usually starts with 1000)
 - gid (id for groups)
 - gecos (additional non mandatory info about users)
 - home dir
 - shell

`/usr/sbin/useradd` `/usr/sbin/usermod` `/usr/sbin/userdel` `/usr/bin/passwd`

to create user `useradd <username> -u <uid> -s <login shell>`
to remove user completely `userdel -rf <username>`

don't use `-D` flag to specify default settings, don't modify `/etc/default/useradd` that apply to useradd only.

wrtie to `/etc/login.defs` to manage default settings

files in `/etc/skel` are created to the user home dir upon creation

limit user access

`usermod -L <user>` to lock account

`usermod -U <user>` to unlock account

note in `/etc/shadow`, locked account would have `!` at the beginning of password section, can be modified manually

also can set user expire date `usermod -e <2099-01-01> <user>` or set login shell `usermod -s <shell> <user>`

check `passwd --help` it may achieve the same as `usermod` 

every user must has at least 1 group, primary group managed in `/etc/passwd` and optional secondary group managed in `/etc/group`

`newgrp` can be used to set primary group membership


`groupadd` add to add groups, similar to users, `groupdel` is to delete group and `groupmod` is to modify group, note that `usermod -G` would overwrite the supplementary groups, use `groupmod -U <user> <group>` to add user to group

`lid -g <groupname>` to list all users that are members of specific group

use `gpasswd --delete <user> <group>` to remove users from a group

use `chage` to change the definition of user password

`chmod(1)`

to calculate ugo (user group others) permissions, read = 4, write = 2, execute = 1

# shared group dir

SGID (set group id) permission ensures all files created in the shared group dir are group owned by the group owner of the dir `chmod g+s <dir>`

the sticky bit permission ensures that the only user who is the owner of the file or the dir contains the file is allowed to delete file `chmod +t <dir>`

`chmod 3770 <dir>` assign SGID and sticky, by adding the `3` of 4 digit oct numbers

`umask(1)` can change the default permission for file is `666` dir `777`, `umask` would do the subtraction from default. eg set `umask` in `.bashrc` to `077`, the new file created would have `600` which is `rw- --- ---`

`chgrp(1)` for changing file group permissions by `chgrp <group> <file>`

`chown(1)` for changing ownership which is more powerful


# managing network configs

`radvd` package can provide daemon to broadcast ipv6 and auto config ipv6 addrs 

`ip link show` to check current device

`ip addr show` to check device config

BIOS device names (Network Interface Controller):
 - em[1-N] for embedded NICs
 - eno[nn] for embedded NICs
 - `p<slot>p<port>` for NICs on the PCI bus

`hostnamectl hostname` used to set hostname, hostname will be written to `/etc/hostname`, to resolve hostnames `/etc/hosts` is used

`/etc/resolv.con` is dns client config

the order of host name resolution is deined in `/etc/nsswitch.conf`

to add a temp ip addr `ip a a dev <device name> <ip/netmask>` `ip a a dev ens160 10.0.0.10/24`

to add entry in routing table `ip route add [default] <ip> via <ip>`

NetworkManager

config file is stored in `/etc/NetworkManager/system-connects`

`nmcli(1)`
 - `nmcli con show` show all connections
 - `nmcli dev status` show current network devices

 `nmcli con add con-name <connection name> ifname <interface name> ipv4.addresses <ipv4 addr> ipv4.gateway <ipv4 gateway> ipv4.method manual type ethernet`

 `nmcli con up <connection name>`


 `ping6(1)` is used for ipv6

 when using `ping6` on link-local addr, must include NIC name in the command eg `ping6 ff02::!%ens33`

 use `ip [-6] route` for ipv4 or ipv6 routing table

 `tracepath(1)` and `tracepath6(1)` is to show all routing path in the network for ipv4 and ipv6 (if exists)

 `ss(1)` socket stats
  - `ss -[tunap]`


# managing software 

`rpm packages`

`rpm -qa` shows all currently installed packages

`rpm -qf <file>` shows from which package file was installed 

`rpm -ql <package>` list files installed from a package

`rpm -q --scripts <package>` shows scripts exec while installing the package

`rpm -q --changelog <package>` shows changelog of package

add `-p <package>` to any commands above to query before a package installation 

`rpm2cpio <package> | cpio -tv` to show rpm package info

`rpm2cpio <package> | cpio -idmv` to extract contents to current dir


# set up repos manually

online rhel repos are accessed by using `subscription-manager`, alternatively, repos can be offered thru red hat satellite

to access repo offered thru subscription manager, use `dnf config-manager --enable <repo>`

3rd party repo can be added using repo file in `/etc/yum.repos.d/` or using `dnf config-manager --add-repo="file:///repo/AppStream"`

repo config file syntax
```
name=something
baseurl=file:///path/to/repo/stream
enabled=1 | 0
gpgcheck=1 | 0
```

add `gpgcheck=0` to repo config files in `/etc/yum.repos.d/` to disable gpg key check

`dnf repolist` to list all repos

`dnf list` lists installed and availabe packages

`dnf search` searches in name and summary, use `dnf search all` to search in description

`dnf provides */<name>` searches in package file list for the package that provides a specific file (deepest search)

`dnf info` shows the info about the package


`dnf update kernel` can update kernel and keep old kernel as a backup


`dnf group list` to see groups (collection of packages)

some groups are installed via environment groups (used for installing a specific usage pattern and may consist of packages and groups) and not separately, use `dnf group list hidden` to check them

use `dnf group info <group>` to see packages in a group

to install optional packages `dnf group install --with-optional <group>` by default, only mandatory and default packages are installed


`dnf history`  for a summary of all installation and removal transcations, all logs of dnf transcations are logged to `/var/log/dnf.rpm.log`

`dnf hist undo n` can undo a specific transcation

to (un)register the rhel system `subscription-manager [un]register` then `subscription-manager attach --auto`

`/etc/pki/product` indicates the installed redhat products

`/etc/pki/consumer` identifies the redhat acc for rego

`/etc/pki/entitlement` indicates which sub is attached


use `rct` to check current entitilement


# process

`ps -fax` to print process hier

`ps -fU <user>` print user's process

check `/proc/meminfo` for memory usage

`lscpu` to get cpu info

`uptime` to check cpu usage, display average runnable processes over the last 1, 5, 15mins

use `nice` and `renice` to prioritise the process


`tuned` is a systemd service that works with different profiles

`tuned-adm list` shows current profiles

`tuned-adm profile <profile>` sets another profile as default

all `tuned` config is in `/etc/tuned/tuned-main.conf`, can changed `reapply_sysctl = 1` to ensure conflict, if 1 then apply sysctl, else 0 apply tuned config

`sysctl -p <file>` to load file values

add `tuned` files are in `/etc/tuned/<profile>/tuned.conf`

```
cat >> /etc/tuned/<profile>/tuned.conf` << EOF

[sysctl]
vm.swappiness = 66
EOF

```

use `loginctl(8)` 
 - `loginctl list-sessions`
 - `loginctl list-users`
 - `loginctl user-status <uid>` to show tree of processes opened by the user
 - `loginctl terminate-session <session num>`
 - `loginctl terminate-user <username>`


# systemd

managed by `systemctl`

`systemctl list-units -t <type>`

`systemctl list-unit-files` list all unit files and status


`systemctl reload` reload config without restatring the unit

`systemctl enable/disable [--now]` enable or disable automatic start upon system start (now)

default system prodvided systemd unit files are in `/usr/lib/systemd/system` but should not be changed, change `/etc/systemd/system` instead, even better, use `systemctl edit unit.service` to edit unit files or `systemctl cat unit.service` to read service files. use `systemctl show` to show available parameters 

use `systemctl list-dependencies` to show complete view of all currently loaded units and their dependencies or `systemctl list-dependencies <unit>` to list dependencies for any unit

some units cannot work simultaneously on the same system, to prevent that, use `systemctl mask` to create link for unit to `/dev/null` which ensures it cant be started and `systemctl unmask` to remove the link

# task scheduling

## systemd

`man 7 systemd-time` has all time formats can be used in systemd

`OnCalendar` allows set timer on systemd services eg `<year>-<month>-<day> <hr>:<min>:<sec>`, you can always use comma to separate the time, like `9:10, 15, 20:00` is `9:10 9:15 9:20`

`OnUnitActivateSec` to start the unit a specific time after the unit was last activated

`OnBootSec` or `OnStartupSec` to start the unit a specific time after booting

an example of how to use systemd scheduling
```
cat >> /etc/systemd/system/myservice.service << EOF
[Unit]
Description=demo

[Service]
Type=oneshot  <--- read man 5 systemd service to get all types
ExecStart=<command to be recuring>
EOF

cat >> /etc/systemd/system/myservice.timer << EOF  <---- name should be the same for systemd to pick up
[Unit]
Description=demo timer

[Timer]
OnCalendar=<timer format>
EOF

systemctl daemon-reload

systemctl start myservice.timer


systemctl stop myservice.timer

```

## cron

`/etc/crontab` is the main config file (DO NOT to change it)

`/etc/cron.d/` is used for drop-in files

`/etc/cron.{hourly, daily, weekly, monthly}` is used for regular drop ins

for user specific cron jobs should be created by `crontab -e`

cron syntax (in `/etc/contab` as well)
```
[min] [hr] [day of month] [month] [day of week]
0     *    *              dec     1-5
```
which is every Monday to Friday on minute 0 in December

## anacron
takes care of jobs in `/etc/cron.{hourly, daily, weekly, monthly}`, it doesnt care about time just run it on daily basis with no specific time, config in `/etc/anacrontab`

mostly deprecated replaced with `systemd`, DO NOT change `/etc/anacrontab` use drop in files or `systemd` instead

## at
`atd` service must be running to run once-only jobs using `at`

`at <time>`
 - type one or more jobs specifications in interative shell
 - ctrl-D to close the shell

`atq` to list pending jobs

`atrm` to remove jobs from the list


## tmp files
`systemd-tmpfiles` started when booting and manages temp files and dirs

```
/usr/lib/tmpfiles.d/ <----RPM package provided tmp file management
/etc/tmpfiles.d/ <--------user created config files
/run/tmpfiles.d/ <--------system generated files

```

`systemd-tmpfiels-setup.service` creates and removes tmp files according to the config

`systemd-tmpfiles-clean.timer` calls `systemd-tmpfiles-clean.service` to reomve tmp files
 - by default 15 mins after booting
 - on a daily basis

`systemd-tmpfiles --clean /etc/tmpfiles.d/<config file>` to check syntax

`man tmpfiles.d` for examples

syntax
```
[type] [path] [mode] [uid] [gid] [age] [args]
d      /tmp   1777   root  root  7d    

```


# Logging

using systemd journald

`journalctl` prints entire journal

`journalctl -p err` shows only msg with a priority error and higher

`journalctl -f` shows the last 10 lines and adds new msgs while they are added (live mode)

`journalctl -u <service>.service` shows msgs for `<service>.service` only

`journalctl --since '1 hour'; journalctl --since today` allows time specification

`journalctl -o verbose` to output more verbose

`journalctl -xb` adds explanation texts to the boot log msgs

`journalctl --list-boots` shows all boots that have been logged on persistent journal

`journalctl -b 3` shows msgs from the 3rd boot log only



## making the journal persistent

systemd journal settings in `/etc/systemd/journal.conf`, setting `Storage=auto` can set to 
 - `persistent` stores journals in `/var/log/journal`
 - `volatile` stores in the tmp `/run/log/journal` dir
 - none doesnt use any storage for the journal at all

use `journalctl | egrep 'Runtime Journal | System Journal'` to check current setting

to make journal persistent, create `/run/log/journal`, restart `systemd-journald`

## `rsyslog`

main config in `/etc/rsyslog.conf`

drop in files in `/etc/rsyslog.d/`

each logger line contains:
 - facility: the specific facility that the log is created for
 - severity: the severity from which should be logged
 - destination: the file or other destination the log sohld be written to

log files normally are in `/var/log`

use `logger` command to write msgs to `rsyslog` manually


## logrotate

started by `systemd timer` to prevent over growing

main config in `/etc/logrotate.conf`, drop in files in `/etc/logrotate.d/`


# Partition and Mounting

`lsblk(1)`

 - `/dev/sdX` SCSI & SATA disks
 - `/dev/vdX` KVM virtual machines
 - `/dev/nvmeXnY` NVME devices

## gpt (guid partition table) and mbr (master boot record)

mbr
 - 512 bytes to store boot info
 - 64 bytes to store partition
 - 4 partitions only with max size 2 TiB
 - extended logical partitions must to be used for more than 4 partitions

gpt
 - 128 partitions max
 - more space to store partitions
 - overcome mbr limitations

`fdisk(8)`, `gdisk(8)`, `parted(8)`

partition type

`fdisk` use `t` to change part type, use `l` for a list, check `m` for menu
 - `linux`: std linux
 - `swap`: linux swap
 - `uefi`: uefi boot
 - `lvm` logical volume manager

`xfs` is default for rhel
 - fast & scalable
 - CoW (copy on write) to guarantee data integrity
 - size can be increased not decreased


`findmnt`, `mount` can be used to check mounted fs

`lsof` to list openfiles

## `/etc/fstab`
```
[device to mount] [mount point] [vfs type] [mount options] [fs freq] [fs passno]
/dev/sda1         /data         ext4       defaults        0         0
```

`systemd-fstab-generator` is used to generate systemd mounts in `/etc/fstab`

to update systemd mounts, `systemctl daemon-reload` is recommended after editing

to avoid problems while booting, use `mount -a` or `findmnt --verify` after editing `fstab`

if reboot with an faulty `fstab`, in boot menu, remove `rhgb quiet` to get verbose, use root shell to check `mount` if `/boot` is `rw` and `lsblk` to check partition and fs and edit `fstab`


## uuid and labels

`blkid` shows all devices with their naming attributes

`tune2fs -L` set a label on `ext` fs

`xfs_admin -L` set a label on an `xfs`

`mkfs.* -L` set a label while creating the fs

`xfs_admin -U generate <device>` to generate a new uuid

in `fstab` uuid can be used to mount fs

`UUID="<uuid>" /ext4 ext4 defaults 0 0`

or by using labels

`LABEL="<label>" /xfs xfs defaults 0 0`

## systemd mounts

`systemctl cat tmp.mount` to see an example of systemd mounting, can enable it but not recommended

## managing swap

swap is ram emulated on disk, linux system should at least have some swaps

swap can be created with partition, set partition type to `linux-swap`, use `mkswap` to make swap fs, activate using `swapon`

in `fdsik` use `t` to change disk partition type

`fstab` set swap partition to `<device> none swap defaults 0 0` in order to make it persistent

`swapon -a` or `swapon <device>` to activate swap

# LVM

steps to creat lvm

1. create partition by setting type to `lvm`

2. `pvcreate /dev/<device>` to create physical volume

3. `vgcreate vgdata /dev/<device>` to create the volume group

4. `lvcreate -n lvdata -L 1G /dev/vgdata` to create logical volume

5. `mkfs.** /dev/vgdata/lvdata` to create fs

6. put in `fstab`

## extents

extents are the elementary blocks of lvm allocation, the extent size can be defined while defining the vg

`vgcreate -s *M vgdata /dev/<device>` to create a volume group with specific extent size, all lvm logical volumes built from vg will use the same extent size

`vgdisplay` to show properties of vgs, `vgs` and `lvs` to check info about vgs and lvs

## device mapper

lvm devices are a sym link to device mappers in `/dev/dm-x`, persistent names are provided as symlink thru `/dev/mapper`

## resizing lv

use `vgs` to verify vg has unused disk space

`vgextend` to add one or more pg to vg

`lvextend -r  -L +1G` to grow the lvm logical volume
 - `resize2fs` to resize ext fs
 - `xfs_growfs` to grow xfs

xfs is not possible for shrinking

removing vg from pv 

use `pvmove -v <src> <dest>` to move used extents from src to dest

`vgreduce <vg> <device>` to remove device from vg group

steps of removing a vg from pv

```bash
fdisk /dev/<device> ---> create lvm partition

vgcreate vgdemo /dev/sda1

lvcreate -L 1G -n lvdemo /dev/vgdemo

vgextend vgdemo /dev/sda2

pvs --> shows all extents as avail in sda2

lvextend -L +500M /dev/vgdemo/lvdemo /dev/sda2

pvs

mkfs.* /dev/vgdemo/lvdemo

mount /dev/vgdemo/lvdemo /mnt

df -h

dd if=/dev/zero of=/mnt/bigfile bs=1M count=1100

pvmove -v /dev/sda2 /dev/sda1 --> move all used extents from sda2 to sda1

vgreduce vgdemo /dev/sda2



```

# managing Stratis

stratis volumes **always** use **xfs**

stratis volumes are thin provisioned by nature

volume storage is allocated from the stratis pool

each stratis volume needs a min size of 4GiB

stratis needs `stratisd` and `stratis-cli`

`stratis` to create pools and fs, `stratis pool list` to list pool info

to mount stratis volumes
 - **MUST** use UUID, `lsblk --output=UUID <device>`
 - include `x-systemd.requires=stratisd.service` in fstab mount option


`stratis blockdev list` to list block devices

`stratis pool create <name> <device>` to create a pool

`stratis fs create <name> <pool>` to create a fs

## stratis snapshots

metadata copy that allows access the state of the snapshot at anytime, not a backup but can be helpful in accessing deleted files

snapshot is mounted by its device name not UUID

`stratis fs snapshot <pool> <fs> <snapshot name>`

`mount /dev/stratis/<pool>/<snapshot> <mount point>`

`stratis fs destroy <pool> <snapshot>`


# boot procedure

firmware -> boot device (disk) -> boot sector (grub) -> kernel + initramfs (initrd) -> systemd -> early stage -> services -> shell

## modify grub2 runtime

in boot menu, press `e` to edit runtime boot options to the end of the line that starts with `linux`
 - `systemd.unit=emergency.target`
 - `systemd.unit=rescue.target`

press `c` to enter command mode

to edit persistent grub2 parameters, edit config `/etc/default/grub` after writing changes, compile changes to `grub.cfg`


for **MBR** system

`grub2-mkconfig --update-bls-cmdline -o /boot/grub2/grub.cfg`

for **EFI** system

`grub2-mkconfig --update-bls-cmdline -o /boot/efi/EFI/redhat/grub.cfg`

to check system type, `lsblk` to check if boot partitiono is `/boot` or `/boot/efi`

use `grubby` to change persistent boot parameter

`grubby --info=ALL`

`grubby --remove-args="argX argY" --args="argA argB" --update-kernel /boot/kernel`

## systemd targets

systemd target is a group of unit files

some targets are isolatable - they define the final state a system is starting in
 - emergency.target
 - rescue.target
 - multi-user.target
 - graphical.target

when enabling a unit, it is added to a specific target

use `systemctl get-default` to see current default target and `systemctl set-default` to set a new default target

## boot into specific target

on grub boot prompt add `systemctl.unit=*.target` to the end of line starts with `linux` to boot into a specific target

on running system use `systemctl isolate *.target`

# Trouble shooting RHEL

## change root passwd
 - enter grub menu when booting, find the line loads the linux kernel append `init=/bin/bash`
 - `mount -o remount,rw /`
 - `passwd root`
 - `touch /.autorelabel`
 - `exec /usr/lib/systemd/systemd`

## debug shell
`systemctl enable --now debug-shell.service`

while booting `ctrl-alt-f9` to enter root shell without password

`systemctl disable --now debug-shell.service`

## fs issues

fragmentation issues
 - `xfs_fsr` xfs fs reorganizer
 - `e4defrag` defrag ext4 fs

`/etc/fstab` issues
 - use `mount -a` to mount all fs
 - `findmnt --verify` to verify the syntax
 - if everything works well, reboot

## network issues
 - subnet mask: `ip a` all nodes in the same network should be in the same subnet
 - router: `ip r` router must be always in local network
 - dns: verify `/etc/resolv.conf`

## performance issues
 - `top`

## software issues
 - rpm
 -  `ldconfig` to update library cache

## mem issues
 - `free`
 - `vmstat` to monitor swapping

# managing ssh

`ssh-keygen`

`ssh-copy-id` copies the pub key over to the target server

## ssh caching keys

`ssh-agent /bin/bash` allocs space in the bash shell to cache the priv key passphrase

`ssh-add` adds the current passphrase to the cache

cached passphrases stay available for the rest of the session duration

## ssh settings

ssh -X enables X forwarding -Y enables trusted X forwarding

options can be persistently set in generic `/etc/ssh/ssh_config`, drop in file `/etc/ssh/ssh_config.d/` or user specific `~/.ssh/ssh_config`

ssh server config in `/etc/ssh/sshd_config`
 - port 22
 - PermitRootLogin
 - PubKeyAuthentication
 - PasswordAuthentication
 - X11Forwading
 - AllowUsers

restart sshd service once changes are applied

## scp and rsync

`sftp` offers FTP client interface to securely transfer files using ssh


# SELinux

apache httpd config in `/etc/httpd/conf/httpd.conf` or drop in files `/etc/httpd/conf/httpd.conf.d/`

mode or state

state
 - enabled
  - permissive (logging)
  - enforcing (operational) `setenforce`
 - disabled 
 - reboot to switch

`getenforce` show current state
`setenforce` toggles enforcing and permissive

edit `/etc/sysconfig/selinux` to manage the default state

never set to disabled if this is meant as a temp measure only

selinux state can be set at boot using kernel parameter
`enforcing=0|1`
`selinux=0|1` disable or enable 

## context label

defines specific permissions

source objects
 - users
 - processes

target objects
 - files and dirs
 - ports

rules to define source context has access to which target context

booleans allow part of the selinux policy to be rewritten to allow or disallow psecific functionality

context label
 - user
 - role
 - type: flags which type of operation is allowed on this obj

many commands support a `-Z` option to show  current context info

`semanage fcontext` will write the context to the selinux policy but not written to the fs

`semanage fcontext -a` to set a new context label

`semanage fcontext -m` to modify an existing context label

to enforce the policy setting on the fs, use `restorecon`, alternatively, use `touch /.autorelabel` to relabel all files to the context that is specified in the policy

`semanage fcontext -l -C` to show only settings that have changed in the current policy


`man semanage-fcontext`

## find right context


read man page `selinux-policy-doc`

`sealert`

## selinux ports
network ports are also provided with selinux context label

selinux policy is config to allow default port access

for non default port acceess use `semanage-port` to apply label, check man page

## booleans
config switch to enable or disable specific parts of the selinux policy

`semanage boolean -l` or `getsebool -a`

to set booleans use `setsebool -P <boolean> [on|off]`

`semanage boolean -l -C` to see all booleans that have a non default setting


## sealert

selinux uses `auditd` to write log msg to audit log

`journalctl | grep sealert` to check sealert messages


`setenforce 0` first to check if the request is blocked by sealert

then `grep AVC /var/log/audit/audit.log` to show selinux error logs

if sealert messages are not shown, install `setroubleshoot-server` to make it able to be shown via journalctl. restart after installing, or restart `auditd`


# network security
`ss -tulpn` shows tcp and udp sockets in listening state and adds process name or pid to the output

# rhel firewalling
`nftables` framework that applies firewalling

`firewalld` is the service managed by systemd, rhel uses for the frontend to manage nftables firewalls


service: the main component contains one ormore ports as well as optional kernel modules that should be loaded


zone: default config to which network cards can be assigned to apply specific settings


ports: optional elements to allow access to specific ports

`firewalld-cmd --list-all`

`--get-services`

`--add-service <service> [--permanent]`

`--permanent` to write rules to persistent


# Automatic Installtions

kickstart file contains all installtion instructions to set up rhel instance

`ksvalidator <file>` to verify syntax, check `dnf provides */ksvalidator`

## common kickstart file options


`url --url="http://....."` specifies url to access for the installation media

`repo --name="myrepo" --baseurl=...` ways to access repos

`text` forces text mode installtion

`vnc --password=password` enables vnc viewer for remote access to the installation

`clearpart --all --drive=sda,sdb` removes all partitions

`part /home --fstype=ext4 --label=home --size=2048 --maxsize=4096 --grow` creates and mounts partition

`autopart` auto create root swap and boot partitions

`network --device=ens.. --bootproto=dhcp --hostname=<hostname>` config the primary network interface

`firewall --enabled --service=ssh,http`

`timesource --ntp-server pool.ntp.org`

`rootpw --plaintext secret` configs plain text root passwd

`selinux --enforcing`

## using kickstart file
`egrep "svm|vmx" /proc/cpuinfo` to check cpu offers virtualisation extentions

install virtualisation package group `dnf group install "Virtualization Host"

validate the host meets all conditions `virt-host-validate`

to install vm
 - `dnf install virt-install`
 - `virt-install --name testvm --memory 2048 --vcpus 2 --disk size 20 --os-type linux --cdrom /rhel9.iso`

using web console
 - `dnf install cockpit-machines`
 - `systemctl enable --now cockpit.socket`
 - check localhost:9090
# Linux Time

`hwclock` is to set/sync hardware time
 - `hwclock --systohc` set systemtime base on hardware time
 - `hwclock --hctosys` set hardware time base on system time

`timedatectl` to manage time and time zone config
 - `timedatectl status` display all time properties currently used
 - `timedatectl set-time`
 - `timedatectl set-timezone` `timedatectl list-timezone`
 - `timedatectl set-ntp` enables or disables ntp time sync

ntp service should be configured to sync time via ntp, rhel9 uses `chrony` and not using `systemd-timesyncd.service`

uses `/etc/chrony.conf`, uses `iburst` to permit fast sync, restart chronyd service to apply changes

`chronyc sources` to verify proper sync

# Remote FS

installing a sample nfs server

```
dnf install nfs-utils

mkdir -p /nfsdata /home/ldap/ldapuser{1..9}

echo "/nfsdata *(rw,no_root_squash)" >> /etc/exports

echo "/home/ldap *(rw,no_root_squash)" >> /etc/exports

systemctl enable --now nfs-server

for i in nfs mountd rpc-bind; do firewall-cmd --add-service $i --permanent; done

firewall-cmd --reload


```

## mounting nfs shares

install `nfs-utils`

`showmount -e nfsserver` to show exports

`mount  nfsserver:/share /mnt` to mount

## automount
install `autofs`

`systemctl enable --now autofs`

in `/etc/auto.master` will identify the dir that automount should manage and the file that is used for additional mount info
 - add `/data /etc/auto.nfsdata`

create `/etc/auto.nfsdata` will identify the sub dir on which mount and what to mount exactly
 - add `files -rw nfsserver:/nfsdata`

check `/etc/auto.misc` for syntax examples on the exam :))


to support different directory names in one automount line, wildcards are used
`* -rw nfsserver:/home/ldap/&`

restart `autofs` service once config is applied


# managing containers
`podman login registry.redhat.io`

`podman login registry.redhat.io --get-login`

registry access is configed in `/etc/containers/registries.conf`

user specified registry can be configed in `~/.config/containers/registries.conf`

default registries are in `[registries.search]` section

registries that dont have ssl certificate are in `[registries.insecure]`

to build image, `podman` is like `docker`, `podman build -t mytag .`

to show images `podman images`

to run images `podman run [-it] <image>`

show all process `podman ps [-a]` `-a` flag is to who containers that have been stopped

`podman inspect` to see content of image


`podman search`

`podman logs`

`podman exec -it`

## managing environment variables

use `-e KEY=VAL` to pass env variables while running the container

`skopeo inspect` can inspect without pulling image

root mapping can be used by `-p 80:80`, rootless containers can only map to non priviledged ports > 1024, ports mapping cant be done once container is running

`podman port -a` to check ports

optional: `firewall-cmd --add-port=8080/tcp --permanent` and reload it to allow it is reachable


## managing storage

container storage is ephemeral

bind-mounting storage, `podman run -d ... -v /hostdir:/containerdir:Z`

to set correct ownership for rootless containers

find user that runs the main application

`podman inspect <image>`

or 

`podman exec <container> grep username /etc/passwd`

use `podman unshare chown nn:nn dir` to set container uid as the owner of the dir on the host

use `podman unshare cat /proc/self/uid_map` to verify mapping 

verify mapped user is owner, use `ls -ld`

to make selinux happy, when specifying bind mount, `/hostdir:/containerdir:Z` adding a `Z` at the end


## starting containers as systemd service

`loginctl enable-linger` to enable start user services for a specific user

create regular user account to manage all containers

`mkdir ~/.config/systemd/user; cd ~/.config/systemd/user`

`podman generate systemd --name <name> --files --new`
 - `--new` will create a new container when systemd unit is started and delete when unit is stopeed

edit the file that `WantedBy` line to `WantedBy=default.target`

managing them using `systemctl --user`, only work when logging in on console or ssh and do not work in `sudo` or `su` sessions
 - `systemctl --user daemon-reload`
 - `systemctl --user enable <app>.service` requires linger
 - `systemctl --user start <app>.service`

for root container do it from `/etc/systemd/system` as the working dir

check `man podman-generate-systemd`

## building containers by using Containerfile

check `dnf provides */Containerfiles` in `buildah-test` gives basic syntax

`Containerfile` is basically the same as `Dockerfile`

```
FROM node:18-alpine
WORKDIR /app
COPY . .
RUN yarn install --production
CMD ["node", "src/index.js"]
EXPOSE 3000

```