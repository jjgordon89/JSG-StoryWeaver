<template>
  <div class="advanced-ai-container">
    <!-- Header with Mode Selection -->
    <div class="ai-header">
      <div class="mode-selector">
        <label for="prose-mode">Prose Mode:</label>
        <select 
          id="prose-mode" 
          v-model="selectedProseMode" 
          @change="onProseModeChange"
          class="prose-mode-select"
        >
          <option 
            v-for="mode in availableProseModes" 
            :key="mode.name" 
            :value="mode.name"
          >
            {{ mode.name }} - {{ mode.description }}
          </option>
        </select>
      </div>
      
      <div class="ai-status">
        <div class="credit-usage">
          <span class="credits-used">{{ totalCreditsUsed }} credits used</span>
          <span v-if="remainingCredits !== undefined" class="credits-remaining">
            ({{ remainingCredits }} remaining)
          </span>
        </div>
        
        <div class="generation-status" v-if="isGenerating || isGeneratingImage">
          <div class="loading-spinner"></div>
          <span>{{ generationStatusText }}</span>
        </div>
      </div>
    </div>

    <!-- Main Content Tabs -->
    <div class="ai-tabs">
      <nav class="tab-nav">
        <button 
          v-for="tab in tabs" 
          :key="tab.id"
          :class="['tab-button', { active: activeTab === tab.id }]"
          @click="activeTab = tab.id"
        >
          <i :class="tab.icon"></i>
          {{ tab.label }}
        </button>
      </nav>

      <!-- Text Generation Tab -->
      <div v-if="activeTab === 'generate'" class="tab-content">
        <AdvancedTextGenerator 
          :prose-mode="selectedProseMode"
          :saliency-enabled="saliencyEnabled"
          :ultra-creative="ultraCreativeMode"
          @generation-complete="onGenerationComplete"
        />
      </div>

      <!-- Image Generation Tab -->
      <div v-if="activeTab === 'visualize'" class="tab-content">
        <ImageGenerator 
          @image-generated="onImageGenerated"
        />
      </div>

      <!-- Brainstorming Tab -->
      <div v-if="activeTab === 'brainstorm'" class="tab-content">
        <BrainstormEngine 
          :current-session="currentBrainstormSession"
          @session-created="onBrainstormSessionCreated"
        />
      </div>

      <!-- Style Examples Tab -->
      <div v-if="activeTab === 'style'" class="tab-content">
        <StyleManager 
          :active-examples="activeStyleExamples"
          @style-analyzed="onStyleAnalyzed"
        />
      </div>

      <!-- Settings Tab -->
      <div v-if="activeTab === 'settings'" class="tab-content">
        <AdvancedAISettings 
          :saliency-enabled="saliencyEnabled"
          :ultra-creative="ultraCreativeMode"
          :auto-enhance="autoEnhancePrompts"
          :cliche-detection="clicheDetectionEnabled"
          @settings-changed="onSettingsChanged"
        />
      </div>
    </div>

    <!-- Floating Action Button for Quick Generation -->
    <button 
      v-if="canShowQuickGenerate"
      class="quick-generate-fab"
      @click="showQuickGenerate = true"
      :disabled="!canGenerate"
    >
      <i class="fas fa-magic"></i>
    </button>

    <!-- Quick Generate Modal -->
    <QuickGenerateModal 
      v-if="showQuickGenerate"
      :prose-mode="selectedProseMode"
      @close="showQuickGenerate = false"
      @generate="onQuickGenerate"
    />

    <!-- Streaming Status Overlay -->
    <StreamingStatusOverlay 
      v-if="streamingStatus && streamingStatus.status !== 'completed'"
      :status="streamingStatus"
      @cancel="cancelStreaming"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue';
import { useAdvancedAIStore } from '../../stores/advancedAIStore';
import { useProjectStore } from '../../stores/projectStore';
import AdvancedTextGenerator from './AdvancedTextGenerator.vue';
import ImageGenerator from './ImageGenerator.vue';
import BrainstormEngine from './BrainstormEngine.vue';
import StyleManager from './StyleManager.vue';
import AdvancedAISettings from './AdvancedAISettings.vue';
import QuickGenerateModal from './QuickGenerateModal.vue';
import StreamingStatusOverlay from './StreamingStatusOverlay.vue';
import type { AdvancedGenerationResult, GeneratedImage, BrainstormSession, StyleAnalysis } from '../../types/advancedAI';

// Stores
const advancedAIStore = useAdvancedAIStore();
const projectStore = useProjectStore();

// Reactive state
const activeTab = ref('generate');
const selectedProseMode = ref('Excellent');
const showQuickGenerate = ref(false);

// Tab configuration
const tabs = [
  { id: 'generate', label: 'Generate', icon: 'fas fa-pen-fancy' },
  { id: 'visualize', label: 'Visualize', icon: 'fas fa-image' },
  { id: 'brainstorm', label: 'Brainstorm', icon: 'fas fa-lightbulb' },
  { id: 'style', label: 'Style', icon: 'fas fa-palette' },
  { id: 'settings', label: 'Settings', icon: 'fas fa-cog' }
];

// Computed properties
const availableProseModes = computed(() => advancedAIStore.availableProseModes);
const totalCreditsUsed = computed(() => advancedAIStore.totalCreditsUsed);
const remainingCredits = computed(() => advancedAIStore.remainingCredits);
const isGenerating = computed(() => advancedAIStore.isGenerating);
const isGeneratingImage = computed(() => advancedAIStore.isGeneratingImage);
const canGenerate = computed(() => advancedAIStore.canGenerate);
const streamingStatus = computed(() => advancedAIStore.streamingStatus);
const saliencyEnabled = computed(() => advancedAIStore.saliencyEnabled);
const ultraCreativeMode = computed(() => advancedAIStore.ultraCreativeMode);
const autoEnhancePrompts = computed(() => advancedAIStore.autoEnhancePrompts);
const clicheDetectionEnabled = computed(() => advancedAIStore.clicheDetectionEnabled);
const currentBrainstormSession = computed(() => advancedAIStore.currentBrainstormSession);
const activeStyleExamples = computed(() => advancedAIStore.activeStyleExamplesList);

const generationStatusText = computed(() => {
  if (isGenerating.value) {
    if (streamingStatus.value) {
      return `Generating... ${Math.round(streamingStatus.value.progress || 0)}%`;
    }
    return 'Generating text...';
  }
  if (isGeneratingImage.value) {
    return 'Generating image...';
  }
  return '';
});

const canShowQuickGenerate = computed(() => {
  return projectStore.currentProject && !isGenerating.value && !isGeneratingImage.value;
});

// Methods
const onProseModeChange = () => {
  advancedAIStore.setCurrentProseMode(selectedProseMode.value);
};

const onGenerationComplete = (result: AdvancedGenerationResult) => {
  // Handle generation completion
  console.log('Generation completed:', result);
  
  // You could emit an event to parent components or show a notification
  // emit('generation-complete', result);
};

const onImageGenerated = (image: GeneratedImage) => {
  // Handle image generation completion
  console.log('Image generated:', image);
  
  // You could emit an event to parent components or show a notification
  // emit('image-generated', image);
};

const onBrainstormSessionCreated = (session: BrainstormSession) => {
  // Handle brainstorm session creation
  console.log('Brainstorm session created:', session);
  
  // Switch to the brainstorm tab if not already there
  if (activeTab.value !== 'brainstorm') {
    activeTab.value = 'brainstorm';
  }
};

const onStyleAnalyzed = (analysis: StyleAnalysis) => {
  // Handle style analysis completion
  console.log('Style analyzed:', analysis);
};

const onSettingsChanged = (settings: Record<string, any>) => {
  // Handle settings changes
  console.log('Settings changed:', settings);
};

const onQuickGenerate = async (prompt: string) => {
  showQuickGenerate.value = false;
  
  if (!projectStore.currentProject) return;
  
  try {
    const request = {
      project_id: projectStore.currentProject.id,
      prompt,
      prose_mode: selectedProseMode.value,
      use_saliency: saliencyEnabled.value,
      ultra_creative: ultraCreativeMode.value,
      enhance_prompt: autoEnhancePrompts.value,
      detect_cliches: clicheDetectionEnabled.value
    };
    
    await advancedAIStore.generateWithProseMode(request);
  } catch (error) {
    console.error('Quick generation failed:', error);
    // Show error notification
  }
};

const cancelStreaming = () => {
  // Implement streaming cancellation
  console.log('Cancelling streaming generation...');
};

// Lifecycle
onMounted(async () => {
  try {
    await advancedAIStore.initialize();
    
    // Set initial prose mode
    if (availableProseModes.value.length > 0) {
      selectedProseMode.value = advancedAIStore.currentProseMode;
    }
    
    // Update credit usage if we have a current project
    if (projectStore.currentProject) {
      await advancedAIStore.updateCreditUsage(projectStore.currentProject.id);
    }
  } catch (error) {
    console.error('Failed to initialize Advanced AI:', error);
  }
});

// Watch for project changes
watch(
  () => projectStore.currentProject,
  async (newProject) => {
    if (newProject) {
      await advancedAIStore.updateCreditUsage(newProject.id);
      await advancedAIStore.loadProjectImages(newProject.id);
    }
  }
);
</script>

<style scoped>
.advanced-ai-container {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--bg-primary);
  border-radius: 8px;
  overflow: hidden;
}

.ai-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem;
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-color);
}

.mode-selector {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.mode-selector label {
  font-weight: 500;
  color: var(--text-primary);
}

.prose-mode-select {
  padding: 0.5rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--bg-primary);
  color: var(--text-primary);
  min-width: 200px;
}

.ai-status {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.credit-usage {
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  font-size: 0.875rem;
}

.credits-used {
  font-weight: 500;
  color: var(--text-primary);
}

.credits-remaining {
  color: var(--text-secondary);
}

.generation-status {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  color: var(--accent-color);
  font-size: 0.875rem;
}

.loading-spinner {
  width: 16px;
  height: 16px;
  border: 2px solid var(--border-color);
  border-top: 2px solid var(--accent-color);
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.ai-tabs {
  flex: 1;
  display: flex;
  flex-direction: column;
}

.tab-nav {
  display: flex;
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-color);
}

.tab-button {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.75rem 1rem;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.2s ease;
  border-bottom: 2px solid transparent;
}

.tab-button:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.tab-button.active {
  color: var(--accent-color);
  border-bottom-color: var(--accent-color);
  background: var(--bg-primary);
}

.tab-content {
  flex: 1;
  padding: 1rem;
  overflow-y: auto;
}

.quick-generate-fab {
  position: fixed;
  bottom: 2rem;
  right: 2rem;
  width: 56px;
  height: 56px;
  border-radius: 50%;
  border: none;
  background: var(--accent-color);
  color: white;
  font-size: 1.25rem;
  cursor: pointer;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  transition: all 0.2s ease;
  z-index: 1000;
}

.quick-generate-fab:hover:not(:disabled) {
  transform: scale(1.1);
  box-shadow: 0 6px 16px rgba(0, 0, 0, 0.2);
}

.quick-generate-fab:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* Dark mode adjustments */
@media (prefers-color-scheme: dark) {
  .advanced-ai-container {
    background: var(--bg-primary-dark);
  }
  
  .ai-header {
    background: var(--bg-secondary-dark);
    border-bottom-color: var(--border-color-dark);
  }
  
  .prose-mode-select {
    background: var(--bg-primary-dark);
    border-color: var(--border-color-dark);
    color: var(--text-primary-dark);
  }
  
  .tab-nav {
    background: var(--bg-secondary-dark);
    border-bottom-color: var(--border-color-dark);
  }
  
  .tab-button {
    color: var(--text-secondary-dark);
  }
  
  .tab-button:hover {
    background: var(--bg-hover-dark);
    color: var(--text-primary-dark);
  }
  
  .tab-button.active {
    background: var(--bg-primary-dark);
  }
}

/* Responsive design */
@media (max-width: 768px) {
  .ai-header {
    flex-direction: column;
    gap: 1rem;
    align-items: stretch;
  }
  
  .mode-selector {
    justify-content: space-between;
  }
  
  .prose-mode-select {
    min-width: auto;
    flex: 1;
  }
  
  .tab-nav {
    overflow-x: auto;
  }
  
  .tab-button {
    white-space: nowrap;
    min-width: 120px;
    justify-content: center;
  }
  
  .quick-generate-fab {
    bottom: 1rem;
    right: 1rem;
  }
}
</style>