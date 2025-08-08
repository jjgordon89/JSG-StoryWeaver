<template>
  <div class="advanced-text-generator">
    <!-- Generation Form -->
    <div class="generation-form">
      <div class="form-header">
        <h3>Advanced Text Generation</h3>
        <div class="prose-mode-info" v-if="proseModeDetails">
          <span class="mode-name">{{ proseModeDetails.name }}</span>
          <span class="mode-description">{{ proseModeDetails.description }}</span>
        </div>
      </div>

      <!-- Prompt Input -->
      <div class="prompt-section">
        <label for="prompt-input">Writing Prompt:</label>
        <textarea
          id="prompt-input"
          v-model="prompt"
          placeholder="Describe what you want to generate..."
          rows="4"
          class="prompt-textarea"
        ></textarea>
        
        <!-- Prompt Enhancement -->
        <div class="prompt-enhancement" v-if="autoEnhance">
          <button 
            @click="enhancePrompt"
            :disabled="!prompt.trim() || isEnhancing"
            class="enhance-button"
          >
            <i class="fas fa-magic"></i>
            {{ isEnhancing ? 'Enhancing...' : 'Enhance Prompt' }}
          </button>
        </div>
      </div>

      <!-- Generation Options -->
      <div class="generation-options">
        <div class="options-grid">
          <!-- Context Length -->
          <div class="option-group">
            <label for="context-length">Context Length:</label>
            <select id="context-length" v-model="contextLength" class="option-select">
              <option value="short">Short (500 tokens)</option>
              <option value="medium">Medium (1000 tokens)</option>
              <option value="long">Long (2000 tokens)</option>
              <option value="extended">Extended (4000 tokens)</option>
            </select>
          </div>

          <!-- Output Length -->
          <div class="option-group">
            <label for="output-length">Output Length:</label>
            <select id="output-length" v-model="outputLength" class="option-select">
              <option value="paragraph">Paragraph (100-200 words)</option>
              <option value="short">Short (200-500 words)</option>
              <option value="medium">Medium (500-1000 words)</option>
              <option value="long">Long (1000-2000 words)</option>
            </select>
          </div>

          <!-- Temperature -->
          <div class="option-group">
            <label for="temperature">Creativity Level:</label>
            <div class="temperature-control">
              <input 
                id="temperature"
                type="range" 
                v-model="temperature" 
                min="0.1" 
                max="1.0" 
                step="0.1"
                class="temperature-slider"
              >
              <span class="temperature-value">{{ temperature }}</span>
            </div>
          </div>
        </div>

        <!-- Advanced Options -->
        <div class="advanced-options">
          <div class="option-toggles">
            <label class="toggle-option">
              <input 
                type="checkbox" 
                v-model="useSaliency"
                :disabled="!saliencyEnabled"
              >
              <span class="toggle-label">
                Use Smart Context (Saliency Engine)
                <i class="fas fa-info-circle" title="Automatically select relevant story elements"></i>
              </span>
            </label>

            <label class="toggle-option">
              <input type="checkbox" v-model="useStreaming">
              <span class="toggle-label">
                Stream Generation
                <i class="fas fa-info-circle" title="See text as it's being generated"></i>
              </span>
            </label>

            <label class="toggle-option">
              <input type="checkbox" v-model="detectCliches">
              <span class="toggle-label">
                Detect Clichés
                <i class="fas fa-info-circle" title="Highlight potential clichés in the output"></i>
              </span>
            </label>

            <label class="toggle-option" v-if="ultraCreative">
              <input type="checkbox" v-model="useUltraCreative">
              <span class="toggle-label">
                Ultra-Creative Mode
                <i class="fas fa-info-circle" title="Maximum creativity and experimentation"></i>
              </span>
            </label>
          </div>
        </div>
      </div>

      <!-- Action Buttons -->
      <div class="action-buttons">
        <button 
          @click="generateText"
          :disabled="!canGenerate"
          class="generate-button primary"
        >
          <i class="fas fa-pen-fancy"></i>
          {{ isGenerating ? 'Generating...' : 'Generate Text' }}
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

    <!-- Generation Results -->
    <div class="generation-results" v-if="lastResult || streamingText">
      <div class="results-header">
        <h4>Generated Text</h4>
        <div class="result-actions">
          <button 
            @click="copyToClipboard"
            class="action-button"
            title="Copy to clipboard"
          >
            <i class="fas fa-copy"></i>
          </button>
          <button 
            @click="insertIntoDocument"
            class="action-button"
            title="Insert into current document"
          >
            <i class="fas fa-plus"></i>
          </button>
          <button 
            @click="saveAsSnippet"
            class="action-button"
            title="Save as snippet"
          >
            <i class="fas fa-bookmark"></i>
          </button>
        </div>
      </div>

      <!-- Generated Text Display -->
      <div class="generated-text">
        <div class="text-content" ref="textContent">
          {{ displayText }}
          <span v-if="isStreaming && streamingText" class="cursor-blink">|</span>
        </div>
        
        <!-- Generation Metadata -->
        <div class="generation-metadata" v-if="lastResult">
          <div class="metadata-item">
            <span class="label">Prose Mode:</span>
            <span class="value">{{ lastResult.prose_mode_used }}</span>
          </div>
          <div class="metadata-item">
            <span class="label">Tokens:</span>
            <span class="value">{{ lastResult.token_count }}</span>
          </div>
          <div class="metadata-item">
            <span class="label">Credits Used:</span>
            <span class="value">{{ lastResult.credits_used }}</span>
          </div>
        </div>
      </div>

      <!-- Cliché Detection Results -->
      <div class="cliche-detection" v-if="clicheResults && clicheResults.length > 0">
        <h5>Potential Clichés Detected:</h5>
        <div class="cliche-list">
          <div 
            v-for="cliche in clicheResults" 
            :key="cliche.phrase"
            class="cliche-item"
          >
            <span class="cliche-phrase">"{{ cliche.phrase }}"</span>
            <span class="cliche-confidence">{{ Math.round(cliche.confidence * 100) }}%</span>
            <span class="cliche-suggestion" v-if="cliche.suggestion">
              Suggestion: {{ cliche.suggestion }}
            </span>
          </div>
        </div>
      </div>
    </div>

    <!-- Saliency Context Display -->
    <div class="saliency-context" v-if="useSaliency && lastSaliencyContext">
      <div class="context-header">
        <h4>Smart Context Used</h4>
        <button @click="showSaliencyDetails = !showSaliencyDetails" class="toggle-details">
          <i :class="showSaliencyDetails ? 'fas fa-chevron-up' : 'fas fa-chevron-down'"></i>
        </button>
      </div>
      
      <div v-if="showSaliencyDetails" class="context-details">
        <div class="context-section" v-if="lastSaliencyContext.selected_elements.characters.length > 0">
          <h5>Characters:</h5>
          <div class="context-items">
            <span 
              v-for="char in lastSaliencyContext.selected_elements.characters" 
              :key="char.id"
              class="context-item"
            >
              {{ char.name }}
            </span>
          </div>
        </div>
        
        <div class="context-section" v-if="lastSaliencyContext.selected_elements.locations.length > 0">
          <h5>Locations:</h5>
          <div class="context-items">
            <span 
              v-for="location in lastSaliencyContext.selected_elements.locations" 
              :key="location.id"
              class="context-item"
            >
              {{ location.name }}
            </span>
          </div>
        </div>
        
        <div class="context-section" v-if="lastSaliencyContext.selected_elements.plot_threads.length > 0">
          <h5>Plot Threads:</h5>
          <div class="context-items">
            <span 
              v-for="thread in lastSaliencyContext.selected_elements.plot_threads" 
              :key="thread.id"
              class="context-item"
            >
              {{ thread.title }}
            </span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, nextTick } from 'vue';
import { useAdvancedAIStore } from '../../stores/advancedAIStore';
import { useProjectStore } from '../../stores/projectStore';
import { useDocumentStore } from '../../stores/documentStore';
import type { 
  ProseGenerationRequest, 
  AdvancedGenerationResult, 
  ClicheDetectionResult,
  SaliencyContext
} from '../../types/advancedAI';

// Props
interface Props {
  proseMode: string;
  saliencyEnabled: boolean;
  ultraCreative: boolean;
}

const props = defineProps<Props>();

// Emits
const emit = defineEmits<{
  'generation-complete': [result: AdvancedGenerationResult];
}>() ;

// Stores
const advancedAIStore = useAdvancedAIStore();
const projectStore = useProjectStore();
const documentStore = useDocumentStore();

// Reactive state
const prompt = ref('');
const contextLength = ref('medium');
const outputLength = ref('medium');
const temperature = ref(0.7);
const useSaliency = ref(true);
const useStreaming = ref(false);
const detectCliches = ref(true);
const useUltraCreative = ref(false);
const isEnhancing = ref(false);
const showSaliencyDetails = ref(false);
const streamingText = ref('');
const clicheResults = ref<ClicheDetectionResult[]>([]);

// Refs
const textContent = ref<HTMLElement>();

// Computed properties
const proseModeDetails = computed(() => {
  return advancedAIStore.availableProseModes.find(mode => mode.name === props.proseMode);
});

const isGenerating = computed(() => advancedAIStore.isGenerating);
const isStreaming = computed(() => {
  return useStreaming.value && advancedAIStore.streamingStatus?.status === 'generating';
});

const lastResult = computed(() => advancedAIStore.lastGenerationResult);
const lastSaliencyContext = computed(() => advancedAIStore.lastSaliencyContext);
const autoEnhance = computed(() => advancedAIStore.autoEnhancePrompts);

const canGenerate = computed(() => {
  return prompt.value.trim().length > 0 && 
         !isGenerating.value && 
         projectStore.currentProject;
});

const displayText = computed(() => {
  if (isStreaming.value && streamingText.value) {
    return streamingText.value;
  }
  return lastResult.value?.generated_text || '';
});

// Methods
const enhancePrompt = async () => {
  if (!prompt.value.trim()) return;
  
  isEnhancing.value = true;
  try {
    // This would call a prompt enhancement service
    // For now, we'll simulate it
    await new Promise(resolve => setTimeout(resolve, 1000));
    
    // Add some enhancement indicators
    const enhancements = [
      'with vivid sensory details',
      'focusing on character emotions',
      'emphasizing the atmosphere',
      'with rich dialogue'
    ];
    
    const randomEnhancement = enhancements[Math.floor(Math.random() * enhancements.length)];
    prompt.value += ` ${randomEnhancement}`;
  } catch (error) {
    console.error('Failed to enhance prompt:', error);
  } finally {
    isEnhancing.value = false;
  }
};

const generateText = async () => {
  if (!canGenerate.value || !projectStore.currentProject) return;
  
  try {
    const request: ProseGenerationRequest = {
      project_id: projectStore.currentProject.id,
      prompt: prompt.value,
      prose_mode: props.proseMode,
      context_length: contextLength.value,
      output_length: outputLength.value,
      temperature: temperature.value,
      use_saliency: useSaliency.value && props.saliencyEnabled,
      ultra_creative: useUltraCreative.value && props.ultraCreative,
      enhance_prompt: autoEnhance.value,
      detect_cliches: detectCliches.value,
      style_examples: advancedAIStore.activeStyleExamplesList.map(ex => ex.id)
    };
    
    let result: AdvancedGenerationResult;
    
    if (useStreaming.value) {
      // Start streaming generation
      const streamId = await advancedAIStore.startStreamingGeneration(request);
      
      // Watch for streaming updates
      const unwatch = watch(
        () => advancedAIStore.streamingStatus,
        (status) => {
          if (status?.current_text) {
            streamingText.value = status.current_text;
            
            // Auto-scroll to bottom
            nextTick(() => {
              if (textContent.value) {
                textContent.value.scrollTop = textContent.value.scrollHeight;
              }
            });
          }
          
          if (status?.status === 'completed') {
            unwatch();
            result = advancedAIStore.lastGenerationResult!;
            handleGenerationComplete(result);
          }
        },
        { immediate: true }
      );
    } else {
      // Regular generation
      result = await advancedAIStore.generateWithProseMode(request);
      handleGenerationComplete(result);
    }
  } catch (error) {
    console.error('Generation failed:', error);
    // Show error notification
  }
};

const handleGenerationComplete = (result: AdvancedGenerationResult) => {
  // Handle cliché detection results if available
  if (result.cliche_detection) {
    clicheResults.value = result.cliche_detection;
  }
  
  emit('generation-complete', result);
};

const clearForm = () => {
  prompt.value = '';
  streamingText.value = '';
  clicheResults.value = [];
  advancedAIStore.clearLastGeneration();
};

const copyToClipboard = async () => {
  const text = displayText.value;
  if (text) {
    try {
      await navigator.clipboard.writeText(text);
      // Show success notification
    } catch (error) {
      console.error('Failed to copy to clipboard:', error);
    }
  }
};

const insertIntoDocument = () => {
  const text = displayText.value;
  if (text && documentStore.currentDocument) {
    // Insert text at current cursor position
    documentStore.insertTextAtCursor(text);
  }
};

const saveAsSnippet = () => {
  const text = displayText.value;
  if (text) {
    // Save as a reusable snippet
    // This would integrate with a snippets system
    console.log('Saving as snippet:', text);
  }
};

// Watch for prop changes
watch(
  () => props.saliencyEnabled,
  (enabled) => {
    if (!enabled) {
      useSaliency.value = false;
    }
  }
);

watch(
  () => props.ultraCreative,
  (enabled) => {
    if (!enabled) {
      useUltraCreative.value = false;
    }
  }
);
</script>

<style scoped>
.advanced-text-generator {
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

.prose-mode-info {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.mode-name {
  font-weight: 600;
  color: var(--accent-color);
}

.mode-description {
  font-size: 0.875rem;
  color: var(--text-secondary);
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
  min-height: 100px;
}

.prompt-enhancement {
  margin-top: 0.5rem;
}

.enhance-button {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 1rem;
  border: 1px solid var(--accent-color);
  border-radius: 4px;
  background: transparent;
  color: var(--accent-color);
  cursor: pointer;
  transition: all 0.2s ease;
}

.enhance-button:hover:not(:disabled) {
  background: var(--accent-color);
  color: white;
}

.enhance-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
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

.temperature-control {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.temperature-slider {
  flex: 1;
}

.temperature-value {
  min-width: 2rem;
  text-align: center;
  font-weight: 500;
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

.generation-results {
  background: var(--bg-secondary);
  border-radius: 8px;
  padding: 1.5rem;
  border: 1px solid var(--border-color);
}

.results-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
}

.results-header h4 {
  margin: 0;
  color: var(--text-primary);
}

.result-actions {
  display: flex;
  gap: 0.5rem;
}

.action-button {
  padding: 0.5rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: transparent;
  color: var(--text-primary);
  cursor: pointer;
  transition: all 0.2s ease;
}

.action-button:hover {
  background: var(--bg-hover);
}

.generated-text {
  margin-bottom: 1rem;
}

.text-content {
  padding: 1rem;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 4px;
  white-space: pre-wrap;
  line-height: 1.6;
  max-height: 400px;
  overflow-y: auto;
  color: var(--text-primary);
}

.cursor-blink {
  animation: blink 1s infinite;
}

@keyframes blink {
  0%, 50% { opacity: 1; }
  51%, 100% { opacity: 0; }
}

.generation-metadata {
  display: flex;
  gap: 1rem;
  margin-top: 0.5rem;
  font-size: 0.875rem;
}

.metadata-item {
  display: flex;
  gap: 0.25rem;
}

.metadata-item .label {
  color: var(--text-secondary);
}

.metadata-item .value {
  font-weight: 500;
  color: var(--text-primary);
}

.cliche-detection {
  margin-top: 1rem;
  padding: 1rem;
  background: var(--warning-bg);
  border: 1px solid var(--warning-border);
  border-radius: 4px;
}

.cliche-detection h5 {
  margin: 0 0 0.5rem 0;
  color: var(--warning-text);
}

.cliche-list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.cliche-item {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
  padding: 0.5rem;
  background: var(--bg-primary);
  border-radius: 4px;
}

.cliche-phrase {
  font-weight: 500;
  color: var(--text-primary);
}

.cliche-confidence {
  font-size: 0.875rem;
  color: var(--warning-text);
}

.cliche-suggestion {
  font-size: 0.875rem;
  color: var(--text-secondary);
  font-style: italic;
}

.saliency-context {
  background: var(--bg-secondary);
  border-radius: 8px;
  padding: 1.5rem;
  border: 1px solid var(--border-color);
}

.context-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
}

.context-header h4 {
  margin: 0;
  color: var(--text-primary);
}

.toggle-details {
  padding: 0.25rem;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
}

.context-details {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.context-section h5 {
  margin: 0 0 0.5rem 0;
  color: var(--text-primary);
}

.context-items {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
}

.context-item {
  padding: 0.25rem 0.5rem;
  background: var(--accent-color);
  color: white;
  border-radius: 12px;
  font-size: 0.875rem;
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
  
  .results-header {
    flex-direction: column;
    gap: 1rem;
    align-items: stretch;
  }
  
  .generation-metadata {
    flex-direction: column;
    gap: 0.5rem;
  }
}
</style>