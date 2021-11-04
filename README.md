# mo

mo is a wrapper around grep. With a search like so:
```bash
$ echo 'some text' > a
$ echo 'some other text' > b
$ mo some .
./b:
[1]: 1:some other text

./a:
[2]: 1:some text
```

`mo 1` would open `b` in your editor at line 1, and `mo 2` would open file a at line 1.

# Status

This currently works with ripgrep, ag, ack, and grep. It tries vscode, emacs, then vi in that order.

This is mostly an exercise to get more comfortable writing Rust.

