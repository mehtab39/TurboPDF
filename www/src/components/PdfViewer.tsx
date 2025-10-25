import React, { useRef, useEffect, useState } from 'react';
import { usePdfRenderer } from '../hooks/usePdfRenderer';
import './PdfViewer.css';

interface PdfViewerProps {
  fileUrl?: string;
  scale?: number;
}

let x = false;
export const PdfViewer: React.FC<PdfViewerProps> = ({
  fileUrl,
  scale = 1.5
}) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const fileInputRef = useRef<HTMLInputElement>(null);
  const { renderer, createRenderer, loading, error } = usePdfRenderer();

  const [currentPage, setCurrentPage] = useState(0);
  const [totalPages, setTotalPages] = useState(0);
  const [currentScale, setCurrentScale] = useState(scale);
  const [pdfLoaded, setPdfLoaded] = useState(false);
  const [pdfRenderer, setPdfRenderer] = useState<any>(null);

  useEffect(() => {
    if (fileUrl && !loading && !renderer) {
      loadPdfFromUrl(fileUrl);
    }
  }, [fileUrl, loading, renderer]);

  const loadPdfFromUrl = async (url: string) => {
    try {
      const response = await fetch(url);
      const arrayBuffer = await response.arrayBuffer();
      const uint8Array = new Uint8Array(arrayBuffer);


      const newRenderer = createRenderer();
      newRenderer.loadPdf(uint8Array);
      setPdfRenderer(newRenderer);

      const pages = newRenderer.getTotalPages();
      setTotalPages(pages);
      setCurrentPage(0);
      setPdfLoaded(true);

      renderCurrentPage(newRenderer, 0, currentScale);
    } catch (err) {
      console.error('Failed to load PDF from URL:', err);
      alert('Failed to load PDF: ' + (err instanceof Error ? err.message : 'Unknown error'));
    }
  };

  if(!x){
    const utf8decoder = new TextDecoder(); // default 'utf-8'
    const encodedText = new Uint8Array([123,243,233,133]);

    console.log('test:mgc', utf8decoder.decode(encodedText));
    x = true;
  }


 

  const handleFileChange = async (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (!file) return;

    try {
      const arrayBuffer = await file.arrayBuffer();
      const uint8Array = new Uint8Array(arrayBuffer);


      const newRenderer = createRenderer();
      newRenderer.loadPdf(uint8Array);
      setPdfRenderer(newRenderer);

      const pages = newRenderer.getTotalPages();
      setTotalPages(pages);
      setCurrentPage(0);
      setPdfLoaded(true);

      renderCurrentPage(newRenderer, 0, currentScale);
    } catch (err) {
      console.error('Failed to load PDF:', err);
      alert('Failed to load PDF: ' + (err instanceof Error ? err.message : 'Unknown error'));
    }
  };

  const renderCurrentPage = (renderer: any, pageNum: number, scale: number) => {
    if (!canvasRef.current || !renderer) return;

    try {
      renderer.renderPage(canvasRef.current, pageNum, scale);
    } catch (err) {
      console.error('Failed to render page:', err);
      alert('Failed to render page: ' + (err instanceof Error ? err.message : 'Unknown error'));
    }
  };

  const goToPage = (pageNum: number) => {
    if (!pdfRenderer || pageNum < 0 || pageNum >= totalPages) return;

    setCurrentPage(pageNum);
    pdfRenderer.setCurrentPage(pageNum);
    renderCurrentPage(pdfRenderer, pageNum, currentScale);
  };

  const previousPage = () => {
    if (currentPage > 0) {
      goToPage(currentPage - 1);
    }
  };

  const nextPage = () => {
    if (currentPage < totalPages - 1) {
      goToPage(currentPage + 1);
    }
  };

  const zoomIn = () => {
    const newScale = currentScale + 0.25;
    setCurrentScale(newScale);
    if (pdfRenderer && pdfLoaded) {
      renderCurrentPage(pdfRenderer, currentPage, newScale);
    }
  };

  const zoomOut = () => {
    const newScale = Math.max(0.5, currentScale - 0.25);
    setCurrentScale(newScale);
    if (pdfRenderer && pdfLoaded) {
      renderCurrentPage(pdfRenderer, currentPage, newScale);
    }
  };

  if (loading) {
    return <div className="pdf-viewer-loading">Loading PDF renderer...</div>;
  }

  if (error) {
    return <div className="pdf-viewer-error">Error: {error}</div>;
  }

  return (
    <div className="pdf-viewer-container">
      <div className="pdf-viewer-toolbar">
        <input
          ref={fileInputRef}
          type="file"
          accept=".pdf"
          onChange={handleFileChange}
          className="pdf-file-input"
        />

        {pdfLoaded && (
          <>
            <div className="pdf-controls">
              <button
                onClick={previousPage}
                disabled={currentPage === 0}
                className="pdf-button"
              >
                Previous
              </button>

              <span className="pdf-page-info">
                Page {currentPage + 1} of {totalPages}
              </span>

              <button
                onClick={nextPage}
                disabled={currentPage === totalPages - 1}
                className="pdf-button"
              >
                Next
              </button>
            </div>

            <div className="pdf-zoom-controls">
              <button onClick={zoomOut} className="pdf-button">
                Zoom Out
              </button>
              <span className="pdf-zoom-info">
                {Math.round(currentScale * 100)}%
              </span>
              <button onClick={zoomIn} className="pdf-button">
                Zoom In
              </button>
            </div>
          </>
        )}
      </div>

      <div className="pdf-canvas-container">
        <canvas ref={canvasRef} className="pdf-canvas" />
      </div>
    </div>
  );
};
