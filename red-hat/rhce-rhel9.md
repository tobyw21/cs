# Setting up the Control Node

`sudo dnf install ansible-core`

`pip install ansible-navigator`


# Set up Managed Nodes

```
cat >> inventory << EOF
ansible1 ansible_ssh_user=<user>
ansible2
EOF

ansible -i inventory all -u <user> -k -b -K -m user -a "name=ansible"

ansible -i inventory all -u <user> -k -b -K -m shell -a "echo <password> | passwd --stdin ansible"

ssh-keygen (control user and ansible user)

for i in ansible1 ansible2; do ssh-copy-id $i; done (control user and ansible user)

ansible -i inventory all -u <user> -k -b -K -m shell -a 'echo "ansible ALL=(ALL) NOPASSWD: ALL" > /etc/sudoers.d/ansible'

ansible -i inventory  all -u ansible -b -m command -a "ls -l /root" -K
```

some notes

shell module can have pipes where command module cannot

# inventory

`/etc/ansible/hosts` is the default inventory

alternatively, use `ansible.cfg` to specify config file elsewhere or use `-i` in command line

use `ansible -i inv <group> --list-hosts` to list all hosts in inventory file

or use `ansible-navigator inventory -i inventory -m stdout --list`

there are 2 implicit groups - ungrouped and all

# ansible.cfg

`/etc/ansible/ansible.cfg` will be used if no cfg file in current project directory, use `ansible --version` to see which ansible.cfg will be used

if playbook contains settings, they will override the settings from ansible.cfg

command line args have the highest priority

2 sections in cfg file

`[defaults]` sets default settings

`[privilege_escalation]` specifies how ansible runs cmds on managed hosts

a template of `ansible.cfg` file
```
[defaults]
inventory = /path/to/inventory  # specifies the pass to the inv
remote_user = user              # name of usr that logs in on the remote hosts
ask_pass = false                # if prompt for password
deprecation_warning = false
collections_path = ~/collections:~/.ansible/collections:/usr/share/ansible/collections


[privilege_escalation]
become = true                   # if needs automatically switch to the become_user
become_method = sudo            # how to become other user
become_user = root              # target rmeote user
become_ask_pass = false         # if password prompt when escalating

```

default proto to remote host is ssh
    - use ssh-keygen and ssh-copy-id to cp pub key to managed hosts

to connect to windows, use `ansible_connection = winrm` and set `ansible_port = 5986`

sudo is used as default mechanism for priv escalation, can set `ansible ALL=(ALL) NOPASSWD: ALL` forr exam as a drop in file under `sudoers.d`

to be more secured...

use `Defaults timestamp_type=global,timestamp_timeout=60` in `/etc/sudoers`

ensure local account has sudo privileges, when in localhost, default become settings are not used but the account ran ansible cmd is used

# ansible-navigator

ansible-navigator uses an execution environment, which is based on a container img

`ansible-navigator images` to load images

`ansible-navigator inventory -i inventory --list`

`ansible-navigator.yml` setting is used for `ansible-navigator`, prioty rule is the same as 

an template setting

```yaml
ansible-navigator:
  execution-environment:
    image: xxxxx
    pull:
      policy: missing       # will only contact container registry if no container img has been pulled
  playbook-artifact:
    enable: false           # required when a playbook prompts for a password

```
# ansible-doc

use `ansible-doc` to check docs

some useful flags
```
-l  ; list

[-t itemtype] item
```

`ansible-navigator doc -m stdout <mod>` also provides help about the mod

`ansible-navigator --pp never` to never look for a newer container image

# mods

use `ansible-galaxy collections install <some collection> -p /path/to/collections` to install new collection

if no -p flag it is installed to `collections_path` in ansible.cfg, defaults to `~/.ansible/collections:/usr/share/ansible/collections`

config ansible-navigator to read collections from /path/to/collections

# requirements.yml

similar to python

`ansible-galaxy collections install -r collection/requirements.yml -p /path/to/collection`

sample requirements
```yaml
collections:
  - name: something can be git/http/tarball/galaxy
    version: 0.0.0
```

to use collections in navigator must include `-p collections` path in order to make it work

some essential modules

they are all in `ansible.builtin.*`
  - ping
  - service - check service status
  - command - run a cmd but not thru shell
  - shell   - run arbitrary cmd thru shell
  - raw     - run a cmd on a remote host without python
  - copy    - copy file to the managed host

# idempotency
command module is not idempotent... avoid it

use specific module to do one thing, if no suitable, use shell

read [docs](docs.ansible.com)

# playbook

```yaml
---
- name: name
  hosts: ...
  tasks:
  - name: ...
    module:
      key: val
  
...
```

`ansible-playbook play.yml` to run the playbook

`ansible-navigator -pp never playbook.yml` to run playbook thru ansible-navigator

# variables

```yaml
- name: ...
  hosts: all
  vars:
    something: something
  var_files:
    - vars/path/to/file

  .....

  {{ variable }} # to access the variable
  # if var is the first element, double quote it
  "{{ variable }}"

```

use `vars_files` to specify the variable file locations, variable file itself contains `key: value` for variables

check `ansible-doc -t keyword vars_files`

`vars` is the default location for ansible to check variable files

define `host_vars` and `group_vars` to take variable specific to host or groups in inventory

system vars - **Don't** use them as var names
 - hostvars: a dict contains all vars that apply to a specific host
 - inventory_hostname: inventory name of curr host
 - inventory_hostname_short
 - groups: all hosts in inventory and groups these hosts belong to
 - group_names: list of groups the curr host is a part of
 - ansible_check_mode: bool, if play is in check mode
 - ansible_play_batch: active hosts in curr play
 - ansible_play_hosts: same above
 - ansible_version

use `register` to get the returned result from task and store in `register: variable`

# use of vault

`ansible-vault <create|edit|view> <file>` to make changes to an vault file

when using it, use `ansible-playbook --ask-vault-pass ...` to run the play

or use a `vault-pass` file
```sh
echo pass > vault-pass
chmod 600 vault-pass
ansible-playbook --vault-pass-file=/path/to/vault-pass ...
```

# facts 

use `gather_facts` to manage facts

`ansible_facts` holds ansible facts

in setup module every fact starts with `ansible_XXXX` but in play `ansible_facts` are not, `ansible_facts.XXXX`

custom facts are stored in `/etc/ansible/facts` as json or ini style file on the managed hosts, this dir does not exist on hosts so need to create it and copy the fact file into
 - the name of these files must end in `.fact`
 - custom facts must have a [label] to help identify the variables

use `ansible hostname -m setup -a "filter=ansible_local"` to display local facts, to refer to custom facts by playbook, use `ansible_facts.ansible_local`


some of the variable precedence:

command line > task vars > included play vars > play vars > hosts facts > inventory vars > role defaults

use debug to print vars if needed

# task controls

`loop` keyword to iterate thru list of items

```yml
- name: ...
  service:
    name: "{{ item }}"
    ...

  loop:
    - vsftpd
    - httpd

```

`item` is auto set when loop thru a list of items, or loop a list of vars defined
```yml

- name: ...
  vars:
    my_services:
      - vsftpd
      - httpd
  tasks:  
    service:
      name: "{{ item }}"
      ...

    loop: "{{ my_services }}"

```

or a list of dict

```yaml
- name: create users
  hosts: ...
  vars: ....
  tasks:
    user:
      name: "{{ item.name }}"
      group: "{{ item.ugroup }}"

    loop:
      - name: bob
        ugroup: wheel
      - name: anna
        ugroup: users

```

check "Migration from with_X to loop" in ansible doc 

# conditions 

using filter to cast the variable type, eg `when: vg_size | int > 5`
```yaml
- name: create users
  hosts: ...
  vars: ....
  tasks:
    - name: ...
      copy:
        ....

      when: ansible_facts.machine == "x86_64"
```

when supports a list of conditions, this is implicit and so all conditions must be true to procceed,
```yaml
when:
  - cond1
  - cond2
  - ....
```
or use `or`
```yaml
when: >
  (cond1 and cond2)
  or
  (cond3 and cond4)

```

```yaml
vars_prompt:
  - name: varname
    prompt: a prompt string
    private: true/false
```

can also use python func like string.find(substring) in when statement

# handlers

handlers run if triggering task has changed something and after running all tasks in a play

use `meta: flush_handlers` module to run handlers immediately

to run handler, `notify` statement is used from the main task

eg,
```yml
force_handlers: false
tasks:
  - name: copy index
    copy:
      src: /path/to/index.html
      dest: /var/www/html/index.html
    notify:
      - restart_httpd

handlers:
  - name: restart_httpd
    service:
      name: httpd
      state: restarted

```

if one of the next tasks in the play fails, the handler will not run but this behavour can be overwritten using `force_handlers: True` in the playbook

`meta` module can change the behavour of handler
 - `flush_handlers` will run all notified handlers now
 - `refresh_inventory` refreshes inventory at the moment it is called
 - `clear_facts` remove all facts
 - `end_host` ends playbook exec for this host can use condition `when`

# blocks

 - use `block` to define the main tasks to run
 - use `rescue` to define tasks that run if tasks defined in block fail
 - use `always` to define tasks that will always run

eg
```yml
- name: using block and rescue
  hosts: ...
  tasks:
  - name: task1
    block:
      - name: remove file
        shell: rm /notafile
    rescue:
      - name: create file
        shell: touch /isafile
    always:
      - name: write log
        shell: echo abcd >> /log
```

# task failures

 - use `ignore_erros: true/false` in a task or play to ignore failures and tasks will continue to run or not
 - use `failed_when` to specify what to look for in cmd output to recognise a failure

```yml
- name: run something
  command: echo something
  ignore_errors: true
  register: command_output
  failed_when: "'something' in command_output.stdout"

```

or use `fail` module

```yml
ignore_errors: true
tasks:
  - name: run something
    command: echo something
    register: command_output
  - name: fail something
    fail:
      msg: task failed
    when: "'something' in command_output.stdout"

```

# managing changed status

using `changed_when`
 - to allow handler to run when a change would not nornally trigger
 - disable some non idempotent cmds to report changed status

```yml
- name: ...
  hosts: ...
  tasks:
    - name: get time
      command: date
      register: command_output
      changed_when: false   # to suppress the changed status because nothing changed

```

# import and include playbooks
 - include: dynamic, ansible process contents of the included files at the moment that include is reached
   - `include_tasks` to dynamically include task file
   - some feature are not available, `ansible-playbook [--list-tasks | --start-at-task]` will not work
   - cannot trigger a handler in an imported task file from the main task file
 - import: static, ansible preporcess the imported file before play starts
   - must be defined at the beginning of the playbook using `import_playbook`
   - `import_tasks` to statically import a task file in the playbook, it will be included at the location where it is imported

both import and include tasks are just a flat list of tasks like, can also have variables in them

```yml
- name: task1
  ...

- name: task2
  ...

```

# file management
 - lineinfile: manipulate line in file
 - blockinfile: manipulate block in file, use `|` to keep formatting and multiple lines under `block`
 - file
 - stat
 - copy: from local to hosts
 - fetch: from remote to local, copies to `/dest/host/path/from/src`, use `flat` option to overwrite this
 - posix.synchronize: works only when rsync is available
 - posix.patch

# jinja2 templates
.

`template` module renders template to a file, use it when need a file generated with multiple variable based content, for simple files, use (line|block)infile

in `ansible.cfg` set `ansible_managed = {file} managed by Ansible on %d-%m-%Y by {uid}` to display a msg on file generated by the template module. in the template file, include `{{ ansible_managed }}` to display the message

put all tmpl files under `templates` ansible can automatically pick up

example to loop groups in j2 template
```
{% for host in groups['all'] %}
   {{ hostvars[host]['ansible_facts']['default_ipv4']['address'] }}
{% endfor %}

```

# manage selinux file context

- install `policycoreutils-python-utils` via yum module
- install `ansible-galaxy collection install community general`
- set below
```yml
- name: set selinux context
  sefcontext:
    target: /path/to/file
    setype: public_content_rw_t
    state: present
notify:
  - run_restorecon

handlers:
  - name: run_restorecon
    command: restorecon -v /path/to/file
```

# roles

roles are included in tasks, using a `roles:` section in the playbook play header

`ansible-galaxy role install`

`roles_path` is used to store roles installed in ansible.cfg, it is overwritting the default setting, so provide full path like roles:/.../:/..../

similar to ansible collection install, `ansible-galaxy role install -p <path>` to install in an alternative path

precedence
 - `roles` dir in curr proj dir
 - `~/.ansible/roles`
 - `/etc/ansible/roles`
 - `/usr/share/ansible/roles`


`ansible-galaxy [role] search 'docker' --author geerlingguy --platform EL` will search roles containing kw docker, written for usage on RHEL and related by author

`ansible-galaxy [role] info gerrlingguy.docker` shows info about the role, typically `author.role`

`ansible-galaxy [role list]`

`ansible-galaxy [role] remove author.role`

# use `roles/requirements.yml`

```yml
- src: some git repo
  scm: git
  version: ...

- src: file:///path
  name: rolename

- src: https://...
  name: ....

```


`ansible-galaxy role install -r requirements.yml -p <path>`

using `roles` section in the play header

using `import_role` or `include_role` in a task, use `include_role` is a better practice

roles in roles section are executed before the task in the play

use `pre_tasks` to trigger tasks to run before roles

use `post_tasks` to force tasks to run after the roles and `tasks`

`tasks` are not necessary

```yml
- name: something
  hosts: all
  pre_tasks:
    - name: ...
      command: ...

  roles:
    - role: author.role

  post_tasks:
    - name: ...
      command: 

```

# custom roles

create required `roles_path`

using `ansible-galaxy role init myrole` to create the dir struct

complete main.yml

myrole
 - defaults: variables can be overwrite by playbook, define default variables here
 - files: temp files needed by playbook, like used by copy
 - handlers: handlers definition
 - meta: metadata
 - vars: system variable cannot be overwrite
 - tasks: a list of tasks


use `dnf install rhel-system-roles`

rhel system roles with sample playbooks in `/usr/share/doc/rhel-system-roles/`, can copy out required ones and change it to work with it

# manage software

`ansible.builtin.dnf`

to install a package group put a `@` in front of the group name like `@Virtualization Host`


to gather facts about packages, use `ansible.builtin.package_facts`, when gathered package facts are written to the `ansible_facts['packages']` variable 

packages are stored in array, need to find its index, usually is in `[0]` eg, `ansible_facts.packages['bash'][0]`

`yum_repository` module is used to create repository file in the `/etc/yum.repos.d/` dir

if `gpgcheck: true` then `rpm_key` must be used to install gpgkey


# manage users

 - `group` module to create groups, groups should exist before user add them
 - `user` module to manage users
   - `generate_ssh_key: true/false` to generate ssh keys
   - `append: true` to append user to secondary groups like usermod -aG group
   - encryped pass should be generated before pass to user module
 - `known_hosts` updates the /etc/ssh/ssh_know_hosts file with the host key of a managed host
 - `authorized_key` manages authorised keys for user accounts on managed hosts

```yml
authorized_key:
  user: ansible
  key: "{{ lookup('file', './ansiblekey/id_rsa.pub') }}"

```

passwords in `/etc/shadow` consists of 3 parts
 - hashing algo used
 - random salt
 - encrypted hash of password

use `password_hash` filter to encrypted password before used in user module

`ansible localhost -m debug -a "msg={{ 'mypassword' | password_hash('sha512', 'mysalt') }}"` or not to give a salt to use random salt

to validate the syntax of sudo file, use `validate: /usr/sbin/visudo -cf %s`


a playbook to configure sudo

```yml
- name: configure sudoers
  hosts: all
  vars_files:
    - vars/sudovars
  vars_prompt:
    - name: userpass
      prompt: user password
      private: true
  tasks:
    - name: add groups
      group:
        - name: "{{ item.groupname }}"
      loop: "{{ usergroups }}"
    
    - name: add users
      user:
        name: "{{ item.username }}"
        groups: "{{ item.groups }}"
        password: "{{ userpass | password_hash('sha512') }}"
      loop: "{{ users }}"
    
    - name: give user sudo
      template:
        src: sudogroups.j2
        dest: /etc/sudoers.d/sudogroups
        validate: '/usr/sbin/visudo -cf %s'
        mode: 0440


```

in jinja2 template, sudogroups.j2
```
{% for item in usergroups %}
{% if item.sudo %}
%{{ item.name }} ALL=(ALL:ALL) NOPASSWD: ALL
{% endif %}
{% endfor %}

```

vars of groups and users
```yml
usergroups:
  - name: tech
    sudo: false

  - name: tax
    sudo: false

  - name: admin
    sudo: true


usernames:
  - name: mary
    groups:
      - tech
      - tax
  - name: jane
    groups:
      - admin


```

# manage processes
 - service: generic system management
 - systemd: systemd specific
 - command: managing default target
 - reboot: use `reboot_timeout` and `test_command` to verify host is available again

an example playbook of managing target and reboot
```yml
- name: change target and reboot
  hosts: all
  tasks:
    - name: get current target
      command: systemctl get-default
      changed_when: false
      register: default
    
    - name: set default target
      command: systemctl set-default multi-user.target
      when: 'multi-user.target' not in default['stdout']
      notify: reboot_server
  handlers:
    - name: reboot_server
      reboot:
        test_command: uptime
        reboot_timeout: 300

```

 - ansible.posix.at: used for one time job scheduling
 - cron: managing linux crond

example of playbook setup cronjob
```yml
- name: setup cronjob
  hosts: all
  tasks:
    - name: set jobs
      cron:
        name: "write messages"
        minute: "0"
        hour: 8-18
        user: ansible
        job: echo "$(date)" >> /tmp/cron-keepalive
        cron_file: keep_alive_msg
        state: present
    
    - name: remove cronjob
      cron:
        name: "write messages"
        cron_file: keep_alive_msg
        state: absent
        user: ansible

```

# manage storage
 - `ansible.posix.mount` used to mount exsiting fs
 - `community.general.parted` partition
 - `community.general.lvg` manage vol groups
 - `community.general.lvol` manage logical vol
 - `community.general.filesystem` create filesystems on new devices

check `redhat.rhel_system_roles.storage`

sample play book to partition the disk
```yml
- name: create partition
  host: all
  tasks:
    - name: create partition
      parted:
        device: /dev/sdb
        number: 1
        state: present
        part_end: 4GiB
      when: ansible_facts['devices']['sdb'] is defined

```

# manage networks

roles for network system
 - `redhat.rhel_system_roles.network` system role allows for the config of network related settings

some variables must be set
 - `network_provider` should be set to `nm` on RHEL7 or later
 - `network_connections` defines the network connection and its properties

check examples in `/usr/share/doc/rhel-system-roles/network/` or in **`README.md`** would have some more examples

an example `networkvars`

```yml
network_provider: nm
network_connections:
  - name: ensXXX
    type: ethernet
    ip:
      address:
        - 192.168.0.1/24
    zone: external

```

a sample that configures network interface
```yml
- name: Manage interfaces
  hosts: ansible1
  vars:
    network_provider: nm
    network_connections:
      - name: ensXXX
        type: ethernet
        ip:
          address: 
            - fc00::202/64
        zone: external
  roles:
    - rhel-system-roles.network

```


`ansible_facts['interfaces']` in setup module can use `ansible_<device>` to check certain device's config

`ansible.posix.firewalld`

`ansible.builtin.hostname`

# ansible logging

use `log_path` in the settings to specify the log to be written to a file

ansible-navigator creates artefact when it runs, use the following to skip artifact creation
```
ansible-navigator:
  playbook-artifact:
    enable: false

```

using debug module
```yml
debug:
  var: "{{ something }}"
  verbosity: 2
```
this will only be triggered when `-vv` flag is specified


`ansible-lint` check against best practices

use `--check` option to perform check mode will not change anything

set `check_mode: true` within a task to always run that specific task without change

`--diff` will see the differences that would be made by template files on a managed hosts

`ansible-playbook --check --diff playbook.yml` to see the differences made by a template file on a managed host

## connectivity issues

- set `remote_user` in settings
- confirm ssh key setup
- `become` and `become_user` setting
- sudo is configured successfully

use `ansible_host` in inventory to specify how to connect