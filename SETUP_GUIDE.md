# WebAssembly Setup Guide - Step by Step

This guide provides detailed instructions for setting up your WebAssembly development environment and completing the assignment.

## 📋 Assignment Checklist

- [ ] Install required tools (Rust, wasm-pack, Node.js)
- [ ] Create and set up the project repository
- [ ] Implement the Rust WebAssembly code
- [ ] Create the JavaScript interface
- [ ] Build and test the application
- [ ] Write the research article
- [ ] Push everything to GitHub
- [ ] Submit the assignment

## 🔧 Step 1: Install Development Tools

### Install Rust
```bash
# Install Rust using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Restart your shell or run:
source ~/.cargo/env

# Verify installation
rustc --version
cargo --version
```

### Install wasm-pack
```bash
# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Verify installation
wasm-pack --version
```

### Install Node.js
Download and install Node.js from [nodejs.org](https://nodejs.org/), then verify:
```bash
node --version
npm --version
```

## 🚀 Step 2: Create Project Structure

### Initialize Rust Project
```bash
# Create new Rust library project
cargo new --lib wasm-game-of-life
cd wasm-game-of-life

# Initialize git repository
git init
git add .
git commit -m "Initial commit"
```

### Create Required Files
Create these files in your project directory:

1. **Copy the Cargo.toml configuration** (from the artifacts above)
2. **Copy the src/lib.rs implementation** (from the artifacts above)
3. **Create index.html** with the complete HTML/JavaScript code
4. **Create package.json** for Node.js dependencies
5. **Create .gitignore** file
6. **Create README.md** for documentation

## 🔨 Step 3: Build and Test

### Build the WebAssembly Module
```bash
# Build for development (with debug info)
wasm-pack build --dev --target web --out-dir pkg

# Or build for production (optimized)
wasm-pack build --target web --out-dir pkg
```

### Serve the Application
```bash
# Using Python (if installed)
python -m http.server 8000

# Or using Node.js
npx serve .

# Or using a local development server
npm install -g live-server
live-server
```

### Test the Application
1. Open `http://localhost:8000` in your browser
2. Test all functionality:
   - Click cells to toggle them
   - Use play/pause controls
   - Try different patterns
   - Check performance metrics

## 📝 Step 4: Write the Research Article

Using the provided template, create your research article including:

1. **Introduction (max 500 words)**
   - What is WebAssembly?
   - Role in your future career
   - Integration with your degree

2. **Tutorial Section**
   - Complete step-by-step implementation
   - Code explanations
   - System interaction analysis

3. **Citations and References**
   - Use proper academic citation format
   - Include all sources used

## 🐛 Troubleshooting Common Issues

### Issue: "wasm-pack not found"
**Solution:**
```bash
# Re-install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
# Or install via cargo
cargo install wasm-pack
```

### Issue: "Cannot load WebAssembly module"
**Solution:**
- Ensure you're serving over HTTP, not file://
- Check browser console for errors
- Verify WebAssembly support: `WebAssembly` object should exist in browser console

### Issue: Build fails with dependency errors
**Solution:**
```bash
# Update Rust
rustup update

# Clean build
cargo clean
wasm-pack build --target web
```

### Issue: CORS errors when loading
**Solution:**
- Always use a local server (python -m http.server, npx serve, etc.)
- Never open HTML file directly in browser

## 📊 Testing Your Implementation

### Functional Tests
- [ ] Cells toggle when clicked
- [ ] Play/pause button works
- [ ] Clear button resets grid
- [ ] Random button generates random pattern
- [ ] Glider pattern spawns correctly
- [ ] Speed controls change animation rate
- [ ] Generation counter updates

### Performance Tests
- [ ] Maintains 60 FPS on 64x64 grid
- [ ] Memory usage stays reasonable
- [ ] No memory leaks during long runs
- [ ] Responsive to user interactions

### Cross-Browser Tests
- [ ] Chrome/Chromium
- [ ] Firefox
- [ ] Safari (if on macOS)
- [ ] Edge (if on Windows)

## 🎯 Optimization Tips

### For Better Performance
```rust
// In Cargo.toml, use release optimizations
[profile.release]
opt-level = "s"  // Optimize for size
lto = true       // Link-time optimization
```

### For Smaller WASM Size
- Only import needed web-sys features
- Use `wee_alloc` for smaller memory allocator
- Strip debug symbols in release builds

### For Better User Experience
- Add loading indicators
- Implement error handling
- Provide keyboard shortcuts
- Add touch/mobile support

## 📤 Step 5: GitHub Submission

### Create GitHub Repository
1. Create new repository on GitHub
2. Clone it locally or add remote to existing project
3. Push your code:

```bash
git remote add origin https://github.com/yourusername/wasm-game-of-life.git
git branch -M main
git push -u origin main
```

### Repository Structure for Submission
Ensure your GitHub repository contains:
```
wasm-game-of-life/
├── src/
│   └── lib.rs              # Your Rust implementation
├── index.html              # Web interface
├── Cargo.toml             # Rust configuration
├── package.json           # Node.js configuration  
├── README.md              # Documentation
├── .gitignore             # Git ignore rules
└── SETUP_GUIDE.md         # This guide
```

### Enable GitHub Pages (Optional)
1. Go to repository Settings
2. Scroll to Pages section
3. Select "Deploy from a branch"
4. Choose "main" branch and "/ (root)"
5. Your demo will be available at: `https://yourusername.github.io/wasm-game-of-life`

## 📋 Final Assignment Submission

### What to Submit to ClickUP
1. **PDF Document**: Your complete research article (from the main artifact)
2. **GitHub Repository Link**: Include in your PDF document
3. **Ensure GitHub Repository Contains**:
   - Complete working code
   - Proper documentation
   - All required files

### PDF Document Checklist
- [ ] Title page with your name and student details
- [ ] Introduction section (max 500 words)
- [ ] Complete tutorial with code explanations
- [ ] System architecture analysis
- [ ] Performance discussion
- [ ] Proper citations and references
- [ ] GitHub repository link included
- [ ] Professional formatting

## 🎓 Grading Criteria Alignment

### Writing Quality (20%)
- Clear, coherent explanations
- Proper grammar and formatting
- Academic writing style
- Appropriate technical language

### Purpose and Context (15%)
- Explains WebAssembly's purpose clearly
- Connects to degree program
- Discusses future career relevance
- Addresses assignment objectives

### Technical Accuracy (20%)
- Correct WebAssembly information
- Accurate code implementations
- Proper technical explanations
- Up-to-date information (2025)

### Code Explanation (20%)
- Detailed explanation of each code section
- Clear reasoning for design choices
- Explanation of system interactions
- Tutorial clarity and completeness

### Completeness (15%)
- All required sections included
- Working code implementation
- Complete tutorial from setup to deployment
- Thorough testing coverage

### Structure and Citations (10%)
- Logical flow of information
- Proper academic citations
- Professional presentation
- Well-organized repository

## 🔄 Iterative Development Process

### Phase 1: Basic Implementation
1. Get basic Conway's Game of Life working
2. Simple grid rendering
3. Basic play/pause functionality

### Phase 2: Enhanced Features  
1. Add pattern insertion
2. Implement user interaction (clicking cells)
3. Add performance metrics
4. Improve visual design

### Phase 3: Polish and Documentation
1. Write comprehensive documentation
2. Add error handling
3. Cross-browser testing
4. Performance optimization

### Phase 4: Research Article
1. Write introduction section
2. Create detailed tutorial
3. Add system analysis
4. Include proper citations

## 📚 Additional Learning Resources

### WebAssembly Fundamentals
- [WebAssembly.org Official Site](https://webassembly.org/)
- [MDN WebAssembly Guide](https://developer.mozilla.org/en-US/docs/WebAssembly)
- [WebAssembly Specification](https://webassembly.github.io/spec/)

### Rust and WebAssembly
- [Rust and WebAssembly Book](https://rustwasm.github.io/book/)
- [wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/)
- [wasm-pack Documentation](https://rustwasm.github.io/wasm-pack/)

### Conway's Game of Life
- [Game of Life Wikipedia](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life)
- [LifeWiki Pattern Collection](https://www.conwaylife.com/wiki/Main_Page)
- [Cellular Automata Theory](https://mathworld.wolfram.com/CellularAutomaton.html)

## 🎯 Advanced Extensions (Optional)

If you want to go beyond the basic requirements:

### Performance Enhancements
```rust
// Add multithreading support
use rayon::prelude::*;

// Parallel processing for large grids
pub fn tick_parallel(&mut self) {
    let mut next = self.cells.clone();
    
    next.par_chunks_mut(self.width as usize)
        .enumerate()
        .for_each(|(row, chunk)| {
            // Process row in parallel
        });
}
```

### Additional Patterns
- Implement pattern loading from RLE format
- Add pattern library with famous configurations
- Include pattern rotation and reflection

### Advanced UI Features
- Grid zooming and panning
- Pattern editor mode
- Save/load functionality
- Animation recording

### Mobile Support
```css
/* Add responsive design */
@media (max-width: 768px) {
    canvas {
        width: 100%;
        height: auto;
    }
    .controls {
        flex-direction: column;
    }
}
```

## 🔍 Code Review Checklist

Before submission, review your code for:

### Rust Code Quality
- [ ] Proper error handling
- [ ] Efficient algorithms
- [ ] Clear variable names
- [ ] Adequate comments
- [ ] Memory safety

### JavaScript Code Quality  
- [ ] Event listener cleanup
- [ ] Error handling for WASM loading
- [ ] Performance optimizations
- [ ] Browser compatibility

### General Quality
- [ ] Consistent code style
- [ ] No hardcoded values
- [ ] Proper documentation
- [ ] Clean git history
- [ ] Professional README

## 📞 Getting Help

### If You Get Stuck
1. **Check browser console** for error messages
2. **Read error messages carefully** - they often contain solutions
3. **Test in different browsers** to isolate issues
4. **Use browser developer tools** for debugging
5. **Check GitHub issues** for similar problems
6. **Ask for help early** rather than struggling alone

### Debugging Tips
```javascript
// Add logging to debug WASM interactions
console.log("Universe created with dimensions:", universe.width(), "x", universe.height());

// Check WebAssembly support
if (typeof WebAssembly === 'object') {
    console.log("WebAssembly is supported");
} else {
    console.error("WebAssembly not supported in this browser");
}
```

## ✅ Final Checklist

Before submitting your assignment:

### Technical Requirements
- [ ] Code compiles without errors
- [ ] Application runs in browser
- [ ] All features work as expected
- [ ] GitHub repository is public and accessible
- [ ] README contains setup instructions

### Academic Requirements  
- [ ] Article follows assignment structure
- [ ] Word count within limits (introduction ≤ 500 words)
- [ ] Proper citations included
- [ ] Professional presentation
- [ ] PDF format for submission

### Quality Assurance
- [ ] Tested on multiple browsers
- [ ] No broken links or references
- [ ] Code is well-commented
- [ ] Documentation is clear and complete
- [ ] Repository is well-organized

---

**Good luck with your WebAssembly assignment! 🚀**

Remember: The goal is to demonstrate your understanding of WebAssembly through both implementation and explanation. Focus on clarity, accuracy, and completeness rather than complexity.