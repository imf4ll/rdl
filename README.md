<div align="center">
    <h3>A tool to download videos from some places</h3>
</div>

## ❗️ Install:
```bash
git clone https://github.com/z3oxs/rdl.git
cd rdl
cargo install
```

or check [releases section](https://github.com/z3oxs/rdl/releases/tag/v0.1.0)

&nbsp;
## 🚀 Usage:
Downloading a video from Twitter:
```bash
rdl -u "https://twitter.com/USER/status/SOMESTATUS"
```

As you can see, you need to only parse the video URL with "-u" flag.

&nbsp;

Download a video and parsing a custom filename and path:
```bash
rdl -u "https://twitter.com/USER/status/SOMESTATUS" -f "some/path/video.mp4"
```

### Available modules
- Twitter
- Facebook
- Rumble

&nbsp;
**If you want to contribute, make a issue requesting a new module or a pull request implementing a new module**
