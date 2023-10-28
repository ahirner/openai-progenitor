# openai-progenitor

Proof of Concept for the [Support of multipart/form-data and file uploads](https://github.com/oxidecomputer/progenitor/pull/609).

## How to Use

`openai-progenator` is a reqwest and CLI client for the [OpenAI API](https://platform.openai.com/docs/api-reference).

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
![](https://oaidalleapiprodscus.blob.core.windows.net/private/org-gdSgy6cn3o3Tnmopt3F6ZvGC/user-bIMnrpPR45yCcCbGAiU8nSTR/img-DVn8CTKKBuBsq8cNkWqmC6Ky.png?st=2023-10-28T20%3A45%3A51Z&se=2023-10-28T22%3A45%3A51Z&sp=r&sv=2021-08-06&sr=b&rscd=inline&rsct=image/png&skoid=6aaadede-4fb3-4698-a8f6-684d7786b067&sktid=a48cca56-e6da-484e-a814-9c849652bcb3&skt=2023-10-27T22%3A55%3A30Z&ske=2023-10-28T22%3A55%3A30Z&sks=b&skv=2021-08-06&sig=gI7gt%2BLOYtm9zvXm2HrHnLqxQSyvHgBicZJJ6LwJ%2Bmo%3D)

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
