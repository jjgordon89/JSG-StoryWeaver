<template>
  <div class="image-generator">
    <!-- Generation Form -->
    <div class="generation-form">
      <div class="form-header">
        <h3>Image Generation</h3>
        <p class="form-description">
          Generate images to visualize scenes, characters, and settings from your story.
        </p>
      </div>

      <!-- Prompt Input -->
      <div class="prompt-section">
        <label for="image-prompt">Image Description:</label>
        <textarea
          id="image-prompt"
          v-model="prompt"
          placeholder="Describe the image you want to generate..."
          rows="3"
          class="prompt-textarea"
        ></textarea>
        
        <!-- Quick Prompts -->
        <div class="quick-prompts">
          <span class="quick-prompts-label">Quick prompts:</span>
          <div class="quick-prompt-buttons">
            <button 
              v-for="quickPrompt in quickPrompts" 
              :key="quickPrompt.label"
              @click="addQuickPrompt(quickPrompt.text)"
              class="quick-prompt-button"
            >
              {{ quickPrompt.label }}
            </button>
          </div>
        </div>
      </div>

      <!-- Generation Options -->
      <div class="generation-options">
        <div class="options-grid">
          <!-- Style -->
          <div class="option-group">
            <label for="image-style">Art Style:</label>
            <select id="image-style" v-model="selectedStyle" class="option-select">
              <option value="realistic">Realistic</option>
              <option value="fantasy">Fantasy Art</option>
              <option value="anime">Anime/Manga</option>
              <option value="oil-painting">Oil Painting</option>
              <option value="watercolor">Watercolor</option>
              <option value="digital-art">Digital Art</option>
              <option value="sketch">Pencil Sketch</option>
              <option value="concept-art">Concept Art</option>
            </select>
          </div>

          <!-- Resolution -->
          <div class="option-group">
            <label for="image-resolution">Resolution:</label>
            <select id="image-resolution" v-model="selectedResolution" class="option-select">
              <option 
                v-for="resolution in availableResolutions" 
                :key="resolution.name"
                :value="resolution.name"
              >
                {{ resolution.name }} ({{ resolution.width }}x{{ resolution.height }}) - {{ resolution.credit_cost }} credits
              </option>
            </select>
          </div>

          <!-- Aspect Ratio -->
          <div class="option-group">
            <label for="aspect-ratio">Aspect Ratio:</label>
            <select id="aspect-ratio" v-model="aspectRatio" class="option-select">
              <option value="1:1">Square (1:1)</option>
              <option value="16:9">Landscape (16:9)</option>
              <option value="9:16">Portrait (9:16)</option>
              <option value="4:3">Classic (4:3)</option>
              <option value="3:2">Photo (3:2)</option>
            </select>
          </div>

          <!-- Quality -->
          <div class="option-group">
            <label for="image-quality">Quality:</label>
            <select id="image-quality" v-model="imageQuality" class="option-select">
              <option value="draft">Draft (Fast, Lower Cost)</option>
              <option value="standard">Standard</option>
              <option value="high">High Quality</option>
              <option value="ultra">Ultra High (Slow, Higher Cost)</option>
            </select>
          </div>
        </div>

        <!-- Advanced Options -->
        <div class="advanced-options">
          <div class="option-toggles">
            <label class="toggle-option">
              <input type="checkbox" v-model="enhancePrompt">
              <span class="toggle-label">
                Auto-enhance prompt
                <i class="fas fa-info-circle" title="Automatically improve the prompt for better results"></i>
              </span>
            </label>

            <label class="toggle-option">
              <input type="checkbox" v-model="useStoryContext">
              <span class="toggle-label">
                Use story context
                <i class="fas fa-info-circle" title="Include relevant story elements in the generation"></i>
              </span>
            </label>

            <label class="toggle-option">
              <input type="checkbox" v-model="generateVariations">
              <span class="toggle-label">
                Generate variations (2x cost)
                <i class="fas fa-info-circle" title="Generate multiple variations of the same prompt"></i>
              </span>
            </label>
          </div>
        </div>
      </div>

      <!-- Cost Estimate -->
      <div class="cost-estimate">
        <div class="cost-info">
          <span class="cost-label">Estimated cost:</span>
          <span class="cost-value">{{ estimatedCost }} credits</span>
        </div>
        <div class="remaining-credits" v-if="remainingCredits !== undefined">
          <span class="credits-remaining">{{ remainingCredits }} credits remaining</span>
        </div>
      </div>

      <!-- Action Buttons -->
      <div class="action-buttons">
        <button 
          @click="generateImage"
          :disabled="!canGenerate"
          class="generate-button primary"
        >
          <i class="fas fa-image"></i>
          {{ isGenerating ? 'Generating...' : 'Generate Image' }}
        </button>
        
        <button 
          @click="clearForm"
          :disabled="isGenerating"
          class="clear-button secondary"
        >
          <i class="fas fa-eraser"></i>
          Clear
        </button>
      </div>
    </div>

    <!-- Generated Images Gallery -->
    <div class="images-gallery" v-if="generatedImages.length > 0">
      <div class="gallery-header">
        <h4>Generated Images</h4>
        <div class="gallery-controls">
          <button @click="viewMode = 'grid'" :class="{ active: viewMode === 'grid' }" class="view-button">
            <i class="fas fa-th"></i>
          </button>
          <button @click="viewMode = 'list'" :class="{ active: viewMode === 'list' }" class="view-button">
            <i class="fas fa-list"></i>
          </button>
        </div>
      </div>

      <div :class="['images-container', viewMode]">
        <div 
          v-for="image in generatedImages" 
          :key="image.id"
          class="image-item"
          @click="selectImage(image)"
        >
          <div class="image-wrapper">
            <img 
              :src="image.file_path" 
              :alt="image.prompt"
              class="generated-image"
              @error="handleImageError"
            >
            <div class="image-overlay">
              <div class="image-actions">
                <button @click.stop="downloadImage(image)" class="action-btn" title="Download">
                  <i class="fas fa-download"></i>
                </button>
                <button @click.stop="copyImageUrl(image)" class="action-btn" title="Copy URL">
                  <i class="fas fa-link"></i>
                </button>
                <button @click.stop="deleteImage(image)" class="action-btn danger" title="Delete">
                  <i class="fas fa-trash"></i>
                </button>
              </div>
            </div>
          </div>
          
          <div class="image-info">
            <div class="image-prompt">{{ truncateText(image.prompt, 60) }}</div>
            <div class="image-metadata">
              <span class="metadata-item">{{ image.resolution.width }}x{{ image.resolution.height }}</span>
              <span class="metadata-item">{{ image.style }}</span>
              <span class="metadata-item">{{ formatDate(image.created_at) }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Image Detail Modal -->
    <ImageDetailModal 
      v-if="selectedImage"
      :image="selectedImage"
      @close="selectedImage = null"
      @delete="onImageDeleted"
      @download="downloadImage"
    />

    <!-- Generation Progress -->
    <div v-if="isGenerating" class="generation-progress">
      <div class="progress-content">
        <div class="progress-spinner"></div>
        <div class="progress-text">
          <h4>Generating Image...</h4>
          <p>This may take 30-60 seconds depending on complexity and quality settings.</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useAdvancedAIStore } from '../../stores/advancedAIStore';
import { useProjectStore } from '../../stores/projectStore';
import ImageDetailModal from './ImageDetailModal.vue';
import type { 
  ImageGenerationRequest, 
  GeneratedImage, 
  ImageResolution 
} from '../../types/advancedAI';

// Emits
const emit = defineEmits<{
  'image-generated': [image: GeneratedImage];
}>();

// Stores
const advancedAIStore = useAdvancedAIStore();
const projectStore = useProjectStore();

// Reactive state
const prompt = ref('');
const selectedStyle = ref('realistic');
const selectedResolution = ref('medium');
const aspectRatio = ref('16:9');
const imageQuality = ref('standard');
const enhancePrompt = ref(true);
const useStoryContext = ref(false);
const generateVariations = ref(false);
const viewMode = ref<'grid' | 'list'>('grid');
const selectedImage = ref<GeneratedImage | null>(null);

// Available resolutions with credit costs
const availableResolutions: ImageResolution[] = [
  { name: 'small', width: 512, height: 512, credit_cost: 1 },
  { name: 'medium', width: 768, height: 768, credit_cost: 2 },
  { name: 'large', width: 1024, height: 1024, credit_cost: 3 },
  { name: 'xl', width: 1536, height: 1536, credit_cost: 5 }
];

// Quick prompt templates
const quickPrompts = [
  { label: 'Character Portrait', text: 'detailed character portrait, ' },
  { label: 'Landscape Scene', text: 'beautiful landscape scene, ' },
  { label: 'Interior Setting', text: 'detailed interior room, ' },
  { label: 'Action Scene', text: 'dynamic action scene, ' },
  { label: 'Atmospheric Mood', text: 'atmospheric and moody scene, ' },
  { label: 'Fantasy Setting', text: 'magical fantasy environment, ' }
];

// Computed properties
const generatedImages = computed(() => advancedAIStore.generatedImages);
const isGenerating = computed(() => advancedAIStore.isGeneratingImage);
const remainingCredits = computed(() => advancedAIStore.remainingCredits);

const currentResolution = computed(() => {
  return availableResolutions.find(r => r.name === selectedResolution.value) || availableResolutions[1];
});

const estimatedCost = computed(() => {
  let baseCost = currentResolution.value.credit_cost;
  
  // Quality multipliers
  const qualityMultipliers = {
    draft: 0.5,
    standard: 1,
    high: 1.5,
    ultra: 2
  };
  
  baseCost *= qualityMultipliers[imageQuality.value as keyof typeof qualityMultipliers];
  
  // Variations multiplier
  if (generateVariations.value) {
    baseCost *= 2;
  }
  
  return Math.ceil(baseCost);
});

const canGenerate = computed(() => {
  return prompt.value.trim().length > 0 && 
         !isGenerating.value && 
         projectStore.currentProject &&
         (remainingCredits.value === undefined || remainingCredits.value >= estimatedCost.value);
});

// Methods
const addQuickPrompt = (text: string) => {
  if (prompt.value && !prompt.value.endsWith(' ')) {
    prompt.value += ' ';
  }
  prompt.value += text;
};

const generateImage = async () => {
  if (!canGenerate.value || !projectStore.currentProject) return;
  
  try {
    const request: ImageGenerationRequest = {
      project_id: projectStore.currentProject.id,
      prompt: prompt.value,
      style: selectedStyle.value,
      resolution: currentResolution.value,
      aspect_ratio: aspectRatio.value,
      quality: imageQuality.value,
      enhance_prompt: enhancePrompt.value,
      use_story_context: useStoryContext.value,
      generate_variations: generateVariations.value
    };
    
    const result = await advancedAIStore.generateImage(request);
    emit('image-generated', result);
    
    // Clear the prompt after successful generation
    prompt.value = '';
  } catch (error) {
    console.error('Image generation failed:', error);
    // Show error notification
  }
};

const clearForm = () => {
  prompt.value = '';
  selectedStyle.value = 'realistic';
  selectedResolution.value = 'medium';
  aspectRatio.value = '16:9';
  imageQuality.value = 'standard';
  enhancePrompt.value = true;
  useStoryContext.value = false;
  generateVariations.value = false;
};

const selectImage = (image: GeneratedImage) => {
  selectedImage.value = image;
};

const downloadImage = async (image: GeneratedImage) => {
  try {
    const response = await fetch(image.file_path);
    const blob = await response.blob();
    const url = window.URL.createObjectURL(blob);
    
    const a = document.createElement('a');
    a.href = url;
    a.download = `storyweaver-image-${image.id}.png`;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    
    window.URL.revokeObjectURL(url);
  } catch (error) {
    console.error('Failed to download image:', error);
  }
};

const copyImageUrl = async (image: GeneratedImage) => {
  try {
    await navigator.clipboard.writeText(image.file_path);
    // Show success notification
  } catch (error) {
    console.error('Failed to copy image URL:', error);
  }
};

const deleteImage = async (image: GeneratedImage) => {
  if (confirm('Are you sure you want to delete this image?')) {
    try {
      await advancedAIStore.deleteGeneratedImage(image.id);
    } catch (error) {
      console.error('Failed to delete image:', error);
    }
  }
};

const onImageDeleted = (imageId: string) => {
  selectedImage.value = null;
};

const handleImageError = (event: Event) => {
  const img = event.target as HTMLImageElement;
  img.src = '/placeholder-image.png'; // Fallback image
};

const truncateText = (text: string, maxLength: number): string => {
  if (text.length <= maxLength) return text;
  return text.substring(0, maxLength) + '...';
};

const formatDate = (dateString: string): string => {
  const date = new Date(dateString);
  return date.toLocaleDateString();
};

// Lifecycle
onMounted(async () => {
  if (projectStore.currentProject) {
    await advancedAIStore.loadProjectImages(projectStore.currentProject.id);
  }
});
</script>

<style scoped>
.image-generator {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  height: 100%;
}

.generation-form {
  background: var(--bg-secondary);
  border-radius: 8px;
  padding: 1.5rem;
  border: 1px solid var(--border-color);
}

.form-header {
  margin-bottom: 1.5rem;
}

.form-header h3 {
  margin: 0 0 0.5rem 0;
  color: var(--text-primary);
}

.form-description {
  margin: 0;
  color: var(--text-secondary);
  font-size: 0.875rem;
}

.prompt-section {
  margin-bottom: 1.5rem;
}

.prompt-section label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: 500;
  color: var(--text-primary);
}

.prompt-textarea {
  width: 100%;
  padding: 0.75rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--bg-primary);
  color: var(--text-primary);
  font-family: inherit;
  resize: vertical;
  min-height: 80px;
}

.quick-prompts {
  margin-top: 0.75rem;
}

.quick-prompts-label {
  display: block;
  margin-bottom: 0.5rem;
  font-size: 0.875rem;
  color: var(--text-secondary);
}

.quick-prompt-buttons {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
}

.quick-prompt-button {
  padding: 0.25rem 0.5rem;
  border: 1px solid var(--border-color);
  border-radius: 12px;
  background: var(--bg-primary);
  color: var(--text-secondary);
  font-size: 0.75rem;
  cursor: pointer;
  transition: all 0.2s ease;
}

.quick-prompt-button:hover {
  background: var(--accent-color);
  color: white;
  border-color: var(--accent-color);
}

.generation-options {
  margin-bottom: 1.5rem;
}

.options-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
  margin-bottom: 1rem;
}

.option-group {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.option-group label {
  font-weight: 500;
  color: var(--text-primary);
}

.option-select {
  padding: 0.5rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--bg-primary);
  color: var(--text-primary);
}

.advanced-options {
  border-top: 1px solid var(--border-color);
  padding-top: 1rem;
}

.option-toggles {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 0.75rem;
}

.toggle-option {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  cursor: pointer;
}

.toggle-label {
  display: flex;
  align-items: center;
  gap: 0.25rem;
  color: var(--text-primary);
}

.cost-estimate {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 4px;
  margin-bottom: 1rem;
}

.cost-info {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.cost-label {
  color: var(--text-secondary);
}

.cost-value {
  font-weight: 600;
  color: var(--accent-color);
}

.credits-remaining {
  font-size: 0.875rem;
  color: var(--text-secondary);
}

.action-buttons {
  display: flex;
  gap: 1rem;
}

.generate-button {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.75rem 1.5rem;
  border: none;
  border-radius: 4px;
  background: var(--accent-color);
  color: white;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.generate-button:hover:not(:disabled) {
  background: var(--accent-color-hover);
}

.generate-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.clear-button {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.75rem 1.5rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: transparent;
  color: var(--text-primary);
  cursor: pointer;
  transition: all 0.2s ease;
}

.clear-button:hover:not(:disabled) {
  background: var(--bg-hover);
}

.images-gallery {
  background: var(--bg-secondary);
  border-radius: 8px;
  padding: 1.5rem;
  border: 1px solid var(--border-color);
}

.gallery-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
}

.gallery-header h4 {
  margin: 0;
  color: var(--text-primary);
}

.gallery-controls {
  display: flex;
  gap: 0.5rem;
}

.view-button {
  padding: 0.5rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.2s ease;
}

.view-button:hover,
.view-button.active {
  background: var(--accent-color);
  color: white;
  border-color: var(--accent-color);
}

.images-container.grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
  gap: 1rem;
}

.images-container.list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.image-item {
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  overflow: hidden;
  cursor: pointer;
  transition: all 0.2s ease;
}

.image-item:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.image-wrapper {
  position: relative;
  aspect-ratio: 16/9;
  overflow: hidden;
}

.generated-image {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.image-overlay {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.7);
  display: flex;
  align-items: center;
  justify-content: center;
  opacity: 0;
  transition: opacity 0.2s ease;
}

.image-item:hover .image-overlay {
  opacity: 1;
}

.image-actions {
  display: flex;
  gap: 0.5rem;
}

.action-btn {
  padding: 0.5rem;
  border: none;
  border-radius: 4px;
  background: rgba(255, 255, 255, 0.9);
  color: var(--text-primary);
  cursor: pointer;
  transition: all 0.2s ease;
}

.action-btn:hover {
  background: white;
}

.action-btn.danger {
  background: rgba(220, 53, 69, 0.9);
  color: white;
}

.action-btn.danger:hover {
  background: #dc3545;
}

.image-info {
  padding: 1rem;
}

.image-prompt {
  font-weight: 500;
  color: var(--text-primary);
  margin-bottom: 0.5rem;
}

.image-metadata {
  display: flex;
  gap: 0.5rem;
  font-size: 0.75rem;
  color: var(--text-secondary);
}

.metadata-item {
  padding: 0.25rem 0.5rem;
  background: var(--bg-secondary);
  border-radius: 12px;
}

.generation-progress {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.progress-content {
  background: var(--bg-primary);
  padding: 2rem;
  border-radius: 8px;
  text-align: center;
  max-width: 400px;
}

.progress-spinner {
  width: 40px;
  height: 40px;
  border: 4px solid var(--border-color);
  border-top: 4px solid var(--accent-color);
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin: 0 auto 1rem;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.progress-text h4 {
  margin: 0 0 0.5rem 0;
  color: var(--text-primary);
}

.progress-text p {
  margin: 0;
  color: var(--text-secondary);
  font-size: 0.875rem;
}

/* List view specific styles */
.images-container.list .image-item {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.images-container.list .image-wrapper {
  width: 120px;
  height: 80px;
  flex-shrink: 0;
  aspect-ratio: 3/2;
}

.images-container.list .image-info {
  flex: 1;
  padding: 0.5rem 0;
}

/* Responsive design */
@media (max-width: 768px) {
  .options-grid {
    grid-template-columns: 1fr;
  }
  
  .option-toggles {
    grid-template-columns: 1fr;
  }
  
  .action-buttons {
    flex-direction: column;
  }
  
  .cost-estimate {
    flex-direction: column;
    gap: 0.5rem;
    align-items: stretch;
  }
  
  .gallery-header {
    flex-direction: column;
    gap: 1rem;
    align-items: stretch;
  }
  
  .images-container.grid {
    grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  }
  
  .quick-prompt-buttons {
    justify-content: center;
  }
}
</style>