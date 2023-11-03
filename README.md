# openai-progenitor

Proof of Concept for the [Support of multipart/form-data and file uploads](https://github.com/oxidecomputer/progenitor/pull/609).

## How to Use

`openai-progenitor` is a reqwest and CLI client for the [OpenAI API](https://platform.openai.com/docs/api-reference).

1. Set API key:
   ```
   export OPENAI_API_KEY=...
   ```

2. Download a square `.png` image, e.g.:
   ```
   curl -L https://avatars.githubusercontent.com/u/8705110 -o /tmp/u.8705110.png
   ```

3. Create a variation of this image:
   ```
   cargo run -- create-image-variation /tmp/u.8705110.png --size 256x256
   ```
   Sample output:
![](https://user-images.githubusercontent.com/6055037/280204775-7576770b-cc86-48b0-9f3b-71ef1e5461d0.png)

## Help

```
$ cargo run -- --help

Usage: cli [COMMAND]

Commands:
  create-image            Creates an image given a prompt.
  create-image-edit       Creates an edited or extended image given an original image and a prompt.
  create-image-variation  Creates a variation of a given image.
  help                    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

## Credits

Huge shoutout to [oxide](https://github.com/oxidecomputer).
