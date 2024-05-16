# pushenv
A CLI utility that reads a .env file before starting a process

## Example usage
```bash
pushenv -- echo $SOME_VAR
```

```bash
pushenv some.env.file -- echo $SOME_VAR
```
