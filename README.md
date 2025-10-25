# Viz PDF - Rust WASM PDF Renderer for React

A high-performance PDF renderer built with Rust and WebAssembly, designed for seamless integration with React applications.

## Features

- **Rust-powered rendering**: Leverages Rust's performance and safety for PDF parsing and rendering
- **WebAssembly integration**: Compiled to WASM for near-native performance in the browser
- **React components**: Easy-to-use React components with TypeScript support
- **Responsive UI**: Modern, mobile-friendly PDF viewer interface
- **Zoom controls**: Scale pages up or down for better readability
- **Page navigation**: Navigate through PDF documents with ease
- **File upload**: Support for loading PDF files from local filesystem

## Architecture

### Rust Layer (`src/lib.rs`)
- PDF parsing using the `pdf` crate
- Canvas rendering via `web-sys`
- WASM bindings with `wasm-bindgen`
- Efficient memory management

### React Layer (`www/src/`)
- TypeScript-based React components
- Custom hooks for WASM integration
- Vite for fast development and building
- Modern CSS with responsive design

## Prerequisites

- **Rust**: Install from [rustup.rs](https://rustup.rs/)
- **wasm-pack**: Install with `cargo install wasm-pack`
- **Node.js**: Version 16 or higher
- **npm** or **yarn**: Package manager

## Installation

1. Clone the repository:
```bash
git clone <your-repo-url>
cd viz-pdf
```

2. Install dependencies:
```bash
npm run setup
```

## Development

### Build WASM module (development mode):
```bash
npm run dev:wasm
```

### Start React development server:
```bash
cd www
npm run dev
```

### Run full development workflow:
```bash
npm run dev
```

The application will be available at `http://localhost:5173`

## Production Build

### Build everything:
```bash
npm run build
```

This will:
1. Compile Rust code to optimized WASM
2. Build the React application
3. Output production-ready files to `www/dist/`

### Build individual parts:

**WASM only:**
```bash
npm run build:wasm
```

**React only:**
```bash
cd www
npm run build
```

## Usage

### Basic Usage

```tsx
import { PdfViewer } from './components/PdfViewer';

function App() {
  return (
    <PdfViewer
      scale={1.5}
      fileUrl="/path/to/your/file.pdf"
    />
  );
}
```

### Props

- `scale` (optional): Initial zoom level (default: 1.5)
- `fileUrl` (optional): URL to load PDF from on mount

### Custom Integration

You can also use the WASM module directly:

```tsx
import { usePdfRenderer } from './hooks/usePdfRenderer';

function CustomViewer() {
  const { createRenderer, loading } = usePdfRenderer();

  // Your custom implementation
}
```

## Project Structure

```
viz-pdf/
├── src/
│   └── lib.rs              # Rust WASM implementation
├── www/
│   ├── src/
│   │   ├── components/
│   │   │   ├── PdfViewer.tsx
│   │   │   └── PdfViewer.css
│   │   ├── hooks/
│   │   │   └── usePdfRenderer.ts
│   │   ├── App.tsx
│   │   ├── App.css
│   │   └── main.tsx
│   ├── public/
│   ├── index.html
│   ├── vite.config.ts
│   ├── tsconfig.json
│   └── package.json
├── Cargo.toml              # Rust dependencies
├── package.json            # Root package configuration
└── README.md
```

## API Reference

### Rust WASM API

**PdfRenderer**
- `new()`: Create a new renderer instance
- `loadPdf(data: &[u8])`: Load PDF from byte array
- `getTotalPages()`: Get total number of pages
- `getCurrentPage()`: Get current page number
- `setCurrentPage(page: number)`: Set current page
- `renderPage(canvas, pageNum, scale)`: Render page to canvas
- `getPageDimensions(pageNum)`: Get page dimensions

### React Components

**PdfViewer**
- Fully-featured PDF viewer with controls
- Handles file upload, navigation, and zoom
- Responsive and mobile-friendly

**usePdfRenderer Hook**
- Manages WASM module lifecycle
- Provides renderer instance
- Handles loading and error states

## Performance Optimization

The production build includes:
- Optimized WASM with LTO (Link Time Optimization)
- Small binary size with `opt-level = "z"`
- Tree-shaking and dead code elimination
- Minified JavaScript and CSS

## Browser Support

- Chrome/Edge: ✅ Full support
- Firefox: ✅ Full support
- Safari: ✅ Full support (Safari 11.1+)
- Mobile browsers: ✅ Supported

## Known Limitations

- Current implementation uses a placeholder rendering system
- For production use, you may want to integrate more advanced PDF rendering libraries
- Large PDFs may require pagination or virtual scrolling for better performance

## Future Enhancements

- [ ] Text selection and copying
- [ ] Search functionality
- [ ] Annotations and highlights
- [ ] Thumbnails sidebar
- [ ] Print support
- [ ] Full PDF rendering (currently placeholder)
- [ ] Password-protected PDF support
- [ ] Multi-page view

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License - see LICENSE file for details

## Troubleshooting

### WASM module fails to load
- Ensure wasm-pack is installed: `cargo install wasm-pack`
- Rebuild WASM: `npm run build:wasm`
- Clear browser cache and reload

### TypeScript errors
- Run `cd www && npm install` to ensure all dependencies are installed
- Check that TypeScript version is 5.2 or higher

### Build fails
- Verify Rust is installed: `rustc --version`
- Verify Node.js version: `node --version` (should be 16+)
- Delete `node_modules` and reinstall: `npm run setup`

## Resources

- [wasm-bindgen Documentation](https://rustwasm.github.io/wasm-bindgen/)
- [Rust PDF Crate](https://docs.rs/pdf/latest/pdf/)
- [React Documentation](https://react.dev/)
- [Vite Documentation](https://vitejs.dev/)
# TurboPDF
