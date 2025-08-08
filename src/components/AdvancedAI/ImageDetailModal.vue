<template>
  <div class="image-detail-modal" @click="handleBackdropClick">
    <div class="modal-content" @click.stop>
      <!-- Modal Header -->
      <div class="modal-header">
        <h3>Image Details</h3>
        <button @click="$emit('close')" class="close-button">
          <i class="fas fa-times"></i>
        </button>
      </div>

      <!-- Modal Body -->
      <div class="modal-body">
        <!-- Image Display -->
        <div class="image-display">
          <img 
            :src="image.file_path" 
            :alt="image.prompt"
            class="detail-image"
            @error="handleImageError"
          >
          
          <!-- Image Actions Overlay -->
          <div class="image-actions-overlay">
            <button @click="$emit('download', image)" class="action-button" title="Download">
              <i class="fas fa-download"></i>
            </button>
            <button @click="copyImageUrl" class="action-button" title="Copy URL">
              <i class="fas fa-link"></i>
            </button>
            <button @click="copyToClipboard" class="action-button" title="Copy Image">
              <i class="fas fa-copy"></i>
            </button>
            <button @click="setAsProjectCover" class="action-button" title="Set as Project Cover">
              <i class="fas fa-star"></i>
            </button>
          </div>
        </div>

        <!-- Image Information -->
        <div class="image-info">
          <!-- Prompt -->
          <div class="info-section">
            <h4>Prompt</h4>
            <div class="prompt-text">{{ image.prompt }}</div>
            <button @click="copyPrompt" class="copy-prompt-button">
              <i class="fas fa-copy"></i>
              Copy Prompt
            </button>
          </div>

          <!-- Enhanced Prompt (if available) -->
          <div v-if="image.enhanced_prompt" class="info-section">
            <h4>Enhanced Prompt</h4>
            <div class="enhanced-prompt-text">{{ image.enhanced_prompt }}</div>
            <button @click="copyEnhancedPrompt" class="copy-prompt-button">
              <i class="fas fa-copy"></i>
              Copy Enhanced
            </button>
          </div>

          <!-- Technical Details -->
          <div class="info-section">
            <h4>Technical Details</h4>
            <div class="technical-details">
              <div class="detail-grid">
                <div class="detail-item">
                  <span class="detail-label">Resolution:</span>
                  <span class="detail-value">{{ image.resolution.width }}x{{ image.resolution.height }}</span>
                </div>
                <div class="detail-item">
                  <span class="detail-label">Style:</span>
                  <span class="detail-value">{{ formatStyle(image.style) }}</span>
                </div>
                <div class="detail-item">
                  <span class="detail-label">Quality:</span>
                  <span class="detail-value">{{ formatQuality(image.quality) }}</span>
                </div>
                <div class="detail-item">
                  <span class="detail-label">Aspect Ratio:</span>
                  <span class="detail-value">{{ image.aspect_ratio }}</span>
                </div>
                <div class="detail-item">
                  <span class="detail-label">Credits Used:</span>
                  <span class="detail-value">{{ image.credits_used }}</span>
                </div>
                <div class="detail-item">
                  <span class="detail-label">Created:</span>
                  <span class="detail-value">{{ formatDateTime(image.created_at) }}</span>
                </div>
              </div>
            </div>
          </div>

          <!-- Generation Settings -->
          <div class="info-section">
            <h4>Generation Settings</h4>
            <div class="settings-tags">
              <span v-if="image.enhanced_prompt" class="setting-tag enhanced">
                <i class="fas fa-magic"></i>
                Enhanced
              </span>
              <span v-if="image.use_story_context" class="setting-tag context">
                <i class="fas fa-book"></i>
                Story Context
              </span>
              <span v-if="image.variations && image.variations.length > 0" class="setting-tag variations">
                <i class="fas fa-clone"></i>
                {{ image.variations.length }} Variations
              </span>
            </div>
          </div>

          <!-- Variations (if available) -->
          <div v-if="image.variations && image.variations.length > 0" class="info-section">
            <h4>Variations</h4>
            <div class="variations-grid">
              <div 
                v-for="variation in image.variations" 
                :key="variation.id"
                class="variation-item"
                @click="selectVariation(variation)"
              >
                <img 
                  :src="variation.file_path" 
                  :alt="variation.prompt"
                  class="variation-image"
                >
                <div class="variation-overlay">
                  <button @click.stop="downloadVariation(variation)" class="variation-action">
                    <i class="fas fa-download"></i>
                  </button>
                </div>
              </div>
            </div>
          </div>

          <!-- Usage Suggestions -->
          <div class="info-section">
            <h4>Usage Suggestions</h4>
            <div class="usage-suggestions">
              <button @click="insertIntoDocument" class="suggestion-button">
                <i class="fas fa-file-text"></i>
                Insert into Document
              </button>
              <button @click="addToStoryBible" class="suggestion-button">
                <i class="fas fa-book-open"></i>
                Add to Story Bible
              </button>
              <button @click="createCharacterReference" class="suggestion-button">
                <i class="fas fa-user"></i>
                Character Reference
              </button>
              <button @click="createLocationReference" class="suggestion-button">
                <i class="fas fa-map-marker-alt"></i>
                Location Reference
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Modal Footer -->
      <div class="modal-footer">
        <div class="footer-actions">
          <button @click="regenerateImage" class="regenerate-button">
            <i class="fas fa-redo"></i>
            Regenerate
          </button>
          <button @click="editPrompt" class="edit-button">
            <i class="fas fa-edit"></i>
            Edit & Regenerate
          </button>
          <button @click="confirmDelete" class="delete-button danger">
            <i class="fas fa-trash"></i>
            Delete
          </button>
        </div>
      </div>
    </div>

    <!-- Delete Confirmation Modal -->
    <div v-if="showDeleteConfirm" class="delete-confirm-modal" @click="showDeleteConfirm = false">
      <div class="delete-confirm-content" @click.stop>
        <h4>Delete Image?</h4>
        <p>This action cannot be undone. The image will be permanently deleted.</p>
        <div class="delete-confirm-actions">
          <button @click="showDeleteConfirm = false" class="cancel-button">
            Cancel
          </button>
          <button @click="deleteImage" class="confirm-delete-button">
            Delete
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useAdvancedAIStore } from '../../stores/advancedAIStore';
import { useProjectStore } from '../../stores/projectStore';
import type { GeneratedImage } from '../../types/advancedAI';

// Props
const props = defineProps<{
  image: GeneratedImage;
}>();

// Emits
const emit = defineEmits<{
  'close': [];
  'delete': [imageId: string];
  'download': [image: GeneratedImage];
  'regenerate': [prompt: string];
  'edit-prompt': [prompt: string];
}>();

// Stores
const advancedAIStore = useAdvancedAIStore();
const projectStore = useProjectStore();

// Reactive state
const showDeleteConfirm = ref(false);

// Methods
const handleBackdropClick = () => {
  emit('close');
};

const handleImageError = (event: Event) => {
  const img = event.target as HTMLImageElement;
  img.src = '/placeholder-image.png';
};

const copyImageUrl = async () => {
  try {
    await navigator.clipboard.writeText(props.image.file_path);
    // Show success notification
    console.log('Image URL copied to clipboard');
  } catch (error) {
    console.error('Failed to copy image URL:', error);
  }
};

const copyToClipboard = async () => {
  try {
    const response = await fetch(props.image.file_path);
    const blob = await response.blob();
    
    const item = new ClipboardItem({ [blob.type]: blob });
    await navigator.clipboard.write([item]);
    
    console.log('Image copied to clipboard');
  } catch (error) {
    console.error('Failed to copy image to clipboard:', error);
  }
};

const copyPrompt = async () => {
  try {
    await navigator.clipboard.writeText(props.image.prompt);
    console.log('Prompt copied to clipboard');
  } catch (error) {
    console.error('Failed to copy prompt:', error);
  }
};

const copyEnhancedPrompt = async () => {
  try {
    if (props.image.enhanced_prompt) {
      await navigator.clipboard.writeText(props.image.enhanced_prompt);
      console.log('Enhanced prompt copied to clipboard');
    }
  } catch (error) {
    console.error('Failed to copy enhanced prompt:', error);
  }
};

const setAsProjectCover = async () => {
  try {
    if (projectStore.currentProject) {
      // Implementation would depend on project store structure
      console.log('Setting as project cover:', props.image.id);
    }
  } catch (error) {
    console.error('Failed to set as project cover:', error);
  }
};

const selectVariation = (variation: GeneratedImage) => {
  // Could emit an event to switch to viewing the variation
  console.log('Selected variation:', variation.id);
};

const downloadVariation = (variation: GeneratedImage) => {
  emit('download', variation);
};

const insertIntoDocument = () => {
  // Implementation would depend on document editor integration
  console.log('Inserting image into document:', props.image.id);
  emit('close');
};

const addToStoryBible = () => {
  // Implementation would depend on Story Bible integration
  console.log('Adding image to Story Bible:', props.image.id);
  emit('close');
};

const createCharacterReference = () => {
  // Implementation would create a character reference with this image
  console.log('Creating character reference with image:', props.image.id);
  emit('close');
};

const createLocationReference = () => {
  // Implementation would create a location reference with this image
  console.log('Creating location reference with image:', props.image.id);
  emit('close');
};

const regenerateImage = () => {
  emit('regenerate', props.image.prompt);
  emit('close');
};

const editPrompt = () => {
  emit('edit-prompt', props.image.prompt);
  emit('close');
};

const confirmDelete = () => {
  showDeleteConfirm.value = true;
};

const deleteImage = async () => {
  try {
    await advancedAIStore.deleteGeneratedImage(props.image.id);
    emit('delete', props.image.id);
    emit('close');
  } catch (error) {
    console.error('Failed to delete image:', error);
  }
  showDeleteConfirm.value = false;
};

// Utility functions
const formatStyle = (style: string): string => {
  return style.split('-').map(word => 
    word.charAt(0).toUpperCase() + word.slice(1)
  ).join(' ');
};

const formatQuality = (quality: string): string => {
  const qualityMap: Record<string, string> = {
    draft: 'Draft',
    standard: 'Standard',
    high: 'High Quality',
    ultra: 'Ultra High'
  };
  return qualityMap[quality] || quality;
};

const formatDateTime = (dateString: string): string => {
  const date = new Date(dateString);
  return date.toLocaleString();
};
</script>

<style scoped>
.image-detail-modal {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.8);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 2000;
  padding: 1rem;
}

.modal-content {
  background: var(--bg-primary);
  border-radius: 12px;
  max-width: 90vw;
  max-height: 90vh;
  width: 1000px;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.3);
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1.5rem;
  border-bottom: 1px solid var(--border-color);
}

.modal-header h3 {
  margin: 0;
  color: var(--text-primary);
}

.close-button {
  padding: 0.5rem;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  border-radius: 4px;
  transition: all 0.2s ease;
}

.close-button:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.modal-body {
  display: grid;
  grid-template-columns: 1fr 400px;
  gap: 1.5rem;
  padding: 1.5rem;
  overflow-y: auto;
  flex: 1;
}

.image-display {
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--bg-secondary);
  border-radius: 8px;
  overflow: hidden;
  min-height: 400px;
}

.detail-image {
  max-width: 100%;
  max-height: 100%;
  object-fit: contain;
  border-radius: 8px;
}

.image-actions-overlay {
  position: absolute;
  top: 1rem;
  right: 1rem;
  display: flex;
  gap: 0.5rem;
  opacity: 0;
  transition: opacity 0.2s ease;
}

.image-display:hover .image-actions-overlay {
  opacity: 1;
}

.action-button {
  padding: 0.75rem;
  border: none;
  border-radius: 6px;
  background: rgba(0, 0, 0, 0.7);
  color: white;
  cursor: pointer;
  transition: all 0.2s ease;
  backdrop-filter: blur(4px);
}

.action-button:hover {
  background: rgba(0, 0, 0, 0.9);
  transform: scale(1.05);
}

.image-info {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  overflow-y: auto;
}

.info-section {
  background: var(--bg-secondary);
  border-radius: 8px;
  padding: 1rem;
}

.info-section h4 {
  margin: 0 0 1rem 0;
  color: var(--text-primary);
  font-size: 1rem;
  font-weight: 600;
}

.prompt-text,
.enhanced-prompt-text {
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 4px;
  padding: 0.75rem;
  color: var(--text-primary);
  line-height: 1.5;
  margin-bottom: 0.75rem;
  font-family: inherit;
}

.copy-prompt-button {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 0.75rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  font-size: 0.875rem;
  transition: all 0.2s ease;
}

.copy-prompt-button:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.detail-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 0.75rem;
}

.detail-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.5rem;
  background: var(--bg-primary);
  border-radius: 4px;
}

.detail-label {
  color: var(--text-secondary);
  font-size: 0.875rem;
}

.detail-value {
  color: var(--text-primary);
  font-weight: 500;
  font-size: 0.875rem;
}

.settings-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
}

.setting-tag {
  display: flex;
  align-items: center;
  gap: 0.25rem;
  padding: 0.25rem 0.5rem;
  border-radius: 12px;
  font-size: 0.75rem;
  font-weight: 500;
}

.setting-tag.enhanced {
  background: rgba(138, 43, 226, 0.1);
  color: #8a2be2;
}

.setting-tag.context {
  background: rgba(34, 139, 34, 0.1);
  color: #228b22;
}

.setting-tag.variations {
  background: rgba(255, 140, 0, 0.1);
  color: #ff8c00;
}

.variations-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(80px, 1fr));
  gap: 0.5rem;
}

.variation-item {
  position: relative;
  aspect-ratio: 1;
  border-radius: 4px;
  overflow: hidden;
  cursor: pointer;
  transition: transform 0.2s ease;
}

.variation-item:hover {
  transform: scale(1.05);
}

.variation-image {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.variation-overlay {
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

.variation-item:hover .variation-overlay {
  opacity: 1;
}

.variation-action {
  padding: 0.25rem;
  border: none;
  border-radius: 4px;
  background: rgba(255, 255, 255, 0.9);
  color: var(--text-primary);
  cursor: pointer;
}

.usage-suggestions {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 0.5rem;
}

.suggestion-button {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.75rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: transparent;
  color: var(--text-primary);
  cursor: pointer;
  font-size: 0.875rem;
  transition: all 0.2s ease;
}

.suggestion-button:hover {
  background: var(--accent-color);
  color: white;
  border-color: var(--accent-color);
}

.modal-footer {
  padding: 1.5rem;
  border-top: 1px solid var(--border-color);
  background: var(--bg-secondary);
}

.footer-actions {
  display: flex;
  gap: 1rem;
  justify-content: flex-end;
}

.regenerate-button,
.edit-button {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.75rem 1rem;
  border: 1px solid var(--accent-color);
  border-radius: 4px;
  background: transparent;
  color: var(--accent-color);
  cursor: pointer;
  transition: all 0.2s ease;
}

.regenerate-button:hover,
.edit-button:hover {
  background: var(--accent-color);
  color: white;
}

.delete-button {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.75rem 1rem;
  border: 1px solid #dc3545;
  border-radius: 4px;
  background: transparent;
  color: #dc3545;
  cursor: pointer;
  transition: all 0.2s ease;
}

.delete-button:hover {
  background: #dc3545;
  color: white;
}

.delete-confirm-modal {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.9);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 3000;
}

.delete-confirm-content {
  background: var(--bg-primary);
  border-radius: 8px;
  padding: 2rem;
  max-width: 400px;
  text-align: center;
}

.delete-confirm-content h4 {
  margin: 0 0 1rem 0;
  color: var(--text-primary);
}

.delete-confirm-content p {
  margin: 0 0 1.5rem 0;
  color: var(--text-secondary);
}

.delete-confirm-actions {
  display: flex;
  gap: 1rem;
  justify-content: center;
}

.cancel-button {
  padding: 0.75rem 1.5rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: transparent;
  color: var(--text-primary);
  cursor: pointer;
}

.confirm-delete-button {
  padding: 0.75rem 1.5rem;
  border: none;
  border-radius: 4px;
  background: #dc3545;
  color: white;
  cursor: pointer;
}

/* Responsive design */
@media (max-width: 768px) {
  .modal-content {
    width: 95vw;
    height: 95vh;
  }
  
  .modal-body {
    grid-template-columns: 1fr;
    grid-template-rows: auto 1fr;
  }
  
  .image-display {
    min-height: 250px;
  }
  
  .detail-grid {
    grid-template-columns: 1fr;
  }
  
  .usage-suggestions {
    grid-template-columns: 1fr;
  }
  
  .footer-actions {
    flex-direction: column;
  }
  
  .variations-grid {
    grid-template-columns: repeat(auto-fill, minmax(60px, 1fr));
  }
}
</style>