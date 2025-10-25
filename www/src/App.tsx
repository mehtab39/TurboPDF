import React from 'react';
import { PdfViewer } from './components/PdfViewer';
import './App.css';

function App() {
  return (
    <div className="App">
      <PdfViewer scale={1.5} />
    </div>
  );
}

export default App;
