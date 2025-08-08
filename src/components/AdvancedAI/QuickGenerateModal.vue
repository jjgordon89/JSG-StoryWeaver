<template>
  <div class="modal-overlay" @click="handleOverlayClick">
    <div class="modal-content" @click.stop>
      <!-- Modal Header -->
      <div class="modal-header">
        <h3>Quick Generate</h3>
        <button @click="$emit('close')" class="close-button">
          <i class="fas fa-times"></i>
        </button>
      </div>

      <!-- Modal Body -->
      <div class="modal-body">
        <!-- Quick Actions -->
        <div class="quick-actions">
          <h4>Quick Actions</h4>
          <div class="action-grid">
            <button 
              v-for="action in quickActions" 
              :key="action.id"
              @click="selectAction(action)"
              :class="['action-button', { active: selectedAction?.id === action.id }]"
            >
              <i :class="action.icon"></i>
              <span class="action-title">{{ action.title }}</span>
              <span class="action-description">{{ action.description }}</span>
            </button>
          </div>
        </div>

        <!-- Generation Form -->
        <div v-if="selectedAction" class="generation-form">
          <h4>{{ selectedAction.title }}</h4>
          
          <!-- Prompt Input -->
          <div class="form-group">
            <label for="quick-prompt">{{ selectedAction.promptLabel || 'Prompt' }}</label>
            <textarea 
              id="quick-prompt"
              v-model="prompt"
              :placeholder="selectedAction.promptPlaceholder"
              class="form-textarea"
              rows="3"
            ></textarea>
          </div>
          
          <!-- Quick Settings -->
          <div class="quick-settings">
            <div class="setting-row">
              <label class="setting-label">
                Length
                <select v-model="settings.length" class="setting-select">
                  <option value="short">Short (50-100 words)</option>
                  <option value="medium">Medium (100-300 words)</option>
                  <option value="long">Long (300-500 words)</option>
                </select>
              </label>
              
              <label class="setting-label">
                Creativity
                <select v-model="settings.creativity" class="setting-select">
                  <option value="conservative">Conservative</option>
                  <option value="balanced">Balanced</option>
                  <option value="creative">Creative</option>
                  <option value="experimental">Experimental</option>
                </select>
              </label>
            </div>
            
            <div class="setting-row">
              <label class="setting-checkbox">
                <input 
                  type="checkbox" 
                  v-model="settings.useContext"
                >
                Use story context
              </label>
              
              <label class="setting-checkbox">
                <input 
                  type="checkbox" 
                  v-model="settings.streaming"
                >
                Stream generation
              </label>
            </div>
          </div>
          
          <!-- Context Preview -->
          <div v-if="settings.useContext && contextPreview" class="context-preview">
            <div class="context-header">
              <i class="fas fa-book-open"></i>
              Context Preview
              <span class="context-length">({{ contextPreview.length }} characters)</span>
            </div>
            <div class="context-content">
              {{ contextPreview.substring(0, 200) }}{{ contextPreview.length > 200 ? '...' : '' }}
            </div>
          </div>
        </div>
      </div>

      <!-- Modal Footer -->
      <div class="modal-footer">
        <div class="footer-info">
          <span v-if="estimatedCredits" class="credit-estimate">
            <i class="fas fa-coins"></i>
            ~{{ estimatedCredits }} credits
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
            @click="handleGenerate"
            :disabled="!canGenerate"
            class="generate-button"
          >
            <i :class="isGenerating ? 'fas fa-spinner fa-spin' : 'fas fa-magic'"></i>
            {{ isGenerating ? 'Generating...' : 'Generate' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue';
import { useAdvancedAIStore } from '../../stores/advancedAIStore';
import { useProjectStore } from '../../stores/projectStore';
import type { AdvancedGenerationRequest } from '../../types/advancedAI';

// Props
interface Props {
  initialAction?: string;
}

const props = defineProps<Props>();

// Emits
interface Emits {
  close: [];
  generated: [result: any];
}

const emit = defineEmits<Emits>();

// Stores
const advancedAIStore = useAdvancedAIStore();
const projectStore = useProjectStore();

// Reactive state
const selectedAction = ref<any>(null);
const prompt = ref('');
const isGenerating = ref(false);

const settings = ref({
  length: 'medium',
  creativity: 'balanced',
  useContext: true,
  streaming: false
});

// Quick actions configuration
const quickActions = ref([
  {
    id: 'continue-scene',
    title: 'Continue Scene',
    description: 'Continue the current scene naturally',
    icon: 'fas fa-play',
    promptLabel: 'Direction (optional)',
    promptPlaceholder: 'e.g., "Focus on dialogue" or "Add tension"'
  },
  {
    id: 'describe-setting',
    title: 'Describe Setting',
    description: 'Add vivid setting description',
    icon: 'fas fa-mountain',
    promptLabel: 'Setting details',
    promptPlaceholder: 'e.g., "A bustling medieval marketplace" or "An abandoned space station"'
  },
  {
    id: 'character-dialogue',
    title: 'Character Dialogue',
    description: 'Generate character conversation',
    icon: 'fas fa-comments',
    promptLabel: 'Dialogue context',
    promptPlaceholder: 'e.g., "Sarah confronts her brother about the secret"'
  },
  {
    id: 'action-sequence',
    title: 'Action Sequence',
    description: 'Create dynamic action scene',
    icon: 'fas fa-running',
    promptLabel: 'Action description',
    promptPlaceholder: 'e.g., "Chase scene through the city" or "Sword fight in the castle"'
  },
  {
    id: 'emotional-moment',
    title: 'Emotional Moment',
    description: 'Develop character emotions',
    icon: 'fas fa-heart',
    promptLabel: 'Emotional context',
    promptPlaceholder: 'e.g., "Character realizes the truth" or "Moment of grief"'
  },
  {
    id: 'plot-twist',
    title: 'Plot Twist',
    description: 'Add unexpected revelation',
    icon: 'fas fa-surprise',
    promptLabel: 'Twist concept',
    promptPlaceholder: 'e.g., "The ally is actually the villain" or "Hidden family connection"'
  },
  {
    id: 'transition',
    title: 'Scene Transition',
    description: 'Bridge between scenes',
    icon: 'fas fa-exchange-alt',
    promptLabel: 'Transition type',
    promptPlaceholder: 'e.g., "Time jump to next morning" or "Location change to the forest"'
  },
  {
    id: 'world-building',
    title: 'World Building',
    description: 'Expand story world details',
    icon: 'fas fa-globe',
    promptLabel: 'World element',
    promptPlaceholder: 'e.g., "Magic system rules" or "Political structure"'
  }
]);

// Computed properties
const canGenerate = computed(() => {
  return selectedAction.value && !isGenerating.value;
});

const contextPreview = computed(() => {
  if (!settings.value.useContext) return null;
  
  // Get current document context
  const currentDoc = projectStore.currentDocument;
  if (!currentDoc?.content) return null;
  
  // Return last 1000 characters as context
  const content = currentDoc.content;
  return content.length > 1000 ? content.slice(-1000) : content;
});

const estimatedCredits = computed(() => {
  if (!selectedAction.value) return null;
  
  let baseCredits = 2;
  
  // Adjust based on length
  switch (settings.value.length) {
    case 'short':
      baseCredits *= 0.5;
      break;
    case 'medium':
      baseCredits *= 1;
      break;
    case 'long':
      baseCredits *= 2;
      break;
  }
  
  // Adjust based on creativity
  switch (settings.value.creativity) {
    case 'conservative':
      baseCredits *= 0.8;
      break;
    case 'balanced':
      baseCredits *= 1;
      break;
    case 'creative':
      baseCredits *= 1.2;
      break;
    case 'experimental':
      baseCredits *= 1.5;
      break;
  }
  
  // Add context cost
  if (settings.value.useContext && contextPreview.value) {
    baseCredits += Math.ceil(contextPreview.value.length / 1000);
  }
  
  return Math.ceil(baseCredits);
});

// Methods
const selectAction = (action: any) => {
  selectedAction.value = action;
  prompt.value = '';
};

const handleGenerate = async () => {
  if (!canGenerate.value || !selectedAction.value) return;
  
  isGenerating.value = true;
  
  try {
    // Build generation request
    const request: AdvancedGenerationRequest = {
      prompt: buildPrompt(),
      proseMode: 'narrative', // Default prose mode for quick generation
      settings: {
        contextLength: settings.value.useContext ? 2000 : 0,
        outputLength: getOutputLength(),
        creativity: getCreativityValue(),
        streaming: settings.value.streaming,
        clicheDetection: false, // Disabled for quick generation
        ultraCreativeMode: settings.value.creativity === 'experimental'
      },
      context: settings.value.useContext ? contextPreview.value || '' : '',
      selectedElements: {
        characters: [],
        locations: [],
        plotThreads: [],
        worldbuildingElements: []
      }
    };
    
    // Generate content
    const result = await advancedAIStore.generateAdvancedText(request);
    
    // Emit result
    emit('generated', {
      action: selectedAction.value,
      result,
      settings: settings.value
    });
    
    // Close modal
    emit('close');
  } catch (error) {
    console.error('Quick generation failed:', error);
    // Handle error (could show toast notification)
  } finally {
    isGenerating.value = false;
  }
};

const buildPrompt = (): string => {
  const action = selectedAction.value;
  const userPrompt = prompt.value.trim();
  
  let basePrompt = '';
  
  switch (action.id) {
    case 'continue-scene':
      basePrompt = userPrompt 
        ? `Continue the scene with focus on: ${userPrompt}`
        : 'Continue the current scene naturally, maintaining the established tone and pacing.';
      break;
    case 'describe-setting':
      basePrompt = `Describe the setting: ${userPrompt || 'the current location'}`;
      break;
    case 'character-dialogue':
      basePrompt = `Write dialogue for: ${userPrompt || 'the characters in this scene'}`;
      break;
    case 'action-sequence':
      basePrompt = `Create an action sequence: ${userPrompt || 'an exciting action scene'}`;
      break;
    case 'emotional-moment':
      basePrompt = `Develop an emotional moment: ${userPrompt || 'a meaningful character moment'}`;
      break;
    case 'plot-twist':
      basePrompt = `Introduce a plot twist: ${userPrompt || 'an unexpected revelation'}`;
      break;
    case 'transition':
      basePrompt = `Create a scene transition: ${userPrompt || 'a smooth transition to the next scene'}`;
      break;
    case 'world-building':
      basePrompt = `Expand the world with: ${userPrompt || 'relevant world-building details'}`;
      break;
    default:
      basePrompt = userPrompt || 'Continue the story.';
  }
  
  return basePrompt;
};

const getOutputLength = (): number => {
  switch (settings.value.length) {
    case 'short': return 75;
    case 'medium': return 200;
    case 'long': return 400;
    default: return 200;
  }
};

const getCreativityValue = (): number => {
  switch (settings.value.creativity) {
    case 'conservative': return 0.3;
    case 'balanced': return 0.7;
    case 'creative': return 0.85;
    case 'experimental': return 0.95;
    default: return 0.7;
  }
};

const handleOverlayClick = () => {
  emit('close');
};

// Watchers
watch(() => props.initialAction, (actionId) => {
  if (actionId) {
    const action = quickActions.value.find(a => a.id === actionId);
    if (action) {
      selectAction(action);
    }
  }
}, { immediate: true });

// Lifecycle
onMounted(() => {
  // Auto-select first action if no initial action specified
  if (!props.initialAction && quickActions.value.length > 0) {
    selectAction(quickActions.value[0]);
  }
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
  max-width: 700px;
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
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.quick-actions h4 {
  margin: 0 0 1rem 0;
  color: var(--text-primary);
}

.action-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 0.75rem;
}

.action-button {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.5rem;
  padding: 1rem;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  background: var(--bg-secondary);
  color: var(--text-primary);
  cursor: pointer;
  transition: all 0.2s ease;
  text-align: center;
}

.action-button:hover {
  background: var(--bg-hover);
  border-color: var(--accent-color);
}

.action-button.active {
  background: var(--accent-color);
  color: white;
  border-color: var(--accent-color);
}

.action-button i {
  font-size: 1.5rem;
}

.action-title {
  font-weight: 500;
  font-size: 0.875rem;
}

.action-description {
  font-size: 0.75rem;
  opacity: 0.8;
  line-height: 1.3;
}

.generation-form {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.generation-form h4 {
  margin: 0;
  color: var(--text-primary);
  border-bottom: 1px solid var(--border-color);
  padding-bottom: 0.5rem;
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

.form-textarea {
  padding: 0.75rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--bg-primary);
  color: var(--text-primary);
  font-family: inherit;
  resize: vertical;
  transition: border-color 0.2s ease;
}

.form-textarea:focus {
  outline: none;
  border-color: var(--accent-color);
}

.quick-settings {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  padding: 1rem;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
}

.setting-row {
  display: flex;
  gap: 1rem;
  align-items: center;
  flex-wrap: wrap;
}

.setting-label {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--text-primary);
  flex: 1;
  min-width: 150px;
}

.setting-select {
  padding: 0.5rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--bg-primary);
  color: var(--text-primary);
  font-size: 0.875rem;
}

.setting-checkbox {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 0.875rem;
  color: var(--text-primary);
  cursor: pointer;
}

.context-preview {
  padding: 1rem;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
}

.context-header {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-weight: 500;
  color: var(--text-primary);
  margin-bottom: 0.5rem;
}

.context-length {
  font-size: 0.75rem;
  color: var(--text-secondary);
  font-weight: normal;
}

.context-content {
  font-family: 'Courier New', monospace;
  font-size: 0.875rem;
  color: var(--text-secondary);
  line-height: 1.4;
  white-space: pre-wrap;
  max-height: 100px;
  overflow-y: auto;
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

.credit-estimate {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 0.875rem;
  color: var(--text-secondary);
}

.footer-actions {
  display: flex;
  gap: 0.5rem;
}

.cancel-button,
.generate-button {
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

.generate-button {
  background: var(--accent-color);
  color: white;
  border-color: var(--accent-color);
}

.generate-button:hover:not(:disabled) {
  background: var(--accent-color-hover);
}

.generate-button:disabled {
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
  
  .action-grid {
    grid-template-columns: 1fr;
  }
  
  .setting-row {
    flex-direction: column;
    align-items: stretch;
  }
  
  .setting-label {
    min-width: auto;
  }
  
  .footer-actions {
    flex-direction: column;
    width: 100%;
  }
  
  .cancel-button,
  .generate-button {
    justify-content: center;
  }
}
</style>