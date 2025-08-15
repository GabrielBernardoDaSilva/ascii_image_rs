# ASCII Renderer

A **Rust** utility to convert images and GIFs into ASCII art directly in the terminal, with support for **colors**, **simple or complex charset**, and **animation**.

## 📌 Features

- Converts **static images** (PNG, JPEG, etc.) to ASCII.
- Renders **animated GIFs** directly in the terminal.
- **Two character sets**:
  - Simple (`@%$#*+=-:. `)
  - Complex (block and symbol characters for higher visual detail)
- Supports **colored** or **grayscale** output.
- Automatically adjusts to terminal width.
- Option to loop GIF animations infinitely.

---

## 🖼️ Usage Examples

### Simple ASCII image:

```bash
cargo run -- path=image.png
```

### Image with complex charset and colors:

```bash
cargo run -- path=image.jpg --cp --cl
```

### Animated GIF in loop:

```bash
cargo run -- path=animation.gif --i
```

### Show help:

```bash
cargo run -- --h
```

---

## ⚙️ Available Parameters

| Parameter          | Description                   |
| ------------------ | ----------------------------- |
| `path=[file]`      | Path to the image or GIF file |
| `-infinity`, `--i` | Infinite loop for GIFs        |
| `-complex`, `--cp` | Use complex character set     |
| `-color`, `--cl`   | Enable colored output         |
| `-help`, `--h`     | Show available options        |

---

## 📥 Installation

### Requirements

- **Rust** installed ([Install Rust](https://www.rust-lang.org/tools/install))
- **Cargo** (comes with Rust)

### Installation Steps

1. Clone the repository:

```bash
git clone https://github.com/your-username/ascii-renderer.git
cd ascii-renderer
```

2. Build the project:

```bash
cargo build --release
```

3. Run:

```bash
cargo run -- path=image.png

4. Install: (Optional)

cargo install --path .
```

---

## 🔍 How it works

1. **File type detection**
   - If the path ends with `.gif`, it will be processed as an animation.
   - Otherwise, it will be processed as a static image.

2. **Resizing**
   - The image is resized to fit the terminal width while keeping the aspect ratio.
   - Height is adjusted (`height_compression = 0.5`) to match character proportions.

3. **Pixel to character mapping**
   - Each pixel is converted into a character based on its luminance value.
   - The value is scaled to the selected character set size.

4. **Colorization**
   - When enabled, the original pixel color is preserved using ANSI escape codes.
   - Otherwise, grayscale output is used.

---

## 📜 License

This project is licensed under the **MIT License** — see the [LICENSE](LICENSE) file for details.
