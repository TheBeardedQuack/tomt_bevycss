# Hot Reload
TOMT_bevycss supports hot reloading of CSS files. Enable the `file_watcher` feature in Bevy.

```toml
bevy = { version = "0.14", features = ["default", "file_watcher"] }
```

Once this feature is enabled, any changes made to a CSS file will be applied immediately, without needing to restart your game.