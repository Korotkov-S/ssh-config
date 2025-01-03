# About

The tool  render list of hostname from a ssh config file.
By default tool read ssh config file by path `.ssh/config`. If config  has another path use option `-p`.
Automate signin to a host by password. Can automate logic on server.


####
<video src='https://vimeo.com/1039097803' width=180 />

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
