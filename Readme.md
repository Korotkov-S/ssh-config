# About


<a href="https://github.com/Korotkov-S/ssh-config"><img src="https://github.com/user-attachments/assets/af6b14b6-0a5d-4cfe-b8b8-773407455fdb" height="150" width="150" ></a>


The tool  render list of hostname from a ssh config file.
By default tool read ssh config file by path `.ssh/config`. If config  has another path use option `-p`.
Automate signin to a host by password. Can automate logic on server.


####
https://github.com/user-attachments/assets/dbfd4e9f-57bf-4888-bf10-47c9e739ec4f



### Example

Show list of hostname from `.ssh/config` file
```bash
ssh-config
```

Show list of hostname from `./custom/path/to/config` file
```bash
ssh-config -p './custom/path/to/config'
```

Create new password by host name
```bash
ssh-config password create
```

Delete a password by host name
```bash
ssh-config password delete
```

### TODO
- [x] using default config file
- [x] support a custom path to config file
- [x] save passwords
- [ ] sign in by host name without interactive mode
- [ ] open config file in a text editor
