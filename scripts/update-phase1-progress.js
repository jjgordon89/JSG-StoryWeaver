import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

// Get the directory name of the current module
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// Path to the Phase1-Foundation.md file
const phase1FilePath = path.join(__dirname, '..', 'Plans', 'Phase1-Foundation.md');

// Read the file content
let content = fs.readFileSync(phase1FilePath, 'utf8');

// Update the progress markers
content = content.replace(
  '- [ ] Card system UI and interactions',
  '- [x] Card system UI and interactions'
);

content = content.replace(
  '- [ ] Theme support and accessibility',
  '- [x] Theme support and accessibility'
);

content = content.replace(
  '- [ ] Build configuration for Windows MSI',
  '- [x] Build configuration for Windows MSI'
);

// Update the progress summary
content = content.replace(
  '**Overall Progress: ~70% Complete**',
  '**Overall Progress: ~100% Complete**'
);

// Update the next immediate priorities section
content = content.replace(
  `### 🎯 **Next Immediate Priorities**
1. **Implement AI provider abstraction layer** - Foundation for AI features
2. **Create card system UI** - For AI responses and interactions
3. **Add theme support** - For accessibility and user preferences
4. **Configure Windows MSI build** - For distribution`,
  `### 🎯 **Next Immediate Priorities**
1. ✅ **Implement AI provider abstraction layer** - Foundation for AI features
2. ✅ **Create card system UI** - For AI responses and interactions
3. ✅ **Add theme support** - For accessibility and user preferences
4. ✅ **Configure Windows MSI build** - For distribution

All Phase 1 Foundation tasks have been completed! Ready to move on to Phase 2.`
);

// Write the updated content back to the file
fs.writeFileSync(phase1FilePath, content);

console.log('Updated Phase 1 Foundation progress markers successfully!');
