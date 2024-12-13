# About

The tool  render list of hostname from a ssh config file.
By default tool read ssh config file by path `.ssh/config`. If config  has another path use option `-p`.

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
