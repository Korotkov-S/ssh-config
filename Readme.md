# About

The tool  render list of hostname from a ssh config file.
By default tool read ssh config file by path `.ssh/config`. If config  has another path use option `-p`.


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

### TODO
- [x] using default config file
- [x] support a custom path to config file
- [ ] save passwords
- [ ] open config file in a text editor
