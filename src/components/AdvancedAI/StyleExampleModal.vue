<template>
  <div class="modal-overlay" @click="handleOverlayClick">
    <div class="modal-content" @click.stop>
      <!-- Modal Header -->
      <div class="modal-header">
        <h3>{{ isEditing ? 'Edit Style Example' : 'Add Style Example' }}</h3>
        <button @click="$emit('close')" class="close-button">
          <i class="fas fa-times"></i>
        </button>
      </div>

      <!-- Modal Body -->
      <div class="modal-body">
        <form @submit.prevent="handleSubmit" class="example-form">
          <!-- Basic Information -->
          <div class="form-section">
            <h4>Basic Information</h4>
            
            <div class="form-group">
              <label for="example-name">Name *</label>
              <input 
                id="example-name"
                v-model="formData.name"
                type="text"
                placeholder="e.g., Dialogue - Character Voice, Action Scene Pacing"
                required
                class="form-input"
              >
              <div class="field-hint">
                Give your style example a descriptive name to easily identify it later.
              </div>
            </div>
            
            <div class="form-group">
              <label for="example-category">Category *</label>
              <select 
                id="example-category"
                v-model="formData.category"
                required
                class="form-select"
              >
                <option value="">Select a category</option>
                <option value="dialogue">Dialogue</option>
                <option value="description">Description</option>
                <option value="action">Action</option>
                <option value="introspection">Introspection</option>
                <option value="narrative">Narrative</option>
                <option value="other">Other</option>
              </select>
              <div class="field-hint">
                Choose the type of writing this example represents.
              </div>
            </div>
          </div>

          <!-- Content -->
          <div class="form-section">
            <h4>Content</h4>
            
            <div class="form-group">
              <label for="example-content">Text Content *</label>
              <textarea 
                id="example-content"
                v-model="formData.content"
                placeholder="Paste or type the example text here..."
                required
                class="form-textarea content-textarea"
                rows="8"
              ></textarea>
              <div class="content-stats">
                <span class="stat-item">
                  <i class="fas fa-font"></i>
                  {{ wordCount }} words
                </span>
                <span class="stat-item">
                  <i class="fas fa-align-left"></i>
                  {{ characterCount }} characters
                </span>
                <span class="stat-item">
                  <i class="fas fa-paragraph"></i>
                  {{ sentenceCount }} sentences
                </span>
              </div>
              <div class="field-hint">
                Include a representative sample of your writing style (recommended: 100-500 words).
              </div>
            </div>
          </div>

          <!-- Style Characteristics -->
          <div class="form-section">
            <h4>Style Characteristics</h4>
            <div class="field-hint section-hint">
              Help the AI understand what makes this example special by describing its key characteristics.
            </div>
            
            <div class="characteristics-grid">
              <div class="characteristic-group">
                <label for="tone">Tone</label>
                <select id="tone" v-model="formData.characteristics.tone" class="form-select">
                  <option value="">Select tone</option>
                  <option value="formal">Formal</option>
                  <option value="informal">Informal</option>
                  <option value="conversational">Conversational</option>
                  <option value="academic">Academic</option>
                  <option value="playful">Playful</option>
                  <option value="serious">Serious</option>
                  <option value="humorous">Humorous</option>
                  <option value="dramatic">Dramatic</option>
                  <option value="mysterious">Mysterious</option>
                  <option value="romantic">Romantic</option>
                </select>
              </div>
              
              <div class="characteristic-group">
                <label for="complexity">Complexity</label>
                <select id="complexity" v-model="formData.characteristics.complexity" class="form-select">
                  <option value="">Select complexity</option>
                  <option value="simple">Simple</option>
                  <option value="moderate">Moderate</option>
                  <option value="complex">Complex</option>
                  <option value="very-complex">Very Complex</option>
                </select>
              </div>
              
              <div class="characteristic-group">
                <label for="pacing">Pacing</label>
                <select id="pacing" v-model="formData.characteristics.pacing" class="form-select">
                  <option value="">Select pacing</option>
                  <option value="slow">Slow</option>
                  <option value="moderate">Moderate</option>
                  <option value="fast">Fast</option>
                  <option value="varied">Varied</option>
                </select>
              </div>
              
              <div class="characteristic-group">
                <label for="perspective">Perspective</label>
                <select id="perspective" v-model="formData.characteristics.perspective" class="form-select">
                  <option value="">Select perspective</option>
                  <option value="first-person">First Person</option>
                  <option value="second-person">Second Person</option>
                  <option value="third-person-limited">Third Person Limited</option>
                  <option value="third-person-omniscient">Third Person Omniscient</option>
                  <option value="multiple">Multiple</option>
                </select>
              </div>
            </div>
          </div>

          <!-- Tags -->
          <div class="form-section">
            <h4>Tags</h4>
            
            <div class="form-group">
              <label for="tags-input">Tags</label>
              <div class="tags-input-container">
                <div class="tags-display">
                  <span 
                    v-for="tag in formData.tags" 
                    :key="tag"
                    class="tag-item"
                  >
                    {{ tag }}
                    <button 
                      type="button"
                      @click="removeTag(tag)"
                      class="tag-remove"
                    >
                      <i class="fas fa-times"></i>
                    </button>
                  </span>
                </div>
                <input 
                  id="tags-input"
                  v-model="newTag"
                  @keydown.enter.prevent="addTag"
                  @keydown.comma.prevent="addTag"
                  type="text"
                  placeholder="Type a tag and press Enter"
                  class="form-input tags-input"
                >
              </div>
              <div class="field-hint">
                Add tags to help categorize and find this example later. Press Enter or comma to add a tag.
              </div>
            </div>
            
            <!-- Suggested Tags -->
            <div v-if="suggestedTags.length > 0" class="suggested-tags">
              <div class="suggested-tags-header">Suggested tags:</div>
              <div class="suggested-tags-list">
                <button 
                  v-for="tag in suggestedTags" 
                  :key="tag"
                  type="button"
                  @click="addSuggestedTag(tag)"
                  class="suggested-tag"
                >
                  {{ tag }}
                </button>
              </div>
            </div>
          </div>

          <!-- Notes -->
          <div class="form-section">
            <h4>Notes</h4>
            
            <div class="form-group">
              <label for="example-notes">Notes</label>
              <textarea 
                id="example-notes"
                v-model="formData.notes"
                placeholder="Add any additional notes about this style example..."
                class="form-textarea"
                rows="4"
              ></textarea>
              <div class="field-hint">
                Optional notes about when to use this style, what makes it effective, or any other relevant information.
              </div>
            </div>
          </div>

          <!-- AI Analysis -->
          <div class="form-section">
            <h4>AI Analysis</h4>
            <div class="field-hint section-hint">
              Let AI analyze this example to automatically detect style characteristics.
            </div>
            
            <div class="analysis-controls">
              <button 
                type="button"
                @click="analyzeExample"
                :disabled="!canAnalyze || isAnalyzing"
                class="analyze-button"
              >
                <i :class="isAnalyzing ? 'fas fa-spinner fa-spin' : 'fas fa-magic'"></i>
                {{ isAnalyzing ? 'Analyzing...' : 'Analyze Style' }}
              </button>
              
              <div class="analysis-options">
                <label class="option-label">
                  <input 
                    type="checkbox" 
                    v-model="analysisOptions.detectTone"
                  >
                  Detect tone
                </label>
                <label class="option-label">
                  <input 
                    type="checkbox" 
                    v-model="analysisOptions.suggestTags"
                  >
                  Suggest tags
                </label>
                <label class="option-label">
                  <input 
                    type="checkbox" 
                    v-model="analysisOptions.analyzeComplexity"
                  >
                  Analyze complexity
                </label>
              </div>
            </div>
            
            <!-- Analysis Results -->
            <div v-if="analysisResults" class="analysis-results">
              <div class="results-header">
                <i class="fas fa-chart-line"></i>
                Analysis Results
              </div>
              
              <div class="results-content">
                <div v-if="analysisResults.detectedTone" class="result-item">
                  <strong>Detected Tone:</strong> {{ analysisResults.detectedTone }}
                  <button 
                    type="button"
                    @click="applyDetectedTone"
                    class="apply-button"
                  >
                    Apply
                  </button>
                </div>
                
                <div v-if="analysisResults.suggestedTags?.length > 0" class="result-item">
                  <strong>Suggested Tags:</strong>
                  <div class="suggested-analysis-tags">
                    <button 
                      v-for="tag in analysisResults.suggestedTags" 
                      :key="tag"
                      type="button"
                      @click="addSuggestedTag(tag)"
                      class="suggested-tag"
                    >
                      {{ tag }}
                    </button>
                  </div>
                </div>
                
                <div v-if="analysisResults.complexity" class="result-item">
                  <strong>Complexity Level:</strong> {{ analysisResults.complexity }}
                  <button 
                    type="button"
                    @click="applyDetectedComplexity"
                    class="apply-button"
                  >
                    Apply
                  </button>
                </div>
                
                <div v-if="analysisResults.insights" class="result-item">
                  <strong>Style Insights:</strong>
                  <p class="insights-text">{{ analysisResults.insights }}</p>
                </div>
              </div>
            </div>
          </div>
        </form>
      </div>

      <!-- Modal Footer -->
      <div class="modal-footer">
        <div class="footer-info">
          <span v-if="hasUnsavedChanges" class="unsaved-indicator">
            <i class="fas fa-circle"></i>
            Unsaved changes
          </span>
        </div>
        
        <div class="footer-actions">
          <button 
            type="button"
            @click="$emit('close')"
            class="cancel-button"
          >
            Cancel
          </button>
          
          <button 
            type="button"
            @click="handleSubmit"
            :disabled="!isFormValid"
            class="save-button"
          >
            <i class="fas fa-save"></i>
            {{ isEditing ? 'Update Example' : 'Save Example' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue';
import { useAdvancedAIStore } from '../../stores/advancedAIStore';
import type { StyleExample } from '../../types/advancedAI';

// Props
interface Props {
  example?: StyleExample | null;
}

const props = defineProps<Props>();

// Emits
interface Emits {
  close: [];
  save: [example: StyleExample];
}

const emit = defineEmits<Emits>();

// Store
const advancedAIStore = useAdvancedAIStore();

// Reactive state
const formData = ref({
  id: '',
  name: '',
  category: '',
  content: '',
  characteristics: {
    tone: '',
    complexity: '',
    pacing: '',
    perspective: ''
  },
  tags: [] as string[],
  notes: '',
  created_at: '',
  updated_at: ''
});

const newTag = ref('');
const isAnalyzing = ref(false);
const analysisResults = ref<any>(null);
const originalData = ref<string>('');

const analysisOptions = ref({
  detectTone: true,
  suggestTags: true,
  analyzeComplexity: true
});

// Computed properties
const isEditing = computed(() => !!props.example);

const wordCount = computed(() => {
  return formData.value.content.trim().split(/\s+/).filter(word => word.length > 0).length;
});

const characterCount = computed(() => {
  return formData.value.content.length;
});

const sentenceCount = computed(() => {
  return formData.value.content.split(/[.!?]+/).filter(sentence => sentence.trim().length > 0).length;
});

const canAnalyze = computed(() => {
  return formData.value.content.trim().length > 50; // Minimum content for analysis
});

const isFormValid = computed(() => {
  return formData.value.name.trim().length > 0 &&
         formData.value.category.trim().length > 0 &&
         formData.value.content.trim().length > 0;
});

const hasUnsavedChanges = computed(() => {
  return JSON.stringify(formData.value) !== originalData.value;
});

const suggestedTags = computed(() => {
  const suggestions: string[] = [];
  
  // Category-based suggestions
  switch (formData.value.category) {
    case 'dialogue':
      suggestions.push('conversation', 'character-voice', 'speech-patterns');
      break;
    case 'description':
      suggestions.push('imagery', 'sensory-details', 'setting');
      break;
    case 'action':
      suggestions.push('pacing', 'movement', 'tension');
      break;
    case 'introspection':
      suggestions.push('thoughts', 'emotions', 'internal-conflict');
      break;
    case 'narrative':
      suggestions.push('storytelling', 'voice', 'flow');
      break;
  }
  
  // Tone-based suggestions
  if (formData.value.characteristics.tone) {
    suggestions.push(formData.value.characteristics.tone);
  }
  
  // Filter out already added tags
  return suggestions.filter(tag => !formData.value.tags.includes(tag));
});

// Methods
const initializeForm = () => {
  if (props.example) {
    formData.value = {
      id: props.example.id,
      name: props.example.name,
      category: props.example.category,
      content: props.example.content,
      characteristics: { ...props.example.characteristics },
      tags: [...props.example.tags],
      notes: props.example.notes || '',
      created_at: props.example.created_at,
      updated_at: props.example.updated_at
    };
  } else {
    const now = new Date().toISOString();
    formData.value = {
      id: `example_${Date.now()}`,
      name: '',
      category: '',
      content: '',
      characteristics: {
        tone: '',
        complexity: '',
        pacing: '',
        perspective: ''
      },
      tags: [],
      notes: '',
      created_at: now,
      updated_at: now
    };
  }
  
  originalData.value = JSON.stringify(formData.value);
};

const addTag = () => {
  const tag = newTag.value.trim().toLowerCase();
  if (tag && !formData.value.tags.includes(tag)) {
    formData.value.tags.push(tag);
    newTag.value = '';
  }
};

const addSuggestedTag = (tag: string) => {
  if (!formData.value.tags.includes(tag)) {
    formData.value.tags.push(tag);
  }
};

const removeTag = (tagToRemove: string) => {
  formData.value.tags = formData.value.tags.filter(tag => tag !== tagToRemove);
};

const analyzeExample = async () => {
  if (!canAnalyze.value || isAnalyzing.value) return;
  
  isAnalyzing.value = true;
  analysisResults.value = null;
  
  try {
    const response = await advancedAIStore.analyzeStyleExample({
      content: formData.value.content,
      options: analysisOptions.value
    });
    
    analysisResults.value = response;
  } catch (error) {
    console.error('Failed to analyze style example:', error);
  } finally {
    isAnalyzing.value = false;
  }
};

const applyDetectedTone = () => {
  if (analysisResults.value?.detectedTone) {
    formData.value.characteristics.tone = analysisResults.value.detectedTone;
  }
};

const applyDetectedComplexity = () => {
  if (analysisResults.value?.complexity) {
    formData.value.characteristics.complexity = analysisResults.value.complexity;
  }
};

const handleSubmit = () => {
  if (!isFormValid.value) return;
  
  const now = new Date().toISOString();
  const exampleData: StyleExample = {
    ...formData.value,
    updated_at: now
  };
  
  emit('save', exampleData);
};

const handleOverlayClick = () => {
  if (hasUnsavedChanges.value) {
    if (confirm('You have unsaved changes. Are you sure you want to close?')) {
      emit('close');
    }
  } else {
    emit('close');
  }
};

// Watchers
watch(() => props.example, () => {
  initializeForm();
}, { immediate: true });

// Lifecycle
onMounted(() => {
  initializeForm();
});
</script>

<style scoped>
.modal-overlay {
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
  padding: 1rem;
}

.modal-content {
  background: var(--bg-primary);
  border-radius: 8px;
  width: 100%;
  max-width: 800px;
  max-height: 90vh;
  display: flex;
  flex-direction: column;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
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
  flex: 1;
  overflow-y: auto;
  padding: 1.5rem;
}

.example-form {
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

.form-section {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.form-section h4 {
  margin: 0;
  color: var(--text-primary);
  font-size: 1.125rem;
  border-bottom: 1px solid var(--border-color);
  padding-bottom: 0.5rem;
}

.section-hint {
  margin-top: -0.5rem;
  font-style: italic;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.form-group label {
  font-weight: 500;
  color: var(--text-primary);
}

.form-input,
.form-select,
.form-textarea {
  padding: 0.75rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--bg-primary);
  color: var(--text-primary);
  font-family: inherit;
  transition: border-color 0.2s ease;
}

.form-input:focus,
.form-select:focus,
.form-textarea:focus {
  outline: none;
  border-color: var(--accent-color);
}

.content-textarea {
  font-family: 'Courier New', monospace;
  line-height: 1.5;
  resize: vertical;
}

.content-stats {
  display: flex;
  gap: 1rem;
  font-size: 0.875rem;
  color: var(--text-secondary);
}

.stat-item {
  display: flex;
  align-items: center;
  gap: 0.25rem;
}

.field-hint {
  font-size: 0.875rem;
  color: var(--text-secondary);
  line-height: 1.4;
}

.characteristics-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
}

.characteristic-group {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.characteristic-group label {
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--text-primary);
}

.tags-input-container {
  border: 1px solid var(--border-color);
  border-radius: 4px;
  padding: 0.5rem;
  background: var(--bg-primary);
  min-height: 2.5rem;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.tags-display {
  display: flex;
  flex-wrap: wrap;
  gap: 0.25rem;
}

.tag-item {
  display: flex;
  align-items: center;
  gap: 0.25rem;
  padding: 0.25rem 0.5rem;
  background: var(--accent-color);
  color: white;
  border-radius: 12px;
  font-size: 0.875rem;
}

.tag-remove {
  padding: 0;
  border: none;
  background: transparent;
  color: white;
  cursor: pointer;
  border-radius: 50%;
  width: 16px;
  height: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 0.75rem;
}

.tag-remove:hover {
  background: rgba(255, 255, 255, 0.2);
}

.tags-input {
  border: none;
  padding: 0.25rem 0;
  background: transparent;
  flex: 1;
}

.tags-input:focus {
  outline: none;
  border: none;
}

.suggested-tags {
  margin-top: 0.5rem;
}

.suggested-tags-header {
  font-size: 0.875rem;
  color: var(--text-secondary);
  margin-bottom: 0.25rem;
}

.suggested-tags-list {
  display: flex;
  flex-wrap: wrap;
  gap: 0.25rem;
}

.suggested-tag {
  padding: 0.25rem 0.5rem;
  border: 1px solid var(--border-color);
  border-radius: 12px;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  font-size: 0.75rem;
  transition: all 0.2s ease;
}

.suggested-tag:hover {
  background: var(--accent-color);
  color: white;
  border-color: var(--accent-color);
}

.analysis-controls {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.analyze-button {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.75rem 1rem;
  border: 1px solid var(--accent-color);
  border-radius: 4px;
  background: var(--accent-color);
  color: white;
  cursor: pointer;
  transition: all 0.2s ease;
  align-self: flex-start;
}

.analyze-button:hover:not(:disabled) {
  background: var(--accent-color-hover);
}

.analyze-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.analysis-options {
  display: flex;
  gap: 1rem;
  flex-wrap: wrap;
}

.option-label {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 0.875rem;
  color: var(--text-primary);
  cursor: pointer;
}

.analysis-results {
  margin-top: 1rem;
  padding: 1rem;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
}

.results-header {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-weight: 500;
  color: var(--text-primary);
  margin-bottom: 1rem;
}

.results-content {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.result-item {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.result-item strong {
  color: var(--text-primary);
}

.apply-button {
  padding: 0.25rem 0.5rem;
  border: 1px solid var(--accent-color);
  border-radius: 4px;
  background: transparent;
  color: var(--accent-color);
  cursor: pointer;
  font-size: 0.75rem;
  transition: all 0.2s ease;
  align-self: flex-start;
}

.apply-button:hover {
  background: var(--accent-color);
  color: white;
}

.suggested-analysis-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 0.25rem;
  margin-top: 0.25rem;
}

.insights-text {
  margin: 0.25rem 0 0 0;
  color: var(--text-primary);
  line-height: 1.5;
  font-size: 0.875rem;
}

.modal-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1.5rem;
  border-top: 1px solid var(--border-color);
  background: var(--bg-secondary);
}

.footer-info {
  display: flex;
  align-items: center;
}

.unsaved-indicator {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 0.875rem;
  color: #ffc107;
}

.unsaved-indicator i {
  font-size: 0.5rem;
}

.footer-actions {
  display: flex;
  gap: 0.5rem;
}

.cancel-button,
.save-button {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.75rem 1rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: transparent;
  color: var(--text-primary);
  cursor: pointer;
  transition: all 0.2s ease;
}

.cancel-button:hover {
  background: var(--bg-hover);
}

.save-button {
  background: var(--accent-color);
  color: white;
  border-color: var(--accent-color);
}

.save-button:hover:not(:disabled) {
  background: var(--accent-color-hover);
}

.save-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* Responsive design */
@media (max-width: 768px) {
  .modal-overlay {
    padding: 0.5rem;
  }
  
  .modal-content {
    max-height: 95vh;
  }
  
  .modal-header,
  .modal-body,
  .modal-footer {
    padding: 1rem;
  }
  
  .characteristics-grid {
    grid-template-columns: 1fr;
  }
  
  .analysis-options {
    flex-direction: column;
  }
  
  .footer-actions {
    flex-direction: column;
    width: 100%;
  }
  
  .cancel-button,
  .save-button {
    justify-content: center;
  }
}
</style>