<div align="center">
    <h3>A tool to download videos from some places</h3>
</div>

## ❗️ Install:

**You need ffmpeg to download some videos who separate video from audio**

Linux and MacOS:
```bash
git clone https://github.com/z3oxs/rdl.git && cd rdl
make install
```

&nbsp;

Windows and other systems:
```bash
git clone https://github.com/z3oxs/rdl.git && cd rdl
cargo install --path .
```

or check [releases section](https://github.com/z3oxs/rdl/releases/)

## ❗️ Update:
```bash
git clone https://github.com/z3oxs/rdl.git && cd rdl
make update
```

&nbsp;

Or, if you maintain the source code
```bash
git pull origin master
make update
```

&nbsp;
## 🚀 Usage:
Downloading a video from Twitter:
```bash
rdl -u "https://twitter.com/USER/status/SOMESTATUS"
```

As you can see, you need to only parse the video URL with "-u" flag.

&nbsp;

<div align="center">
### Flags:

| Flag | Description |
|------|-------------|
| --fast | Skip format choosing |

</div>

&nbsp;

Download a video and parsing a custom filename and path:
```bash
rdl -u "https://twitter.com/USER/status/SOMESTATUS" -f "some/path/video.mp4"
```

### Available modules
- Twitter
- Facebook
- Rumble
- Reddit (new)

&nbsp;
**If you want to contribute, make a issue requesting a new module or a pull request implementing a new module**

&nbsp;

## ⚙️ Configuration (Linux and MacOS):

Default template: "config.json"
```json
{
    "path": "path/to/your/folder"
}
```

&nbsp;
<div align="center">

### Available options:
| Option | Description |
|--------|-------------|
| path | Path to default folder |

</div>
