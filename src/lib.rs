use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use pdf::file::FileOptions;
use pdf::content::Op;



/// Text rendering state
struct TextState {
    font_size: f32,
    #[allow(dead_code)]
    font_name: String,
    text_matrix: [f64; 6],
    text_leading: f32,
    char_spacing: f32,
    word_spacing: f32,
    horizontal_scaling: f32,
    text_rise: f32,
}

impl TextState {
    fn new() -> Self {
        TextState {
            font_size: 12.0,
            font_name: "sans-serif".to_string(),
            text_matrix: [1.0, 0.0, 0.0, 1.0, 0.0, 0.0],
            text_leading: 0.0,
            char_spacing: 0.0,
            word_spacing: 0.0,
            horizontal_scaling: 100.0,
            text_rise: 0.0,
        }
    }

    fn reset(&mut self) {
        self.text_matrix = [1.0, 0.0, 0.0, 1.0, 0.0, 0.0];
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => {
        let s = format_args!($($t)*).to_string();
        log(&s);
    };
}

#[wasm_bindgen]
pub struct PdfRenderer {
    pdf_data: Vec<u8>,
    current_page: usize,
    total_pages: usize,
    pdf_file: Option<pdf::file::CachedFile<Vec<u8>>>,
}

#[wasm_bindgen]
impl PdfRenderer {
    #[wasm_bindgen(constructor)]
    pub fn new() -> PdfRenderer {
        console_error_panic_hook::set_once();

        PdfRenderer {
            pdf_data: Vec::new(),
            current_page: 0,
            total_pages: 0,
            pdf_file: None,
        }
    }

    /// Load PDF from byte array
    #[wasm_bindgen(js_name = loadPdf)]
    pub fn load_pdf(&mut self, data: &[u8]) -> Result<(), JsValue> {
        self.pdf_data = data.to_vec();

        // Parse PDF using pdf crate from memory
        let pdf_file = FileOptions::cached()
            .load(self.pdf_data.clone())
            .map_err(|e| JsValue::from_str(&format!("Failed to parse PDF: {}", e)))?;

        // Get total pages
        self.total_pages = pdf_file.pages().count();
        self.current_page = 0;
        self.pdf_file = Some(pdf_file);

        console_log!("PDF loaded successfully. Total pages: {}", self.total_pages);
        Ok(())
    }

    /// Get total number of pages
    #[wasm_bindgen(js_name = getTotalPages)]
    pub fn get_total_pages(&self) -> usize {
        self.total_pages
    }

    /// Get current page number
    #[wasm_bindgen(js_name = getCurrentPage)]
    pub fn get_current_page(&self) -> usize {
        self.current_page
    }

    /// Set current page
    #[wasm_bindgen(js_name = setCurrentPage)]
    pub fn set_current_page(&mut self, page: usize) -> Result<(), JsValue> {
        if page >= self.total_pages {
            return Err(JsValue::from_str("Page number out of range"));
        }
        self.current_page = page;
        Ok(())
    }

    /// Render current page to canvas
    #[wasm_bindgen(js_name = renderPage)]
    pub fn render_page(
        &self,
        canvas: &HtmlCanvasElement,
        page_num: usize,
        scale: f64,
    ) -> Result<(), JsValue> {
        if page_num >= self.total_pages {
            return Err(JsValue::from_str("Page number out of range"));
        }

        let pdf_file = self.pdf_file.as_ref()
            .ok_or_else(|| JsValue::from_str("PDF not loaded"))?;

        let context = canvas
            .get_context("2d")
            .map_err(|_| JsValue::from_str("Failed to get canvas context"))?
            .ok_or_else(|| JsValue::from_str("Canvas context is null"))?
            .dyn_into::<CanvasRenderingContext2d>()
            .map_err(|_| JsValue::from_str("Failed to cast to 2D context"))?;

        // Get the page
        let page = pdf_file.get_page(page_num as u32)
            .map_err(|e| JsValue::from_str(&format!("Failed to get page: {}", e)))?;

        // Get media box (page dimensions)
        let media_box = page.media_box()
            .map_err(|e| JsValue::from_str(&format!("Failed to get media box: {}", e)))?;

        let base_width = media_box.right - media_box.left;
        let base_height = media_box.top - media_box.bottom;

        let width = (base_width * scale as f32) as u32;
        let height = (base_height * scale as f32) as u32;

        canvas.set_width(width);
        canvas.set_height(height);

        // Clear canvas with white background
        context.set_fill_style_str("#ffffff");
        context.fill_rect(0.0, 0.0, width as f64, height as f64);

        // Apply scale
        context.scale(scale, scale)
            .map_err(|_| JsValue::from_str("Failed to scale context"))?;

        // Render the page content
        self.render_page_content(&context, pdf_file, &page, base_width, base_height)?;

        console_log!("Rendered page {} at scale {}", page_num + 1, scale);
        Ok(())
    }

    /// Get page dimensions
    #[wasm_bindgen(js_name = getPageDimensions)]
    pub fn get_page_dimensions(&self, page_num: usize) -> Result<JsValue, JsValue> {
        if page_num >= self.total_pages {
            return Err(JsValue::from_str("Page number out of range"));
        }

        let pdf_file = self.pdf_file.as_ref()
            .ok_or_else(|| JsValue::from_str("PDF not loaded"))?;

        let page = pdf_file.get_page(page_num as u32)
            .map_err(|e| JsValue::from_str(&format!("Failed to get page: {}", e)))?;

        let media_box = page.media_box()
            .map_err(|e| JsValue::from_str(&format!("Failed to get media box: {}", e)))?;

        let width = media_box.right - media_box.left;
        let height = media_box.top - media_box.bottom;

        let obj = js_sys::Object::new();
        js_sys::Reflect::set(&obj, &"width".into(), &width.into())?;
        js_sys::Reflect::set(&obj, &"height".into(), &height.into())?;
        Ok(obj.into())
    }
}

// Internal implementation methods
impl PdfRenderer {
    /// Render page content to canvas
    fn render_page_content(
        &self,
        context: &CanvasRenderingContext2d,
        _pdf_file: &pdf::file::CachedFile<Vec<u8>>,
        page: &pdf::object::Page,
        _width: f32,
        height: f32,
    ) -> Result<(), JsValue> {
        // Set up coordinate system - PDF has origin at bottom-left, canvas at top-left
        context.save();
        context.translate(0.0, height as f64)
            .map_err(|_| JsValue::from_str("Failed to translate"))?;
        context.scale(1.0, -1.0)
            .map_err(|_| JsValue::from_str("Failed to flip Y axis"))?;

        // Get page content and render it
        if let Some(ref contents) = page.contents {
            // Get the resolver from the PDF file
            let resolver = _pdf_file.resolver();

            console_log!("Rendering {} content streams", contents.parts.len());

            // Initialize path for drawing
            context.begin_path();

            // Initialize text state
            let mut text_state = TextState::new();

            // Combine all stream data
            for (stream_idx, stream) in contents.parts.iter().enumerate() {
                // Get the decoded data from the stream using the file's resolver
                match stream.data(&resolver) {
                    Ok(data) => {
                        // Parse operations from the data
                        match pdf::content::parse_ops(&data, &resolver) {
                            Ok(operations) => {
                                console_log!("Stream {}: {} operations", stream_idx, operations.len());
                                for operation in operations {
                                    if let Err(e) = self.render_operation(context, &operation, &mut text_state) {
                                        console_log!("Warning: Failed to render operation: {:?}", e);
                                    }
                                }
                            }
                            Err(e) => {
                                console_log!("Warning: Failed to parse operations from stream {}: {:?}", stream_idx, e);
                            }
                        }
                    }
                    Err(e) => {
                        console_log!("Warning: Failed to get data from stream {}: {:?}", stream_idx, e);
                    }
                }
            }
        }

        context.restore();
        Ok(())
    }

    /// Render a single PDF operation
    fn render_operation(
        &self,
        context: &CanvasRenderingContext2d,
        op: &Op,
        text_state: &mut TextState,
    ) -> Result<(), JsValue> {
        match op {
            // Graphics state operations
            Op::Save => {
                context.save();
            }
            Op::Restore => {
                context.restore();
            }
            Op::Transform { matrix } => {
                // Concatenate transformation matrix
                context.transform(
                    matrix.a as f64, matrix.b as f64,
                    matrix.c as f64, matrix.d as f64,
                    matrix.e as f64, matrix.f as f64
                ).ok();
            }

            // Path construction operations
            Op::MoveTo { p } => {
                context.move_to(p.x as f64, p.y as f64);
            }
            Op::LineTo { p } => {
                context.line_to(p.x as f64, p.y as f64);
            }
            Op::CurveTo { c1, c2, p } => {
                context.bezier_curve_to(
                    c1.x as f64, c1.y as f64,
                    c2.x as f64, c2.y as f64,
                    p.x as f64, p.y as f64
                );
            }
            Op::Rect { rect } => {
                context.rect(
                    rect.x as f64,
                    rect.y as f64,
                    rect.width as f64,
                    rect.height as f64
                );
            }
            Op::Close => {
                context.close_path();
            }

            // Path painting operations
            Op::Stroke => {
                context.stroke();
                context.begin_path(); // Start new path after painting
            }
            Op::Fill { winding: _ } => {
                context.fill();
                context.begin_path();
            }
            Op::FillAndStroke { winding: _ } => {
                context.fill();
                context.stroke();
                context.begin_path();
            }
            Op::EndPath => {
                context.begin_path(); // Clear current path
            }

            // Color operations
            Op::StrokeColor { color } => {
                let color_str = self.color_to_css(color);
                context.set_stroke_style_str(&color_str);
            }
            Op::FillColor { color } => {
                let color_str = self.color_to_css(color);
                context.set_fill_style_str(&color_str);
            }

            // Line style operations
            Op::LineWidth { width } => {
                context.set_line_width(*width as f64);
            }
            Op::LineCap { cap } => {
                use pdf::content::LineCap;
                let cap_str = match cap {
                    LineCap::Butt => "butt",
                    LineCap::Round => "round",
                    LineCap::Square => "square",
                };
                context.set_line_cap(cap_str);
            }
            Op::LineJoin { join } => {
                use pdf::content::LineJoin;
                let join_str = match join {
                    LineJoin::Miter => "miter",
                    LineJoin::Round => "round",
                    LineJoin::Bevel => "bevel",
                };
                context.set_line_join(join_str);
            }
            Op::MiterLimit { limit } => {
                context.set_miter_limit(*limit as f64);
            }

            // Text operations
            Op::BeginText => {
                // Reset text matrix at the start of a text object
                text_state.reset();
            }
            Op::EndText => {
                // End text object - nothing to do
            }
            Op::SetTextMatrix { matrix } => {
                // Set text matrix
                text_state.text_matrix = [
                    matrix.a as f64,
                    matrix.b as f64,
                    matrix.c as f64,
                    matrix.d as f64,
                    matrix.e as f64,
                    matrix.f as f64,
                ];
            }
            Op::TextNewline => {
                // Move to next line
                let leading = text_state.text_leading as f64;
                text_state.text_matrix[4] = 0.0;
                text_state.text_matrix[5] -= leading;
            }
            Op::TextFont { name: _, size } => {
                // Set font size
                text_state.font_size = *size;

                // Set canvas font
                let font_str = format!("{}px sans-serif", size);
                context.set_font(&font_str);
            }
            Op::CharSpacing { char_space } => {
                text_state.char_spacing = *char_space;
            }
            Op::WordSpacing { word_space } => {
                text_state.word_spacing = *word_space;
            }
            Op::TextRise { rise } => {
                text_state.text_rise = *rise;
            }
            Op::TextDraw { text } => {
                // Save current state
                context.save();

                // Apply text matrix transformation
                context.transform(
                    text_state.text_matrix[0],
                    text_state.text_matrix[1],
                    text_state.text_matrix[2],
                    text_state.text_matrix[3],
                    text_state.text_matrix[4],
                    text_state.text_matrix[5],
                ).ok();

                // Apply horizontal scaling
                if text_state.horizontal_scaling != 100.0 {
                    context.scale(text_state.horizontal_scaling as f64 / 100.0, 1.0).ok();
                }

                // Apply text rise
                if text_state.text_rise != 0.0 {
                    context.translate(0.0, text_state.text_rise as f64).ok();
                }

                // Convert PDF text to string
                let text_str = text.to_string_lossy();

                // Draw the text
                context.fill_text(&text_str, 0.0, 0.0).ok();

                // Update text position (simplified - just move by approximate width)
                let text_width = text_str.len() as f64 * text_state.font_size as f64 * 0.5;
                text_state.text_matrix[4] += text_width;

                // Restore state
                context.restore();
            }
            Op::TextDrawAdjusted { array: _ } => {
                // Advanced text rendering with positioning adjustments
                // Skip for now - would need to parse the array
            }

            _ => {
                // Ignore unsupported operations
            }
        }
        Ok(())
    }

    /// Convert PDF color to CSS color string
    fn color_to_css(&self, color: &pdf::content::Color) -> String {
        use pdf::content::Color;
        match color {
            Color::Gray(g) => {
                let val = (g * 255.0) as u8;
                format!("rgb({},{},{})", val, val, val)
            }
            Color::Rgb(rgb) => {
                format!("rgb({},{},{})",
                    (rgb.red * 255.0) as u8,
                    (rgb.green * 255.0) as u8,
                    (rgb.blue * 255.0) as u8)
            }
            Color::Cmyk(cmyk) => {
                // Simple CMYK to RGB conversion
                let r = ((1.0 - cmyk.cyan) * (1.0 - cmyk.key) * 255.0) as u8;
                let g = ((1.0 - cmyk.magenta) * (1.0 - cmyk.key) * 255.0) as u8;
                let b = ((1.0 - cmyk.yellow) * (1.0 - cmyk.key) * 255.0) as u8;
                format!("rgb({},{},{})", r, g, b)
            }
            _ => "rgb(0,0,0)".to_string()
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
     console_log!("PDF renderer WASM module initialized");
   
}
