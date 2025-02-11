# Rerun changelog


## [Unreleased](https://github.com/rerun-io/rerun/compare/latest...HEAD)

## [0.7.0](https://github.com/rerun-io/rerun/compare/v0.6.0...v0.7.0) - improved transforms, better color mapping, bug & doc fixes - 2023-06-16

[Rerun](https://www.rerun.io/) is an easy-to-use visualization toolbox for computer vision and robotics.

* Python: `pip install rerun-sdk`
* Rust: `cargo add rerun` and `cargo install rerun-cli`
* Online demo: <https://app.rerun.io/version/0.7.0/index.html>

### Overview & Highlights

While we're working on significant updates around interfaces and customizability,
here's a smaller release packed with useful improvements 🎉

* Much more powerful transformation logging
  * any affine transforms works now!
  * supports many more formats and shows them in the viewer as-is
* Better color mapping range detection for images and tensors
* Many small improvements to samples & documentation

### In detail

#### 🐍 Python SDK
- Improved 3D transform ingestion & affine transform support [#2102](https://github.com/rerun-io/rerun/pull/2102)
- Normalize Python typing syntax to 3.8+ [#2361](https://github.com/rerun-io/rerun/pull/2361)
- Enforce `from __future__ import annotations` in Python files [#2377](https://github.com/rerun-io/rerun/pull/2377)
- Add `jpeg_quality` parameter to `log_image` [#2418](https://github.com/rerun-io/rerun/pull/2418)

#### 🦀 Rust SDK
- Improved 3D transform ingestion & affine transform support [#2102](https://github.com/rerun-io/rerun/pull/2102)
- `impl Copy for Arrow3D`. [#2239](https://github.com/rerun-io/rerun/pull/2239) (thanks [@kpreid](https://github.com/kpreid)!)

#### 🪳 Bug Fixes
- Stable image order, fixing flickering [#2191](https://github.com/rerun-io/rerun/pull/2191)
- Fix double clicking objects no longer focusing the camera on them [#2227](https://github.com/rerun-io/rerun/pull/2227)
- Fix off-by-half pixel error in textured rectangle shader [#2294](https://github.com/rerun-io/rerun/pull/2294)
- Update wgpu-hal to 0.16.1 to fix mobile Safari [#2296](https://github.com/rerun-io/rerun/pull/2296)
- Fix some browsers failing due to 8k texture requirement, pick always highest available now [#2409](https://github.com/rerun-io/rerun/pull/2409)
- Fix visibility toggles for time series not working [#2444](https://github.com/rerun-io/rerun/pull/2444)

#### 🌁 Viewer Improvements
- Time panel now always talks about "events" instead of "messages" [#2247](https://github.com/rerun-io/rerun/pull/2247)
- Automatically determine image/tensor color mapping & need for sRGB decoding [#2342](https://github.com/rerun-io/rerun/pull/2342)

#### 🚀 Performance Improvements
- Optimization: avoid a memory allocation when padding RGB u8 to RGBA [#2345](https://github.com/rerun-io/rerun/pull/2345)

#### 🧑‍🏫 Examples
- Example of how to embed the Rerun Viewer inside your own GUI (+ ergonomic improvements) [#2250](https://github.com/rerun-io/rerun/pull/2250)
- Objectron Rust example: install `protoc` for the user [#2280](https://github.com/rerun-io/rerun/pull/2280)
- Remove weird-looking argument parsing in examples [#2398](https://github.com/rerun-io/rerun/pull/2398)
- Fix `tracking_hf example`: put scaled thing under its own root entity [#2419](https://github.com/rerun-io/rerun/pull/2419)
- Clean up our examples [#2424](https://github.com/rerun-io/rerun/pull/2424)
- New face detection example based on MediaPipe [#2360](https://github.com/rerun-io/rerun/pull/2360)
- Update web examples [#2420](https://github.com/rerun-io/rerun/pull/2420)
- Update titles and tags for examples with real data [#2416](https://github.com/rerun-io/rerun/pull/2416)

#### 📚 Docs
- Merge `rerun-docs` repository into this monorepo [#2284](https://github.com/rerun-io/rerun/pull/2284)
- Add manifest + readmes to examples [#2309](https://github.com/rerun-io/rerun/pull/2309)
- Fix and clean up BUILD.md [#2319](https://github.com/rerun-io/rerun/pull/2319)
- Link to `/examples` in PR description [#2320](https://github.com/rerun-io/rerun/pull/2320)
- Make examples setup a separate page [#2323](https://github.com/rerun-io/rerun/pull/2323)
- Add `site_url` to `mkdocs.yml` [#2326](https://github.com/rerun-io/rerun/pull/2326)
- Add `log_cleared` to the common index [#2400](https://github.com/rerun-io/rerun/pull/2400)
- Use forked `mkdocs-redirects` [#2404](https://github.com/rerun-io/rerun/pull/2404)
- Add support for classes to generated python common API index [#2401](https://github.com/rerun-io/rerun/pull/2401)
- Added support for creating multi-resolution stacks with upload_image.py [#2411](https://github.com/rerun-io/rerun/pull/2411)
- Document annotation context in manual [#2453](https://github.com/rerun-io/rerun/pull/2453)

#### 🕸️ Web
- Update `wasm-bindgen` to 0.2.87 [#2406](https://github.com/rerun-io/rerun/pull/2406)
- When loading on web, match style and show a progress indicator while wasm is loading [#2421](https://github.com/rerun-io/rerun/pull/2421)

#### 📈 Analytics
- Add crash retriever script [#2168](https://github.com/rerun-io/rerun/pull/2168)

#### 🧑‍💻 Dev-experience
- Image uploader script [#2164](https://github.com/rerun-io/rerun/pull/2164)
- Replace `wasm-bindgen-cli` with library `wasm-bindgen-cli-support` [#2257](https://github.com/rerun-io/rerun/pull/2257)
- Fix manual release/dispatch workflows [#2230](https://github.com/rerun-io/rerun/pull/2230)
- Add instructions on how to fix weird `gsutil` crash [#2278](https://github.com/rerun-io/rerun/pull/2278)
- Link to preview of latest commit in PR body [#2287](https://github.com/rerun-io/rerun/pull/2287)
- CI: Retry `linkinator` [#2299](https://github.com/rerun-io/rerun/pull/2299)
- Remove long dead code python unit test [#2356](https://github.com/rerun-io/rerun/pull/2356)
- Added gcloud project name to `upload_image.py` [#2381](https://github.com/rerun-io/rerun/pull/2381)
- Fix typo in `run_all.py` [#2441](https://github.com/rerun-io/rerun/pull/2441)
- Small changelog improvements [#2442](https://github.com/rerun-io/rerun/pull/2442)
- Minor fixes/improvements of `upload_image.py` [#2449](https://github.com/rerun-io/rerun/pull/2449)
- Improve changelog generator [#2447](https://github.com/rerun-io/rerun/pull/2447)

#### 🗣 Refactors
- Centralize freestanding store helpers [#2153](https://github.com/rerun-io/rerun/pull/2153)

#### 📦 Dependencies
- Update `xml-rs` v0.8.13 -> v0.8.14 [#2425](https://github.com/rerun-io/rerun/pull/2425)
- Update pip package `requests` to 2.31 with bug fix [#2426](https://github.com/rerun-io/rerun/pull/2426)


## [0.6.0](https://github.com/rerun-io/rerun/compare/v0.5.1...v0.6.0) - 3D in 2D and SDK batching - 2023-05-26

### Overview & Highlights

- You can now show 3D objects in 2D views connected by Pinhole transforms [#2008](https://github.com/rerun-io/rerun/pull/2008)
- You can quickly view images and meshes with `rerun mesh.obj image.png` [#2060](https://github.com/rerun-io/rerun/pull/2060)
- The correct to install the `rerun` binary is now with `cargo install rerun-cli` [#2183](https://github.com/rerun-io/rerun/pull/2183)
- `native_viewer` is now an opt-in feature of the `rerun` library, leading to faster compilation times [#2064](https://github.com/rerun-io/rerun/pull/2064)
- Experimental WebGPU support [#1965](https://github.com/rerun-io/rerun/pull/1965)
- SDK log calls are now batched on the wire, saving CPU time and bandwidth

### In Detail

#### 🐍 Python SDK
- ⚠️ BREAKING: You must now call `rr.init` if you want logging to work.
- ⚠️ BREAKING: `set_enabled` has been removed.
  In order to disable logging at runtime, call `set_global_data_recording(None)`.
  See also [the doc section on this topic](https://www.rerun.io/docs/reference/sdk-logging-controls#dynamically-turn-logging-onoff).
- `log_mesh_file`: accept either path or bytes [#2098](https://github.com/rerun-io/rerun/pull/2098)
- Add `draw_order` to 2D primitives [#2138](https://github.com/rerun-io/rerun/pull/2138)
- Add `rr.version()` [#2084](https://github.com/rerun-io/rerun/pull/2084)
- Add an experimental text-box component and logtype [#2011](https://github.com/rerun-io/rerun/pull/2011)
- Fix a race condition for notebooks [#2073](https://github.com/rerun-io/rerun/pull/2073)
- Redesign multi-recording & multi-threading [#2061](https://github.com/rerun-io/rerun/pull/2061)
- More robust wait for exit condition during `.serve()` [#1939](https://github.com/rerun-io/rerun/pull/1939)
- SDK batching/revamp 3: sunset `PythonSession` [#1985](https://github.com/rerun-io/rerun/pull/1985)

#### 🦀 Rust SDK
- ⚠️ BREAKING: `set_enabled` has been removed.
  In order to disable logging at runtime, create a no-op recording via `RecordingStream::disabled()`.
  See also [the doc section on this topic](https://www.rerun.io/docs/reference/sdk-logging-controls#dynamically-turn-logging-onoff).
- ⚠️ BREAKING: `Session` has been replaced by `RecordingStream` [#1983](https://github.com/rerun-io/rerun/pull/1983)
- ⚠️ BREAKING: `native_viewer` is now an opt-in feature of the `rerun` library [#2064](https://github.com/rerun-io/rerun/pull/2064)
- Rust SDK: bring back support for implicit splats [#2059](https://github.com/rerun-io/rerun/pull/2059)
- Introduce a 2D `DrawOrder` component [#2056](https://github.com/rerun-io/rerun/pull/2056)
- Add `Tensor::from_image_file` and `Tensor::from_image_bytes` [#2097](https://github.com/rerun-io/rerun/pull/2097)
- Redesign multi-recording & multi-threading [#2061](https://github.com/rerun-io/rerun/pull/2061)

#### 🌁 Viewer Improvements
- Support projecting 3D entities in 2D views [#2008](https://github.com/rerun-io/rerun/pull/2008)
- Set Rerun viewer native app icon using eframe [#1976](https://github.com/rerun-io/rerun/pull/1976)
- Use `alt` key again for rolling camera in 3d views [#2066](https://github.com/rerun-io/rerun/pull/2066)
- Show tensors shaped [H, W, 1, 1] as images (and more!) [#2075](https://github.com/rerun-io/rerun/pull/2075)
- Show meshes and images with `rerun foo.obj bar.png` [#2060](https://github.com/rerun-io/rerun/pull/2060)
- Don't persist blueprints for unknown apps [#2165](https://github.com/rerun-io/rerun/pull/2165)

#### 🪳 Bug Fixes
- Fix hover/select highlights when picking single points in a scene with multiple point clouds [#1942](https://github.com/rerun-io/rerun/pull/1942)
- Fix crash for missing class ids causing zero sized texture [#1947](https://github.com/rerun-io/rerun/pull/1947)
- Handle leaking of prerelease into alpha version [#1953](https://github.com/rerun-io/rerun/pull/1953)
- Fix incorrect memory usage stats for destroyed on-creation-mapped buffers [#1963](https://github.com/rerun-io/rerun/pull/1963)
- Fix: don't starve web-socket decoding task [#1977](https://github.com/rerun-io/rerun/pull/1977)
- When hovering a 3D view in the presence of images, fix previously incorrect depth shown in 2D view [#2009](https://github.com/rerun-io/rerun/pull/2009)
- Fix: use the mac icon on mac [#2023](https://github.com/rerun-io/rerun/pull/2023)
- SDK batching/revamp 2.2: homegrown arrow size estimation routines [#2002](https://github.com/rerun-io/rerun/pull/2002)
- Fix twice as wide alpha-to-coverage edge on circles, leading to artifacts [#2053](https://github.com/rerun-io/rerun/pull/2053)
- Bugfix: allow hovered items to be clicked to set selection [#2057](https://github.com/rerun-io/rerun/pull/2057)
- Detect, warn and gracefully handle corrupt cells in `lookup_arrow` [#2055](https://github.com/rerun-io/rerun/pull/2055)
- Fix failing dependency install of mesh_to_sdf [#2081](https://github.com/rerun-io/rerun/pull/2081)
- Stop playback when we reach the end of the data [#2085](https://github.com/rerun-io/rerun/pull/2085)
- `tornado` >6.1 doesn't work with recent `jupyter` [#2092](https://github.com/rerun-io/rerun/pull/2092)
- Premultiply alpha of RGBA u8 images [#2095](https://github.com/rerun-io/rerun/pull/2095)
- Fix premature pausing when reaching end of still-streaming stream [#2106](https://github.com/rerun-io/rerun/pull/2106)
- 2D layering fixes [#2080](https://github.com/rerun-io/rerun/pull/2080)
- Fix depth precision issues on WebGL due to different NDC space [#2123](https://github.com/rerun-io/rerun/pull/2123)
- Fix flushing race in new multi-recording SDK [#2125](https://github.com/rerun-io/rerun/pull/2125)
- Web viewer: catch and show panic messages that happens at startup [#2157](https://github.com/rerun-io/rerun/pull/2157)
- Don't early-exit on non-pinhole transforms when looking up cameras [#2194](https://github.com/rerun-io/rerun/pull/2194)
- Mitigate depth offset precision issues on web [#2187](https://github.com/rerun-io/rerun/pull/2187)
- Fix colormaps [#2204](https://github.com/rerun-io/rerun/pull/2204)
- Fix annotation images sometimes drawn in the background [#1933](https://github.com/rerun-io/rerun/pull/1933)
- Fix hovering depth clouds [#1943](https://github.com/rerun-io/rerun/pull/1943)
- Fix incorrect 2D camera for scenes with negative 2D coordinates [#2051](https://github.com/rerun-io/rerun/pull/2051)
- Fix web depth/projection regression, causing incorrect rendering on all 3D scenes [#2170](https://github.com/rerun-io/rerun/pull/2170)

#### 🚀 Performance Improvements
- SDK batching/revamp 1: impl `DataTableBatcher` [#1980](https://github.com/rerun-io/rerun/pull/1980)
- Upgrade arrow2/convert and use native buffers for the tensor u8 types [#1375](https://github.com/rerun-io/rerun/pull/1375)
- Use the same RRD encoding for the SDK comms as for everything else [#2065](https://github.com/rerun-io/rerun/pull/2065)
- Optimize GLTF/GLB texture loading in debug builds [#2096](https://github.com/rerun-io/rerun/pull/2096)
- Premultiply the alpha on the GPU [#2190](https://github.com/rerun-io/rerun/pull/2190)
- Switch compression algorithm from zstd to lz4 [#2112](https://github.com/rerun-io/rerun/pull/2112)
- Support RRD streams with and without compression. Turn off for SDK comms [#2219](https://github.com/rerun-io/rerun/pull/2219)

#### 🧑‍🏫 Examples
- Join threads at end of multi-threading example [#1934](https://github.com/rerun-io/rerun/pull/1934)
- Add argument parsing to the rerun_demo [#1925](https://github.com/rerun-io/rerun/pull/1925)
- Use zipfile python library instead of `unzip` command in arkitscene [#1936](https://github.com/rerun-io/rerun/pull/1936)
- Fix backslashes in arkitscene rigid transformation path [#1938](https://github.com/rerun-io/rerun/pull/1938)
- Fix mp_pose example 2D points having incorrectly interpreted depth [#2034](https://github.com/rerun-io/rerun/pull/2034)
- SDK batching/revamp 2.1: `clock` example for Rust [#2000](https://github.com/rerun-io/rerun/pull/2000)
- Add `scripts/run_all.py` [#2046](https://github.com/rerun-io/rerun/pull/2046)
- Check `examples/python/requirements.txt` in CI [#2063](https://github.com/rerun-io/rerun/pull/2063)
- Fix glb mesh data set downloads [#2100](https://github.com/rerun-io/rerun/pull/2100)
- Add more examples to https://app.rerun.io/ [#2062](https://github.com/rerun-io/rerun/pull/2062)

#### 🖼 UI Improvements
- Update egui to latest and wgpu to 0.16 [#1958](https://github.com/rerun-io/rerun/pull/1958)
- Add keyboard shortcut for "Follow", and stop following on "Restart" [#1986](https://github.com/rerun-io/rerun/pull/1986) (thanks [@h3mosphere](https://github.com/h3mosphere)!)
- Improve UI for keypoint and class-ids of annotations contexts [#2071](https://github.com/rerun-io/rerun/pull/2071)
- Improvements to memory measurements and reporting [#2069](https://github.com/rerun-io/rerun/pull/2069)
- Switch from `egui_dock` to `egui_tiles` [#2082](https://github.com/rerun-io/rerun/pull/2082)
- Allow horizontal scrolling in blueprint panel [#2114](https://github.com/rerun-io/rerun/pull/2114)
- Nicer (& fixed up) help texts for space views [#2070](https://github.com/rerun-io/rerun/pull/2070)
- Allow dragging time cursor in plots [#2115](https://github.com/rerun-io/rerun/pull/2115)

#### 🕸️ Web
- Set the GC limit to 2.5GB on web [#1944](https://github.com/rerun-io/rerun/pull/1944)
- Better crash reports on Web, plus WebGPU support detection [#1975](https://github.com/rerun-io/rerun/pull/1975)
- Work around https://github.com/sebcrozet/instant/issues/49 [#2094](https://github.com/rerun-io/rerun/pull/2094)
- Update `wasm-bindgen` to 0.2.86 [#2161](https://github.com/rerun-io/rerun/pull/2161)

#### 🎨 Renderer Improvements
- Full (experimental) WebGPU support [#1965](https://github.com/rerun-io/rerun/pull/1965)
- Depth offset for lines & points [#2052](https://github.com/rerun-io/rerun/pull/2052)
- Update to wgpu 0.16.1 [#2205](https://github.com/rerun-io/rerun/pull/2205)

#### 🚜 Refactors
- Replace complex uses of `query_entity_with_primary` with `query_latest_single` [#2137](https://github.com/rerun-io/rerun/pull/2137)
- Make selection state independent of blueprint [#2035](https://github.com/rerun-io/rerun/pull/2035)
- Remove unused MeshSourceData [#2036](https://github.com/rerun-io/rerun/pull/2036)
- Move selection state into an independent crate, re_viewer_context [#2037](https://github.com/rerun-io/rerun/pull/2037)
- Move item-ui to separate module, move AppOptions to re_viewer_context [#2040](https://github.com/rerun-io/rerun/pull/2040)
- Move `Caches` to `re_viewer_ctx` and make it generic [#2043](https://github.com/rerun-io/rerun/pull/2043)
- Move time control to re_viewer_context [#2045](https://github.com/rerun-io/rerun/pull/2045)
- Move `ViewerContext` & `ComponentUiRegistry` to `viewer_context` [#2047](https://github.com/rerun-io/rerun/pull/2047)
- Move data ui to new `re_data_ui` crate [#2048](https://github.com/rerun-io/rerun/pull/2048)
- Use instant for `Time::now()` [#2090](https://github.com/rerun-io/rerun/pull/2090)
- Move from `instant` -> `web_time` [#2093](https://github.com/rerun-io/rerun/pull/2093)
- "namespace" flag parameters for linestrip & point cloud shader flags [#2033](https://github.com/rerun-io/rerun/pull/2033)

#### ✨ Other Enhancement
- Update minimum supported Rust version to `1.69.0` [#1935](https://github.com/rerun-io/rerun/pull/1935)
- Allow users to select the bind address (ip) to use with `--bind` [#2159](https://github.com/rerun-io/rerun/pull/2159)

#### 🧑‍💻 Dev-experience
- Suggest users open an issue on crash, and other fixes [#1993](https://github.com/rerun-io/rerun/pull/1993)
- Lint error names in `map_err` [#1948](https://github.com/rerun-io/rerun/pull/1948)
- New dispatch-only workflow for running the lint-job [#1950](https://github.com/rerun-io/rerun/pull/1950)
- Move clippy_wasm/clippy.toml to under scripts [#1949](https://github.com/rerun-io/rerun/pull/1949)
- Fix run-wasm crash on trying to wait for server [#1959](https://github.com/rerun-io/rerun/pull/1959)
- Introduce new reusable workflow jobs and cleanup manual trigger [#1954](https://github.com/rerun-io/rerun/pull/1954)
- Use new CI workflows on pull-request [#1955](https://github.com/rerun-io/rerun/pull/1955)
- Try making pull-request workflows non-concurrent [#1970](https://github.com/rerun-io/rerun/pull/1970)
- Another attempt to make jobs non-concurrent on a per-PR basis [#1974](https://github.com/rerun-io/rerun/pull/1974)
- If there's a `{{ pr-build-summary }}` in the PR description, update it. [#1971](https://github.com/rerun-io/rerun/pull/1971)
- Run the cube notebook on PR [#1972](https://github.com/rerun-io/rerun/pull/1972)
- Add ability to manually run a web build to upload to an adhoc name [#1966](https://github.com/rerun-io/rerun/pull/1966)
- Limit ipython to 8.12 in the jupyter example [#2001](https://github.com/rerun-io/rerun/pull/2001)
- New manual job to publish a release based on pre-built wheels [#2025](https://github.com/rerun-io/rerun/pull/2025)
- Use the correct rust analyzer settings [#2028](https://github.com/rerun-io/rerun/pull/2028)
- New helper for sticking Serde-encodable data into arrow [#2004](https://github.com/rerun-io/rerun/pull/2004)
- Fix `taplo-cli` failing to install [#2068](https://github.com/rerun-io/rerun/pull/2068)
- `run_all.py`: add `--fast`, `--separate`, and `--close` [#2054](https://github.com/rerun-io/rerun/pull/2054)
- Remove `Clipboard::set_text` [#2078](https://github.com/rerun-io/rerun/pull/2078)
- run_all.py: print output on sequential run failure [#2079](https://github.com/rerun-io/rerun/pull/2079)
- Use the american spelling of "gray" [#2099](https://github.com/rerun-io/rerun/pull/2099)
- Make sure `rerun/rerun_py/re_viewer` build info is updated on each build [#2087](https://github.com/rerun-io/rerun/pull/2087)
- Fix setup scripts for Mac M1/MacPort configuration [#2169](https://github.com/rerun-io/rerun/pull/2169) (thanks [@abey79](https://github.com/abey79)!)
- Better error messages in `build.rs` [#2173](https://github.com/rerun-io/rerun/pull/2173)
- `cargo install rerun-cli` [#2183](https://github.com/rerun-io/rerun/pull/2183)
- Fix `cargo test` [#2199](https://github.com/rerun-io/rerun/pull/2199)
- Fix run all for new rust-cli target & add rerun-web alias for quick running of the web player [#2203](https://github.com/rerun-io/rerun/pull/2203)

#### 🤷‍♂️ Other
- Fix secret in dispatch_lint.yml [4848f98f2605a3caf9b7695273e0871efa2d44c8](https://github.com/rerun-io/rerun/commit/4848f98f2605a3caf9b7695273e0871efa2d44c8)
- Only maintain a single manual-dispatch job for testing workflows [98f7de3b52b0fea6abe364f9d0ce0bd4c459caf1](https://github.com/rerun-io/rerun/commit/98f7de3b52b0fea6abe364f9d0ce0bd4c459caf1)
- Add other build parameterizations to manual_dispatch.yml [dbdf275eaf17220d14811dc34b69b6a76e948e73](https://github.com/rerun-io/rerun/commit/dbdf275eaf17220d14811dc34b69b6a76e948e73)
- Use proper if gates on the manual_dispatch.yml jobs [9ad62011678caaed04260ba160763e24e64a7402](https://github.com/rerun-io/rerun/commit/9ad62011678caaed04260ba160763e24e64a7402)
- Add ability to save cache to manual_disaptch.yml [5c61b37a1bc40f1a223c370b3b69b08654aada47](https://github.com/rerun-io/rerun/commit/5c61b37a1bc40f1a223c370b3b69b08654aada47)
- Standard case of inputs [2729c71f1ba9f7cdbe64adc3c610caf9464324e4](https://github.com/rerun-io/rerun/commit/2729c71f1ba9f7cdbe64adc3c610caf9464324e4)
- Add manual step for packaging to 'manual_dispatch.yml' [a3178e6143c068175b477cb236f2ba2477e083ea](https://github.com/rerun-io/rerun/commit/a3178e6143c068175b477cb236f2ba2477e083ea)
- New workflow_dispatch for building wheels for a PR [3bc2cb73ece98f914254221ce0ea129015834f59](https://github.com/rerun-io/rerun/commit/3bc2cb73ece98f914254221ce0ea129015834f59)
- Rename build_wheels_for_pr.yml -> manual_build_wheels_for_pr.yml [778c4d363b3814aeb777d07bfa63f081bc1dac32](https://github.com/rerun-io/rerun/commit/778c4d363b3814aeb777d07bfa63f081bc1dac32)
- New manual workflow for running benches [840a127e3a74c3520a27c0b19eb1d3d9a7255b07](https://github.com/rerun-io/rerun/commit/840a127e3a74c3520a27c0b19eb1d3d9a7255b07)
- New manual workflow for adhoc web builds [01080d6509e94fd2e2d3c4ff05beb0970ebe0b6e](https://github.com/rerun-io/rerun/commit/01080d6509e94fd2e2d3c4ff05beb0970ebe0b6e)
- Fix name of on_push_main.yml [bf5f63344663b3ebfc74f847db696a749b3e716c](https://github.com/rerun-io/rerun/commit/bf5f63344663b3ebfc74f847db696a749b3e716c)
- Fix usage of long commit in generate_prerelease_pip_index.py [579ce91556d6dd3cb9e6bd46971a7b6db6e42cdd](https://github.com/rerun-io/rerun/commit/579ce91556d6dd3cb9e6bd46971a7b6db6e42cdd)
- Jobs with duplicated instances still need separate concurrency keys based on platform [0ad19980be99cb2f669d38c2f1410a38206cbe74](https://github.com/rerun-io/rerun/commit/0ad19980be99cb2f669d38c2f1410a38206cbe74)
- New manual CI job for creating a release [fb2d41af5ec089f6c7583629eda3fb332e420488](https://github.com/rerun-io/rerun/commit/fb2d41af5ec089f6c7583629eda3fb332e420488)
- Version check needs to run in bash [6feca463d21ea03538889df08064b6974edb1fd2](https://github.com/rerun-io/rerun/commit/6feca463d21ea03538889df08064b6974edb1fd2)
- Update changelog with 0.5.1 release notes [40fc2fd7d61689100dc40bfe59e4ddfbcc819c7d](https://github.com/rerun-io/rerun/commit/40fc2fd7d61689100dc40bfe59e4ddfbcc819c7d)
- `RecordingStream`: automatic `log_tick` timeline [#2072](https://github.com/rerun-io/rerun/pull/2072)
- Add support for `f16` tensors [#1449](https://github.com/rerun-io/rerun/pull/1449)
- Make `RecordingId` a string [#2088](https://github.com/rerun-io/rerun/pull/2088)
- Update to latest `egui_tiles` [#2091](https://github.com/rerun-io/rerun/pull/2091)
- Make every `RecordingId` typed and preclude the existence of 'Defaults' [#2110](https://github.com/rerun-io/rerun/pull/2110)
- Add unit test of `re_smart_channel` `is_connected` [#2119](https://github.com/rerun-io/rerun/pull/2119)
- `BeingRecordingMsg` -> `SetRecordingInfo` [#2149](https://github.com/rerun-io/rerun/pull/2149)
- Update egui and eframe [#2184](https://github.com/rerun-io/rerun/pull/2184)
- Update to egui 0.22 [#2195](https://github.com/rerun-io/rerun/pull/2195)
- Simpler SIGINT handling [#2198](https://github.com/rerun-io/rerun/pull/2198)
- `cargo update` [#2196](https://github.com/rerun-io/rerun/pull/2196)
- Replace `ctrlc` crate with `tokio` [#2207](https://github.com/rerun-io/rerun/pull/2207)
- Comment indicating blueprints aren't available in 0.6 [b6c05776ab48e759370d6fed645ffd0ea68ec8c0](https://github.com/rerun-io/rerun/commit/b6c05776ab48e759370d6fed645ffd0ea68ec8c0)


## [0.5.1](https://github.com/rerun-io/rerun/compare/v0.5.1...v0.5.0) - Patch Release - 2023-05-01

### Overview & Highlights
This Release fixes a few small bugs on top of the v0.5.0 release.

### In Detail
* Bump hyper version due to RUSTSEC-2023-0034 [#1951](https://github.com/rerun-io/rerun/pull/1951)
* Round to nearest color_index when doing color mapping [#1969](https://github.com/rerun-io/rerun/pull/1969)
* Use an sRGB-correct gray gradient when displaying grayscale images [#2014](https://github.com/rerun-io/rerun/pull/2014)
* Don't use console.error [#1984](https://github.com/rerun-io/rerun/pull/1984)
* Fix failure to save files when split table contains no data [#2007](https://github.com/rerun-io/rerun/pull/2007)


## [0.5.0](https://github.com/rerun-io/rerun/compare/v0.4.0...v0.5.0) - Jupyter MVP, GPU-based picking & colormapping, new datastore! - 2023-04-20

### Overview & Highlights

This new release adds MVP support for embedding Rerun in Jupyter notebooks, and brings significant performance improvements across all layers of the stack.

* Rerun can now be embedded in Jupyter notebooks
    * Tested with Jupyter Notebook Classic, Jupyter Lab, VSCode & Google Colab; checkout our [How-to guide](https://www.rerun.io/docs/howto/notebook)
    * Try it out live on [Google Colab](https://colab.research.google.com/drive/1R9I7s4o6wydQC_zkybqaSRFTtlEaked_?usp=sharing)
* All colormapping tasks are now done directly on the GPU
    * This yields _very significant_ performance improvements for colormapping heavy workload (e.g. segmentation)
    * Try it out in our new [`segment_anything` example](https://www.rerun.io/docs/getting-started/examples#segment-anything) that shows off the latest models from Meta AI
* GPU picking & hovering now works with all of our primitives, including meshes & depth clouds
    * This fixes all the shortcomings of the previous CPU-based system
    * Rerun's automatic backprojection of depth textures ("depth clouds") is now feature complete
    * Try it out in our updated [`nyud` example](https://www.rerun.io/docs/getting-started/examples#nyud)
* Our datastore has been completely revamped to more closely match our latest data model
    * This yields _very significant_ performance improvements for workloads with many events
    * Checkout [this post](https://github.com/rerun-io/rerun/issues/1619#issuecomment-1511046649) for a detailed walkthrough of the changes

### In Detail

#### 🐍 Python SDK
- Document that we also accept colors in 0-1 floats [#1740](https://github.com/rerun-io/rerun/pull/1740)
- Don't initialize an SDK session if we are only going to be launching the app [#1768](https://github.com/rerun-io/rerun/pull/1768)
- Allow torch tensors for `log_rigid3` [#1769](https://github.com/rerun-io/rerun/pull/1769)
- Always send `recording_id` as part of `LogMsg` [#1778](https://github.com/rerun-io/rerun/pull/1778)
- New `reset_time` API [#1826](https://github.com/rerun-io/rerun/pull/1826) [#1854](https://github.com/rerun-io/rerun/pull/1854)
- Always flush when we remove a sink [#1830](https://github.com/rerun-io/rerun/pull/1830)
- More robust wait for exit condition during .serve() [#1939](https://github.com/rerun-io/rerun/pull/1939)

#### 🪳 Bug Fixes
- Fix broken outlines (hover/select effect) for lines [#1724](https://github.com/rerun-io/rerun/pull/1724)
- Fix logged obb being displayed with half of the requested size [#1749](https://github.com/rerun-io/rerun/pull/1749) (thanks [@BenjaminDev](https://github.com/BenjaminDev)!)
- Fix `log_obb` usage [#1761](https://github.com/rerun-io/rerun/pull/1761)
- Always create the `log_time` timeline [#1763](https://github.com/rerun-io/rerun/pull/1763)
- Fix undo/redo selection shortcut/action changing selection history without changing selection [#1765](https://github.com/rerun-io/rerun/pull/1765)
- Fix various crashes [#1780](https://github.com/rerun-io/rerun/pull/1780)
- Fix crash when trying to do picking on depth clouds [d94ca3dd35e73e1984ccb969d0c7abd0d3e0faa9](https://github.com/rerun-io/rerun/commit/d94ca3dd35e73e1984ccb969d0c7abd0d3e0faa9)
- ci: fix benchmarks [#1799](https://github.com/rerun-io/rerun/pull/1799)
- ci: fix `cargo deny` [#1806](https://github.com/rerun-io/rerun/pull/1806)
- Fix "too many points" crash [#1822](https://github.com/rerun-io/rerun/pull/1822)
- Allow re-use of `RowId`s if no conflict is possible [#1832](https://github.com/rerun-io/rerun/pull/1832)
- Reduce memory used by staging belts on Web [#1836](https://github.com/rerun-io/rerun/pull/1836)
- Test and handle all tensor dtypes as images [#1840](https://github.com/rerun-io/rerun/pull/1840)
- Fix the python build when running without `web_viewer` enabled [#1856](https://github.com/rerun-io/rerun/pull/1856)
- Error instead of `expect` inside `msg_encode` [#1857](https://github.com/rerun-io/rerun/pull/1857)
- Fix shutdown race condition in `re_sdk_comms` client [#1861](https://github.com/rerun-io/rerun/pull/1861)
- Fix broken instance picking in presence of images [#1876](https://github.com/rerun-io/rerun/pull/1876)
- Make sure JPEGs are always decoded [#1884](https://github.com/rerun-io/rerun/pull/1884)
- Fix crash when saving store to file [#1909](https://github.com/rerun-io/rerun/pull/1909)
- Don't clean up `LogDb`s that only contain a `BeginRecordingMsg` [#1914](https://github.com/rerun-io/rerun/pull/1914)
- Fix picking entities with image + another object (or label) twice [#1908](https://github.com/rerun-io/rerun/pull/1908)
- Fix double clicking camera no longer focusing on said camera [#1911](https://github.com/rerun-io/rerun/pull/1911)
- Fix annotation images sometimes drawn in the background [#1933](https://github.com/rerun-io/rerun/pull/1933)
- Use `zipfile` python library instead of `unzip` command in `arkitscene` demo [#1936](https://github.com/rerun-io/rerun/pull/1936)
- Fix backslashes in `arkitscene` rigid transformation path [#1938](https://github.com/rerun-io/rerun/pull/1938)
- Fix hover/select highlights when picking single points in a scene with multiple point clouds [#1942](https://github.com/rerun-io/rerun/pull/1942)
- Fix hovering depth clouds [#1943](https://github.com/rerun-io/rerun/pull/1943)

#### 🚀 Performance Improvements
- batching 4: retire `MsgBundle` + batching support in transport layer [#1679](https://github.com/rerun-io/rerun/pull/1679)
- Optimize the depth-cloud shader when `depth=0` [#1729](https://github.com/rerun-io/rerun/pull/1729)
- `arrow2_convert` primitive (de)serialization benchmarks [#1742](https://github.com/rerun-io/rerun/pull/1742)
- `arrow2` `estimated_bytes_size` benchmarks [#1743](https://github.com/rerun-io/rerun/pull/1743)
- `arrow2` erased refcounted clones benchmarks [#1745](https://github.com/rerun-io/rerun/pull/1745)
- benchmarks for common vector ops across `smallvec`/`tinyvec`/std [#1747](https://github.com/rerun-io/rerun/pull/1747)
- Columnar `TimePoint`s in data tables and during transport [#1767](https://github.com/rerun-io/rerun/pull/1767)
- Compile with `panic = "abort"` [#1813](https://github.com/rerun-io/rerun/pull/1813)
- Process 2D points per entities like 3D points [#1820](https://github.com/rerun-io/rerun/pull/1820)
- re_query: use latest data types (`DataRow`/`DataCell`) [#1828](https://github.com/rerun-io/rerun/pull/1828)
- Depth cloud textures are now cached frame-to-frame [#1913](https://github.com/rerun-io/rerun/pull/1913)

#### 🧑‍🏫 Examples
- Add new `ARKitScenes` example [#1538](https://github.com/rerun-io/rerun/pull/1538) (thanks [@pablovela5620](https://github.com/pablovela5620)!)
- New example code for Facebook research's `segment-anything` [#1788](https://github.com/rerun-io/rerun/pull/1788)
- Add `minimal_options` example for Rust SDK [#1773](https://github.com/rerun-io/rerun/pull/1773) (thanks [@h3mosphere](https://github.com/h3mosphere)!)
- Remove manual depth projection from `car` and `nyud` examples [#1869](https://github.com/rerun-io/rerun/pull/1869)
- Always spawn instead of fork in multiprocessing example [#1922](https://github.com/rerun-io/rerun/pull/1922)
- Add `--num-frames` arg to canny (webcam) example [#1923](https://github.com/rerun-io/rerun/pull/1923)
- Add argument parsing to `rerun_demo` [#1925](https://github.com/rerun-io/rerun/pull/1925)
- Join threads at end of `multithreading` example [#1934](https://github.com/rerun-io/rerun/pull/1934)

#### 📚 Docs
- Add `typing_extensions` to `requirements-doc.txt` [#1786](https://github.com/rerun-io/rerun/pull/1786)
- Fix typos in notebook readme [#1852](https://github.com/rerun-io/rerun/pull/1852)
- Update docs related to notebook [#1915](https://github.com/rerun-io/rerun/pull/1915)

#### 🖼 UI Improvements
- Hover rays for tracked 3D cameras [#1751](https://github.com/rerun-io/rerun/pull/1751)
- Collapse space-view by default if there is only one child [#1762](https://github.com/rerun-io/rerun/pull/1762)
- Option to show scene bounding box [#1770](https://github.com/rerun-io/rerun/pull/1770)
- Assign default colors to class-ids when annotation context is missing [#1783](https://github.com/rerun-io/rerun/pull/1783)
- Add Restart command and keyboard shortcut for moving time to start of timeline [#1802](https://github.com/rerun-io/rerun/pull/1802) (thanks [@h3mosphere](https://github.com/h3mosphere)!)
- New option to disable persistent storage [#1825](https://github.com/rerun-io/rerun/pull/1825)
- Show previews of colormaps when selecting them [#1846](https://github.com/rerun-io/rerun/pull/1846)
- Smooth out scroll wheel input for camera zooming [#1920](https://github.com/rerun-io/rerun/pull/1920)

#### 🤷‍♂️ Other Viewer Improvements
- Change `EntityPathHash` to be 64 bit [#1723](https://github.com/rerun-io/rerun/pull/1723)
- Central `GpuReadback` handling for re_viewer, experimental space view screenshots [#1717](https://github.com/rerun-io/rerun/pull/1717)
- Readback depth from GPU picking [#1752](https://github.com/rerun-io/rerun/pull/1752)
- Use GPU picking for points, streamline/share picking code some more [#1814](https://github.com/rerun-io/rerun/pull/1814)
- Use GPU picking for line(like) primitives, fix `interactive` flags [#1829](https://github.com/rerun-io/rerun/pull/1829)
- Use GPU colormapping when showing images in the GUI [#1865](https://github.com/rerun-io/rerun/pull/1865)

#### 🕸️ Web
- Make CI publish `latest` tagged web-viewer to `app.rerun.io` [#1725](https://github.com/rerun-io/rerun/pull/1725)
- Implement `re_tuid::Tuid::random()` on web [#1796](https://github.com/rerun-io/rerun/pull/1796)
- Refactor the relationship between the assorted web / websocket servers [#1844](https://github.com/rerun-io/rerun/pull/1844)
- Notebooks: make `presentation_id` consistent and use data-attribute for rrd [#1881](https://github.com/rerun-io/rerun/pull/1881)
- 2.5GB before GC kick in on web [#1944](https://github.com/rerun-io/rerun/pull/1944)

#### 🎨 Renderer Improvements
- GPU based picking with points [#1721](https://github.com/rerun-io/rerun/pull/1721)
- improved renderer label handling [#1731](https://github.com/rerun-io/rerun/pull/1731)
- Improved readback data handling [#1734](https://github.com/rerun-io/rerun/pull/1734)
- GPU based mesh picking [#1737](https://github.com/rerun-io/rerun/pull/1737)
- Improve dealing with raw buffers for texture read/write [#1744](https://github.com/rerun-io/rerun/pull/1744)
- GPU colormapping, first step [#1835](https://github.com/rerun-io/rerun/pull/1835)
- GPU tensor colormapping [#1841](https://github.com/rerun-io/rerun/pull/1841)
- GPU picking for depth clouds [#1849](https://github.com/rerun-io/rerun/pull/1849)
- Implement billinear filtering of textures [#1850](https://github.com/rerun-io/rerun/pull/1850) [#1859](https://github.com/rerun-io/rerun/pull/1859) [#1860](https://github.com/rerun-io/rerun/pull/1860)
- Refactor: remove `GpuTexture2DHandle::invalid` [#1866](https://github.com/rerun-io/rerun/pull/1866)
- Fix filtering artifact for non-color images [#1886](https://github.com/rerun-io/rerun/pull/1886)
- Refactor: Add helper functions to `GpuTexture2DHandle` [#1900](https://github.com/rerun-io/rerun/pull/1900)

#### 🛢 Datastore Improvements
- Datastore: revamp bench suite [#1733](https://github.com/rerun-io/rerun/pull/1733)
- Datastore revamp 1: new indexing model & core datastructures [#1727](https://github.com/rerun-io/rerun/pull/1727)
- Datastore revamp 2: serialization & formatting [#1735](https://github.com/rerun-io/rerun/pull/1735)
- Datastore revamp 3: efficient incremental stats [#1739](https://github.com/rerun-io/rerun/pull/1739)
- Datastore revamp 4: sunset `MsgId` [#1785](https://github.com/rerun-io/rerun/pull/1785)
- Datastore revamp 5: `DataStore::to_data_tables()` [#1791](https://github.com/rerun-io/rerun/pull/1791)
- Datastore revamp 6: sunset `LogMsg` storage + save store to disk [#1795](https://github.com/rerun-io/rerun/pull/1795)
- Datastore revamp 7: garbage collection [#1801](https://github.com/rerun-io/rerun/pull/1801)
- Incremental metadata registry stats [#1833](https://github.com/rerun-io/rerun/pull/1833)

#### 🗣 Merged RFCs
- RFC: datastore state of the union & end-to-end batching  [#1610](https://github.com/rerun-io/rerun/pull/1610)

#### 🧑‍💻 Dev-experience
- Post-release cleanup [#1726](https://github.com/rerun-io/rerun/pull/1726)
- Remove unnecessary dependencies [#1711](https://github.com/rerun-io/rerun/pull/1711) (thanks [@vsuryamurthy](https://github.com/vsuryamurthy)!)
- Use copilot markers in PR template [#1784](https://github.com/rerun-io/rerun/pull/1784)
- re_format: barebone support for custom formatting [#1776](https://github.com/rerun-io/rerun/pull/1776)
- Refactor: Add new helper crate `re_log_encoding` [#1772](https://github.com/rerun-io/rerun/pull/1772)
- `setup_web.sh` supports pacman package manager [#1797](https://github.com/rerun-io/rerun/pull/1797) (thanks [@urholaukkarinen](https://github.com/urholaukkarinen)!)
- Add `rerun --strict`: crash if any warning or error is logged [#1812](https://github.com/rerun-io/rerun/pull/1812)
- End-to-end testing of python logging -> store ingestion [#1817](https://github.com/rerun-io/rerun/pull/1817)
- Fix e2e test on CI: Don't try to re-build `rerun-sdk` [#1821](https://github.com/rerun-io/rerun/pull/1821)
- Install the rerun-sdk in CI using `--no-index` and split out linux wheel build to run first [#1838](https://github.com/rerun-io/rerun/pull/1838)
- Remove more unused dependencies [#1863](https://github.com/rerun-io/rerun/pull/1863)
- Improve end-to-end testing slightly [#1862](https://github.com/rerun-io/rerun/pull/1862)
- Turn off benchmarks comment in each PR [#1872](https://github.com/rerun-io/rerun/pull/1872)
- Fix double-negation in `scripts/run_python_e2e_test.py` [#1896](https://github.com/rerun-io/rerun/pull/1896)
- Improve PR template with better comment, and no copilot by default [#1901](https://github.com/rerun-io/rerun/pull/1901)
- Optimize `generate_changelog.py` [#1912](https://github.com/rerun-io/rerun/pull/1912)

#### 🤷‍♂️ Other
- Fix videos for GitHub in `CHANGELOG.md` [af7d3b192157f942e35f64d3561a9a8dbcc18bfa](https://github.com/rerun-io/rerun/commit/af7d3b192157f942e35f64d3561a9a8dbcc18bfa)
- Don't run 3rd party bench suites on CI [#1787](https://github.com/rerun-io/rerun/pull/1787)
- Remove `TensorTrait` [#1819](https://github.com/rerun-io/rerun/pull/1819)
- Disable wheel tests for `x86_64-apple-darwin` [#1853](https://github.com/rerun-io/rerun/pull/1853)
- Update `enumflags2` to non-yanked version [#1874](https://github.com/rerun-io/rerun/pull/1874)
- Collect extra egui features into the main `Cargo.toml` [#1926](https://github.com/rerun-io/rerun/pull/1926)
- `just rs-run-all` [b14087b40bd805c95f030a4c7d3fb7a0482e13f4](https://github.com/rerun-io/rerun/commit/b14087b40bd805c95f030a4c7d3fb7a0482e13f4)
- `just py-run-all-{native|web|rrd}` [#1927](https://github.com/rerun-io/rerun/pull/1927)


## [0.4.0](https://github.com/rerun-io/rerun/compare/v0.3.1...v0.4.0) - Outlines, web viewer and performance improvements - 2023-03-28

https://user-images.githubusercontent.com/1220815/228241887-03b311e2-80e9-4541-9281-6d334a15ab04.mp4

### Overview & Highlights
* Add support for mesh vertex colors [#1671](https://github.com/rerun-io/rerun/pull/1671)
* Lower memory use [#1535](https://github.com/rerun-io/rerun/pull/1535)
* Improve garbage collection [#1560](https://github.com/rerun-io/rerun/pull/1560)
* Improve the web viewer [#1596](https://github.com/rerun-io/rerun/pull/1596) [#1594](https://github.com/rerun-io/rerun/pull/1594) [#1682](https://github.com/rerun-io/rerun/pull/1682)  [#1716](https://github.com/rerun-io/rerun/pull/1716) …
* Nice outlines when hovering/selecting
* Add an example of forever-streaming a web-camera image to Rerun [#1502](https://github.com/rerun-io/rerun/pull/1502)
* Fix crash-on-save on some versions of Linux [#1402](https://github.com/rerun-io/rerun/pull/1402)
* And a lot of other bug fixes
* Many performance improvements

We now host an experimental and unpolished web-viewer at <https://app.rerun.io/> for anyone to try out!

### In Detail

#### 🐍 Python SDK
- Expose all Rerun enums and types to main module scope [#1598](https://github.com/rerun-io/rerun/pull/1598)
- Make `log_point` more forgiving and update docstring [#1663](https://github.com/rerun-io/rerun/pull/1663)
- Add support for mesh vertex colors [#1671](https://github.com/rerun-io/rerun/pull/1671)

#### 🦀 Rust SDK
- ⚠️ `Session::new` has been replaced with `SessionBuilder` [#1528](https://github.com/rerun-io/rerun/pull/1528)
- ⚠️ `session.spawn(…)` -> `rerun::native_viewer::spawn(session, …)` [#1507](https://github.com/rerun-io/rerun/pull/1507)
- ⚠️ `session.show()` -> `rerun::native_viewer::show(session)`  [#1507](https://github.com/rerun-io/rerun/pull/1507)
- ⚠️ `session.serve(…)` -> `rerun::serve_web_viewer(session, …);`  [#1507](https://github.com/rerun-io/rerun/pull/1507)
- ⚠️ `rerun::global_session` is now hidden behind the `global_session` feature flag  [#1507](https://github.com/rerun-io/rerun/pull/1507)
- Add support for mesh vertex colors [#1671](https://github.com/rerun-io/rerun/pull/1671)

#### 🪳 Bug Fixes
- datastore: disable compaction (fixes 2x memory issue) [#1535](https://github.com/rerun-io/rerun/pull/1535)
- Fix garbage collection [#1560](https://github.com/rerun-io/rerun/pull/1560)
- Avoid using undefined extern "C" on windows [#1577](https://github.com/rerun-io/rerun/pull/1577)
- Fix crash on decoding old .rrd files [#1579](https://github.com/rerun-io/rerun/pull/1579)
- datastore: stabilize dataframe sorts [#1549](https://github.com/rerun-io/rerun/pull/1549)
- Stop using infinities in wgsl shaders [#1594](https://github.com/rerun-io/rerun/pull/1594)
- Workaround for alpha to coverage state leaking on (Web)GL renderer [#1596](https://github.com/rerun-io/rerun/pull/1596)
- Use a patched `wasm-bindgen-cli` with fix for 2GiB bug [#1605](https://github.com/rerun-io/rerun/pull/1605)
- Misc: make example in `log_pinhole` runable [#1609](https://github.com/rerun-io/rerun/pull/1609) (thanks [@Sjouks](https://github.com/Sjouks)!)
- Early-out on zero-sized space-views to prevent crashes [#1623](https://github.com/rerun-io/rerun/pull/1623)
- Print our own callstack on panics [#1622](https://github.com/rerun-io/rerun/pull/1622)
- Handle ctrl+c to gracefully shutdown the server(s) [#1613](https://github.com/rerun-io/rerun/pull/1613)
- Fix crash on serve exit, second attempt [#1633](https://github.com/rerun-io/rerun/pull/1633)
- Fix wrong remove-tooltip for entities and groups [#1637](https://github.com/rerun-io/rerun/pull/1637)
- Fix requiring requiring focus for shutdown via ctrl+c when starting viewer from command line [#1646](https://github.com/rerun-io/rerun/pull/1646)
- Fix eye spin after eye reset [#1652](https://github.com/rerun-io/rerun/pull/1652)
- Fix crash on negative radii by instead warning [#1654](https://github.com/rerun-io/rerun/pull/1654)
- Fix crash when trying to listen on a taken TCP port [#1650](https://github.com/rerun-io/rerun/pull/1650)
- Don't show 2D labels in 3D space views. [#1641](https://github.com/rerun-io/rerun/pull/1641)
- Fix Z fighting with improved depth offset math [#1661](https://github.com/rerun-io/rerun/pull/1661)
- Whether a spatial view is 2d or 3d is now reevaluated over time unless picked explicitly [#1660](https://github.com/rerun-io/rerun/pull/1660)
- Update wgpu to v0.15.3, fixing meshes on Windows Chrome [#1682](https://github.com/rerun-io/rerun/pull/1682)
- Fix a bug in the image hover code, causing the wrong RGBA values to be printed [#1690](https://github.com/rerun-io/rerun/pull/1690)
- Fix a bug that caused points to be render too large [#1690](https://github.com/rerun-io/rerun/pull/1690)
- Fix web crash on missing uniform buffer padding [#1699](https://github.com/rerun-io/rerun/pull/1699)
- Fix `memory_usage` example relying on implicit recursive features [#1709](https://github.com/rerun-io/rerun/pull/1709)
- Track changed state in nav mode combo box [#1703](https://github.com/rerun-io/rerun/pull/1703)
- Fix crash-on-save by switching file-picker dialog to `xdg-portal` [#1402](https://github.com/rerun-io/rerun/pull/1402)
- Change roll-shortcut from ALT to SHIFT [#1715](https://github.com/rerun-io/rerun/pull/1715)
- Fix CpuWriteGpuReadBelt producing unaligned gpu buffer offsets [#1716](https://github.com/rerun-io/rerun/pull/1716)
- Fix arrows requiring a radius to be visible [#1720](https://github.com/rerun-io/rerun/pull/1720)

#### 🚀 Performance Improvements
- Add re_arrow_store profile scopes [#1546](https://github.com/rerun-io/rerun/pull/1546)
- datastore: early exit missing components at table level [#1554](https://github.com/rerun-io/rerun/pull/1554)
- datastore: track bucket count in store stats & mem panel [#1555](https://github.com/rerun-io/rerun/pull/1555)
- LogDb: dont split on index bucket size [#1558](https://github.com/rerun-io/rerun/pull/1558)
- Introduce a simpler cache dedicated to just decode JPEGs  [#1550](https://github.com/rerun-io/rerun/pull/1550)
- Implement outlines for points 2d/3d/depth & use them for select & hover in Viewer [#1568](https://github.com/rerun-io/rerun/pull/1568)
- Simplify ImageCache [#1551](https://github.com/rerun-io/rerun/pull/1551)
- New time panel density graph [#1557](https://github.com/rerun-io/rerun/pull/1557)
- Refactor the Arrow Mesh3D type to use zero-copy Buffers [#1691](https://github.com/rerun-io/rerun/pull/1691)
- Remove the redundant costly transform check during categorization [#1695](https://github.com/rerun-io/rerun/pull/1695)
- batching 3: `DataRow` & `DataTable` + no bundles outside of transport [#1673](https://github.com/rerun-io/rerun/pull/1673)

#### 🧑‍🏫 Examples
- Very simple example streaming from an opencv camera [#1502](https://github.com/rerun-io/rerun/pull/1502)
- Initial TurtleBot subscriber demo [#1523](https://github.com/rerun-io/rerun/pull/1523)

#### 📚 Docs
- Link to the Python SDK build instructions in `rerun_py/README.md` [#1565](https://github.com/rerun-io/rerun/pull/1565)

#### 🖼 UI Improvements
- Fix combining outline mask for selection & hover [#1552](https://github.com/rerun-io/rerun/pull/1552)
- Implement outlines for rectangles & use them for select & hover of image primitives in Viewer [#1559](https://github.com/rerun-io/rerun/pull/1559)
- Show log messages in egui toast notifications [#1603](https://github.com/rerun-io/rerun/pull/1603)
- Adapt UI for smaller screens [#1608](https://github.com/rerun-io/rerun/pull/1608)
- Nicer toast notifications [#1621](https://github.com/rerun-io/rerun/pull/1621)
- Don't hover things in 2D/3D views if we are dragging something [#1643](https://github.com/rerun-io/rerun/pull/1643)
- Allow rolling 3D camera with primary mouse button + alt modifier [#1659](https://github.com/rerun-io/rerun/pull/1659)
- Name space views after the space and indicate duplicate names [#1653](https://github.com/rerun-io/rerun/pull/1653)
- Add banner about mobile browsers being unsupported [#1674](https://github.com/rerun-io/rerun/pull/1674)
- Improve ui for tensors and color map selection [#1683](https://github.com/rerun-io/rerun/pull/1683)
- Only show the mobile OS warning banner on web [#1685](https://github.com/rerun-io/rerun/pull/1685)
- Improve the depth backprojection feature [#1690](https://github.com/rerun-io/rerun/pull/1690)
- Swap overlay order of selection & hover outlines [#1705](https://github.com/rerun-io/rerun/pull/1705)
- Turn on depth cloud backprojection by default [#1710](https://github.com/rerun-io/rerun/pull/1710)
- Add radius boost for depth clouds on outline [#1713](https://github.com/rerun-io/rerun/pull/1713)

#### 🤷‍♂️ Other Viewer Improvements
- Fix web feature name in error messages [#1521](https://github.com/rerun-io/rerun/pull/1521)
- Use outlines for mesh selections instead of highlight colors [#1540](https://github.com/rerun-io/rerun/pull/1540)
- Implement outlines for line renderer & use them for select & hover of "line-like" primitives in Viewer [#1553](https://github.com/rerun-io/rerun/pull/1553)
- Load .rrd file over HTTP [#1600](https://github.com/rerun-io/rerun/pull/1600)
- Revert "Handle ctrl+c to gracefully shutdown the server(s)" [#1632](https://github.com/rerun-io/rerun/pull/1632)
- More eager GC, and remove `--fast-math` optimization for wasm [#1656](https://github.com/rerun-io/rerun/pull/1656)
- Detect failure to install GUI log callback [#1655](https://github.com/rerun-io/rerun/pull/1655)
- Warn when most of the RAM has been used up by Rerun [#1651](https://github.com/rerun-io/rerun/pull/1651)
- Apply color maps to all types of depth tensors [#1686](https://github.com/rerun-io/rerun/pull/1686)
- Size boosted outlines for points & lines, color & size tweaking [#1667](https://github.com/rerun-io/rerun/pull/1667)
- Default point radius to 1.5 ui points [#1706](https://github.com/rerun-io/rerun/pull/1706)
- When streaming an rrd from http: play it, don't follow it [#1707](https://github.com/rerun-io/rerun/pull/1707)

#### 🕸️ Web
- Use `log` as our log backend instead of `tracing` [#1590](https://github.com/rerun-io/rerun/pull/1590)
- Turn on allocation tracker at run-time and for web [#1591](https://github.com/rerun-io/rerun/pull/1591)
- Set correct MIME types in re_web_viewer_server [#1602](https://github.com/rerun-io/rerun/pull/1602)
- Upload web viewer to a bucket [#1606](https://github.com/rerun-io/rerun/pull/1606)
- Use hostname for default websocket address [#1664](https://github.com/rerun-io/rerun/pull/1664)
- Upload the colmap rrd file to gcloud [#1666](https://github.com/rerun-io/rerun/pull/1666)
- Show a warning by default on mobile browsers [#1670](https://github.com/rerun-io/rerun/pull/1670)
- Add analytics to the hosted index.html [#1675](https://github.com/rerun-io/rerun/pull/1675)
- Always upload latest prerelease to a dedicated prefix [#1676](https://github.com/rerun-io/rerun/pull/1676)
- Allow url param override on app.rerun.io [#1678](https://github.com/rerun-io/rerun/pull/1678)
- Show the git commit in the about section in pre-release builds [#1677](https://github.com/rerun-io/rerun/pull/1677)
- Update the web icon [#1688](https://github.com/rerun-io/rerun/pull/1688)

#### 🎨 Renderer Improvements
- Outlines via masking & postprocessing in `re_renderer` [#1532](https://github.com/rerun-io/rerun/pull/1532)
- Add missing profiling scopes in `re_renderer` [#1567](https://github.com/rerun-io/rerun/pull/1567)
- Don't call `wgpu::Device::poll` on the web [#1626](https://github.com/rerun-io/rerun/pull/1626)
- Merge final outline render into composite step in order to fix blending [#1629](https://github.com/rerun-io/rerun/pull/1629)
- renderer: fix the groupby logic in mesh instancing [#1657](https://github.com/rerun-io/rerun/pull/1657)
- Fix outlines being offset diagonally by about half a pixel [#1668](https://github.com/rerun-io/rerun/pull/1668)
- Gpu readback belt for fast & easy data readback from gpu [#1687](https://github.com/rerun-io/rerun/pull/1687)
- Make CpuWriteGpuReadBelt texture copies easier/less error prone [#1689](https://github.com/rerun-io/rerun/pull/1689)

#### ✨ Other Enhancement
- datastore: split out formatting & sanity checks in their own modules [#1625](https://github.com/rerun-io/rerun/pull/1625)
- Add `rerun --save`: stream incoming log stream to an rrd file [#1662](https://github.com/rerun-io/rerun/pull/1662)
- batching 1: introduce `DataCell` & retire `ComponentBundle` [#1634](https://github.com/rerun-io/rerun/pull/1634)
- Data store batching 2: split out component traits [#1636](https://github.com/rerun-io/rerun/pull/1636)

#### 📈 Analytics
- Analytics: don't spam warning when there is an HTTP connection problem [#1564](https://github.com/rerun-io/rerun/pull/1564)
- Analytics: Rename "location" to "file_line" in the "crash-panic" event [#1575](https://github.com/rerun-io/rerun/pull/1575)

#### 🗣 Merged RFCs
- RFC: component-datatype conversions [#1595](https://github.com/rerun-io/rerun/pull/1595)
- RFC: pre-proposal for blueprint store [#1582](https://github.com/rerun-io/rerun/pull/1582)

#### 🧑‍💻 Dev-experience
- Update `rayon` [#1541](https://github.com/rerun-io/rerun/pull/1541)
- Fix some `1.68` clippy lints [#1569](https://github.com/rerun-io/rerun/pull/1569)
- Remove duplicated 'nix' crate [#1479](https://github.com/rerun-io/rerun/pull/1479)
- Better MsgId format [#1566](https://github.com/rerun-io/rerun/pull/1566)
- Lint vertical spacing in Rust code [#1572](https://github.com/rerun-io/rerun/pull/1572)
- CI: Replace wasm_bindgen_check.sh with actually building the web-viewer [#1604](https://github.com/rerun-io/rerun/pull/1604)
- Add --all-features to Rust Analyzer flags [#1624](https://github.com/rerun-io/rerun/pull/1624)
- Run clippy for wasm, with own clippy.toml config file [#1628](https://github.com/rerun-io/rerun/pull/1628)
- Update tokio v1.24.1 -> v1.26.0 [#1635](https://github.com/rerun-io/rerun/pull/1635)
- Add a workflow input for running benchmarks manually [#1698](https://github.com/rerun-io/rerun/pull/1698)
- Add missing } to fix rust workflow [#1700](https://github.com/rerun-io/rerun/pull/1700)
- Fix `lint.py` [#1719](https://github.com/rerun-io/rerun/pull/1719)
- Add a script that generates a changelog from recent PRs and their labels [#1718](https://github.com/rerun-io/rerun/pull/1718)

#### 🤷‍♂️ Other
- Clean up opencv_canny example slightly [b487e550dcb87225858dc6f76b791a25e938e75e](https://github.com/rerun-io/rerun/commit/b487e550dcb87225858dc6f76b791a25e938e75e)
- Lint fixes [9901e7c6735356b1970ddabc926bc5378d82e057](https://github.com/rerun-io/rerun/commit/9901e7c6735356b1970ddabc926bc5378d82e057)


## [0.3.1](https://github.com/rerun-io/rerun/compare/v0.3.0...v0.3.1) - Remove potentially sensitive analytics - 2023-03-13

Remove potentially sensitive analytics, including path to rerun source code on panics, and rerun branch name when building from source [#1563](https://github.com/rerun-io/rerun/pull/1563)


## [0.3.0](https://github.com/rerun-io/rerun/compare/v0.2.0...v0.3.0) - 2023-03-07
### Overview & Highlights

After a successful launch a couple of weeks ago, we're back with our second release!
With a few exceptions this release focuses on internal refactors & improving our processes.
However, we think you'll enjoy these goodies that made it in nonetheless!

https://user-images.githubusercontent.com/2910679/222510504-23871b8c-0bef-49c2-bbd2-37baab4247e8.mp4


You can now generate point clouds directly from depth textures and choose a wide variety of color maps.
Check out this [video](https://user-images.githubusercontent.com/1220815/223365363-da13585f-3a91-4cb8-a6ef-8a6fadbeb4eb.webm) on how to use it.
This is **a lot** faster and more convenient than doing so manually in your own code
Some caveats: Picking is not yet working and visible history may behave differently (related to [#723](https://github.com/rerun-io/rerun/issues/723))

Other highlights:

* Viewer
  * Improved formatting of date-times in plots [#1356](https://github.com/rerun-io/rerun/pull/1356)
  * Labels for 3D objects have now a color can now be selected & hovered [#1438](https://github.com/rerun-io/rerun/pull/1438)
  * Scale factor is saved across sessions and more persistent between screens [#1448](https://github.com/rerun-io/rerun/pull/1448)
  * Showing tensors in the viewer is now faster
* SDK
  * Python packages now work with Ubuntu-20.04 [#1334](https://github.com/rerun-io/rerun/pull/1334)
  * u8 segmentation stay u8 now (they converted to u16 before) [#1376](https://github.com/rerun-io/rerun/pull/1376)
  * 2D Line strips can now be logged directly [#1430](https://github.com/rerun-io/rerun/pull/1430)
  * Add a `strict` mode to the Python SDK where misuses of the API result in exceptions being raised.[#1477](https://github.com/rerun-io/rerun/pull/1477)
  * Fix disabling Python API through `init` not working [#1517](https://github.com/rerun-io/rerun/pull/1517)
* General
  * We build now with fewer build dependencies (there is however [still more work to do!](https://github.com/rerun-io/rerun/issues/1316)).
  Notably, we previously used a version of the `time` crate which had a security issue (CVE-2020-26235), thanks @mpizenberg for helping out!
  * Print more information & troubleshooting info on crash

Meanwhile, we did a bunch of improvements to our manual. If you had trouble running Rerun so far, check our updated [troubleshooting](https://www.rerun.io/docs/getting-started/troubleshooting) page (and as always, please [open an issue](https://github.com/rerun-io/rerun/issues/new/choose) if something doesn't work).

⚠️ BREAKING: old `.rrd` files no longer load ⚠️

### In Detail
#### New Features
* Generate point clouds directly from depth textures
  * re_renderer: implement depth cloud renderer [#1415](https://github.com/rerun-io/rerun/pull/1415)
  * Integrate depth clouds into Rerun [#1421](https://github.com/rerun-io/rerun/pull/1421)
  * CPU & GPU color maps [#1484](https://github.com/rerun-io/rerun/pull/1484)
  * Integrate GPU color maps into depth clouds  [#1486](https://github.com/rerun-io/rerun/pull/1486)
* Python SDK: Add strict mode [#1477](https://github.com/rerun-io/rerun/pull/1477)
* OS independent Zoom factor & serialization thereof [#1448](https://github.com/rerun-io/rerun/pull/1448)
* Labels for 3D objects have now a color can now be selected & hovered [#1438](https://github.com/rerun-io/rerun/pull/1438)
* Add 2d support for linestrips [#1430](https://github.com/rerun-io/rerun/pull/1430)
* Add signal handler on *nix with troubleshooting and stacktrace [#1340](https://github.com/rerun-io/rerun/pull/1340)
  * Point users to our troubleshooting page on panic [#1338](https://github.com/rerun-io/rerun/pull/1338)

#### Performance
* Speed up conversions for color arrays in Python [#1454](https://github.com/rerun-io/rerun/pull/1454)
* Speed up fixed-sized array iteration [#1050](https://github.com/rerun-io/rerun/pull/1050)
* Speed up tensor handling by padding data through more directly
  * Direct conversion to dynamic image from Tensors [#1455](https://github.com/rerun-io/rerun/pull/1455)
  * Convert view_tensor to use the new native Tensors [#1439](https://github.com/rerun-io/rerun/pull/1439)
* Add option to show performance metrics in the UI in release builds too [#1444](https://github.com/rerun-io/rerun/pull/1444)
* Faster stable diffusion sample [#1364](https://github.com/rerun-io/rerun/pull/1364)
* SDK: stream to disk with `save` feature [#1405](https://github.com/rerun-io/rerun/pull/1405)
* `re_renderer` has now a direct CPU->GPU copy mechanism
  * `CpuWriteGpuReadBelt` for fast frame by frame memory transfers [#1382](https://github.com/rerun-io/rerun/pull/1382)
  * Uniform buffer utility using `CpuWriteGpuReadBelt` [#1400](https://github.com/rerun-io/rerun/pull/1400)
  * Use `CpuWriteGpuReadBelt` for mesh data gpu upload [#1416](https://github.com/rerun-io/rerun/pull/1416)

#### Small improvements & Bugfixes
* UI
  * Add scroll-bars the "Add/Remove entities" window [#1445](https://github.com/rerun-io/rerun/pull/1445)
  * Unify the time formatting between the time panel and the plot [#1369](https://github.com/rerun-io/rerun/pull/1369)
  * Timeline
    * Fix precision issue when zooming in on the timeline [#1370](https://github.com/rerun-io/rerun/pull/1370)
    * Improve the gap-detector [#1363](https://github.com/rerun-io/rerun/pull/1363)
  * Better time axis on plot view [#1356](https://github.com/rerun-io/rerun/pull/1356)
  * Prevent wrap on 'Streams' text [#1308](https://github.com/rerun-io/rerun/pull/1308)
  * Update to eframe 0.21.3 with fix for web text input [#1311](https://github.com/rerun-io/rerun/pull/1311)
* `re_renderer`
  * Fix crash due to always expecting Rgba8Unorm backbuffer on Web & Bgra8Unorm on native [#1413](https://github.com/rerun-io/rerun/pull/1413)
  * Allow controlling the graphics backend & power preference through standard wgpu env vars [#1332](https://github.com/rerun-io/rerun/pull/1332)
* Heuristic for camera frustum length is now based on scene size [#1433](https://github.com/rerun-io/rerun/pull/1433)
* Fix python type signature for tensor names [#1443](https://github.com/rerun-io/rerun/pull/1443)
* Don't convert u8 segmentation images to u16 [#1376](https://github.com/rerun-io/rerun/pull/1376)
* Docs (excluding the manual)
  * Improve the docs of `connect` and `serve` [#1450](https://github.com/rerun-io/rerun/pull/1450)
  * Update log_mesh and log_meshes docs. [#1286](https://github.com/rerun-io/rerun/pull/1286)
  * Add guidelines for adding dependencies in a PR [#1431](https://github.com/rerun-io/rerun/pull/1431)
  * Add a few more sections to `CODE_STYLE.md` [#1365](https://github.com/rerun-io/rerun/pull/1365)
  * Fixup for some doc links [#1314](https://github.com/rerun-io/rerun/pull/1314)
  * Document undocumented environment variables on help page. [#1335](https://github.com/rerun-io/rerun/pull/1335)
  * Link to SDK operating modes doc in both SDK [#1330](https://github.com/rerun-io/rerun/pull/1330)
* More information in `--version` [#1388](https://github.com/rerun-io/rerun/pull/1388)
* Remove already broken `show` method from Python SDK [#1429](https://github.com/rerun-io/rerun/pull/1429)
* Analytics
  * Send analytics events with callstacks on panics and signals [#1409](https://github.com/rerun-io/rerun/pull/1409)
  * Put all analytics to one bucket [#1390](https://github.com/rerun-io/rerun/pull/1390)
  * add event for when we serve the web-viewer .wasm [#1379](https://github.com/rerun-io/rerun/pull/1379)
  * register SDK language and data source [#1371](https://github.com/rerun-io/rerun/pull/1371)
  * Refactor analytics [#1368](https://github.com/rerun-io/rerun/pull/1368)
* Versioned log streams streams [#1420](https://github.com/rerun-io/rerun/pull/1420)
* Fix path issues when running debug viewer within workspace [#1341](https://github.com/rerun-io/rerun/pull/1341)
* Detailed errors for re_renderer `include_file!` [#1339](https://github.com/rerun-io/rerun/pull/1339)
* Limit logging in web-viewer to `warn` in order to workaround a crash issue (and reduce log spam) [1514](https://github.com/rerun-io/rerun/pull/1514)
* Fix disabling API through `init` not working [#1517](https://github.com/rerun-io/rerun/pull/1517)

#### CI, Testing & Build improvements
* Reduce build dependencies
  * Get rid of time 0.1.* dependency [#1408](https://github.com/rerun-io/rerun/pull/1408)
  * Remove unnecessary ordered-float [#1461](https://github.com/rerun-io/rerun/pull/1461)
  * Remove extraneous `image` features and dependencies [#1425](https://github.com/rerun-io/rerun/pull/1425)
  * Replace `reqwest` with `ureq` [#1407](https://github.com/rerun-io/rerun/pull/1407)
  * Remove derive_more dependency [#1406](https://github.com/rerun-io/rerun/pull/1406)
* Use different artifact names for wasm/js in debug builds [#1428](https://github.com/rerun-io/rerun/pull/1428)
* Separate mac wheels & trigger wheel build from ui [#1499](https://github.com/rerun-io/rerun/pull/1499)
* Add spell checking to CI [#1492](https://github.com/rerun-io/rerun/pull/1492)
* Repo size
  * Always create new orphaned branch for gh-pages [#1490](https://github.com/rerun-io/rerun/pull/1490)
  * GitHub Action to prevent large files [#1478](https://github.com/rerun-io/rerun/pull/1478)
* Python
  * Remove the python job path filters [#1452](https://github.com/rerun-io/rerun/pull/1452)
  * Use ruff for our python lints [#1378](https://github.com/rerun-io/rerun/pull/1378)
  * Use python3 in the jobs that weren't tested in PR [#1348](https://github.com/rerun-io/rerun/pull/1348)
* Testing
  * Add a test of memory use when logging a lot of big images [#1372](https://github.com/rerun-io/rerun/pull/1372)
* Switch ci_docker to a container based on ubuntu 20.04 [#1334](https://github.com/rerun-io/rerun/pull/1334)
* Release handling
  * Switch release action to ncipollo [#1489](https://github.com/rerun-io/rerun/pull/1489)
  * Fix our continuous pre-releases [#1458](https://github.com/rerun-io/rerun/pull/1458)
  * Delete the prerelease before creating the new one [#1485](https://github.com/rerun-io/rerun/pull/1485)
  * Set prerelease to true even for version-tagged CI job [#1504](https://github.com/rerun-io/rerun/pull/1504)
  * Let the release job take care of creating the tag [#1501](https://github.com/rerun-io/rerun/pull/1501)
  * Use `cargo update -w` instead of `cargo check` when prepping prerelease [#1500](https://github.com/rerun-io/rerun/pull/1500)
  * Use prerelease tag instead of latest and update pointer on prerelease [#1481](https://github.com/rerun-io/rerun/pull/1481)
  * Include date in pre-release version [#1472](https://github.com/rerun-io/rerun/pull/1472)
  * Switch pre-release action to ncipollo/release-action [#1466](https://github.com/rerun-io/rerun/pull/1466)
* Disallow some methods and types via Clippy[#1411](https://github.com/rerun-io/rerun/pull/1411)

#### Other non-user-facing refactors
* Fix: don't create a dummy LogDb when opening the Rerun Menu [#1440](https://github.com/rerun-io/rerun/pull/1440)
* `re_renderer`
  * `Draw Phases` in preparation of executing `Renderer` several times on different targets [#1419](https://github.com/rerun-io/rerun/pull/1419)
    * Fix mesh creation failing to copy index data. [#1473](https://github.com/rerun-io/rerun/pull/1473)
    * do not silently drop draw phases [#1471](https://github.com/rerun-io/rerun/pull/1471)
  * Simplify bind group allocation call by passing pool collection object. [#1459](https://github.com/rerun-io/rerun/pull/1459)
  * Interior mutable buffer/texture/bindgroup pools [#1374](https://github.com/rerun-io/rerun/pull/1374)
  * Rename all instances of `frame_maintenance` to `begin_frame` [#1360](https://github.com/rerun-io/rerun/pull/1360)
  * Texture & buffer call now wgpu's `destroy` on removal from pool [#1359](https://github.com/rerun-io/rerun/pull/1359)
  * Arrow buffers as (optional) first-class citizen [#1482](https://github.com/rerun-io/rerun/pull/1482)
  * Log static re_renderer resource generation [#1464](https://github.com/rerun-io/rerun/pull/1464)
* Internal log_text_entry_internal to break circular deps [#1488](https://github.com/rerun-io/rerun/pull/1488)
* Delete ClassicTensor and cleanup [#1456](https://github.com/rerun-io/rerun/pull/1456)
* Fix re_renderer file watcher watching the same file several times [#1463](https://github.com/rerun-io/rerun/pull/1463)
* Analytics
  * More ergonomic API [#1410](https://github.com/rerun-io/rerun/pull/1410)
  * Streamlining host vs. recorder python/rust versions [#1380](https://github.com/rerun-io/rerun/pull/1380)
  * Fix workspace detection [#1437](https://github.com/rerun-io/rerun/pull/1437)
* Introduce `DeserializableComponent` trait and high-level `query_latest` [#1417](https://github.com/rerun-io/rerun/pull/1417)


[Full Changelog](https://github.com/rerun-io/rerun/compare/v0.2.0...v0.3.0)

## 0.2.0 - 2023-02-14
First public release!
