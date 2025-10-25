import { useState, useEffect, useRef } from 'react';

interface PdfRendererModule {
  PdfRenderer: new () => PdfRenderer;
}

interface PdfRenderer {
  loadPdf(data: Uint8Array): void;
  getTotalPages(): number;
  getCurrentPage(): number;
  setCurrentPage(page: number): void;
  renderPage(canvas: HTMLCanvasElement, pageNum: number, scale: number): void;
  getPageDimensions(pageNum: number): { width: number; height: number };
  free(): void;
}

export function usePdfRenderer() {
  const [wasmModule, setWasmModule] = useState<PdfRendererModule | null>(null);
  const [renderer, setRenderer] = useState<PdfRenderer | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let isMounted = true;

    async function loadWasm() {
      try {
        setLoading(true);
        // @ts-ignore - WASM module will be available at runtime
        const module = await import('../../pkg/viz_pdf.js');

        // Initialize the WASM module before using it
        await module.default();

        if (isMounted) {
          setWasmModule(module);
          setLoading(false);
        }
      } catch (err) {
        console.error('Failed to load WASM module:', err);
        if (isMounted) {
          setError(err instanceof Error ? err.message : 'Failed to load WASM module');
          setLoading(false);
        }
      }
    }

    loadWasm();

    return () => {
      isMounted = false;
      if (renderer) {
        renderer.free();
      }
    };
  }, []);

  const createRenderer = () => {
    if (!wasmModule) {
      throw new Error('WASM module not loaded');
    }
    const newRenderer = new wasmModule.PdfRenderer();
    setRenderer(newRenderer);
    return newRenderer;
  };

  return {
    wasmModule,
    renderer,
    createRenderer,
    loading,
    error,
  };
}
