# jsonf

jsonf formats JSON files in place.

```
$ jsonf your_file.json
```

`your_file.json` is now pretty-printed.

jsonf can also sort JSON array values it encounters.

```
$ jsonf --sort your_file.json
```

Any arrays in `your_file.json` are now in order.

## JSON Lines

jsonf also supports [JSON Lines](https://jsonlines.org/) files (`.jsonl`).
Each line is parsed as a separate JSON value and re-serialized in compact
form, so each record stays on a single line.

```
$ jsonf your_file.jsonl
```

With `--sort`, jsonf sorts arrays inside each record and sorts the lines
themselves by their JSON content.

```
$ jsonf --sort your_file.jsonl
```

## Why?

I frequently deal with big JSON files and pretty-printed files are way
easier to read. They're also much faster to open in a text editor.

You can do something similar with `jq .` or `python -m json.tool`:

```
jq . < your_file.json > tmp.json; mv tmp.json your_file.json
```

but it's not as convenient, and it depends on what is installed on the
system. jsonf is a self-contained binary for just this use case.

(You could avoid the temporary file with `sponge`, but that's another
binary you need to install.)
